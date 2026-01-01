//! Scoring and statistics calculation for typing tests.
//!
//! This module provides pure functions for calculating WPM, CPM, accuracy,
//! and other typing test metrics.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Result of a typing test with all calculated metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Words per minute
    pub wpm: f64,
    /// Characters per minute
    pub cpm: f64,
    /// Accuracy percentage (0.0-100.0)
    pub accuracy: f64,
    /// Test duration
    #[serde(with = "duration_serde")]
    pub duration: Duration,
    /// Number of errors made during typing
    pub error_count: usize,
    /// Total characters typed
    pub total_chars: usize,
    /// Correct characters typed
    pub correct_chars: usize,
}

/// Serde helper for Duration serialization
mod duration_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(duration.as_secs_f64())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = f64::deserialize(deserializer)?;
        Ok(Duration::from_secs_f64(secs))
    }
}

impl TestResult {
    /// Calculate test results from target text, typed text, and duration.
    ///
    /// # Arguments
    ///
    /// * `target` - The text that should have been typed
    /// * `typed` - The text that was actually typed
    /// * `duration` - How long the test took
    /// * `error_count` - Number of errors made during typing (first-try mistakes)
    ///
    /// # Returns
    ///
    /// A `TestResult` with all metrics calculated.
    pub fn calculate(target: &str, typed: &str, duration: Duration, error_count: usize) -> Self {
        let seconds = duration.as_secs_f64();

        // Calculate WPM (assuming average word length of 5 characters)
        let words = target.split_whitespace().count();
        let wpm = if seconds > 0.0 {
            (words as f64 / seconds) * 60.0
        } else {
            0.0
        };

        // Calculate CPM
        let chars = target.chars().count();
        let cpm = if seconds > 0.0 {
            (chars as f64 / seconds) * 60.0
        } else {
            0.0
        };

        // Calculate accuracy
        let (correct_chars, total_chars) = Self::calculate_accuracy_metrics(target, typed);
        let accuracy = if total_chars > 0 {
            (correct_chars as f64 / total_chars as f64) * 100.0
        } else {
            100.0
        };

        Self {
            wpm,
            cpm,
            accuracy,
            duration,
            error_count,
            total_chars,
            correct_chars,
        }
    }

    /// Calculate correct and total characters for accuracy.
    ///
    /// Compares the typed text character-by-character with the target.
    fn calculate_accuracy_metrics(target: &str, typed: &str) -> (usize, usize) {
        let target_chars: Vec<char> = target.chars().collect();
        let typed_chars: Vec<char> = typed.chars().collect();

        let correct = typed_chars
            .iter()
            .zip(target_chars.iter())
            .filter(|(t, s)| t == s)
            .count();

        let total = typed_chars.len();

        (correct, total)
    }

    /// Get a rating string based on WPM and accuracy.
    ///
    /// # Returns
    ///
    /// A human-readable rating of the performance.
    pub fn rating(&self) -> &'static str {
        if self.accuracy >= 98.0 && self.wpm >= 60.0 {
            "PERFEKT! Ausgezeichnete Leistung!"
        } else if self.accuracy >= 95.0 && self.wpm >= 45.0 {
            "SEHR GUT! Starke Performance!"
        } else if self.accuracy >= 90.0 && self.wpm >= 30.0 {
            "GUT! Weiter so!"
        } else {
            "Übung macht den Meister!"
        }
    }

    /// Check if this result qualifies for highscore saving.
    ///
    /// # Arguments
    ///
    /// * `min_accuracy` - Minimum accuracy percentage required
    ///
    /// # Returns
    ///
    /// `true` if the result meets the criteria for saving.
    pub fn qualifies_for_highscore(&self, min_accuracy: f64) -> bool {
        self.accuracy >= min_accuracy
    }

    /// Format duration as a human-readable string.
    pub fn duration_string(&self) -> String {
        format!("{:.2}s", self.duration.as_secs_f64())
    }
}

/// Calculate real-time accuracy during typing.
///
/// This is used for live feedback during the test.
///
/// # Arguments
///
/// * `target` - The target text
/// * `typed` - The currently typed text
///
/// # Returns
///
/// Accuracy percentage (0.0-100.0)
pub fn calculate_realtime_accuracy(target: &str, typed: &str) -> f64 {
    if typed.is_empty() {
        return 100.0;
    }

    let target_chars: Vec<char> = target.chars().collect();
    let typed_chars: Vec<char> = typed.chars().collect();

    let correct = typed_chars
        .iter()
        .zip(target_chars.iter())
        .filter(|(t, s)| t == s)
        .count();

    (correct as f64 / typed_chars.len() as f64) * 100.0
}

/// Calculate real-time progress percentage.
///
/// # Arguments
///
/// * `target` - The target text
/// * `typed` - The currently typed text
///
/// # Returns
///
/// Progress percentage (0.0-100.0)
pub fn calculate_progress(target: &str, typed: &str) -> f64 {
    if target.is_empty() {
        return 100.0;
    }

    (typed.len() as f64 / target.len() as f64) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_accuracy() {
        let result = TestResult::calculate(
            "hello world",
            "hello world",
            Duration::from_secs(10),
            0,
        );

        assert_eq!(result.accuracy, 100.0);
        assert_eq!(result.correct_chars, 11);
        assert_eq!(result.total_chars, 11);
    }

    #[test]
    fn test_wpm_calculation() {
        // "hello world" = 2 words, 10 seconds = 12 WPM
        let result = TestResult::calculate(
            "hello world",
            "hello world",
            Duration::from_secs(10),
            0,
        );

        assert_eq!(result.wpm, 12.0);
    }

    #[test]
    fn test_partial_accuracy() {
        let result = TestResult::calculate(
            "hello world",
            "hallo world", // One wrong character
            Duration::from_secs(10),
            1,
        );

        // 10 out of 11 characters correct
        assert!((result.accuracy - 90.909).abs() < 0.01);
    }

    #[test]
    fn test_rating() {
        let perfect = TestResult {
            wpm: 70.0,
            cpm: 350.0,
            accuracy: 99.0,
            duration: Duration::from_secs(10),
            error_count: 0,
            total_chars: 100,
            correct_chars: 99,
        };

        assert_eq!(perfect.rating(), "PERFEKT! Ausgezeichnete Leistung!");
    }

    #[test]
    fn test_realtime_accuracy() {
        let accuracy = calculate_realtime_accuracy("hello", "hallo");
        assert_eq!(accuracy, 80.0); // 4 out of 5 correct
    }

    #[test]
    fn test_progress() {
        let progress = calculate_progress("hello world", "hello");
        assert!((progress - 45.45).abs() < 0.01); // 5 out of 11 characters
    }
}
