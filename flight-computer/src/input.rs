//! This module contains the input data for the flight computer

use std::io::Error as IoError;

use protocols::api::Velocity;
use protocols::api::{Location, SensorMessage};
use protocols::server::MessageReceiver;

/// Stores the current inputs of the system, given by the avionics
pub struct Inputs {
    pub location: Location,
    pub velocity: Velocity,
}

impl Default for Inputs {
    fn default() -> Self {
        Inputs {
            location: Location::INVALID,
            velocity: Velocity::INVALID,
        }
    }
}

impl Inputs {
    /// Update the inputs with new data
    pub fn update(&mut self, receiver: &mut MessageReceiver<SensorMessage>) {
        // Try to receive new messages
        while let Ok(Some(message)) = receiver
            .try_recv()
            .inspect_err(|e| eprintln!("problem with receiving message: {e}"))
        {
            self.update_from_message(message);
        }
    }

    /// Update the inputs from a single message
    fn update_from_message(&mut self, message: SensorMessage) {
        match message {
            SensorMessage::LocationData(loc) => {
                self.location = loc;
            }
            SensorMessage::VelocityData(vel) => {
                self.velocity = vel;
            }
        }
    }
}
