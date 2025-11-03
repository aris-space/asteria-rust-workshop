//! This module contains the server to listen for messages

use std::io::Error as IoError;
use std::marker::PhantomData;
use std::net::ToSocketAddrs as _;

use tokio::io::{AsyncBufReadExt as _, BufReader};
use tokio::net::{TcpSocket, TcpStream};

/// This struct can receive messages sent from other componets
///
/// Create it with [`Self::new()`] and receive messages with [`Self::recv()`].
pub struct MessageReceiver<T> {
    stream: BufReader<TcpStream>,
    _type: std::marker::PhantomData<T>,
}

impl<T> MessageReceiver<T> {
    /// Creates a new message receiver by waiting for a connection to establish on `port`.
    pub async fn new(port: u16) -> Result<Self, IoError> {
        // Create a network socket
        let addr = ("127.0.0.1", port)
            .to_socket_addrs()?
            .next()
            .ok_or(IoError::new(std::io::ErrorKind::InvalidInput, "bad port"))?;
        let socket = TcpSocket::new_v4()?;

        // Listen for connections on it
        socket.bind(addr)?;
        let listener = socket.listen(1)?;

        // Accept the first connection and store it
        let (tcp, _) = listener.accept().await?;
        let reader = BufReader::new(tcp);

        Ok(Self {
            stream: reader,
            _type: PhantomData,
        })
    }
}

impl<T: serde::de::DeserializeOwned> MessageReceiver<T> {
    /// Read a single message from the connection.
    pub async fn recv(&mut self) -> Result<T, IoError> {
        // Read a line from the stream
        let mut line = String::new();
        self.stream.read_line(&mut line).await?;
        // Parse this line as a json `T`.
        serde_json::from_str::<T>(&line)
            .map_err(|e| IoError::new(std::io::ErrorKind::InvalidData, e))
    }
}
