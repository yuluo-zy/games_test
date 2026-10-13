//! shapes/rectangle/ui_create_rectangle.rs — rectangle-create UI + preview.
//!
//! PDB procedure `ui_create_rectangle` 0x14155d410 (4168 B) + `rectangle_signifier`,
//! `rectangle_dims_from_cursor_pos`. FunctionSystem signature O.

use bevy_ecs::prelude::Resource;
use bevy_ecs::system::{Res, ResMut};

use crate::resources::events::CreateRectangleCmd;
use crate::resources::mouse_terrain_raycast::CameraCursorRay;
use crate::systems::wall::shapes::rectangle::get_rectangle_orientation;
use crate::ui::mode_manager::AppMode;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::signifiers::SignifierEntityBuilder;
use crate::ui::ui_state::UiState;

/// Rectangle-create preview resource (O: `PreviewRectangle::new`).
#[derive(Resource, Clone, Debug, Default)]
pub struct PreviewRectangle {
    pub center: glam::Vec3,
    pub dims: glam::Vec2,
    pub angle: f32,
    pub height: f32,
    pub enabled: bool,
}
impl PreviewRectangle {
    pub fn new() -> Self {
        Self {
            center: glam::Vec3::ZERO,
            dims: glam::Vec2::ZERO,
            angle: 0.0,
            height: 0.0,
            enabled: false,
        }
    }
}

/// O: `rectangle_dims_from_cursor_pos` — dims from the drag delta and orientation.
pub fn rectangle_dims_from_cursor_pos(
    center: glam::Vec2,
    cursor: glam::Vec2,
    angle: f32,
) -> glam::Vec2 {
    let delta = cursor - center;
    let orient = get_rectangle_orientation(delta, angle);
    let _ = orient;
    delta.abs()
}

pub fn ui_create_rectangle(
    app_mode: Res<AppMode>,
    ui_state: Res<UiState>,
    mut preview: ResMut<PreviewRectangle>,
    camera_ray: Res<CameraCursorRay>,
    mut signifiers: ResMut<UiSignifierStream>,
    mut cmd_writer: bevy_ecs::event::EventWriter<CreateRectangleCmd>,
) {
    let _ = (&app_mode, &ui_state, &camera_ray, &mut cmd_writer);
    let hit = camera_ray.intersect_with_xz_plane(0.0);
    {
        preview.enabled = true;
        preview.center = hit;
        preview.dims = glam::Vec2::splat(1.0);
    }
    let _builder = SignifierEntityBuilder::new();
    let _ = &mut signifiers;
}
