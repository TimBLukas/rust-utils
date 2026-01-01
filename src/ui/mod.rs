pub mod app;
pub mod render;
pub mod tui;

use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use std::time::Duration;

use crate::core::Config;
use crate::modules::typing::{WordLoader, TestResult, scorer};
use app::{App, CurrentScreen};
use std::time::Instant;

/// Run the TUI application
pub fn run(config: Config, initial_screen: CurrentScreen) -> Result<()> {
    // Initialize terminal
    let mut terminal = tui::init()?;
    
    // Create app state
    let mut app = App::new(config);
    app.current_screen = initial_screen;

    // Main loop
    loop {
        // Render
        terminal.draw(|frame| {
            render::render(&mut app, frame);
        })?;

        // Handle events
        if let Some(event) = tui::read_event(Duration::from_millis(16))? {
            match event {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        handle_key_event(&mut app, key)?;
                    }
                }
                _ => {}
            }
        }

        // Check exit condition
        if app.exit {
            break;
        }
    }

    // Restore terminal
    tui::restore()?;

    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<()> {
    match app.current_screen {
        CurrentScreen::Menu => handle_menu_input(app, key),
        CurrentScreen::TypingTest => handle_typing_input(app, key),
        CurrentScreen::LearningSelect => handle_learning_select_input(app, key),
        CurrentScreen::LearningMode => handle_learning_mode_input(app, key),
        CurrentScreen::Settings => handle_settings_input(app, key),
        CurrentScreen::TypingResults => {
            if key.code == KeyCode::Enter || key.code == KeyCode::Esc {
                app.current_screen = CurrentScreen::Menu;
            }
        }
        CurrentScreen::Statistics => {
             if key.code == KeyCode::Esc {
                app.current_screen = CurrentScreen::Menu;
            }
        }
        _ => {
            if key.code == KeyCode::Esc {
                app.current_screen = CurrentScreen::Menu;
            }
        }
    }
    Ok(())
}

fn handle_learning_select_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.current_screen = CurrentScreen::Menu,
        KeyCode::Up => {
            if app.file_explorer_state.selected_index > 0 {
                app.file_explorer_state.selected_index -= 1;
            }
        }
        KeyCode::Down => {
            if app.file_explorer_state.selected_index < app.file_explorer_state.files.len().saturating_sub(1) {
                app.file_explorer_state.selected_index += 1;
            }
        }
        KeyCode::Enter => {
            if let Some(path) = app.file_explorer_state.files.get(app.file_explorer_state.selected_index) {
                if path.is_dir() {
                    // Navigate into directory
                    app.file_explorer_state.current_dir = path.clone();
                    app.file_explorer_state.selected_index = 0;
                    refresh_file_list(app);
                } else {
                    // Load file
                    if let Ok(set) = crate::modules::learning::load_auto(path) {
                        app.learning_state = app::LearningState::default();
                        app.learning_state.set = Some(set);
                        app.current_screen = CurrentScreen::LearningMode;
                    }
                }
            }
        }
        _ => {}
    }
}

fn handle_learning_mode_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.current_screen = CurrentScreen::Menu,
        KeyCode::Char(c) => {
            if !app.learning_state.show_back {
                app.learning_state.user_input.push(c);
            }
        }
        KeyCode::Backspace => {
            if !app.learning_state.show_back {
                app.learning_state.user_input.pop();
            }
        }
        KeyCode::Enter => {
            if app.learning_state.show_back {
                // Next card
                app.learning_state.current_card_index += 1;
                app.learning_state.user_input.clear();
                app.learning_state.show_back = false;
                app.learning_state.match_result = None;
            } else {
                // Submit answer
                if let Some(set) = &app.learning_state.set {
                    if let Some(card) = set.cards.get(app.learning_state.current_card_index) {
                        let matcher = crate::modules::learning::FuzzyMatcher::new(
                            app.config.learning.fuzzy_threshold,
                            0.10
                        );
                        let result = matcher.check_answer(&app.learning_state.user_input, &card.back);
                        app.learning_state.match_result = Some(result);
                        app.learning_state.show_back = true;
                    }
                }
            }
        }
        _ => {}
    }
}

fn handle_settings_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.current_screen = CurrentScreen::Menu,
        KeyCode::Char('l') => {
            app.config.defaults.language = match app.config.defaults.language.as_str() {
                "en" => "de".to_string(),
                _ => "en".to_string(),
            };
        }
        KeyCode::Char('d') => {
            app.config.defaults.difficulty = match app.config.defaults.difficulty.as_str() {
                "Easy" => "Medium".to_string(),
                "Medium" => "Hard".to_string(),
                _ => "Easy".to_string(),
            };
        }
        KeyCode::Char('s') => {
            if let Err(e) = app.config.save_to_file("config/default.toml") {
                // TODO: Show error in UI
                eprintln!("Failed to save config: {}", e);
            }
        }
        _ => {}
    }
}

