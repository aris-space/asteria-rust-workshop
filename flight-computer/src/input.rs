//! This module contains the input data for the flight computer

use std::io::Error as IoError;
use std::time::Duration;

use protocols::api::Velocity;
use protocols::api::{Location, SensorMessage};
use protocols::server::MessageReceiver;
use tokio::time::timeout;

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
    pub async fn update(
        &mut self,
        receiver: &mut MessageReceiver<SensorMessage>,
    ) -> Result<(), IoError> {
        // Try to receive new messages
        // Because async reads block until data is available we use a timeout to avoid blocking forever.
        // However, this is not ideal as it requires cancellation safety, otherwise data might be lost.
        let r = timeout(Duration::from_millis(10), async {
            loop {
                let message = receiver.recv().await?;
                self.update_from_message(message);
            }
            #[allow(unreachable_code)] // just used for type inference
            Result::<(), IoError>::Ok(())
        })
        .await;
        match r {
            Ok(Err(e)) => Err(e), // IO error
            _ => Ok(()),          // timeout reached, no more messages
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
