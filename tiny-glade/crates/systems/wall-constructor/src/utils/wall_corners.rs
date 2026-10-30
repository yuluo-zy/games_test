//! 墙角砖（utils/wall_corners.rs）。
//!
//! `add_wall_corners`（0x141215540，construct_walls 实调；G05 caller 链）。
use glam::Vec2;

/// 原 0x141215540 的夹角窗口（rine-read .rdata）：
/// 0x142c1c4e0 = −0.1736482、0x142c1c4e4 = 0.17364822（±10° 的余弦窗口）。
pub const CORNER_ANGLE_MIN: f32 = f32::from_bits(0xbe31d0d5);
pub const CORNER_ANGLE_MAX: f32 = f32::from_bits(0x3e31d0d7);

/// 原指令先将两条向量的长度平方相乘，再开方；保持计算顺序。
pub fn check_angle(previous: Vec2, current: Vec2, next: Vec2) -> bool {
    let a = next - current;
    let b = current - previous;
    let numerator = a.y * b.y + a.x * b.x;
    let denominator = ((a.x * a.x + a.y * a.y) * (b.x * b.x + b.y * b.y)).sqrt();
    let dot = numerator / denominator;
    dot.to_bits() & 0x7fff_ffff <= 0x7f7f_ffff
        && CORNER_ANGLE_MIN <= dot
        && dot < CORNER_ANGLE_MAX
}

/// 原配对扫描；闭合轮廓以倒数第二点作为接缝的前一点。
pub fn find_corner_indices(points: &[Vec2], closed: bool) -> Vec<usize> {
    let mut out = Vec::new();
    if closed {
        assert!(points.len() >= 2, "closed corner outline needs its repeated endpoint");
        if check_angle(points[points.len()-2], points[0], points[1]) { out.push(0); }
    }
    let mut cursor = 0usize;
    let mut previous = 0usize;
    let mut current = 0usize;
    let mut first = true;
    let mut ordinal = 0usize;
    while cursor != points.len() {
        let mut next = cursor + 1;
        if first {
            if next == points.len() || cursor + 2 == points.len() { break; }
            previous = cursor;
            current = next;
            cursor += 2;
            next = cursor + 1;
        }
        if check_angle(points[previous], points[current], points[cursor]) {
            ordinal += 1;
            previous = current;
            current = cursor;
            cursor = next;
        } else {
            ordinal += 2;
            loop {
                if next == points.len() { return out; }
                if check_angle(points[current], points[cursor], points[next]) { break; }
                ordinal += 1;
                current = cursor;
                cursor = next;
                next += 1;
            }
            previous = cursor;
            current = next;
            cursor = next + 1;
        }
        out.push(ordinal);
        first = false;
    }
    out
}

/// 开放轮廓入口；闭合状态必须由真实墙形状提供。
pub fn find_corners(points: &[Vec2]) -> Vec<usize> {
    find_corner_indices(points, false)
}


/// 原 0x141215b66..c24：角处原始洞口的中心高度进入随机种子。
fn corner_rng_seed(
    corner: usize,
    x: f32,
    holes: &[country_core::resources::walls::wall_holes::RawWallHole],
) -> u64 {
    let mut seed = corner as u64;
    for hole in holes {
        if (x - hole.center[0]).abs() < 0.01 {
            seed = seed.wrapping_add(hole.center[1] as u64);
        }
    }
    seed
}

/// 原 0x141215b80 前的高度分割；调用者保留同一 RNG 供随后的砖宽消费。
fn corner_vertical_splits(bottom: f32, top: f32, rng: &fastrand::Rng) -> Vec<f32> {
    let height = top - bottom;
    let count = (height / 0.69).ceil() as usize;
    utils::random_splits(if count < 3 { 2 } else { count }, 0.21 / height, rng)
}

/// 原 0x141215e6d..eb0：每行先消费随机值，闭合轮廓末行再覆盖为固定宽度。
fn corner_row_width(rng: &fastrand::Rng, final_closed_row: bool) -> f32 {
    let sample = rng.f32();
    if final_closed_row { 0.76 } else { (sample * 0.16 - (1.0 - sample) * 0.16) + 0.92 }
}

