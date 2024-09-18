use serde::Deserialize;

use super::data::{ServerDescription, ServerPlayers, ServerVersion};

/// The decoded JSON response from a status query over
/// ServerListPing.
#[derive(Debug, Clone, Deserialize)]
pub struct StatusResponse {
    /// Information about the server's version.
    pub version: ServerVersion,

    /// Information about currently online players.
    pub players: ServerPlayers,

    /// Single-field struct containing the server's MOTD.
    pub description: ServerDescription,

    /// Optional field containing a path to the server's
    /// favicon.
    pub favicon: Option<String>,
}
