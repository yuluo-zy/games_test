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
//!   `[0, param_3]` 与 `[param_3, param_4]`，且
//!   `flag = 1.2 < |param_4−param_3|`。
//! - 每段 `span = (b−a) − (flag?0.25:0)`；
//!   `count = max(3, round(span/0.56))`；`random_splits_into(count,
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
            (0.0, bottom, true),
            (bottom, top, delta > f32::from_bits(0x3f99999a)),
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
    let rng = fastrand::Rng::with_seed(seed);
    let y = y + f32::from_bits(0xbe4ccccd); // −0.2：原件 fVar33 = param_5[1] − 0.2
    let segments = pillar_segments(bottom, top);
    let mut emitted: u32 = 0;
    for (a, b, flag) in segments {
        let gap = if flag {
            f32::from_bits(0x3e800000)
        } else {
            0.0
        }; // 0.25
        let span = (b - a) - gap;
        if !(span > 0.0) {
            continue;
        }
        // count = max(3, round(span / 0.56))：原件 `if (uVar16 < 4) uVar16 = 3`。
        let count = {
            let rounded = (span / f32::from_bits(0x3f0f5c29)).round(); // 0.56
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
        for pair in heights.windows(2) {
            let (low, high) = (pair[0], pair[1]);
            let length = (high - low).abs();
            if length < f32::from_bits(0x3c23d70a) {
                // 0.01 ε
                continue;
            }
            let (width_x, width_z) = if !flag {
                jitter_widths(&rng)
            } else if length_is_last_pair(&heights, low, high) {
                // 原件的“末段固定 0.7/0.84”判据：lVar20 == len-2。
                (f32::from_bits(0x3f333333), f32::from_bits(0x3f570a3e))
            } else {
                jitter_widths(&rng)
            };
            let brick = StonePillarBrick {
                center: [x, y + (low + high) * 0.5, z],
                width_x,
                height: length,
                width_z,
            };
            out.push(
                brick
                    .to_ssbo(seed as u32 + emitted, context)
                    .observed_bytes(),
            );
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
        out.push(cap.to_ssbo(seed as u32 + emitted, context).observed_bytes());
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

/// 末段判定：pair 是 heights 的最后两个元素（原件 `lVar20 == len-2`）。
fn length_is_last_pair(heights: &[f32], low: f32, high: f32) -> bool {
    heights.len() >= 2 && heights[heights.len() - 2] == low && heights[heights.len() - 1] == high
}

/// B 0x140753670 `place_pillars` 的候选/装配宿主。
///
/// **未完成（I）**：52KB 原函数包含双缓冲队列、0x88B 段落、LoopRange/
/// CurveSlice、木柱二路与 MaterialStats；本轮只恢复 `add_proposal` /
/// `assemble_stone_pillar` 两个原语并把候选采集接到真实支柱原语上。
/// `PillarEmitContext` 的 material/flags/wall_distance 来源仍 U。
pub fn place_pillars(
    mut changed: EventReader<OnWallChanged>,
    mut bricks: EventReader<RecalculateWallBricks>,
    mut create_roof: EventReader<CreateRoofCmd>,
    walls: Res<PublicWalls>,
    ssbo: Res<AssetSsboLibrary>,
    mut proposals: ResMut<PillarProposals>,
) {
    let recalc: Vec<u64> = changed
        .read()
        .map(|e| e.0.0)
        .chain(bricks.read().map(|e| e.0.0))
        .chain(create_roof.read().map(|e| e.0.wall_id().0))
        .collect();
    for wall_id in recalc {
        let id = country_core::resources::walls::WallId(wall_id);
        let Some(wall) = walls.get(id) else {
            continue;
        };
        let curve = wall.simplified_curve_2d();
        if curve.points.len() < 2 {
            continue;
        }
        // 候选源仍 U（原件的双缓冲队列 + 0x88B 段 + 0.1875 半砖偏移 +
        // find_stone_column_start_y 未重建）。下面的 1.2m 间隔与 64 上限是
        // **显式占位常量**，不是原算法；只用于让 add_proposal 有真实事件
        // 驱动，不得按完成计。assemble_stone_pillar 需要真实 bottom/top 源，
        // 当前不在此处伪造调用。
        let total = curve.length;
        let count = ((total / f32::from_bits(0x3f99999a)).ceil() as usize).clamp(0, 64);
        for i in 0..count {
            let t = (i as f32 + 0.5) / count.max(1) as f32;
            let idx = ((t * (curve.points.len() - 1).max(1) as f32) as usize)
                .min(curve.points.len().saturating_sub(2));
            let p = curve.points[idx];
            let mut proposal = PillarPlacementProposal { bytes: [0u8; 28] };
            // score@+0x14：候选高度（原件由 find_stone_column_start_y 等给出）。
            proposal.set_score(p[1]);
            // 提案 index@+0x00（原件 0x1cB 记录首字段为采样索引）。
            proposal.bytes[0..4].copy_from_slice(&(idx as u32).to_le_bytes());
            proposals.add_proposal(wall_id, proposal);
        }
        let _ = ssbo;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let segs = pillar_segments(1.1, 3.0);
        assert_eq!(segs.len(), 2);
        assert_eq!(segs[0].0, 0.0);
        assert!((segs[0].1 - 1.3).abs() < 1e-6);
        assert!(segs[0].2);
        assert!(segs[1].2, "|Δ| = 1.9 > 1.2");
        let segs = pillar_segments(1.1, 2.0);
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