/// 原交替墙边的砖中心；闭合墙按原墙空间投影接缝。
fn corner_brick_x(space: &utils::wall_space::WallSpace, corner_x: f32, width: f32, row: usize) -> f32 {
    use utils::comparative_space::ComparativeSpace;
    let offset = (width * 0.5 + -0.28) + -0.0385;
    let direction = if row % 2 == 0 { 1.0 } else { -1.0 };
    space.project_clamp(direction * offset + corner_x)
}

/// B `add_wall_corners`（0x141215540）。局部 WIP 洞口先供所有角读取，
/// 新角洞在函数结束时统一追加；输出与父构造器共用主砖数组。
pub fn add_wall_corners_new(
    wall: &country_core::resources::walls::public_wall_state::PublicWallState,
    id: country_core::resources::walls::WallId,
    path_y: &[f32],
    top_heights: &[f32],
    out: &mut Vec<country_core::geometry::instanced_wall::BrickTransformSsbo>,
    holes: &mut country_core::resources::walls::wall_holes::WallHoleStorage,
    mode: u8,
    top: country_core::systems::wall::construction::production::WallTopBounds,
) {
    use country_core::geometry::instanced_wall::BrickTransformSsbo;
    use country_core::resources::roofs::geometry::surface::Curve2;
    use country_core::resources::walls::publication_fields::WallId as ObservedWallId;
    use country_core::resources::walls::wall_holes::RawWallHole;
    use country_core::systems::wall::construction::bricks::{resolve_hole_overlap, WallBrick};
    use country_core::systems::wall::construction::parameters::brick_source_id;
    use std::collections::VecDeque;
    use utils::comparative_space::ComparativeSpace;
    use utils::geometry::Aabb2;
    use utils::wall_space::{WallSpace, WallSpaceCoord};
    let path = wall.prepared_path.as_ref().expect("墙路径尚未准备");
    let curve = Curve2::try_new(path.positions.iter().map(|p| [p[0], p[2]]).collect(), false)
        .expect("墙曲线无效");
    let outline = wall.simplified_curve_2d();
    let points: Vec<Vec2> = outline.points.iter().map(|p| Vec2::new(p[0], p[1])).collect();
    let closed = wall.shape().tag() != 2;
    let corners = find_corner_indices(&points, closed);
    let space = WallSpace { is_loop: closed, length: wall.wall_space_length() };
    let mut generated = Vec::new();
    for corner in corners {
        let previous = points[if corner == 0 { points.len() - 2 } else { corner - 1 }];
        let current = points[corner];
        let next = points[if corner + 1 == points.len() { 0 } else { corner + 1 }];
        let u = outline.points_u[corner];
        let (index, t) = curve.coord_at_u(u);
        let bottom = path_y[index] + (path_y[index + 1] - path_y[index]) * t;
        let mut upper = top_heights[index] + (top_heights[index + 1] - top_heights[index]) * t;
        if mode != 0 {
            upper += -0.46;
        } else if corner == 0 {
            let (last, last_t) = curve.coord_at_u(1.0);
            let last_upper = top_heights[last] + (top_heights[last + 1] - top_heights[last]) * last_t;
            upper = upper.max(last_upper);
        }
        let corner_x = u * space.length;
        let seed = corner_rng_seed(corner, corner_x, holes.raw_holes());
        let mut ordinal = seed.wrapping_sub(corner as u64);
        let rng = fastrand::Rng::with_seed(seed);
        let splits = corner_vertical_splits(bottom, upper, &rng);
        for (row, pair) in splits.windows(2).enumerate() {
            let lower = pair[0] * upper + (1.0 - pair[0]) * bottom;
            let higher = pair[1] * upper + (1.0 - pair[1]) * bottom;
            let mut width = corner_row_width(&rng, closed && row + 2 == splits.len());
            if higher - lower <= 0.0 || width <= 0.0 { continue; }
            if top.tag != 2 && lower >= wall.max_y() { width = 0.56; }
            let x = corner_brick_x(&space, corner_x, width, row);
            let proposal = Aabb2::from_center_dims(Vec2::new(x, (lower + higher) * 0.5), Vec2::new(width, higher - lower));
            let mut pieces = VecDeque::from([WallBrick {
                aabb: [proposal.min.x, proposal.min.y, proposal.max.x, proposal.max.y],
                deformation: [0.0; 4], seed: 0, split_y: 0, split_x: 0,
            }]);
            if closed && row + 2 == splits.len() {
                // 末行的洞口按中心位置收缩上下边，不作普通矩形分裂。
                let mut trimmed = proposal;
                for hole in holes.raw_holes() {
                    let h = Aabb2::from_center_dims(Vec2::from_array(hole.center), Vec2::from_array(hole.dimensions));
                    if h.min.x <= trimmed.center().x && trimmed.center().x < h.max.x {
                        trim_corner_final_row(&mut trimmed, h);
                    }
                    let center = h.center();
                    if center.x + (h.max.x - h.min.x) * 0.5 > space.length {
                        let shifted = Aabb2::from_center_dims(Vec2::new(center.x - space.length, center.y), Vec2::from_array(hole.dimensions));
                        trim_corner_final_row(&mut trimmed, shifted);
                    } else if center.x < (h.max.x - h.min.x) * 0.5 {
                        let shifted = Aabb2::from_center_dims(Vec2::new(center.x + space.length, center.y), Vec2::from_array(hole.dimensions));
                        trim_corner_final_row(&mut trimmed, shifted);
                    }
                    if trimmed.min.y >= trimmed.max.y { break; }
                }
                pieces.clear();
                if !(trimmed.max.y - trimmed.min.y <= 0.1) {
                    pieces.push_back(WallBrick {
                        aabb: [trimmed.min.x, trimmed.min.y, trimmed.max.x, trimmed.max.y],
                        deformation: [0.0; 4], seed: 0, split_y: 0, split_x: 0,
                    });
                }
            } else {
                for hole in holes.raw_holes() {
                    let h = Aabb2::from_center_dims(Vec2::from_array(hole.center), Vec2::from_array(hole.dimensions));
                    if h.intersects(proposal) {
                        resolve_hole_overlap([h.min.x, h.min.y, h.max.x, h.max.y], &mut pieces)
                            .expect("墙角洞口切分失败");
                    }
                    if closed {
                        let center = h.center();
                        let shifted = if center.x + (h.max.x - h.min.x) * 0.5 > space.length {
                            Some(center.x - space.length)
                        } else if center.x < (h.max.x - h.min.x) * 0.5 {
                            Some(center.x + space.length)
                        } else { None };
                        if let Some(x) = shifted {
                            let h = Aabb2::from_center_dims(Vec2::new(x, center.y), Vec2::from_array(hole.dimensions));
                            resolve_hole_overlap([h.min.x, h.min.y, h.max.x, h.max.y], &mut pieces)
                                .expect("墙角接缝洞口切分失败");
                        }
                    }
                }
            }
            let sample = |x: f32| {
                let coord = space.project_clamp(x) / space.length;
                let (index, t) = curve.coord_at_u(coord);
                let blend = |values: &[f32]| t * values[index + 1] + (1.0 - t) * values[index];
                [blend(&wall.segment_heights), blend(path_y), blend(top_heights)]
            };
            let a = sample(x - width * 0.5);
            let b = sample(x);
            let c = sample(x + width * 0.5);
            let surface = if !(a[0] + b[0] + c[0] <= 0.0) {
                [a[0] + a[1], b[0] + b[1], c[0] + c[1]]
            } else { [-100.0; 3] };
            if surface[0] > higher && surface[1] > higher && surface[2] > higher { continue; }
            let direction = if row % 2 == 0 { next - current } else { current - previous };
            let inv = 1.0 / (direction.y * direction.y + direction.x * direction.x).sqrt();
            let direction = direction * inv;
            let axes = utils::xz_rotation_mat3_bad(Vec2::new(-direction.y, direction.x));
            ordinal = ordinal.wrapping_add(1);
            for piece in pieces {
                let pw = piece.aabb[2] - piece.aabb[0];
                let ph = piece.aabb[3] - piece.aabb[1];
                if !(pw > 0.1 && higher - lower > 0.1) { continue; }
                let px = (piece.aabb[0] + piece.aabb[2]) * 0.5;
                let py = (piece.aabb[1] + piece.aabb[3]) * 0.5;
                let position = WallSpaceCoord::new_unchecked(px, py).to_world_space(&curve);
                let x_axis = axes.x_axis * pw;
                let y_axis = axes.y_axis * ph;
                let z_axis = axes.z_axis * 0.637;
                let is_fence = top.tag != 2 && top.tag != 0;
                let reaches_fence = is_fence && (py + ph * 0.5 > a[2] || py + ph * 0.5 > c[2]);
                let piece_surface = if reaches_fence { [a[2] - 0.1, b[2] - 0.1, c[2] - 0.1] } else { surface };
                let flags = if reaches_fence { 12 } else if (py - upper).abs() < 0.75 { 20 } else { 4 };
                out.push(BrickTransformSsbo {
                    affine_columns: [x_axis.to_array(), y_axis.to_array(), z_axis.to_array(), position.to_array()],
                    deformation: piece.deformation, surface: piece_surface, seed: ordinal as u32,
                    material: brick_source_id(ObservedWallId(id.0)),
                    wall_distance: [px - pw * 0.5, px + pw * 0.5], flags,
                });
                generated.push(RawWallHole { center: [px, py], dimensions: [pw, ph], kind: 2 });
            }
        }
    }
    for hole in generated { holes.add(hole).expect("墙角要求 WIP 洞口"); }
}

