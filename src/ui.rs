use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{app::App, graph::GraphGrid};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(20),
            Constraint::Min(0),
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
    //Row::new(Cell::)
    frame.render_widget(title, chunks[0]);
    frame.render_widget(GraphGrid, chunks[1]);
    // render_canvas(frame, chunks[1]);
}
