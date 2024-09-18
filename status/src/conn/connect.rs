use super::{config::ConnectionConfig, error::ServerError, status::StatusConnection};

/// Convenience wrapper for easily connecting
/// to a server on the default port with
/// the latest protocol version.
pub async fn connect(address: String) -> Result<StatusConnection, ServerError> {
    ConnectionConfig::build(address).connect().await
}
