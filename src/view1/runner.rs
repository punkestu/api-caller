use std::sync::{Arc, RwLock};

use tui_textarea::TextArea;

use super::{service, state::State};

pub fn run(method: &str, url: &str, headers: Vec<String>, body: &str, state: Arc<RwLock<State>>) {
    let response_str = match service::request(method, url, headers, body.to_string()) {
        Ok(response) => response,
        Err(err) => err.to_string(),
    };
    state.write().unwrap().response = TextArea::from(response_str.lines());
}
