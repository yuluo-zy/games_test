//! calc_wall_bend.rs — recomputes a wall's bend angle after any wall change.
//!
//! Source SHA256 `afaeda6144...`; PDB `2d9686b8...`; procedure `calc_wall_bend` 0x140a8a710
//! (B patched EXE). FunctionSystem signature O (enriched file_interface_candidates).
//!
//! Business logic (O, decompiled 0x140a8a710):
//!   1. puffin scope.
//!   2. For each `OnWallChanged` wall id: `PublicWalls::get` (hashbrown lookup). Missing
//!      key -> hashbrown's not-found exit (no event).
//!   3. Validate the public wall: source point count must be `>= 2` and source curve length
//!      `> 0`; otherwise panic (`core::panicking::panic`).
//!   4. Read the published curve point Vec (map value +0x48/+0x50). If point count `> 5`:
//!      normalise the first four segment deltas, accumulate the three consecutive dot
//!      products, average (`sum / (n-1)`), clamp to [-1,1], `acosf`, then clamp the angle to
//!      `<= 1.0`. Otherwise the bend is `0.0`.
//!   5. Emit `WallBendChangedEvent { bend }` for every processed wall (EventWriter push).

use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::system::Res;

use crate::puffin_contract::{are_scopes_on, ProfilerScope};
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::wall::wall_state::publish_public_wall_state::OnWallChanged;

/// Wall bend event (O: 16-byte `EventInstance` row; payload = f32 bend @+8).
#[derive(Clone, Copy, Debug, Default)]
pub struct WallBendChangedEvent {
    pub bend: f32,
}

/// O: validated wall lookup — count `>= 2`, curve length `> 0` (panic guard).
fn validate_public_wall(count: u64, source_curve_length: f32) {
    assert!(count >= 2 && source_curve_length > 0.0);
}

/// O: bend from the first four normalised segment deltas of the published curve.
fn compute_bend(points: &[glam::Vec3]) -> f32 {
    if points.len() <= 5 {
        return 0.0;
    }
    let mut dirs: Vec<glam::Vec2> = Vec::with_capacity(4);
    for i in 0..4 {
        let a = points[i];
        let b = points[i + 1];
        let d = b - a;
        let len = (d.y * d.y + d.x * d.x).sqrt();
        if len == 0.0 {
            dirs.push(glam::Vec2::ZERO);
        } else {
            dirs.push(glam::Vec2::new(d.x / len, d.y / len));
        }
    }
    let mut dot_sum = 0.0f32;
    for i in 0..dirs.len().saturating_sub(1) {
        dot_sum += dirs[i].dot(dirs[i + 1]);
    }
    let avg = dot_sum / (dirs.len() as f32 - 1.0);
    let clamped = avg.clamp(-1.0, 1.0);
    clamped.acos().min(1.0)
}

pub fn calc_wall_bend(
    mut reader: EventReader<OnWallChanged>,
    public_walls: Res<PublicWalls>,
    mut writer: EventWriter<WallBendChangedEvent>,
) {
    let _scope = if are_scopes_on() {
        Some(ProfilerScope::new("calc_wall_bend"))
    } else {
        None
    };
    for OnWallChanged(wall) in reader.read() {
        let Some(state) = public_walls.get(*wall) else {
            continue;
        };
        let count = state.point_count();
        let length = state.source_curve_length();
        validate_public_wall(count, length);
        let bend = compute_bend(state.published_curve_points());
        writer.write(WallBendChangedEvent { bend });
    }
}
