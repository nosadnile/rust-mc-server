use std::{io::Cursor, time::Duration};

use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::packet::{
    raw::RawPacket,
    traits::{ExpectedPacketId, PacketId},
};

use super::{
    error::ProtocolError,
    traits::{
        AsyncReadFromBuffer, AsyncReadRawPacket, AsyncWireReadExt, AsyncWireWriteExt,
        AsyncWriteRawPacket, AsyncWriteToBuffer,
    },
};

#[async_trait]
impl<R: AsyncRead + Unpin + Send + Sync> AsyncWireReadExt for R {
    async fn read_varint(&mut self) -> Result<usize, ProtocolError> {
        let mut read = 0;
        let mut result = 0;
        loop {
            let read_value = self.read_u8().await?;
            let value = read_value & 0b0111_1111;
            result |= (value as usize) << (7 * read);
            read += 1;
            if read > 5 {
                return Err(ProtocolError::InvalidVarInt);
            }
            if (read_value & 0b1000_0000) == 0 {
                return Ok(result);
            }
        }
    }

    async fn read_string(&mut self) -> Result<String, ProtocolError> {
        let length = self.read_varint().await?;

        let mut buffer = vec![0; length];
        self.read_exact(&mut buffer).await?;

        Ok(String::from_utf8(buffer).map_err(|_| ProtocolError::InvalidResponseBody)?)
    }
}

#[async_trait]
impl<W: AsyncWrite + Unpin + Send + Sync> AsyncWireWriteExt for W {
    async fn write_varint(&mut self, int: usize) -> Result<(), ProtocolError> {
        let mut int = (int as u64) & 0xFFFF_FFFF;
        let mut written = 0;
        let mut buffer = [0; 5];
        loop {
            let temp = (int & 0b0111_1111) as u8;
            int >>= 7;
            if int != 0 {
                buffer[written] = temp | 0b1000_0000;
            } else {
                buffer[written] = temp;
            }
            written += 1;
            if int == 0 {
                break;
            }
        }

        self.write(&buffer[0..written]).await?;

        Ok(())
    }

    async fn write_string(&mut self, string: &str) -> Result<(), ProtocolError> {
        self.write_varint(string.len()).await?;
        self.write_all(string.as_bytes()).await?;

        Ok(())
    }
}

#[async_trait]
impl<R: AsyncRead + Unpin + Send + Sync> AsyncReadRawPacket for R {
    async fn read_packet<T: ExpectedPacketId + AsyncReadFromBuffer + Send + Sync>(
        &mut self,
    ) -> Result<T, ProtocolError> {
        let length = self.read_varint().await?;

        if length == 0 {
            return Err(ProtocolError::InvalidPacketLength);
        }

        let packet_id = self.read_varint().await?;

        let expected_packet_id = T::get_expected_packet_id();

        if packet_id != expected_packet_id {
            return Err(ProtocolError::InvalidPacketId {
                expected: expected_packet_id,
                actual: packet_id,
            });
        }

        let mut buffer = vec![0; length - 1];

        self.read_exact(&mut buffer).await?;

        T::read_from_buffer(buffer).await
    }

    async fn read_packet_with_timeout<T: ExpectedPacketId + AsyncReadFromBuffer + Send + Sync>(
        &mut self,
        timeout: Duration,
    ) -> Result<T, ProtocolError> {
        tokio::time::timeout(timeout, self.read_packet()).await?
    }
}

#[async_trait]
impl<W: AsyncWrite + Unpin + Send + Sync> AsyncWriteRawPacket for W {
    async fn write_packet<T: PacketId + AsyncWriteToBuffer + Send + Sync>(
        &mut self,
        packet: T,
    ) -> Result<(), ProtocolError> {
        let packet_buffer = packet.write_to_buffer().await?;

        let raw_packet = RawPacket::new(packet.get_packet_id(), packet_buffer.into_boxed_slice());

        let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        buffer.write_varint(raw_packet.id).await?;
        buffer.write_all(&raw_packet.data).await?;

        let inner = buffer.into_inner();

        self.write_varint(inner.len()).await?;
        self.write(&inner).await?;

        Ok(())
    }

    async fn write_packet_with_timeout<T: PacketId + AsyncWriteToBuffer + Send + Sync>(
        &mut self,
        packet: T,
        timeout: Duration,
    ) -> Result<(), ProtocolError> {
        tokio::time::timeout(timeout, self.write_packet(packet)).await?
    }
}
