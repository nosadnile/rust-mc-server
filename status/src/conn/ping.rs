use std::time::Duration;

use tokio::net::TcpStream;

use crate::{
    io::traits::{AsyncReadRawPacket, AsyncWriteRawPacket},
    packet::{ping::PingPacket, pong::PongPacket},
};

use super::{error::ServerError, response::StatusResponse};

/// Wraps a built connection
///
/// Constructed by calling `status()` on
/// a `StatusConnection` struct.
#[derive(Debug)]
pub struct PingConnection {
    pub stream: TcpStream,
    pub protocol_version: usize,
    pub address: String,
    pub port: u16,
    pub timeout: Duration,
    pub status: StatusResponse,
}

impl PingConnection {
    /// Sends a ping to the Minecraft server with the
    /// provided payload and asserts that the returned
    /// payload is the same.
    ///
    /// Server closes the connection after a ping call,
    /// so this method consumes the connection.
    pub async fn ping(mut self, payload: u64) -> Result<(), ServerError> {
        let ping = PingPacket::new(payload);

        self.stream
            .write_packet_with_timeout(ping, self.timeout)
            .await?;

        let pong: PongPacket = self.stream.read_packet_with_timeout(self.timeout).await?;

        if pong.payload != payload {
            return Err(ServerError::MismatchedPayload {
                expected: payload,
                actual: pong.payload,
            });
        }

        Ok(())
    }
}
