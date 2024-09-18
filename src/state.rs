use std::collections::HashMap;

use flume::{Receiver, Sender};
use noise::SuperSimplex;
use valence::{
    prelude::{Resource, UnloadedChunk},
    ChunkPos,
};

/// The order in which chunks should be processed by the thread pool. Smaller
/// values are sent first.
pub type Priority = u64;

#[derive(Resource)]
pub struct GameState {
    /// Chunks that need to be generated. Chunks without a priority have already
    /// been sent to the thread pool.
    pub pending: HashMap<ChunkPos, Option<Priority>>,
    pub sender: Sender<ChunkPos>,
    pub receiver: Receiver<(ChunkPos, UnloadedChunk)>,
}

pub struct ChunkWorkerState {
    pub sender: Sender<(ChunkPos, UnloadedChunk)>,
    pub receiver: Receiver<ChunkPos>,
    // Noise functions
    pub density: SuperSimplex,
    pub hilly: SuperSimplex,
    pub stone: SuperSimplex,
    pub gravel: SuperSimplex,
    pub grass: SuperSimplex,
}
