//! 原B141290b30：两条世界剖面→脊板及左右瓦面，直接写唯一64B瓦实例。
//! 随机流由装饰世界位置初始化；分割、扰动和实例种子的消费次序不可交换。

use country_core::resources::{
    roof_tile_ssbo::RoofTileSsbo,
    roofs::{
        geometry::surface::RoofSection,
        roof_shape::{Roof, RoofIntersectionSection},
    },
    walls::cached_decorator_transforms::CachedDecoratorTransform,
};
use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
use glam::{Mat3, Quat, Vec2, Vec3};
use utils::{
    curve::Curve3,
    geometry::{Circle2d, Rectangle2d},
};

#[inline]
pub fn to_f16_bits(value: f32) -> u16 {
    half::f16::from_f32(value).to_bits()
}
#[inline]
pub fn from_f16_bits(bits: u16) -> f32 {
    half::f16::from_bits(bits).to_f32()
}

/// 原写面共有的字段；不是通用主屋顶record（其circle/main标志不同）。
pub(super) fn tile_record(
    position: Vec3,
    dimensions: Vec3,
    rotation: Quat,
    roof_id: u32,
    rng: &fastrand::Rng,
) -> RoofTileSsbo {
    let d = dimensions.to_array().map(to_f16_bits);
    let q = rotation.to_array().map(to_f16_bits);
    RoofTileSsbo {
        base_position: position,
        current_height: 0.0,
        clipping_plane: [0x3c00_0000, 0x7bff_0000],
        scale_t: [u32::from(d[0]) | (u32::from(d[1]) << 16), u32::from(d[2])],
        rotation_quat: [
            u32::from(q[0]) | (u32::from(q[1]) << 16),
            u32::from(q[2]) | (u32::from(q[3]) << 16),
        ],
        roof_id,
        velocity_rotation: 0,
        animation_t: 1.0,
        seed: rng.i32(-100..100),
        is_circle: 0,
        is_main_roof_body: 0,
    }
}

pub(super) fn section_parameters(section: RoofSection) -> ShapeParameters {
    ShapeParameters::from_shape(match section {
        RoofSection::Circle {
            frame,
            radius,
            word_0x18,
        } => Shape::Circle(Circle2d {
            center: Vec2::from_array(frame),
            radius,
            rotation: f32::from_bits(word_0x18),
        }),
        RoofSection::Rectangle { frame, dimensions } => Shape::Rectangle(Rectangle2d {
            axis: Vec2::new(frame[0], frame[1]),
            center: Vec2::new(frame[2], frame[3]),
            size: Vec2::from_array(dimensions),
        }),
    })
}

fn section_at_y(roof: &Roof, y: f32) -> Option<ShapeParameters> {
    match roof.roof_shape_at_intersection_y(y) {
        RoofIntersectionSection::OutOfBounds => None,
        RoofIntersectionSection::Section(section) => Some(section_parameters(section)),
        RoofIntersectionSection::RawSection(raw) => {
            let f = |i| f32::from_le_bytes(raw[i..i + 4].try_into().unwrap());
            Some(ShapeParameters::from_shape(if raw[0] & 1 == 0 {
                Shape::Circle(Circle2d {
                    center: Vec2::new(f(4), f(8)),
                    radius: f(12),
                    rotation: f(16),
                })
            } else {
                Shape::Rectangle(Rectangle2d {
                    axis: Vec2::new(f(4), f(8)),
                    center: Vec2::new(f(12), f(16)),
                    size: Vec2::new(f(20), f(24)),
                })
            }))
        }
    }
}

/// 原次序为y²+x²再加z²；保留x86标量求和以免行列数跨阈值。
pub(super) fn length(v: Vec3) -> f32 {
    ((v.y * v.y + v.x * v.x) + v.z * v.z).sqrt()
}

pub(super) fn position_rng(position: Vec3) -> fastrand::Rng {
    let p = position * 10.0;
    // seed原是z²+(y²+x²)，Rust饱和转换也保留负数/NaN/溢出边界。
    fastrand::Rng::with_seed(length(p) as u64)
}

