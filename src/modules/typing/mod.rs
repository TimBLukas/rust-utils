//! Typing test module.
//!
//! This module contains all logic related to typing speed tests,
//! including word loading, scoring, and highscore management.

pub mod highscore;
pub mod scorer;
pub mod word_loader;

// Re-export commonly used items
pub use highscore::{HighScore, HighScoreManager, HighScoreStatistics};
pub use scorer::TestResult;
pub use word_loader::WordLoader;
