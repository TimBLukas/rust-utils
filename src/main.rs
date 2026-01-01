//! Rust Util Tools - Main entry point
//!
//! This is the CLI application entry point using clap for argument parsing.

use anyhow::Result;
use clap::{Parser, Subcommand};
use rust_util_tools::core::Config;
use std::path::PathBuf;

/// Rust Util Tools - All-in-One Learning & Utility CLI Suite
#[derive(Parser)]
#[command(name = "rut")]
#[command(author = "Rust Util Tools Contributors")]
#[command(version = "0.2.0")]
#[command(about = "All-in-One Learning & Utility CLI Suite", long_about = None)]
struct Cli {
    /// Path to configuration file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a typing speed test
    Typing {
        /// Language (de/en)
        #[arg(short, long, value_name = "LANG")]
        language: Option<String>,

        /// Difficulty (easy/medium/hard)
        #[arg(short, long, value_name = "DIFF")]
        difficulty: Option<String>,
    },

    /// Start learning mode with flashcards or quizzes
    Learn {
        /// Path to learning set file
        #[arg(value_name = "FILE")]
        set: PathBuf,

        /// Enable spaced repetition
        #[arg(short, long)]
        spaced: bool,
    },

    /// Show statistics and highscores
    Stats {
        /// Filter by language
        #[arg(short, long)]
        language: Option<String>,

        /// Filter by difficulty
        #[arg(short, long)]
        difficulty: Option<String>,
    },

    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Run demo/example
    Demo,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show current configuration
    Show,
    /// Initialize default configuration file
    Init,
    /// Validate configuration
    Validate,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let config = if let Some(config_path) = cli.config {
        Config::load_from_file(config_path)?
    } else {
        Config::load_or_default()
    };

    if cli.verbose {
        println!("Loaded configuration: {:?}", config);
    }

    // Validate configuration
    config.validate()?;

    // Execute command
    match cli.command {
        Commands::Typing {
            language: _language,
            difficulty: _difficulty,
        } => {
            // TODO: Pass language/difficulty to TUI
            rust_util_tools::ui::run(config, rust_util_tools::ui::app::CurrentScreen::TypingTest)?;
        }
        Commands::Learn { set: _set, spaced: _spaced } => {
            // TODO: Pass set/spaced to TUI
            rust_util_tools::ui::run(config, rust_util_tools::ui::app::CurrentScreen::LearningMode)?;
        }
        Commands::Stats {
            language: _language,
            difficulty: _difficulty,
        } => {
            rust_util_tools::ui::run(config, rust_util_tools::ui::app::CurrentScreen::Statistics)?;
        }
        Commands::Config { action } => match action {
            ConfigAction::Show => {
                println!("{:#?}", config);
            }
            ConfigAction::Init => {
                config.save_to_file("config/default.toml")?;
                println!("✓ Configuration file created at config/default.toml");
            }
            ConfigAction::Validate => {
                config.validate()?;
                println!("✓ Configuration is valid");
            }
        },
        Commands::Demo => {
            // Run TUI Main Menu
            rust_util_tools::ui::run(config, rust_util_tools::ui::app::CurrentScreen::Menu)?;
        }
    }

    Ok(())
}


