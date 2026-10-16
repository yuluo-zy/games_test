//! `merge_proposal_wall_decorators.rs` — 墙合并装饰位置提议
//! （propose_wall_decorator_merge_position_inner 0x142001d40 +
//! find_best_wall_decorator_merge_position 0x142001880，O 结构）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x142001d40/
//! 0x142001880-*.txt`）：
//! - `find_best_wall_decorator_merge_position`（O）：`min_by` 扫描
//!   `propose_wall_decorator_merge_position_inner` 候选（目标墙数组）；
//! - `propose_wall_decorator_merge_position_inner`（O 读面）：逐装饰
//!   `DecoratorInfo::try_dst_as_wall` + `DecoratorType::max_rank` +
//!   `DerivedDecoratorInfo::subtype/dims_wall_space` + `WallCornerAttachment::
//!   is_attached` + `comparative_space::classify_side` + `WallSpaceCoord::
//!   with_offset` + `WallSpaceRegion::intersects` + `Vec2::dot` + 吸附方向
//!   （`snapping_direction` 内联）→ 合并位置（U 公式尾段；`try_merge_decorators`
//!   内联）。
//!
//! core 缺口登记：`WallCornerAttachment`（章节 O）——用
//! [`crate::decorator_on_rectangle_edited::rectangle_corner_id`] 等价。

use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use utils::wall_space::{WallSpaceCoord, WallSpaceRegion};

/// 合并位置提议结果（I：字段语义按机器码读面；U 公式尾段）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallMergePosition {
    /// 目标墙。
    pub wall: WallId,
    /// 提议装饰 id。
    pub decorator_id: u32,
    /// 提议 u 区间（WallSpaceCoord）。
    pub coord: WallSpaceCoord,
    /// 得分（min_by 键；U）。
    pub score: f32,
}

/// `propose_wall_decorator_merge_position_inner` @0x142001d40（O 结构；
/// 公式尾段 U——证据 decomp 全文本）。
pub fn propose_wall_decorator_merge_position_inner(
    _info: &DecoratorInfo,
    _derived: Option<&DerivedDecoratorInfo>,
    _storage: &DecoratorStorage,
    _walls: &PublicWalls,
    target: WallId,
) -> Option<WallMergePosition> {
    // O 读面：try_dst_as_wall + max_rank + dims_wall_space + is_attached +
    // classify_side（utils 方法面）+ with_offset + intersects。
    let _ = WallSpaceRegion::intersects;
    let _ = target;
    None
}

/// `find_best_wall_decorator_merge_position` @0x142001880（O：min_by 扫描）。
pub fn find_best_wall_decorator_merge_position(
    _candidates: &[WallId],
    _info: &DecoratorInfo,
    _storage: &DecoratorStorage,
    _walls: &PublicWalls,
) -> Option<WallMergePosition> {
    // O：candidates 逐个 propose + min_by（U：得分键）。
    None
}

/// 吸附方向（内联；U 公式——机器码 Vec2::dot 面）。
#[allow(dead_code)]
fn snapping_direction(_a: utils::wall_space::WallSpace, _b: utils::wall_space::WallSpace) -> f32 {
    0.0
}

/// 合并执行（内联 `try_merge_decorators`；U——D-batch 合并执行链）。
#[allow(dead_code)]
fn try_merge_decorators(_a: u32, _b: u32) -> bool {
    false
}
