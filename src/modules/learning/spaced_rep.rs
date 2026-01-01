//! Spaced repetition system using the Leitner box method.
//!
//! This module implements a simple but effective spaced repetition algorithm
//! to help users learn more efficiently by reviewing difficult items more frequently.

use std::collections::VecDeque;

/// Leitner box system for spaced repetition.
///
/// Items start in box 0. When answered correctly, they move to the next box.
/// When answered incorrectly, they move back to box 0.
/// Items in lower boxes are reviewed more frequently.
pub struct LeitnerBox {
    /// Number of boxes in the system
    num_boxes: usize,
    /// Each box contains indices of items
    boxes: Vec<VecDeque<usize>>,
    /// Track which box each item is in
    item_locations: Vec<usize>,
}

impl LeitnerBox {
    /// Create a new Leitner box system.
    ///
    /// # Arguments
    ///
    /// * `num_boxes` - Number of boxes (typically 3-7)
    /// * `num_items` - Total number of items to manage
    ///
    /// # Example
    ///
    /// ```
    /// use rust_util_tools::modules::learning::spaced_rep::LeitnerBox;
    ///
    /// let leitner = LeitnerBox::new(5, 20);
    /// ```
    pub fn new(num_boxes: usize, num_items: usize) -> Self {
        let mut boxes = vec![VecDeque::new(); num_boxes];
        
        // All items start in box 0
        for i in 0..num_items {
            boxes[0].push_back(i);
        }

        let item_locations = vec![0; num_items];

        Self {
            num_boxes,
            boxes,
            item_locations,
        }
    }

    /// Record a correct answer for an item.
    ///
    /// Moves the item to the next box (if not already in the last box).
    ///
    /// # Arguments
    ///
    /// * `item_id` - Index of the item
    pub fn answer_correct(&mut self, item_id: usize) {
        if item_id >= self.item_locations.len() {
            return;
        }

        let current_box = self.item_locations[item_id];
        let next_box = (current_box + 1).min(self.num_boxes - 1);

        // Remove from current box
        self.boxes[current_box].retain(|&id| id != item_id);

        // Add to next box
        self.boxes[next_box].push_back(item_id);
        self.item_locations[item_id] = next_box;
    }

    /// Record an incorrect answer for an item.
    ///
    /// Moves the item back to box 0.
    ///
    /// # Arguments
    ///
    /// * `item_id` - Index of the item
    pub fn answer_incorrect(&mut self, item_id: usize) {
        if item_id >= self.item_locations.len() {
            return;
        }

        let current_box = self.item_locations[item_id];

        // Remove from current box
        self.boxes[current_box].retain(|&id| id != item_id);

        // Add to box 0
        self.boxes[0].push_back(item_id);
        self.item_locations[item_id] = 0;
    }

    /// Get the next item to review.
    ///
    /// Returns items from lower boxes first (more frequent review).
    ///
    /// # Returns
    ///
    /// The index of the next item to review, or `None` if all boxes are empty.
    pub fn get_next_item(&self) -> Option<usize> {
        // Check boxes from lowest to highest
        for box_items in &self.boxes {
            if let Some(&item_id) = box_items.front() {
                return Some(item_id);
            }
        }
        None
    }

    /// Get the box number for a specific item.
    pub fn get_item_box(&self, item_id: usize) -> Option<usize> {
        self.item_locations.get(item_id).copied()
    }

    /// Get the number of items in each box.
    pub fn get_box_counts(&self) -> Vec<usize> {
        self.boxes.iter().map(|b| b.len()).collect()
    }

    /// Get total number of items remaining to review.
    pub fn remaining_items(&self) -> usize {
        self.boxes.iter().map(|b| b.len()).sum()
    }

    /// Check if all items have been mastered (in the last box).
    pub fn all_mastered(&self) -> bool {
        self.boxes[self.num_boxes - 1].len() == self.item_locations.len()
    }

    /// Reset all items back to box 0.
    pub fn reset(&mut self) {
        for box_items in &mut self.boxes {
            box_items.clear();
        }

        for (i, location) in self.item_locations.iter_mut().enumerate() {
            self.boxes[0].push_back(i);
            *location = 0;
        }
    }

    /// Get a summary of the current state.
    pub fn summary(&self) -> LeitnerSummary {
        let counts = self.get_box_counts();
        let total = self.item_locations.len();
        let mastered = counts.last().copied().unwrap_or(0);
        let in_progress = total - mastered;

        LeitnerSummary {
            total_items: total,
            mastered_items: mastered,
            in_progress_items: in_progress,
            box_counts: counts,
        }
    }
}

/// Summary of the Leitner box state.
#[derive(Debug, Clone)]
pub struct LeitnerSummary {
    pub total_items: usize,
    pub mastered_items: usize,
    pub in_progress_items: usize,
    pub box_counts: Vec<usize>,
}

impl LeitnerSummary {
    /// Calculate mastery percentage.
    pub fn mastery_percentage(&self) -> f64 {
        if self.total_items == 0 {
            return 0.0;
        }
        (self.mastered_items as f64 / self.total_items as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leitner_basic() {
        let mut leitner = LeitnerBox::new(5, 10);

        // All items start in box 0
        assert_eq!(leitner.get_item_box(0), Some(0));
        assert_eq!(leitner.get_box_counts()[0], 10);
    }

    #[test]
    fn test_correct_answer_progression() {
        let mut leitner = LeitnerBox::new(5, 10);

        // Answer item 0 correctly
        leitner.answer_correct(0);
        assert_eq!(leitner.get_item_box(0), Some(1));

        // Answer again
        leitner.answer_correct(0);
        assert_eq!(leitner.get_item_box(0), Some(2));
    }

    #[test]
    fn test_incorrect_answer_reset() {
        let mut leitner = LeitnerBox::new(5, 10);

        // Move item to box 2
        leitner.answer_correct(0);
        leitner.answer_correct(0);
        assert_eq!(leitner.get_item_box(0), Some(2));

        // Answer incorrectly - should go back to box 0
        leitner.answer_incorrect(0);
        assert_eq!(leitner.get_item_box(0), Some(0));
    }

    #[test]
    fn test_get_next_item() {
        let mut leitner = LeitnerBox::new(5, 10);

        // Move some items to different boxes
        leitner.answer_correct(0);
        leitner.answer_correct(1);
        leitner.answer_correct(1);

        // Next item should be from box 0 (lowest priority)
        let next = leitner.get_next_item().unwrap();
        assert!(next >= 2); // Items 0 and 1 are in higher boxes
    }

    #[test]
    fn test_mastery() {
        let mut leitner = LeitnerBox::new(3, 5);

        // Move all items to last box
        for i in 0..5 {
            leitner.answer_correct(i);
            leitner.answer_correct(i);
        }

        assert!(leitner.all_mastered());

        let summary = leitner.summary();
        assert_eq!(summary.mastered_items, 5);
        assert_eq!(summary.mastery_percentage(), 100.0);
    }

    #[test]
    fn test_reset() {
        let mut leitner = LeitnerBox::new(5, 10);

        // Move some items
        leitner.answer_correct(0);
        leitner.answer_correct(1);

        // Reset
        leitner.reset();

        // All items should be back in box 0
        assert_eq!(leitner.get_box_counts()[0], 10);
        for i in 0..10 {
            assert_eq!(leitner.get_item_box(i), Some(0));
        }
    }
}
