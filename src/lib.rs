#[macro_use]
extern crate serde;

pub mod config;
pub mod init;
pub mod macros;
pub mod ping;
pub mod plugins;
pub mod state;
pub mod update;
pub mod world;

use std::{fs, net::SocketAddr, path::PathBuf};

use anyhow::Result;
use bevy_ecs::system::Res;
use config::ServerConfig;
use init::{clients::init_clients, setup::setup};
use ping::PingCallbacks;
use plugins::ServerPlugins;
use tracing::info;
use update::{digging::digging, placing::place_blocks};

use valence::{
    app::{App, PostStartup, Startup, Update},
    client::despawn_disconnected_clients,
    prelude::{ConnectionMode, IntoSystemConfigs, NetworkSettings},
};
use world::net::{remove_unviewed_chunks, send_recv_chunks, update_client_views};

pub fn get_config() -> Result<ServerConfig> {
    Ok(toml::from_str(&fs::read_to_string("server.toml")?)?)
}

// Why is it async? For the `Future`. (Yes I plan to do async thingimabobs)
pub async fn init() -> Result<()> {
    if !PathBuf::from("server.toml").exists() {
        fs::write(
            "server.toml",
            toml::to_string_pretty(&toml::from_str::<ServerConfig>("")?)?,
        )?;
    }

    let config = get_config()?;

    // Migrate any missing values
    fs::write("server.toml", toml::to_string_pretty(&config)?)?;

    App::new()
        .insert_resource(NetworkSettings {
            connection_mode: ConnectionMode::Online {
                prevent_proxy_connections: true,
            },
            callbacks: PingCallbacks.into(),
            address: SocketAddr::new(config.host.parse()?, config.port),
            ..Default::default()
        })
        .add_plugins(ServerPlugins)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, show_startup)
        .add_systems(
            Update,
            (
                (
                    init_clients,
                    remove_unviewed_chunks,
                    update_client_views,
                    send_recv_chunks,
                )
                    .chain(),
                despawn_disconnected_clients,
                digging,
                place_blocks,
            ),
        )
        .run();

    Ok(())
}

pub fn show_startup(settings: Res<NetworkSettings>) {
    info!("Server listening on {}", settings.address);
}
