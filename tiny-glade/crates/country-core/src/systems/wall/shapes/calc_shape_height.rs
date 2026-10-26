//! B `calc_shape_height`（140ac80e0/827B）：规则墙预览的高度与木框架缓存。
//! 原第六参数是TimberframeConstructionParamsCache；旧CalcShapeHeight只能兼容别名，
//! 不能另外保存状态。高度写ShapePlacement+40，抬升读取+3c，预览几何不拥有高度字段。
use ::utils::{
    curve::Curve3,
    geometry::{Circle2d, Rectangle2d},
};
use bevy_ecs::prelude::*;
use glam::Vec3;

use super::{PreviewCircle, PreviewRectangle, ShapePlacement};
use crate::{
    resources::{
        terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
        time::GameTime,
    },
    systems::mode_manager::{AppMode, WallShape},
};

/// 原16B缓存：tag0未就绪，tag1为正在平滑的木框架参数。
/// +8是绘制基准，+c是从基准到平滑顶部的高度，两个构造系统直接消费这两项。
#[repr(C)]
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct TimberframeConstructionParamsCache {
    pub tag: u32,
    pub current: f32,
    pub previous: f32,
    pub delta: f32,
}

/// 原maxNum展开：相等保留前值，前值NaN时取后值；不滤除非有限高度。
fn maximum(previous: f32, next: f32) -> f32 {
    if previous.is_nan() || next > previous {
        next
    } else {
        previous
    }
}

fn update_from_samples(
    points: &[Vec3],
    base: f32,
    placement: &mut ShapePlacement,
    cache: &mut TimberframeConstructionParamsCache,
    delta: f32,
) {
    let mut sum = 0.0f32;
    let mut highest = 0.0f32;
    for point in points {
        sum += point.y;
        highest = maximum(highest, point.y);
    }
    let top = if placement.surface() == 2 {
        // 原屋顶吸附以Option<float>固定基高为准，不重新采样形状中心。
        let fixed = if placement.height_type() as u8 == 0 {
            0.0
        } else {
            placement.fixed_height()
        };
        base + fixed
    } else {
        (sum / points.len() as f32 + base) + (-0.46)
    };
    let raw = top - (highest + (-0.46));
    let low = if raw < 0.7 { 0.7 } else { raw };
    let height = if low > 14.0 { 14.0 } else { low };
    placement.set_computed_height(height);
    let elevation = placement.elevation();
    cache.previous = if elevation == 0.0 { -2.5 } else { elevation };
    let target = (height + highest) + (-0.46);
    // 初帧直接到目标；后续复用同一缓存的顶部，不从抬升/旧预览字段重启。
    let previous = if cache.tag as u8 == 0 {
        target
    } else {
        cache.current
    };
    cache.current = ::utils::time_continuous_lerp(previous, target, delta, 10.0);
    cache.tag = 1;
    cache.delta = cache.current - cache.previous;
}

fn sampled_preview(
    mode: AppMode,
    rectangle: &PreviewRectangle,
    circle: &PreviewCircle,
    terrain: &TerrainHeightsData<TerrainHeightSsbo>,
    elevation: f32,
) -> Option<(Curve3, f32)> {
    let (curve, base) = match mode {
        AppMode::Wall(WallShape::Rectangle) => (
            Rectangle2d {
                axis: rectangle.orientation,
                center: rectangle.center,
                size: rectangle.dimensions,
            }
            .as_curve2(),
            6.0,
        ),
        AppMode::Wall(WallShape::Circle) => (
            Circle2d {
                center: circle.center,
                radius: circle.radius,
                rotation: circle.rotation,
            }
            .as_curve2(0.2),
            8.0,
        ),
        _ => return None,
    };
    let remapped: Curve3 =
        curve.crashy_resample_remap_per_segment(0.5, |_, _, point: glam::Vec2| {
            Vec3::new(
                point.x,
                maximum(terrain.sample_at_world_xz(point.x, point.y), elevation),
                point.y,
            )
        });
    Some((remapped, base))
}

