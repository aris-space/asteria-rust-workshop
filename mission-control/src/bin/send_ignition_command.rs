use protocols::{api::TelemetryCommandMessage, client::MessageSender};

/// This binary sends an ignition command from the mission control system.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a message sender for the Telemetry Uplink channel
    let mut sender = MessageSender::<TelemetryCommandMessage>::connect().await?;
    // Send the ignition command message
    sender.send(&TelemetryCommandMessage::StartIgntion).await?;
    println!("Ignition command sent.");
    Ok(())
}
