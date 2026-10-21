//! B `decorator_visual/stairs.rs` — 玩家楼梯装饰（D1）。
//!
//! 证据：fresh RE-MCP 反编译（11 业务 VA）+ 骨架 callees（O）+ 常量 rne-read。
//!
//! 已全文恢复（O）：
//! - `determine_segment_type` 0x14080c810：段类型（坡度比 → 1/2/3）
//! - `densely_sample_spline_segment` 0x140809ad0：委托 utils::Spline2D::
//!   densely_sample_segment（utils 侧已按本函数 VA 恢复，B 原文 451 模块）
//! - `derive_stair_node_params` 0x14080c640：按 dst tag 跳表分派
//!
//! 结构恢复（O 业务链，U 主体）：find_alignment_preferences /
//! generate_playermade_stair_decorators / generate_playermade_stair_requests
//! <false,true> / walk_shape / maintain_stair_graph（apply_decorator_op 消费）/
//! compute_stair_assembly_params / find_best_neighbor_alignments。
//!
//! 常量（O）：段类型阈值 = |Δh|/弦长（0x1429cd064=2.5、0x142925950=0.25）。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::stairs::stairs_assembly_params::AlignKind;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use glam::{Vec2, Vec3};
use utils::natural_cubic::{DenseSplineSegment2D, Spline2D};

use crate::blueprint::DecoratorBlueprints;

/// 段类型阈值（O：0x1429cd064 = 2.5、0x142925950 = 0.25）。
const STEEP_SLOPE: f32 = 2.5;
const GENTLE_SLOPE: f32 = 0.25;

/// 0x14080c810（O 全文）：段类型（坡度比 |Δh|/弦长 → 1 缓 / 2 中 / 3 陡）。
/// 参数：param_1 = 起点 {x f32, y f32}（u64 各）、param_2 = 点数、param_3 =
/// 高度序列、param_4 = 高度数。弦长 = 首尾 2D 距离；坡度 = |Δh0 - ΔhN|/弦长。
pub fn determine_segment_type(
    start: (f32, f32),
    end: (f32, f32),
    heights: &[f32],
) -> u8 {
    if heights.is_empty() {
        // 原 panic（UNK_142a8e3d8 = stairs.rs 源）。
        panic!("determine_segment_type: empty heights");
    }
    let first = *heights.first().unwrap();
    let last = *heights.last().unwrap();
    let chord = Vec2::new(end.0 - start.0, end.1 - start.1).length();
    let slope = (first - last).abs() / chord;
    if slope <= STEEP_SLOPE {
        if GENTLE_SLOPE < slope {
            2
        } else {
            1
        }
    } else {
        3
    }
}

/// 0x140809ad0（O）：密集采样样条段（原属 stairs.rs 模块 451；已恢复于
/// utils::Spline2D::densely_sample_segment，此处直通）。
pub fn densely_sample_spline_segment(spline: &Spline2D, index: usize) -> DenseSplineSegment2D {
    spline.densely_sample_segment(index)
}

/// 0x14080c640（O 结构）：按 dst tag（DecoratorInfo+0x22）分派楼梯节点参数
/// （跳表 0x142a8e448：{墙/屋顶/地形/楼梯} 四处理）。U：各分支载荷语义。
pub fn derive_stair_node_params(
    info: &country_core::resources::walls::decorator::DecoratorInfo,
    _key: u64,
) -> u8 {
    // O：跳表索引 `(tag-2)<3 ? tag-1 : 0`（fresh capstone：`lea ecx,[tag-2];
    // cmp cl,3; cmovb idx,tag-1`）。idx 0=墙 / 1=屋顶 / 2=地形 / 3=楼梯。
    // 各分支载荷语义仍 U（返回索引，不伪造参数记录）。
    let tag = info.dst_tag();
    if tag.wrapping_sub(2) < 3 {
        tag - 1
    } else {
        0
    }
}

/// 0x14080beb0（O 结构，6+ 参面）：对齐偏好计算。
/// U：偏好准则（邻墙对齐）主体。
pub fn find_alignment_preferences(
    _p1: u64,
    _p2: u64,
    _p3: u64,
    _f4: f32,
    _f5: f32,
) -> Vec<(u32, f32)> {
    Vec::new() // U。
}

/// B 0x14080c360：遍历完整邻接候选组合，返回所有选择及其总分。
/// 冲突方向每次扣 10；严格大于才替换，保持原先后顺序与 NaN 分支。
pub fn find_best_neighbor_alignments(
    candidates: &[(DecoratorId, Vec<(AlignKind, f32)>)],
    used_directions: u8,
) -> (Vec<(DecoratorId, AlignKind)>, f32) {
    let Some(((id, options), remaining)) = candidates.split_first() else {
        return (Vec::new(), 0.0);
    };
    let mut best = Vec::new();
    let mut best_score = -f32::MAX;
    for &(kind, score) in options {
        let direction = match kind.tag {
            0 => 1,
            1 => {
                let left = kind.first.x * kind.second.y;
                let right = kind.first.y * kind.second.x;
                // 原 UCOMISS/SETBE 在 unordered 时也取 mask4。
                if left > right { 2 } else { 4 }
            }
            _ => unreachable!(),
        };
        let (mut choices, child_score) =
            find_best_neighbor_alignments(remaining, used_directions | direction);
        let penalty = if used_directions & direction != 0 { -10.0 } else { 0.0 };
        let total = (score + child_score) + penalty;
        if total > best_score {
            // 接管递归的真实 Vec；原算法在前部插入当前邻居，避免整表 clone。
            choices.insert(0, (*id, kind));
            best = choices;
            best_score = total;
        }
    }
    (best, best_score)
}

