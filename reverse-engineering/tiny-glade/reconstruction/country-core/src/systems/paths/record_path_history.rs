//! record_path_history.rs — 路径编辑历史记录（O 0x140b1c270）。
//!
//! 源 `systems/paths/record_path_history.rs`；命令实体：
//! `RecordPathEditToHistoryCmd` / `RecordCobblestone1/2/3EditToHistoryCmd`。
//! 路径绘制结束 → 记录编辑到撤销历史。

#![allow(dead_code)]

/// 记录路径编辑命令（O：PDB 实体；负载 U）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct RecordPathEditToHistoryCmd(pub crate::Gap);

/// 记录 cobblestone1 编辑命令（O）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct RecordCobblestone1EditToHistoryCmd(pub crate::Gap);

/// 记录 cobblestone2 编辑命令（O）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct RecordCobblestone2EditToHistoryCmd(pub crate::Gap);

/// 记录 cobblestone3 编辑命令（O）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct RecordCobblestone3EditToHistoryCmd(pub crate::Gap);

/// `record_path_history`（O 0x140b1c270）：消费上述命令写入历史。
pub fn record_path_history(_world: &mut bevy_ecs::world::World) {}
