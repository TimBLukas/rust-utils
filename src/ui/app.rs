use crate::core::{Config, Difficulty, Language};
use crate::modules::learning::{LearningSet, MatchResult};
use crate::modules::typing::{HighScoreManager, TestResult, WordLoader};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurrentScreen {
    Menu,
    TypingTest,
    TypingResults,
    LearningSelect,
    LearningMode,
    LearningResults,
    Statistics,
    Settings,
    Exiting,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub config: Config,
    pub exit: bool,
    
    // Menu state
    pub menu_cursor: usize,
    pub menu_items: Vec<&'static str>,

    // Typing Test State
    pub typing_state: TypingState,
    
    // Learning Mode State
    pub learning_state: LearningState,

    // File Explorer State
    pub file_explorer_state: FileExplorerState,

    // Statistics State
    pub statistics_state: StatisticsState,
}

pub struct FileExplorerState {
    pub current_dir: std::path::PathBuf,
    pub files: Vec<std::path::PathBuf>,
    pub selected_index: usize,
    pub error: Option<String>,
}

impl Default for FileExplorerState {
    fn default() -> Self {
        Self {
            current_dir: std::env::current_dir().unwrap_or_default(),
            files: Vec::new(),
            selected_index: 0,
            error: None,
        }
    }
}

pub struct StatisticsState {
    pub highscores: Vec<crate::modules::typing::HighScore>,
    pub stats_summary: Option<crate::modules::typing::HighScoreStatistics>,
}

impl Default for StatisticsState {
    fn default() -> Self {
        Self {
            highscores: Vec::new(),
            stats_summary: None,
        }
    }
}

pub struct TypingState {
    pub language: Language,
    pub difficulty: Difficulty,
    pub target_text: String,
    pub typed_text: String,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub error_count: usize,
    pub is_active: bool,
    pub result: Option<TestResult>,
}

impl Default for TypingState {
    fn default() -> Self {
        Self {
            language: Language::English,
            difficulty: Difficulty::Medium,
            target_text: String::new(),
            typed_text: String::new(),
            start_time: None,
            end_time: None,
            error_count: 0,
            is_active: false,
            result: None,
        }
    }
}

pub struct LearningState {
    pub set: Option<LearningSet>,
    pub current_card_index: usize,
    pub user_input: String,
    pub show_back: bool,
    pub match_result: Option<MatchResult>,
    pub correct_count: usize,
    pub total_count: usize,
}

impl Default for LearningState {
    fn default() -> Self {
        Self {
            set: None,
            current_card_index: 0,
            user_input: String::new(),
            show_back: false,
            match_result: None,
            correct_count: 0,
            total_count: 0,
        }
    }
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            current_screen: CurrentScreen::Menu,
            config,
            exit: false,
            menu_cursor: 0,
            menu_items: vec![
                "Typing Test",
                "Learning Mode",
                "Statistics",
                "Settings",
                "Quit",
            ],
            typing_state: TypingState::default(),
            learning_state: LearningState::default(),
            file_explorer_state: FileExplorerState::default(),
            statistics_state: StatisticsState::default(),
        }
    }

    pub fn reset_typing(&mut self) {
        self.typing_state = TypingState::default();
        // Preserve config defaults if needed, but for now reset to defaults
        self.typing_state.language = self.config.defaults.language.parse().unwrap_or(Language::English);
        self.typing_state.difficulty = self.config.defaults.difficulty.parse().unwrap_or(Difficulty::Medium);
    }
}
