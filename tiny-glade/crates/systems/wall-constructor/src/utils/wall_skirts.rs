//! 墙裙边（utils/wall_skirts.rs）。
//!
//! `add_skirts`（0x1412221f0，construct_walls 实调；G05 caller 链）。

/// 原 0x141222524..688：同一裙层的所有区间共用一条随机流。
/// 调用者传入原矩形边界，其他形状传入 `[0, 1]`。
fn skirt_split_boundaries(height: f32, maximum_y: f32, length: f32, ranges: &[[f32; 2]]) -> Vec<f32> {
    let seed = ((height + (maximum_y + length)) * 1000.0) as u64;
    let rng = fastrand::Rng::with_seed(seed);
    let mut boundaries = vec![0.0];
    let mut splits = Vec::new();
    for &[start, end] in ranges {
        let span = (end - start) * length;
        let count = ((span / 0.69).ceil() as usize).wrapping_add(1);
        splits.clear();
        utils::random_splits_into(count, 0.32 / span, &rng, &mut splits);
        boundaries.reserve(splits.len().saturating_sub(1));
        for &t in splits.iter().skip(1) {
            assert!((0.0..=1.0).contains(&t), "裙边分割坐标超出范围");
            boundaries.push(start + (end - start) * t);
        }
    }
    boundaries
}


/// 原 0x141222c7c..0x141222f40 的单块裙砖洞口扣除。
/// 洞口数据使用已最终化存储的原始坐标，不归一化。
fn clip_skirt_brick(
    proposal: utils::geometry::Aabb2,
    holes: &[country_core::resources::walls::wall_holes::FinalizedWallHole],
    closed: bool,
    length: f32,
) -> Result<std::collections::VecDeque<country_core::systems::wall::construction::bricks::WallBrick>,
    country_core::systems::wall::construction::bricks::WallConstructionError> {
    use country_core::systems::wall::construction::bricks::{resolve_hole_overlap, WallBrick};
    use utils::geometry::Aabb2;
    let mut pieces = std::collections::VecDeque::from([WallBrick {
        aabb: [proposal.min.x, proposal.min.y, proposal.max.x, proposal.max.y],
        deformation: [0.0; 4], seed: 0, split_y: 0, split_x: 0,
    }]);
    for hole in holes.iter().filter(|hole| hole.origin == 0) {
        let original = Aabb2 { min: glam::Vec2::new(hole.aabb[0], hole.aabb[1]), max: glam::Vec2::new(hole.aabb[2], hole.aabb[3]) };
        let width = (original.max.x - original.min.x) + 0.2;
        let width = if width > 0.46 { width } else { 0.46 };
        let height = original.max.y - original.min.y;
        let center = original.center();
        let expanded = Aabb2::from_center_dims(center, glam::Vec2::new(width, height));
        let raw = |a: Aabb2| [a.min.x, a.min.y, a.max.x, a.max.y];
        if expanded.intersects(proposal) { resolve_hole_overlap(raw(expanded), &mut pieces)?; }
        if closed {
            let shifted_x = if expanded.center().x + (expanded.max.x - expanded.min.x) * 0.5 > length {
                Some(expanded.center().x - length)
            } else if expanded.center().x < (expanded.max.x - expanded.min.x) * 0.5 {
                Some(expanded.center().x + length)
            } else { None };
            if let Some(x) = shifted_x {
                let shifted = Aabb2::from_center_dims(glam::Vec2::new(x, expanded.center().y), glam::Vec2::new(width, height));
                resolve_hole_overlap(raw(shifted), &mut pieces)?;
            }
        }
    }
    Ok(pieces)
}

