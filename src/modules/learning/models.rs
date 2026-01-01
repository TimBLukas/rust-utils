//! Data models for the learning system.
//!
//! This module defines the core data structures for flashcards,
//! quiz questions, and learning sets.

use serde::{Deserialize, Serialize};

/// A flashcard with front and back sides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// Front side of the card (question/prompt)
    pub front: String,
    /// Back side of the card (answer)
    pub back: String,
    /// Optional tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
    /// Optional explanation or additional context
    #[serde(default)]
    pub explanation: Option<String>,
}

/// A quiz question with multiple choice or text answer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestion {
    /// The question text
    pub question: String,
    /// The correct answer
    pub correct_answer: String,
    /// Alternative answers for multiple choice (empty for text input)
    #[serde(default)]
    pub alternatives: Vec<String>,
    /// Optional explanation shown after answering
    #[serde(default)]
    pub explanation: Option<String>,
    /// Optional tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
}

impl QuizQuestion {
    /// Check if this is a multiple choice question.
    pub fn is_multiple_choice(&self) -> bool {
        !self.alternatives.is_empty()
    }

    /// Get all answer options (correct + alternatives) shuffled.
    pub fn get_shuffled_options(&self) -> Vec<String> {
        use rand::seq::SliceRandom;
        let mut options = self.alternatives.clone();
        options.push(self.correct_answer.clone());
        options.shuffle(&mut rand::thread_rng());
        options
    }
}

/// A collection of learning content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSet {
    /// Name of the learning set
    pub name: String,
    /// Description of what this set covers
    #[serde(default)]
    pub description: String,
    /// Flashcards in this set
    #[serde(default)]
    pub cards: Vec<Card>,
    /// Quiz questions in this set
    #[serde(default)]
    pub questions: Vec<QuizQuestion>,
    /// Metadata tags
    #[serde(default)]
    pub tags: Vec<String>,
}

impl LearningSet {
    /// Get total number of items (cards + questions).
    pub fn total_items(&self) -> usize {
        self.cards.len() + self.questions.len()
    }

    /// Check if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty() && self.questions.is_empty()
    }
}

/// Statistics for a learning session.
#[derive(Debug, Default, Clone)]
pub struct SessionStats {
    /// Total items reviewed
    pub total_reviewed: usize,
    /// Items answered correctly
    pub correct: usize,
    /// Items answered incorrectly
    pub incorrect: usize,
    /// Items where user override was used
    pub user_overrides: usize,
}

impl SessionStats {
    /// Calculate accuracy percentage.
    pub fn accuracy(&self) -> f64 {
        if self.total_reviewed == 0 {
            return 0.0;
        }
        (self.correct as f64 / self.total_reviewed as f64) * 100.0
    }

    /// Record a correct answer.
    pub fn record_correct(&mut self) {
        self.total_reviewed += 1;
        self.correct += 1;
    }

    /// Record an incorrect answer.
    pub fn record_incorrect(&mut self) {
        self.total_reviewed += 1;
        self.incorrect += 1;
    }

    /// Record a user override (fuzzy match that user corrected).
    pub fn record_override(&mut self, was_correct: bool) {
        self.total_reviewed += 1;
        self.user_overrides += 1;
        if was_correct {
            self.correct += 1;
        } else {
            self.incorrect += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz_question_multiple_choice() {
        let q = QuizQuestion {
            question: "What is 2+2?".to_string(),
            correct_answer: "4".to_string(),
            alternatives: vec!["3".to_string(), "5".to_string()],
            explanation: None,
            tags: vec![],
        };

        assert!(q.is_multiple_choice());
        let options = q.get_shuffled_options();
        assert_eq!(options.len(), 3);
        assert!(options.contains(&"4".to_string()));
    }

    #[test]
    fn test_session_stats() {
        let mut stats = SessionStats::default();
        stats.record_correct();
        stats.record_correct();
        stats.record_incorrect();

        assert_eq!(stats.total_reviewed, 3);
        assert_eq!(stats.correct, 2);
        assert_eq!(stats.incorrect, 1);
        assert!((stats.accuracy() - 66.666).abs() < 0.01);
    }

    #[test]
    fn test_learning_set_empty() {
        let set = LearningSet {
            name: "Test".to_string(),
            description: String::new(),
            cards: vec![],
            questions: vec![],
            tags: vec![],
        };

        assert!(set.is_empty());
        assert_eq!(set.total_items(), 0);
    }
}
