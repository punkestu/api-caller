use super::State;
use crate::components::{tab::tabs, textarea::setup};
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
        let mut w_state = state.write().unwrap();
        setup(
            &w_state.focus.focus_on(),
            "url_input".into(),
            "URL [u]".into(),
            &mut w_state.url_input,
        );
        setup(
            &w_state.focus.focus_on(),
            "body_input".into(),
            "BODY [b]".into(),
            &mut w_state.body_input,
        );
        setup(
            &w_state.focus.focus_on(),
            "header_input".into(),
            "HEADER [h]".into(),
            &mut w_state.header_input,
        );
        setup(
            &w_state.focus.focus_on(),
            "method_input".into(),
            "METHOD [m]".into(),
            &mut w_state.method_input,
        );
        setup(
            &w_state.focus.focus_on(),
            "response_field".into(),
            "RESPONSE [r]".into(),
            &mut w_state.response,
        );
        let main = Block::default()
            .borders(Borders::ALL)
            .title("API CALLER [q]")
            .title_alignment(Alignment::Center);

        let inner = main.inner(area);
        let vertical = Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Percentage(80),
            ])
            .split(inner);
        let method_url_send = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(12),
                Constraint::Percentage(80),
                Constraint::Min(14),
            ])
            .split(vertical[1]);
        let header_req = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
            .split(vertical[2]);
        let req_res = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(header_req[1]);

        tabs(frame, w_state.tag, vertical[0]);
        frame.render_widget(main, area);
        frame.render_widget(w_state.method_input.widget(), method_url_send[0]);
        frame.render_widget(w_state.url_input.widget(), method_url_send[1]);
        frame.render_widget(
            Paragraph::new("SEND [enter]")
                .bold()
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center),
            method_url_send[2],
        );
        frame.render_widget(w_state.header_input.widget(), header_req[0]);
        frame.render_widget(w_state.body_input.widget(), req_res[0]);
        frame.render_widget(w_state.response.widget(), req_res[1]);
    })?;
    Ok(())
}
