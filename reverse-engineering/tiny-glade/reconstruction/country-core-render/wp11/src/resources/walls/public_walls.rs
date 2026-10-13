//! public_walls.rs — 公共墙集合资源（WP11 状态家族；WP6 骨架扩展）。
//!
//! 源骨架 `resources/walls/public_walls.rs`（B）；PDB 命名实体：
//! `PublicWalls` / `WallEntities` / `WallPlasters`。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;
use std::collections::HashMap;

use crate::resources::walls::public_wall_state::PublicWallState;
use crate::resources::walls::WallId;
use crate::typed_gap::Gap;

/// 公共墙集合资源（O：`HashMap<WallId, PublicWallState>` 发布状态）。
/// 内部字段（roof hashmap 关联、dirty）为 typed gap。
#[derive(Resource, Clone, Debug, Default)]
pub struct PublicWalls {
    /// 公共墙记录（O：hashbrown HashMap<WallId, PublicWallState>，PDB 泛型证据）。
    pub walls: HashMap<WallId, PublicWallState>,
    /// 其它未恢复字段。
    pub gap: Gap,
}

/// 墙实体集合（O：命名实体；`HashMap<WallId, Entity>`）。布局 U。
#[derive(Resource, Clone, Debug, Default)]
pub struct WallEntities(pub Gap);

/// 墙石膏集合（O：命名实体；`HashMap<WallId, Entity>`）。布局 U。
#[derive(Resource, Clone, Debug, Default)]
pub struct WallPlasters(pub Gap);
