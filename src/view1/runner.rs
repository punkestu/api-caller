use std::sync::{Arc, RwLock};

use tui_textarea::TextArea;

use super::{service, state::State};

pub fn run(method: &str, url: &str, headers: Vec<String>, body: &str, state: Arc<RwLock<State>>) {
    let mut url = String::from(url);
    if !url.starts_with("http://") && !url.starts_with("https://") {
        url.insert_str(0, "http://");
    }
    let response_str = match service::request(method, url.as_str(), headers, body.to_string()) {
        Ok(response) => response,
        Err(err) => err.to_string(),
    };
    state.write().unwrap().response = TextArea::from(response_str.lines());
}
