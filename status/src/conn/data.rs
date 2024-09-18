use serde::Deserialize;

/// Contains information about the server version.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerVersion {
    /// The server's Minecraft version, i.e. "1.15.2".
    pub name: String,

    /// The server's ServerListPing protocol version.
    pub protocol: u32,
}

/// Contains information about a player.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerPlayer {
    /// The player's in-game name.
    pub name: String,

    /// The player's UUID.
    pub id: String,
}

/// Contains information about the currently online
/// players.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerPlayers {
    /// The configured maximum number of players for the
    /// server.
    pub max: u32,

    /// The number of players currently online.
    pub online: u32,

    /// An optional list of player information for
    /// currently online players.
    pub sample: Option<Vec<ServerPlayer>>,
}

/// Contains the server's MOTD.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ServerDescription {
    Plain(String),
    Object { text: String },
}
