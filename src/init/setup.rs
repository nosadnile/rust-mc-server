use std::{collections::HashMap, sync::Arc, thread, time::SystemTime};

use bevy_ecs::system::{Commands, Res};
use noise::SuperSimplex;
use tracing::info;
use valence::{
    ident,
    prelude::{BiomeRegistry, DimensionTypeRegistry},
    LayerBundle, Server,
};

use crate::{
    state::{ChunkWorkerState, GameState},
    world::gen::worker::chunk_worker,
};

pub fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    let seconds_per_day = 86_400;

    let seed = (SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / seconds_per_day) as u32;

    info!("Current seed: {}", seed);

    let (finished_sender, finished_receiver) = flume::unbounded();
    let (pending_sender, pending_receiver) = flume::unbounded();

    let state = Arc::new(ChunkWorkerState {
        sender: finished_sender,
        receiver: pending_receiver,
        density: SuperSimplex::new(seed),
        hilly: SuperSimplex::new(seed.wrapping_add(1)),
        stone: SuperSimplex::new(seed.wrapping_add(2)),
        gravel: SuperSimplex::new(seed.wrapping_add(3)),
        grass: SuperSimplex::new(seed.wrapping_add(4)),
    });

    // Chunks are generated in a thread pool for parallelism and to avoid blocking
    // the main tick loop. You can use your thread pool of choice here (rayon,
    // bevy_tasks, etc). Only the standard library is used in the example for the
    // sake of simplicity.
    //
    // If your chunk generation algorithm is inexpensive then there's no need to do
    // this.
    for _ in 0..thread::available_parallelism().unwrap().get() {
        let state = state.clone();
        thread::spawn(move || chunk_worker(state));
    }

    commands.insert_resource(GameState {
        pending: HashMap::new(),
        sender: pending_sender,
        receiver: finished_receiver,
    });

    let layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    info!("Spawning overworld layer!");

    commands.spawn(layer);
}
