use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::app::{App, CurrentScreen};

/// Render the application state
pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(frame.size());

    render_header(app, frame, chunks[0]);
    render_content(app, frame, chunks[1]);
    render_footer(app, frame, chunks[2]);
}

fn render_header(app: &App, frame: &mut Frame, area: Rect) {
    let title = match app.current_screen {
        CurrentScreen::Menu => " Main Menu ",
        CurrentScreen::TypingTest => " Typing Test ",
        CurrentScreen::TypingResults => " Test Results ",
        CurrentScreen::LearningSelect => " Select Learning Set ",
        CurrentScreen::LearningMode => " Learning Mode ",
        CurrentScreen::LearningResults => " Learning Results ",
        CurrentScreen::Statistics => " Statistics ",
        CurrentScreen::Settings => " Settings ",
        CurrentScreen::Exiting => " Exiting ",
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(Span::styled(
        format!(" Rust Util Tools - {} ", title),
        Style::default().add_modifier(Modifier::BOLD),
    ))
    .block(block)
    .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_footer(app: &App, frame: &mut Frame, area: Rect) {
    let help_text = match app.current_screen {
        CurrentScreen::Menu => "Use ↑/↓ to navigate, Enter to select, q to quit",
        CurrentScreen::TypingTest => "Type the text! Esc to cancel",
        CurrentScreen::TypingResults => "Press Enter to continue",
        CurrentScreen::LearningSelect => "Enter path to file, Esc to back",
        CurrentScreen::LearningMode => "Type answer + Enter, Esc to back",
        CurrentScreen::LearningResults => "Press Enter to continue",
        CurrentScreen::Statistics => "Press Esc to back",
        CurrentScreen::Settings => "l: Lang, d: Diff, s: Save, Esc: Back",
        _ => "",
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray));

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_content(app: &mut App, frame: &mut Frame, area: Rect) {
    match app.current_screen {
        CurrentScreen::Menu => render_menu(app, frame, area),
        CurrentScreen::TypingTest => render_typing_test(app, frame, area),
        CurrentScreen::TypingResults => render_typing_results(app, frame, area),
        CurrentScreen::Statistics => render_statistics(app, frame, area),
        CurrentScreen::Settings => render_settings(app, frame, area),
        CurrentScreen::LearningSelect => render_learning_select(app, frame, area),
        CurrentScreen::LearningMode => render_learning_mode(app, frame, area),
        _ => render_placeholder(app, frame, area),
    }
}

fn render_learning_select(app: &mut App, frame: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = app
        .file_explorer_state
        .files
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
            let style = if i == app.file_explorer_state.selected_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![Span::styled(file_name, style)]))
        })
        .collect();

    let title = format!(" Select Learning Set (Current: {}) ", app.file_explorer_state.current_dir.display());
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    frame.render_widget(list, area);
}

fn render_menu(app: &App, frame: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = app
        .menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.menu_cursor {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![Span::styled(*item, style)]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Select Mode "))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    // Center the menu
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(10),
            Constraint::Percentage(20),
        ])
        .split(area);
    
    let h_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(layout[1]);

    frame.render_widget(list, h_layout[1]);
}

fn render_typing_test(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Target text
            Constraint::Percentage(50), // Typed text
        ])
        .split(area);

    // Target Text
    let target_block = Block::default()
        .borders(Borders::ALL)
        .title(" Target Text ");
    
    let target_text = Paragraph::new(app.typing_state.target_text.as_str())
        .block(target_block)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::Gray));
    
    frame.render_widget(target_text, chunks[0]);

    // Typed Text
    let typed_block = Block::default()
        .borders(Borders::ALL)
        .title(" Your Input ");
    
    // Colorize typed text (green for correct, red for wrong)
    // This is a simplified view; for a real typing test we'd want character-by-character coloring
    // relative to the target.
    let typed_text = Paragraph::new(app.typing_state.typed_text.as_str())
        .block(typed_block)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White));

    frame.render_widget(typed_text, chunks[1]);
}

fn render_typing_results(app: &App, frame: &mut Frame, area: Rect) {
    if let Some(result) = &app.typing_state.result {
        let text = vec![
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(
                format!("WPM: {:.1}", result.wpm),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(
                format!("Accuracy: {:.1}%", result.accuracy),
                Style::default().fg(Color::Blue),
            )]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw(format!("Time: {:.2}s", result.duration.as_secs_f64()))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(
                result.rating(),
                Style::default().fg(Color::Yellow),
            )]),
        ];

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(" Results "))
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
    }
}

