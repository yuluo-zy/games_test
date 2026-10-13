//! stairs_removed_supports.rs — 已移除楼梯支撑（WP21）。
//!
//! O：PDB 实体 `StairsRemovedSupports`；serde serialize/deserialize 各 1 编译实例
//! （存档类型）。字段 U。

use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

use crate::typed_gap::Gap;

/// 已移除的楼梯支撑集（O：serde 存档类型；判别/字段 U）。
#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize)]
pub struct StairsRemovedSupports {
    pub raw: Gap,
}
