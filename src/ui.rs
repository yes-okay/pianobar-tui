use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::state::AppState;
use crate::theme::Theme;

pub fn render(frame: &mut Frame, state: &AppState, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new("pianobar-tui")
        .block(Block::default().borders(Borders::ALL).title("Header"))
        .style(
            Style::default()
                .fg(theme.foreground)
                .bg(theme.background),
        );

    frame.render_widget(header, chunks[0]);

    let log_text = state.logs.join("\n");

    let logs = Paragraph::new(log_text)
        .block(Block::default().borders(Borders::ALL).title("Logs"))
        .style(
            Style::default()
                .fg(theme.foreground)
                .bg(theme.background),
        );

    frame.render_widget(logs, chunks[1]);

    let footer =
        Paragraph::new("q quit • n next • p pause • + love • - ban")
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .style(
            Style::default()
                .fg(theme.accent)
                .bg(theme.background),
        );

    frame.render_widget(footer, chunks[2]);
}