fn render_statistics(app: &App, frame: &mut Frame, area: Rect) {
    use ratatui::widgets::{Table, Row};

    if app.statistics_state.highscores.is_empty() {
        let paragraph = Paragraph::new("No highscores found yet.")
            .block(Block::default().borders(Borders::ALL).title(" Statistics "))
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
        return;
    }

    let rows: Vec<Row> = app.statistics_state.highscores.iter().map(|score| {
        Row::new(vec![
            score.name.clone(),
            format!("{:.1}", score.wpm),
            format!("{:.1}%", score.accuracy),
            score.difficulty.clone(),
            score.language.clone(),
            score.timestamp.clone(),
        ])
    }).collect();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Summary
            Constraint::Min(1),    // Table
        ])
        .split(area);

    // Calculate summary
    let total_tests = app.statistics_state.highscores.len();
    let avg_wpm = app.statistics_state.highscores.iter().map(|s| s.wpm).sum::<f64>() / total_tests as f64;
    let avg_acc = app.statistics_state.highscores.iter().map(|s| s.accuracy).sum::<f64>() / total_tests as f64;
    let best_wpm = app.statistics_state.highscores.iter().map(|s| s.wpm).fold(0.0f64, f64::max);

    let summary_text = format!(
        "Total Tests: {} | Avg WPM: {:.1} | Avg Accuracy: {:.1}% | Best WPM: {:.1}",
        total_tests, avg_wpm, avg_acc, best_wpm
    );

    let summary = Paragraph::new(summary_text)
        .block(Block::default().borders(Borders::ALL).title(" Summary "))
        .alignment(Alignment::Center);
    
    frame.render_widget(summary, chunks[0]);

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(15),
            Constraint::Percentage(10),
            Constraint::Percentage(35),
        ]
    )
    .header(Row::new(vec!["Name", "WPM", "Acc", "Diff", "Lang", "Date"])
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
    .block(Block::default().borders(Borders::ALL).title(" Highscores "));

    frame.render_widget(table, chunks[1]);
}

fn render_settings(app: &App, frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::raw("Language: "),
            Span::styled(
                app.config.defaults.language.clone(),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" (Press 'l' to toggle)"),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::raw("Difficulty: "),
            Span::styled(
                app.config.defaults.difficulty.clone(),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" (Press 'd' to toggle)"),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::raw("Press 's' to save configuration")]),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Settings "))
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_learning_mode(app: &App, frame: &mut Frame, area: Rect) {
    if let Some(set) = &app.learning_state.set {
        if app.learning_state.current_card_index >= set.cards.len() {
             let paragraph = Paragraph::new("Learning Session Complete!")
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
            return;
        }

        let card = &set.cards[app.learning_state.current_card_index];
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40), // Question
                Constraint::Percentage(20), // Input
                Constraint::Percentage(40), // Answer/Feedback
            ])
            .split(area);

        // Question
        let question_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Card {}/{} ", app.learning_state.current_card_index + 1, set.cards.len()));
        
        let question_text = Paragraph::new(card.front.as_str())
            .block(question_block)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        
        frame.render_widget(question_text, chunks[0]);

        // Input
        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(" Your Answer ");
        
        let input_text = Paragraph::new(app.learning_state.user_input.as_str())
            .block(input_block)
            .wrap(Wrap { trim: true });
        
        frame.render_widget(input_text, chunks[1]);

        // Feedback/Answer
        if app.learning_state.show_back {
            let feedback_block = Block::default()
                .borders(Borders::ALL)
                .title(" Result ");
            
            let feedback_content = if let Some(match_result) = &app.learning_state.match_result {
                let color = match match_result {
                    crate::modules::learning::MatchResult::AutoCorrect { .. } => Color::Green,
                    crate::modules::learning::MatchResult::AutoIncorrect { .. } => Color::Red,
                    crate::modules::learning::MatchResult::NeedsUserDecision { .. } => Color::Yellow,
                };
                
                let result_text = crate::modules::learning::fuzzy::format_match_result(match_result);
                let full_text = format!("{}\n\nCorrect Answer: {}", result_text, card.back);
                
                Paragraph::new(full_text)
                    .block(feedback_block)
                    .wrap(Wrap { trim: true })
                    .style(Style::default().fg(color))
            } else {
                 Paragraph::new(card.back.as_str())
                    .block(feedback_block)
                    .wrap(Wrap { trim: true })
            };
            
            frame.render_widget(feedback_content, chunks[2]);
        }
    } else {
        let paragraph = Paragraph::new("No learning set loaded")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
    }
}

fn render_placeholder(_app: &App, frame: &mut Frame, area: Rect) {
    let paragraph = Paragraph::new("Not implemented")
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}
