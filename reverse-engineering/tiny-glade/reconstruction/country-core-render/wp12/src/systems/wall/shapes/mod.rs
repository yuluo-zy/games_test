//! shapes — wall shape-authoring systems (circle / rectangle / shape placement).

pub mod calc_shape_height;
pub mod circle;
pub mod rectangle;
pub mod shape_placement;
pub mod shape_placement_signifiers;
pub mod utils;

use crate::ui::App;

/// O: `shapes::impl$0::add_wall_shape_ux` (0x140945f50) — registers the shape-authoring
/// resources (`PreviewCircle`, `PreviewRectangle`, `ShapePlacement`, `CalcShapeHeight`).
pub fn add_wall_shape_ux(app: &mut App) {
    app.init_resource::<circle::ui_create_circle::PreviewCircle>();
    app.init_resource::<rectangle::ui_create_rectangle::PreviewRectangle>();
    app.init_resource::<shape_placement::ShapePlacement>();
    app.init_resource::<calc_shape_height::CalcShapeHeight>();
}
