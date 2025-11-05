//! This module contains the server to listen for messages

use std::io::Error as IoError;
use std::marker::PhantomData;

use tokio::net::UdpSocket;

/// This struct can receive messages sent from other componets
///
/// Create it with [`Self::listen()`] and receive messages with [`Self::recv()`] or [`Self::try_recv()`].
pub struct MessageReceiver<T> {
    socket: UdpSocket,
    _type: std::marker::PhantomData<T>,
}

impl<T> MessageReceiver<T> {
    /// Creates a new message receiver by listening on a specific port.
    pub async fn listen(port: u16) -> Result<Self, IoError> {
        let addr = format!("[::1]:{port}");
        let socket = UdpSocket::bind(addr).await?;

        Ok(Self {
            socket,
            _type: PhantomData,
        })
    }
}

impl<T: serde::de::DeserializeOwned> MessageReceiver<T> {
    /// Read a single message from other component.
    /// This function will block until a message is received, but is safe to cancel.
    pub async fn recv(&mut self) -> Result<T, IoError> {
        // Receive data into the buffer
        let mut buffer = vec![0; 1024]; // Maximum packet size we support
        let len = self.socket.recv(&mut buffer).await?;

        // Parse the received data as JSON
        serde_json::from_slice::<T>(&buffer[..len])
            .map_err(|e| IoError::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Read a single message from other component if available.
    /// This function will not block and return `Ok(None)` if no message is available.
    pub fn try_recv(&mut self) -> Result<Option<T>, IoError> {
        // Receive data into the buffer
        let mut buffer = vec![0; 1024]; // Maximum packet size we support
        match self.socket.try_recv(&mut buffer) {
            Ok(len) => {
                // Parse the received data as JSON
                let message = serde_json::from_slice::<T>(&buffer[..len])
                    .map_err(|e| IoError::new(std::io::ErrorKind::InvalidData, e))?;
                Ok(Some(message))
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e),
        }
    }
}
