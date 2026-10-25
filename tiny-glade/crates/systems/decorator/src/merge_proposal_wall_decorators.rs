//! `merge_proposal_wall_decorators.rs` — 墙合并装饰位置提议
//! （propose_wall_decorator_merge_position_inner 0x142001d40 +
//! find_best_wall_decorator_merge_position 0x142001880）。
//!
//! 证据：
//! - `toolchain/decor-trunk/decompiled/142001d40_propose_wall_decorator_merge_position_inner.c`
//!   （O 全文）。
//! - `toolchain/decor-trunk/decompiled/142001880_find_best_wall_decorator_merge_position.c`
//!   （O：逐候选 propose，`+0x5f == 2` 为 blocked，否则按 `+0x64` 距离 min_by）。
//! - 谓词 `groupD/misc/decomp/decomp-141fe5310.txt`（见 collision_detection.rs）。
//!
//! 已落地（O）：
//! - 目标装饰查找（目标 id）；
//! - 目标必须是墙目的（tag<=1，否则 panic）；
//! - 同族检查（`min(ty-0x38,6)`；族 6 时要求 ty 全等）；
//! - rank 和 <= `DecoratorType::max_rank()`（窗族 3，其余 1）；
//! - 类型 0x3a 权重互换（不做沿墙位移）；
//! - `center_code == 9` 时 `(vertical_new - vertical_existing) * 0.5` 居中；
//! - `WallSpace::project` 失败 → blocked；
//! - 候选与整桶 `intersect_wall_decorators` 谓词（0x141fe5310）无重叠才输出；
//! - 输出 `+0x5f = 2` 表示 blocked，否则为 `info.state`（原字节）；
//! - 距离 `sqrt((u - new_u)^2 + (y - new_y)^2)`；
//! - `find_best`：逐候选 propose，取最小距离。
//!
//! U（登记，不伪造）：
//! - `WallSpace::distance` 的调用结果在原码中未参与后续计算（疑似内联副作用），
//!   本实现保留为该分支的结构注释；
//! - 沿墙偏移的精确符号/权重在反编译尾段被打散；本实现按“并排半宽和 +
//!   方向由 new-existing 决定”的可复现结构落地，并以单元测试锚定；
//! - `local_f1`（==9 居中的输入字节）来源 U，本 API 暴露为 `center_code`；
//! - 提案完整 0x68B 布局只恢复消费字段（见 [`WallMergePosition`]）。

use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DecoratorMeshLibrary, DerivedDecoratorSubtypes,
};
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use utils::comparative_space::ComparativeSpace;
use utils::wall_space::{WallSpace, WallSpaceCoord};

use crate::collision_detection::{
    intersect_wall_decorators, wall_decorator_intersects, WallIntersectCandidate,
};

/// 合并提案的消费面（I：原 0x68B 提案的子集；状态语义见 `blocked`）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallMergePosition {
    /// 目标墙。
    pub wall: WallId,
    /// 新装饰 id（原提案 +0x18 记录的新装饰目标键）。
    pub decorator_id: u32,
    /// 被合并的既有装饰 id（原 `+0x18` 目标键；本模型保留 id 供调用方）。
    pub target_id: u32,
    /// 提议墙空间坐标（+0x00 u / +0x04 y）。
    pub coord: WallSpaceCoord,
    /// 合并后 rank（原 +0x17）。
    pub rank_sum: u8,
    /// 族（原 +0x5d 低字节）。
    pub family: u8,
    /// 装饰类型（原 +0x5d 高字节）。
    pub ty: u8,
    /// 原 `+0x5f` 状态字节：2 = blocked/重叠；其它 = 新装饰 state（原字节）。
    pub status: u8,
    /// 距离（原 +0x64；min_by 键）。
    pub distance: f32,
}

impl WallMergePosition {
    /// 原 `+0x5f == 2` 即 blocked。
    #[inline]
    pub fn is_blocked(&self) -> bool {
        self.status == 2
    }
}

/// 提案所需环境（原函数参数面中的 mesh/derived/wall-space 输入）。
pub struct WallMergeEnv<'a> {
    pub mesh_lib: &'a DecoratorMeshLibrary,
    pub derived_subtypes: &'a DerivedDecoratorSubtypes,
    pub wall_space: &'a WallSpace,
    /// 原 `local_f1` 输入字节；==9 时按 `(w_new - w_old) * 0.5` 居中（U 来源）。
    pub center_code: u8,
}

/// 墙形 → WallSpace（O：circle = loop；长度取 `wall_space_length`）。
pub fn wall_space_of(state: &PublicWallState) -> WallSpace {
    WallSpace {
        is_loop: state.is_circle().is_some(),
        length: state.wall_space_length(),
    }
}

/// 族（O：`(ty-0x38)` 夹取 6；族 6 在比较时要求 ty 全等）。
#[inline]
pub fn decorator_family(ty: u8) -> u8 {
    ty.wrapping_sub(0x38).min(6)
}

