//! 支柱放置（place_pillars.rs）。
//!
//! 原 B 源：`crates/systems/wall-constructor/src/place_pillars.rs`。
//!
//! `PillarProposals::add_proposal`（0x1407534d0，逐语句重建）：
//! 三向量结构 `{Vec<PillarPlacementProposal 0x1cB>, Vec<f32> scores,
//! Vec<u64> keys}`。key 只在新分组开始时 push；同 key 时：
//! scores 末项 = NaN-aware min(old, new)（NaN 旧值取新值）；提案末项的
//! score@+0x14 小于等于新 score 时保留旧提案，仅当新 score **严格更大**
//! 时用新 28B 记录覆盖末提案（0x1407534d0 原判据
//! `new < old || new == old → return`）。
//!
//! `assemble_stone_pillar`（0x14120d670 + closure$4 0x14120d340）：
//! - `param_3 += 0.2`、`param_4 += 0.2`（原件先加）；
//!   `param_3 ≤ 1.26` → 单段 `[0, param_4]`；否则双段
//!   `[0, param_4]` 与 `[param_4, param_3]`，且
//!   `flag = 1.2 < |param_4−param_3|`。
//! - 每段 `span = (b−a) − (flag?0.25:0)`；
//!   `count = max(3, ceil(span/0.56))`；`random_splits_into(count,
//!   0.28/span, rng)` 生成切点，映射 `t*span+a`；flag 段再追加段尾 b。
//! - 相邻切点跨度 ≥0.01 时发射：中心 = (x, (y0+y1)/2, z)，高度 = |Δy|，
//!   截面宽 = 0.42±0.05 / 0.7±0.05（flag 段末段固定 0.7/0.84）。
//! - `param_3 > 2.1` → 顶部盖砖（参数 0.72/0.5/0.85），中心 y−0.45；
//!   顶盖门控与 1.26 单段判据使用**同一个** `param_3`（bottom+0.2）。
//! - 常量：0.2 十字偏移、1.2 分段阈值、0.01 ε、0.25 间隙、2.1 高度限、
//!   0.56 行距、1.26 单段限。
use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::geometry::instanced_wall::BrickTransformSsbo;
use country_core::resources::{
    roofs::commands::{CreateRoofCmd, RecalculateWallBricks},
    walls::public_walls::{OnWallChanged, PublicWalls},
};
use itertools::Itertools;

/// 28B `PillarPlacementProposal` 观测投影（原件 stride 0x1c）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PillarPlacementProposal {
    pub bytes: [u8; 28],
}

impl PillarPlacementProposal {
    /// 原件 score 位于 +0x14。
    pub fn score(&self) -> f32 {
        f32::from_le_bytes(self.bytes[0x14..0x18].try_into().unwrap())
    }

    pub fn set_score(&mut self, score: f32) {
        self.bytes[0x14..0x18].copy_from_slice(&score.to_le_bytes());
    }
}

/// B `PillarProposals`：三向量 `{proposals, scores, keys}`。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct PillarProposals {
    pub proposals: Vec<PillarPlacementProposal>,
    pub scores: Vec<f32>,
    pub keys: Vec<u64>,
}

impl PillarProposals {
    /// B `add_proposal`（0x1407534d0，逐语句重建）。
    pub fn add_proposal(&mut self, key: u64, mut proposal: PillarPlacementProposal) {
        let last_key = self.keys.last().copied();
        if last_key == Some(key) {
            let new_score = proposal.score();
            let last_score = *self
                .scores
                .last()
                .expect("PillarProposals key without score");
            // NaN-aware min：旧值为 NaN 时取新值。
            *self.scores.last_mut().unwrap() = if last_score.is_nan() || new_score < last_score {
                new_score
            } else {
                last_score
            };
            let last = self
                .proposals
                .last_mut()
                .expect("PillarProposals key without proposal");
            // 原判据：新 score ≤ 末提案 score 时保留旧提案；仅严格更大覆盖。
            if new_score > last.score() {
                // 原件先拷贝 +0xc..+0x14、再 +0x14..+0x1c、最后 +0..+0x10。
                let mut bytes = last.bytes;
                bytes[0x0c..0x14].copy_from_slice(&proposal.bytes[0x0c..0x14]);
                bytes[0x14..0x1c].copy_from_slice(&proposal.bytes[0x14..0x1c]);
                bytes[0x00..0x10].copy_from_slice(&proposal.bytes[0x00..0x10]);
                last.bytes = bytes;
            }
            // 保持参数可读语义：score 写入位已在上面 min 逻辑处理。
            proposal.set_score(new_score);
            return;
        }
        self.scores.push(proposal.score());
        self.proposals.push(proposal);
        self.keys.push(key);
    }
}

/// 96B 记录发射上下文（closure$4 0x14120d340 的 env 字段；逐字段 U，
/// 由 place_pillars 的 SSBO/材质源提供，不能以默认值冒充业务）。
#[derive(Clone, Copy, Debug)]
pub struct PillarEmitContext {
    pub material: u32,
    pub flags: u32,
    pub wall_distance: [f32; 2],
}

/// 一个石柱砖段：世界中心 + 三轴尺寸。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StonePillarBrick {
    pub center: [f32; 3],
    pub width_x: f32,
    pub height: f32,
    pub width_z: f32,
}

impl StonePillarBrick {
    fn to_ssbo(self, seed: u32, context: PillarEmitContext) -> BrickTransformSsbo {
        BrickTransformSsbo {
            affine_columns: [
                [self.width_x, 0.0, 0.0],
                [0.0, self.height, 0.0],
                [0.0, 0.0, self.width_z],
                self.center,
            ],
            deformation: [0.0; 4],
            surface: [-100.0; 3],
            seed,
            material: context.material,
            wall_distance: context.wall_distance,
            flags: context.flags,
        }
    }
}

