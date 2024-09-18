use valence::{
    abilities::AbilitiesPlugin,
    action::ActionPlugin,
    advancement::AdvancementPlugin,
    anvil::AnvilPlugin,
    app::{PluginGroup, PluginGroupBuilder},
    boss_bar::BossBarPlugin,
    client::ClientPlugin,
    client_command::ClientCommandPlugin,
    client_settings::ClientSettingsPlugin,
    command::manager::CommandPlugin,
    custom_payload::CustomPayloadPlugin,
    entity::{hitbox::HitboxPlugin, EntityPlugin},
    event_loop::EventLoopPlugin,
    hand_swing::HandSwingPlugin,
    interact_block::InteractBlockPlugin,
    interact_entity::InteractEntityPlugin,
    interact_item::InteractItemPlugin,
    inventory::InventoryPlugin,
    keepalive::KeepalivePlugin,
    layer::LayerPlugin,
    log::LogPlugin,
    message::MessagePlugin,
    movement::MovementPlugin,
    network::NetworkPlugin,
    op_level::OpLevelPlugin,
    player_list::PlayerListPlugin,
    registry::{biome::BiomePlugin, dimension_type::DimensionTypePlugin, RegistryPlugin},
    resource_pack::ResourcePackPlugin,
    scoreboard::ScoreboardPlugin,
    status::StatusPlugin,
    status_effect::StatusEffectPlugin,
    teleport::TeleportPlugin,
    weather::WeatherPlugin,
    world_border::WorldBorderPlugin,
    ServerPlugin,
};

/// This plugin group will add all the default plugins for a Valence
/// application.
pub struct ServerPlugins;

impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ServerPlugin)
            .add(RegistryPlugin)
            .add(BiomePlugin)
            .add(DimensionTypePlugin)
            .add(EntityPlugin)
            .add(HitboxPlugin)
            .add(LayerPlugin)
            .add(ClientPlugin)
            .add(EventLoopPlugin)
            .add(MovementPlugin)
            .add(ClientCommandPlugin)
            .add(KeepalivePlugin)
            .add(InteractEntityPlugin)
            .add(ClientSettingsPlugin)
            .add(ActionPlugin)
            .add(TeleportPlugin)
            .add(MessagePlugin)
            .add(CustomPayloadPlugin)
            .add(HandSwingPlugin)
            .add(InteractBlockPlugin)
            .add(InteractItemPlugin)
            .add(OpLevelPlugin)
            .add(ResourcePackPlugin)
            .add(StatusPlugin)
            .add(StatusEffectPlugin)
            .add(AbilitiesPlugin)
            .add(LogPlugin::default())
            .add(NetworkPlugin)
            .add(PlayerListPlugin)
            .add(InventoryPlugin)
            .add(AnvilPlugin)
            .add(AdvancementPlugin)
            .add(WeatherPlugin)
            .add(WorldBorderPlugin)
            .add(BossBarPlugin)
            .add(CommandPlugin)
            .add(ScoreboardPlugin)
    }
}
