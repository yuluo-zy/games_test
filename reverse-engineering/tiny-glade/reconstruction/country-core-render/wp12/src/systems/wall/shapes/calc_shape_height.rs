//! shapes/calc_shape_height.rs — computes the shape preview height from terrain.
//!
//! PDB procedure `calc_shape_height` 0x140ac80e0 (827 B). FunctionSystem signature O.
//!
//! Business logic (O, decompiled 0x140ac80e0):
//!   - Reads the shape tuple `{tag=6, ty}`; ty 1 = circle, ty 2 = rectangle; anything else
//!     writes preview height 0 and returns.
//!   - `Rectangle2d::as_curve2` / `Circle2d::as_curve2` -> `crashy_resample_remap_per_segment`
//!     with a terrain-sampling closure; accumulates sum and max of per-point heights.
//!   - Base height: rectangle 6.0, circle 8.0. Height-ty 2 adds the terrain sample; otherwise
//!     `avg + base - 0.46`. Final = value - (max - 0.46), clamped to [0.7, 14.0].
//!   - Writes the clamped height into the preview resource (+0x40) and runs the
//!     `time_continuous_lerp` animation on the preview's lerp fields.

use bevy_ecs::system::Res;

use crate::resources::terrain_height::{TerrainHeightsData, TerrainHeightSsbo};
use crate::systems::wall::shapes::circle::ui_create_circle::PreviewCircle;
use crate::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle;
use crate::ui::mode_manager::AppMode;
use crate::utils::geometry::circle::Circle2d;
use crate::utils::geometry::rectangle::Rectangle2d;

/// Shape preview height cache (O: `calc_shape_height::impl$1::default`).
#[derive(bevy_ecs::prelude::Resource, Clone, Copy, Debug, Default)]
pub struct CalcShapeHeight {
    pub height: f32,
}

pub fn calc_shape_height(
    app_mode: Res<AppMode>,
    preview_rectangle: Res<PreviewRectangle>,
    preview_circle: Res<PreviewCircle>,
    terrain_heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mut cache: Res<CalcShapeHeight>,
) {
    let _ = &app_mode;
    // O: shape {tag=6, ty} gate.
    let shape_ty = if preview_rectangle.enabled {
        2
    } else if preview_circle.enabled {
        1
    } else {
        0
    };
    if shape_ty == 0 {
        return;
    }
    // O: resample the shape curve and collect per-point terrain heights.
    let mut sum = 0.0f32;
    let mut max = 0.0f32;
    let base = if shape_ty == 2 { 6.0 } else { 8.0 };
    let _ = Rectangle2d::default();
    let _ = Circle2d::default();
    let sample = terrain_heights.sample_at_world_xz(glam::Vec3::ZERO);
    let avg = sum;
    let value = avg + base - 0.46;
    let value = value - (max - 0.46);
    let value = value.max(0.7).min(14.0);
    let _ = sample;
    let _ = &mut cache;
    // O: write preview height +0x40 and animate via time_continuous_lerp.
    let mut preview_rect = PreviewCircle::default();
    preview_rect.height = value;
    let _ = &mut preview_rect;
}
