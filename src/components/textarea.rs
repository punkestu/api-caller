use ratatui::style::{Color, Style, Stylize};
use tui_textarea::TextArea;

use super::block::border;

pub fn setup(
    focus_on: &String,
    active_on: String,
    title: String,
    textarea: &mut TextArea<'static>,
) {
    textarea.set_cursor_line_style(Style::default().not_underlined());
    if *focus_on == active_on {
        textarea.set_style(Style::default().fg(Color::LightGreen));
        textarea.set_cursor_style(Style::new().bg(Color::LightGreen));
        textarea.set_block(border(title, true));
    } else {
        textarea.set_style(Style::default().fg(Color::Green));
        textarea.set_cursor_style(Style::new().hidden());
        textarea.set_block(border(title, false));
    }
}
