//! Fuzzy string matching for answer validation.
//!
//! This module provides fuzzy matching capabilities to validate user answers
//! with a configurable threshold and user override mechanism.

use strsim::jaro_winkler;

/// Result of a fuzzy match comparison.
#[derive(Debug, Clone, PartialEq)]
pub enum MatchResult {
    /// Answer is automatically accepted (score >= threshold)
    AutoCorrect { score: f64 },
    /// Answer is automatically rejected (score very low)
    AutoIncorrect { score: f64 },
    /// User needs to decide (score close to threshold)
    NeedsUserDecision {
        score: f64,
        user_input: String,
        correct_answer: String,
    },
}

/// Fuzzy matcher with configurable threshold.
pub struct FuzzyMatcher {
    /// Threshold for automatic acceptance (0.0-1.0)
    threshold: f64,
    /// Margin around threshold where user decision is needed
    decision_margin: f64,
}

impl FuzzyMatcher {
    /// Create a new fuzzy matcher.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Similarity threshold for automatic acceptance (0.0-1.0)
    /// * `decision_margin` - Margin around threshold for user decision zone
    ///
    /// # Example
    ///
    /// ```
    /// use rust_util_tools::modules::learning::fuzzy::FuzzyMatcher;
    ///
    /// let matcher = FuzzyMatcher::new(0.85, 0.10);
    /// ```
    pub fn new(threshold: f64, decision_margin: f64) -> Self {
        Self {
            threshold: threshold.clamp(0.0, 1.0),
            decision_margin: decision_margin.clamp(0.0, 0.5),
        }
    }

    /// Check if a user's answer matches the correct answer.
    ///
    /// # Arguments
    ///
    /// * `user_input` - The user's answer
    /// * `correct_answer` - The correct answer
    ///
    /// # Returns
    ///
    /// A `MatchResult` indicating whether the answer is accepted, rejected,
    /// or needs user decision.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_util_tools::modules::learning::fuzzy::{FuzzyMatcher, MatchResult};
    ///
    /// let matcher = FuzzyMatcher::new(0.85, 0.10);
    /// let result = matcher.check_answer("photosynthesis", "photosynthesis");
    ///
    /// match result {
    ///     MatchResult::AutoCorrect { score } => println!("Correct! ({})", score),
    ///     _ => {}
    /// }
    /// ```
    pub fn check_answer(&self, user_input: &str, correct_answer: &str) -> MatchResult {
        // Normalize inputs (trim, lowercase)
        let user_normalized = user_input.trim().to_lowercase();
        let correct_normalized = correct_answer.trim().to_lowercase();

        // Exact match is always correct
        if user_normalized == correct_normalized {
            return MatchResult::AutoCorrect { score: 1.0 };
        }

        // Calculate similarity score using Jaro-Winkler
        let score = jaro_winkler(&user_normalized, &correct_normalized);

        // Determine result based on threshold and margin
        let upper_bound = self.threshold + self.decision_margin;
        let lower_bound = (self.threshold - self.decision_margin).max(0.0);

        if score >= upper_bound {
            MatchResult::AutoCorrect { score }
        } else if score < lower_bound {
            MatchResult::AutoIncorrect { score }
        } else {
            MatchResult::NeedsUserDecision {
                score,
                user_input: user_input.to_string(),
                correct_answer: correct_answer.to_string(),
            }
        }
    }

    /// Calculate similarity score between two strings.
    ///
    /// This is a convenience method that just returns the raw score.
    pub fn similarity(&self, a: &str, b: &str) -> f64 {
        let a_norm = a.trim().to_lowercase();
        let b_norm = b.trim().to_lowercase();
        jaro_winkler(&a_norm, &b_norm)
    }

    /// Check if two strings are similar enough (above threshold).
    pub fn is_similar(&self, a: &str, b: &str) -> bool {
        self.similarity(a, b) >= self.threshold
    }
}

impl Default for FuzzyMatcher {
    fn default() -> Self {
        Self::new(0.85, 0.10)
    }
}

/// Helper function to format a match result for display.
pub fn format_match_result(result: &MatchResult) -> String {
    match result {
        MatchResult::AutoCorrect { score } => {
            format!("✓ Richtig! (Übereinstimmung: {:.1}%)", score * 100.0)
        }
        MatchResult::AutoIncorrect { score } => {
            format!("✗ Falsch (Übereinstimmung: {:.1}%)", score * 100.0)
        }
        MatchResult::NeedsUserDecision {
            score,
            user_input,
            correct_answer,
        } => {
            format!(
                "? Unsicher (Übereinstimmung: {:.1}%)\n  Deine Antwort: '{}'\n  Korrekte Antwort: '{}'\n  War deine Antwort richtig?",
                score * 100.0,
                user_input,
                correct_answer
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let matcher = FuzzyMatcher::new(0.85, 0.10);
        let result = matcher.check_answer("hello", "hello");

        assert_eq!(result, MatchResult::AutoCorrect { score: 1.0 });
    }

    #[test]
    fn test_case_insensitive() {
        let matcher = FuzzyMatcher::new(0.85, 0.10);
        let result = matcher.check_answer("HELLO", "hello");

        assert_eq!(result, MatchResult::AutoCorrect { score: 1.0 });
    }

    #[test]
    fn test_close_match() {
        let matcher = FuzzyMatcher::new(0.85, 0.10);
        let result = matcher.check_answer("photosynthesis", "photosynthesys");

        // Should be high similarity (typo in last few chars)
        match result {
            MatchResult::AutoCorrect { score } | MatchResult::NeedsUserDecision { score, .. } => {
                assert!(score > 0.85);
            }
            _ => panic!("Expected high similarity"),
        }
    }

    #[test]
    fn test_very_different() {
        let matcher = FuzzyMatcher::new(0.85, 0.10);
        let result = matcher.check_answer("cat", "photosynthesis");

        match result {
            MatchResult::AutoIncorrect { score } => {
                assert!(score < 0.75);
            }
            _ => panic!("Expected auto-incorrect"),
        }
    }

    #[test]
    fn test_similarity_method() {
        let matcher = FuzzyMatcher::new(0.85, 0.10);

        assert_eq!(matcher.similarity("hello", "hello"), 1.0);
        assert!(matcher.similarity("hello", "hallo") > 0.8);
        assert!(matcher.similarity("cat", "dog") < 0.5);
    }

    #[test]
    fn test_whitespace_handling() {
        let matcher = FuzzyMatcher::new(0.85, 0.10);
        let result = matcher.check_answer("  hello  ", "hello");

        assert_eq!(result, MatchResult::AutoCorrect { score: 1.0 });
    }
}
