//! B construct_timberframe_circle@1421b67a0, complete5102B RNE range.
//! Verticals use main-height scale at the cached base. Skirt tuple-windows
//! emit count-1 segments; roof beams run from the perimeter to the roof tip.
use crate::timberframe_beam_count_change_audio::TimberframeBeamCount;
use crate::timberframe_rectangle::segment_plank;
use crate::TimberframeInstanceBuffer;
use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::events::ConstructTimberframeCmd;
use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
use country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache;
use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
use glam::{Affine3A, Quat, Vec3};

pub const CIRCLE_COUNT_DIVISOR: f32 = 1.5;
pub const MIN_CIRCLE_SEGMENTS: usize = 6;
pub const BEAM_SECTION: f32 = 0.2;

pub fn construct_timberframe_circle(
    mut constructs: EventReader<ConstructTimberframeCmd>,
    buffer: Res<TimberframeInstanceBuffer>,
    mut library: ResMut<AssetSsboLibrary>,
    params: Res<TimberframeConstructionParamsCache>,
    mut beams: ResMut<TimberframeBeamCount>,
) {
    let ssbo = library
        .get_mut::<WoodenPlankSsbo>()
        .expect("called `Option::unwrap()` on a `None` value");
    // Rectangle runs first and creates/clears the one shared allocation.
    let allocation = buffer
        .allocation
        .as_ref()
        .expect("called `Option::unwrap()` on a `None` value");
    let mut writer = ssbo.writer(allocation);
    let mut exhausted = false;
    for cmd in constructs.read() {
        let shape = ShapeParameters::from_observed(cmd.shape);
        let Some(Shape::Circle(circle)) = shape.get_shape() else {
            continue;
        };
        let n = ((circle.perimeter() / CIRCLE_COUNT_DIVISOR) as usize).max(MIN_CIRCLE_SEGMENTS);
        let mut count = 0i64;
        for point in circle.as_points2_ptcount(n) {
            let plank = WoodenPlankSsbo::timberframe(Affine3A::from_scale_rotation_translation(
                Vec3::new(BEAM_SECTION, params.main_height, BEAM_SECTION),
                Quat::IDENTITY,
                Vec3::new(point.x, params.skirt_offset, point.y),
            ));
            exhausted |= writer.push(&plank).is_err();
            count = count.wrapping_add(1);
        }
        let skirts = country_core::resources::walls::split_height_into_skirts(params.main_height);
        for ring in skirts.into_iter().chain([0.0, params.main_height]) {
            let height = ring + params.skirt_offset;
            let offset = Vec3::new(height * 0.0, height, height * 0.0);
            // Original itertools::tuple_windows over the mapped point iterator,
            // not modulo wrapping the already closed sample sequence.
            let mut points = circle.as_points2_ptcount(n);
            let mut previous = points.next().map(|p| Vec3::new(p.x, 0.0, p.y));
            for p in points {
                let b = Vec3::new(p.x, 0.0, p.y);
                if let Some(a) = previous {
                    exhausted |= writer
                        .push(&segment_plank(a, b, (a + b) * 0.5 + offset))
                        .is_err();
                    count = count.wrapping_add(1);
                }
                previous = Some(b);
            }
        }
        if cmd.roof_gate == 0 {
            let base = params.skirt_offset + params.main_height;
            let peak = country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape(
                &cmd.shape,
            ) + base;
            let zero = base * 0.0;
            let tip = Vec3::new(circle.center.x, peak, circle.center.y);
            for p in circle.as_points2_ptcount(n) {
                let a = Vec3::new(p.x + zero, base, p.y + zero);
                exhausted |= writer
                    .push(&segment_plank(a, tip, (a + tip) * 0.5))
                    .is_err();
                count = count.wrapping_add(1);
            }
        }
        beams.count = count;
    }
    if exhausted {
        eprintln!("Timberframe constructor ran out of memory");
    }
}
