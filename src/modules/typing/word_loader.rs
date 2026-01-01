//! Word loading and filtering for typing tests.
//!
//! This module handles loading word lists from JSON files and filtering
//! them based on language, difficulty, and CEFR level.

use crate::core::{Difficulty, Language, Result, UtilError};
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Mutex;

/// English word structure from JSON file.
#[derive(Debug, Clone, Deserialize)]
pub struct EnglishWord {
    pub word: String,
    #[serde(default)]
    pub useful_for_flashcard: bool,
    #[serde(default)]
    pub cefr_level: String,
    #[serde(default)]
    pub pos: String,
    #[serde(default)]
    pub word_frequency: u32,
}

/// German word structure from JSON file.
#[derive(Debug, Clone, Deserialize)]
pub struct GermanWord {
    pub word: String,
    #[serde(default)]
    pub useful_for_flashcard: bool,
    #[serde(default)]
    pub cefr_level: String,
    #[serde(default)]
    pub pos: String,
    #[serde(default)]
    pub word_frequency: u32,
    #[serde(default)]
    pub capitalization_sensitive: bool,
}

/// Global word cache to avoid reloading files.
static WORD_CACHE: Lazy<Mutex<HashMap<Language, Vec<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Word loader with caching capabilities.
pub struct WordLoader {
    data_dir: std::path::PathBuf,
}

impl WordLoader {
    /// Create a new WordLoader with the specified data directory.
    ///
    /// # Arguments
    ///
    /// * `data_dir` - Directory containing word JSON files
    pub fn new<P: AsRef<Path>>(data_dir: P) -> Self {
        Self {
            data_dir: data_dir.as_ref().to_path_buf(),
        }
    }

    /// Load and filter words for a typing test.
    ///
    /// This function loads words from the appropriate JSON file, filters them
    /// based on difficulty and CEFR level, and returns a shuffled selection.
    ///
    /// # Arguments
    ///
    /// * `language` - The language to load words for
    /// * `difficulty` - The difficulty level (affects filtering and count)
    ///
    /// # Returns
    ///
    /// A vector of words ready for the typing test.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The word file cannot be read
    /// - The JSON is malformed
    /// - No words match the filtering criteria
    pub fn load_words(&self, language: Language, difficulty: Difficulty) -> Result<Vec<String>> {
        // Try to get from cache first
        let cache = WORD_CACHE.lock().unwrap();
        let cached = cache.get(&language);

        let all_words = if let Some(words) = cached {
            words.clone()
        } else {
            drop(cache); // Release lock before loading
            self.load_and_cache_words(language)?
        };

        // Filter and select words
        let mut working_words = all_words;
        self.filter_words(&mut working_words, language, difficulty)?;
        let selected = self.select_random_words(working_words, difficulty.word_count());

        Ok(selected)
    }

    /// Load words from file and cache them.
    fn load_and_cache_words(&self, language: Language) -> Result<Vec<String>> {
        let words = match language {
            Language::English => self.load_english_words()?,
            Language::German => self.load_german_words()?,
        };

        // Cache the loaded words
        let mut cache = WORD_CACHE.lock().unwrap();
        cache.insert(language, words.clone());

        Ok(words)
    }

    /// Load English words from JSON file.
    fn load_english_words(&self) -> Result<Vec<String>> {
        let path = self.data_dir.join("english_words.json");
        let file = File::open(&path).map_err(|e| UtilError::WordLoadError {
            path: path.display().to_string(),
            source: e,
        })?;

        let reader = BufReader::new(file);
        let words: Vec<EnglishWord> =
            serde_json::from_reader(reader).map_err(|e| UtilError::WordLoadError {
                path: path.display().to_string(),
                source: std::io::Error::new(std::io::ErrorKind::InvalidData, e),
            })?;

        Ok(words.into_iter().map(|w| w.word).collect())
    }

    /// Load German words from JSON file.
    fn load_german_words(&self) -> Result<Vec<String>> {
        let path = self.data_dir.join("german_words.json");
        let file = File::open(&path).map_err(|e| UtilError::WordLoadError {
            path: path.display().to_string(),
            source: e,
        })?;

        let reader = BufReader::new(file);
        let words: Vec<GermanWord> =
            serde_json::from_reader(reader).map_err(|e| UtilError::WordLoadError {
                path: path.display().to_string(),
                source: std::io::Error::new(std::io::ErrorKind::InvalidData, e),
            })?;

        // For German words, handle capitalization
        Ok(words
            .into_iter()
            .map(|w| {
                if w.capitalization_sensitive {
                    w.word
                } else {
                    w.word.to_lowercase()
                }
            })
            .collect())
    }

    /// Filter words based on difficulty criteria.
    ///
    /// This is a placeholder - in the real implementation, we'd need to
    /// reload the full word data with CEFR levels. For now, we just filter
    /// by length as a proxy.
    fn filter_words(
        &self,
        words: &mut Vec<String>,
        _language: Language,
        difficulty: Difficulty,
    ) -> Result<()> {
        let max_length = difficulty.max_word_length();

        words.retain(|w| w.len() <= max_length && !w.is_empty());

        if words.is_empty() {
            return Err(UtilError::NoMatchingWords {
                language: _language.to_string(),
                difficulty: difficulty.to_string(),
            });
        }

        Ok(())
    }

    /// Select random words from the filtered list.
    fn select_random_words(&self, mut words: Vec<String>, count: usize) -> Vec<String> {
        let mut rng = rand::thread_rng();
        words.shuffle(&mut rng);
        words.into_iter().take(count).collect()
    }

    /// Generate a text string from words for typing test.
    pub fn generate_text(&self, language: Language, difficulty: Difficulty) -> Result<String> {
        let words = self.load_words(language, difficulty)?;
        Ok(words.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_loader_creation() {
        let loader = WordLoader::new("data");
        assert_eq!(loader.data_dir, std::path::PathBuf::from("data"));
    }

    #[test]
    fn test_filter_by_length() {
        let loader = WordLoader::new("data");
        let mut words = vec![
            "cat".to_string(),
            "dog".to_string(),
            "elephant".to_string(),
            "a".to_string(),
        ];

        loader
            .filter_words(&mut words, Language::English, Difficulty::Easy)
            .unwrap();

        // Easy difficulty has max_length of 6
        assert!(words.iter().all(|w| w.len() <= 6));
        assert!(!words.contains(&"elephant".to_string()));
    }
}
