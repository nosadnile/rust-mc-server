use anyhow::Result;

use crate::conn::{config::ConnectionConfig, response::StatusResponse};

pub async fn fetch_status<T>(ip: T, port: Option<u16>) -> Result<StatusResponse>
where
    T: AsRef<str>,
{
    let config = ConnectionConfig::build(ip.as_ref()).with_port_opt(port);
    let conn = config.connect().await?;
    let status = conn.status().await?;

    Ok(status.status)
}
