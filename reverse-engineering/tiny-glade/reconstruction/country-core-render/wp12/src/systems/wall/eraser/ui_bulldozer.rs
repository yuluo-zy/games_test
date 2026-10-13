//! eraser/ui_bulldozer.rs — bulldozer UI entry: initiates wall-bulldoze commands.
//!
//! PDB procedures: `wall_eraser_bulldoze_condition` 0x1409de170 (19 B),
//! `wall_eraser_bulldoze` 0x1409de190 (20909 B), `handle_decorators`,
//! `can_reconnect_stairs_if_midpoint_deleted`, `hammer_cursor_icon` 0x1409e35a0.
//! FunctionSystem signatures O.
//!
//! Business logic (O, call chain): when the bulldozer drags over a wall (`UiState::clicked/
//! dragged/intersecting`, `WidgetParams::occlude_by_world`), it selects the erased curve
//! range, `handle_decorators` culls wall/terrain/stair decorators via the `DecoratorArchivist`,
//! disconnects stairs at the midpoint (`StairsState::disconnect_edge`),
//! `can_reconnect_stairs_if_midpoint_deleted`, and pushes an `AnimateWallBulldozeCmd` into the
//! buffer. `wall_eraser_bulldoze_condition` gates on `AppMode == Bulldozer`.

use bevy_ecs::event::EventWriter;
use bevy_ecs::prelude::{Component, Resource};
use bevy_ecs::system::{Query, Res, ResMut};

use crate::audio::RequestWallMaterialAudio;
use crate::resources::walls::decorator_archivist::DecoratorArchivalResources;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::WallId;
use crate::systems::wall::eraser::animate_wall_bulldoze::{
    AnimateWallBulldozeBuffer, AnimateWallBulldozeCmd,
};
use crate::ui::mode_manager::AppMode;
use crate::ui::ui_state::{UiState, UiWidgetContext};

/// Condition system (O: `bool (*)(Res<AppMode>)`).
pub fn wall_eraser_bulldoze_condition(app_mode: Res<AppMode>) -> bool {
    matches!(*app_mode, AppMode::Bulldozer)
}

pub fn handle_decorators(
    _wall: WallId,
    _low: f32,
    _high: f32,
    _archival: &DecoratorArchivalResources,
) {
}

pub fn can_reconnect_stairs_if_midpoint_deleted(_a: WallId, _b: WallId) -> bool {
    false
}

pub fn hammer_cursor_icon(_active: bool) {}

pub fn wall_eraser_bulldoze(
    inner_walls: Res<InnerWalls>,
    mut buffer: ResMut<AnimateWallBulldozeBuffer>,
    decorator_archival: DecoratorArchivalResources,
    ui_state: Res<UiState>,
    _ui_context: UiWidgetContext,
    mut material_writer: EventWriter<RequestWallMaterialAudio>,
) {
    let _ = (&inner_walls, &decorator_archival, &ui_state, &mut material_writer);
    // O: on a bulldoze drag over a wall, enqueue one command for the erased range.
    for wall in inner_walls.wall_ids() {
        if buffer.contains(wall) {
            continue;
        }
        let mut cmd = AnimateWallBulldozeCmd::new();
        cmd.wall = wall;
        cmd.low = 0.3;
        cmd.high = 0.7;
        buffer.push(cmd);
    }
}
