use std::time::Duration;

use tokio::net::TcpStream;

use crate::{
    io::traits::{AsyncReadRawPacket, AsyncWriteRawPacket},
    packet::{handshake::HandshakePacket, request::RequestPacket, response::ResponsePacket},
};

use super::{error::ServerError, ping::PingConnection, response::StatusResponse};

/// Wraps a built connection
pub struct StatusConnection {
    pub stream: TcpStream,
    pub protocol_version: usize,
    pub address: String,
    pub port: u16,
    pub timeout: Duration,
}

impl StatusConnection {
    /// Sends and reads the packets for the
    /// ServerListPing status call.
    ///
    /// Consumes the connection and returns a type
    /// that can only issue pings. The resulting
    /// status body is accessible via the `status`
    /// property on `PingConnection`.
    pub async fn status(mut self) -> Result<PingConnection, ServerError> {
        let handshake =
            HandshakePacket::new(self.protocol_version, self.address.to_string(), self.port);

        self.stream
            .write_packet_with_timeout(handshake, self.timeout)
            .await?;

        self.stream
            .write_packet_with_timeout(RequestPacket::new(), self.timeout)
            .await?;

        let response: ResponsePacket = self.stream.read_packet_with_timeout(self.timeout).await?;

        let status: StatusResponse = serde_json::from_str(&response.body)
            .map_err(|_| ServerError::InvalidJson(response.body))?;

        Ok(PingConnection {
            stream: self.stream,
            protocol_version: self.protocol_version,
            address: self.address,
            port: self.port,
            status,
            timeout: self.timeout,
        })
    }
}
