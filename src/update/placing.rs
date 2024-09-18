use valence::{
    block::{BlockKind, PropName, PropValue},
    interact_block::InteractBlockEvent,
    inventory::HeldItem,
    prelude::{EventReader, Inventory, Query},
    ChunkLayer, Direction, GameMode, Hand, ItemStack,
};

pub fn place_blocks(
    mut clients: Query<(&mut Inventory, &GameMode, &HeldItem)>,
    mut layers: Query<&mut ChunkLayer>,
    mut events: EventReader<InteractBlockEvent>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        let Ok((mut inventory, game_mode, held)) = clients.get_mut(event.client) else {
            continue;
        };

        if event.hand != Hand::Main {
            continue;
        }

        // get the held item
        let slot_id = held.slot();
        let stack = inventory.slot(slot_id);

        if stack.is_empty() {
            continue;
        }

        let Some(block_kind) = BlockKind::from_item_kind(stack.item) else {
            continue;
        };

        if *game_mode == GameMode::Survival {
            if stack.count > 1 {
                let amount = stack.count - 1;
                inventory.set_slot_amount(slot_id, amount);
            } else {
                inventory.set_slot(slot_id, ItemStack::EMPTY);
            }
        }

        let real_pos = event.position.get_in_direction(event.face);

        let state = block_kind.to_state().set(
            PropName::Axis,
            match event.face {
                Direction::Down | Direction::Up => PropValue::Y,
                Direction::North | Direction::South => PropValue::Z,
                Direction::West | Direction::East => PropValue::X,
            },
        );

        layer.set_block(real_pos, state);
    }
}
