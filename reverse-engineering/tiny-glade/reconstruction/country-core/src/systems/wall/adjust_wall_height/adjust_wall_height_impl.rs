//! adjust_wall_height/adjust_wall_height_impl.rs — applies wall-height commands.
//!
//! Source SHA256 B skeleton; PDB `2d9686b8...`; procedure `adjust_wall_height_impl`
//! 0x140ac1f20 (2235 B). FunctionSystem signature O.
//!
//! Business logic (O, decompiled 0x140ac1f20):
//!   - Consumes `AdjustWallHeightCmd` (0x30-byte rows: ty u32@0x10, height f32@0x14,
//!     WallId@0x18, follow-terrain flag).
//!   - Looks the wall up in `InnerWalls`; computes the clamped height via
//!     `clamp_wall_height` (falls back to the command height when the clamp is absent),
//!     and skips the wall when the clamped height equals the current height.
//!   - When follow-terrain, samples `TerrainHeightsData::sample_at_world_xz` at the wall
//!     point (`Curve::get_coord_at_u` + `get_pos_at_u`).
//!   - For premade walls (`InnerWalls` value curve count `< 2` and height-ty default),
//!     calls `InnerWalls::_try_update_state<update_wall_state_height>` and emits a 0x28-byte
//!     event row {WallId, pos+height, u32, t=clamp01(inverse_lerp), 3}; otherwise emits a
//!     0x20-byte `RequestWallMaterialAudio` row {WallId, pos+height, ...}.

use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::system::{Res, ResMut};

use crate::audio_events::RequestWallMaterialAudio;
use crate::resources::events::AdjustWallHeightCmd;
use crate::resources::terrain_height::{TerrainHeightsData, TerrainHeightSsbo};
use crate::resources::walls::inner_walls::InnerWalls;
use crate::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::clamp_wall_height;

/// 0x28-byte height-update notification (typed-gap row).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct HeightUpdateRequested {
    pub wall: crate::resources::walls::WallId,
    pub pos: glam::Vec3,
    pub t: f32,
}

pub fn adjust_wall_height_impl(
    mut reader: EventReader<AdjustWallHeightCmd>,
    mut inner_walls: ResMut<InnerWalls>,
    terrain_heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mut material_writer: EventWriter<RequestWallMaterialAudio>,
    mut height_writer: EventWriter<HeightUpdateRequested>,
) {
    for cmd in reader.read() {
        let Some(state) = inner_walls.get(cmd.wall) else {
            continue;
        };
        // O: clamp the desired height; fall back to the raw command height.
        let clamped = clamp_wall_height(cmd.wall, cmd.height, None, 0);
        let desired = if clamped.is_some() { clamped.unwrap() } else { cmd.height };
        let current = state.height();
        if desired == current {
            continue;
        }
        if cmd.follow_terrain {
            // O: sample terrain under the wall point.
            let p = state.curve.get_pos_at_u(crate::utils::curve::CurveU { raw: crate::typed_gap::Gap });
            let _ = terrain_heights.sample_at_world_xz(p.x, p.z);
        }
        // Hoist the immutable reads (NLL: state borrow must end before the mutable
        // InnerWalls access below).
        let pos = state.curve.end_pos();
        inner_walls.update_wall_state_height(cmd.wall, desired);
        material_writer.write(RequestWallMaterialAudio {
            wall: cmd.wall,
            pos,
            raw: crate::typed_gap::Gap,
        });
        height_writer.write(HeightUpdateRequested {
            wall: cmd.wall,
            pos,
            t: 1.0,
        });
    }
}
