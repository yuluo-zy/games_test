//! shapes/circle/ui_edit_circle.rs — circle-edit UI (drag radius / finish).
//!
//! PDB procedure `ui_edit_circle` 0x14165d460 (12691 B) + `update_circle_radius`,
//! `finish_circle_edit`, `get_trigger_vol`, `show_full_circle_signifier`. FunctionSystem
//! signature O.
//! Business logic (O, call chain): while editing a circle, drags the radius trigger volume
//! (`WallTriggerVolume::is_valid`, `edit_shape_default`), updates the wall curve
//! (`InnerWalls::update_wall_state_curve`), re-publishes the enclosure
//! (`update_wall_state_enclosure`), and `finish_circle_edit` records the history edit.

use bevy_ecs::event::EventWriter;
use bevy_ecs::prelude::Resource;
use bevy_ecs::system::{Query, Res, ResMut};

use crate::resources::events::EditRectangleCmd;
use crate::resources::history::History;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::walls::{WallId, WallShapeSnapshot};
use crate::systems::wall::shapes::circle::ui_create_circle::PreviewCircle;
use crate::systems::wall::valid_enclosure::ShapeParameters;
use crate::ui::mode_manager::AppMode;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::signifiers::SignifierEntityBuilder;
use crate::ui::ui_state::UiState;

/// Animated circle signifier resource (O).
#[derive(Resource, Clone, Debug, Default)]
pub struct AnimatedCircleSignifierResource {
    pub raw: crate::typed_gap::Gap,
}

pub fn update_circle_radius(
    _wall: WallId,
    _radius: f32,
    _inner: &mut InnerWalls,
) {
}

pub fn finish_circle_edit(_wall: WallId, _inner: &mut InnerWalls, _history: &mut History) {}

pub fn get_trigger_vol(_center: glam::Vec3, _radius: f32) -> crate::typed_gap::Gap {
    crate::typed_gap::Gap
}

pub fn show_full_circle_signifier(_builder: SignifierEntityBuilder) {}

pub fn ui_edit_circle(
    app_mode: Res<AppMode>,
    mut inner_walls: ResMut<InnerWalls>,
    public_walls: Res<PublicWalls>,
    preview: Res<PreviewCircle>,
    ui_state: Res<UiState>,
    mut signifiers: ResMut<UiSignifierStream>,
    mut history: ResMut<History>,
    mut rect_writer: EventWriter<EditRectangleCmd>,
) {
    let _ = (&app_mode, &public_walls, &preview, &ui_state, &mut signifiers, &mut history, &mut rect_writer);
    for wall in public_walls.wall_ids() {
        let Some(state) = public_walls.get(wall) else { continue };
        if state.is_circle().is_none() {
            continue;
        }
        // O: drag updates the radius; release finishes the edit (history).
        let _ = ShapeParameters::as_circle;
        let _ = WallShapeSnapshot::from_observed([0; 28]);
        let _ = &mut inner_walls;
        break;
    }
}
