use chrono::{Duration, NaiveDate};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    app::{App, CurrentScreen},
    graph::GraphGrid,
};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "GitHub Commit History",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);
    frame.render_widget(title, chunks[0]);
    frame.render_widget(GraphGrid::new(app), chunks[1]);
    frame.render_widget(footer_hint(app), chunks[2]);
    frame.render_widget(footer_info(app), chunks[3]);
}
fn footer_hint(app: &App) -> Paragraph<'_> {
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => {
                let content_hint  = format!("(wasd  ↑←↓→)  to move /(k,l) increase,decrease commits/(esc/q) to quit,(e)toggle no-commit mode");
                Span::styled(content_hint, Style::default().fg(Color::Red))
            }
            CurrentScreen::Exiting => Span::styled(
                "(y) to create commits / (e) to go back",
                Style::default().fg(Color::Red),
            ),
        }
    };

    Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL))
}

fn format_date(day: Option<NaiveDate>, days_to_add: i64) -> String {
    day.map_or_else(
        || "No date".to_string(),
        |date| {
            let new_date = date + Duration::days(days_to_add);
            new_date.format("%d-%m-%Y").to_string()
        },
    )
}

fn footer_info(app: &App) -> Paragraph<'_> {
    let current_keys_info = {
        match app.current_screen {
            CurrentScreen::Main => {
                let content_hint = format!(
                    "commits-{} date  {}",
                    app.commits[app.pos],
                    format_date(app.start_date, app.pos as i64)
                );
                Span::styled(content_hint, Style::default().fg(Color::Red))
            }
            CurrentScreen::Exiting => Span::styled(
                "this will create commits and exit",
                Style::default().fg(Color::Red),
            ),
        }
    };
    Paragraph::new(Line::from(current_keys_info)).block(Block::default().borders(Borders::ALL))
}
