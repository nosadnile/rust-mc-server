use async_trait::async_trait;

use crate::{
    impl_default_from_new, impl_packet_id,
    io::{error::ProtocolError, traits::AsyncWriteToBuffer},
};

/// RequestPacket is the second of two packets
/// to be sent during a status check for
/// ServerListPing.
#[derive(Debug, Clone)]
pub struct RequestPacket {
    pub packet_id: usize,
}

impl RequestPacket {
    pub fn new() -> Self {
        Self { packet_id: 0 }
    }
}

#[async_trait]
impl AsyncWriteToBuffer for RequestPacket {
    async fn write_to_buffer(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(Vec::new())
    }
}

impl_default_from_new!(RequestPacket);
impl_packet_id!(RequestPacket);
