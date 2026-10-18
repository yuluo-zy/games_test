//! B `systems/wall/shapes/calc_shape_height.rs`: shape preview height.
//!
//! Evidence: `chapters/systems/wall/shapes/calc_shape_height.rs.md` (O,
//! 0x140ac80e0). Base heights are 6.0 (rectangle) / 8.0 (circle); follow-
//! terrain height type adds the terrain sample, otherwise the sampled average
//! is used with the -0.46 constant; the final value is clamped to [0.7, 14.0]
//! and written to the preview +0x40 field; CalcShapeHeight records the
//! time-continuous interpolation.

use bevy_ecs::prelude::*;
use ::utils::curve::Curve3XZ;
use ::utils::geometry::{Circle2d, Rectangle2d};

use super::{
    CalcShapeHeight, PreviewCircle, PreviewRectangle, SelectedWallShape,
};
use crate::resources::terrain_height::{TerrainHeightsData, TerrainHeightSsbo};

/// Rectangle base height (0x40c00000 @0x142934690).
const RECTANGLE_BASE_HEIGHT: f32 = 6.0;
/// Circle base height (0x41000000 @0x142934604).
const CIRCLE_BASE_HEIGHT: f32 = 8.0;
/// Height offset constant (0xbeeb851f @0x142a7f250).
const HEIGHT_OFFSET: f32 = -0.46;
/// Clamp range (0x3f333333 @0x1429cd030 / 0x41600000 @0x142a7de00).
const MIN_HEIGHT: f32 = 0.7;
const MAX_HEIGHT: f32 = 14.0;
/// Circle sample step (0x3e4ccccd @0x1429cd068).
const CIRCLE_SAMPLE_STEP: f32 = 0.2;
/// Interpolation rate (0x41200000 @0x1429345a8).
const INTERPOLATION_RATE: f32 = 10.0;
/// Previous-height default when the old preview height is zero (0xc0200000
/// @0x1429df1c0).
const DEFAULT_PREVIOUS: f32 = -2.5;

/// B 0x140ac80e0 (7-param system).
pub fn calc_shape_height(
    selected: Res<SelectedWallShape>,
    mut preview_circle: ResMut<PreviewCircle>,
    mut preview_rectangle: ResMut<PreviewRectangle>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mut state: ResMut<CalcShapeHeight>,
) {
    let is_rect = selected.is_rectangle();
    if !(selected.is_circle() || is_rect) {
        state.tag = 0;
        return;
    }

    // Build the shape Curve2 and sample terrain heights along it.
    let curve = if is_rect {
        Rectangle2d {
            axis: preview_rectangle.orientation,
            center: preview_rectangle.center,
            size: preview_rectangle.dimensions,
        }
        .as_curve2()
    } else {
        Circle2d {
            center: preview_circle.center,
            radius: preview_circle.radius,
            rotation: preview_circle.rotation,
        }
        .as_curve2(CIRCLE_SAMPLE_STEP)
    };
    let remapped: Curve3XZ = curve.crashy_resample_remap_per_segment(
        1.0,
        |_segment, _t, point: glam::Vec2| {
            let h = terrain.sample_at_world_xz(point.x, point.y);
            glam::Vec3::new(point.x, h, point.y)
        },
    );
    let mut sum = 0.0f32;
    let mut max = f32::NEG_INFINITY;
    let mut count = 0usize;
    for point in &remapped.points {
        sum += point.y;
        count += 1;
        if point.y > max {
            max = point.y;
        }
    }

    let base = if is_rect {
        RECTANGLE_BASE_HEIGHT
    } else {
        CIRCLE_BASE_HEIGHT
    };
    let height_ty = if is_rect {
        preview_rectangle.height_ty
    } else {
        preview_circle.height_ty
    };
    let mut height = if height_ty == 2 {
        // Follow terrain: base + terrain sample (None treated as 0).
        let center_h = if is_rect {
            terrain.sample_at_world_xz(preview_rectangle.center.x, preview_rectangle.center.y)
        } else {
            terrain.sample_at_world_xz(preview_circle.center.x, preview_circle.center.y)
        };
        base + center_h
    } else {
        let average = if count == 0 { 0.0 } else { sum / count as f32 };
        average + base + HEIGHT_OFFSET
    };
    height -= max + HEIGHT_OFFSET;
    let clamped = if height < MIN_HEIGHT {
        MIN_HEIGHT
    } else if height > MAX_HEIGHT {
        MAX_HEIGHT
    } else {
        height
    };

    // Write preview +0x40.
    if is_rect {
        preview_rectangle.height = clamped;
    } else {
        preview_circle.height = clamped;
    }

    // Time-continuous interpolation record.
    let target = height + max - HEIGHT_OFFSET;
    let previous = if is_rect {
        if preview_rectangle.old_height == 0.0 {
            DEFAULT_PREVIOUS
        } else {
            preview_rectangle.old_height
        }
    } else if preview_circle.old_height == 0.0 {
        DEFAULT_PREVIOUS
    } else {
        preview_circle.old_height
    };
    let current = lerp_rate(previous, target, INTERPOLATION_RATE);
    state.tag = 1;
    state.current = current;
    state.previous = previous;
    state.delta = current - previous;
}

