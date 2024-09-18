use std::io::Cursor;

use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

use crate::{
    impl_packet_id,
    io::{error::ProtocolError, traits::AsyncWriteToBuffer},
};

#[derive(Debug, Clone)]
pub struct PingPacket {
    pub packet_id: usize,
    pub payload: u64,
}

impl PingPacket {
    pub fn new(payload: u64) -> Self {
        Self {
            packet_id: 1,
            payload,
        }
    }
}

#[async_trait]
impl AsyncWriteToBuffer for PingPacket {
    async fn write_to_buffer(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut buffer = Cursor::new(Vec::<u8>::new());

        buffer.write_u64(self.payload).await?;

        Ok(buffer.into_inner())
    }
}

impl_packet_id!(PingPacket);
