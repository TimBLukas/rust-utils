# Rust Util Tools (rut)

**All-in-One Learning & Utility CLI Suite**

A modern, modular Rust application combining typing speed tests, intelligent learning systems with flashcards and quizzes, and various utility features.

## Features

### 🎯 Typing Speed Test
- Multi-language support (German, English)
- Multiple difficulty levels with CEFR-based word filtering
- Real-time WPM and accuracy calculation
- Persistent highscore tracking
- Different test modes (time-based, word-count, code snippets)

### 🧠 Learning System
- Flashcard and quiz support
- **Fuzzy matching** for answer validation with user override
- **Spaced repetition** using the Leitner box algorithm
- Multiple format support (JSON, CSV, Markdown)
- Session management and progress tracking

### 📊 Statistics & Analytics
- Comprehensive highscore management
- Performance tracking over time
- Filterable statistics by language and difficulty

## Architecture

This project follows idiomatic Rust patterns with a clean, modular architecture:

```
src/
├── core/           # Core types, errors, configuration
│   ├── error.rs    # Custom error types (thiserror)
│   ├── types.rs    # Type-safe enums (Language, Difficulty)
│   └── config.rs   # TOML configuration management
├── modules/
│   ├── typing/     # Typing test logic
│   │   ├── word_loader.rs   # Word loading with caching
│   │   ├── scorer.rs        # WPM/accuracy calculation
│   │   └── highscore.rs     # Highscore management
│   └── learning/   # Learning system
│       ├── models.rs        # Data structures
│       ├── fuzzy.rs         # Fuzzy string matching
│       ├── spaced_rep.rs    # Leitner box algorithm
│       └── parsers.rs       # JSON/CSV/MD parsers
├── ui/             # Terminal UI (ratatui)
└── utils/          # Helper utilities
```

## Installation

### Prerequisites
- Rust 1.70+ (edition 2021)
- Cargo

### Build from source

```bash
git clone <repository-url>
cd rust-typing-test
cargo build --release
```

The binary will be available at `target/release/rut`.

## Usage

### Quick Start

Run the comprehensive demo:
```bash
cargo run -- demo
```

### Typing Test

Start a typing test:
```bash
# Default settings
cargo run -- typing

# Specify language and difficulty
cargo run -- typing --language en --difficulty hard
cargo run -- typing -l de -d easy
```

### Learning Mode

Start a learning session:
```bash
# Load a learning set
cargo run -- learn data/learning_sets/biology_basics.json

# Enable spaced repetition
cargo run -- learn data/learning_sets/biology_basics.json --spaced
```

### Statistics

View highscores and statistics:
```bash
# All statistics
cargo run -- stats

# Filter by language
cargo run -- stats --language en

# Filter by difficulty
cargo run -- stats --difficulty hard
```

### Configuration

```bash
# Show current configuration
cargo run -- config show

# Initialize default config file
cargo run -- config init

# Validate configuration
cargo run -- config validate
```

## Configuration

Configuration is stored in `config/default.toml`:

```toml
[paths]
data_dir = "data"
highscore_file = "data/highscores.json"
learning_sets_dir = "data/learning_sets"

[theme]
correct_color = "green"
error_color = "red"
animations = true

[defaults]
language = "en"
difficulty = "medium"
min_accuracy_for_highscore = 80.0

[learning]
fuzzy_threshold = 0.85
spaced_repetition = true
leitner_boxes = 5
```

## Learning Set Formats

### JSON Format

```json
{
  "name": "Biology Basics",
  "description": "Fundamental biology concepts",
  "cards": [
    {
      "front": "What is photosynthesis?",
      "back": "Process converting light to energy",
      "tags": ["biology", "plants"]
    }
  ],
  "questions": [
    {
      "question": "What is DNA?",
      "correct_answer": "Deoxyribonucleic acid",
      "alternatives": ["RNA", "Protein", "Lipid"]
    }
  ]
}
```

### CSV Format

```csv
front,back,tags
"What is DNA?","Deoxyribonucleic acid","biology;genetics"
"Capital of France?","Paris","geography"
```

### Markdown Format

```markdown
# Biology Basics

## Card 1
**Front:** What is photosynthesis?
**Back:** Process by which plants convert light into energy

## Card 2
**Front:** What is DNA?
**Back:** Deoxyribonucleic acid
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific module tests
cargo test modules::typing
```

### Code Quality

```bash
# Run clippy
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

### Documentation

```bash
# Generate and open documentation
cargo doc --open
```

## Key Design Decisions

### Error Handling
- **No `unwrap()` or `panic!` in production code**
- Uses `thiserror` for library errors
- Uses `anyhow` for application-level errors
- Context-rich error messages

### Performance
- **Lazy loading** with `once_cell` for word caching
- Efficient shuffling with `rand::seq::SliceRandom`
- Minimal allocations and cloning

### Type Safety
- **Type-safe enums** instead of strings (Language, Difficulty)
- Compile-time guarantees for valid states
- Comprehensive validation

### Memory Safety
- All dependencies are pure Rust (no unsafe code)
- Proper ownership and borrowing
- No data races (enforced by Rust)

## Dependencies

| Crate | Purpose | Version |
|-------|---------|---------|
| `clap` | CLI argument parsing | 4.5 |
| `ratatui` | Terminal UI framework | 0.26 |
| `crossterm` | Terminal manipulation | 0.27 |
| `serde` | Serialization | 1.0 |
| `thiserror` | Error types | 1.0 |
| `anyhow` | Error handling | 1.0 |
| `strsim` | Fuzzy string matching | 0.11 |
| `once_cell` | Lazy initialization | 1.19 |

## Roadmap

- [x] Core architecture and error handling
- [x] Word loading with caching
- [x] Scoring system (WPM, accuracy)
- [x] Highscore management
- [x] Learning system data models
- [x] Fuzzy matching with user override
- [x] Spaced repetition (Leitner box)
- [x] Multiple format parsers
- [ ] Full TUI with ratatui
- [ ] Interactive typing test
- [ ] Interactive learning mode
- [ ] Statistics dashboard with charts
- [ ] Export/import functionality
- [ ] Code snippet typing mode
- [ ] Custom word list support

## License

MIT

## Contributing

Contributions are welcome! Please ensure:
- All tests pass
- Code is formatted with `cargo fmt`
- No clippy warnings
- Documentation is updated

---

**Built with ❤️ in Rust**
