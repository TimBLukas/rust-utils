//! Rust Util Tools - Main entry point
//!
//! This is the CLI application entry point using clap for argument parsing.

use anyhow::Result;
use clap::{Parser, Subcommand};
use rust_util_tools::core::{Config, Difficulty, Language};
use rust_util_tools::modules::learning;
use rust_util_tools::modules::typing::{HighScoreManager, WordLoader};
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
            language,
            difficulty,
        } => {
            run_typing_demo(&config, language, difficulty)?;
        }
        Commands::Learn { set, spaced } => {
            run_learning_demo(&config, set, spaced)?;
        }
        Commands::Stats {
            language,
            difficulty,
        } => {
            show_statistics(&config, language, difficulty)?;
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
            run_demo(&config)?;
        }
    }

    Ok(())
}

/// Run a typing test demo (simplified version for now)
fn run_typing_demo(
    config: &Config,
    language: Option<String>,
    difficulty: Option<String>,
) -> Result<()> {
    println!("\n╔════════════════════════════════════════════════╗");
    println!("║           TYPING SPEED TEST DEMO              ║");
    println!("╚════════════════════════════════════════════════╝\n");

    // Parse language and difficulty
    let lang = language
        .unwrap_or_else(|| config.defaults.language.clone())
        .parse::<Language>()
        .unwrap_or(Language::English);

    let diff = difficulty
        .unwrap_or_else(|| config.defaults.difficulty.clone())
        .parse::<Difficulty>()
        .unwrap_or(Difficulty::Medium);

    println!("Language: {}", lang);
    println!("Difficulty: {}", diff);

    // Load words
    let loader = WordLoader::new(&config.paths.data_dir);
    match loader.generate_text(lang, diff) {
        Ok(text) => {
            println!(
                "\nGenerated text ({} words):",
                text.split_whitespace().count()
            );
            println!("{}\n", text);
            println!("Note: Full typing test UI will be implemented with ratatui.");
        }
        Err(e) => {
            eprintln!("Error loading words: {}", e);
            eprintln!(
                "\nMake sure word files are in: {}",
                config.paths.data_dir.display()
            );
        }
    }

    Ok(())
}

/// Run a learning mode demo
fn run_learning_demo(config: &Config, set_path: PathBuf, use_spaced: bool) -> Result<()> {
    println!("\n╔════════════════════════════════════════════════╗");
    println!("║           LEARNING MODE DEMO                   ║");
    println!("╚════════════════════════════════════════════════╝\n");

    // Load learning set
    let set = learning::load_auto(&set_path)?;

    println!("Loaded: {}", set.name);
    if !set.description.is_empty() {
        println!("Description: {}", set.description);
    }
    println!("Cards: {}", set.cards.len());
    println!("Questions: {}", set.questions.len());
    println!(
        "Spaced repetition: {}",
        if use_spaced { "enabled" } else { "disabled" }
    );

    // Demo fuzzy matching
    if !set.cards.is_empty() {
        println!("\n--- Fuzzy Matching Demo ---");
        let matcher = learning::FuzzyMatcher::new(config.learning.fuzzy_threshold, 0.10);

        let card = &set.cards[0];
        println!("Card: {}", card.front);
        println!("Correct answer: {}", card.back);

        // Test exact match
        let result = matcher.check_answer(&card.back, &card.back);
        println!("Exact match result: {:?}", result);

        // Test close match
        let close = format!("{}x", &card.back); // Add one char
        let result = matcher.check_answer(&close, &card.back);
        println!("Close match result: {:?}", result);
    }

    println!("\nNote: Full learning UI will be implemented with ratatui.");

    Ok(())
}

