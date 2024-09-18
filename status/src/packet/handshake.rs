use std::io::Cursor;

use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

use crate::{
    impl_packet_id,
    io::{
        error::ProtocolError,
        traits::{AsyncWireWriteExt, AsyncWriteToBuffer},
    },
};

use super::state::State;

/// HandshakePacket is the first of two packets
/// to be sent during a status check for
/// ServerListPing.
#[derive(Debug, Clone)]
pub struct HandshakePacket {
    pub packet_id: usize,
    pub protocol_version: usize,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: State,
}

impl HandshakePacket {
    pub fn new(protocol_version: usize, server_address: String, server_port: u16) -> Self {
        Self {
            packet_id: 0,
            protocol_version,
            server_address,
            server_port,
            next_state: State::Status,
        }
    }
}

#[async_trait]
impl AsyncWriteToBuffer for HandshakePacket {
    async fn write_to_buffer(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut buffer = Cursor::new(Vec::<u8>::new());

        buffer.write_varint(self.protocol_version).await?;
        buffer.write_string(&self.server_address).await?;
        buffer.write_u16(self.server_port).await?;
        buffer.write_varint(self.next_state.into()).await?;

        Ok(buffer.into_inner())
    }
}

impl_packet_id!(HandshakePacket);
