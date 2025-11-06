//! This crate implements the state of the rocket and its transition logic.

use protocols::api::{AvionicsCommandMessage, TelemetryCommandMessage};
use protocols::client::MessageSender;

use crate::input::Inputs;

pub enum State {
    Idle,
    Thrusting { last_velocity: f32 },
    Coasting,
    Descend { max_velocity: f32 },
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
            State::Idle | State::Shutdown => {}
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
                    println!(
                        "Apogee at {:.1}m, starting descent",
                        inputs.location.altitude
                    );
                    // deploy drogue, ignore send errors because the transimission is unreliable anyway
                    let _ = avionics.send(&AvionicsCommandMessage::DeployDrogue).await;
                    *self = State::Descend { max_velocity: 0.0 };
                }
            }
            State::Descend { max_velocity } => {
                if inputs.velocity.down.abs() < 1.0 && inputs.location.altitude <= 1500. {
                    println!("Touchdown, max descent velocity was {max_velocity:.1} m/s");
                    *self = State::Shutdown;
                } else {
                    *max_velocity = max_velocity.max(inputs.velocity.down.abs());
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
