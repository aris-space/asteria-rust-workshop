//! This module contains the server to listen for messages

use std::io::Error as IoError;
use std::marker::PhantomData;

use tokio::io::{AsyncBufReadExt as _, BufReader};
use tokio::net::{TcpListener, TcpStream};

/// This struct can receive messages sent from other componets
///
/// Create it with [`Self::establish()`] and receive messages with [`Self::recv()`].
pub struct MessageReceiver<T> {
    stream: BufReader<TcpStream>,
    line_buf: Vec<u8>,
    _type: std::marker::PhantomData<T>,
}

impl<T> MessageReceiver<T> {
    /// Creates a new message receiver by waiting for a connection to establish on `port`.
    pub async fn establish(port: u16) -> Result<Self, IoError> {
        // Create the network listener
        let addr = format!("127.0.0.1:{port}");
        let listener = TcpListener::bind(addr).await?;

        // Accept the first connection and store it
        let (tcp, _) = listener.accept().await?;
        let reader = BufReader::new(tcp);

        Ok(Self {
            stream: reader,
            line_buf: Vec::new(),
            _type: PhantomData,
        })
    }
}

impl<T: serde::de::DeserializeOwned> MessageReceiver<T> {
    /// Read a single message from the connection.
    ///
    /// This is cancelation safe.
    pub async fn recv(&mut self) -> Result<T, IoError> {
        // Continue reading the line from the stream.
        // `read_until` is cancelation safe, if used with the persistent buffer,
        // which the clients of this function needs.
        self.stream
            .read_until('\n'.try_into().expect("fits into byte"), &mut self.line_buf)
            .await?;
        // Parse this line as a json `T`.
        let res = serde_json::from_slice::<T>(&self.line_buf)
            .map_err(|e| IoError::new(std::io::ErrorKind::InvalidData, e));
        self.line_buf.clear();
        res
    }
}
