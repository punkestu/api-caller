use crate::state::{AppState, FocusState};
use tui_textarea::TextArea;

#[derive(Default)]
pub struct State {
    pub state: AppState,
    pub focus: FocusState,
    pub url_input: TextArea<'static>,
    pub method_input: TextArea<'static>,
    pub header_input: TextArea<'static>,
    pub body_input: TextArea<'static>,
    pub response: TextArea<'static>,
    pub tag: usize,
}

impl State {
    pub fn new(tag: usize) -> Self {
        Self {
            tag,
            ..State::default()
        }
    }
}