/// 原七参数生产系统：真实AppMode→规则曲线→0.5逐段地形重采样→放置/木框架。
#[allow(clippy::too_many_arguments)]
pub fn calc_shape_height(
    mode: Res<AppMode>,
    preview_rectangle: Res<PreviewRectangle>,
    preview_circle: Res<PreviewCircle>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mut placement: ResMut<ShapePlacement>,
    mut cache: ResMut<TimberframeConstructionParamsCache>,
    time: Res<GameTime>,
) {
    let Some((remapped, base)) = sampled_preview(
        *mode,
        &preview_rectangle,
        &preview_circle,
        &terrain,
        placement.elevation(),
    ) else {
        cache.tag = 0;
        return;
    };
    update_from_samples(
        &remapped.points,
        base,
        &mut placement,
        &mut cache,
        time.delta,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;
    #[derive(serde::Deserialize)]
    struct NativeCase {
        mode: u32,
        shape: u8,
        center: [u32; 2],
        radius: u32,
        dimensions: [u32; 2],
        axis: [u32; 2],
        rotation: u32,
        placement: Vec<u8>,
        cache: [u32; 4],
        delta: u32,
        grid_seed: usize,
        centered: bool,
        terrain_range: [u32; 2],
        points: Vec<[u32; 3]>,
        parameters: Vec<u32>,
        length: u32,
        height: u32,
        output_cache: [u32; 4],
    }
    fn bits_equal(actual: f32, expected: u32, context: &str) {
        if f32::from_bits(expected).is_nan() {
            assert!(actual.is_nan(), "{context}");
        } else {
            assert_eq!(actual.to_bits(), expected, "{context}");
        }
    }
    /// 真执行原827B主函数、原两种曲线、逐段重采样及地形采样；CRT数学边界另记。
    #[test]
    fn actual_seven_resource_system_matches_original_complete_business_chain() {
        // 原全局地形开关是进程级依赖；两种路径在独立测试进程验证，避免污染其它测试。
        if std::env::var_os("TB_CALC_HEIGHT_NATIVE_CHILD").is_none() {
            let output=std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "systems::wall::shapes::calc_shape_height::tests::actual_seven_resource_system_matches_original_complete_business_chain", "--nocapture"])
                .env("TB_CALC_HEIGHT_NATIVE_CHILD","1").output().unwrap();
            assert!(
                output.status.success(),
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        let cases: Vec<NativeCase> = serde_json::from_str(include_str!(
            "../../../../../../tests/fixtures/native/toolbar-calc-shape-height.json"
        ))
        .unwrap();
        assert_eq!(cases.len(), 96);
        for (index, case) in cases.into_iter().enumerate() {
            ::utils::float_grid::CENTERED_PATH
                .store(case.centered, std::sync::atomic::Ordering::Relaxed);
            let mode = if case.mode == 6 {
                AppMode::Wall(if case.shape == 1 {
                    WallShape::Circle
                } else {
                    WallShape::Rectangle
                })
            } else {
                AppMode::default()
            };
            let rectangle = PreviewRectangle {
                center: Vec2::from_array(case.center.map(f32::from_bits)),
                dimensions: Vec2::from_array(case.dimensions.map(f32::from_bits)),
                orientation: Vec2::from_array(case.axis.map(f32::from_bits)),
                ..Default::default()
            };
            let circle = PreviewCircle {
                center: Vec2::from_array(case.center.map(f32::from_bits)),
                radius: f32::from_bits(case.radius),
                rotation: f32::from_bits(case.rotation),
                ..Default::default()
            };
            let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
            terrain.set_height_range(
                f32::from_bits(case.terrain_range[0]),
                f32::from_bits(case.terrain_range[1]),
            );
            for (i, value) in terrain.heights_mut().iter_mut().enumerate() {
                let x = i % 512;
                let z = i / 512;
                *value = ((x * 97 + z * 43 + case.grid_seed) & 65535) as u16;
            }
            let placement = ShapePlacement::from_observed(case.placement.try_into().unwrap());
            // 比较实际生产几何帮助函数的完整输出，不从平均值/最高点倒造预期。
            let sampled =
                sampled_preview(mode, &rectangle, &circle, &terrain, placement.elevation());
            if let Some((samples, _)) = sampled {
                bits_equal(
                    samples.length,
                    case.length,
                    &format!("case {index} XYZ curve length"),
                );
                assert_eq!(samples.points_u.len(), case.parameters.len());
                for (coordinate, (actual, expected)) in samples
                    .points_u
                    .iter()
                    .copied()
                    .zip(case.parameters)
                    .enumerate()
                {
                    bits_equal(
                        actual,
                        expected,
                        &format!("case {index} curve u[{coordinate}]"),
                    );
                }
                assert_eq!(samples.points.len(), case.points.len(), "case {index}");
                for (point, (actual, expected)) in
                    samples.points.iter().zip(case.points).enumerate()
                {
                    for (axis, (a, e)) in actual.to_array().into_iter().zip(expected).enumerate() {
                        bits_equal(a, e, &format!("case {index} point {point} axis {axis}"));
                    }
                }
            } else {
                assert!(case.points.is_empty());
            }
            let mut world = World::new();
            world.insert_resource(mode);
            world.insert_resource(rectangle);
            world.insert_resource(circle);
            world.insert_resource(terrain);
            world.insert_resource(placement);
            world.insert_resource(TimberframeConstructionParamsCache {
                tag: case.cache[0],
                current: f32::from_bits(case.cache[1]),
                previous: f32::from_bits(case.cache[2]),
                delta: f32::from_bits(case.cache[3]),
            });
            world.insert_resource(GameTime {
                delta: f32::from_bits(case.delta),
                ..Default::default()
            });
            let mut schedule = Schedule::default();
            schedule.add_systems(calc_shape_height);
            schedule.run(&mut world);
            bits_equal(
                world.resource::<ShapePlacement>().computed_height(),
                case.height,
                &format!("case {index} placement height"),
            );
            let cache = world.resource::<TimberframeConstructionParamsCache>();
            assert_eq!(cache.tag, case.output_cache[0], "case {index} tag");
            for (field, (actual, expected)) in [cache.current, cache.previous, cache.delta]
                .into_iter()
                .zip(case.output_cache[1..].iter().copied())
                .enumerate()
            {
                bits_equal(
                    actual,
                    expected,
                    &format!("case {index} cache field {field}"),
                );
            }
        }
    }

    #[test]
    fn persistent_cache_uses_real_time_and_mode_exit_preserves_payload() {
        let mut world = World::new();
        world.insert_resource(AppMode::Wall(WallShape::Rectangle));
        world.insert_resource(PreviewRectangle::default());
        world.insert_resource(PreviewCircle::default());
        world.insert_resource(ShapePlacement::default());
        let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        terrain.set_height_range(3., 3.);
        world.insert_resource(terrain);
        world.insert_resource(GameTime {
            delta: 0.0,
            ..Default::default()
        });
        world.init_resource::<TimberframeConstructionParamsCache>();
        let mut schedule = Schedule::default();
        schedule.add_systems(calc_shape_height);
        schedule.run(&mut world);
        let first = *world.resource::<TimberframeConstructionParamsCache>();
        assert_eq!(first.tag, 1);
        assert_eq!(first.previous, -2.5);
        assert_eq!(world.resource::<ShapePlacement>().computed_height(), 6.0);
        world
            .resource_mut::<TerrainHeightsData<TerrainHeightSsbo>>()
            .set_height_range(12., 12.);
        schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<TimberframeConstructionParamsCache>()
                .current,
            first.current,
            "delta=0不能以固定0.1推进"
        );
        world.resource_mut::<GameTime>().delta = 0.04;
        schedule.run(&mut world);
        let moved = *world.resource::<TimberframeConstructionParamsCache>();
        assert!(moved.current > first.current && moved.current < 17.54);
        world.insert_resource(AppMode::default());
        world.resource_mut::<PreviewRectangle>().dimensions = Vec2::ZERO;
        schedule.run(&mut world);
        let cleared = *world.resource::<TimberframeConstructionParamsCache>();
        assert_eq!(
            cleared,
            TimberframeConstructionParamsCache { tag: 0, ..moved },
            "退出只清tag且不构造无效曲线"
        );
        assert_eq!(
            std::any::TypeId::of::<TimberframeConstructionParamsCache>(),
            std::any::TypeId::of::<super::super::CalcShapeHeight>(),
            "别名不能产生另一资源"
        );
    }
}
