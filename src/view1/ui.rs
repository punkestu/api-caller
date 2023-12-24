use super::State;
use crate::components::textarea::setup;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io::Result,
    sync::{Arc, RwLock},
};

pub fn render<B: Backend>(terminal: &mut Terminal<B>, state: &Arc<RwLock<State>>) -> Result<()> {
    terminal.draw(|frame| {
        let area = frame.size();
        let focus_on: String = state.read().unwrap().focus.focus_on();
        setup(
            &focus_on,
            "url_input".into(),
            "URL [u]".into(),
            &mut state.write().unwrap().url_input,
        );
        setup(
            &focus_on,
            "body_input".into(),
            "BODY [b]".into(),
            &mut state.write().unwrap().body_input,
        );
        setup(
            &focus_on,
            "header_input".into(),
            "HEADER [h]".into(),
            &mut state.write().unwrap().header_input,
        );
        setup(
            &focus_on,
            "method_input".into(),
            "METHOD [m]".into(),
            &mut state.write().unwrap().method_input,
        );
        let main = Block::default()
            .borders(Borders::ALL)
            .title("API CALLER [esc]")
            .title_alignment(Alignment::Center);

        let inner = main.inner(area);
        let vertical = Layout::default()
            .constraints([Constraint::Percentage(8), Constraint::Percentage(92)])
            .split(inner);
        let method_url_send = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(vertical[0]);
        let header_req = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
            .split(vertical[1]);
        let req_res = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(header_req[1]);

        frame.render_widget(main, area);
        frame.render_widget(
            state.read().unwrap().method_input.widget(),
            method_url_send[0],
        );
        frame.render_widget(state.read().unwrap().url_input.widget(), method_url_send[1]);
        frame.render_widget(
            Paragraph::new("SEND [enter]")
                .bold()
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center),
            method_url_send[2],
        );
        frame.render_widget(state.read().unwrap().header_input.widget(), header_req[0]);
        frame.render_widget(state.read().unwrap().body_input.widget(), req_res[0]);
        frame.render_widget(
            Paragraph::new(state.read().unwrap().response.as_str())
                .block(Block::default().borders(Borders::ALL).title("RESPONSE")),
            req_res[1],
        )
    })?;
    Ok(())
}
