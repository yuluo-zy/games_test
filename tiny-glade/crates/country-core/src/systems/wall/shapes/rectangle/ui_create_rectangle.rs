//! B `systems/wall/shapes/rectangle/ui_create_rectangle.rs`: rectangle drawing
//! preview.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/ui_create_rectangle.rs.md`
//! (O, 0x14155d410). The drag rectangle dims/orientation are written to
//! PreviewRectangle and (on submit) emitted as CreateRectangleCmd. The exact
//! `rectangle_dims_from_cursor_pos` (0x140c99520) projection is 待取证
//! (structural approximation below); the `wall_rectangle_canvas` input action
//! is proxied by [`RectangleCanvasInput`].

use bevy_ecs::prelude::*;
use glam::Vec2;

use super::get_rectangle_orientation;
use super::super::{CreateRectangleCmd, PreviewRectangle, SelectedWallShape};
use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};

/// Local proxy for the `wall_rectangle_canvas` input action (待取证).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct RectangleCanvasInput {
    pub anchor: Option<Vec2>,
    pub submitting: bool,
}

/// B 0x14155e460: `cursor_rectangle_` preview signifier (0x28 B record).
pub fn rectangle_signifier(center: Vec2, dimensions: Vec2) -> UiAppendSignifier {
    let mut extra = [0u8; 12];
    let floats = [dimensions.x, dimensions.y, 0.0f32].map(f32::to_le_bytes);
    for (i, bytes) in floats.iter().enumerate() {
        extra[i * 4..i * 4 + 4].copy_from_slice(bytes);
    }
    UiAppendSignifier {
        tag: 1,
        position: glam::Vec3::new(center.x, 0.0, center.y),
        extra,
    }
}

/// B 0x14155d410 (12+ param system).
pub fn ui_create_rectangle(
    selected: Res<SelectedWallShape>,
    cursor: Res<CursorTerrainRaycast>,
    mut preview: ResMut<PreviewRectangle>,
    mut input: ResMut<RectangleCanvasInput>,
    mut commands: EventWriter<CreateRectangleCmd>,
    mut stream: ResMut<UiSignifierStream>,
) {
    if !selected.is_rectangle() {
        *preview = PreviewRectangle::default();
        return;
    }
    if cursor.tag == 0 {
        return;
    }
    let cursor_pos = Vec2::new(cursor.pos.x, cursor.pos.z);
    let anchor = match input.anchor {
        Some(anchor) => anchor,
        None => {
            input.anchor = Some(cursor_pos);
            cursor_pos
        }
    };
    // Drag rectangle: center at the drag midpoint, dims = double the projected
    // delta along the orientation axes (0x140c99520 待取证).
    let orientation = get_rectangle_orientation(0.0);
    let perpendicular = Vec2::new(-orientation.y, orientation.x);
    let delta = cursor_pos - anchor;
    let width = (delta.dot(orientation)).abs() * 2.0;
    let height = (delta.dot(perpendicular)).abs() * 2.0;
    let center = anchor + delta * 0.5;
    let dimensions = Vec2::new(width, height);
    preview.center = center;
    preview.dimensions = dimensions;
    preview.orientation = orientation;

    if input.submitting {
        commands.write(CreateRectangleCmd {
            center,
            dimensions,
            orientation,
        });
        input.anchor = None;
        input.submitting = false;
    }
    stream.write(rectangle_signifier(center, dimensions));
}
