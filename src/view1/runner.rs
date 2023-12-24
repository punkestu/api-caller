use std::sync::{Arc, RwLock};

use super::{service, state::State};

pub fn run(state: Arc<RwLock<State>>) {
    let method = state.read().unwrap().method_input.lines()[0].clone();
    let url = state.read().unwrap().url_input.lines()[0].clone();
    match service::request(method.as_str(), url.as_str()) {
        Ok(response) => state.write().unwrap().response = response,
        Err(err) => state.write().unwrap().response = err.to_string(),
    }
}
