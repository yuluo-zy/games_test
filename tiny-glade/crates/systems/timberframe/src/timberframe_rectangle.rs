//! B construct_timberframe_rectangle@141a41ba0, complete9275B RNE range.
//! Full affine instances replace the old position/section/quaternion proxy.
//! Native phases: verticals, spaced verticals, skirt segments, roof rafters,
//! then the selected ridge edge. See TG-ff033b6ba4d486ec81/WOODEN-PLANK-CONTRACT.md.
use bevy_ecs::prelude::*;
use glam::{Affine3A, Mat3, Quat, Vec2, Vec3};

use crate::TimberframeInstanceBuffer;
use crate::timberframe_beam_count_change_audio::TimberframeBeamCount;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::events::ConstructTimberframeCmd;
use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
use country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache;
use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};

pub const EDGE_DIST_DIVISOR: f32 = 1.51;
pub const BEAM_SECTION: f32 = 0.2;
/// Native immediate141a4273f, one ULP below the Rust literal0.18.
pub const BEAM_SECTION_MID: f32 = f32::from_bits(0x3e3851eb);
pub const RIDGE_MIN_PERIMETER: f32 = 0.5;

/// Native orientation_mat3@140c99570; retained public projection API.
pub fn orientation_mat3(axis: Vec2) -> Mat3 {
    Mat3::from_cols(
        Vec3::new(axis.x, 0.0, axis.y),
        Vec3::Y,
        Vec3::new(-axis.y, 0.0, axis.x),
    )
}

/// Scalar sum order in native timberframe's length computations.
pub(crate) fn segment_length(delta: Vec3) -> f32 {
    ((delta.y * delta.y + delta.x * delta.x) + delta.z * delta.z).sqrt()
}

/// Shared source projection of the inlined segment instances. The original
/// utils function supplies the orientation; local+Z receives the full length.
pub(crate) fn segment_plank(a: Vec3, b: Vec3, translation: Vec3) -> WoodenPlankSsbo {
    let delta = b - a;
    let inverse = 1.0 / segment_length(delta);
    let rotation = utils::geometry::rotation_from_normal(delta * inverse);
    let length = segment_length(a - b);
    WoodenPlankSsbo::timberframe(Affine3A::from_scale_rotation_translation(
        Vec3::new(BEAM_SECTION, BEAM_SECTION, length),
        rotation,
        translation,
    ))
}

