//! B `merge_proposal_wall_decorators.rs` — 墙装饰合并提议（D2 消费契约；主体归
//! D-owner）。
//!
//! `find_best_wall_decorator_merge_position`（calculate_decorator_interaction_intent
//! 0x14127cf70 case-0 消费；lib.rs 符号 `propose_wall_decorator_merge_position_inner`
//! 族）：在目标墙上搜索最优合并位置（I：精确几何/评分 U，D2 面只承载签名）。
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::public_wall_state::PublicWallState;
use glam::Vec3;

/// 合并位置提议（I）。
#[derive(Clone, Copy, Debug, Default)]
pub struct WallDecoratorMergePosition {
    /// 世界位置。
    pub pos: Vec3,
    /// 目标装饰 id。
    pub target: Option<DecoratorId>,
}

/// 最优合并位置（I：签名按调用点；算法 U）。
pub fn find_best_wall_decorator_merge_position(
    _id: DecoratorId,
    _wall: &PublicWallState,
) -> Option<WallDecoratorMergePosition> {
    None
}
