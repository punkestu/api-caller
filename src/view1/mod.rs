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
    let state: Vec<Arc<RwLock<State>>> = vec![
        Arc::from(RwLock::from(State::new(1))),
        Arc::from(RwLock::from(State::new(2))),
    ];
    let mut active_tab = 0;
    loop {
        ui::render(terminal, &state[active_tab])?;
        if poll(config::TICK_TIMEOUT)? {
            let get_tab = control::handler(read()?, &state[active_tab]);
            if get_tab > 0 {
                active_tab = get_tab - 1;
            }
        }
        if let AppState::Close = state[active_tab].read().unwrap().state {
            break;
        }
    }
    Ok(())
}
