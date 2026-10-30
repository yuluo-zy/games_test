//! 石柱碰撞存储（walls/wall_stone_columns.rs）。
//!
//! O：`StoneColumnCollisionStorage::clear_and_get_mut` 0xa78880 ——
//! `{ cleared: Vec<WallId>, map: HashMap<WallId, Vec<StoneColumn>> }`；
//! `maintain_stone_column_colliders`（0x14078a6e0）消费。
use bevy_ecs::resource::Resource;
use std::collections::HashMap;

use crate::resources::walls::WallId;

/// 石柱碰撞几何：原构造器将 Rectangle3d 写入每项 32 字节的列向量。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StoneColumn {
    pub bounds: utils::geometry::rectangle::Rectangle3d,
}

/// 石柱碰撞存储（O：clear_and_get_mut 0xa78880）。
#[derive(Resource, Debug, Default)]
pub struct StoneColumnCollisionStorage {
    /// 本帧已清的墙 id 列表（保留重复与顺序）（O：先 push 后清 map）。
    pub cleared: Vec<WallId>,
    /// 每墙石柱列表（O：HashMap<WallId, Vec<StoneColumn>>）。
    pub columns: HashMap<WallId, Vec<StoneColumn>>,
    /// 碰撞句柄由此存储独占，随墙身份更新和删除。
    #[cfg(feature = "native-physics")]
    pub colliders: HashMap<WallId, crate::systems::collision::world::ColliderMeshInstance>,
}

impl StoneColumnCollisionStorage {
    /// `clear_and_get_mut`（O 0xa78880）：依次追加墙 ID、清空该墙列，返回可写引用。
    pub fn clear_and_get_mut(&mut self, id: WallId) -> &mut Vec<StoneColumn> {
        self.cleared.push(id);
        let v = self.columns.entry(id).or_default();
        v.clear();
        v
    }
}

/// 石柱碰撞发生变更时发送的零大小事件。
#[derive(Clone, Copy, Debug, Default, PartialEq, bevy_ecs::event::Event)]
pub struct StoneColumnsChanged;
