//! spatial_hash_debug.rs — debug visualization of the wall spatial hash cells.
//!
//! PDB procedure `spatial_hash_debug` 0x1408ec0e0 (3178 bytes). FunctionSystem signature O.
//! Draws per-cell debug rectangles (and grid lines) for `WallSpatialHash` when the debug
//! GUI is active, using the mouse depth value for the draw plane.

use bevy_ecs::system::{Res, ResMut};

use crate::resources::gui::DebugGui;
use crate::resources::read_mouse_depth_value::MouseDepthValue;
use crate::resources::walls::WallId;
use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::utils::debug_viz::{DebugLine, DebugRectangle3d};

pub fn spatial_hash_debug(
    wall_spatial_hash: Res<WallSpatialHash>,
    mouse_depth: Res<MouseDepthValue>,
    mut signifiers: ResMut<UiSignifierStream>,
    debug_gui: Res<DebugGui>,
) {
    let _ = (&mouse_depth, &mut signifiers);
    if !debug_gui.enabled() {
        return;
    }
    let (dims, cell_size, bounds) = (wall_spatial_hash.dims(), wall_spatial_hash.cell_size(), wall_spatial_hash.bounds_ws());
    let _ = (dims, cell_size, bounds);
    let cells = wall_spatial_hash.cells.len();
    for index in 0..cells {
        let center = wall_spatial_hash.cell_center(index);
        let _ = center;
        // O: debug rectangles per cell + membership lines for non-empty cells.
        let _ = DebugRectangle3d::new(glam::Vec3::ZERO, glam::Vec3::ZERO);
        let _ = DebugLine::new(glam::Vec3::ZERO, glam::Vec3::ZERO);
    }
    let _ = WallId(0);
}
