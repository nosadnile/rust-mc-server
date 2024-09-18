/// PacketId is used to allow AsyncWriteRawPacket
/// to generically get a packet's ID.
pub trait PacketId {
    fn get_packet_id(&self) -> usize;
}

/// ExpectedPacketId is used to allow AsyncReadRawPacket
/// to generically get a packet's expected ID.
pub trait ExpectedPacketId {
    fn get_expected_packet_id() -> usize;
}
