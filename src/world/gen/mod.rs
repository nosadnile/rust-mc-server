use math::{fbm, lerp, lerpstep, noise01};
use valence::math::DVec3;

use crate::state::ChunkWorkerState;

pub mod math;
pub mod worker;

pub const SPAWN_POS: DVec3 = DVec3::new(0.0, 200.0, 0.0);
pub const HEIGHT: u32 = 384;

pub fn has_terrain_at(state: &ChunkWorkerState, p: DVec3) -> bool {
    let hilly = lerp(0.1, 1.0, noise01(&state.hilly, p / 400.0)).powi(2);

    let lower = 15.0 + 100.0 * hilly;
    let upper = lower + 100.0 * hilly;

    if p.y <= lower {
        return true;
    } else if p.y >= upper {
        return false;
    }

    let density = 1.0 - lerpstep(lower, upper, p.y);

    let n = fbm(&state.density, p / 100.0, 4, 2.0, 0.5);

    n < density
}
