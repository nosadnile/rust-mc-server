use crate::default;

default!(host: String = #into "127.0.0.1");
default!(port: u16 = 25565);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,
}
