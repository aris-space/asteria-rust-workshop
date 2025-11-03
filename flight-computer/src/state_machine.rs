//! This crate implements the state of the rocket and its transition logic.

use crate::input::Inputs;

pub enum State {
    Idle,
    Thrusting,
    Coasting,
    Descend,
    Shutdown,
}

impl State {
    #[allow(clippy::match_same_arms)]
    pub fn tick(&mut self, _inputs: &Inputs) {
        // TODO: actually do something
        match self {
            State::Idle => {}
            State::Thrusting => {}
            State::Coasting => {}
            State::Descend => {}
            State::Shutdown => {}
        }
    }
}
