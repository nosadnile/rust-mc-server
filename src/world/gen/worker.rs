use std::sync::Arc;

use valence::{
    block::{PropName, PropValue},
    math::DVec3,
    prelude::{Chunk, UnloadedChunk},
    BlockState,
};

use crate::{
    state::ChunkWorkerState,
    world::gen::{
        has_terrain_at,
        math::{fbm, noise01},
    },
};

use super::HEIGHT;

pub fn chunk_worker(state: Arc<ChunkWorkerState>) {
    while let Ok(pos) = state.receiver.recv() {
        let mut chunk = UnloadedChunk::with_height(HEIGHT);

        for offset_z in 0..16 {
            for offset_x in 0..16 {
                let x = offset_x as i32 + pos.x * 16;
                let z = offset_z as i32 + pos.z * 16;

                let mut in_terrain = false;
                let mut depth = 0;

                // Fill in the terrain column.
                for y in (0..chunk.height() as i32).rev() {
                    const WATER_HEIGHT: i32 = 55;

                    let p = DVec3::new(f64::from(x), f64::from(y), f64::from(z));

                    let block = if has_terrain_at(&state, p) {
                        let gravel_height = WATER_HEIGHT
                            - 1
                            - (fbm(&state.gravel, p / 10.0, 3, 2.0, 0.5) * 6.0).floor() as i32;

                        if in_terrain {
                            if depth > 0 {
                                depth -= 1;
                                if y < gravel_height {
                                    BlockState::GRAVEL
                                } else {
                                    BlockState::DIRT
                                }
                            } else {
                                BlockState::STONE
                            }
                        } else {
                            in_terrain = true;
                            let n = noise01(&state.stone, p / 15.0);

                            depth = (n * 5.0).round() as u32;

                            if y < gravel_height {
                                BlockState::GRAVEL
                            } else if y < WATER_HEIGHT - 1 {
                                BlockState::DIRT
                            } else {
                                BlockState::GRASS_BLOCK
                            }
                        }
                    } else {
                        in_terrain = false;
                        depth = 0;
                        if y < WATER_HEIGHT {
                            BlockState::WATER
                        } else {
                            BlockState::AIR
                        }
                    };

                    chunk.set_block_state(offset_x, y as u32, offset_z, block);
                }

                // Add grass on top of grass blocks.
                for y in (0..chunk.height()).rev() {
                    if chunk.block_state(offset_x, y, offset_z).is_air()
                        && chunk.block_state(offset_x, y - 1, offset_z) == BlockState::GRASS_BLOCK
                    {
                        let p = DVec3::new(f64::from(x), f64::from(y), f64::from(z));
                        let density = fbm(&state.grass, p / 5.0, 4, 2.0, 0.7);

                        if density > 0.55 {
                            if density > 0.7
                                && chunk.block_state(offset_x, y + 1, offset_z).is_air()
                            {
                                let upper =
                                    BlockState::TALL_GRASS.set(PropName::Half, PropValue::Upper);
                                let lower =
                                    BlockState::TALL_GRASS.set(PropName::Half, PropValue::Lower);

                                chunk.set_block_state(offset_x, y + 1, offset_z, upper);
                                chunk.set_block_state(offset_x, y, offset_z, lower);
                            } else {
                                chunk.set_block_state(offset_x, y, offset_z, BlockState::GRASS);
                            }
                        }
                    }
                }
            }
        }

        let _ = state.sender.try_send((pos, chunk));
    }
}
