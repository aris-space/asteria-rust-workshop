use protocols::{api::TelemetryCommandMessage, client::MessageSender};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sender = MessageSender::<TelemetryCommandMessage>::connect().await?;

    sender.send(&TelemetryCommandMessage::StartIgntion).await?;

    Ok(())
}
