use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub fn border(title: String, is_active: bool) -> Block<'static> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if is_active {
            Color::Gray
        } else {
            Color::DarkGray
        }))
}
