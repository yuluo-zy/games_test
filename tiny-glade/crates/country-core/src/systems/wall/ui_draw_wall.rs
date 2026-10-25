//! B wall/fence input state machine, 0x140aafe40. This CPU edit producer uses
//! the existing UiState, border, archive and shared InProgressCurve owner.
//! The separate water-hint signifier closure is a renderer boundary.
use bevy_ecs::{prelude::*, system::SystemParam};
use glam::Vec2;

use crate::{
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder},
        in_progress_curve::InProgressCurve,
        tree_storage::HideTreesNextToWallCmd,
        walls::{decorator_archivist::LiveDecoratorArchivist, decorator_storage::DecoratorStorage},
    },
    systems::{
        audio::WallBendAudioParamReset,
        history::history_process::DebugHistoryMsg,
        mode_manager::{AppMode, WallShape},
        ui::resources::ui_state::UiState,
        wall::{
            draw_wall::{CurveNursery, RecordMouseInputIntoWall},
            finish_wall::FinishWall,
            ui_continue_wall::SuppressContinueWall,
        },
    },
};

#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct WallDrawState {
    /// Native byte: 0 idle, 1 start, 2 drawing, 3 finish.
    pub state: u8,
}

#[derive(SystemParam)]
pub struct WallDrawUiEffects<'w> {
    record: EventWriter<'w, RecordMouseInputIntoWall>,
    finish: EventWriter<'w, FinishWall>,
    debug: EventWriter<'w, DebugHistoryMsg>,
    bend: EventWriter<'w, WallBendAudioParamReset>,
    outside: EventWriter<'w, DisplayOutOfBorder>,
    feedback: EventWriter<'w, FeedbackInputOutOfBorder>,
    hide_trees: EventWriter<'w, HideTreesNextToWallCmd>,
}

