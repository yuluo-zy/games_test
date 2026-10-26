//! 农舍老虎窗真实曲面凸体与量化缓存；完整原B14128a0b0/10280B。
//! 同一system Local持有真实Arc，命中不重建；未命中保留原有序采样/闭合点。

use country_core::resources::roofs::geometry::surface::RoofSection;
use ::utils::scalar::clamp01;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
use country_core::systems::collision::world::RaycastMeshData;
use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
use glam::{Affine3A, Vec2, Vec3};
use itertools::Itertools;
use std::{collections::HashMap, fmt, sync::Arc};
use utils::curve::{curve_u_circular::CurveUCircular, CurveU};
use utils::geometry::{Circle2d, Rectangle2d};

pub const DORMER_KEY_QUANT_SCALE: f32 = 5.0;

/// 原三个无符号量化值与rank；不是世界XZ或有符号屋顶索引。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DormerKey {
    pub curvature: u32,
    pub height_delta: u32,
    pub surface_delta: u32,
    pub rank: u8,
}

impl DormerKey {
    /// 原round后饱和转换：负值/NaN到0，正溢出到u32::MAX。
    pub fn quantize(value: f32) -> u32 {
        (value * DORMER_KEY_QUANT_SCALE).round() as u32
    }

    pub fn quantized_from(curvature: f32, height_delta: f32, surface_delta: f32, rank: u8) -> Self {
        // 原指令先量化表面距离，再高度差，最后曲率。
        let surface_delta = Self::quantize(surface_delta);
        let height_delta = Self::quantize(height_delta);
        let curvature = Self::quantize(curvature);
        Self { curvature, height_delta, surface_delta, rank }
    }
}

/// 对应原DormerCollisionGeoLib，复用现有名字；由外层system Local拥有。
#[derive(Clone, Default)]
pub struct DormerCollisionCache {
    map: HashMap<DormerKey, Arc<RaycastMeshData>>,
}

impl fmt::Debug for DormerCollisionCache {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("DormerCollisionCache").field("len", &self.map.len()).finish()
    }
}