fn trim_corner_final_row(trimmed: &mut utils::geometry::Aabb2, hole: utils::geometry::Aabb2) {
    let hole_y = hole.center().y;
    let brick_y = trimmed.center().y;
    if hole_y > brick_y {
        trimmed.max.y = trimmed.max.y.min(hole.min.y);
    } else if brick_y > hole_y {
        trimmed.min.y = trimmed.min.y.max(hole.max.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missed_window_and_closed_seam() {
        let p = [Vec2::ZERO, Vec2::X, Vec2::new(2.0, 0.0), Vec2::new(2.0, 1.0), Vec2::new(3.0, 1.0), Vec2::new(3.0, 2.0)];
        assert_eq!(find_corners(&p), [2, 3, 4]);
        let p = [Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y, Vec2::ZERO];
        assert_eq!(find_corner_indices(&p, true), [0, 1, 2, 3]);
    }

    #[test]
    fn check_angle_detects_right_angle_window() {
        assert!(check_angle(
            Vec2::new(-1.0, 0.0),
            Vec2::ZERO,
            Vec2::new(0.0, 1.0)
        ));
        // 45° 转角不在 ±10° 窗口内。
        assert!(!check_angle(
            Vec2::new(-1.0, 0.0),
            Vec2::ZERO,
            Vec2::new(1.0, 1.0)
        ));
    }

    #[test]
    fn find_corners_greedy_scan() {
        // 直角折线：角点位于索引 1。
        let points = [
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
        ];
        assert_eq!(find_corners(&points), vec![1]);
        // 直线无角点。
        let line = [Vec2::ZERO, Vec2::new(1.0, 0.0), Vec2::new(2.0, 0.0)];
        assert!(find_corners(&line).is_empty());
    }
    #[test]
    fn vertical_splits_match_original_corner_centers() {
        let rng = fastrand::Rng::with_seed(1);
        let boundaries = corner_vertical_splits(0.0, 3.0, &rng);
        let expected = [0x3ed492fe, 0x3f8fecde, 0x3fe6866e, 0x4025df28];
        assert_eq!(boundaries.len(), expected.len() + 1);
        for (pair, expected) in boundaries.windows(2).zip(expected) {
            let bottom = pair[0] * 3.0 + (1.0 - pair[0]) * 0.0;
            let top = pair[1] * 3.0 + (1.0 - pair[1]) * 0.0;
            assert_eq!(((bottom + top) * 0.5).to_bits(), expected);
        }
    }

    #[test]
    fn vertical_splits_and_widths_share_original_random_stream() {
        let rng = fastrand::Rng::with_seed(1);
        let _ = corner_vertical_splits(0.0, 3.0, &rng);
        let centers = [3.220787763595581f32, 2.930558919906616, 3.2156245708465576, 2.8723583221435547];
        let expected = [1.078575611114502f32, 0.7758822441101074, 1.068249225616455, 0.8922834396362305];
        for (center, expected) in centers.into_iter().zip(expected) {
            let width = corner_row_width(&rng, false);
            let bounds = utils::geometry::Aabb2::from_center_dims(Vec2::new(center, 0.0), Vec2::new(width, 1.0));
            assert_eq!((bounds.max.x - bounds.min.x).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn original_corner_hole_changes_random_seed_before_splitting() {
        use country_core::resources::walls::wall_holes::RawWallHole;
        let hole = RawWallHole { center: [3.0, 1.5], dimensions: [0.5, 1.0], kind: 1 };
        let seed = corner_rng_seed(1, 3.0, &[hole]);
        assert_eq!(seed, 2);
        let rng = fastrand::Rng::with_seed(seed);
        let b = corner_vertical_splits(0.0, 3.0, &rng);
        let first = ((b[0] * 3.0 + b[1] * 3.0) * 0.5).to_bits();
        assert_eq!(first, 0x3ec007c2);
    }

    #[test]
    fn original_corner_alternates_projected_centers() {
        let rng = fastrand::Rng::with_seed(1);
        let _ = corner_vertical_splits(0.0, 3.0, &rng);
        let space = utils::wall_space::WallSpace { is_loop: false, length: 6.0 };
        let expected = [0x404e2163, 0x403b8e47, 0x404dcccb, 0x4037d4b8];
        for (row, expected) in expected.into_iter().enumerate() {
            let width = corner_row_width(&rng, false);
            let x = corner_brick_x(&space, 3.0, width, row);
            let bounds = utils::geometry::Aabb2::from_center_dims(Vec2::new(x, 0.0), Vec2::new(width, 1.0));
            assert_eq!(bounds.center().x.to_bits(), expected);
        }
    }

    #[test]
    fn freeform_corner_matches_original_bricks_and_raw_holes() {
        use country_core::resources::{
            roofs::geometry::surface::Curve2,
            walls::{WallId, inner_wall_state::WallState, public_wall_state::PublicWallState,
                publication_source::TerrainHeightGrid512, wall_holes::{WallHoleStorage, RawWallHole}},
        };
        use country_core::systems::wall::{wall_state::path_preparation::PreparedWallPath,
            construction::production::WallTopBounds};
        let curve = Curve2::try_new(vec![[0.0, 0.0], [3.0, 0.0], [3.0, 3.0]], false).unwrap();
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut wall = PublicWallState::collect(&WallState::new(curve, 3.0, 0, 0, false), &terrain).unwrap();
        wall.prepared_path = Some(PreparedWallPath { positions: vec![[0.0, 0.0, 0.0], [3.0, 0.0, 0.0], [3.0, 0.0, 3.0]],
            points_u: vec![0.0, 0.5, 1.0], length: 6.0, segment_values: vec![0.0; 3] });
        wall.segment_heights = vec![0.0; 3];
        let fixture = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-wall-finish-native-fixtures.json"));
        for name in ["corner-empty", "corner-hole"] {
            let mut holes = WallHoleStorage::new_in_progress();
            if name == "corner-hole" {
                holes.add(RawWallHole { center: [3.0, 1.5], dimensions: [0.5, 1.0], kind: 1 }).unwrap();
            }
            let mut out = Vec::new();
            add_wall_corners_new(&wall, WallId(17), &[0.0; 3], &[3.0; 3], &mut out, &mut holes, 0, WallTopBounds::PLAIN);
            let bricks: Vec<u8> = out.into_iter().flat_map(|brick| brick.observed_bytes()).collect();
            let mut raw = Vec::new();
            for hole in holes.raw_holes() {
                for x in hole.center.into_iter().chain(hole.dimensions) { raw.extend_from_slice(&x.to_bits().to_le_bytes()); }
                raw.extend_from_slice(&[hole.kind, 0, 0, 0]);
            }
            let row = fixture.split(&format!("\"name\": \"{name}\"")).nth(1).unwrap();
            let stdout = row.split("\"stdout\": [").nth(1).unwrap();
            let decode = |value: &str| -> Vec<u8> { value.as_bytes().chunks_exact(2).map(|p|
                u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap()).collect() };
            assert_eq!(bricks, decode(stdout.split('"').nth(3).unwrap()));
            assert_eq!(raw, decode(stdout.split('"').nth(5).unwrap()));
        }
        // 原入口结构顶样本，最后一砖跨过顶线并使用 flag 12。
        wall.regular_heights = Some(country_core::resources::walls::height_cache::RegularWallHeightCache {
            maximum_y: 3.0, arch_segment_heights: vec![3.0; 3],
        });
        let mut holes = WallHoleStorage::new_in_progress();
        let mut out = Vec::new();
        add_wall_corners_new(&wall, WallId(17), &[0.5, 0.5, 0.0], &[4.0, 5.0, 2.0],
            &mut out, &mut holes, 0, WallTopBounds { tag: 1, ..WallTopBounds::PLAIN });
        assert_eq!(out.last().unwrap().flags, 12);
        let matrix = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-corner-top-matrix-recheck.json"));
        let row = matrix.split("\"case\": 8,").nth(1).unwrap();
        let native = row.split("\"native\": [").nth(1).unwrap();
        let hex = native.split('"').nth(3).unwrap();
        let expected: Vec<u8> = hex.as_bytes().chunks_exact(2).map(|p|
            u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap()).collect();
        let bricks: Vec<u8> = out.into_iter().flat_map(|brick| brick.observed_bytes()).collect();
        assert_eq!(bricks, expected);
        // 原入口经 Aabb2::from_center_dims 对非有限尺寸触发 panic。
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut holes = WallHoleStorage::new_in_progress();
            let mut out = Vec::new();
            add_wall_corners_new(&wall, WallId(17), &[0.0, f32::NAN, 0.0], &[3.0; 3],
                &mut out, &mut holes, 0, WallTopBounds::PLAIN);
        }));
        assert!(failed.is_err());
    }

    #[test]
    fn closed_corner_matches_original_with_holes_at_corner_and_seam() {
        use country_core::resources::{
            roofs::geometry::surface::Curve2,
            walls::{WallId, inner_wall_state::WallState, public_wall_state::PublicWallState,
                publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512,
                wall_holes::{WallHoleStorage, RawWallHole}},
        };
        use country_core::systems::wall::{wall_state::path_preparation::PreparedWallPath,
            construction::production::WallTopBounds};
        let curve = Curve2::try_new(vec![[0.0, 0.0], [3.0, 0.0], [3.0, 3.0], [0.0, 3.0], [0.0, 0.0]], false).unwrap();
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut snapshot = WallState::new(curve, 3.0, 0, 0, false).snapshot().clone();
        let mut shape = [0u8; 28]; shape[..4].copy_from_slice(&1u32.to_le_bytes());
        for (index, value) in [1.0f32, 0.0, 0.0, 0.0, 3.0, 3.0].iter().enumerate() {
            shape[4 + index * 4..8 + index * 4].copy_from_slice(&value.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let mut wall = PublicWallState::collect(&WallState::from_snapshot(snapshot, 0), &terrain).unwrap();
        wall.prepared_path = Some(PreparedWallPath { positions: vec![[0.0, 0.0, 0.0], [3.0, 0.0, 0.0],
            [3.0, 0.0, 3.0], [0.0, 0.0, 3.0], [0.0, 0.0, 0.0]],
            points_u: vec![0.0, 0.25, 0.5, 0.75, 1.0], length: 12.0, segment_values: vec![0.0; 5] });
        wall.segment_heights = vec![0.0; 5];
        let fixture = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-wall-finish-native-fixtures.json"));
        for (name, x) in [("corner-closed-empty", None), ("corner-closed-hole", Some(3.0)), ("corner-closed-seam", Some(0.0))] {
            let mut holes = WallHoleStorage::new_in_progress();
            if let Some(x) = x {
                holes.add(RawWallHole { center: [x, 2.7], dimensions: [0.5, 0.5], kind: 1 }).unwrap();
            }
            let mut out = Vec::new();
            add_wall_corners_new(&wall, WallId(17), &[0.0; 5], &[3.0; 5], &mut out, &mut holes, 0, WallTopBounds::PLAIN);
            let bricks: Vec<u8> = out.into_iter().flat_map(|brick| brick.observed_bytes()).collect();
            let mut raw = Vec::new();
            for hole in holes.raw_holes() {
                for x in hole.center.into_iter().chain(hole.dimensions) { raw.extend_from_slice(&x.to_bits().to_le_bytes()); }
                raw.extend_from_slice(&[hole.kind, 0, 0, 0]);
            }
            let row = fixture.split(&format!("\"name\": \"{name}\"")).nth(1).unwrap();
            let stdout = row.split("\"stdout\": [").nth(1).unwrap();
            let decode = |value: &str| -> Vec<u8> { value.as_bytes().chunks_exact(2).map(|p|
                u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap()).collect() };
            assert_eq!(bricks, decode(stdout.split('"').nth(3).unwrap()));
            assert_eq!(raw, decode(stdout.split('"').nth(5).unwrap()));
        }
    }

}
