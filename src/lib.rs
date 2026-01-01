//! Rust Util Tools - All-in-One Learning & Utility CLI Suite
//!
//! This library provides modules for typing tests, learning systems with
//! flashcards and quizzes, and various utility features.
//!
//! # Modules
//!
//! - `core`: Core types, errors, and configuration
//! - `modules`: Main application logic (typing, learning)
//! - `ui`: Terminal user interface components
//! - `utils`: Helper utilities
//!
//! # Example
//!
//! ```no_run
//! use rust_util_tools::core::{Config, Language, Difficulty};
//! use rust_util_tools::modules::typing::WordLoader;
//!
//! let config = Config::default();
//! let loader = WordLoader::new(&config.paths.data_dir);
//! let text = loader.generate_text(Language::English, Difficulty::Medium).unwrap();
//! println!("Generated text: {}", text);
//! ```

pub mod core;
pub mod modules;
pub mod ui;
pub mod utils;

// Re-export commonly used items for convenience
pub use core::{Config, Difficulty, Language, Result, UtilError};
pub use modules::{FuzzyMatcher, HighScoreManager, LearningSet, TestResult, WordLoader};