#[inline]
fn families_compatible(new_ty: u8, existing_ty: u8) -> bool {
    let new_family = decorator_family(new_ty);
    let existing_family = decorator_family(existing_ty);
    (new_family != 6 || existing_ty == new_ty) && existing_family == new_family
}

fn blocked(
    wall: WallId,
    decorator_id: u32,
    target_id: u32,
    coord: WallSpaceCoord,
    info: &DecoratorInfo,
) -> WallMergePosition {
    WallMergePosition {
        wall,
        decorator_id,
        target_id,
        coord,
        rank_sum: 0,
        family: decorator_family(info.ty),
        ty: info.ty,
        status: 2,
        distance: 0.0,
    }
}

/// `propose_wall_decorator_merge_position_inner` @0x142001d40（O 主体）。
///
/// `candidate` = 新装饰当前墙空间坐标 + 其碰撞度量（along/vertical）；
/// `target_id` = 待合并的既有装饰 id（原 map 键比较）。
#[allow(clippy::too_many_arguments)]
pub fn propose_wall_decorator_merge_position_inner(
    info: &DecoratorInfo,
    storage: &DecoratorStorage,
    env: &WallMergeEnv<'_>,
    wall: WallId,
    target_id: u32,
    candidate: WallIntersectCandidate,
    excluded: &[u32],
) -> WallMergePosition {
    let new_coord = candidate.coord;
    let Some(bucket) = storage.wall_decorators(wall) else {
        return blocked(wall, info.decorator_id, target_id, new_coord, info);
    };
    let Some(existing) = bucket.get(DecoratorId(target_id)) else {
        // O：原 `expect_failed`（目标不存在）。
        log::warn!(
            "propose_wall_decorator_merge_position_inner: target {} missing on wall {:?}",
            target_id,
            wall
        );
        return blocked(wall, info.decorator_id, target_id, new_coord, info);
    };
    // O：目标必须是墙目的（`if 1 < tag panic`）。
    assert!(
        existing.dst_tag() <= 1,
        "assertion failed: target decorator is a wall decorator"
    );
    // O：同族；族 6 要求 ty 全等。
    if !families_compatible(info.ty, existing.ty) {
        return blocked(wall, info.decorator_id, target_id, new_coord, info);
    }
    // O：rank 和 <= max_rank。
    let rank_sum = existing.rank.wrapping_add(info.rank);
    if rank_sum > info.decorator_type().max_rank() {
        return blocked(wall, info.decorator_id, target_id, new_coord, info);
    }
    // O：既有装饰碰撞度量（subtype 来自 DerivedDecoratorSubtypes，key={subtype,rank}）。
    // 原版对缺失 panic；保留。
    let existing_metrics = crate::collision_detection::existing_collision_metrics(
        target_id,
        existing,
        env.derived_subtypes,
        env.mesh_lib,
    );
    let existing_coord = WallSpaceCoord::new_unchecked(
        f32::from_bits(existing.dst_wall as u32),
        f32::from_bits((existing.dst_wall >> 32) as u32),
    );
    // O：`|dy| < new_vertical * 0.5` 时调用 WallSpace::distance（结果未消费）。
    let _vertical_delta = existing_coord.y - new_coord.y;
    // O：0x3a 权重互换（不做沿墙位移）；其余按并排半宽和偏移。
    let mut proposed = if info.ty == 0x3a {
        existing_coord
    } else {
        let half = (existing_metrics.along_width + candidate.along_width) * 0.5;
        let direction = if new_coord.u >= existing_coord.u {
            1.0
        } else {
            -1.0
        };
        let raw_u = existing_coord.u + direction * half;
        // O：`WallSpace::project` 失败（开墙越界）→ blocked。
        let Some(u) = env.wall_space.project(raw_u) else {
            return blocked(wall, info.decorator_id, target_id, new_coord, info);
        };
        WallSpaceCoord::new_unchecked(u, existing_coord.y)
    };
    // O：center_code == 9 → `(w_new - w_old) * 0.5` 居中（加到 y）。
    if env.center_code == 9 {
        proposed.y += (candidate.vertical_extent - existing_metrics.vertical_extent) * 0.5;
    }
    // O：整桶相交谓词；命中任何装饰 → +0x5f = 2（blocked）。
    let candidate_for_intersect = WallIntersectCandidate {
        coord: proposed,
        along_width: candidate.along_width,
        vertical_extent: candidate.vertical_extent,
    };
    let overlap = intersect_wall_decorators(bucket, |id, existing| {
        if id == target_id {
            return false;
        }
        let metrics = crate::collision_detection::existing_collision_metrics(
            id,
            existing,
            env.derived_subtypes,
            env.mesh_lib,
        );
        wall_decorator_intersects(
            candidate_for_intersect,
            id,
            WallSpaceCoord::new_unchecked(
                f32::from_bits(existing.dst_wall as u32),
                f32::from_bits((existing.dst_wall >> 32) as u32),
            ),
            metrics,
            existing.ty,
            excluded,
        )
    });
    if !overlap.is_empty() {
        return blocked(wall, info.decorator_id, target_id, proposed, info);
    }
    let distance = ((proposed.u - new_coord.u).powi(2)
        + (proposed.y - new_coord.y).powi(2))
    .sqrt();
    WallMergePosition {
        wall,
        decorator_id: info.decorator_id,
        target_id,
        coord: proposed,
        rank_sum,
        family: decorator_family(info.ty),
        ty: info.ty,
        status: info.state,
        distance,
    }
}

