//! ivy_leaf_spawner.rs — 常春藤叶生成（O 0x140996080）。
//!
//! 源 `systems/ivy/ivy_leaf_spawner.rs`；消息 `IvySegmentDoneGrowingMsg`。
//! 沿末段（`Ivy::last_segment_point_ids` + `get_point_id`）按间距生成叶
//! （`IvyLeafStorage::add`）；`IvySegmentDoneGrowingMsg` 通知叶生成时机。

#![allow(dead_code)]

/// 段生长完成消息（O：`IvySegmentDoneGrowingMsg`）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct IvySegmentDoneGrowingMsg(pub crate::Gap);

/// `ivy_leaf_spawner`（O 0x140996080）。
pub fn ivy_leaf_spawner(_world: &mut bevy_ecs::world::World) {}
