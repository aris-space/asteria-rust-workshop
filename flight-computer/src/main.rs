use std::time::Duration;

use csv_async::{AsyncSerializer, AsyncWriterBuilder};
use protocols::api::{
    AvionicsCommandMessage, SensorMessage, TelemetryCommandMessage, TelemetryDataMessage,
};
use protocols::client::MessageSender;
use protocols::server::MessageReceiver;
use std::error::Error;
use tokio::fs::File;

pub mod input;
pub mod state_machine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create csv file
    let mut file_location = File::create("loc.csv").await?;

    let mut file_velocity = File::create("velocity.csv").await?;
    
    // initialize writer
    let mut wrt = AsyncWriterBuilder::new().has_headers(true).create_writer(vec![]);
    
    // initialize serializer
    let mut ser = AsyncSerializer::from_writer(vec![]);
    
    let mut wrt = AsyncWriterBuilder::new().has_headers(true).create_writer(vec![]);
    let mut ser_2 = AsyncSerializer::from_writer(vec![]);


    // Initialize inputs
    let mut inputs = input::Inputs::default();

    // Initialize state machine
    let mut state = state_machine::State::Idle;

    // Connect to other components
    let mut sensor_receiver = MessageReceiver::<SensorMessage>::listen().await?;
    let mut telemetry_sender = MessageSender::<TelemetryDataMessage>::connect().await?;
    let mut avionics_command_sender = MessageSender::<AvionicsCommandMessage>::connect().await?;
    let mut telemetry_command_receiver =
        MessageReceiver::<TelemetryCommandMessage>::listen().await?;

    // Main loop at 20Hz
    let mut interval = tokio::time::interval(Duration::from_secs_f32(1. / 20.));
    let mut phase_counter = 0; // where in the second we are
    loop {
        // Wait for the next tick
        interval.tick().await;

        // Update inputs from sensors, print every 20 ticks
        inputs.update(&mut sensor_receiver);
        if phase_counter == 0 {
            println!(
                "> {:.6}°N {:.6}°E {:.1}m ASL, vel_down={:.1} m/s",
                inputs.location.latitude,
                inputs.location.longitude,
                inputs.location.altitude,
                inputs.velocity.down
            );
            // insert data
            ser.serialize(&
                inputs.location
            ).await?;

            ser_2.serialize(&inputs.velocity).await?;



        }

        // Handle incoming telemetry commands
        while let Ok(Some(command)) = telemetry_command_receiver
            .try_recv()
            .inspect_err(|e| eprintln!("problem with receiving telemetry command: {e}"))
        {
            state
                .handle_command(command, &mut avionics_command_sender)
                .await;
        }

        // Update state machine with current inputs
        state.tick(&inputs, &mut avionics_command_sender).await;

        // Send telemetry data every 3 ticks
        if phase_counter % 3 == 0 {
            let _ = inputs.send_telemetry(&mut telemetry_sender).await;
        }

        phase_counter = (phase_counter + 1) % 20;
    }
}
