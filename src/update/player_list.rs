use bevy_ecs::{
    entity::Entity,
    query::{Added, With},
    system::{Commands, Query, Res, ResMut},
};
use valence::{
    client::Client,
    keepalive::Ping,
    player_list::{DisplayName, PlayerListEntryBundle},
    prelude::{PlayerList, PlayerListEntry},
    rand::{self, Rng},
    text::{Color, IntoText},
    uuid::Uuid,
    Despawned, Server, UniqueId,
};

const PLAYER_UUID_1: Uuid = Uuid::from_u128(1);
const PLAYER_UUID_2: Uuid = Uuid::from_u128(2);

pub fn override_display_name(
    mut clients: Query<&mut DisplayName, (Added<DisplayName>, With<Client>)>,
) {
    for mut display_name in &mut clients {
        display_name.0 = Some("ඞ".color(Color::rgb(255, 87, 66)));
    }
}

pub fn update_player_list(
    mut player_list: ResMut<PlayerList>,
    server: Res<Server>,
    mut entries: Query<(Entity, &UniqueId, &mut DisplayName), With<PlayerListEntry>>,
    mut commands: Commands,
) {
    let tick = server.current_tick();

    player_list.set_header("Current tick: ".into_text() + tick);
    player_list
        .set_footer("Current tick but in purple: ".into_text() + tick.color(Color::LIGHT_PURPLE));

    if tick % 5 == 0 {
        for (_, uuid, mut display_name) in &mut entries {
            if uuid.0 == PLAYER_UUID_1 {
                let mut rng = rand::thread_rng();
                let color = Color::rgb(rng.gen(), rng.gen(), rng.gen());

                let new_name = display_name.0.clone().unwrap_or_default().color(color);
                display_name.0 = Some(new_name);
            }
        }
    }

    if tick % 20 == 0 {
        if let Some((entity, _, _)) = entries.iter().find(|(_, uuid, _)| uuid.0 == PLAYER_UUID_2) {
            commands.entity(entity).insert(Despawned);
        } else {
            commands.spawn(PlayerListEntryBundle {
                uuid: UniqueId(PLAYER_UUID_2),
                display_name: DisplayName(Some("Hello!".into())),
                ping: Ping(300),
                ..Default::default()
            });
        }
    }
}
