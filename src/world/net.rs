use std::collections::hash_map::Entry;

use valence::{
    client::{Client, OldView, View},
    prelude::{DetectChanges, Query, ResMut},
    ChunkLayer, ChunkPos,
};

use crate::state::GameState;

pub fn remove_unviewed_chunks(mut layers: Query<&mut ChunkLayer>) {
    layers
        .single_mut()
        .retain_chunks(|_, chunk| chunk.viewer_count_mut() > 0);
}

pub fn update_client_views(
    mut layers: Query<&mut ChunkLayer>,
    mut clients: Query<(&mut Client, View, OldView)>,
    mut state: ResMut<GameState>,
) {
    let layer = layers.single_mut();

    for (client, view, old_view) in &mut clients {
        let view = view.get();
        let queue_pos = |pos: ChunkPos| {
            if layer.chunk(pos).is_none() {
                match state.pending.entry(pos) {
                    Entry::Occupied(mut oe) => {
                        if let Some(priority) = oe.get_mut() {
                            let dist = view.pos.distance_squared(pos);
                            *priority = (*priority).min(dist);
                        }
                    }
                    Entry::Vacant(ve) => {
                        let dist = view.pos.distance_squared(pos);
                        ve.insert(Some(dist));
                    }
                }
            }
        };

        // Queue all the new chunks in the view to be sent to the thread pool.
        if client.is_added() {
            view.iter().for_each(queue_pos);
        } else {
            let old_view = old_view.get();
            if old_view != view {
                view.diff(old_view).for_each(queue_pos);
            }
        }
    }
}

pub fn send_recv_chunks(mut layers: Query<&mut ChunkLayer>, state: ResMut<GameState>) {
    let mut layer = layers.single_mut();
    let state = state.into_inner();

    // Insert the chunks that are finished generating into the instance.
    for (pos, chunk) in state.receiver.drain() {
        layer.insert_chunk(pos, chunk);
        assert!(state.pending.remove(&pos).is_some());
    }

    // Collect all the new chunks that need to be loaded this tick.
    let mut to_send = vec![];

    for (pos, priority) in &mut state.pending {
        if let Some(pri) = priority.take() {
            to_send.push((pri, pos));
        }
    }

    // Sort chunks by ascending priority.
    to_send.sort_unstable_by_key(|(pri, _)| *pri);

    // Send the sorted chunks to be loaded.
    for (_, pos) in to_send {
        let _ = state.sender.try_send(*pos);
    }
}
