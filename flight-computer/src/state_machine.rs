//! This crate implements the state of the rocket and its transition logic.

use protocols::api::{AvionicsCommandMessage, TelemetryCommandMessage};
use protocols::client::MessageSender;

use crate::input::Inputs;

pub enum State {
    Idle,
    Thrusting { last_velocity: f32 },
    Coasting,
    Descend,
    MainDescend,
    Touchdown,
    Shutdown,
}

impl State {
    #[allow(clippy::unused_async)] // will be needed to send to avionics
    pub async fn tick(
        &mut self,
        inputs: &Inputs,
        avionics: &mut MessageSender<AvionicsCommandMessage>,
    ) {
        match self {
            State::Idle | State::Shutdown | State::Touchdown => {}
            State::Thrusting { last_velocity } => {
                if inputs.velocity.down.abs() < *last_velocity {
                    println!("Starting to decelerate");
                    *self = State::Coasting;
                } else {
                    *last_velocity = inputs.velocity.down.abs();
                }
            }
            State::Coasting => {
                if inputs.velocity.down > 0.0 {
                    let _ = avionics.send(&AvionicsCommandMessage::DeployDrogue).await;
                    *self = State::Descend;
                }
            }
            State::Descend => {
                if inputs.location.altitude < 3000.0 {
                    println!("We are so great");
                    let _ = avionics.send(&AvionicsCommandMessage::DeployMain).await;
                    *self = State::MainDescend;
                }
            }
            State::MainDescend => {
                if inputs.location.altitude < 0.1 && inputs.velocity.down.abs() < 0.1 {
                    *self = State::Touchdown;
                }
            }
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
                        *self = State::Thrusting { last_velocity: 0.0 };
                    }
                    Err(e) => eprintln!("Failed to send ignition command to avionics: {e}"),
                }
            }
        }
    }
}
