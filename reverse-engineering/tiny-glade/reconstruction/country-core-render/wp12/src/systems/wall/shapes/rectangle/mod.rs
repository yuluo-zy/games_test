//! shapes/rectangle — rectangle wall-shape systems.

pub mod create_rectangle;
pub mod edit_rectangle;
pub mod finish_edit_rectangle;
pub mod signifier_edit_dims_line;
pub mod ui_create_rectangle;
pub mod ui_edit_rectangle_dims;
pub mod ui_edit_rectangle_orientation;
pub mod ui_modes;

use crate::typed_gap::Gap;
use crate::utils::geometry::rectangle::Rectangle2d;

/// Wip rectangle resource (O: `Res<Option<WipRectangle>>` in finish_edit_rectangle).
#[derive(bevy_ecs::prelude::Resource, Clone, Debug, Default)]
pub struct WipRectangle {
    pub wall: Option<crate::resources::walls::WallId>,
    pub dims: glam::Vec2,
    pub angle: f32,
    pub raw: Gap,
}

/// O: `get_rectangle_orientation` — orientation from dims/angle.
pub fn get_rectangle_orientation(dims: glam::Vec2, angle: f32) -> glam::Vec2 {
    let _ = dims;
    glam::Vec2::from_angle(angle).truncate()
}
