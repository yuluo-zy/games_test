//! B 0x140ae4620, checked against original RNE instructions.
use super::{
    record_garden_history::RecordGardenEditToHistoryCmd, GardenDrawEvent, GardenEraseEvent,
    GardenPinnedClutterCircle,
};
use crate::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    resources::{
        active_raster_stroke::ActiveRasterStrokesStorage,
        brush_sizes::BrushSizes,
        cursor_terrain_raycast::CursorTerrainRaycast,
        glade_border::{BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder},
        onboarding::NotifyHintSystemToolWasJustUsed,
        walls::{decorator_archivist::DecoratorArchivist, EraserSizeMultiplier},
        world_raster_defs::{GardenRaster, GardenRasterStagingSsbo},
    },
    systems::{
        history::history_process::DebugHistoryMsg,
        mode_manager::{AppMode, EraserLayer},
        paths::DrawState,
        ui::resources::ui_state::UiState,
        world_raster::{draw_raster, RasterDrawResources, RasterStrokeCmd},
    },
};
use bevy_ecs::{prelude::*, system::SystemParam};
use glam::Vec2;
use utils::geometry::Circle2d;

#[derive(SystemParam)]
pub struct GardenRasterResources<'w, C: Send + Sync + 'static> {
    strokes: ResMut<'w, ActiveRasterStrokesStorage>,
    archivist: ResMut<'w, DecoratorArchivist<C>>,
    library: ResMut<'w, AssetSsboLibrary>,
    tool_used: ResMut<'w, Events<NotifyHintSystemToolWasJustUsed>>,
    debug_history: ResMut<'w, Events<DebugHistoryMsg>>,
}

#[allow(clippy::too_many_arguments)]
pub fn draw_garden<C: Send + Sync + 'static>(
    mode: Res<AppMode>,
    mut resources: GardenRasterResources<C>,
    ui: Res<UiState>,
    mut raster: ResMut<GardenRaster>,
    mut draw_events: EventWriter<GardenDrawEvent>,
    mut erase_events: EventWriter<GardenEraseEvent>,
    mut record_events: EventWriter<RecordGardenEditToHistoryCmd>,
    cursor: Res<CursorTerrainRaycast>,
    border: (
        Res<GladeBorder>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
    ),
    mut state: Local<DrawState>,
    brush: (Res<EraserSizeMultiplier>, Res<BrushSizes>),
    mut pins: EventWriter<GardenPinnedClutterCircle>,
) {
    let (radius, draw, name, argument_79) = match *mode {
        AppMode::Garden => (brush.1.garden, true, "garden", 5),
        AppMode::Eraser(EraserLayer::All | EraserLayer::Garden) => {
            // B inverse_lerp 0x140c904a0: no clamp (the clamped entry is +0x10).
            let t = (brush.0 .0 - 0.3) / (2.0 - 0.3);
            (t * 4.0 + (1.0 - t), false, "eraser", 9)
        }
        _ => return,
    };
    assert!(radius.is_finite(), "garden brush radius must be finite");
    let widget = ui.clearing_canvas_w_layer_and_sound(name, argument_79, 3);
    let position = cursor.position();
    let center = Vec2::new(position.x, position.z);
    let (border, mut display_outside, mut feedback_outside) = border;
    let allowed = !draw
        || border.is_shape_inside(BorderShape::Circle(Circle2d::new(
            center,
            (0.4 * radius) * 0.8,
        )));
    let phase = widget.dragged();
    if !allowed {
        display_outside.write(DisplayOutOfBorder);
        if phase != 2 {
            feedback_outside.write(FeedbackInputOutOfBorder);
        }
    }

    // Preserve the native wrapping subtraction, blocked extension state, and
    // cancellation without commit when the widget's phase becomes 2.
    let r = phase.wrapping_sub(2).min(3);
    let command = if r.wrapping_sub(1) < 2 {
        if state.0.wrapping_sub(1) < 2 {
            state.0 = 2;
            if !allowed {
                return;
            }
            if draw {
                pins.write(GardenPinnedClutterCircle::brush_circle(
                    Circle2d::new(center, radius * 0.5), position.y + 1.5,
                ));
            }
            2
        } else {
            state.0 = allowed as u8;
            if !allowed {
                return;
            }
            draw as u32
        }
    } else {
        if r == 0 || state.0.wrapping_sub(1) >= 2 {
            state.0 = 0;
            return;
        }
        state.0 = 3;
        3
    };
    let mut borrowed = RasterDrawResources {
        strokes: &mut resources.strokes,
        archivist: &mut resources.archivist,
        cursor: &cursor,
        tool_used: &mut resources.tool_used,
        debug_history: &mut resources.debug_history,
    };
    draw_raster::<GardenRasterStagingSsbo, C>(
        RasterStrokeCmd {
            discriminant: command,
            strength: 0.5,
            radius,
            draw,
        },
        position,
        5,
        &mut borrowed,
        &mut raster,
        &mut resources.library,
        |position, radius, observed_flag| {
            draw_events.write(GardenDrawEvent {
                position,
                radius,
                observed_flag,
            });
        },
        |position, radius, observed_flag| {
            erase_events.write(GardenEraseEvent {
                position,
                radius,
                observed_flag,
            });
        },
        || {
            record_events.write(RecordGardenEditToHistoryCmd);
        },
    );
}
