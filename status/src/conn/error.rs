use thiserror::Error;

use crate::io::error::ProtocolError;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("error reading or writing data")]
    ProtocolError,

    #[error("failed to connect to server")]
    FailedToConnect,

    #[error("invalid JSON response: \"{0}\"")]
    InvalidJson(String),

    #[error("mismatched pong payload (expected \"{expected}\", got \"{actual}\")")]
    MismatchedPayload { expected: u64, actual: u64 },
}

impl From<ProtocolError> for ServerError {
    fn from(_err: ProtocolError) -> Self {
        ServerError::ProtocolError
    }
}