/// Show statistics
fn show_statistics(
    config: &Config,
    language: Option<String>,
    difficulty: Option<String>,
) -> Result<()> {
    println!("\n╔════════════════════════════════════════════════╗");
    println!("║           STATISTICS                           ║");
    println!("╚════════════════════════════════════════════════╝\n");

    let manager =
        HighScoreManager::new(&config.paths.highscore_file, config.defaults.max_highscores);

    // Parse filters
    let lang_filter = language.and_then(|l| l.parse::<Language>().ok());
    let diff_filter = difficulty.and_then(|d| d.parse::<Difficulty>().ok());

    // Get filtered scores
    let scores = manager.get_filtered(lang_filter, diff_filter)?;

    if scores.is_empty() {
        println!("No highscores found.");
        return Ok(());
    }

    println!("Top Highscores:");
    println!("{}", "─".repeat(80));
    for (i, score) in scores.iter().take(10).enumerate() {
        println!(
            "{}. {} - {:.1} WPM ({:.1}%) [{}] [{}] - {}",
            i + 1,
            score.name,
            score.wpm,
            score.accuracy,
            score.difficulty,
            score.language.to_uppercase(),
            score.timestamp
        );
    }

    // Show statistics
    let stats = manager.get_statistics()?;
    println!("\n{}", "─".repeat(80));
    println!("Total tests: {}", stats.total_tests);
    println!("Average WPM: {:.1}", stats.avg_wpm);
    println!("Average accuracy: {:.1}%", stats.avg_accuracy);
    println!("Best WPM: {:.1}", stats.best_wpm);
    println!("\nTests by difficulty:");
    println!("  Easy:   {}", stats.easy_count);
    println!("  Medium: {}", stats.medium_count);
    println!("  Hard:   {}", stats.hard_count);

    Ok(())
}

/// Run a comprehensive demo
fn run_demo(config: &Config) -> Result<()> {
    println!("\n╔════════════════════════════════════════════════╗");
    println!("║     RUST UTIL TOOLS - COMPREHENSIVE DEMO      ║");
    println!("╚════════════════════════════════════════════════╝\n");

    println!("This demo showcases the refactored architecture:\n");

    // 1. Configuration demo
    println!("1. Configuration System");
    println!("   ✓ Loaded from: config/default.toml (or defaults)");
    println!("   ✓ Data directory: {}", config.paths.data_dir.display());
    println!("   ✓ Fuzzy threshold: {}", config.learning.fuzzy_threshold);

    // 2. Word loading demo
    println!("\n2. Word Loading (with caching)");
    let loader = WordLoader::new(&config.paths.data_dir);
    match loader.load_words(Language::English, Difficulty::Easy) {
        Ok(words) => {
            println!(
                "   ✓ Loaded {} English words (Easy difficulty)",
                words.len()
            );
            println!(
                "   ✓ Sample: {}",
                words.iter().take(5).cloned().collect::<Vec<_>>().join(", ")
            );
        }
        Err(e) => {
            println!("   ✗ Error: {}", e);
        }
    }

    // 3. Fuzzy matching demo
    println!("\n3. Fuzzy Matching System");
    let matcher = learning::FuzzyMatcher::new(0.85, 0.10);
    let test_pairs = vec![
        ("photosynthesis", "photosynthesis"),
        ("photosynthesis", "photosynthesys"),
        ("cat", "dog"),
    ];

    for (input, correct) in test_pairs {
        let result = matcher.check_answer(input, correct);
        println!("   '{}' vs '{}': {:?}", input, correct, result);
    }

    // 4. Spaced repetition demo
    println!("\n4. Spaced Repetition (Leitner Box)");
    let mut leitner = learning::LeitnerBox::new(5, 10);
    println!("   ✓ Created Leitner system with 5 boxes, 10 items");
    leitner.answer_correct(0);
    leitner.answer_correct(0);
    println!("   ✓ Item 0 moved to box: {:?}", leitner.get_item_box(0));
    let summary = leitner.summary();
    println!("   ✓ Mastery: {:.1}%", summary.mastery_percentage());

    println!("\n╔════════════════════════════════════════════════╗");
    println!("║  All core systems operational! ✓               ║");
    println!("╚════════════════════════════════════════════════╝\n");

    println!("Next steps:");
    println!("  • Implement full TUI with ratatui");
    println!("  • Add interactive typing test");
    println!("  • Add interactive learning mode");
    println!("  • Add statistics dashboard");

    Ok(())
}
