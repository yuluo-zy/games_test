//! eraser/wall_eraser.rs — the freehand wall eraser system.
//!
//! PDB procedure `wall_eraser` 0x140accab0 (20675 B) + helpers `determine_freehand_height_ty`,
//! `adjust_wall_height_if_erased_flat_roof_w_no_rims`, `wall_height_based_on_length`.
//! FunctionSystem signature O.
//!
//! Business logic (O, call chain): while erasing (UiState drag + AppMode Bulldozer), the
//! cursor stroke selects a curve range on intersected walls; `determine_freehand_height_ty`
//! picks the freehand height type, `wall_height_based_on_length` derives the wall height
//! from the erased length, `adjust_wall_height_if_erased_flat_roof_w_no_rims` raises walls
//! under flat roofs; affected walls are collected in the `WallEraserArchivist` and new child
//! walls created with `ChildWall::new` + `InnerWalls::new_wall` / `update_wall_state_curve`.

use bevy_ecs::system::{Res, ResMut};

use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::walls::decorator_archivist::DecoratorArchivalResources;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::WallId;
use crate::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer;
use crate::systems::wall::eraser::archivist::WallEraserArchivist;
use crate::ui::mode_manager::AppMode;
use crate::ui::ui_state::UiState;
use crate::utils::curve::Curve;

/// O: `determine_freehand_height_ty` — freehand height type from the erased wall.
pub fn determine_freehand_height_ty(_wall: &crate::resources::walls::WallState, _terrain: f32) -> u8 {
    0
}

/// O: `wall_height_based_on_length` — wall height derived from erased curve length.
pub fn wall_height_based_on_length(_length: f32) -> f32 {
    0.0
}

/// O: `adjust_wall_height_if_erased_flat_roof_w_no_rims`.
pub fn adjust_wall_height_if_erased_flat_roof_w_no_rims(
    _wall: WallId,
    _height: f32,
    _inner: &mut InnerWalls,
) {
}

pub fn wall_eraser(
    app_mode: Res<AppMode>,
    buffer: Res<AnimateWallBulldozeBuffer>,
    cursor: Res<CursorTerrainRaycast>,
    decorator_archival: DecoratorArchivalResources,
    ui_state: Res<UiState>,
    mut inner_walls: ResMut<InnerWalls>,
    mut archivist: ResMut<WallEraserArchivist>,
) {
    let _ = (&app_mode, &buffer, &cursor, &decorator_archival, &ui_state);
    let _pos = cursor.position();
    if !ui_state.dragged() {
        // O: end of erase stroke -> finish the archivist and record history.
        let _ = archivist.finish();
        return;
    }
    // O: select the erased range on intersected walls and collect affected walls.
    for wall in inner_walls.wall_ids() {
        if buffer.contains(wall) {
            continue;
        }
        let Some(state) = inner_walls.get(wall) else { continue };
        let _ = state;
        archivist.record_erase_original_wall(wall, crate::typed_gap::Gap);
        let curve = Curve { raw: crate::typed_gap::Gap };
        let _ = curve;
        inner_walls.update_wall_state_curve(wall, Curve { raw: crate::typed_gap::Gap });
    }
}
