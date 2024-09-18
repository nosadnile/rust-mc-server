/// RawPacket is the underlying wrapper of data that
/// gets read from and written to the socket.
///
/// Typically, the flow looks like this:
/// 1. Construct a specific packet (HandshakePacket
///   for example).
/// 2. Write that packet's contents to a byte buffer.
/// 3. Construct a RawPacket using that byte buffer.
/// 4. Write the RawPacket to the socket.
#[derive(Debug, Clone)]
pub struct RawPacket {
    pub id: usize,
    pub data: Box<[u8]>,
}

impl RawPacket {
    pub fn new(id: usize, data: Box<[u8]>) -> Self {
        RawPacket { id, data }
    }
}