/// B `assemble_stone_pillar` 的分段结果（不发射，便于测试/审阅）。
/// 返回 `(bottom, top, flag)` 段表。
pub fn pillar_segments(bottom: f32, top: f32) -> Vec<(f32, f32, bool)> {
    let bottom = bottom + f32::from_bits(0x3e4ccccd); // 0.2
    let top = top + f32::from_bits(0x3e4ccccd); // 0.2
    if bottom <= f32::from_bits(0x3fa147ae) {
        // 1.26
        vec![(0.0, top, false)]
    } else {
        let delta = (top - bottom).abs();
        vec![
            (0.0, top, true),
            (top, bottom, delta > f32::from_bits(0x3f99999a)),
        ]
    }
}

/// B `assemble_stone_pillar`（0x14120d670）：按分段/随机行发射 96B 石柱砖。
///
/// 原件 `param_5 = [x, y, z]`（`y` 为柱子基高，独立于 bottom/top），
/// `param_3=bottom`、`param_4=top`。`seed` 为 fastrand 状态
/// （原件 closure env 的 rng）。
pub fn assemble_stone_pillar(
    x: f32,
    y: f32,
    z: f32,
    bottom: f32,
    top: f32,
    seed: u64,
    context: PillarEmitContext,
    out: &mut Vec<[u8; 96]>,
) {
    visit_stone_bricks(x, y, z, bottom, top, seed, |brick, ordinal| {
        out.push(brick.to_ssbo(ordinal, context).observed_bytes());
    });
}

fn visit_stone_bricks(
    x: f32, y: f32, z: f32, bottom: f32, top: f32, seed: u64,
    mut emit: impl FnMut(StonePillarBrick, u32),
) {
    let rng = fastrand::Rng::with_seed(seed);
    let base_y = y;
    let y = y + f32::from_bits(0xbe4ccccd); // 顶盖使用提前偏移的原基高。
    let segments = pillar_segments(bottom, top);
    let mut emitted: u32 = 0;
    for (a, b, flag) in segments {
        let gap = if flag {
            f32::from_bits(0x3e800000)
        } else {
            0.0
        }; // 0.25
        let span = (b - a) - gap;
        // 原 14120d670 使用 ceilf，随后最小值为 3。
        let count = {
            let rounded = (span / f32::from_bits(0x3f0f5c29)).ceil(); // 0.56
            let saturated = if rounded < 0.0 {
                0usize
            } else if rounded > u64::MAX as f32 {
                usize::MAX
            } else {
                rounded as usize
            };
            if saturated < 4 { 3 } else { saturated }
        };
        let mut splits = Vec::new();
        country_core::systems::wall::construction::bricks::random_splits_into(
            count,
            f32::from_bits(0x3e8f5c29) / span, // 0.28 / span
            &rng,
            &mut splits,
        );
        let mut heights: Vec<f32> = splits.into_iter().map(|t| t * span + a).collect();
        if flag {
            // 原件追加段尾（0.25 间隙保留在最后一对里）。
            heights.push(b);
        }
        for (index, (low, high)) in heights.iter().tuple_windows().enumerate() {
            let (low, high) = (*low, *high);
            let low_y = low + base_y - 0.2;
            let high_y = high + base_y - 0.2;
            let delta_y = high_y - low_y;
            let length = (delta_y * delta_y + 0.0 + 0.0).sqrt();
            if length < f32::from_bits(0x3c23d70a) {
                // 0.01 ε
                continue;
            }
            let (width_x, width_z) = if !flag {
                jitter_widths(&rng)
            } else if index == heights.len() - 2 {
                // 原件的“末段固定 0.7/0.84”判据：lVar20 == len-2。
                (f32::from_bits(0x3f333333), f32::from_bits(0x3f570a3e))
            } else {
                jitter_widths(&rng)
            };
            let brick = StonePillarBrick {
                // 原先分别把两个端点加基高、减0.2，再求中点。
                center: [x, (low_y + high_y) * 0.5, z],
                width_x,
                height: length,
                width_z,
            };
            emit(brick, emitted);
            emitted += 1;
        }
    }
    // 顶部盖砖：原件 `2.1 < param_3`（param_3 = bottom + 0.2），与 1.26
    // 单段判据共用同一参数。
    let adjusted_bottom = bottom + f32::from_bits(0x3e4ccccd);
    if adjusted_bottom > f32::from_bits(0x40066666) {
        let cap = StonePillarBrick {
            center: [x, y - f32::from_bits(0x3e800000), z],
            width_x: f32::from_bits(0x3f3851ec), // 0.72
            height: f32::from_bits(0x3f000000),  // 0.5
            width_z: f32::from_bits(0x3f59999a), // 0.85
        };
        emit(cap, emitted);
    }
}

/// 原件 jitter：`(rand*0.05 − (1−rand)*0.05) + base`。
/// width_x base 0.42（bits3ed70a3e），width_z base 0.7（bits3f333333）。
fn jitter_widths(rng: &fastrand::Rng) -> (f32, f32) {
    let a = rng.f32();
    let b = rng.f32();
    let width_x = (a * f32::from_bits(0x3d4ccccd) - (1.0 - a) * f32::from_bits(0x3d4ccccd))
        + f32::from_bits(0x3ed70a3e);
    let width_z = (b * f32::from_bits(0x3d4ccccd) - (1.0 - b) * f32::from_bits(0x3d4ccccd))
        + f32::from_bits(0x3f333333);
    (width_x, width_z)
}