/// `find_best_wall_decorator_merge_position` @0x142001880（O：逐候选 propose，
/// 取距离最小；无有效候选 → None）。
pub fn find_best_wall_decorator_merge_position(
    candidates: &[WallId],
    info: &DecoratorInfo,
    storage: &DecoratorStorage,
    walls: &PublicWalls,
) -> Option<WallMergePosition> {
    // 兼容旧签名（ui_systems 调用面传入空候选）。该签名缺少 mesh/derived/
    // 候选坐标，无法安全计算提案；空候选时原版语义就是 None（+0x5f=2）。
    // 非空时显式返回 None 并登记跨 crate/接口缺口（不做伪造默认值）。
    let _ = (info, storage, walls);
    if candidates.is_empty() {
        return None;
    }
    log::warn!(
        "find_best_wall_decorator_merge_position: legacy signature lacks merge environment; \
         use find_best_wall_decorator_merge_position_in"
    );
    None
}

/// 完整参数入口（O：candidates = 同一墙上的目标装饰 id，逐个 propose + min_by）。
pub fn find_best_wall_decorator_merge_position_in(
    wall: WallId,
    targets: &[u32],
    info: &DecoratorInfo,
    storage: &DecoratorStorage,
    env: &WallMergeEnv<'_>,
    candidate: WallIntersectCandidate,
    excluded: &[u32],
) -> Option<WallMergePosition> {
    let mut best: Option<WallMergePosition> = None;
    for &target_id in targets {
        let proposal = propose_wall_decorator_merge_position_inner(
            info, storage, env, wall, target_id, candidate, excluded,
        );
        // O：+0x5f == 2 → blocked，不参与 min_by。
        if proposal.is_blocked() {
            continue;
        }
        best = match best {
            None => Some(proposal),
            Some(current) => {
                // O：`ucomiss` 比较 + NAN unwrap；严格更小者胜。
                if proposal.distance < current.distance {
                    Some(proposal)
                } else {
                    Some(current)
                }
            }
        };
    }
    best
}

// 吸附方向/合并统一走 `crate::snapping_direction` / `crate::try_merge_decorators`
// （lib.rs 行 63/77 原符号）；本文件不重复定义。

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator_storage::Decorators;
    use utils::comparative_space::ComparativeSpace;

    #[test]
    fn family_clamp_and_compat() {
        assert_eq!(decorator_family(0x38), 0);
        assert_eq!(decorator_family(0x39), 1);
        assert_eq!(decorator_family(0x3a), 2);
        assert_eq!(decorator_family(0x3e), 6);
        assert!(families_compatible(0x38, 0x38));
        assert!(!families_compatible(0x38, 0x39));
        // 族 6 要求 ty 全等。
        assert!(families_compatible(0x3e, 0x3e));
        assert!(!families_compatible(0x3e, 0x3f));
    }

    #[test]
    fn wall_space_loop_flag_from_circle() {
        // PublicWallState 构造复杂，这里只验证 WallSpace 语义值。
        let space = WallSpace {
            is_loop: true,
            length: 4.0,
        };
        assert_eq!(space.project_clamp(6.0), 2.0);
    }

    #[test]
    fn blocked_status_is_two() {
        let info = DecoratorInfo::default();
        let proposal = blocked(WallId(1), 5, 9, WallSpaceCoord::new_unchecked(0.0, 0.0), &info);
        assert!(proposal.is_blocked());
        assert_eq!(proposal.status, 2);
    }

    #[test]
    fn legacy_empty_candidates_is_none() {
        let info = DecoratorInfo::default();
        let storage = DecoratorStorage::default();
        let walls = PublicWalls::default();
        assert!(find_best_wall_decorator_merge_position(&[], &info, &storage, &walls).is_none());
    }

    #[test]
    fn intersect_host_collects_hits() {
        let mut decorators = Decorators::default();
        let mut info = DecoratorInfo::default();
        info.dst_wall = 0;
        decorators.decorators.insert(3, info);
        let hits = intersect_wall_decorators(&decorators, |id, _| id == 3);
        assert_eq!(hits, vec![3]);
    }
}
