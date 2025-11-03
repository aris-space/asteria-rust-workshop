//! Contains API definitions and helpers to communicate via JSON over TCP between the other components.

pub mod api;
pub mod client;
pub mod server;

#[cfg(test)]
mod tests {
    use crate::api::Location;
    use crate::client::MessageSender;
    use crate::server::MessageReceiver;

    /// Sends and receives some data to ensure the server and client work as expected.
    /// Can also be used as an example.
    #[tokio::test]
    async fn send_and_receive() -> Result<(), Box<dyn std::error::Error>> {
        // Create the connection. Uses `join!`, to create them at the same time. Otherwise we deadlock!
        const PORT: u16 = 4242;
        let (receiver, sender) = tokio::join!(
            MessageReceiver::<Location>::establish(PORT),
            MessageSender::<Location>::connect(PORT),
        );
        let (mut receiver, mut sender) = (receiver?, sender?);

        // Create some data to be sent
        #[allow(clippy::unreadable_literal)]
        let dübi = Location {
            latitude: 47.405582,
            longitude: 8.632077,
            altitude: 434.,
        };

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
}
