//! Core type definitions used throughout the application.
//!
//! This module defines type-safe enums and structs that replace
//! string-based identifiers for better compile-time safety.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Supported languages for typing tests and learning content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// German language
    German,
    /// English language
    English,
}

impl Language {
    /// Returns the two-letter language code.
    pub fn code(&self) -> &'static str {
        match self {
            Language::German => "de",
            Language::English => "en",
        }
    }

    /// Returns the full language name.
    pub fn name(&self) -> &'static str {
        match self {
            Language::German => "Deutsch",
            Language::English => "English",
        }
    }

    /// Returns the word file name for this language.
    pub fn word_file(&self) -> &'static str {
        match self {
            Language::German => "german_words.json",
            Language::English => "english_words.json",
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "de" | "german" | "deutsch" => Ok(Language::German),
            "en" | "english" => Ok(Language::English),
            _ => Err(format!("Unknown language: {}", s)),
        }
    }
}

/// Difficulty levels for typing tests and learning content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    /// Easy difficulty (A1-A2 CEFR level, shorter words)
    Easy,
    /// Medium difficulty (A2-B2 CEFR level, medium words)
    Medium,
    /// Hard difficulty (B2-C2 CEFR level, longer words)
    Hard,
}

impl Difficulty {
    /// Returns the number of words for this difficulty level.
    pub fn word_count(&self) -> usize {
        match self {
            Difficulty::Easy => 15,
            Difficulty::Medium => 30,
            Difficulty::Hard => 50,
        }
    }

    /// Returns the allowed CEFR levels for this difficulty.
    pub fn allowed_cefr_levels(&self) -> &'static [&'static str] {
        match self {
            Difficulty::Easy => &["A1", "A2"],
            Difficulty::Medium => &["A2", "B1", "B2"],
            Difficulty::Hard => &["B2", "C1", "C2"],
        }
    }

    /// Returns the maximum word length for this difficulty.
    pub fn max_word_length(&self) -> usize {
        match self {
            Difficulty::Easy => 6,
            Difficulty::Medium => 9,
            Difficulty::Hard => 15,
        }
    }

    /// Returns a human-readable description of this difficulty.
    pub fn description(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Einfach/Easy - 15 Wörter (A1-A2 Niveau)",
            Difficulty::Medium => "Mittel/Medium - 30 Wörter (A2-B2 Niveau)",
            Difficulty::Hard => "Schwer/Hard - 50 Wörter (B2-C2 Niveau)",
        }
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Difficulty::Easy => "Einfach/Easy",
            Difficulty::Medium => "Mittel/Medium",
            Difficulty::Hard => "Schwer/Hard",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for Difficulty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "easy" | "einfach" | "1" => Ok(Difficulty::Easy),
            "medium" | "mittel" | "2" => Ok(Difficulty::Medium),
            "hard" | "schwer" | "3" => Ok(Difficulty::Hard),
            _ => Err(format!("Unknown difficulty: {}", s)),
        }
    }
}

/// CEFR (Common European Framework of Reference) levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CefrLevel {
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
}

impl fmt::Display for CefrLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for CefrLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A1" => Ok(CefrLevel::A1),
            "A2" => Ok(CefrLevel::A2),
            "B1" => Ok(CefrLevel::B1),
            "B2" => Ok(CefrLevel::B2),
            "C1" => Ok(CefrLevel::C1),
            "C2" => Ok(CefrLevel::C2),
            _ => Err(format!("Unknown CEFR level: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_str() {
        assert_eq!("de".parse::<Language>().unwrap(), Language::German);
        assert_eq!("en".parse::<Language>().unwrap(), Language::English);
        assert_eq!("German".parse::<Language>().unwrap(), Language::German);
        assert!("fr".parse::<Language>().is_err());
    }

    #[test]
    fn test_difficulty_from_str() {
        assert_eq!("easy".parse::<Difficulty>().unwrap(), Difficulty::Easy);
        assert_eq!("2".parse::<Difficulty>().unwrap(), Difficulty::Medium);
        assert_eq!("schwer".parse::<Difficulty>().unwrap(), Difficulty::Hard);
    }

    #[test]
    fn test_difficulty_word_count() {
        assert_eq!(Difficulty::Easy.word_count(), 15);
        assert_eq!(Difficulty::Medium.word_count(), 30);
        assert_eq!(Difficulty::Hard.word_count(), 50);
    }
}
