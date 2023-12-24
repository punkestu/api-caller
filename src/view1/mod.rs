use crate::{config, state::AppState};
use crossterm::event::{poll, read};
use ratatui::{backend::Backend, Terminal};
use std::{
    io::Result,
    sync::{Arc, RwLock},
};

use self::state::State;

mod control;
mod runner;
mod service;
mod state;
mod ui;

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let state = Arc::from(RwLock::from(State::default()));
    loop {
        ui::render(terminal, &state)?;
        if poll(config::TICK_TIMEOUT)? {
            control::handler(read()?, &state);
        }
        if let AppState::Close = state.read().unwrap().state {
            break;
        }
    }
    Ok(())
}
