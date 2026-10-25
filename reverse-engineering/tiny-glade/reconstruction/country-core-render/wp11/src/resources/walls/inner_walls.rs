//! inner_walls.rs — 内部墙集合资源（WP11）。
//!
//! 源骨架 `resources/walls/inner_walls.rs`（B）；PDB 命名实体 `InnerWalls`。
//! WP12 系统以 `ResMut<InnerWalls>` 消费（adjust_wall_height_impl 等骨架签名）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;
use std::collections::HashMap;

use crate::resources::walls::{WallId, inner_wall_state::WallState};
use crate::typed_gap::Gap;

/// 内部墙集合资源（O：`HashMap<WallId, WallState>` 权威编辑状态）。
/// 内部字段（dirty 集合、roof 关联、terrain 收集）为 typed gap。
#[derive(Resource, Clone, Debug, Default)]
pub struct InnerWalls {
    /// 内部墙记录（O：HashMap<WallId, WallState>；hashbrown/std map 属库实现）。
    pub walls: HashMap<WallId, WallState>,
    /// 其它未恢复字段（dirty 等）。
    pub gap: Gap,
}
