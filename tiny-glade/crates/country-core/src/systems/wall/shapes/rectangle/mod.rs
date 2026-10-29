//! B `systems/wall/shapes/rectangle/mod.rs`: rectangle shape submodule root.

pub mod create_rectangle;
pub mod edit_rectangle;
pub mod finish_edit_rectangle;
pub mod signifier_edit_dims_line;
pub mod ui_create_rectangle;
pub mod ui_edit_rectangle_dims;
pub mod ui_edit_rectangle_orientation;
pub mod ui_modes;
pub use finish_edit_rectangle::{OnRectangleEdited, WipRectangle};

pub mod edit_decision;
