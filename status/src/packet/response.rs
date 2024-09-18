use std::io::Cursor;

use async_trait::async_trait;

use crate::io::{
    error::ProtocolError,
    traits::{AsyncReadFromBuffer, AsyncWireReadExt},
};

use super::traits::ExpectedPacketId;

/// ResponsePacket is the response from the
/// server to a status check for
/// ServerListPing.
#[derive(Debug, Clone)]
pub struct ResponsePacket {
    pub packet_id: usize,
    pub body: String,
}

impl ExpectedPacketId for ResponsePacket {
    fn get_expected_packet_id() -> usize {
        0
    }
}

#[async_trait]
impl AsyncReadFromBuffer for ResponsePacket {
    async fn read_from_buffer(buffer: Vec<u8>) -> Result<Self, ProtocolError> {
        let mut reader = Cursor::new(buffer);

        let body = reader.read_string().await?;

        Ok(ResponsePacket { packet_id: 0, body })
    }
}
