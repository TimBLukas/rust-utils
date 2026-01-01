//! Custom error types for the rust-util-tools application.
//!
//! This module defines all error types using `thiserror` for ergonomic error handling.
//! All errors implement `std::error::Error` and provide context-rich messages.

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for the application.
///
/// This enum covers all possible error conditions that can occur during
/// the execution of rust-util-tools.
#[derive(Debug, Error)]
pub enum UtilError {
    /// Error when loading word files
    #[error("Failed to load words from {path}: {source}")]
    WordLoadError {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// Error when no words match the filtering criteria
    #[error("No words match the criteria (language: {language}, difficulty: {difficulty})")]
    NoMatchingWords {
        language: String,
        difficulty: String,
    },

    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Error when parsing configuration files
    #[error("Failed to parse config file at {path}: {source}")]
    ConfigParseError {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    /// Error when loading learning sets
    #[error("Failed to load learning set from {path}: {source}")]
    LearningSetLoadError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Error when parsing learning set data
    #[error("Invalid learning set format in {path}: {reason}")]
    InvalidLearningSetFormat { path: PathBuf, reason: String },

    /// Error when highscore file operations fail
    #[error("Highscore operation failed: {0}")]
    HighscoreError(String),

    /// Generic I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// TOML parsing errors
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    /// Terminal/UI errors
    #[error("Terminal error: {0}")]
    Terminal(String),

    /// User cancelled operation
    #[error("Operation cancelled by user")]
    Cancelled,
}

/// Convenience type alias for Results using our custom error type.
///
/// This allows us to write `Result<T>` instead of `Result<T, UtilError>`
/// throughout the codebase.
pub type Result<T> = std::result::Result<T, UtilError>;
