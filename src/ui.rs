use chrono::{Duration, NaiveDate};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
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
        "Commit Graph",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);
    frame.render_widget(title, chunks[0]);
    frame.render_widget(GraphGrid::new(app), chunks[1]);
    frame.render_widget(footer_hint(app), chunks[2]);
    frame.render_widget(footer_info(app), chunks[3]);

    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default().title("Y/N").borders(Borders::NONE);

        let exit_text = Text::styled(
            "Would you like to add-commits (y) exit (n) go-back (backspace)",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}
fn footer_hint(app: &App) -> Paragraph<'_> {
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => {
                let content_hint  = format!("(wasd  ↑←↓→) move (k,l) increase,decrease commits (esc/q) quit (e) paint/no-paint (enter) save");
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

fn format_date(day: NaiveDate, days_to_add: i64) -> String {
    let new_date = day + Duration::days(days_to_add);
    new_date.format("%d-%m-%Y").to_string()
}

fn footer_info(app: &App) -> Paragraph<'_> {
    let current_keys_info = {
        match app.current_screen {
            CurrentScreen::Main => {
                let edit_st = if app.is_editing {
                    ""
                } else {
                    "you are not adding commits press e to start"
                };
                let content_hint = format!(
                    "commits {} on date {} {}",
                    app.commits[app.pos],
                    format_date(app.start_date, app.pos as i64),
                    edit_st
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

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