pub fn construct_timberframe_rectangle(
    mut constructs: EventReader<ConstructTimberframeCmd>,
    mut buffer: ResMut<TimberframeInstanceBuffer>,
    mut library: ResMut<AssetSsboLibrary>,
    mut beams: ResMut<TimberframeBeamCount>,
    params: Res<TimberframeConstructionParamsCache>,
) {
    // Host adaptation: the current library's mutation API takes &mut self;
    // the native Res<AssetSsboLibrary> used its interior mutable storage.
    let ssbo = library
        .get_mut::<WoodenPlankSsbo>()
        .expect("called `Option::unwrap()` on a `None` value");
    if buffer.allocation.is_none() {
        buffer.allocation = Some(ssbo.new_allocation());
    }
    let mut writer = ssbo.writer(buffer.allocation.as_ref().unwrap());
    writer.clear();
    let mut exhausted = false;
    for cmd in constructs.read() {
        let shape = ShapeParameters::from_observed(cmd.shape);
        let Some(Shape::Rectangle(rect)) = shape.get_shape() else {
            continue;
        };
        let mut count = 0i64;
        let rotation = Quat::from_mat3(&orientation_mat3(rect.axis));
        let curve = rect.as_curve3(params.previous);
        // The native closed curve has five vertices, including its repeated
        // first vertex. It is deliberately not deduplicated here.
        for &position in &curve.points {
            let plank = WoodenPlankSsbo::timberframe(Affine3A::from_scale_rotation_translation(
                Vec3::new(BEAM_SECTION, params.delta, BEAM_SECTION),
                rotation,
                position,
            ));
            exhausted |= writer.push(&plank).is_err();
            count = count.wrapping_add(1);
        }
        for edge in curve.points.windows(2) {
            let n = (segment_length(edge[0] - edge[1]) / EDGE_DIST_DIVISOR).floor() as usize;
            if n < 2 {
                continue;
            }
            count = count.wrapping_add((n - 1) as i64);
            for i in 1..n {
                let t = i as f32 / n as f32;
                let position = edge[0] * (1.0 - t) + edge[1] * t;
                let plank =
                    WoodenPlankSsbo::timberframe(Affine3A::from_scale_rotation_translation(
                        Vec3::new(BEAM_SECTION_MID, params.delta, BEAM_SECTION_MID),
                        rotation,
                        position,
                    ));
                exhausted |= writer.push(&plank).is_err();
            }
        }
        let skirts = country_core::resources::walls::split_height_into_skirts(params.delta);
        for ring in skirts.into_iter().chain([0.0, params.delta]) {
            let height = ring + params.previous;
            let offset = Vec3::new(height * 0.0, height, height * 0.0);
            let perimeter = rect.as_curve3(0.0);
            for edge in perimeter.points.windows(2) {
                let translation = (edge[0] + edge[1]) * 0.5 + offset;
                exhausted |= writer
                    .push(&segment_plank(edge[0], edge[1], translation))
                    .is_err();
                count = count.wrapping_add(1);
            }
        }
        if cmd.roof_gate == 0 {
            use country_core::resources::roofs::{
                roof_ridge::RoofRidgeDims,
                roof_shape::{
                    ridge_length_01_from_rectangle_ratio, roof_height_ws_from_wall_shape,
                },
            };
            let roof_height = roof_height_ws_from_wall_shape(&cmd.shape);
            let ratio = ridge_length_01_from_rectangle_ratio(rect.size.x, rect.size.y);
            let direction = u8::from(rect.size.y > rect.size.x);
            let dims = RoofRidgeDims::calc(ratio, direction, rect.size.to_array()).get();
            let ridge = utils::geometry::Rectangle2d {
                size: Vec2::from_array(dims),
                ..rect
            };
            let outer = rect.as_curve2();
            let inner = ridge.as_curve2();
            let base = params.previous + params.delta;
            let peak = roof_height + base;
            for (a, b) in outer.points.iter().zip(&inner.points) {
                let a = Vec3::new(a.x, base, a.y);
                let b = Vec3::new(b.x, peak, b.y);
                exhausted |= writer.push(&segment_plank(a, b, (a + b) * 0.5)).is_err();
                count = count.wrapping_add(1);
            }
            if ridge.perimeter() > RIDGE_MIN_PERIMETER {
                // Original SETBE includes the unordered case.
                let index = usize::from(
                    ridge.size.y.partial_cmp(&ridge.size.x) != Some(core::cmp::Ordering::Greater),
                );
                let a = ridge.get_point(index);
                let b = ridge.get_point(index + 1);
                let a = Vec3::new(a.x, peak, a.y);
                let b = Vec3::new(b.x, peak, b.y);
                exhausted |= writer.push(&segment_plank(a, b, (a + b) * 0.5)).is_err();
                count = count.wrapping_add(1);
            }
        }
        beams.count = count;
    }
    if exhausted {
        eprintln!("Timberframe constructor ran out of memory");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ops::Range;
    use country_core::render::ssbo::rhapsody_boundary::{
        BackendError, Buffer, BufferDesc, GraphicsDevice,
    };
    use utils::geometry::{Circle2d, Rectangle2d};

    struct CpuDevice;
    impl GraphicsDevice for CpuDevice {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            _: &str,
            _: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            Ok(Buffer {
                byte_size: desc.size as u64,
                ..Default::default()
            })
        }
        fn update_buffer_bytes(
            &self,
            _: &Buffer,
            _: &[u8],
            _: &[Range<usize>],
            _: usize,
        ) -> Result<(), BackendError> {
            unreachable!("this fixture exercises CPU production, not GPU upload")
        }
    }
    fn run_with_setup(
        shape: Shape,
        gate: u8,
        setup: impl FnOnce(&mut World),
    ) -> (i64, Vec<WoodenPlankSsbo>) {
        let mut world = World::new();
        let library = AssetSsboLibrary::new();
        library.add_new::<WoodenPlankSsbo>(&CpuDevice);
        world.insert_resource(library);
        world.init_resource::<Events<ConstructTimberframeCmd>>();
        world.init_resource::<TimberframeInstanceBuffer>();
        world.init_resource::<TimberframeBeamCount>();
        setup(&mut world);
        world
            .resource_mut::<Events<ConstructTimberframeCmd>>()
            .send(ConstructTimberframeCmd::from_shape(
                ShapeParameters::from_shape(shape),
                gate,
            ));
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                construct_timberframe_rectangle,
                crate::timberframe_circle::construct_timberframe_circle,
            )
                .chain_ignore_deferred(),
        );
        schedule.run(&mut world);
        let library = world.resource::<AssetSsboLibrary>();
        let ssbo = library.get::<WoodenPlankSsbo>().unwrap();
        let allocation = world.resource::<TimberframeInstanceBuffer>();
        let ranges =
            ssbo.allocation_to_draw_multi_instanced(allocation.allocation.as_ref().unwrap());
        let records = ranges
            .into_iter()
            .flat_map(|(first, count)| {
                ssbo.data[first as usize..(first + count) as usize]
                    .iter()
                    .copied()
            })
            .collect();
        (world.resource::<TimberframeBeamCount>().count, records)
    }
    fn run(shape: Shape, gate: u8) -> (i64, Vec<WoodenPlankSsbo>) {
        run_with_setup(shape, gate, |world| {
            // 既有显式消费者夹具，不将其数值当作游戏启动默认值。
            world.insert_resource(TimberframeConstructionParamsCache {
                tag: 1,
                current: 5.0,
                previous: 1.0,
                delta: 4.0,
            });
        })
    }

    /// 真实高度生产者与两种木框架消费者共享同一World、同一Cache资源。
    #[test]
    fn actual_height_producer_drives_both_construction_consumers() {
        use bevy_ecs::system::RunSystemOnce;
        use country_core::{
            resources::{
                terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
                time::GameTime,
            },
            systems::{
                mode_manager::{AppMode, WallShape},
                wall::shapes::{
                    PreviewCircle, PreviewRectangle, ShapePlacement,
                    calc_shape_height::calc_shape_height,
                },
            },
        };
        for shape in [
            Shape::Rectangle(Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::new(4., 6.))),
            Shape::Circle(Circle2d::new(Vec2::new(2., -1.), 1.)),
        ] {
            let mut actual = None;
            let (count, records) = run_with_setup(shape, 1, |world| {
                let (mode, rectangle, circle) = match shape {
                    Shape::Rectangle(rect) => (
                        AppMode::Wall(WallShape::Rectangle),
                        PreviewRectangle {
                            center: rect.center,
                            orientation: rect.axis,
                            dimensions: rect.size,
                            ..Default::default()
                        },
                        PreviewCircle::default(),
                    ),
                    Shape::Circle(circle) => (
                        AppMode::Wall(WallShape::Circle),
                        PreviewRectangle::default(),
                        PreviewCircle {
                            center: circle.center,
                            radius: circle.radius,
                            rotation: circle.rotation,
                            ..Default::default()
                        },
                    ),
                };
                world.insert_resource(mode);
                world.insert_resource(rectangle);
                world.insert_resource(circle);
                world.insert_resource(ShapePlacement::default());
                world.insert_resource(GameTime {
                    delta: 1.0 / 60.0,
                    ..Default::default()
                });
                let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
                terrain.set_height_range(3., 3.);
                world.insert_resource(terrain);
                world.init_resource::<TimberframeConstructionParamsCache>();
                world.run_system_once(calc_shape_height).unwrap();
                actual = Some(*world.resource::<TimberframeConstructionParamsCache>());
            });
            let actual = actual.unwrap();
            assert_eq!(actual.tag, 1);
            assert_eq!(actual.previous, -2.5);
            assert!(actual.delta > 1.0);
            assert_eq!(count as usize, records.len());
            assert!(!records.is_empty());
            assert_eq!(
                records[0].transform.cols[1],
                Vec3::new(0., actual.delta, 0.)
            );
            assert_eq!(records[0].transform.cols[3].y, actual.previous);
        }
    }

    #[test]
    fn canonical_ecs_rectangle_uses_full_affines_and_independent_roof_gate() {
        assert_eq!(core::mem::size_of::<ConstructTimberframeCmd>(), 32);
        assert_eq!(core::mem::align_of::<ConstructTimberframeCmd>(), 4);
        assert_eq!(
            core::mem::offset_of!(ConstructTimberframeCmd, roof_gate),
            28
        );
        let shape = Shape::Rectangle(Rectangle2d::new_axis_aligned(
            Vec2::ZERO,
            Vec2::new(2.0, 8.0),
        ));
        let (count, records) = run(shape, 1);
        assert_eq!((count, records.len()), (21, 21));
        assert_eq!(records[0].transform.cols[1], Vec3::new(0.0, 4.0, 0.0));
        assert_eq!(records[0].transform.cols[3].y, 1.0);
        assert_eq!(
            records[5].transform.cols[0].length().to_bits(),
            BEAM_SECTION_MID.to_bits()
        );
        assert!(
            records
                .iter()
                .all(|p| (p.gate_seed, p.piece_seed, p.wall_id, p.flags) == (0, 0, 0, 2))
        );
        let (count, records) = run(shape, 0);
        assert_eq!((count, records.len()), (27, 27));
        // Real ridge edge runs along the long dimension, not the old outer
        // rectangle diagonal. Its short-axis coordinate is the native -0.05.
        let ridge = records.last().unwrap();
        assert!((ridge.transform.cols[2] - Vec3::new(0.0, 0.0, 8.0)).length() < 1e-5);
        assert!((ridge.transform.cols[3].x + 0.05).abs() < 1e-6);
    }
    #[test]
    fn canonical_ecs_circle_uses_tuple_windows_and_radial_roof_beams() {
        let circle = Circle2d::new(Vec2::new(4.0, -2.0), 1.0);
        let (count, records) = run(Shape::Circle(circle), 1);
        assert_eq!((count, records.len()), (16, 16));
        assert_eq!(records[0].transform.cols[1], Vec3::new(0.0, 4.0, 0.0));
        assert_eq!(records[0].transform.cols[3].y, 1.0);
        let (count, records) = run(Shape::Circle(circle), 0);
        assert_eq!((count, records.len()), (22, 22));
        let roof = &records[16];
        let point = circle.get_pos_at_u(0.0);
        let shape = ShapeParameters::from_shape(Shape::Circle(circle));
        let peak = country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape(
            shape.bytes(),
        ) + 5.0;
        let base = Vec3::new(point.x, 5.0, point.y);
        let tip = Vec3::new(circle.center.x, peak, circle.center.y);
        assert!((roof.transform.cols[2] - (tip - base)).length() < 1e-4);
        assert!((roof.transform.cols[3] - (tip + base) * 0.5).length() < 1e-5);
    }
}
