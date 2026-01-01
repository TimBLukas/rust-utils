//! Learning module.
//!
//! This module contains all logic for the learning system, including
//! flashcards, quizzes, fuzzy matching, and spaced repetition.

pub mod fuzzy;
pub mod models;
pub mod parsers;
pub mod spaced_rep;

// Re-export commonly used items
pub use fuzzy::{FuzzyMatcher, MatchResult};
pub use models::{Card, LearningSet, QuizQuestion, SessionStats};
pub use parsers::{load_auto, load_from_json};
pub use spaced_rep::{LeitnerBox, LeitnerSummary};
