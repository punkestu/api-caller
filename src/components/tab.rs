use ratatui::{
    prelude::{Alignment, Color, Constraint, Direction, Layout, Rect, Style},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn tabs(frame: &mut Frame<'_>, tab_act: usize, area: Rect) {
    let tabs = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(0),
        ])
        .split(area);
    for i in 1..3 {
        tab(frame, i.to_string().as_str(), tabs[i - 1], i == tab_act);
    }
}

fn tab(frame: &mut Frame<'_>, tag: &str, area: Rect, active: bool) {
    if active {
        frame.render_widget(
            Paragraph::new("active")
                .bold()
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center),
            area,
        );
    } else {
        frame.render_widget(
            Paragraph::new(tag)
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center),
            area,
        );
    }
}
