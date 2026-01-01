//! Parsers for loading learning sets from various formats.
//!
//! This module provides parsers for JSON, CSV, and Markdown formats.

use crate::core::{Result, UtilError};
use crate::modules::learning::models::{Card, LearningSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Load a learning set from a JSON file.
///
/// # Arguments
///
/// * `path` - Path to the JSON file
///
/// # Returns
///
/// A `LearningSet` parsed from the file.
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
///
/// # Example JSON Format
///
/// ```json
/// {
///   "name": "Biology Basics",
///   "description": "Fundamental biology concepts",
///   "cards": [
///     {
///       "front": "What is photosynthesis?",
///       "back": "Process by which plants convert light into energy",
///       "tags": ["biology", "plants"]
///     }
///   ],
///   "questions": [
///     {
///       "question": "What is the powerhouse of the cell?",
///       "correct_answer": "Mitochondria",
///       "alternatives": ["Nucleus", "Ribosome", "Chloroplast"]
///     }
///   ]
/// }
/// ```
pub fn load_from_json<P: AsRef<Path>>(path: P) -> Result<LearningSet> {
    let path = path.as_ref();
    let file = File::open(path).map_err(|e| UtilError::LearningSetLoadError {
        path: path.to_path_buf(),
        source: e,
    })?;

    let reader = BufReader::new(file);
    let set: LearningSet = serde_json::from_reader(reader)?;

    if set.is_empty() {
        return Err(UtilError::InvalidLearningSetFormat {
            path: path.to_path_buf(),
            reason: "Learning set contains no cards or questions".to_string(),
        });
    }

    Ok(set)
}

/// Load flashcards from a simple CSV file.
///
/// # CSV Format
///
/// ```csv
/// front,back,tags
/// "What is DNA?","Deoxyribonucleic acid","biology,genetics"
/// "Capital of France?","Paris","geography"
/// ```
///
/// # Arguments
///
/// * `path` - Path to the CSV file
/// * `name` - Name for the learning set
///
/// # Returns
///
/// A `LearningSet` with cards parsed from the CSV.
pub fn load_cards_from_csv<P: AsRef<Path>>(path: P, name: String) -> Result<LearningSet> {
    let path = path.as_ref();
    let file = File::open(path).map_err(|e| UtilError::LearningSetLoadError {
        path: path.to_path_buf(),
        source: e,
    })?;

    let reader = BufReader::new(file);
    let mut cards = Vec::new();
    let mut lines = reader.lines();

    // Skip header
    if let Some(Ok(_header)) = lines.next() {
        // Process data lines
        for (line_num, line) in lines.enumerate() {
            let line = line.map_err(|e| UtilError::LearningSetLoadError {
                path: path.to_path_buf(),
                source: e,
            })?;

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 2 {
                return Err(UtilError::InvalidLearningSetFormat {
                    path: path.to_path_buf(),
                    reason: format!("Invalid CSV format at line {}", line_num + 2),
                });
            }

            let front = parts[0].trim().trim_matches('"').to_string();
            let back = parts[1].trim().trim_matches('"').to_string();
            let tags = if parts.len() > 2 {
                parts[2]
                    .trim()
                    .trim_matches('"')
                    .split(';')
                    .map(|s| s.trim().to_string())
                    .collect()
            } else {
                Vec::new()
            };

            cards.push(Card {
                front,
                back,
                tags,
                explanation: None,
            });
        }
    }

    if cards.is_empty() {
        return Err(UtilError::InvalidLearningSetFormat {
            path: path.to_path_buf(),
            reason: "No cards found in CSV file".to_string(),
        });
    }

    Ok(LearningSet {
        name,
        description: String::new(),
        cards,
        questions: Vec::new(),
        tags: Vec::new(),
    })
}

/// Load flashcards from a simple Markdown file.
///
/// # Markdown Format
///
/// ```markdown
/// # Learning Set Name
///
/// ## Card 1
/// **Front:** What is photosynthesis?
/// **Back:** Process by which plants convert light into energy
///
/// ## Card 2
/// **Front:** What is the capital of France?
/// **Back:** Paris
/// ```
///
/// # Arguments
///
/// * `path` - Path to the Markdown file
///
/// # Returns
///
/// A `LearningSet` with cards parsed from the Markdown.
pub fn load_from_markdown<P: AsRef<Path>>(path: P) -> Result<LearningSet> {
    let path = path.as_ref();
    let file = File::open(path).map_err(|e| UtilError::LearningSetLoadError {
        path: path.to_path_buf(),
        source: e,
    })?;

    let reader = BufReader::new(file);
    let mut cards = Vec::new();
    let mut name = String::from("Unnamed Set");
    let mut current_front: Option<String> = None;

    for line in reader.lines() {
        let line = line.map_err(|e| UtilError::LearningSetLoadError {
            path: path.to_path_buf(),
            source: e,
        })?;

        let trimmed = line.trim();

        // Parse title
        if trimmed.starts_with("# ") {
            name = trimmed[2..].to_string();
        }
        // Parse front
        else if trimmed.starts_with("**Front:**") || trimmed.starts_with("Front:") {
            let front_text = trimmed
                .trim_start_matches("**Front:**")
                .trim_start_matches("Front:")
                .trim()
                .to_string();
            current_front = Some(front_text);
        }
        // Parse back
        else if (trimmed.starts_with("**Back:**") || trimmed.starts_with("Back:"))
            && current_front.is_some()
        {
            let back_text = trimmed
                .trim_start_matches("**Back:**")
                .trim_start_matches("Back:")
                .trim()
                .to_string();

            if let Some(front) = current_front.take() {
                cards.push(Card {
                    front,
                    back: back_text,
                    tags: Vec::new(),
                    explanation: None,
                });
            }
        }
    }

    if cards.is_empty() {
        return Err(UtilError::InvalidLearningSetFormat {
            path: path.to_path_buf(),
            reason: "No cards found in Markdown file".to_string(),
        });
    }

    Ok(LearningSet {
        name,
        description: String::new(),
        cards,
        questions: Vec::new(),
        tags: Vec::new(),
    })
}

/// Auto-detect format and load learning set.
///
/// Detects format based on file extension.
pub fn load_auto<P: AsRef<Path>>(path: P) -> Result<LearningSet> {
    let path = path.as_ref();
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match extension.to_lowercase().as_str() {
        "json" => load_from_json(path),
        "csv" => {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unnamed")
                .to_string();
            load_cards_from_csv(path, name)
        }
        "md" | "markdown" => load_from_markdown(path),
        _ => Err(UtilError::InvalidLearningSetFormat {
            path: path.to_path_buf(),
            reason: format!("Unsupported file extension: {}", extension),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_from_json() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let json_content = r#"{
            "name": "Test Set",
            "description": "A test learning set",
            "cards": [
                {
                    "front": "Question 1",
                    "back": "Answer 1",
                    "tags": ["test"]
                }
            ],
            "questions": []
        }"#;

        temp_file.write_all(json_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let set = load_from_json(temp_file.path()).unwrap();
        assert_eq!(set.name, "Test Set");
        assert_eq!(set.cards.len(), 1);
        assert_eq!(set.cards[0].front, "Question 1");
    }

    #[test]
    fn test_load_from_csv() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let csv_content = "front,back,tags\n\"Q1\",\"A1\",\"tag1;tag2\"\n\"Q2\",\"A2\",\"\"";

        temp_file.write_all(csv_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let set = load_cards_from_csv(temp_file.path(), "Test".to_string()).unwrap();
        assert_eq!(set.cards.len(), 2);
        assert_eq!(set.cards[0].tags.len(), 2);
    }
}
