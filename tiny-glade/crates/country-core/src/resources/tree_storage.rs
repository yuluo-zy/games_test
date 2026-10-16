//! Authoritative tree-index → entity owner (B 0x140a38460..0x140a38600).
//! The native table uses std::HashMap; its bucket layout is not game logic.
use bevy_ecs::{component::Component, entity::Entity, resource::Resource};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[repr(transparent)]
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TreeStorageIndex(pub u64);

/// 树存储条目（O：0xa0B 树条目 + 0x48B 树记录族——`+0x10` 打包位置、
/// `+0x38` 树 id、`+0x00` Arc prefab（RE-ANALYSIS-2 task ④）；语义字段承载
/// 实体 + 世界位置）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TreeStorageEntry {
    /// 树实体。
    pub entity: Entity,
    /// 世界位置（O：0x48B 记录 `+0x10` 打包 {x, z} 的解包面）。
    pub position: glam::Vec3,
}

#[derive(Resource, Debug, Default)]
pub struct TreeStorage {
    entries: HashMap<TreeStorageIndex, TreeStorageEntry>,
    next_index: u64,
}
impl TreeStorage {
    pub fn iter(&self) -> impl Iterator<Item = (&TreeStorageIndex, &TreeStorageEntry)> {
        self.entries.iter()
    }
    pub fn get(&self, index: TreeStorageIndex) -> Option<&TreeStorageEntry> {
        self.entries.get(&index)
    }
    /// Native insert returns the previous entry. The counter advances for an
    /// explicit restored index as well as an allocated one; replacement is valid.
    pub fn insert(&mut self, index: TreeStorageIndex, entry: TreeStorageEntry) -> Option<TreeStorageEntry> {
        let previous = self.entries.insert(index, entry);
        self.next_index = self.next_index.max(index.0.wrapping_add(1));
        previous
    }
    pub fn remove(&mut self, index: TreeStorageIndex) -> Option<TreeStorageEntry> {
        self.entries.remove(&index)
    }
    /// 0x140a38600 returns the old counter and increments it with release wrapping.
    pub fn next_index(&mut self) -> TreeStorageIndex {
        let index = self.next_index;
        self.next_index = index.wrapping_add(1);
        TreeStorageIndex(index)
    }
    /// 清空（O 面：`startup_tree_tool::reset_tree_tool` 消费；计数保留策略 I）。
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

// ---------------------------------------------------------------- 组 A 接线

/// 可交互森林树（O：`tree_storage::InteractableForestTree`；forest
/// manage_tree_prefabs 0x141bf16c0 的候选评估面 + system_critters 消费。
/// 字段布局 U——re-analyst-A 契约：树记录 0x50B（{Arc<String> prefab, u64 打包
/// 位置, f32 高, f32 缩放, …}），语义字段名 I）。
#[derive(Component, Clone, Debug, Default)]
pub struct InteractableForestTree {
    /// 树 id（O：0x58B 树命令 {1,1,变体,树 id} 载荷族；I）。
    pub id: u64,
    /// 世界位置（I：pack_2x16f_uint 打包位的语义解）。
    pub position: glam::Vec3,
    /// 缩放（I：fastrand lerp(0.3, 0.75) 族）。
    pub scale: f32,
    /// typed gap：其余字段（U）。
    pub _gap: u64,
}

/// 可交互森林放置位点（O：`tree_storage::InteractableForestPlacements`；
/// manage_tree_prefabs 的 ResMut 写侧（0x50B 树记录 Vec）+ 36B 默认实例
/// （0x142b8fc58 静态初值，re-analyst-A 契约 §2）。字段明细 U。
#[derive(Resource, Clone, Debug, Default)]
pub struct InteractableForestPlacements {
    /// 树记录 Vec（0x50B 条目；语义 I）。
    pub records: Vec<InteractableForestTree>,
}

/// 树移动命令（O：`tree_storage::MoveTreeCmd`；ui_move_tree 0x14226b1a0 发射、
/// process_tree_cmds 0x142267430 消费。载荷语义 U）。
#[derive(bevy_ecs::event::Event, Clone, Debug, Default)]
pub struct MoveTreeCmd {
    /// typed gap。
    pub _gap: u64,
}

/// 区域树隐藏命令（O：`tree_storage::HideTreesinAreaCmd`；hide_trees 0x14226a260
/// 消费——相机圈过滤门（Circle2d::with_expanded_diameter 0x140c980c0 +
/// contains_point 0x140c980f0）；载荷 = {圈中心, 半径}，I 字段名）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct HideTreesinAreaCmd {
    /// 圈中心（世界 xz）。
    pub center: glam::Vec2,
    /// 圈半径。
    pub radius: f32,
}

/// 墙旁树隐藏命令（O：`tree_storage::HideTreesNextToWallCmd`；hide_trees 消费；
/// 载荷 = 墙圈 {中心, 半径}，I）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct HideTreesNextToWallCmd {
    /// 圈中心（世界 xz）。
    pub center: glam::Vec2,
    /// 圈半径。
    pub radius: f32,
}
