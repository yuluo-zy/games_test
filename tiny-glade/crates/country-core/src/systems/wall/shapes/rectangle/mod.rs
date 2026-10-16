//! B `systems/wall/shapes/rectangle/mod.rs`: rectangle shape submodule root.

pub mod create_rectangle;
pub mod edit_rectangle;
pub mod finish_edit_rectangle;
pub mod signifier_edit_dims_line;
pub mod ui_create_rectangle;
pub mod ui_edit_rectangle_dims;
pub mod ui_edit_rectangle_orientation;
pub mod ui_modes;

/// Inline B line 83: rectangle rotation axis from the orientation angle.
/// Consumed by ui_create_rectangle's `rectangle_dims_from_cursor_pos`.
pub fn get_rectangle_orientation(angle: f32) -> glam::Vec2 {
    let (sin, cos) = angle.sin_cos();
    glam::Vec2::new(cos, sin)
}
