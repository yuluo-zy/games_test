//! shapes/rectangle/ui_edit_rectangle_dims.rs — rectangle dimension edit UI.
//!
//! PDB procedures: `ui_edit_rectangle_dims` 0x140ad6250 (2977 B), `do_ui_edit_rectangle_dims`
//! 0x140ad6e00 (4605 B), `get_distance_to_edge`. FunctionSystem signatures O.
//! Business logic (O, call chain): drags an edge trigger volume to change the rectangle
//! dims; `do_ui_edit_rectangle_dims` applies the new dims (`UiRectangleDimsEdit` widget),
//! culls decorators (`decorator_cull_allow_any`), and emits `EditRectangleCmd`.

use bevy_ecs::event::EventWriter;
use bevy_ecs::prelude::Resource;
use bevy_ecs::system::{Res, ResMut};

use crate::resources::events::EditRectangleCmd;
use crate::resources::walls::decorator_archivist::DecoratorArchivist;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::wall::shapes::rectangle::WipRectangle;
use crate::ui::mode_manager::AppMode;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::ui_state::UiState;

/// Active dimension-widget resource (O).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct UiEditRectangleActiveWidgetResource {
    pub raw: crate::typed_gap::Gap,
}

pub fn get_distance_to_edge(
    _center: glam::Vec2,
    _dims: glam::Vec2,
    _point: glam::Vec2,
) -> f32 {
    0.0
}

pub fn ui_edit_rectangle_dims(
    app_mode: Res<AppMode>,
    mut signifiers: ResMut<UiSignifierStream>,
    ui_state: Res<UiState>,
    mut active_widget: ResMut<UiEditRectangleActiveWidgetResource>,
    mut wip: Option<ResMut<WipRectangle>>,
    public_walls: Res<PublicWalls>,
) {
    let _ = (&app_mode, &mut signifiers, &ui_state, &mut active_widget, &mut wip, &public_walls);
}

pub fn do_ui_edit_rectangle_dims(
    app_mode: Res<AppMode>,
    mut public_walls: ResMut<PublicWalls>,
    mut decorator_archivist: ResMut<DecoratorArchivist>,
    mut wip: Option<ResMut<WipRectangle>>,
    mut rect_writer: EventWriter<EditRectangleCmd>,
    ui_state: Res<UiState>,
) {
    let _ = (&app_mode, &mut public_walls, &mut decorator_archivist, &mut wip, &mut rect_writer, &ui_state);
}