fn refresh_file_list(app: &mut App) {
    app.file_explorer_state.files.clear();
    // Add parent directory if not at root
    if let Some(parent) = app.file_explorer_state.current_dir.parent() {
        app.file_explorer_state.files.push(parent.to_path_buf());
    }
    
    if let Ok(entries) = std::fs::read_dir(&app.file_explorer_state.current_dir) {
        for entry in entries.flatten() {
            app.file_explorer_state.files.push(entry.path());
        }
    }
    // Sort: directories first, then files
    app.file_explorer_state.files.sort_by(|a, b| {
        if a.is_dir() && !b.is_dir() {
            std::cmp::Ordering::Less
        } else if !a.is_dir() && b.is_dir() {
            std::cmp::Ordering::Greater
        } else {
            a.cmp(b)
        }
    });
}

fn handle_menu_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.exit = true;
        }
        KeyCode::Up => {
            if app.menu_cursor > 0 {
                app.menu_cursor -= 1;
            } else {
                app.menu_cursor = app.menu_items.len() - 1;
            }
        }
        KeyCode::Down => {
            if app.menu_cursor < app.menu_items.len() - 1 {
                app.menu_cursor += 1;
            } else {
                app.menu_cursor = 0;
            }
        }
        KeyCode::Enter => {
            match app.menu_cursor {
                0 => { // Typing Test
                    app.reset_typing();
                    // Load words (simplified for now)
                    let loader = WordLoader::new(&app.config.paths.data_dir);
                    if let Ok(text) = loader.generate_text(app.typing_state.language.clone(), app.typing_state.difficulty.clone()) {
                        app.typing_state.target_text = text;
                        app.current_screen = CurrentScreen::TypingTest;
                    }
                }
                1 => { // Learning Mode
                    app.current_screen = CurrentScreen::LearningSelect;
                    app.file_explorer_state.current_dir = std::env::current_dir().unwrap_or_default();
                    refresh_file_list(app);
                }
                2 => { // Statistics
                    app.current_screen = CurrentScreen::Statistics;
                    // Load statistics
                    let manager = crate::modules::typing::HighScoreManager::new(
                        &app.config.paths.highscore_file,
                        app.config.defaults.max_highscores
                    );
                    if let Ok(scores) = manager.load() {
                        app.statistics_state.highscores = scores;
                    }
                }
                3 => { // Settings
                    app.current_screen = CurrentScreen::Settings;
                }
                4 => { // Quit
                    app.exit = true;
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn handle_typing_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.current_screen = CurrentScreen::Menu;
            app.typing_state.is_active = false;
        }
        KeyCode::Char(c) => {
            if !app.typing_state.is_active {
                app.typing_state.is_active = true;
                app.typing_state.start_time = Some(Instant::now());
            }
            
            app.typing_state.typed_text.push(c);
            
            // Check for error (simplified: just checking if char matches target at that position)
            let idx = app.typing_state.typed_text.len() - 1;
            if let Some(target_char) = app.typing_state.target_text.chars().nth(idx) {
                if c != target_char {
                    app.typing_state.error_count += 1;
                }
            } else {
                // Typed beyond target
                app.typing_state.error_count += 1;
            }

            check_typing_completion(app);
        }
        KeyCode::Backspace => {
            app.typing_state.typed_text.pop();
        }
        _ => {}
    }
}

fn check_typing_completion(app: &mut App) {
    if app.typing_state.typed_text.len() >= app.typing_state.target_text.len() {
        app.typing_state.end_time = Some(Instant::now());
        app.typing_state.is_active = false;
        
        // Calculate results
        let duration = app.typing_state.end_time.unwrap().duration_since(app.typing_state.start_time.unwrap());
        let result = TestResult::calculate(
            &app.typing_state.target_text,
            &app.typing_state.typed_text,
            duration,
            app.typing_state.error_count
        );
        
        // Save score
        let manager = crate::modules::typing::HighScoreManager::new(
            &app.config.paths.highscore_file,
            app.config.defaults.max_highscores
        );
        
        let score = crate::modules::typing::HighScore::from_result(
            "Player".to_string(), // TODO: Get name
            &result,
            app.typing_state.language.clone(),
            app.typing_state.difficulty.clone()
        );
        
        let _ = manager.add_score(score); // Ignore error for now
        
        app.typing_state.result = Some(result);
        app.current_screen = CurrentScreen::TypingResults;
    }
}
