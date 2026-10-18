//! `stairs_removed_supports.rs` — 已移除楼梯支撑存档集（B `resources/stairs/
//! stairs_removed_supports.rs`，1 业务全 serde 胶水）。
//!
//! 原 `_::impl$0::serialize<Value::Serializer>`（0x140b3bad0）为 serde derive；
//! `replay_stairs`（0x140af75b0）对 `StairsRemovedSupports` 的增删 key 是 **全边键**
//! `StairGraphEdge`（`HashMap::insert<StairGraphEdge, tuple_<>>` / `remove_entry<StairGraphEdge, ()>`），
//! 值为 `()`（单位）——即"已移除支撑的边键集合"。Rust 旧实现 `HashMap<u32,u32>`（edge1→edge2，
//! erase 只按 edge1）与机器不同构，已按机器码改为 `HashMap<StairGraphEdge, ()>`。
//! 章节：`chapters/resources/stairs/stairs_removed_supports.rs.md`。
use crate::resources::stairs::stairs_state::StairGraphEdge;
use bevy_ecs::resource::Resource;
use std::collections::HashMap;

/// 已移除楼梯支撑存档集（全边键集合；机器码 `HashMap<StairGraphEdge, ()>`）。
#[derive(Resource, Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StairsRemovedSupports {
    /// 边键集合（键=StairGraphEdge、值=()，O：replay_stairs insert/remove_entry key 均为 StairGraphEdge）。
    pub edges: HashMap<StairGraphEdge, ()>,
}

impl StairsRemovedSupports {
    /// undo==0（动画路径）/undo!=0（instant 路径）：记录被移除支撑边（全边键）。
    pub fn record(&mut self, edge: StairGraphEdge) {
        self.edges.insert(edge, ());
    }

    /// undo!=0（动画路径）/undo==0（instant 路径）：从存档集移除边（全边键）。
    pub fn erase(&mut self, edge: &StairGraphEdge) -> bool {
        self.edges.remove(edge).is_some()
    }

    /// 是否已记录该边。
    pub fn contains(&self, edge: &StairGraphEdge) -> bool {
        self.edges.contains_key(edge)
    }
}
