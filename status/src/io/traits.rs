use std::time::Duration;

use async_trait::async_trait;

use crate::packet::traits::{ExpectedPacketId, PacketId};

use super::error::ProtocolError;

/// AsyncWireReadExt adds varint and varint-backed
/// string support to things that implement AsyncRead.
#[async_trait]
pub trait AsyncWireReadExt {
    async fn read_varint(&mut self) -> Result<usize, ProtocolError>;
    async fn read_string(&mut self) -> Result<String, ProtocolError>;
}

/// AsyncWireWriteExt adds varint and varint-backed
/// string support to things that implement AsyncWrite.
#[async_trait]
pub trait AsyncWireWriteExt {
    async fn write_varint(&mut self, int: usize) -> Result<(), ProtocolError>;
    async fn write_string(&mut self, string: &str) -> Result<(), ProtocolError>;
}

/// AsyncReadFromBuffer is used to allow
/// AsyncReadRawPacket to generically read a
/// packet's specific data from a buffer.
#[async_trait]
pub trait AsyncReadFromBuffer: Sized {
    async fn read_from_buffer(buffer: Vec<u8>) -> Result<Self, ProtocolError>;
}

/// AsyncWriteToBuffer is used to allow
/// AsyncWriteRawPacket to generically write a
/// packet's specific data into a buffer.
#[async_trait]
pub trait AsyncWriteToBuffer {
    async fn write_to_buffer(&self) -> Result<Vec<u8>, ProtocolError>;
}

/// AsyncReadRawPacket is the core piece of
/// the read side of the protocol. It allows
/// the user to construct a specific packet
/// from something that implements AsyncRead.
#[async_trait]
pub trait AsyncReadRawPacket {
    async fn read_packet<T: ExpectedPacketId + AsyncReadFromBuffer + Send + Sync>(
        &mut self,
    ) -> Result<T, ProtocolError>;

    async fn read_packet_with_timeout<T: ExpectedPacketId + AsyncReadFromBuffer + Send + Sync>(
        &mut self,
        timeout: Duration,
    ) -> Result<T, ProtocolError>;
}

/// AsyncWriteRawPacket is the core piece of
/// the write side of the protocol. It allows
/// the user to write a specific packet to
/// something that implements AsyncWrite.
#[async_trait]
pub trait AsyncWriteRawPacket {
    async fn write_packet<T: PacketId + AsyncWriteToBuffer + Send + Sync>(
        &mut self,
        packet: T,
    ) -> Result<(), ProtocolError>;

    async fn write_packet_with_timeout<T: PacketId + AsyncWriteToBuffer + Send + Sync>(
        &mut self,
        packet: T,
        timeout: Duration,
    ) -> Result<(), ProtocolError>;
}
