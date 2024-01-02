use std::{
    sync::{Arc, RwLock},
    thread,
};

use super::{runner, State};
use crate::state::{AppState, FocusState};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use tui_textarea::TextArea;

pub fn handler(event: Event, state: &Arc<RwLock<State>>) -> usize {
    let mut want_active = 0;
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
                    KeyCode::Char('q') => {
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
                    KeyCode::Char('r') => {
                        w_state.focus = FocusState::Focus("response_field".into());
                        w_state.state = AppState::Typing;
                    }
                    KeyCode::Char('1') => {
                        want_active = 1;
                    }
                    KeyCode::Char('2') => {
                        want_active = 2;
                    }
                    KeyCode::Enter => {
                        w_state.response = TextArea::from(["sending response".to_string()]);
                        let method = w_state.method_input.lines()[0].clone();
                        let url = w_state.url_input.lines()[0].clone();
                        let body = w_state.body_input.lines().join("\n");
                        let header = w_state.header_input.lines().to_vec();
                        let t_state = state.clone();
                        thread::spawn(move || {
                            runner::run(
                                method.as_str(),
                                url.as_str(),
                                header,
                                body.as_str(),
                                t_state,
                            );
                        });
                    }
                    _ => {}
                }
            }
        }
    }
    want_active
}

fn input_handler(key: KeyEvent, state: &mut State) {
    if let FocusState::Focus(focus_on) = &state.focus {
        match focus_on.as_str() {
            "url_input" => match key.code {
                KeyCode::Enter => {
                    state.state = AppState::Idle;
                    state.focus = FocusState::None;
                }
                _ => {
                    state.url_input.input(key);
                }
            },
            "body_input" => {
                state.body_input.input(key);
            }
            "header_input" => {
                state.header_input.input(key);
            }
            "method_input" => match key.code {
                KeyCode::Enter => {
                    state.state = AppState::Idle;
                    state.focus = FocusState::None;
                }
                _ => {
                    state.method_input.input(key);
                }
            },
            "response_field" => match key.code {
                KeyCode::Char(_) => {}
                _ => {
                    state.response.input(key);
                }
            },
            _ => {}
        }
    }
}
