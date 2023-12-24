use std::{
    sync::{Arc, RwLock},
    thread,
};

use super::{runner, State};
use crate::state::{AppState, FocusState};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

pub fn handler(event: Event, state: &Arc<RwLock<State>>) {
    if let Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            let mut w_state = state.write().unwrap();
            if w_state.state.is_typing() {
                if let KeyCode::Esc = key.code {
                    w_state.state = AppState::Idle;
                    w_state.focus = FocusState::None;
                }
                input_handler(key, &mut w_state);
            } else {
                match key.code {
                    KeyCode::Esc => {
                        w_state.state = AppState::Close;
                    }
                    KeyCode::Char('u') => {
                        w_state.focus = FocusState::Focus("url_input".into());
                        w_state.state = AppState::Typing;
                    }
                    KeyCode::Char('b') => {
                        w_state.focus = FocusState::Focus("body_input".into());
                        w_state.state = AppState::Typing;
                    }
                    KeyCode::Char('h') => {
                        w_state.focus = FocusState::Focus("header_input".into());
                        w_state.state = AppState::Typing;
                    }
                    KeyCode::Char('m') => {
                        w_state.focus = FocusState::Focus("method_input".into());
                        w_state.state = AppState::Typing;
                    }
                    KeyCode::Enter => {
                        w_state.response = "sending response".to_string();
                        let t_state = state.clone();
                        thread::spawn(move || {
                            runner::run(t_state);
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

fn input_handler(key: KeyEvent, state: &mut State) {
    if let FocusState::Focus(focus_on) = &state.focus {
        match focus_on.as_str() {
            "url_input" => {
                state.url_input.input(key);
            }
            "body_input" => {
                state.body_input.input(key);
            }
            "header_input" => {
                state.header_input.input(key);
            }
            "method_input" => {
                state.method_input.input(key);
            }
            _ => {}
        }
    }
}