/// 不收集另一套瓦数组；调用者持有实际SparseInstanceBufferWriter。
/// profile缓存按原语义借用；越界截面原本直接返回，不创建默认截面。
pub fn assemble_gothic_dormer_tiles(
    profiles: &[Curve3; 2],
    roof: &Roof,
    transform: &CachedDecoratorTransform,
    mut emit: impl FnMut(RoofTileSsbo),
) {
    let first = *profiles[0]
        .points
        .first()
        .expect("called Option::unwrap() on a None value");
    let Some(section) = section_at_y(roof, first.y) else {
        return;
    };
    let point = Vec2::new(first.x, first.z);
    let closest = if section.tag() == 0 {
        section.as_circle().closest_point_on_perimeter(point)
    } else {
        section.as_rectangle().closest_point_on_perimeter(point)
    };
    let anchor = Vec3::new(closest.x, first.y, closest.y);
    let rng = position_rng(transform.translation);
    let depth = length(first - anchor);
    let ridge_splits = utils::random_splits(
        ((depth / 0.46875).round().max(2.0)) as usize,
        0.09375 / depth,
        &rng,
    );
    let start = first + (first - anchor) * 0.05;
    let ridge_delta = start - anchor;
    let ridge_direction = ridge_delta * (1.0 / length(ridge_delta));
    let right = ridge_direction.cross(Vec3::Y).normalize();
    let ridge_rotation = Quat::from_mat3(&Mat3::from_cols(right, ridge_direction, Vec3::Y));
    let roof_id = roof.wall_id().0 as u32;
    for pair in ridge_splits.windows(2) {
        let t = (pair[0] + pair[1]) * 0.5;
        emit(tile_record(
            anchor * t + start * (1.0 - t),
            Vec3::new(0.1, (pair[1] - pair[0]) * depth * 1.2, 0.2),
            ridge_rotation,
            roof_id,
            &rng,
        ));
    }
    let depth_splits =
        utils::random_splits(((depth / 0.3).ceil() as usize).max(2), 0.03 / depth, &rng);
    let forward = transform.rotation * Vec3::Z;
    for (side, profile) in profiles.iter().enumerate() {
        let profile_splits = utils::random_splits(
            ((profile.length / f32::from_bits(0x3ea3_d70b)).ceil() as usize).max(2),
            0.032 / profile.length,
            &rng,
        );
        let normal = transform.rotation * Vec3::new(0.0, 0.0, if side == 0 { 1.0 } else { -1.0 });
        for depth_pair in depth_splits.windows(2) {
            let middle = (depth_pair[0] + depth_pair[1]) * 0.5;
            let depth_center = (1.0 - middle) * 0.0 - middle * depth;
            let tile_width = (depth_pair[1] - depth_pair[0]) * depth;
            for pair in profile_splits.windows(2) {
                let a = profile.get_pos_at_u(pair[0]);
                let b = profile.get_pos_at_u(pair[1]);
                let tangent = (b - a) * (1.0 / length(b - a));
                let cross = tangent.cross(normal);
                let basis = Quat::from_mat3(&Mat3::from_cols(normal, cross, tangent));
                let pitch = rng.f32();
                let extra = rng.f32();
                let angle = (((1.0 - pitch) + (1.0 - pitch)) + pitch * 5.0)
                    * f32::from_bits(0x3c8e_fa35)
                    * 0.5;
                let (sin, cos) = angle.sin_cos();
                let rotation = (basis * Quat::from_xyzw(sin, sin * 0.0, sin * 0.0, cos))
                    * Quat::from_xyzw(0.70710677, 0.0, 0.0, 0.70710677);
                let extension = extra * 0.2 + (1.0 - extra) * 0.1;
                let thickness = f32::from_bits(0x3da3_d70b);
                let position = cross * -thickness * 0.5
                    + (tangent * extension * 0.5 + (b * 0.5 + a * 0.5))
                    + forward * depth_center;
                emit(tile_record(
                    position,
                    Vec3::new(tile_width, length(a - b) + extension, thickness),
                    rotation,
                    roof_id,
                    &rng,
                ));
            }
        }
    }
}

#[cfg(test)]
mod complete_gothic_tile_native_tests {
    use super::*;
    type Case = (
        [u8; 88],
        [u32; 4],
        [u32; 3],
        [[[u32; 3]; 4]; 2],
        [[u32; 4]; 2],
        [u32; 2],
        &'static [[u32; 16]],
    );
    fn words(t: RoofTileSsbo) -> [u32; 16] {
        [
            t.base_position.x.to_bits(),
            t.base_position.y.to_bits(),
            t.base_position.z.to_bits(),
            t.current_height.to_bits(),
            t.clipping_plane[0],
            t.clipping_plane[1],
            t.scale_t[0],
            t.scale_t[1],
            t.rotation_quat[0],
            t.rotation_quat[1],
            t.roof_id,
            t.velocity_rotation,
            t.animation_t.to_bits(),
            t.seed as u32,
            t.is_circle as u32,
            t.is_main_roof_body as u32,
        ]
    }
    #[test]
    fn gothic_original_tiles_compare_complete_native_sides_ridge_and_rng_all_words() {
        let cases: &[Case] = include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-gothic-tile-native-fixtures.rs"
        ));
        for (index, (roof, rotation, position, points, us, lengths, expected)) in
            cases.iter().enumerate()
        {
            let transform = CachedDecoratorTransform {
                rotation: Quat::from_array(rotation.map(f32::from_bits)),
                translation: Vec3::from_array(position.map(f32::from_bits)),
                scale: Vec3::ONE,
            };
            let profiles = std::array::from_fn(|i| {
                Curve3::from_cached_parts(
                    points[i]
                        .map(|p| Vec3::from_array(p.map(f32::from_bits)))
                        .to_vec(),
                    us[i].map(f32::from_bits).to_vec(),
                    f32::from_bits(lengths[i]),
                )
            });
            let mut actual = Vec::new();
            assemble_gothic_dormer_tiles(&profiles, &Roof::from_observed(*roof), &transform, |r| {
                actual.push(words(r))
            });
            assert_eq!(actual.as_slice(), *expected, "original case {index}");
        }
    }
    #[test]
    fn native_first_profile_is_required_but_out_of_domain_roof_emits_no_tiles() {
        let empty = || Curve3::from_cached_parts(Vec::new(), Vec::new(), 0.);
        let roof = Roof::from_observed([0; 88]);
        let transform = CachedDecoratorTransform::default();
        assert!(
            std::panic::catch_unwind(|| assemble_gothic_dormer_tiles(
                &[empty(), empty()],
                &roof,
                &transform,
                |_| panic!("must not emit")
            ))
            .is_err()
        );
        let profile = Curve3::try_new_from_points(vec![Vec3::Y, Vec3::splat(2.)], false).unwrap();
        assemble_gothic_dormer_tiles(&[profile, empty()], &roof, &transform, |_| {
            panic!("out of domain must not emit")
        });
    }
}
