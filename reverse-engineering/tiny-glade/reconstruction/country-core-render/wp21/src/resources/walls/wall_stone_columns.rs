//! wall_stone_columns.rs — 石柱碰撞存储（WP21）。
//!
//! O：`StoneColumnCollisionStorage::clear_and_get_mut` 0xa78880 ——
//! `{ cleared: Vec<WallId>, map: HashMap<WallId, Vec<StoneColumn>> }`，清空后返回
//! `&mut Vec<StoneColumn>`（插入默认 cap=4）。

use std::collections::HashMap;

use bevy_ecs::resource::Resource;

use crate::boundary::WallId;
use crate::typed_gap::Gap;

/// 单根石柱碰撞数据（O 名；布局 U）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StoneColumn(pub Gap);

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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StoneColumnsChanged(pub Gap);
