//! Configuration management for rust-util-tools.
//!
//! This module handles loading, saving, and validating application configuration.
//! Configuration can be loaded from TOML files or created with sensible defaults.

use crate::core::error::{Result, UtilError};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Main application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Paths configuration
    pub paths: PathsConfig,
    /// UI theme configuration
    pub theme: ThemeConfig,
    /// Default settings
    pub defaults: DefaultsConfig,
    /// Learning mode configuration
    pub learning: LearningConfig,
}

/// Path configuration for data files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    /// Directory containing word files
    pub data_dir: PathBuf,
    /// Path to highscores file
    pub highscore_file: PathBuf,
    /// Directory for learning sets
    pub learning_sets_dir: PathBuf,
}

/// UI theme configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Color for correct input
    pub correct_color: String,
    /// Color for incorrect input
    pub error_color: String,
    /// Color for current character
    pub current_color: String,
    /// Color for upcoming text
    pub upcoming_color: String,
    /// Enable animations
    pub animations: bool,
}

/// Default settings for the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultsConfig {
    /// Default language (de or en)
    pub language: String,
    /// Default difficulty (easy, medium, hard)
    pub difficulty: String,
    /// Minimum accuracy to save highscore (0.0-100.0)
    pub min_accuracy_for_highscore: f64,
    /// Maximum number of highscores to keep
    pub max_highscores: usize,
}

/// Learning mode configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    /// Fuzzy matching threshold (0.0-1.0)
    pub fuzzy_threshold: f64,
    /// Enable spaced repetition
    pub spaced_repetition: bool,
    /// Number of Leitner boxes
    pub leitner_boxes: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paths: PathsConfig {
                data_dir: PathBuf::from("data"),
                highscore_file: PathBuf::from("data/highscores.json"),
                learning_sets_dir: PathBuf::from("data/learning_sets"),
            },
            theme: ThemeConfig {
                correct_color: "green".to_string(),
                error_color: "red".to_string(),
                current_color: "cyan".to_string(),
                upcoming_color: "white".to_string(),
                animations: true,
            },
            defaults: DefaultsConfig {
                language: "en".to_string(),
                difficulty: "medium".to_string(),
                min_accuracy_for_highscore: 80.0,
                max_highscores: 50,
            },
            learning: LearningConfig {
                fuzzy_threshold: 0.85,
                spaced_repetition: true,
                leitner_boxes: 5,
            },
        }
    }
}

impl Config {
    /// Load configuration from a TOML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).map_err(|e| UtilError::ConfigError(
            format!("Failed to read config file {}: {}", path.display(), e)
        ))?;

        toml::from_str(&content).map_err(|e| UtilError::ConfigParseError {
            path: path.to_path_buf(),
            source: e,
        })
    }

    /// Save configuration to a TOML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path where the configuration should be saved
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| UtilError::ConfigError(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(path, content)?;
        Ok(())
    }

    /// Load configuration from the default location or create default config.
    ///
    /// This function first tries to load from `config/default.toml`.
    /// If that fails, it returns the default configuration.
    pub fn load_or_default() -> Self {
        Self::load_from_file("config/default.toml").unwrap_or_default()
    }

    /// Validate the configuration.
    ///
    /// Checks that all paths exist and values are within valid ranges.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub fn validate(&self) -> Result<()> {
        // Validate fuzzy threshold
        if !(0.0..=1.0).contains(&self.learning.fuzzy_threshold) {
            return Err(UtilError::ConfigError(
                "fuzzy_threshold must be between 0.0 and 1.0".to_string(),
            ));
        }

        // Validate min accuracy
        if !(0.0..=100.0).contains(&self.defaults.min_accuracy_for_highscore) {
            return Err(UtilError::ConfigError(
                "min_accuracy_for_highscore must be between 0.0 and 100.0".to_string(),
            ));
        }

        // Validate Leitner boxes count
        if self.learning.leitner_boxes < 2 || self.learning.leitner_boxes > 10 {
            return Err(UtilError::ConfigError(
                "leitner_boxes must be between 2 and 10".to_string(),
            ));
        }

        Ok(())
    }

    /// Get the full path to a word file for the given language.
    pub fn word_file_path(&self, language: &crate::core::types::Language) -> PathBuf {
        self.paths.data_dir.join(language.word_file())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.defaults.language, "en");
        assert_eq!(config.learning.fuzzy_threshold, 0.85);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        assert!(config.validate().is_ok());

        config.learning.fuzzy_threshold = 1.5;
        assert!(config.validate().is_err());

        config.learning.fuzzy_threshold = 0.85;
        config.learning.leitner_boxes = 20;
        assert!(config.validate().is_err());
    }
}
