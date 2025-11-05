//! This crate implements the state of the rocket and its transition logic.

use protocols::api::{AvionicsCommandMessage, TelemetryCommandMessage};
use protocols::client::MessageSender;

use crate::input::Inputs;

pub enum State {
    Idle,
    Thrusting,
    Coasting,
    Descend,
    Shutdown,
}

impl State {
    #[allow(clippy::match_same_arms)] // todo
    #[allow(clippy::unused_async)] // will be needed to send to avionics
    pub async fn tick(
        &mut self,
        _inputs: &Inputs,
        _avionics: &mut MessageSender<AvionicsCommandMessage>,
    ) {
        // TODO: actually do something
        match self {
            State::Idle => {}
            State::Thrusting => {}
            State::Coasting => {}
            State::Descend => {}
            State::Shutdown => {}
        }
    }

    /// Handle a telemetry command
    pub async fn handle_command(
        &mut self,
        command: TelemetryCommandMessage,
        avionics: &mut MessageSender<AvionicsCommandMessage>,
    ) {
        match command {
            TelemetryCommandMessage::StartIgntion => {
                match avionics.send(&AvionicsCommandMessage::IgniteEngine).await {
                    Ok(_) => {
                        println!("Starting ignition sequence");
                        *self = State::Thrusting;
                    }
                    Err(e) => eprintln!("Failed to send ignition command to avionics: {e}"),
                }
            }
        }
    }
}
