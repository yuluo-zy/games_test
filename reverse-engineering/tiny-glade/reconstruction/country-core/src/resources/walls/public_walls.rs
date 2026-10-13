//! public_walls.rs — 公共墙集合资源（WP11 状态家族 + WP12 系统向字段/方法面）。
//!
//! 源骨架 `resources/walls/public_walls.rs`（B）；PDB 命名实体：
//! `PublicWalls` / `WallEntities` / `WallPlasters`。

#![allow(dead_code, unused_imports)]

use bevy_ecs::entity::Entity;
use bevy_ecs::resource::Resource;
use std::collections::HashMap;

use crate::resources::walls::public_wall_state::PublicWallState;
use crate::resources::walls::WallId;
use crate::typed_gap::Gap;

/// 公共墙集合资源（O：`HashMap<WallId, PublicWallState>` 发布状态）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PublicWalls {
    /// 公共墙记录（O：hashbrown HashMap<WallId, PublicWallState>，PDB 泛型证据）。
    pub walls: HashMap<WallId, PublicWallState>,
    /// 墙 id → 墙实体（O：`delete_wall` / `wall_roof_hashmap` 读写）。
    pub wall_entities_map: HashMap<WallId, Entity>,
    /// 墙 id → roof 实体（O：`wall_roof_hashmap` 写）。
    pub roofs: HashMap<WallId, Entity>,
    /// 其它未恢复字段。
    pub gap: Gap,
}

impl PublicWalls {
    /// O：按 id 取发布状态。
    pub fn get(&self, id: WallId) -> Option<&PublicWallState> {
        self.walls.get(&id)
    }
    /// O：全部发布墙 id。
    pub fn wall_ids(&self) -> Vec<WallId> {
        self.walls.keys().copied().collect()
    }
    /// O：墙实体查询（wall_highlight / delete_wall）。
    pub fn wall_entity(&self, id: WallId) -> Option<Entity> {
        self.wall_entities_map.get(&id).copied()
    }
    /// O：roof 实体查询。
    pub fn roof_entity(&self, id: WallId) -> Option<Entity> {
        self.roofs.get(&id).copied()
    }
    /// O：包含判定。
    pub fn contains(&self, id: WallId) -> bool {
        self.walls.contains_key(&id)
    }
}

/// 墙实体集合（O：命名实体；`HashMap<WallId, Entity>`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct WallEntities {
    pub map: HashMap<WallId, Entity>,
}

impl WallEntities {
    /// O：`wall_entity`（0x140981390 消费）。
    pub fn wall_entity(&self, id: WallId) -> Option<Entity> {
        self.map.get(&id).copied()
    }
}

/// 墙石膏集合（O：命名实体；`HashMap<WallId, Vec<Entity>>`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct WallPlasters {
    pub map: HashMap<WallId, Vec<Entity>>,
}
