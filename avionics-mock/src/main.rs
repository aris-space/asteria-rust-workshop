use std::time::Duration;

use protocols::api::{Location, SensorMessage};
use protocols::client::MessageSender;

#[allow(clippy::unreadable_literal)]
const WICHLEN: Location = Location {
    latitude: 46.888542,
    longitude: 9.114844,
    altitude: 1387.2,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the flight computer
    let mut message_sender =
        MessageSender::<SensorMessage>::connect(SensorMessage::COMMUNICATIONS_PORT).await?;

    // With 5Hz send the current position
    let mut interval = tokio::time::interval(Duration::from_secs_f32(1. / 5.));
    loop {
        interval.tick().await;

        message_sender
            .send(&SensorMessage::LocationData(WICHLEN))
            .await?;
    }
}