/// 0x140803dd0（O 结构）：生成玩家楼梯装饰视觉（walk_shape 消费 + 蓝图）。
pub fn generate_playermade_stair_decorators(
    _storage: Res<DecoratorStorage>,
    _derived: Res<DerivedDecoratorSubtypes>,
    _blueprints: ResMut<DecoratorBlueprints>,
    _transient: ResMut<crate::decorator_visual_state::DecoratorTransientVisualState>,
) {
    // U：楼梯视觉生成主体待取证。
}

/// 0x140276e80 / 0x14027e470（O 结构，泛型 <false,true>）：生成玩家楼梯请求
/// （形状遍历 → 段类型 → 节点参数 → 装配参数）。
pub fn generate_playermade_stair_requests<const X: bool>() {
    // U：主体待取证。
}

/// 0x140809cc0（O 结构）：形状遍历（多边形 → 边段采样）。
/// U：遍历算法主体。
pub fn walk_shape(
    _shape: &[Vec2],
) -> Vec<(u32, Vec3)> {
    Vec::new() // U。
}

/// 0x140806110（O 结构，8 参面）：维护楼梯图（StairsState 消费 +
/// apply_decorator_op 应用）。U：图维护主体。
pub fn maintain_stair_graph(
    _storage: Res<DecoratorStorage>,
    _stairs: Res<country_core::resources::stairs::stairs_state::StairsState>,
    _commands: Commands,
) {
    // U。
}

/// 0x140807090（O 结构，6 参面）：计算楼梯装配参数。
/// U：装配参数主体（StairWallAnchor 消费）。
pub fn compute_stair_assembly_params(
    _nodes: &[(u32, Vec3)],
) -> Vec<f32> {
    Vec::new() // U。
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator::DecoratorInfo;

    fn info_tag(tag: u8) -> DecoratorInfo {
        DecoratorInfo {
            u20: u32::from(tag) << 16,
            ..DecoratorInfo::default()
        }
    }

    #[test]
    fn stair_node_dispatch_index_matches_native_formula() {
        // O：tag 2/3/4 → idx 1/2/3；其余 → 0。
        assert_eq!(derive_stair_node_params(&info_tag(2), 0), 1);
        assert_eq!(derive_stair_node_params(&info_tag(3), 0), 2);
        assert_eq!(derive_stair_node_params(&info_tag(4), 0), 3);
        assert_eq!(derive_stair_node_params(&info_tag(0), 0), 0);
        assert_eq!(derive_stair_node_params(&info_tag(1), 0), 0);
        assert_eq!(derive_stair_node_params(&info_tag(5), 0), 0);
    }
    fn direction_kind(tag: u8, first: Vec2, second: Vec2) -> AlignKind {
        AlignKind { tag, first, second, ..AlignKind::default() }
    }

    #[test]
    fn alignment_search_chooses_the_global_non_conflicting_combination() {
        let straight = direction_kind(0, Vec2::X, Vec2::ZERO);
        let turn = direction_kind(1, Vec2::X, Vec2::Y);
        let candidates = vec![
            (DecoratorId(7), vec![(straight, 9.0), (turn, 8.0)]),
            (DecoratorId(29), vec![(straight, 9.0)]),
        ];
        let before = candidates.clone();
        let (choices, score) = find_best_neighbor_alignments(&candidates, 0);
        assert_eq!(choices, vec![(DecoratorId(7), turn), (DecoratorId(29), straight)]);
        assert_eq!(score, 17.0);
        assert_eq!(candidates, before);
    }

    #[test]
    fn alignment_search_preserves_the_first_equal_score_candidate() {
        let first = direction_kind(0, Vec2::X, Vec2::ZERO);
        let second = direction_kind(0, Vec2::Y, Vec2::ZERO);
        let candidates = [(DecoratorId(3), vec![(first, 5.0), (second, 5.0)])];
        assert_eq!(find_best_neighbor_alignments(&candidates, 0),
            (vec![(DecoratorId(3), first)], 5.0));
    }

    #[test]
    fn alignment_empty_inputs_and_empty_candidate_lists_have_distinct_scores() {
        assert_eq!(find_best_neighbor_alignments(&[], 7), (vec![], 0.0));
        let candidates = [(DecoratorId(3), vec![])];
        assert_eq!(find_best_neighbor_alignments(&candidates, 0), (vec![], -f32::MAX));
    }

    #[test]
    fn alignment_unordered_cross_uses_mask_four_and_nan_score_never_wins() {
        let kind = direction_kind(1, Vec2::new(f32::NAN, 0.0), Vec2::Y);
        let candidates = [(DecoratorId(3), vec![(kind, 5.0)])];
        assert_eq!(find_best_neighbor_alignments(&candidates, 4).1, -5.0);
        assert_eq!(find_best_neighbor_alignments(&candidates, 2).1, 5.0);
        let straight = direction_kind(0, Vec2::X, Vec2::ZERO);
        let scores = [(DecoratorId(3), vec![(straight, f32::NAN), (straight, 1.0)])];
        assert_eq!(find_best_neighbor_alignments(&scores, 0),
            (vec![(DecoratorId(3), straight)], 1.0));
        let all_nan = [(DecoratorId(3), vec![(straight, f32::NAN)])];
        assert_eq!(find_best_neighbor_alignments(&all_nan, 0), (vec![], -f32::MAX));
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn alignment_none_variant_retains_original_panic() {
        let none = AlignKind { tag: 2, ..AlignKind::default() };
        let _ = find_best_neighbor_alignments(&[(DecoratorId(3), vec![(none, 0.0)])], 0);
    }

}
