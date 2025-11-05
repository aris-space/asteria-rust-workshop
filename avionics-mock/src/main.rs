use std::time::Duration;

use protocols::api::{AvionicsCommandMessage, Location, SensorMessage};
use protocols::client::MessageSender;
use protocols::server::MessageReceiver;

mod sim;

#[allow(clippy::unreadable_literal)]
const WICHLEN: Location = Location {
    latitude: 46.888542,
    longitude: 9.114844,
    altitude: 1387.2,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the flight computer
    let mut message_sender = MessageSender::<SensorMessage>::connect().await?;
    let mut command_receiver = MessageReceiver::<AvionicsCommandMessage>::listen().await?;

    // Simulation state
    let mut sim_state = sim::SimulationState::new();

    // With 5Hz simulate and send the current position
    let mut last_tick = tokio::time::Instant::now();
    let mut interval = tokio::time::interval(Duration::from_secs_f32(1. / 5.));
    loop {
        interval.tick().await;

        // Check for commands
        while let Some(command) = command_receiver.try_recv()? {
            match command {
                AvionicsCommandMessage::IgniteEngine => {
                    println!("Igniting engine!");
                    sim_state.ignite_engine();
                }
            }
        }

        // Advance simulation
        let now = tokio::time::Instant::now();
        let dt = (now - last_tick).as_secs_f32();
        last_tick = now;
        sim_state.tick(dt);

        // Send data
        message_sender
            .send(&SensorMessage::LocationData(sim_state.location.clone()))
            .await?;
        message_sender
            .send(&SensorMessage::VelocityData(sim_state.velocity.clone()))
            .await?;
    }
}
