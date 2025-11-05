//! Contains API definitions and helpers to communicate via JSON over TCP between the other components.

pub mod api;
pub mod client;
pub mod server;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::api::{Location, SensorMessage};
    use crate::client::MessageSender;
    use crate::server::MessageReceiver;

    /// Sends and receives some data to ensure the server and client work as expected.
    /// Can also be used as an example.
    #[tokio::test]
    async fn send_and_receive() -> Result<(), Box<dyn std::error::Error>> {
        // Create the connection.
        let mut receiver = MessageReceiver::<SensorMessage>::listen().await?;
        let mut sender = MessageSender::<SensorMessage>::connect().await?;

        // Create some data to be sent
        #[allow(clippy::unreadable_literal)]
        let dübi = SensorMessage::LocationData(Location {
            latitude: 47.405582,
            longitude: 8.632077,
            altitude: 434.,
        });

        // Send the location, receive it on the other end and make sure they are the same.
        sender.send(&dübi).await?;
        let location = receiver.recv().await?;
        assert_eq!(location, dübi);

        // Repeat it to make sure it works repeatedly.
        sender.send(&dübi).await?;
        let location = receiver.recv().await?;
        assert_eq!(location, dübi);

        Ok(())
    }

    #[tokio::test]
    async fn send_and_try_recieve() -> Result<(), Box<dyn std::error::Error>> {
        // Create the connection.
        let mut receiver = MessageReceiver::<SensorMessage>::listen().await?;
        // Try to receive data when none is sent
        let message = receiver.try_recv()?;
        assert!(message.is_none());

        let mut sender = MessageSender::<SensorMessage>::connect().await?;

        // Create some data to be sent
        #[allow(clippy::unreadable_literal)]
        let dübi = SensorMessage::LocationData(Location {
            latitude: 47.405582,
            longitude: 8.632077,
            altitude: 434.,
        });

        for _ in 0..5 {
            // Send the location,
            sender.send(&dübi).await?;
        }

        for i in 0..5 {
            // loop until we receive the data
            let mut watchdog = 0;
            let message = loop {
                if let Some(msg) = receiver.try_recv()? {
                    break msg;
                }
                watchdog += 1;
                tokio::time::sleep(Duration::from_millis(1)).await;
                assert!((watchdog <= 100), "Did not receive message in time {i}");
            };
            assert_eq!(message, dübi);
        }

        Ok(())
    }
}
