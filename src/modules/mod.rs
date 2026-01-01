//! Modules containing the main application logic.

pub mod learning;
pub mod typing;

// Re-export commonly used items
pub use learning::{FuzzyMatcher, LearningSet, LeitnerBox};
pub use typing::{HighScoreManager, TestResult, WordLoader};
