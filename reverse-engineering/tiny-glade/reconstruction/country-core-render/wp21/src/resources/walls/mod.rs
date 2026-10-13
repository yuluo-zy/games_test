//! walls/ — 墙资源余量（WP21：arches / dormers / wall_holes / wall_stone_columns /
//! wooden_gates / decorator_collision / decorator_outline / decorator_subtype /
//! elevation_support_stats）。WP11 状态族 + WP12 facade 已由整合 crate 恢复。

pub mod arches;
pub mod decorator_collision;
pub mod decorator_outline;
pub mod decorator_subtype;
pub mod dormers;
pub mod elevation_support_stats;
pub mod wall_holes;
pub mod wall_stone_columns;
pub mod wooden_gates;

// 复用整合 crate 的权威类型名（本包只定义余量模块，WallId 走 boundary）。
pub use crate::boundary::WallId;
