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
use country_core::systems::wall::construction::segmentation::WallPathSegmentationMask;
use country_core::systems::wall::construction::segmentation::wall_path_segmentation;

/// B 0x14074f310：分段计算主入口（每墙掩码产出 → 136B 掩码写回）。
pub fn calculate_wall_path_segmentation(
    mut changed: EventReader<OnWallChanged>,
    walls: Res<PublicWalls>,
    mut masks: ResMut<crate::construct_arches::WallPathSegmentationMasks>,
    _signifiers: Res<UiSignifierStream>,
) {
    for event in changed.read() {
        let Some(wall) = walls.get(event.0) else {
            continue;
        };
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
                observed_special_flag: projection.observed_294(),
            };
        if let Ok(mask) = wall_path_segmentation(input) {
            masks.masks.insert(event.0.0, mask);
        }
    }
}

/// B 0x140750420：逐段处理。
/// **未实现（U）**：旧 `[0.0;4]` 比例近似不是原逻辑。原件为 mask LoopRange
/// 长度分配 + 逐段 flags/coords 处理；其 closure/输出结构尚未逐语句锚定。
pub fn process_segment(mask: &WallPathSegmentationMask, out: &mut Vec<[f32; 4]>) {
    let _ = (mask, out);
    todo!("未实现: process_segment 0x140750420（段处理 closure/输出结构未取证）")
}

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

/// B 0x140751610：分裂候选处理。
/// **未实现（U）**：原件无返回值，按 `flags[index]&0xf==0` 走
/// `CurveSlice::extend_left_until/extend_right_until`（closure_env$0/$1
/// 未反编译）→ 两次 `sample_at_coord` → 以墙形参数化坐标写回；closure
/// 停止条件无证据，不能以距离公式冒充。
pub fn score_split_candidate(segment: &WallPathSegmentationMask, index: usize) -> f32 {
    let _ = (segment, index);
    todo!("未实现: score_split_candidate 0x140751610（extend_left/right_until closure 未取证）")
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
}
