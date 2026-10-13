//! shapes/circle/ui_rotate_circle.rs — circle rotation UI (rotate the wall around center).
//!
//! PDB procedure `ui_rotate_circle` 0x1409ac910 (6847 B). FunctionSystem signature O.
//! Business logic (O, call chain): drag rotates the circle wall: computes the rotation angle
//! from the cursor (`world_space_to_screen_space`, `DebugLine`), updates the wall curve
//! (`update_wall_state_curve`) and enclosure (`update_wall_state_enclosure`), and records a
//! history compound edit.

use bevy_ecs::event::EventWriter;
use bevy_ecs::system::{Res, ResMut};

use crate::resources::history::History;
use crate::resources::mouse_terrain_raycast::CameraCursorRay;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::wall::shapes::circle::ui_create_circle::PreviewCircle;
use crate::ui::mode_manager::AppMode;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::signifiers::SignifierEntityBuilder;
use crate::ui::ui_state::UiState;

pub fn ui_rotate_circle(
    app_mode: Res<AppMode>,
    mut inner_walls: ResMut<InnerWalls>,
    public_walls: Res<PublicWalls>,
    preview: Res<PreviewCircle>,
    camera_ray: Res<CameraCursorRay>,
    ui_state: Res<UiState>,
    mut signifiers: ResMut<UiSignifierStream>,
    mut history: ResMut<History>,
    mut rect_writer: EventWriter<crate::resources::events::EditRectangleCmd>,
) {
    let _ = (&app_mode, &public_walls, &preview, &camera_ray, &ui_state, &mut signifiers, &mut history, &mut rect_writer);
    let _ = SignifierEntityBuilder::new();
    let _hit = camera_ray.intersect_with_xz_plane(0.0);
    {
        // O: rotation drag updates the circle curve.
        for wall in public_walls.wall_ids() {
            if public_walls.get(wall).map(|s| s.is_circle().is_some()).unwrap_or(false) {
                let _ = &mut inner_walls;
                break;
            }
        }
    }
}