/// B `add_skirts`（0x1412221f0）。输入来自同一墙的预处理曲线、
/// 路径 Y、segment 高度和局部已最终化洞口；输出直接追加到主砖数组。
pub fn add_skirts_new(
    wall: &country_core::resources::walls::public_wall_state::PublicWallState,
    id: country_core::resources::walls::WallId,
    curve: &country_core::resources::roofs::geometry::surface::Curve2,
    path_y: &[f32],
    segment_heights: &[f32],
    holes: &[country_core::resources::walls::wall_holes::FinalizedWallHole],
    out: &mut Vec<country_core::geometry::instanced_wall::BrickTransformSsbo>,
) {
    use country_core::geometry::instanced_wall::BrickTransformSsbo;
    use country_core::resources::walls::publication_fields::WallId as ObservedWallId;
    use country_core::systems::wall::construction::parameters::brick_source_id;
    use utils::geometry::Aabb2;
    use utils::wall_space::WallSpaceCoord;
    let maximum_y = wall.calculated_max_y().expect("墙高度尚未计算");
    let ranges: Vec<[f32; 2]> = match wall.is_rectangle() {
        Some(rectangle) => (0..4).map(|i| [rectangle.get_point_u(i), rectangle.get_point_u(i + 1)]).collect(),
        None => vec![[0.0, 1.0]],
    };
    let length = curve.length;
    for height in country_core::resources::walls::split_height_into_skirts(maximum_y) {
        if height < wall.authority.elevation() + 3.5 { continue; }
        let boundaries = skirt_split_boundaries(height, maximum_y, length, &ranges);
        assert!(curve.points.len() >= 2, "墙曲线至少需要两个点");
        let mut index = 0usize;
        let first_width = (curve.points_u[1] - curve.points_u[0]).abs();
        let ratio = first_width / first_width;
        let mut t = if ratio.is_nan() && !first_width.is_nan() {
            f32::from_bits(0xffc0_0000)
        } else { 1.0 - ratio };
        let mut start_profile = sample_skirt_height(segment_heights, index, t);
        let mut start_y = sample_skirt_height(path_y, index, t);
        for pair in boundaries.windows(2) {
            let half = (pair[1] - pair[0]) * 0.5;
            assert!(half >= 0.0, "裙边分割顺序无效");
            advance_skirt_coord(&curve.points_u, &mut index, &mut t, half);
            let middle_profile = sample_skirt_height(segment_heights, index, t);
            let middle_y = sample_skirt_height(path_y, index, t);
            advance_skirt_coord(&curve.points_u, &mut index, &mut t, half);
            let end_profile = sample_skirt_height(segment_heights, index, t);
            let end_y = sample_skirt_height(path_y, index, t);
            let min_y = if start_y <= end_y { start_y } else { end_y };
            let reaches_layer = !(height < min_y) &&
                (wall.shape().tag() < 2 || !(height - middle_y > wall.geometry.metrics.constrained_extra));
            if reaches_layer {
                let profile = if start_profile + middle_profile + end_profile <= 0.0 {
                    [-100.0; 3]
                } else {
                    [start_profile, middle_profile, end_profile]
                };
                let surface = [start_y + profile[0], middle_y + profile[1], end_y + profile[2]];
                if !(height < surface[0] && height < surface[1] && height < surface[2]) {
                    let midpoint = (pair[0] + pair[1]) * 0.5;
                    let half_width = (pair[1] - pair[0]) * 0.5;
                    let distance = [(midpoint - half_width) * length, (midpoint + half_width) * length];
                    let proposal = Aabb2::from_center_dims(
                        glam::Vec2::new(midpoint * length, height),
                        glam::Vec2::new((pair[1] - pair[0]) * length, 0.2),
                    );
                    let pieces = clip_skirt_brick(proposal, holes, wall.shape().tag() != 2, length)
                        .expect("裙边洞口切分失败");
                    for piece in pieces {
                        let width = piece.aabb[2] - piece.aabb[0];
                        let brick_height = piece.aabb[3] - piece.aabb[1];
                        if brick_height < 0.1 { continue; }
                        let start = curve.pos_at_u(pair[0]);
                        let end = curve.pos_at_u(pair[1]);
                        let dx = end[0] - start[0];
                        let dz = end[1] - start[1];
                        let inv = 1.0 / (dz * dz + dx * dx).sqrt();
                        let direction = glam::Vec2::new(dx * inv, dz * inv);
                        let center_x = (piece.aabb[0] + piece.aabb[2]) * 0.5;
                        let center_y = (piece.aabb[1] + piece.aabb[3]) * 0.5;
                        let position = WallSpaceCoord::new_unchecked(center_x, center_y).to_world_space(curve);
                        out.push(BrickTransformSsbo {
                            affine_columns: [
                                [width * direction.x, 0.0, width * direction.y],
                                [0.0, brick_height, 0.0],
                                [-direction.y * 0.76, 0.0, direction.x * 0.76],
                                [position.x, position.y, position.z],
                            ],
                            deformation: [0.0; 4], surface, seed: 0,
                            material: brick_source_id(ObservedWallId(id.0)),
                            wall_distance: distance, flags: 4,
                        });
                    }
                }
            }
            start_profile = end_profile;
            start_y = end_y;
        }
    }
}

fn sample_skirt_height(values: &[f32], index: usize, t: f32) -> f32 {
    t * values[index + 1] + (1.0 - t) * values[index]
}