pub fn ui_draw_wall(
    mode: Res<AppMode>,
    cursor: Res<CursorTerrainRaycast>,
    ui: Res<UiState>,
    border: Res<GladeBorder>,
    mut state: ResMut<WallDrawState>,
    mut nursery: ResMut<CurveNursery>,
    active: Res<InProgressCurve>,
    mut suppress: ResMut<SuppressContinueWall>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    storage: Res<DecoratorStorage>,
    mut effects: WallDrawUiEffects,
) {
    let fence = match *mode {
        AppMode::Wall(WallShape::Freeform) => false,
        AppMode::Fences => true,
        _ => return,
    };
    let context = ui.clearing_canvas_w_layer_and_sound(if fence { "fences" } else { "wall" }, 1, 3);
    let point = if cursor.tag & 1 != 0 {
        Vec2::new(cursor.pos.x, cursor.pos.z)
    } else {
        Vec2::ZERO
    };
    let inside = border.is_pos_inside(point);
    if !inside {
        effects.outside.write(DisplayOutOfBorder);
    }
    match context.dragged().wrapping_sub(2).min(3) {
        0 => {}
        1 => {
            if inside {
                state.state = 1;
            } else {
                effects.feedback.write(FeedbackInputOutOfBorder);
            }
            effects.bend.write(WallBendAudioParamReset);
        }
        2 => {
            if inside {
                state.state = if state.state.wrapping_sub(1) < 2 {
                    2
                } else {
                    0
                };
            } else {
                effects.feedback.write(FeedbackInputOutOfBorder);
                if state.state.wrapping_sub(1) < 2 {
                    state.state = 3;
                }
            }
        }
        _ => {
            if state.state.wrapping_sub(1) < 2 {
                state.state = 3;
            }
        }
    }
    match state.state {
        1 => {
            if let Some(terrain) = storage.terrain.as_ref() {
                archivist.archive_storage_decorators(1, terrain, None);
                archivist
                    .culling
                    .allowed_ids
                    .extend(terrain.decorators().keys().copied());
            } else {
                // Original archive_current records an empty before bucket too.
                archivist.archive_storage_decorators(1, &Default::default(), None);
            }
            nursery.points.clear();
            nursery.kind = u8::from(fence);
            effects.record.write(RecordMouseInputIntoWall);
        }
        2 => {
            effects.record.write(RecordMouseInputIntoWall);
            if let Some(wall) = active.wall {
                effects.hide_trees.write(HideTreesNextToWallCmd {
                    wall,
                    finished: false,
                });
            }
        }
        3 => {
            effects.finish.write(FinishWall(true));
            effects.debug.write(DebugHistoryMsg);
            state.state = 0;
            suppress.suppressed = true;
            if let Some(wall) = active.wall {
                effects.hide_trees.write(HideTreesNextToWallCmd {
                    wall,
                    finished: true,
                });
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{glade_border::BorderShape, walls::WallId};
    use crate::systems::ui::resources::ui_state::{UiInteraction, WidgetId};
    use glam::Vec3;
    use utils::geometry::Circle2d;

    #[test]
    fn original_entry_320_state_and_event_port_fixtures() {
        // Native entry cases resolve only border/UI registry/archival dependency
        // boundaries. This test executes the real current World and UI owner.
        let cases: &[(u32, u8, u8, u8, bool, bool, u8, usize, u8, u8, [usize; 7])] = &[
            (6, 0, 0, 0, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 1, 0, 0]),
            (6, 0, 0, 0, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 1, 0, 0]),
            (6, 0, 0, 0, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 0, 0, 0, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 0, 0, 1, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 1, 0, 0]),
            (6, 0, 0, 1, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 1, 0, 0]),
            (6, 0, 0, 1, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 0, 0, 1, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 0, 0, 2, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 1, 0, 0]),
            (6, 0, 0, 2, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 1, 0, 0]),
            (6, 0, 0, 2, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 0, 0, 2, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 0, 0, 3, false, false, 0, 2, 1, 0, [0, 0, 0, 1, 1, 1, 0]),
            (6, 0, 0, 3, false, true, 0, 2, 1, 0, [0, 0, 0, 1, 1, 1, 0]),
            (6, 0, 0, 3, true, false, 1, 0, 0, 0, [1, 0, 0, 1, 0, 0, 0]),
            (6, 0, 0, 3, true, true, 1, 0, 0, 0, [1, 0, 0, 1, 0, 0, 0]),
            (6, 0, 0, 4, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 1, 1, 0]),
            (6, 0, 0, 4, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 1, 1, 0]),
            (6, 0, 0, 4, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 0, 0, 4, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 0, 1, 0, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (6, 0, 1, 0, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (6, 0, 1, 0, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (6, 0, 1, 0, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (6, 0, 1, 1, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (6, 0, 1, 1, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (6, 0, 1, 1, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (6, 0, 1, 1, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (6, 0, 1, 2, false, false, 1, 0, 0, 0, [1, 0, 0, 0, 1, 0, 0]),
            (6, 0, 1, 2, false, true, 1, 0, 0, 0, [1, 0, 0, 0, 1, 0, 0]),
            (6, 0, 1, 2, true, false, 1, 0, 0, 0, [1, 0, 0, 0, 0, 0, 0]),
            (6, 0, 1, 2, true, true, 1, 0, 0, 0, [1, 0, 0, 0, 0, 0, 0]),
            (6, 0, 1, 3, false, false, 1, 0, 0, 0, [1, 0, 0, 1, 1, 1, 0]),
            (6, 0, 1, 3, false, true, 1, 0, 0, 0, [1, 0, 0, 1, 1, 1, 0]),
            (6, 0, 1, 3, true, false, 1, 0, 0, 0, [1, 0, 0, 1, 0, 0, 0]),
            (6, 0, 1, 3, true, true, 1, 0, 0, 0, [1, 0, 0, 1, 0, 0, 0]),
            (6, 0, 1, 4, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 1, 0]),
            (6, 0, 1, 4, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 1, 1]),
            (6, 0, 1, 4, true, false, 2, 2, 1, 0, [1, 0, 0, 0, 0, 0, 0]),
            (6, 0, 1, 4, true, true, 2, 2, 1, 0, [1, 0, 0, 0, 0, 0, 1]),
            (6, 0, 2, 0, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (6, 0, 2, 0, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (6, 0, 2, 0, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (6, 0, 2, 0, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (6, 0, 2, 1, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (6, 0, 2, 1, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (6, 0, 2, 1, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (6, 0, 2, 1, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (6, 0, 2, 2, false, false, 2, 2, 1, 0, [1, 0, 0, 0, 1, 0, 0]),
            (6, 0, 2, 2, false, true, 2, 2, 1, 0, [1, 0, 0, 0, 1, 0, 1]),
            (6, 0, 2, 2, true, false, 2, 2, 1, 0, [1, 0, 0, 0, 0, 0, 0]),
            (6, 0, 2, 2, true, true, 2, 2, 1, 0, [1, 0, 0, 0, 0, 0, 1]),
            (6, 0, 2, 3, false, false, 2, 2, 1, 0, [1, 0, 0, 1, 1, 1, 0]),
            (6, 0, 2, 3, false, true, 2, 2, 1, 0, [1, 0, 0, 1, 1, 1, 1]),
            (6, 0, 2, 3, true, false, 1, 0, 0, 0, [1, 0, 0, 1, 0, 0, 0]),
            (6, 0, 2, 3, true, true, 1, 0, 0, 0, [1, 0, 0, 1, 0, 0, 0]),
            (6, 0, 2, 4, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 1, 0]),
            (6, 0, 2, 4, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 1, 1]),
            (6, 0, 2, 4, true, false, 2, 2, 1, 0, [1, 0, 0, 0, 0, 0, 0]),
            (6, 0, 2, 4, true, true, 2, 2, 1, 0, [1, 0, 0, 0, 0, 0, 1]),
            (6, 0, 3, 0, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (6, 0, 3, 0, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (6, 0, 3, 0, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (6, 0, 3, 0, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (6, 0, 3, 1, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (6, 0, 3, 1, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (6, 0, 3, 1, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (6, 0, 3, 1, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (6, 0, 3, 2, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (6, 0, 3, 2, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (6, 0, 3, 2, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (6, 0, 3, 2, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (6, 0, 3, 3, false, false, 0, 2, 1, 1, [0, 1, 1, 1, 1, 1, 0]),
            (6, 0, 3, 3, false, true, 0, 2, 1, 1, [0, 1, 1, 1, 1, 1, 1]),
            (6, 0, 3, 3, true, false, 1, 0, 0, 0, [1, 0, 0, 1, 0, 0, 0]),
            (6, 0, 3, 3, true, true, 1, 0, 0, 0, [1, 0, 0, 1, 0, 0, 0]),
            (6, 0, 3, 4, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 1, 0]),
            (6, 0, 3, 4, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 1, 1]),
            (6, 0, 3, 4, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 0, 3, 4, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (18, 0, 0, 0, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 1, 0, 0]),
            (18, 0, 0, 0, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 1, 0, 0]),
            (18, 0, 0, 0, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (18, 0, 0, 0, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (18, 0, 0, 1, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 1, 0, 0]),
            (18, 0, 0, 1, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 1, 0, 0]),
            (18, 0, 0, 1, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (18, 0, 0, 1, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (18, 0, 0, 2, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 1, 0, 0]),
            (18, 0, 0, 2, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 1, 0, 0]),
            (18, 0, 0, 2, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (18, 0, 0, 2, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (18, 0, 0, 3, false, false, 0, 2, 1, 0, [0, 0, 0, 1, 1, 1, 0]),
            (18, 0, 0, 3, false, true, 0, 2, 1, 0, [0, 0, 0, 1, 1, 1, 0]),
            (18, 0, 0, 3, true, false, 1, 0, 1, 0, [1, 0, 0, 1, 0, 0, 0]),
            (18, 0, 0, 3, true, true, 1, 0, 1, 0, [1, 0, 0, 1, 0, 0, 0]),
            (18, 0, 0, 4, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 1, 1, 0]),
            (18, 0, 0, 4, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 1, 1, 0]),
            (18, 0, 0, 4, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (18, 0, 0, 4, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (18, 0, 1, 0, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (18, 0, 1, 0, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (18, 0, 1, 0, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (18, 0, 1, 0, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (18, 0, 1, 1, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (18, 0, 1, 1, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (18, 0, 1, 1, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (18, 0, 1, 1, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (18, 0, 1, 2, false, false, 1, 0, 1, 0, [1, 0, 0, 0, 1, 0, 0]),
            (18, 0, 1, 2, false, true, 1, 0, 1, 0, [1, 0, 0, 0, 1, 0, 0]),
            (18, 0, 1, 2, true, false, 1, 0, 1, 0, [1, 0, 0, 0, 0, 0, 0]),
            (18, 0, 1, 2, true, true, 1, 0, 1, 0, [1, 0, 0, 0, 0, 0, 0]),
            (18, 0, 1, 3, false, false, 1, 0, 1, 0, [1, 0, 0, 1, 1, 1, 0]),
            (18, 0, 1, 3, false, true, 1, 0, 1, 0, [1, 0, 0, 1, 1, 1, 0]),
            (18, 0, 1, 3, true, false, 1, 0, 1, 0, [1, 0, 0, 1, 0, 0, 0]),
            (18, 0, 1, 3, true, true, 1, 0, 1, 0, [1, 0, 0, 1, 0, 0, 0]),
            (18, 0, 1, 4, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 1, 0]),
            (18, 0, 1, 4, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 1, 1]),
            (18, 0, 1, 4, true, false, 2, 2, 1, 0, [1, 0, 0, 0, 0, 0, 0]),
            (18, 0, 1, 4, true, true, 2, 2, 1, 0, [1, 0, 0, 0, 0, 0, 1]),
            (18, 0, 2, 0, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (18, 0, 2, 0, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (18, 0, 2, 0, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (18, 0, 2, 0, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (18, 0, 2, 1, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (18, 0, 2, 1, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (18, 0, 2, 1, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (18, 0, 2, 1, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (18, 0, 2, 2, false, false, 2, 2, 1, 0, [1, 0, 0, 0, 1, 0, 0]),
            (18, 0, 2, 2, false, true, 2, 2, 1, 0, [1, 0, 0, 0, 1, 0, 1]),
            (18, 0, 2, 2, true, false, 2, 2, 1, 0, [1, 0, 0, 0, 0, 0, 0]),
            (18, 0, 2, 2, true, true, 2, 2, 1, 0, [1, 0, 0, 0, 0, 0, 1]),
            (18, 0, 2, 3, false, false, 2, 2, 1, 0, [1, 0, 0, 1, 1, 1, 0]),
            (18, 0, 2, 3, false, true, 2, 2, 1, 0, [1, 0, 0, 1, 1, 1, 1]),
            (18, 0, 2, 3, true, false, 1, 0, 1, 0, [1, 0, 0, 1, 0, 0, 0]),
            (18, 0, 2, 3, true, true, 1, 0, 1, 0, [1, 0, 0, 1, 0, 0, 0]),
            (18, 0, 2, 4, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 1, 0]),
            (18, 0, 2, 4, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 1, 1]),
            (18, 0, 2, 4, true, false, 2, 2, 1, 0, [1, 0, 0, 0, 0, 0, 0]),
            (18, 0, 2, 4, true, true, 2, 2, 1, 0, [1, 0, 0, 0, 0, 0, 1]),
            (18, 0, 3, 0, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (18, 0, 3, 0, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (18, 0, 3, 0, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (18, 0, 3, 0, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (18, 0, 3, 1, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (18, 0, 3, 1, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (18, 0, 3, 1, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (18, 0, 3, 1, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (18, 0, 3, 2, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 0]),
            (18, 0, 3, 2, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 0, 1]),
            (18, 0, 3, 2, true, false, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 0]),
            (18, 0, 3, 2, true, true, 0, 2, 1, 1, [0, 1, 1, 0, 0, 0, 1]),
            (18, 0, 3, 3, false, false, 0, 2, 1, 1, [0, 1, 1, 1, 1, 1, 0]),
            (18, 0, 3, 3, false, true, 0, 2, 1, 1, [0, 1, 1, 1, 1, 1, 1]),
            (18, 0, 3, 3, true, false, 1, 0, 1, 0, [1, 0, 0, 1, 0, 0, 0]),
            (18, 0, 3, 3, true, true, 1, 0, 1, 0, [1, 0, 0, 1, 0, 0, 0]),
            (18, 0, 3, 4, false, false, 0, 2, 1, 1, [0, 1, 1, 0, 1, 1, 0]),
            (18, 0, 3, 4, false, true, 0, 2, 1, 1, [0, 1, 1, 0, 1, 1, 1]),
            (18, 0, 3, 4, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (18, 0, 3, 4, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 0, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 0, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 0, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 0, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 1, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 1, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 1, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 1, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 2, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 2, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 2, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 2, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 3, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 3, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 3, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 3, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 4, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 4, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 4, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 0, 4, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 0, false, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 0, false, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 0, true, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 0, true, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 1, false, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 1, false, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 1, true, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 1, true, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 2, false, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 2, false, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 2, true, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 2, true, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 3, false, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 3, false, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 3, true, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 3, true, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 4, false, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 4, false, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 4, true, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 1, 4, true, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 0, false, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 0, false, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 0, true, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 0, true, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 1, false, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 1, false, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 1, true, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 1, true, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 2, false, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 2, false, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 2, true, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 2, true, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 3, false, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 3, false, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 3, true, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 3, true, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 4, false, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 4, false, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 4, true, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 2, 4, true, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 0, false, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 0, false, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 0, true, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 0, true, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 1, false, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 1, false, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 1, true, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 1, true, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 2, false, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 2, false, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 2, true, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 2, true, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 3, false, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 3, false, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 3, true, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 3, true, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 4, false, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 4, false, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 4, true, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (6, 1, 3, 4, true, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 0, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 0, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 0, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 0, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 1, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 1, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 1, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 1, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 2, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 2, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 2, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 2, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 3, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 3, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 3, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 3, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 4, false, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 4, false, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 4, true, false, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 0, 4, true, true, 0, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 0, false, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 0, false, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 0, true, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 0, true, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 1, false, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 1, false, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 1, true, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 1, true, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 2, false, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 2, false, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 2, true, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 2, true, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 3, false, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 3, false, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 3, true, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 3, true, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 4, false, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 4, false, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 4, true, false, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 1, 4, true, true, 1, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 0, false, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 0, false, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 0, true, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 0, true, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 1, false, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 1, false, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 1, true, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 1, true, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 2, false, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 2, false, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 2, true, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 2, true, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 3, false, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 3, false, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 3, true, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 3, true, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 4, false, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 4, false, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 4, true, false, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 2, 4, true, true, 2, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 0, false, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 0, false, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 0, true, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 0, true, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 1, false, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 1, false, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 1, true, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 1, true, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 2, false, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 2, false, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 2, true, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 2, true, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 3, false, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 3, false, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 3, true, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 3, true, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 4, false, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 4, false, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 4, true, false, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
            (0, 0, 3, 4, true, true, 3, 2, 1, 0, [0, 0, 0, 0, 0, 0, 0]),
        ];
        for (
            index,
            &(
                mode_tag,
                subtype,
                initial,
                phase,
                inside,
                active,
                final_state,
                nursery_len,
                kind,
                suppressed,
                counts,
            ),
        ) in cases.iter().enumerate()
        {
            let mut world = World::new();
            world.insert_resource(match mode_tag {
                6 => AppMode::Wall(if subtype == 0 {
                    WallShape::Freeform
                } else {
                    WallShape::Circle
                }),
                18 => AppMode::Fences,
                _ => AppMode::Editing(crate::systems::mode_manager::EditSubject::None),
            });
            world.insert_resource(CursorTerrainRaycast {
                tag: 1,
                pos: Vec3::new(1.0, 2.0, 3.0),
                ..Default::default()
            });
            world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(
                Vec2::ZERO,
                if inside { 10.0 } else { 1.0 },
            ))));
            let name = if mode_tag == 18 { "fences" } else { "wall" };
            let mut ui = UiState::new(UiInteraction {
                native_tag: 2,
                native_phase: phase,
                widget: WidgetId::clearing_canvas(name),
            });
            ui.set_intersection_tag(2);
            world.insert_resource(ui);
            world.insert_resource(WallDrawState { state: initial });
            world.insert_resource(CurveNursery {
                points: vec![Vec2::new(9.0, 1.0), Vec2::new(10.0, 2.0)],
                kind: 1,
            });
            let mut progress = InProgressCurve::default();
            if active {
                progress.wall = Some(WallId(7));
            }
            world.insert_resource(progress);
            world.init_resource::<SuppressContinueWall>();
            world.init_resource::<LiveDecoratorArchivist>();
            world.init_resource::<DecoratorStorage>();
            world.init_resource::<Events<RecordMouseInputIntoWall>>();
            world.init_resource::<Events<FinishWall>>();
            world.init_resource::<Events<crate::systems::wall::merge_walls::MergeWallsCmd>>();
            world.init_resource::<Events<crate::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>>();
            world.init_resource::<Events<DebugHistoryMsg>>();
            world.init_resource::<Events<WallBendAudioParamReset>>();
            world.init_resource::<Events<DisplayOutOfBorder>>();
            world.init_resource::<Events<FeedbackInputOutOfBorder>>();
            world.init_resource::<Events<HideTreesNextToWallCmd>>();
            let mut schedule = Schedule::default();
            schedule.add_systems(ui_draw_wall);
            schedule.run(&mut world);
            assert_eq!(
                world.resource::<WallDrawState>().state,
                final_state,
                "state case {index}"
            );
            assert_eq!(
                world.resource::<CurveNursery>().points.len(),
                nursery_len,
                "nursery case {index}"
            );
            assert_eq!(
                world.resource::<CurveNursery>().kind,
                kind,
                "kind case {index}"
            );
            assert_eq!(
                u8::from(world.resource::<SuppressContinueWall>().suppressed),
                suppressed,
                "suppress case {index}"
            );
            let actual = [
                world.resource::<Events<RecordMouseInputIntoWall>>().len(),
                world.resource::<Events<FinishWall>>().len(),
                world.resource::<Events<DebugHistoryMsg>>().len(),
                world.resource::<Events<WallBendAudioParamReset>>().len(),
                world.resource::<Events<DisplayOutOfBorder>>().len(),
                world.resource::<Events<FeedbackInputOutOfBorder>>().len(),
                world.resource::<Events<HideTreesNextToWallCmd>>().len(),
            ];
            assert_eq!(actual, counts, "event ports case {index}");
            let finish_events = world.resource::<Events<FinishWall>>();
            for event in finish_events.get_cursor().read(finish_events) {
                assert!(event.0);
            }
            let tree_events = world.resource::<Events<HideTreesNextToWallCmd>>();
            for event in tree_events.get_cursor().read(tree_events) {
                assert_eq!(event.wall, WallId(7));
                assert_eq!(event.finished, counts[1] != 0);
            }
            let canvas = world
                .resource::<UiState>()
                .clearing_canvas_params(WidgetId::clearing_canvas(name));
            if mode_tag == 18 || (mode_tag == 6 && subtype == 0) {
                let canvas = canvas.unwrap();
                assert_eq!((canvas.argument_79, canvas.layer), (1, 3));
                assert!(!canvas.no_history_recording);
            } else {
                assert!(canvas.is_none());
            }
        }
    }
    #[test]
    fn empty_terrain_before_is_archived_once_and_survives_later_bucket_creation() {
        use crate::resources::walls::{
            decorator::{DecoratorId, DecoratorInfo},
            decorator_storage::Decorators,
        };
        let mut world = World::new();
        world.insert_resource(AppMode::Wall(WallShape::Freeform));
        world.insert_resource(CursorTerrainRaycast::default());
        world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(
            Vec2::ZERO,
            10.0,
        ))));
        world.insert_resource(UiState::new(UiInteraction {
            native_tag: 2,
            native_phase: 3,
            widget: WidgetId::clearing_canvas("wall"),
        }));
        world.init_resource::<WallDrawState>();
        world.init_resource::<CurveNursery>();
        world.init_resource::<InProgressCurve>();
        world.init_resource::<SuppressContinueWall>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<Events<RecordMouseInputIntoWall>>();
        world.init_resource::<Events<FinishWall>>();
        world.init_resource::<Events<crate::systems::wall::merge_walls::MergeWallsCmd>>();
        world.init_resource::<Events<crate::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>>();
        world.init_resource::<Events<DebugHistoryMsg>>();
        world.init_resource::<Events<WallBendAudioParamReset>>();
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        world.init_resource::<Events<HideTreesNextToWallCmd>>();
        assert!(
            world
                .resource::<UiState>()
                .clearing_canvas_params(WidgetId::clearing_canvas("wall"))
                .is_none()
        );
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_draw_wall);
        schedule.run(&mut world);
        assert!(world.resource::<DecoratorStorage>().terrain.is_none());
        let archive = world.resource::<LiveDecoratorArchivist>();
        let before = archive
            .special1
            .as_ref()
            .expect("empty terrain before must be Some");
        assert!(before.items.is_empty() && before.removed.is_empty() && before.added.is_empty());
        assert!(archive.culling.allowed_ids.is_empty());
        let mut terrain = Decorators::default();
        terrain.insert(DecoratorId(17), DecoratorInfo::default());
        world.resource_mut::<DecoratorStorage>().terrain = Some(terrain);
        schedule.run(&mut world); // phase3 attempts archive again
        let archive = world.resource::<LiveDecoratorArchivist>();
        let before = archive.special1.as_ref().unwrap();
        assert!(before.items.is_empty() && before.removed.is_empty() && before.added.is_empty());
        // Only cull permission reads today's real bucket on the second call.
        assert!(archive.culling.allowed_ids.contains(&17));
    }
}
