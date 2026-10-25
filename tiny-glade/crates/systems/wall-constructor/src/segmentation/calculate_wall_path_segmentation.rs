//! 墙路径分段计算（segmentation/calculate_wall_path_segmentation.rs）。
//!
//! `calculate_wall_path_segmentation`（0x14074f7a0）/`calculate_path_wall_segmentation`
//! （0x14074f310）。候选参数面（skeleton 富化）：
//! `(EventReader<OnWallChanged>, ResMut<PublicWalls>,
//!  ResMut<WallPathSegmentationMasks>, Res<UiSignifierStream>)`。
//!
//! 机器码（building-segmentation B-core/B-process 冻结）：
//! - 分段主流程（puffin scope）；`process_segment`（0x140750420）逐段处理；
//! - `add_splits_in_empty_range`（0x140751240）：常量 1.0/0.5/1.5 +
//!   u32 转换守卫 0x5f000000/0x5f7fffff；
//! - `score_split_candidate`（0x140751610）：分裂候选评分。
use bevy_ecs::prelude::*;
use country_core::resources::walls::public_walls::{OnWallChanged, PublicWalls};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::wall::construction::segmentation::wall_path_segmentation;

/// B 0x14074f310：分段计算主入口（每墙掩码产出 → 136B 掩码写回）。
pub fn calculate_wall_path_segmentation(
    mut changed: EventReader<OnWallChanged>,
    mut walls: ResMut<PublicWalls>,
    mut masks: ResMut<crate::construct_arches::WallPathSegmentationMasks>,
    _signifiers: Res<UiSignifierStream>,
) {
    for event in changed.read() {
        let wall = walls
            .get_mut(event.0)
            .expect("segmentation changed event must reference a published wall");
        let projection = wall.building_projection();
        let path = wall
            .prepared_path
            .as_ref()
            .expect("segmentation requires prepared wall path");
        let input =
            country_core::systems::wall::construction::segmentation::WallSegmentationInput {
                prepared_positions: &path.positions,
                path_values: &path.segment_values,
                shape: &projection.shape(),
                observed_special_flag: projection.observed_296(),
            };
        let mask = wall_path_segmentation(input)
            .expect("invalid authoritative wall path segmentation input");
        wall.material_stats
            .init_this_update(path.length, projection.observed_296() != 0);
        wall.material_stats
            .set_path_segmentation_count(mask.len() as u32);
        masks.masks.insert(event.0.0, mask);
    }
}

/// B 0x140750420：逐段处理。真实实现位于 country-core 的权威分段生产者
/// （`wall_path_segmentation` 的逐段构造，0x14074f310 的活路径调用同一函数），
/// 此处直接复用，避免第二份副本。
pub use country_core::systems::wall::construction::segmentation::process_segment;

/// B 0x140751240：空区间补分裂（逐语句重建：
/// building-segmentation/B-process/140751240.c）。
///
/// - 由 `LoopSlice<Vec3>` 的 xz 点构造 Curve；
/// - `n = floor(curve.length / spacing)`；`n == 1 且 length < spacing*1.5`
///   → 0（0x3fc00000=1.5）；
/// - `for i in 1..=n`：`t = i/(n+1)`，断言 t∈[0,1] 且 !NaN；
///   `coord = curve.coord_at_u(t)`；`index = base + segment +
///   (0.5 < fraction)`；`flags[index] |= 8`；`coords[index] = coord`。
///
/// `base` 为掩码全局起点（原件 in_RCX），`flags`/`coords` 为掩码数组。
pub fn add_splits_in_empty_range(
    base: usize,
    points: &[[f32; 3]],
    flags: &mut [u8],
    coords: &mut [WallPathSegmentationMaskCoord],
    spacing: f32,
) {
    use country_core::resources::roofs::geometry::surface::Curve2;
    let planar: Vec<[f32; 2]> = points.iter().map(|p| [p[0], p[2]]).collect();
    let Ok(curve) = Curve2::try_new(planar, false) else {
        return;
    };
    let rounded = (curve.length / spacing).floor();
    if rounded <= 0.0 {
        return;
    }
    let mut count = if rounded > u64::MAX as f32 {
        u64::MAX
    } else {
        rounded as u64
    };
    // `n == 1 && length < spacing*1.5 → 0`。
    if count == 1 && curve.length < spacing * f32::from_bits(0x3fc00000) {
        count = 0;
    }
    for i in 1..=count {
        let t = i as f32 / (count + 1) as f32;
        assert!(
            t.is_finite() && (0.0..=1.0).contains(&t),
            "add_splits_in_empty_range: coordinate out of range"
        );
        let (segment, fraction) = curve.coord_at_u(t);
        let index = base + segment + usize::from(fraction > 0.5);
        assert!(
            index < flags.len() && index < coords.len(),
            "add_splits_in_empty_range: index out of bounds"
        );
        flags[index] |= 8;
        coords[index] = WallPathSegmentationMaskCoord {
            point_index: segment as u32,
            fraction,
        };
    }
}

