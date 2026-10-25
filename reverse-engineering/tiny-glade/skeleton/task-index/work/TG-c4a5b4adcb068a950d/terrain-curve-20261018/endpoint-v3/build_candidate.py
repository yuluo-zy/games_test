from pathlib import Path
b=Path(__file__).resolve().parent
s=(b.parent/'continuation-v2/ui_continue_wall.rs').read_text().replace('//! B continuation suppression, 0x140aadbb0 (375B). The endpoint UI producer\n//! is separate; this resource retains its actual release flag and reset inputs.','//! B endpoint continuation (0x140aadd30) and suppression (0x140aadbb0).\n//! CPU input uses the existing spatial widget registry, authority/publication\n//! and InProgressCurve owner. The signifier renderer closure is independent.')
s=s.replace('use bevy_ecs::prelude::*;','''use bevy_ecs::{prelude::*, system::SystemParam};
use glam::{Vec2, Vec3};
use utils::curve::Curve2 as InputCurve;

use crate::{
    resources::{
        color_id::{HalftimberColorIds, WallColorIds},
        cursor_terrain_raycast::CursorTerrainRaycast,
        glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder},
        in_progress_curve::InProgressCurve,
        walls::{
            InnerWalls, WallId, decorator_archivist::LiveDecoratorArchivist,
            decorator_storage::DecoratorStorage, in_progress_wall::WallDrawingMetadata,
            public_walls::PublicWalls,
        },
    },
    systems::{
        camera_rig::main_camera::MainCameraMoved,
        mode_manager::{AppMode, WallShape},
        ui::resources::{
            ui_state::UiState,
            widget_params::{WidgetParams, WidgetVolumeShape},
        },
        wall::{
            draw_wall::RecordMouseInputIntoWall,
            finish_wall::{ContinueWallOriginalState, FinishWall},
        },
    },
};''',1)
body='''
/// The original one-byte state: idle0, endpoint selected1, dragging2, ended3.
/// The selected wall/end are transient locals, not a second live-wall owner.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct WallContinueState {
    pub state: u8,
}

#[derive(SystemParam)]
pub struct ContinueWallUiEffects<'w> {
    record: EventWriter<'w, RecordMouseInputIntoWall>,
    finish: EventWriter<'w, FinishWall>,
    outside: EventWriter<'w, DisplayOutOfBorder>,
    feedback: EventWriter<'w, FeedbackInputOutOfBorder>,
}

#[derive(SystemParam)]
pub struct ContinueWallColors<'w> {
    wall: Res<'w, WallColorIds>,
    fence: Res<'w, HalftimberColorIds>,
}

/// Stable native format pieces and actual WallId Display specialization.
/// Both endpoint hit volumes use this same name for capture across frames.
pub fn continuation_widget_name(wall: WallId, extend_start: bool) -> String {
    format!("continue_wall_{extend_start}_{wall}")
}

pub fn ui_continue_wall(
    mode: Res<AppMode>,
    ui: Res<UiState>,
    walls: Res<InnerWalls>,
    public: Res<PublicWalls>,
    mut progress: ResMut<InProgressCurve>,
    mut continuation: ResMut<ContinueWallOriginalState>,
    mut state: ResMut<WallContinueState>,
    cursor: Res<CursorTerrainRaycast>,
    suppress: Res<SuppressContinueWall>,
    colors: ContinueWallColors,
    mut camera_moved: EventReader<MainCameraMoved>,
    border: Res<GladeBorder>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    storage: Res<DecoratorStorage>,
    mut effects: ContinueWallUiEffects,
) {
    let fence = match *mode {
        AppMode::Wall(WallShape::Freeform) => false,
        AppMode::Fences => true,
        _ => return,
    };
    if state.state == 3 {
        state.state = 0;
    }
    if state.state == 0 {
        // Suppression must not consume camera events. Active dragging bypasses
        // both gates, so a camera event remains for the next idle invocation.
        if suppress.suppressed {
            return;
        }
        if !camera_moved.is_empty() {
            camera_moved.clear();
            return;
        }
    }
    let mut selected = None;
    for (wall, published) in public.all() {
        // Original iterator subsequently performs the validated get as well.
        published.assert_valid();
        let authority = walls.get(wall);
        if published.is_being_bulldozed() || authority.is_none() {
            if state.state != 0 {
                effects.finish.write(FinishWall(false));
                if state.state.wrapping_sub(1) < 2 {
                    state.state = 3;
                }
            }
            continue;
        }
        let authority = authority.unwrap();
        if authority.source().shape.tag() != 2 {
            continue;
        }
        let kind = authority.source_flag_65();
        // B preserves raw non-0/1 values; only exactly 0/1 restrict the mode.
        if (kind == 1 && !fence) || (kind == 0 && fence) {
            continue;
        }
        let outline = &published.geometry.outline;
        let first = outline.first().expect("wall outline first point");
        let last = outline.last().expect("wall outline last point");
        for (point, extend_start) in [(first, true), (last, false)] {
            // MAXSS source zero wins for NaN and negative/signed-zero inputs.
            let point = Vec3::new(point[0], point[1].max(0.0), point[2]);
            let name = continuation_widget_name(wall, extend_start);
            let context = ui.add(
                &name,
                // Sphere's layer byte is native enum padding. Zero here only
                // fills the existing host compatibility field.
                WidgetParams::new(WidgetVolumeShape::sphere(point, 1.0, 0), [0; 2]),
            );
            match context.dragged().wrapping_sub(2).min(3) {
                0 => {},
                1 => {
                    let position = if cursor.tag & 1 != 0 {
                        Vec2::new(cursor.pos.x, cursor.pos.z)
                    } else {
                        Vec2::ZERO
                    };
                    if !border.is_pos_inside(position) {
                        effects.outside.write(DisplayOutOfBorder);
                        effects.feedback.write(FeedbackInputOutOfBorder);
                    }
                    // Unlike fresh drawing, outside still selects/captures the
                    // endpoint. No sampling event is emitted on this phase.
                    state.state = 1;
                    selected = Some((wall, extend_start));
                },
                2 => {
                    if state.state.wrapping_sub(1) < 2 {
                        if state.state == 1 {
                            if let Some(terrain) = storage.terrain.as_ref() {
                                archivist.archive_storage_decorators(1, terrain, None);
                                archivist.culling.allowed_ids
                                    .extend(terrain.decorators().keys().copied());
                            }
                        }
                        let position = if cursor.tag & 1 != 0 {
                            Vec2::new(cursor.pos.x, cursor.pos.z)
                        } else {
                            Vec2::ZERO
                        };
                        if border.is_pos_inside(position) {
                            effects.record.write(RecordMouseInputIntoWall);
                        } else {
                            effects.outside.write(DisplayOutOfBorder);
                            effects.feedback.write(FeedbackInputOutOfBorder);
                        }
                        state.state = 2;
                    } else {
                        state.state = 0;
                    }
                },
                _ => {
                    effects.finish.write(FinishWall(false));
                    if state.state.wrapping_sub(1) < 2 {
                        state.state = 3;
                    }
                },
            }
        }
    }
    if let Some((wall, extend_start)) = selected {
        let source = walls.get_unwrapped(wall);
        // Preserve all source and metadata, not just curve/height. Replacement
        // occurs before color indexing and before constructing the new input.
        continuation.previous = Some(source.clone());
        let kind = source.source_flag_65();
        let color_id = if kind == 0 {
            colors.wall.0[wall.0 as usize]
        } else {
            colors.fence.0[wall.0 as usize]
        };
        let source_curve = &source.source().curve;
        let mut curve = InputCurve::crashy_new_from_points(
            source_curve.points.iter().copied().map(Vec2::from_array).collect(), false,
        );
        // An unreversed continuation copies the actual stored parameters.
        curve.points_u.clone_from(&source_curve.points_u);
        curve.length = source_curve.length;
        if extend_start {
            // Same original leaf 0x14146c190: reverse then rebuild parameters.
            curve.reverse();
        }
        *progress = InProgressCurve::from_curve(curve);
        progress.wall = Some(wall);
        progress.current_wall = wall;
        progress.flags = u8::from(extend_start);
        progress.drawing = Some(WallDrawingMetadata {
            first_unsnapped_cursor: None,
            desired_height: source.height(),
            color_id,
            kind,
            freehand_ty: source.freehand_ty(),
        });
    }
}

'''
s=s.replace('#[cfg(test)]',body+'#[cfg(test)]',1)
(b/'ui_continue_wall.rs').write_text(s)
