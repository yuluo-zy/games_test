//! 石柱碰撞存储（walls/wall_stone_columns.rs）。
//!
//! O：`StoneColumnCollisionStorage::clear_and_get_mut` 0xa78880 ——
//! `{ cleared: Vec<WallId>, map: HashMap<WallId, Vec<StoneColumn>> }`；
//! `maintain_stone_column_colliders`（0x14078a6e0）消费。
use bevy_ecs::resource::Resource;
use std::collections::HashMap;

use crate::resources::walls::WallId;

/// 单根石柱碰撞数据（O 名；布局 U）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StoneColumn {
    /// 碰撞网格句柄（观测 u64；布局 I）。
    pub handle: u64,
}

/// 石柱碰撞存储（O：clear_and_get_mut 0xa78880）。
#[derive(Resource, Clone, Debug, Default)]
pub struct StoneColumnCollisionStorage {
    /// 本帧已清（去重）的墙 id 列表（O：先 push 后清 map）。
    pub cleared: Vec<WallId>,
    /// 每墙石柱列表（O：HashMap<WallId, Vec<StoneColumn>>）。
    pub columns: HashMap<WallId, Vec<StoneColumn>>,
}

impl StoneColumnCollisionStorage {
    /// `clear_and_get_mut`（O 0xa78880）：记录 cleared，清空该墙列，返回可写引用。
    pub fn clear_and_get_mut(&mut self, id: WallId) -> &mut Vec<StoneColumn> {
        if !self.cleared.contains(&id) {
            self.cleared.push(id);
        }
        let v = self.columns.entry(id).or_default();
        v.clear();
        v
    }
}

/// 石柱变更事件（O 名；布局 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq, bevy_ecs::event::Event)]
pub struct StoneColumnsChanged(pub WallId);