/// 原 1407541xx：按 LoopRange 的起点及模数映射两个准备路径端点，
/// 再逐分量使用 `(next-current)*fraction+current`；非法模数或索引保留失败。
fn sample_pillar_position(
    points: &[[f32; 3]],
    range: country_core::systems::wall::construction::segmentation::WallLoopRange,
    coord: country_core::systems::wall::construction::segmentation::ThresholdedCurveCoord,
) -> [f32; 3] {
    let index = (coord.point_index as i32).wrapping_add(range.start);
    let first = index.rem_euclid(range.modulus) as usize;
    let second = index.wrapping_add(1).rem_euclid(range.modulus) as usize;
    let a = points[first];
    let b = points[second];
    std::array::from_fn(|axis| (b[axis] - a[axis]) * coord.fraction + a[axis])
}

/// 原自由墙端点按 0.1875 延伸至墙内；非端点保持原采样坐标。
fn adjust_freehand_pillar_coord(
    points: &[[f32; 3]],
    range: country_core::systems::wall::construction::segmentation::WallLoopRange,
    key: usize,
    flag: u8,
    coord: country_core::systems::wall::construction::segmentation::ThresholdedCurveCoord,
) -> country_core::systems::wall::construction::segmentation::ThresholdedCurveCoord {
    use utils::curve::{CurveCoordSemanticThresholded3D, CurveInternalCoord,
        curve_slice::{CurvePoints, CurveSlice, LoopCurvePoints}};
    use country_core::systems::wall::construction::segmentation::ThresholdedCurveCoord;
    // 140bece40 读取 LoopIdx 的原始有符号值；这里不取模。
    let first = range.start == 0 && flag & 2 != 0;
    let end = i64::from(range.end) == points.len() as i64;
    if !first && !(end && flag & 4 != 0) { return coord; }
    let length = range.len();
    assert!(key < length, "原柱候选超出 LoopRange");
    let start = CurveInternalCoord::new_for_point_count(key, 0.0, length);
    let loop_range = utils::loop_range::LoopRange { start_idx: range.start, end_idx: range.end, period: range.modulus };
    let view = LoopCurvePoints { points, range: loop_range };
    let result = if first {
        let mut slice = CurveSlice::<[f32; 3], _, CurveCoordSemanticThresholded3D, glam::Vec2, _>::from_coord_range_with_remap(
            view, start, start, |_, p: &[f32; 3]| glam::Vec2::new(p[0], p[2]));
        slice.extend_right(0.1875);
        slice.end()
    } else {
        // B141206c50 uses f32 throughout and deliberately evaluates
        // ((t - 1) + 1) before multiplying the first segment length.
        assert!(start.segment < length.saturating_sub(1), "segment outside point set");
        let mut result = start;
        let mut residual = 0.0f32;
        let mut initial = true;
        for upper in (1..=start.segment + 1).rev() {
            let a = view.point(upper - 1);
            let b = view.point(upper);
            let dx = b[0] - a[0];
            let dz = b[2] - a[2];
            let length = (dz * dz + dx * dx).sqrt();
            residual = if initial { ((start.t - 1.0) + 1.0) * length - 0.1875 }
                else { residual + length };
            initial = false;
            result = CurveInternalCoord { segment: upper - 1, t: 0.0 };
            if residual >= 0.0 {
                result.t = residual / length;
                break;
            }
        }
        result
    };
    ThresholdedCurveCoord { point_index: result.segment as u32, fraction: result.t }
}

/// 原 0x88 段的三个缓存按最短长度 zip；key 同时索引 flags 与 coords。
/// 自由墙端点需先执行原 CurveSlice 延伸，由调用者提供调整后的坐标。
#[derive(Clone, Copy, Debug)]
struct SegmentPillar {
    proposal: PillarPlacementProposal,
    score: f32,
    position: [f32; 3],
    key: usize,
}

/// 原 1407542xx–6a3 的柱木记录；两条水平轴由提案方向与 Y 叉积给出。
fn wooden_pillar_record(
    pillar: &SegmentPillar,
    wall: u64,
    segment_ordinal: usize,
    pillar_ordinal: usize,
) -> country_core::resources::roofs::roof_buffers::WoodenPlankSsbo {
    use country_core::resources::{render::Affine3Packed, roofs::roof_buffers::WoodenPlankSsbo};
    let read = |offset| f32::from_le_bytes(pillar.proposal.bytes[offset..offset + 4].try_into().unwrap());
    let x = read(0);
    let z = read(4);
    let height = (read(0x14) + 0.2) + 0.1;
    // 保留原叉积及零乘/加顺序，负零与非有限方向均会影响结果位型。
    let direction = glam::Vec3::new(x, 0.0, z);
    let cross = glam::Vec3::new(z * 1.0 - 0.0 * 0.0,
        x * 0.0 - z * 0.0, 0.0 * 0.0 - x * 1.0);
    let zero = glam::Vec3::ZERO;
    let cross_zero = cross * 0.0;
    let direction_zero = direction * 0.0;
    let position = glam::Vec3::new(pillar.position[0], pillar.position[1] - 0.2, pillar.position[2]);
    WoodenPlankSsbo {
        transform: Affine3Packed::from_cols(
            (cross * 0.375 + zero) + direction_zero,
            (glam::Vec3::Y * height + cross_zero) + direction_zero,
            direction * 0.375 + (cross_zero + zero),
            (cross_zero + direction_zero) + position,
        ),
        gate_seed: segment_ordinal as i32,
        piece_seed: pillar_ordinal as i32,
        wall_id: wall as u32,
        flags: 0,
    }
}

