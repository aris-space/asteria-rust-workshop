//! The module contains the client to send messages.

use std::io::Error as IoError;
use std::marker::PhantomData;

use tokio::net::UdpSocket;

use crate::api::MessageChannel;

/// This struct can send messages of type `T` to other components
///
/// Create it with [`Self::connect()`] and send messages with [`Self::send()`].
pub struct MessageSender<T> {
    socket: UdpSocket,
    _type: PhantomData<T>,
}

impl<T> MessageSender<T> {
    /// Creates a new message sender for a specific channel.
    pub async fn connect() -> Result<Self, IoError>
    where
        T: MessageChannel,
    {
        // Bind to a random local port
        let socket = UdpSocket::bind("[::1]:0").await?;

        // Set the target address
        let addr = format!("[::1]:{port}", port = T::COMMUNICATIONS_PORT);
        socket.connect(addr).await?;

        Ok(Self {
            socket,
            _type: PhantomData,
        })
    }
}

impl<T: serde::Serialize> MessageSender<T> {
    /// Send a message to the server.
    pub async fn send(&mut self, message: &T) -> Result<(), IoError> {
        // Serialize the message to JSON
        let json = serde_json::to_string(message)?;
        let bytes = json.as_bytes();
        if bytes.len() > 1024 {
            return Err(IoError::new(
                std::io::ErrorKind::InvalidInput,
                "Message too large to send",
            ));
        }

        // Send the JSON data over UDP
        self.socket.send(bytes).await?;
        Ok(())
    }
}
