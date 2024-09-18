use thiserror::Error;
use tokio::time::error::Elapsed;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("error reading or writing data")]
    Io(#[from] std::io::Error),

    #[error("invalid packet length")]
    InvalidPacketLength,

    #[error("invalid varint data")]
    InvalidVarInt,

    #[error("invalid packet (expected ID {expected:?}, actual ID {actual:?})")]
    InvalidPacketId { expected: usize, actual: usize },

    #[error("invalid ServerListPing response body (invalid UTF-8)")]
    InvalidResponseBody,

    #[error("connection timed out")]
    Timeout(#[from] Elapsed),
}