impl DormerCollisionCache {
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

/// 真实Roof截面的字段桥；只投影已有字段，不另造几何或默认截面。
fn roof_section(roof: &Roof, position: Vec3) -> ShapeParameters {
    ShapeParameters::from_shape(match roof.get_roof_shape_from_pos_ws_clamped(position) {
        RoofSection::Circle { frame, radius, word_0x18 } => Shape::Circle(Circle2d {
            center: Vec2::from_array(frame), radius, rotation: f32::from_bits(word_0x18),
        }),
        RoofSection::Rectangle { frame, dimensions } => Shape::Rectangle(Rectangle2d {
            axis: Vec2::new(frame[0], frame[1]),
            center: Vec2::new(frame[2], frame[3]),
            size: Vec2::from_array(dimensions),
        }),
    })
}

/// 原全部真实输入→量化查询→有序曲面凸体→同一Local缓存。
/// rank1..3的width由外层原表提供；此处不以decorator库mesh/bbox代替动态几何。
#[allow(clippy::too_many_arguments)]
pub fn get_collision_mesh(
    transform: &CachedDecoratorTransform,
    width: f32,
    height: f32,
    rank: u8,
    padding: f32,
    roof: &Roof,
    cache: &mut DormerCollisionCache,
) -> Arc<RaycastMeshData> {
    let roof_height = roof.height();
    assert!((1..=3).contains(&rank), "internal error: entered unreachable code");
    let factor = [0.1, 0.15, 0.2][usize::from(rank - 1)];
    let position = transform.translation;
    let bottom = roof_section(roof, position);
    let perimeter = bottom.perimeter();
    let height_with_padding = height + padding;
    let front_base_y = height_with_padding + position.y;
    let top_y = roof_height * factor + front_base_y;
    let top = roof_section(roof, Vec3::new(position.x, top_y, position.z));
    let location = Vec2::new(position.x, position.z);
    let bottom_u = bottom.get_closest_u_from_pos(location);
    let top_u = top.get_closest_u_from_pos(location);
    let bottom_position = bottom.get_pos_at_u(bottom_u);
    let top_position = top.get_pos_at_u(top_u);
    let curvature = if bottom.tag() & 1 == 0 {
        let lower = f32::from_bits(0x3f66_6667);
        // 原utils::inverse_lerp140c904a0/16B；当前共享导出缺失，按独审批准内联三指令。
        let inverse_lerp = (bottom.as_circle().radius - lower) / (8.0 - lower);
        1.0 - clamp01(inverse_lerp)
    } else {
        0.0
    };
    let delta = bottom_position - top_position;
    let surface_delta = (delta.y * delta.y + delta.x * delta.x).sqrt();
    let key = DormerKey::quantized_from(curvature, top_y - front_base_y, surface_delta, rank);
    if let Some(mesh) = cache.map.get(&key) {
        return Arc::clone(mesh);
    }

    // inverse和点生成都在miss后，保原量化缓存对不同pose/scale的懒复用。
    let points = collision_points(transform, width, rank, padding, roof, perimeter,
        height_with_padding, front_base_y, top_y);
    let mesh = Arc::new(RaycastMeshData::new_convex(&points).expect("convex hull"));
    cache.map.insert(key, Arc::clone(&mesh));
    mesh
}

/// 仅生成原凸体输入；顺序与重复点影响几何hash，不能排序、去重或省略面。
#[allow(clippy::too_many_arguments)]
fn collision_points(
    transform: &CachedDecoratorTransform,
    width: f32,
    rank: u8,
    padding: f32,
    roof: &Roof,
    perimeter: f32,
    height_with_padding: f32,
    front_base_y: f32,
    top_y: f32,
) -> Vec<Vec3> {
    let inverse = Affine3A::from_scale_rotation_translation(
        transform.scale, transform.rotation, transform.translation,
    ).inverse();
    let samples = if rank == 1 { 2usize } else { 3usize };
    let mut points = Vec::with_capacity(6 * samples - 6);
    let extent = (width + padding) / perimeter;
    let position = transform.translation;
    let location = Vec2::new(position.x, position.z);
    // 原(0..2)相邻高度对；保0乘非有限值的传播，不直接化简两个端点。
    for (low_t, high_t) in (0..2).map(|index| index as f32).tuple_windows() {
        let low_weight = 1.0 - low_t;
        let high_weight = 1.0 - high_t;
        let low_roof_y = top_y * low_t + position.y * low_weight;
        let high_roof_y = top_y * high_t + position.y * high_weight;
        let low = roof_section(roof, Vec3::new(position.x, low_roof_y, position.z))
            .expand(low_t * 0.0 + low_weight * padding);
        let high = roof_section(roof, Vec3::new(position.x, high_roof_y, position.z))
            .expand(high_weight * padding + 0.0);
        let low_center = low.get_closest_u_from_pos(location);
        let high_center = high.get_closest_u_from_pos(location);
        let low_y = top_y * low_t + front_base_y * low_weight;
        let high_y = top_y * high_t + front_base_y * high_weight;
        let low_drop_y = low_y - low_roof_y;
        let high_drop_y = high_y - high_roof_y;
        let low_drop = Vec3::new(low_drop_y * 0.0, low_drop_y, low_drop_y * 0.0);
        let high_drop = Vec3::new(high_drop_y * 0.0, high_drop_y, high_drop_y * 0.0);
        let total_drop = Vec3::new(height_with_padding * 0.0, height_with_padding,
            height_with_padding * 0.0);
        let low_start = CurveUCircular::new(low_center + extent * -0.5);
        let low_end = CurveUCircular::new(low_center + extent * 0.5);
        let high_start = CurveUCircular::new(high_center + extent * -0.5);
        let high_end = CurveUCircular::new(high_center + extent * 0.5);
        let angles = (0..samples).map(|index| {
            let fraction = index as f32 / (samples - 1) as f32;
            (low_start.wrapping_lerp(low_end, fraction),
                high_start.wrapping_lerp(high_end, fraction))
        });
        for (index, ((low_a, high_a), (low_b, high_b))) in angles.tuple_windows().enumerate() {
            let a = local_point(&low, low_a, low_y, &inverse);
            let b = local_point(&low, low_b, low_y, &inverse);
            let c = local_point(&high, high_a, high_y, &inverse);
            let d = local_point(&high, high_b, high_y, &inverse);
            points.extend_from_slice(&[a, b, c, b, d, c]);
            if index == 0 || (rank != 1 && index == 1) {
                points.extend_from_slice(&[a, a - low_drop, c, a - low_drop, c - high_drop, c]);
            }
            if index == samples - 2 {
                points.extend_from_slice(&[b - low_drop, b, d - high_drop, b, d, d - high_drop]);
            }
            if low_t == 0.0 {
                points.extend_from_slice(&[a, b, a - total_drop, b, b - total_drop, a - total_drop]);
            }
        }
    }
    points
}

fn local_point(shape: &ShapeParameters, coordinate: CurveUCircular, height: f32, inverse: &Affine3A) -> Vec3 {
    let coordinate: CurveU = coordinate.into();
    let position = shape.get_pos_at_u(coordinate.get());
    inverse.transform_point3(Vec3::new(position.x, height, position.y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Quat;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    fn roof(rectangle: bool) -> Roof {
        let mut bytes = [0u8; 88];
        bytes[8..12].copy_from_slice(&u32::from(rectangle).to_le_bytes());
        for (offset, value) in [(0x0c, if rectangle { 1.0f32 } else { 0.0 }), (0x10, 0.0),
            (0x14, if rectangle { 0.0 } else { 3.0 }), (0x18, 0.0),
            (0x1c, 8.0), (0x20, 6.0), (0x2c, 0.5), (0x30, 0.5), (0x34, 0.5)] {
            bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        Roof::from_observed(bytes)
    }

    fn transform() -> CachedDecoratorTransform {
        CachedDecoratorTransform {
            rotation: Quat::IDENTITY,
            translation: Vec3::new(-3.0, 0.25, 0.0),
            scale: Vec3::ONE,
        }
    }

    #[test]
    fn native_key_rounds_and_saturates_unsigned_factors() {
        assert_eq!(DormerKey::quantize(0.1), 1);
        assert_eq!(DormerKey::quantize(0.09), 0);
        assert_eq!(DormerKey::quantize(-0.1), 0);
        assert_eq!(DormerKey::quantize(f32::NAN), 0);
        assert_eq!(DormerKey::quantize(f32::INFINITY), u32::MAX);
        assert_eq!(DormerKey::quantized_from(0.2, 1.0, 2.0, 3),
            DormerKey { curvature: 1, height_delta: 5, surface_delta: 10, rank: 3 });
    }

    #[test]
    fn real_circle_and_rectangle_produce_a_convex_and_reuse_the_same_arc() {
        for rectangle in [false, true] {
            for (rank, width) in [(1, 0.8), (2, 1.5), (3, 2.2)] {
                let mut cache = DormerCollisionCache::default();
                let roof = roof(rectangle);
                let transform = transform();
                let mesh = get_collision_mesh(&transform, width, 1.0, rank, 0.2, &roof, &mut cache);
                assert!(mesh.shape_ref().unwrap().as_convex_polyhedron().unwrap().points().len() >= 4);
                let same = get_collision_mesh(&transform, width, 1.0, rank, 0.2, &roof, &mut cache);
                assert!(Arc::ptr_eq(&mesh, &same));
                assert_eq!(cache.len(), 1);
                // 原key不含scale；命中不能提前进入奇异inverse/构造凸体。
                let singular = CachedDecoratorTransform { scale: Vec3::ZERO, ..transform };
                let same = get_collision_mesh(&singular, width, 1.0, rank, 0.2, &roof, &mut cache);
                assert!(Arc::ptr_eq(&mesh, &same));
            }
        }
    }

    #[test]
    fn invalid_rank_and_failed_hull_do_not_publish_a_cache_entry() {
        let roof = roof(false);
        let transform = transform();
        let mut cache = DormerCollisionCache::default();
        for rank in [0, 4, 255] {
            assert!(catch_unwind(AssertUnwindSafe(|| {
                get_collision_mesh(&transform, 0.8, 1.0, rank, 0.2, &roof, &mut cache);
            })).is_err());
            assert!(cache.is_empty());
        }
        assert!(catch_unwind(AssertUnwindSafe(|| {
            get_collision_mesh(&transform, 0.0, 0.0, 1, 0.0, &roof, &mut cache);
        })).is_err());
        assert!(cache.is_empty());
    }

    #[test]
    fn ordered_hull_inputs_keep_all_native_closing_faces_and_scale_inverse() {
        let roof = roof(false);
        let transform = transform();
        let perimeter = roof_section(&roof, transform.translation).perimeter();
        let hp = 1.0 + 0.2;
        let front_y = hp + transform.translation.y;
        for (rank, width, count) in [(1, 0.8, 24), (2, 1.5, 42), (3, 2.2, 42)] {
            let top_y = roof.height() * [0.1, 0.15, 0.2][usize::from(rank - 1)] + front_y;
            let points = collision_points(&transform, width, rank, 0.2, &roof, perimeter, hp, front_y, top_y);
            assert_eq!(points.len(), count);
            assert!(points.iter().all(|point| point.is_finite()));
            // 主面之后是起始面；底面保持精确竖向偏移和重复顶点。
            assert_eq!(points[6], points[0]);
            assert_eq!(points[8], points[2]);
            let bottom = if rank == 1 { 18 } else { 12 };
            assert_eq!(points[bottom], points[0]);
            assert_eq!(points[bottom + 1], points[1]);
            assert_eq!(points[bottom + 2], points[0] - Vec3::Y * hp);
            let scaled = CachedDecoratorTransform { scale: Vec3::new(2.0, 1.0, 2.0), ..transform };
            let local = collision_points(&scaled, width, rank, 0.2, &roof, perimeter, hp, front_y, top_y);
            assert_eq!(points.len(), local.len());
            for (original, scaled) in points.iter().zip(local) {
                assert!((original.x * 0.5 - scaled.x).abs() < 1.0e-6);
                assert!((original.y - scaled.y).abs() < 1.0e-6);
                assert!((original.z * 0.5 - scaled.z).abs() < 1.0e-6);
            }
        }
    }
}
