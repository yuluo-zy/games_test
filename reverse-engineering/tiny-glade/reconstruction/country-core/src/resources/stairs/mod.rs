//! stairs/mod.rs — 楼梯资源族（WP6 恢复；WP21 楼梯图余量）。
//!
//! - `visual`（WP6）：楼梯可视化资源。
//! - `stairs_state` / `stairs_assembly_params` / `stairs_collider` /
//!   `structural_graph` / `stairs_removed_supports` / `invalidate`（WP21）。

#![allow(dead_code, unused_imports)]

pub mod invalidate;
pub mod stairs_assembly_params;
pub mod stairs_collider;
pub mod stairs_removed_supports;
pub mod stairs_state;
pub mod structural_graph;
pub mod visual;

use crate::typed_gap::Gap;

/// `ColliderHandle`（WP21 楼梯域 typed-gap 边界）。O：空判别字节 0x11；
/// 实际碰撞句柄为 `systems::collision::world::SubworldColliderHandle`
/// `{subworld, index}`，其恢复面未含 Empty 判别，故楼梯结构图保留本局部
/// 枚举作为节点键（GraphMap 需要 Copy+Ord+Hash）。见 INTEGRATION_FINAL.md。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ColliderHandle {
    Empty,
    Handle(u64),
}

impl ColliderHandle {
    /// O：空判别字节 0x11。
    pub const EMPTY: u8 = 0x11;
}

/// `DecoratorId` re-export convenience（`stairs_state` 图节点值类型）。
pub use crate::resources::walls::decorator::DecoratorId;
