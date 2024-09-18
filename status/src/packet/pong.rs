use std::io::Cursor;

use async_trait::async_trait;
use tokio::io::AsyncReadExt;

use crate::{
    impl_packet_id,
    io::{error::ProtocolError, traits::AsyncReadFromBuffer},
};

use super::traits::ExpectedPacketId;

#[derive(Debug, Clone)]
pub struct PongPacket {
    pub packet_id: usize,
    pub payload: u64,
}

impl ExpectedPacketId for PongPacket {
    fn get_expected_packet_id() -> usize {
        1
    }
}

#[async_trait]
impl AsyncReadFromBuffer for PongPacket {
    async fn read_from_buffer(buffer: Vec<u8>) -> Result<Self, ProtocolError> {
        let mut reader = Cursor::new(buffer);

        let payload = reader.read_u64().await?;

        Ok(PongPacket {
            packet_id: 0,
            payload,
        })
    }
}

impl_packet_id!(PongPacket);