/// B func_0x000140c904e0: exponential time-continuous lerp toward `target` at
/// the given rate.
fn lerp_rate(previous: f32, target: f32, rate: f32) -> f32 {
    let t = (rate * 0.1).clamp(0.0, 1.0);
    previous + (target - previous) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_height(selected: SelectedWallShape, rect: PreviewRectangle) -> (CalcShapeHeight, PreviewRectangle) {
        let mut world = World::new();
        world.insert_resource(selected);
        world.insert_resource(PreviewCircle::default());
        world.insert_resource(rect);
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        world.insert_resource(CalcShapeHeight::default());
        let mut schedule = Schedule::default();
        schedule.add_systems(calc_shape_height);
        schedule.run(&mut world);
        (
            *world.resource::<CalcShapeHeight>(),
            *world.resource::<PreviewRectangle>(),
        )
    }

    #[test]
    fn non_wall_shape_disables_the_preview() {
        let (state, _) = run_height(SelectedWallShape { tag: 3, ty: 0 }, PreviewRectangle::default());
        assert_eq!(state.tag, 0);
    }

    #[test]
    fn rectangle_height_uses_six_and_clamps() {
        let (state, rect) = run_height(
            SelectedWallShape {
                tag: SelectedWallShape::WALL_SHAPES_TAG,
                ty: SelectedWallShape::TY_RECTANGLE,
            },
            PreviewRectangle {
                center: glam::Vec2::ZERO,
                dimensions: glam::Vec2::new(4.0, 4.0),
                orientation: glam::Vec2::X,
                height_ty: 0,
                height: 0.0,
                old_height: 0.0,
            },
        );
        assert!((0.7..=14.0).contains(&rect.height));
        assert_eq!(state.tag, 1);
    }
}

// ---------------------------------------------------------------- 组 A 接线

/// 木框架构造参数缓存（O：`calc_shape_height::TimberframeConstructionParamsCache`；
/// `timberframe` crate 两构造系统（0x1421b67a0/0x141a41ba0）以
/// `Res<TimberframeConstructionParamsCache>` 读取墙形状→屋顶参数。
/// 布局（O，restorer-wave3-timber-roof fresh 0x141a41ba0/0x1421b67a0）：`+0x00` 墙形状/
/// 屋顶参数族（语义 U，typed gap）、`+0x08` 裙层偏移、`+0x0c` 主高——两 f32 被
/// rect/circle 构造系统直接读取（`fVar33=*(cache+0xc)` 主环高、`fVar33+*(cache+8)`
/// 裙层高），本处按机器码面暴露字段名（原 `{_gap: u64}` 8B 占位扩展为 16B 对齐原布局）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct TimberframeConstructionParamsCache {
    /// @+0x00：墙形状/屋顶参数族（语义 U，typed gap）。
    pub _gap: u64,
    /// @+0x08：裙层高度偏移（O：rect/circle 裙层 `h + *(cache+8)`）。
    pub skirt_offset: f32,
    /// @+0x0c：主高（O：rect 垂直梁/circle 主环高度）。
    pub main_height: f32,
}
