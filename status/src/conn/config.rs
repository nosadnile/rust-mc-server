use std::time::Duration;

use tokio::net::TcpStream;

use super::{error::ServerError, status::StatusConnection};

pub const LATEST_PROTOCOL_VERSION: usize = 764;
pub const DEFAULT_PORT: u16 = 25565;
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(2);

/// Builder for a Minecraft
/// ServerListPing connection.
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    protocol_version: usize,
    address: String,
    port: u16,
    timeout: Duration,
}

impl ConnectionConfig {
    /// Initiates the Minecraft server
    /// connection build process.
    pub fn build<T: Into<String>>(address: T) -> Self {
        ConnectionConfig {
            protocol_version: LATEST_PROTOCOL_VERSION,
            address: address.into(),
            port: DEFAULT_PORT,
            timeout: DEFAULT_TIMEOUT,
        }
    }

    /// Sets a specific
    /// protocol version for the connection to
    /// use. If not specified, the latest version
    /// will be used.
    pub fn with_protocol_version(mut self, protocol_version: usize) -> Self {
        self.protocol_version = protocol_version;
        self
    }

    /// Sets a specific port for the
    /// connection to use. If not specified, the
    /// default port of 25565 will be used.
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_port_opt(mut self, port: Option<u16>) -> Self {
        if let Some(port) = port {
            self.port = port;
        }

        self
    }

    /// Sets a specific timeout for the
    /// connection to use. If not specified, the
    /// timeout defaults to two seconds.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Connects to the server and consumes the builder.
    pub async fn connect(self) -> Result<StatusConnection, ServerError> {
        let stream = TcpStream::connect(format!("{}:{}", self.address, self.port))
            .await
            .map_err(|_| ServerError::FailedToConnect)?;

        Ok(StatusConnection {
            stream,
            protocol_version: self.protocol_version,
            address: self.address,
            port: self.port,
            timeout: self.timeout,
        })
    }
}
