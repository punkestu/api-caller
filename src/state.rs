#[derive(Default)]
pub enum AppState {
    #[default]
    Idle,
    Typing,
    Close,
}

impl AppState {
    pub fn is_typing(&self) -> bool {
        matches!(self, AppState::Typing)
    }
}

#[derive(Default)]
pub enum FocusState {
    #[default]
    None,
    Focus(String),
}

impl FocusState {
    pub fn focus_on(&self) -> String {
        match self {
            FocusState::None => "".into(),
            FocusState::Focus(focus_on) => focus_on.clone(),
        }
    }
}
