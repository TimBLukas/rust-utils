//! Core module containing fundamental types, errors, and configuration.
//!
//! This module provides the foundational building blocks used throughout
//! the rust-util-tools application.

pub mod config;
pub mod error;
pub mod types;

// Re-export commonly used items
pub use config::Config;
pub use error::{Result, UtilError};
pub use types::{Difficulty, Language};