/// 掩码坐标（原 `ThresholdedCurveCoord` 的 8B 记录投影）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallPathSegmentationMaskCoord {
    pub point_index: u32,
    pub fraction: f32,
}

impl WallPathSegmentationMaskCoord {
    pub fn observed_bytes(&self) -> [u8; 8] {
        let mut out = [0u8; 8];
        out[..4].copy_from_slice(&self.point_index.to_le_bytes());
        out[4..].copy_from_slice(&self.fraction.to_le_bytes());
        out
    }
}

/// B 0x140751610 `score_split_candidate(index, curve_slice, flags, len)`：
/// `flags[index] & 0xf != 0` → 0；否则沿 flags 非零点向两侧各延伸 2.0
/// （`extend_left_until/extend_right_until`，停止条件 `flags & 0xf != 0`），
/// 取两端点连线上的投影，返回 `(|xz 偏差| + proj.y - p.y) * 0.5`。
/// 与 country-core 活路径共用同一实现。
pub fn score_split_candidate(index: usize, points: &[[f32; 3]], flags: &[u8]) -> f32 {
    country_core::systems::wall::construction::segmentation::score(index, points, flags)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_splits_marks_nearest_endpoint_bucket() {
        let points = [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0]];
        let mut flags = vec![0u8; 16];
        let mut coords = vec![
            WallPathSegmentationMaskCoord {
                point_index: 0,
                fraction: 0.0,
            };
            16
        ];
        add_splits_in_empty_range(0, &points, &mut flags, &mut coords, 2.0);
        assert_eq!(flags[0], 8);
        assert_eq!(coords[0].point_index, 0);
        assert!(coords[0].fraction > 0.0 && coords[0].fraction <= 1.0);
    }

    #[test]
    fn add_splits_skips_single_short_segment() {
        let points = [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0]];
        let mut flags = vec![0u8; 4];
        let mut coords = vec![
            WallPathSegmentationMaskCoord {
                point_index: 0,
                fraction: 0.0,
            };
            4
        ];
        // length=10, spacing=10 → n=1 且 10 < 15 → 跳过。
        add_splits_in_empty_range(0, &points, &mut flags, &mut coords, 10.0);
        assert_eq!(flags, vec![0u8; 4]);
    }

    #[test]
    fn score_split_candidate_flagged_point_is_zero_and_collinear_is_zero() {
        let pts = [[0.0, 0.0, 0.0], [5.0, 0.0, 0.0], [10.0, 0.0, 0.0], [15.0, 0.0, 0.0]];
        let flags = [2u8, 0, 0, 4];
        assert_eq!(score_split_candidate(1, &pts, &flags), 0.0);
        let flagged = [2u8, 1, 0, 4];
        assert_eq!(score_split_candidate(1, &pts, &flagged), 0.0);
    }

    #[test]
    fn score_split_candidate_bent_point_scores_half_deviation() {
        // B(10,0,2) 两侧各 2.0 弧长处的连线在 B 的投影 z = 2*(1-2/|AB|)...
        let pts = [[0.0, 0.0, 0.0], [10.0, 0.0, 2.0], [20.0, 0.0, 0.0], [30.0, 0.0, 0.0]];
        let flags = [2u8, 0, 0, 4];
        let ab = (104.0f32).sqrt();
        let proj_z = 2.0 * (1.0 - 2.0 / ab);
        let expect = (2.0 - proj_z) * 0.5;
        let got = score_split_candidate(1, &pts, &flags);
        assert!((got - expect).abs() < 1e-3, "got {got} expect {expect}");
    }
}