/// 逐柱石砖候选：原闭包以柱方向定向砖轴，沿墙参数由提案 u 与发布曲线长度计算。
/// 原输入/记录的数值等价仍须用 14120d670→14120d340 原输出逐字节验证。
fn visit_stone_pillar_records(
    pillar: &SegmentPillar,
    wall: country_core::resources::walls::WallId,
    curve_length: f32,
    mut emit: impl FnMut(BrickTransformSsbo),
) {
    use country_core::geometry::instanced_wall::BrickSourceId;
    let read = |offset| f32::from_le_bytes(pillar.proposal.bytes[offset..offset + 4].try_into().unwrap());
    let distance = read(0x18) * curve_length;
    let context = PillarEmitContext { material: BrickSourceId::from_wall_id(
        country_core::resources::walls::publication_fields::WallId(wall.0)).0, flags: 0,
        wall_distance: [distance + f32::from_bits(0xbe6b851f), distance + f32::from_bits(0x3e6b851f)] };
    let x = read(0);
    let z = read(4);
    let inv = 1.0 / (x * x + z * z).sqrt();
    let (x, z) = (x * inv, z * inv);
    visit_stone_bricks(pillar.position[0], pillar.position[1], pillar.position[2],
        read(0x14), pillar.score, wall.0.wrapping_add(pillar.key as u64), |brick, ordinal| {
            let mut record = brick.to_ssbo(ordinal, context);
            record.affine_columns = [[x * brick.width_x, 0.0, z * brick.width_x], [0.0, brick.height, 0.0],
                [(-z * brick.width_z) + 0.0, 0.0, x * brick.width_z], brick.center];
            emit(record);
        });
}

#[cfg(test)]
fn stone_pillar_records(pillar: &SegmentPillar, wall: country_core::resources::walls::WallId,
    curve_length: f32) -> Vec<BrickTransformSsbo> {
    let mut out = Vec::new();
    visit_stone_pillar_records(pillar, wall, curve_length, |record| out.push(record));
    out
}

fn segment_pillars<'a>(
    mask: &'a country_core::systems::wall::construction::segmentation::WallPathSegmentationMask,
    points: Option<&'a [[f32; 3]]>,
    mut adjust: impl FnMut(usize, u8, country_core::systems::wall::construction::segmentation::ThresholdedCurveCoord)
        -> country_core::systems::wall::construction::segmentation::ThresholdedCurveCoord + 'a,
) -> impl Iterator<Item = SegmentPillar> + 'a {
    mask.pillar_proposals.iter().zip(&mask.pillar_scores).zip(&mask.pillar_keys)
        .map(move |((proposal, &score), &key)| {
            let flag = mask.flags[key];
            let coord = adjust(key, flag, mask.coords[key]);
            SegmentPillar { proposal: PillarPlacementProposal { bytes: *proposal }, score,
                position: sample_pillar_position(points.expect("非空柱提案需要已准备墙路径"), mask.range, coord), key }
        })
}

