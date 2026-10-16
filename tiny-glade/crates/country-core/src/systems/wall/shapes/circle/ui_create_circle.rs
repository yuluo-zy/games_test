//! B `systems/wall/shapes/circle/ui_create_circle.rs`: circle drawing preview.
//!
//! Evidence: `chapters/systems/wall/shapes/circle/ui_create_circle.rs.md` (O,
//! 0x14165bed0). The drag radius from the locked anchor is clamped to
//! [0.9, 8.0], written to PreviewCircle and (on submit) emitted as
//! CreateCircleCmd. The original reads the `wall_circle_canvas` input action
//! state (FUN_1409236d0) and a camera ray; this reconstruction consumes the
//! world-space [`CursorTerrainRaycast`] directly and keeps a local input proxy.

use bevy_ecs::prelude::*;
use glam::Vec2;

use super::super::{CreateCircleCmd, PreviewCircle, SelectedWallShape};
use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};

/// Minimum circle radius (0x3f666667 @0x142a7c880).
const MIN_RADIUS: f32 = 0.9;
/// Maximum circle radius (0x40e00000 @0x142934604).
const MAX_RADIUS: f32 = 8.0;

/// Local proxy for the `wall_circle_canvas` input action state (B
/// FUN_1409236d0; exact input resource 待取证).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct CircleCanvasInput {
    /// Anchor locked on first press; circle center.
    pub anchor: Option<Vec2>,
    /// Release submits the CreateCircleCmd.
    pub submitting: bool,
}

/// B 0x14165cd40: `cursor_circle_` preview signifier record.
pub fn circle_signifier(center: Vec2, radius: f32) -> UiAppendSignifier {
    let mut extra = [0u8; 12];
    extra[..4].copy_from_slice(&radius.to_le_bytes());
    UiAppendSignifier {
        tag: 1,
        position: glam::Vec3::new(center.x, 0.0, center.y),
        extra,
    }
}

/// B 0x14165bed0 (12-param system).
pub fn ui_create_circle(
    selected: Res<SelectedWallShape>,
    cursor: Res<CursorTerrainRaycast>,
    mut preview: ResMut<PreviewCircle>,
    mut input: ResMut<CircleCanvasInput>,
    mut commands: EventWriter<CreateCircleCmd>,
    mut stream: ResMut<UiSignifierStream>,
) {
    if !selected.is_circle() {
        *preview = PreviewCircle::default();
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
    let distance = (cursor_pos - anchor).length();
    let radius = if distance < MIN_RADIUS {
        MIN_RADIUS
    } else if distance > MAX_RADIUS {
        MAX_RADIUS
    } else {
        distance
    };
    preview.center = anchor;
    preview.radius = radius;

    if input.submitting {
        commands.write(CreateCircleCmd {
            center: anchor,
            radius,
        });
        input.anchor = None;
        input.submitting = false;
    }
    stream.write(circle_signifier(anchor, radius));
}
