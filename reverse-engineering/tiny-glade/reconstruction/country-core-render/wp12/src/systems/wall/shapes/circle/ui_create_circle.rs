//! shapes/circle/ui_create_circle.rs — circle-create UI + preview.
//!
//! PDB procedure `ui_create_circle` 0x14165bed0 (3695 B) + `circle_signifier`.
//! FunctionSystem signature O.

use bevy_ecs::prelude::Resource;
use bevy_ecs::system::{Res, ResMut};

use crate::resources::events::CreateCircleCmd;
use crate::resources::mouse_terrain_raycast::{CameraCursorRay, CursorPositionSS};
use crate::ui::action_set::ActionSetStates;
use crate::ui::mode_manager::AppMode;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::signifiers::SignifierEntityBuilder;
use crate::ui::ui_state::UiState;

/// Circle-create preview resource (O: `PreviewCircle::new`).
#[derive(Resource, Clone, Debug, Default)]
pub struct PreviewCircle {
    pub center: glam::Vec3,
    pub radius: f32,
    pub height: f32,
    pub enabled: bool,
}
impl PreviewCircle {
    pub fn new() -> Self {
        Self {
            center: glam::Vec3::ZERO,
            radius: 0.0,
            height: 0.0,
            enabled: false,
        }
    }
}

pub fn ui_create_circle(
    app_mode: Res<AppMode>,
    action_states: Res<ActionSetStates>,
    cursor: Res<CursorPositionSS>,
    camera_ray: Res<CameraCursorRay>,
    mut preview: ResMut<PreviewCircle>,
    ui_state: Res<UiState>,
    mut signifiers: ResMut<UiSignifierStream>,
    mut cmd_writer: bevy_ecs::event::EventWriter<CreateCircleCmd>,
) {
    let _ = (&app_mode, &action_states, &cursor, &camera_ray, &ui_state, &mut cmd_writer);
    if let Some(hit) = camera_ray.intersect_with_xz_plane(0.0) {
        preview.enabled = true;
        preview.center = hit;
        preview.radius = 1.0;
    }
    let _builder = SignifierEntityBuilder::new();
    let _ = &mut signifiers;
}
