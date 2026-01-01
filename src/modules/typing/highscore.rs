//! Highscore management and persistence.
//!
//! This module handles loading, saving, and managing typing test highscores.

use crate::core::{Difficulty, Language, Result, UtilError};
use crate::modules::typing::scorer::TestResult;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

/// A highscore entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighScore {
    /// Player name
    pub name: String,
    /// Words per minute
    pub wpm: f64,
    /// Accuracy percentage
    pub accuracy: f64,
    /// Language used
    pub language: String,
    /// Difficulty level
    pub difficulty: String,
    /// Timestamp when the score was achieved
    pub timestamp: String,
}

impl HighScore {
    /// Create a new highscore from a test result.
    ///
    /// # Arguments
    ///
    /// * `name` - Player name
    /// * `result` - The test result
    /// * `language` - Language used in the test
    /// * `difficulty` - Difficulty level
    pub fn from_result(
        name: String,
        result: &TestResult,
        language: Language,
        difficulty: Difficulty,
    ) -> Self {
        Self {
            name,
            wpm: result.wpm,
            accuracy: result.accuracy,
            language: language.code().to_string(),
            difficulty: difficulty.to_string(),
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

/// Highscore manager for loading and saving scores.
pub struct HighScoreManager {
    file_path: PathBuf,
    max_scores: usize,
}

impl HighScoreManager {
    /// Create a new highscore manager.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the highscores JSON file
    /// * `max_scores` - Maximum number of scores to keep
    pub fn new<P: AsRef<Path>>(file_path: P, max_scores: usize) -> Self {
        Self {
            file_path: file_path.as_ref().to_path_buf(),
            max_scores,
        }
    }

    /// Load all highscores from file.
    ///
    /// # Returns
    ///
    /// A vector of highscores, or an empty vector if the file doesn't exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the file exists but cannot be read or parsed.
    pub fn load(&self) -> Result<Vec<HighScore>> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.file_path).map_err(|e| UtilError::HighscoreError(
            format!("Failed to open highscore file: {}", e),
        ))?;

        let reader = BufReader::new(file);
        let scores = serde_json::from_reader(reader).map_err(|e| {
            UtilError::HighscoreError(format!("Failed to parse highscore file: {}", e))
        })?;

        Ok(scores)
    }

    /// Save highscores to file.
    ///
    /// # Arguments
    ///
    /// * `scores` - The highscores to save
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn save(&self, scores: &[HighScore]) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = File::create(&self.file_path).map_err(|e| {
            UtilError::HighscoreError(format!("Failed to create highscore file: {}", e))
        })?;

        serde_json::to_writer_pretty(file, scores).map_err(|e| {
            UtilError::HighscoreError(format!("Failed to write highscore file: {}", e))
        })?;

        Ok(())
    }

    /// Add a new highscore and save.
    ///
    /// This function loads existing scores, adds the new one, sorts by WPM,
    /// truncates to max_scores, and saves back to file.
    ///
    /// # Arguments
    ///
    /// * `score` - The new highscore to add
    ///
    /// # Errors
    ///
    /// Returns an error if loading or saving fails.
    pub fn add_score(&self, score: HighScore) -> Result<()> {
        let mut scores = self.load().unwrap_or_default();
        scores.push(score);

        // Sort by WPM (descending)
        scores.sort_by(|a, b| {
            b.wpm
                .partial_cmp(&a.wpm)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Keep only top scores
        scores.truncate(self.max_scores);

        self.save(&scores)
    }

    /// Get top N highscores.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of scores to return
    ///
    /// # Returns
    ///
    /// Up to N highscores, sorted by WPM (descending).
    pub fn get_top(&self, n: usize) -> Result<Vec<HighScore>> {
        let scores = self.load()?;
        Ok(scores.into_iter().take(n).collect())
    }

    /// Get highscores filtered by language and/or difficulty.
    ///
    /// # Arguments
    ///
    /// * `language` - Optional language filter
    /// * `difficulty` - Optional difficulty filter
    ///
    /// # Returns
    ///
    /// Filtered highscores.
    pub fn get_filtered(
        &self,
        language: Option<Language>,
        difficulty: Option<Difficulty>,
    ) -> Result<Vec<HighScore>> {
        let scores = self.load()?;

        let filtered = scores
            .into_iter()
            .filter(|s| {
                let lang_match = language
                    .map(|l| s.language == l.code())
                    .unwrap_or(true);
                let diff_match = difficulty
                    .map(|d| s.difficulty.contains(&d.to_string()))
                    .unwrap_or(true);

                lang_match && diff_match
            })
            .collect();

        Ok(filtered)
    }

    /// Calculate statistics from all highscores.
    pub fn get_statistics(&self) -> Result<HighScoreStatistics> {
        let scores = self.load()?;

        if scores.is_empty() {
            return Ok(HighScoreStatistics::default());
        }

        let total_tests = scores.len();
        let avg_wpm = scores.iter().map(|s| s.wpm).sum::<f64>() / total_tests as f64;
        let avg_accuracy = scores.iter().map(|s| s.accuracy).sum::<f64>() / total_tests as f64;
        let best_wpm = scores
            .iter()
            .map(|s| s.wpm)
            .fold(0.0f64, f64::max);

        let easy_count = scores
            .iter()
            .filter(|s| s.difficulty.contains("Easy"))
            .count();
        let medium_count = scores
            .iter()
            .filter(|s| s.difficulty.contains("Medium"))
            .count();
        let hard_count = scores
            .iter()
            .filter(|s| s.difficulty.contains("Hard"))
            .count();

        Ok(HighScoreStatistics {
            total_tests,
            avg_wpm,
            avg_accuracy,
            best_wpm,
            easy_count,
            medium_count,
            hard_count,
        })
    }
}

/// Statistics calculated from highscores.
#[derive(Debug, Default)]
pub struct HighScoreStatistics {
    pub total_tests: usize,
    pub avg_wpm: f64,
    pub avg_accuracy: f64,
    pub best_wpm: f64,
    pub easy_count: usize,
    pub medium_count: usize,
    pub hard_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_highscore_manager() {
        let temp_file = NamedTempFile::new().unwrap();
        let manager = HighScoreManager::new(temp_file.path(), 10);

        // Initially empty
        let scores = manager.load().unwrap();
        assert!(scores.is_empty());

        // Add a score
        let score = HighScore {
            name: "Test".to_string(),
            wpm: 50.0,
            accuracy: 95.0,
            language: "en".to_string(),
            difficulty: "Medium".to_string(),
            timestamp: "2024-01-01 12:00:00".to_string(),
        };

        manager.add_score(score).unwrap();

        // Load and verify
        let scores = manager.load().unwrap();
        assert_eq!(scores.len(), 1);
        assert_eq!(scores[0].name, "Test");
    }

    #[test]
    fn test_max_scores_truncation() {
        let temp_file = NamedTempFile::new().unwrap();
        let manager = HighScoreManager::new(temp_file.path(), 3);

        // Add 5 scores
        for i in 0..5 {
            let score = HighScore {
                name: format!("Player{}", i),
                wpm: (i * 10) as f64,
                accuracy: 95.0,
                language: "en".to_string(),
                difficulty: "Medium".to_string(),
                timestamp: "2024-01-01 12:00:00".to_string(),
            };
            manager.add_score(score).unwrap();
        }

        // Should only keep top 3
        let scores = manager.load().unwrap();
        assert_eq!(scores.len(), 3);
        assert_eq!(scores[0].wpm, 40.0); // Highest WPM first
    }
}
