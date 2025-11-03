//! The module contains the client to send messages.

use std::io::Error as IoError;
use std::marker::PhantomData;

use tokio::io::{AsyncWriteExt as _, BufWriter};
use tokio::net::TcpStream;

/// This struct can send messages of type `T` to other components
///
/// Create it with [`Self::new()`] and send messages with [`Self::send()`].
pub struct MessageSender<T> {
    stream: BufWriter<TcpStream>,
    _type: PhantomData<T>,
}

impl<T> MessageSender<T> {
    /// Creates a new message sender by connecting to the server at `port`.
    pub async fn new(port: u16) -> Result<Self, IoError> {
        // Try to connect and wait until the connection has been established.
        let addr = format!("127.0.0.1:{port}");
        let tcp = TcpStream::connect(&addr).await?;
        let writer = BufWriter::new(tcp);

        Ok(Self {
            stream: writer,
            _type: PhantomData,
        })
    }
}

impl<T: serde::Serialize> MessageSender<T> {
    /// Send a message to the server.
    pub async fn send(&mut self, message: &T) -> Result<(), IoError> {
        // Serialize the message to JSON
        let json = serde_json::to_string(message)?;
        // Write the JSON line to the stream
        self.stream.write_all(json.as_bytes()).await?;
        self.stream.write_all(b"\n").await?;
        self.stream.flush().await?;
        Ok(())
    }
}
