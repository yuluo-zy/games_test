//! `stairs_removed_supports.rs` — 已移除楼梯支撑存档集（B `resources/stairs/
//! stairs_removed_supports.rs`，1 业务全 serde 胶水）。
//!
//! 原 `_::impl$0::serialize<Value::Serializer>`（0x140b3bad0）为 serde derive；
//! `replay_stairs`（0x140af75b0）undo==0 → hashbrown insert（键=edge1，值=edge2），
//! undo!=0 → `remove_entry(edge1, edge2)`（FUN_1402c8640）——键为边端点。
//! 章节：`chapters/resources/stairs/stairs_removed_supports.rs.md`。
use bevy_ecs::resource::Resource;
use std::collections::HashMap;

/// 已移除楼梯支撑存档集（edge1 → edge2）。
#[derive(Resource, Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StairsRemovedSupports {
    /// 边端点键值（键=edge1、值=edge2，O：replay_stairs remove_entry(edge1, edge2)）。
    pub edges: HashMap<u32, u32>,
}

impl StairsRemovedSupports {
    /// undo==0 路径：记录被移除支撑边（键=edge1、值=edge2）。
    pub fn record(&mut self, edge1: u32, edge2: u32) {
        self.edges.insert(edge1, edge2);
    }

    /// undo!=0 路径：从存档集移除边（remove_entry 键=edge1）。
    pub fn erase(&mut self, edge1: u32) -> bool {
        self.edges.remove(&edge1).is_some()
    }

    /// 是否已记录该边。
    pub fn contains(&self, edge1: u32) -> bool {
        self.edges.contains_key(&edge1)
    }
}