/// 按曲线缓存的 u 段长度推进，保留逐段减法与原求值顺序。
fn advance_skirt_coord(points_u: &[f32], index: &mut usize, t: &mut f32, distance: f32) {
    let mut remaining;
    loop {
        if *index + 1 >= points_u.len() { *t = 1.0; return; }
        let span = (points_u[*index + 1] - points_u[*index]).abs();
        remaining = (1.0 - *t) * span - distance;
        if remaining >= 0.0 { *t = 1.0 - remaining / span; return; }
        if *index + 2 >= points_u.len() { *t = 1.0; return; }
        *index += 1;
        *t = 0.0;
        break;
    }
    while *index + 1 < points_u.len() {
        let span = (points_u[*index + 1] - points_u[*index]).abs();
        remaining += span;
        if remaining >= 0.0 { *t = 1.0 - remaining / span; return; }
        if *index + 2 >= points_u.len() { *t = 1.0; return; }
        *index += 1;
    }
    *t = 1.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skirt_split_endpoints_match_b_original() {
        let boundaries = skirt_split_boundaries(4.5, 6.0, 6.0, &[[0.0, 1.0]]);
        let expected = [
            0x3f1c9ef2, 0x3fa3197b, 0x4003af74, 0x4029bb48, 0x40583298,
            0x40839fe4, 0x40924acb, 0x40ac60e0, 0x40c00000,
        ];
        assert_eq!(boundaries.len(), expected.len() + 1);
        for (pair, expected) in boundaries.windows(2).zip(expected) {
            let center = (pair[0] + pair[1]) * 0.5;
            let half = (pair[1] - pair[0]) * 0.5;
            assert_eq!(((center + half) * 6.0).to_bits(), expected);
        }
    }
    #[test]
    fn rectangle_ranges_share_the_original_random_stream() {
        let b = skirt_split_boundaries(4.5, 6.0, 12.0, &[[0.0, 0.25], [0.25, 0.5], [0.5, 0.75], [0.75, 1.0]]);
        let expected = [0x3f0f3a1b, 0x3f86a998, 0x3fef41bf, 0x401633da, 0x40400000, 0x406c71e6, 0x4087a021, 0x4095e2ea, 0x40ad1cee, 0x40c00000, 0x40d237d4, 0x40e4b508, 0x40fa0e48, 0x4105921b, 0x41100000, 0x4119df1d, 0x4121e5b1, 0x412aa0c8, 0x4135d104, 0x41400000];
        assert_eq!(b.len(), expected.len() + 1);
        for (pair, expected) in b.windows(2).zip(expected) {
            let center = (pair[0] + pair[1]) * 0.5;
            let half = (pair[1] - pair[0]) * 0.5;
            assert_eq!(((center + half) * 12.0).to_bits(), expected);
        }
    }

    fn native_skirt_fixture(name: &str) -> Vec<u8> {
        let json = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-wall-finish-native-fixtures.json"));
        let row = json.split(&format!("\"name\": \"{name}\"")).nth(1).unwrap();
        let stdout = row.split("\"stdout\": [").nth(1).unwrap();
        let hex = stdout.split('"').nth(3).unwrap();
        hex.as_bytes().chunks_exact(2).map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()
        }).collect()
    }

    #[test]
    fn freeform_skirts_match_original_with_and_without_hole() {
        use country_core::resources::{
            roofs::geometry::surface::Curve2,
            walls::{WallId, inner_wall_state::WallState, public_wall_state::PublicWallState,
                publication_source::TerrainHeightGrid512, height_cache::RegularWallHeightCache,
                wall_holes::FinalizedWallHole},
        };
        let curve = Curve2::try_new(vec![[0.0, 0.0], [3.0, 0.0], [3.0, 3.0]], false).unwrap();
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut wall = PublicWallState::collect(&WallState::new(curve.clone(), 6.0, 1, 0, false), &terrain).unwrap();
        wall.regular_heights = Some(RegularWallHeightCache { maximum_y: 6.0, arch_segment_heights: vec![6.0; 3] });
        wall.geometry.metrics.constrained_extra = 6.0;
        let hole = FinalizedWallHole { aabb: [2.5, 4.2, 3.5, 4.8], kind: 1, origin: 0 };
        for (name, holes) in [("skirt-empty", &[][..]), ("skirt-hole", &[hole][..])] {
            let mut out = Vec::new();
            add_skirts_new(&wall, WallId(17), &curve, &[0.0; 3], &[0.0; 3], holes, &mut out);
            let bytes: Vec<u8> = out.into_iter().flat_map(|brick| brick.observed_bytes()).collect();
            assert_eq!(bytes, native_skirt_fixture(name));
        }
        let mut sloped = Vec::new();
        add_skirts_new(&wall, WallId(17), &curve, &[0.0, 2.0, 0.0], &[0.0, 2.0, 1.0], &[], &mut sloped);
        let bytes: Vec<u8> = sloped.into_iter().flat_map(|brick| brick.observed_bytes()).collect();
        assert_eq!(bytes, native_skirt_fixture("skirt-sloped"));
        wall.geometry.metrics.constrained_extra = 4.5;
        let mut sentinel = Vec::new();
        add_skirts_new(&wall, WallId(17), &curve, &[4.5, 1.0, 2.0], &[0.0, 0.0, 1.0], &[], &mut sentinel);
        let bytes: Vec<u8> = sentinel.into_iter().flat_map(|brick| brick.observed_bytes()).collect();
        assert_eq!(bytes, native_skirt_fixture("skirt-ground-sentinel"));
    }

    #[test]
    fn rectangle_skirts_match_original_across_closed_seam() {
        use country_core::resources::{
            roofs::geometry::surface::Curve2,
            walls::{WallId, inner_wall_state::WallState, public_wall_state::PublicWallState,
                publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512,
                height_cache::RegularWallHeightCache, wall_holes::FinalizedWallHole},
        };
        let curve = Curve2::try_new(vec![[0.0, 0.0], [3.0, 0.0], [3.0, 3.0], [0.0, 3.0], [0.0, 0.0]], false).unwrap();
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut snapshot = WallState::new(curve.clone(), 6.0, 1, 0, false).snapshot().clone();
        let mut shape = [0u8; 28];
        shape[..4].copy_from_slice(&1u32.to_le_bytes());
        for (index, value) in [1.0f32, 0.0, 0.0, 0.0, 3.0, 3.0].iter().enumerate() {
            shape[4 + index * 4..8 + index * 4].copy_from_slice(&value.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let mut wall = PublicWallState::collect(&WallState::from_snapshot(snapshot, 0), &terrain).unwrap();
        wall.regular_heights = Some(RegularWallHeightCache { maximum_y: 6.0, arch_segment_heights: vec![6.0; 5] });
        wall.geometry.metrics.constrained_extra = 6.0;
        let hole = FinalizedWallHole { aabb: [-0.2, 4.2, 0.3, 4.8], kind: 1, origin: 0 };
        for (name, holes) in [("skirt-rectangle-empty", &[][..]), ("skirt-closed-seam", &[hole][..])] {
            let mut out = Vec::new();
            add_skirts_new(&wall, WallId(17), &curve, &[0.0; 5], &[0.0; 5], holes, &mut out);
            let bytes: Vec<u8> = out.into_iter().flat_map(|brick| brick.observed_bytes()).collect();
            assert_eq!(bytes, native_skirt_fixture(name));
        }
        let curve = Curve2::try_new(vec![[0.0, 0.0], [3.0, 0.0], [3.0, 5.0], [0.0, 5.0], [0.0, 0.0]], false).unwrap();
        let mut snapshot = wall.authority.snapshot().clone();
        snapshot.source.curve = curve.clone();
        shape[24..28].copy_from_slice(&5.0f32.to_le_bytes());
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let mut wall = PublicWallState::collect(&WallState::from_snapshot(snapshot, 0), &terrain).unwrap();
        wall.regular_heights = Some(RegularWallHeightCache { maximum_y: 6.0, arch_segment_heights: vec![6.0; 5] });
        wall.geometry.metrics.constrained_extra = 6.0;
        let mut out = Vec::new();
        add_skirts_new(&wall, WallId(17), &curve, &[0.0; 5], &[0.0; 5], &[], &mut out);
        let bytes: Vec<u8> = out.into_iter().flat_map(|brick| brick.observed_bytes()).collect();
        assert_eq!(bytes, native_skirt_fixture("skirt-rectangle-3x5"));
    }

    #[test]
    fn repeated_first_point_preserves_original_unordered_surface() {
        use country_core::resources::{
            roofs::geometry::surface::Curve2,
            walls::{WallId, inner_wall_state::WallState, public_wall_state::PublicWallState,
                publication_source::TerrainHeightGrid512, height_cache::RegularWallHeightCache},
        };
        let curve = Curve2::try_new(vec![[0.0, 0.0], [0.0, 0.0], [3.0, 0.0], [3.0, 3.0]], false).unwrap();
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut wall = PublicWallState::collect(&WallState::new(curve.clone(), 6.0, 1, 0, false), &terrain).unwrap();
        wall.regular_heights = Some(RegularWallHeightCache { maximum_y: 6.0, arch_segment_heights: vec![6.0; 4] });
        wall.geometry.metrics.constrained_extra = 6.0;
        let mut out = Vec::new();
        add_skirts_new(&wall, WallId(17), &curve, &[0.0, 0.0, 1.0, 0.0], &[0.0, 0.0, 1.0, 0.0], &[], &mut out);
        let bytes: Vec<u8> = out.into_iter().flat_map(|brick| brick.observed_bytes()).collect();
        assert_eq!(bytes, native_skirt_fixture("skirt-repeat-first"));
    }

}