/// B 0x140753670：消费事件与上游分段柱缓存，写同墙拱/过梁 owner。
/// 三种缓存由原分段生产者写入；通用砖／木实例槽与高程支撑独立。
pub fn place_pillars(
    mut changed: EventReader<OnWallChanged>,
    mut bricks: EventReader<RecalculateWallBricks>,
    mut create_roof: EventReader<CreateRoofCmd>,
    mut walls: ResMut<PublicWalls>,
    mut ssbo: ResMut<AssetSsboLibrary>,
    masks: Res<crate::construct_lintels::WallPathSegmentationMasksMinusStairs>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
    mut trace_frame: Local<u64>,
) {
    *trace_frame = trace_frame.wrapping_add(1);
    let changed_ids: Vec<_> = changed.read().map(|e| e.0.0).collect();
    let brick_ids: Vec<_> = bricks.read().map(|e| e.0.0).collect();
    let roof_ids: Vec<_> = create_roof.read().map(|e| e.0.wall_id().0).collect();
    if std::env::var_os("TG_M6_PILLAR_TRACE").is_some() {
        for &wall in changed_ids.iter().chain(&brick_ids).chain(&roof_ids) {
            if walls.get(country_core::resources::walls::WallId(wall)).is_none() {
                eprintln!("M6_PILLAR_MISSING frame={} wall={} changed={} bricks={} create_roof={} published_delta_changed={:?} published_delta_removed={:?}",
                    *trace_frame, wall, changed_ids.contains(&wall), brick_ids.contains(&wall), roof_ids.contains(&wall),
                    walls.last_delta.changed, walls.last_delta.removed);
            }
        }
    }
    let mut recalc: Vec<u64> = changed_ids.into_iter().chain(brick_ids).chain(roof_ids).collect();
    // 原 140753cxx 将三组事件的墙 ID 合并后无符号排序、去重。
    recalc.sort_unstable();
    recalc.dedup();
    let mut brick_exhausted = false;
    let mut plank_exhausted = false;
    for wall_id in recalc {
        let id = country_core::resources::walls::WallId(wall_id);
        let wall = walls.get_mut(id).expect("柱放置事件必须引用已发布墙");
        let projection = wall.building_projection();
        let freehand = projection.shape().tag() == 2;
        let material_tag = projection.observed_296();
        let wood = material_tag == 1;
        let curve_length = projection.published_source_curve_length();
        let points = wall.prepared_path.as_ref().map(|path| path.positions.as_slice());
        let segments = masks.masks.get(&wall_id).expect("柱放置需要处理后的墙分段");
        let mut positions = Vec::new();
        let mut count = 0u32;
        for (segment_index, mask) in segments.iter().enumerate() {
            let pillars = segment_pillars(mask, points, move |key, flag, coord| {
                if freehand { adjust_freehand_pillar_coord(points.expect("自由墙柱提案需要已准备墙路径"), mask.range, key, flag, coord) } else { coord }
            });
            for (pillar_index, pillar) in pillars.enumerate() {
                positions.push([pillar.position[0], pillar.position[2]]);
                count = count.wrapping_add(1);
                if wood {
                    let storage = ssbo.get_mut::<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>()
                        .expect("原柱木 SSBO 缺失");
                    let mut writer = output.gpu.wall_plank_writer(storage, wall_id);
                    plank_exhausted |= writer.push(&wooden_pillar_record(&pillar, wall_id, segment_index, pillar_index)).is_err();
                } else {
                    let storage = ssbo.get_mut::<BrickTransformSsbo>().expect("原柱砖 SSBO 缺失");
                    let mut writer = output.gpu.wall_brick_writer(storage, wall_id);
                    visit_stone_pillar_records(&pillar, id, curve_length, |record| {
                        brick_exhausted |= writer.push(&record).is_err();
                    });
                }
            }
        }
        // 14075432d selects wood only for byte == 1; 14075492a selects
        // lintel statistics for every nonzero byte.
        if material_tag != 0 { wall.material_stats.set_lintel_pillars_count(count); }
        else { wall.material_stats.set_arch_pillars_count(count); }
        wall.material_stats.set_pillar_placements(positions);
    }
    if brick_exhausted { log::error!("原柱砖分配容量不足"); }
    if plank_exhausted { log::error!("原柱木分配容量不足"); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pillar_position_uses_loop_range_and_interpolates_in_original_order() {
        use country_core::systems::wall::construction::segmentation::{ThresholdedCurveCoord, WallLoopRange};
        let points = [[0., 1., 2.], [4., 5., 6.], [8., 9., 10.]];
        let range = WallLoopRange { start: -1, end: 2, modulus: 3 };
        assert_eq!(sample_pillar_position(&points, range, ThresholdedCurveCoord { point_index: 0, fraction: 0.25 }), [6., 7., 8.]);
    }

    #[test]
    fn segment_pillars_uses_three_cache_lengths_and_indexed_coordinates() {
        use country_core::systems::wall::construction::segmentation::{ThresholdedCurveCoord, WallLoopRange, WallPathSegmentationMask};
        let mask = WallPathSegmentationMask { flags: vec![0, 7],
            coords: vec![ThresholdedCurveCoord { point_index: 0, fraction: 0.0 },
                ThresholdedCurveCoord { point_index: 1, fraction: 0.5 }],
            pillar_proposals: vec![[1; 28], [2; 28]], pillar_scores: vec![3.0], pillar_keys: vec![1, 0],
            range: WallLoopRange { start: 0, end: 3, modulus: 3 }, length: 8.0 };
        let points = [[0., 0., 0.], [4., 2., 0.], [8., 4., 0.]];
        let result: Vec<_> = segment_pillars(&mask, Some(&points), |key, flag, coord| {
            assert_eq!((key, flag), (1, 7)); coord
        }).collect();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].proposal.bytes, [1; 28]);
        assert_eq!((result[0].score, result[0].key, result[0].position), (3.0, 1, [6., 3., 0.]));
    }

    #[test]
    fn wooden_pillar_keeps_original_identity_and_axis_mapping() {
        let mut proposal = PillarPlacementProposal { bytes: [0; 28] };
        proposal.bytes[0..4].copy_from_slice(&1.0f32.to_le_bytes());
        proposal.bytes[0x14..0x18].copy_from_slice(&2.0f32.to_le_bytes());
        let pillar = SegmentPillar { proposal, score: 1.0, position: [4.0, 3.0, 5.0], key: 2 };
        let record = wooden_pillar_record(&pillar, 42, 3, 7);
        assert_eq!((record.gate_seed, record.piece_seed, record.wall_id, record.flags), (3, 7, 42, 0));
        assert_eq!(record.transform, country_core::resources::render::Affine3Packed::from_cols(
            glam::Vec3::new(0.0, 0.0, -0.375), glam::Vec3::new(0.0, (2.0f32 + 0.2) + 0.1, 0.0),
            glam::Vec3::new(0.375, 0.0, 0.0), glam::Vec3::new(4.0, 2.8, 5.0)));
    }

    #[test]
    fn wooden_pillar_original_six_records_match() {
        let inputs = [([1., 0.], [4., 5., 6.], 2.), ([0., 1.], [4., 5., 6.], 2.),
            ([0.6, 0.8], [4., 5., 6.], 2.), ([-0.0, 1.], [-0.0, 0., -0.0], 0.),
            ([1., -0.0], [0., -0.0, 0.], -0.2), ([-1., 0.], [-1., 1., -1.], 1.)];
        let fixture = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-place-wood-oracle-2026-10-29.jsonl"));
        for (case, ((direction, position, height), line)) in inputs.into_iter().zip(fixture.lines()).enumerate() {
            let hex = line.split("\"original\": \"").nth(1).unwrap().split('"').next().unwrap();
            let expected: Vec<u8> = (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i+2], 16).unwrap()).collect();
            let mut proposal = PillarPlacementProposal { bytes: [0; 28] };
            for (offset, value) in [(0, direction[0]), (4, direction[1]), (0x14, height)] {
                proposal.bytes[offset..offset+4].copy_from_slice(&f32::to_le_bytes(value));
            }
            let pillar = SegmentPillar { proposal, score: 0., position, key: 0 };
            let record = wooden_pillar_record(&pillar, 123, 2, 7);
            let mut actual = Vec::new();
            for column in record.transform.cols { for value in column.to_array() { actual.extend(value.to_le_bytes()); } }
            for value in [record.gate_seed as u32, record.piece_seed as u32, record.wall_id, record.flags] { actual.extend(value.to_le_bytes()); }
            assert_eq!(actual, expected, "原木柱完整记录不一致，样本 {case}");
        }
    }

    #[test]
    fn stone_pillar_first_original_oracle_matches_full_records() {
        let line = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-place-stone-oracle-2026-10-29.jsonl"))
            .lines().next().unwrap();
        let list = line.split("\"bricks\": [").nth(1).unwrap().split(']').next().unwrap();
        let expected: Vec<Vec<u8>> = list.split(',').map(|s| {
            let hex = s.trim().trim_matches('"');
            (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect()
        }).collect();
        let mut proposal = PillarPlacementProposal { bytes: [0; 28] };
        for (offset, value) in [(0, 1.0f32), (0x14, 1.0), (0x18, 0.25)] {
            proposal.bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        let pillar = SegmentPillar { proposal, score: 2.0, position: [4.0, 5.0, 6.0], key: 0 };
        let actual = stone_pillar_records(&pillar, country_core::resources::walls::WallId(123), 10.0);
        assert_eq!(actual.iter().map(|record| record.observed_bytes().to_vec()).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn stone_pillar_all_original_oracle_records_match() {
        let fixture = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-place-stone-oracle-2026-10-29.jsonl"));
        for (case, line) in fixture.lines().enumerate() {
            let input = line.split("\"input\": ").nth(1).unwrap().split(", \"original\"").next().unwrap();
            let values: Vec<_> = input.trim_matches(|c| c == '[' || c == ']').split(',').map(str::trim).collect();
            let bottom: f32 = values[0].parse().unwrap();
            let top: f32 = values[1].parse().unwrap();
            let seed: u64 = values[2].parse().unwrap();
            let dx: f32 = values[3].trim_start_matches('[').parse().unwrap();
            let dz: f32 = values[4].trim_end_matches(']').parse().unwrap();
            let list = line.split("\"bricks\": [").nth(1).unwrap().split(']').next().unwrap();
            let expected: Vec<Vec<u8>> = if list.trim().is_empty() { Vec::new() } else { list.split(',').map(|s| {
                let hex = s.trim().trim_matches('"');
                (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect()
            }).collect() };
            let mut proposal = PillarPlacementProposal { bytes: [0; 28] };
            for (offset, value) in [(0, dx), (4, dz), (0x14, bottom), (0x18, 0.25)] {
                proposal.bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
            }
            let pillar = SegmentPillar { proposal, score: top, position: [4.0, 5.0, 6.0],
                key: seed.wrapping_sub(123) as usize };
            let actual = stone_pillar_records(&pillar, country_core::resources::walls::WallId(123), 10.0);
            assert_eq!(actual.iter().map(|record| record.observed_bytes().to_vec()).collect::<Vec<_>>(), expected,
                "原柱完整记录不一致，样本 {case}");
        }
    }

    #[test]
    fn actual_lintel_producer_feeds_place_pillars_and_material_stats() {
        use country_core::{
            render::{sparse_instance_buffer::SparseInstanceBuffer, ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice}},
            resources::{roofs::{geometry::surface::Curve2, roof_buffers::WoodenPlankSsbo},
                walls::{InnerWalls, arches::LintelSegments, publication_source::TerrainHeightGrid512},
                world_raster::DirtyTilePublisher,
                world_raster_defs::{PathRaster, Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster}},
            systems::wall::{construction::segmentation::{wall_path_segmentation, WallSegmentationInput},
                registration::register_wall_cpu_resources,
                wall_state::publish_public_wall_state::{publish_public_wall_state, WallPublicationTerrain}},
        };
        struct CpuDevice;
        impl GraphicsDevice for CpuDevice {
            fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
                Ok(Buffer { id: 812, byte_size: desc.size as u64, ..Default::default() })
            }
            fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> {
                Err(BackendError::invalid_usage("此测试不提供 GPU 上传"))
            }
        }
        let mut world = World::new();
        register_wall_cpu_resources(&mut world);
        world.init_resource::<Events<CreateRoofCmd>>();
        world.insert_resource(WallPublicationTerrain(TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false }));
        let mut path = PathRaster::new(DirtyTilePublisher::unique());
        path.write_composite(Some(&vec![255;512*512]));
        world.insert_resource(path);
        world.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
        world.init_resource::<LintelSegments>();
        world.init_resource::<crate::construct_lintels::WallPathSegmentationMasksMinusStairs>();
        world.init_resource::<crate::construct_arches::WallConstructorOutput>();
        let library = AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::<WoodenPlankSsbo>::new_storage_buffer(&CpuDevice, None));
        library.register::<BrickTransformSsbo>(SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&CpuDevice, None));
        world.insert_resource(library);
        let id = world.resource_mut::<InnerWalls>().new_wall_w_params(
            Curve2::try_new(vec![[-6.,0.],[-1.,0.],[3.,2.],[7.,2.]], false).unwrap(), 1.2, 0, 1, false);
        let unusual_id = world.resource_mut::<InnerWalls>().new_wall_w_params(
            Curve2::try_new(vec![[-6.,10.],[-1.,10.],[3.,12.],[7.,12.]], false).unwrap(), 1.2, 0, 2, false);
        let mut publication = Schedule::default(); publication.add_systems(publish_public_wall_state); publication.run(&mut world);
        let mut owners = Schedule::default(); owners.add_systems(crate::gpu_output::replace_published_plank_owners); owners.run(&mut world);
        let masks = {
            let walls = world.resource::<PublicWalls>();
            let wall = walls.get(id).unwrap();
            let path = wall.prepared_path.as_ref().unwrap();
            wall_path_segmentation(WallSegmentationInput { prepared_positions: &path.positions,
                path_values: &path.segment_values, shape: wall.shape(), observed_special_flag: wall.authority.source_flag_65() }).unwrap()
        };
        world.resource_mut::<crate::construct_lintels::WallPathSegmentationMasksMinusStairs>().masks.insert(id.0, masks);
        let unusual_masks = {
            let walls = world.resource::<PublicWalls>();
            let wall = walls.get(unusual_id).unwrap();
            let path = wall.prepared_path.as_ref().unwrap();
            wall_path_segmentation(WallSegmentationInput { prepared_positions: &path.positions,
                path_values: &path.segment_values, shape: wall.shape(), observed_special_flag: wall.authority.source_flag_65() }).unwrap()
        };
        world.resource_mut::<crate::construct_lintels::WallPathSegmentationMasksMinusStairs>().masks.insert(unusual_id.0, unusual_masks);
        let mut source = Schedule::default();
        source.add_systems((crate::gpu_output::clear_constructor_planks, crate::construct_lintels::construct_lintels).chain());
        source.run(&mut world);
        // The real publisher owns the shared brick allocation; arches clear it
        // before place_pillars appends, including for material byte 2.
        world.resource_scope(|world, mut output: Mut<crate::construct_arches::WallConstructorOutput>| {
            let mut library = world.resource_mut::<AssetSsboLibrary>();
            let storage = library.get_mut::<BrickTransformSsbo>().unwrap();
            output.gpu.clear_bricks(storage, unusual_id.0);
        });
        let mask = &world.resource::<crate::construct_lintels::WallPathSegmentationMasksMinusStairs>().masks[&id.0];
        let expected: usize = mask.iter().map(|m| m.pillar_proposals.len()).sum();
        assert!(expected > 0, "真实过梁必须生成非空柱提案");
        let unusual_expected: usize = world.resource::<crate::construct_lintels::WallPathSegmentationMasksMinusStairs>()
            .masks[&unusual_id.0].iter().map(|m| m.pillar_proposals.len()).sum();
        assert!(unusual_expected > 0, "非零材质字节必须有真实柱提案");
        let brick_before: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<BrickTransformSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, n)| *n).sum();
        let before: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<WoodenPlankSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, n)| *n).sum();
        let mut place = Schedule::default(); place.add_systems(place_pillars); place.run(&mut world);
        let wall = world.resource::<PublicWalls>();
        let stats = &wall.get(id).unwrap().material_stats;
        assert_eq!(stats.pillar_placements.len(), expected);
        assert_eq!(stats.counts[2] as usize, expected);
        let unusual_stats = &wall.get(unusual_id).unwrap().material_stats;
        assert_eq!(unusual_stats.counts[2] as usize, unusual_expected);
        assert_eq!(unusual_stats.counts[4], 0);
        let brick_after: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<BrickTransformSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, n)| *n).sum();
        assert!(brick_after > brick_before, "字节2必须输出石柱砖");
        let after: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<WoodenPlankSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, n)| *n).sum();
        assert_eq!(after - before, expected as u32);
        place.run(&mut world);
        let quiet: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<WoodenPlankSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, n)| *n).sum();
        assert_eq!(quiet, after, "无新事件不得重复写柱");
        // 实际洞口最终化 producer 可单独发 RecalculateWallBricks；记录当前
        // 重算事件与 owner 的可观察行为，不推断原版有额外清理系统。
        world.resource_mut::<country_core::resources::walls::wall_holes::WallHoles>().add(id,
            country_core::resources::walls::wall_holes::RawWallHole { center: [1., 1.], dimensions: [0.5, 0.5], kind: 0 }).unwrap();
        let mut finalize = Schedule::default();
        finalize.add_systems(country_core::systems::wall::construct_wall_bricks::finalize_wall_holes);
        finalize.run(&mut world);
        place.run(&mut world);
        let quiet: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<WoodenPlankSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, n)| *n).sum();
        assert_eq!(quiet - after, expected as u32, "单独的真实洞口重算会追加柱记录");
        let old_placements = world.resource::<PublicWalls>().get(id).unwrap().material_stats.pillar_placements.clone();
        {
            let mut masks = world.resource_mut::<crate::construct_lintels::WallPathSegmentationMasksMinusStairs>();
            let mask = masks.masks.get_mut(&id.0).unwrap().iter_mut().find(|mask| !mask.pillar_proposals.is_empty()).unwrap();
            mask.pillar_proposals.push(mask.pillar_proposals[0]);
            mask.pillar_scores.push(mask.pillar_scores[0]);
            mask.pillar_keys.push(usize::MAX);
        }
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(id));
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| place.run(&mut world))).is_err());
        let partial: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<WoodenPlankSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, n)| *n).sum();
        assert!(partial > quiet, "后续索引错误必须保留已写柱前缀");
        assert_eq!(world.resource::<PublicWalls>().get(id).unwrap().material_stats.pillar_placements, old_placements,
            "该墙未完成时不得提交新统计");
    }

    #[test]
    fn freehand_endpoint_moves_inward_but_nonendpoint_does_not() {
        use country_core::systems::wall::construction::segmentation::{ThresholdedCurveCoord, WallLoopRange};
        let points = [[0., 0., 0.], [4., 0., 0.], [8., 0., 0.]];
        let range = WallLoopRange { start: 0, end: 3, modulus: 3 };
        let start = ThresholdedCurveCoord { point_index: 0, fraction: 0.0 };
        let adjusted = adjust_freehand_pillar_coord(&points, range, 0, 2, start);
        assert_eq!(sample_pillar_position(&points, range, adjusted), [0.1875, 0., 0.]);
        assert_eq!(adjust_freehand_pillar_coord(&points, range, 0, 0, start), start);
        let end = ThresholdedCurveCoord { point_index: 1, fraction: 1.0 };
        let adjusted = adjust_freehand_pillar_coord(&points, range, 2, 4, end);
        assert_eq!(sample_pillar_position(&points, range, adjusted), [7.8125, 0., 0.]);
    }

    #[test]
    fn freehand_original_five_endpoint_samples_match() {
        use country_core::systems::wall::construction::segmentation::{ThresholdedCurveCoord, WallLoopRange};
        let fixture = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-place-freehand-oracle-2026-10-29.jsonl"));
        let points = [[0., 0., 0.], [4., 0., 0.], [8., 0., 0.]];
        let inputs = [(4,0,2,0,0.), (4,2,4,1,1.), (4,0,0,0,0.), (3,2,4,1,1.), (4,1,2,1,0.5)];
        for (case, ((period, key, flag, segment, fraction), line)) in inputs.into_iter().zip(fixture.lines()).enumerate() {
            let expected = line.split("\"original\": \"").nth(1).unwrap().split('"').next().unwrap();
            let range = WallLoopRange { start: 0, end: 3, modulus: period };
            let coord = adjust_freehand_pillar_coord(&points, range, key, flag,
                ThresholdedCurveCoord { point_index: segment, fraction });
            let actual: String = sample_pillar_position(&points, range, coord).into_iter()
                .flat_map(f32::to_le_bytes).map(|byte| format!("{byte:02x}")).collect();
            assert_eq!(actual, expected, "原自由墙柱坐标不一致，样本 {case}");
        }
    }

    #[test]
    fn freehand_original_short_and_raw_range_endpoints_match() {
        use country_core::systems::wall::construction::segmentation::{ThresholdedCurveCoord, WallLoopRange};
        let points = [[0., 0., 0.], [0.1, 0., 0.]];
        for (start, end, key, flag, segment, fraction, expected) in [
            (0,2,0,2,0,0., 0.1f32), (0,2,1,4,0,1., 0.),
            (1,2,0,2,0,0., 0.1), (-2,0,0,2,0,0., 0.),
        ] {
            let range = WallLoopRange { start, end, modulus: 2 };
            let coord = adjust_freehand_pillar_coord(&points, range, key, flag,
                ThresholdedCurveCoord { point_index: segment, fraction });
            assert_eq!(sample_pillar_position(&points, range, coord)[0].to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn freehand_end_uses_native_f32_residual_for_infinite_segment() {
        use country_core::systems::wall::construction::segmentation::{ThresholdedCurveCoord, WallLoopRange};
        let points = [[0., 0., 0.], [f32::INFINITY, 0., 0.]];
        let range = WallLoopRange { start: 0, end: 2, modulus: 2 };
        let result = adjust_freehand_pillar_coord(&points, range, 1, 4,
            ThresholdedCurveCoord { point_index: 0, fraction: 1.0 });
        // B141206dcf..e7b computes (infinity - 0.1875) / infinity.
        assert!(result.fraction.is_nan());
    }

    fn proposal(score: f32, marker: u8) -> PillarPlacementProposal {
        let mut p = PillarPlacementProposal { bytes: [0u8; 28] };
        p.set_score(score);
        p.bytes[0] = marker;
        p
    }

    #[test]
    fn add_proposal_pushes_new_key_and_replaces_on_strictly_larger_score() {
        let mut proposals = PillarProposals::default();
        proposals.add_proposal(7, proposal(2.0, 1));
        proposals.add_proposal(7, proposal(3.0, 2));
        assert_eq!(proposals.keys, vec![7]);
        assert_eq!(proposals.proposals.len(), 1);
        // scores 取 NaN-aware min；提案在新 score 严格更大时覆盖。
        assert_eq!(proposals.scores, vec![2.0]);
        assert_eq!(proposals.proposals[0].score(), 3.0);
        assert_eq!(proposals.proposals[0].bytes[0], 2);
        // 新 score 1.0 ≤ 3.0 → 保留旧提案；scores 仍取 min。
        proposals.add_proposal(7, proposal(1.0, 3));
        assert_eq!(proposals.scores, vec![1.0]);
        assert_eq!(proposals.proposals[0].score(), 3.0);
        assert_eq!(proposals.proposals[0].bytes[0], 2);
        // 新 key 追加分组。
        proposals.add_proposal(9, proposal(5.0, 4));
        assert_eq!(proposals.keys, vec![7, 9]);
    }

    #[test]
    fn pillar_segments_split_at_126_and_flag_large_delta() {
        assert_eq!(pillar_segments(1.0, 1.0).len(), 1);
        let segs = pillar_segments(3.0, 1.1);
        assert_eq!(segs.len(), 2);
        assert_eq!(segs[0].0, 0.0);
        assert!((segs[0].1 - 1.3).abs() < 1e-6);
        assert!(segs[0].2);
        assert!(segs[1].2, "|Δ| = 1.9 > 1.2");
        let segs = pillar_segments(2.0, 1.1);
        assert!(!segs[1].2, "|Δ| = 0.9 <= 1.2");
    }

    #[test]
    fn assemble_emits_bricks_and_top_cap() {
        let context = PillarEmitContext {
            material: 3,
            flags: 0,
            wall_distance: [0.0, 0.0],
        };
        let mut out = Vec::new();
        // bottom+0.2 = 2.2 > 2.1 → 顶盖门控（与 1.26 判据同一 param_3）。
        assemble_stone_pillar(1.0, 0.5, 2.0, 2.0, 3.0, 42, context, &mut out);
        assert!(!out.is_empty());
        for brick in &out {
            assert_eq!(u32::from_le_bytes(brick[80..84].try_into().unwrap()), 3);
            // surface 默认哨兵 −100。
            assert_eq!(
                f32::from_le_bytes(brick[64..68].try_into().unwrap()),
                -100.0
            );
        }
        // 盖砖 width_x = 0.72；至少存在一块。
        assert!(out.iter().any(|brick| {
            (f32::from_le_bytes(brick[0..4].try_into().unwrap()) - 0.72).abs() < 1e-6
        }));
    }

    #[test]
    fn assemble_single_segment_has_no_gap() {
        let context = PillarEmitContext {
            material: 0,
            flags: 0,
            wall_distance: [0.0, 0.0],
        };
        let mut out = Vec::new();
        assemble_stone_pillar(0.0, 0.0, 0.0, 0.0, 1.0, 1, context, &mut out);
        // bottom+0.2=0.2 ≤1.26 → 单段，span=1.2，count=round(2.14)=2 → max(3)=3。
        assert_eq!(out.len(), 2, "3 splits → 2 spans ≥0.01");
    }
}
