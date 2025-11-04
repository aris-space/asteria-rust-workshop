//! This module contains the input data for the flight computer

use std::io::Write;
use std::time::Duration;

use protocols::api::{Location, SensorMessage};
use protocols::server::MessageReceiver;
use tokio::time::timeout;

/// Stores the current inputs of the system, given by the avionics
pub struct Inputs {
    pub location: Location,
}

impl Default for Inputs {
    fn default() -> Self {
        Inputs {
            location: Location::INVALID,
        }
    }
}

impl Inputs {
    /// Update the inputs with new data
    pub async fn update(&mut self, receiver: &mut MessageReceiver<SensorMessage>) {
        // Try to receive new messages
        // Because async reads block until data is available and I can't find a non-blocking read,
        // we use a timeout to avoid blocking forever.
        // However, this is not ideal as it requires cancellation safety, otherwise data might be lost.
        let _ = timeout(Duration::from_millis(10), async {
            loop {
                let Ok(message) = receiver.recv().await else {
                    eprintln!("malformed message");
                    continue;
                };
                self.update_from_message(message);
            }
        })
        .await;
    }

    /// Update the inputs from a single message
    fn update_from_message(&mut self, message: SensorMessage) {
        match message {
            SensorMessage::LocationData(loc) => {
                print!(".");
                std::io::stdout().flush().expect("failed to flush stdout");
                self.location = loc;
            }
        }
    }
}
