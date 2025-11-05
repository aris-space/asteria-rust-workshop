use std::time::Duration;

use protocols::api::SensorMessage;
use protocols::server::MessageReceiver;

pub mod input;
pub mod state_machine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize inputs
    let mut inputs = input::Inputs::default();

    // Initialize state machine
    let mut state = state_machine::State::Idle;

    // Connect to other components
    let mut sensor_receiver =
        MessageReceiver::<SensorMessage>::listen(SensorMessage::COMMUNICATIONS_PORT).await?;

    // Main loop at 20Hz
    let mut interval = tokio::time::interval(Duration::from_secs_f32(1. / 20.));
    loop {
        // Wait for the next tick
        interval.tick().await;

        // Update inputs from sensors
        inputs.update(&mut sensor_receiver).await?;

        // Update state machine with current inputs
        state.tick(&inputs);
    }
}
