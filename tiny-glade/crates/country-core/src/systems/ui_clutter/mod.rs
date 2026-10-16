//! Automatic clutter deletion authority, the recovered ID/history path and
//! the picking/visible-consumption producers (rmb/ui_autoclutter/debug spawn).
//!
//! 原文件清单（inventory，B 版）：`mod.rs`(default 胶水)、`defered_autoclutter_history.rs`
//! （`history_record_auto_clutter` @0x1409a9160）、`rmb_autoclutter.rs`
//! （`handle_rmb_autoclutter` @0x140976ca0）、`ui_autoclutter.rs`
//! （`ui_convert_autoclutter_to_decoclutter` @0x14099ca60）、`debug_spawn_clutter.rs`
//! （`debug_spawn_all_clutter` @0x1409a1f50）、`decoclutter_show_pins.rs`
//! （`decoclutter_show_pins` @0x1412357c0）、`ui_focus_decoclutter.rs`
//! （`ui_focus_decoclutter` @0x1409a92c0）。
pub mod debug_spawn_clutter;
pub mod decoclutter_show_pins;
pub mod defered_autoclutter_history;
pub mod edit_autoclutter;
pub mod registration;
pub mod rmb_autoclutter;
pub mod ui_autoclutter;
pub mod ui_focus_decoclutter;

use bevy_ecs::prelude::Resource;
use std::collections::HashSet;

/// B 0x140aa4a60 reads one table of 8-byte IDs; the PDB default at source
/// ui_clutter/mod.rs:20 is std::collections::HashSet::default. Folded generic
/// HashMap aliases do not imply additional wall/decorator deletion tables.
#[derive(Resource, Clone, Debug, Default)]
pub struct DeletedAutoClutter(pub HashSet<u64>);

impl DeletedAutoClutter {
    pub fn is_deleted(&self, id: u64) -> bool {
        self.0.contains(&id)
    }
}

/// auto-clutter 实体记录（I：0x58B 步长记录的语义字段；宿主从实体组件表同步）。
/// 字段名按旋转公式（三向量点积 + 开方 + `FUN_1408b83d0`）的输入语义命名；
/// 0x58B 布局字段名待取证。
#[derive(Clone, Copy, Debug)]
pub struct AutoClutterRecord {
    /// 世界位置（0x48B 事件 @+0x18 载荷）。
    pub position: glam::Vec3,
    /// 记录四元数分量 w/x/y（旋转公式输入，O）。
    pub w: f32,
    pub x: f32,
    pub y: f32,
    /// 入口角（`CircularF32::as_f32` 输入，O）。
    pub entry_angle: f32,
    /// 类型字节（@+0x3c 载荷经 ClutterMeshTy::from）。
    pub ty: crate::systems::mode_manager::ClutterMeshTy,
}

/// 右键 probe 目标表（I：host 从实体哈希表维护；`raw_entry` 键查找与
/// FUN_1405b0d70 hashbrown probe 语义对应）。
#[derive(Resource, Clone, Debug, Default)]
pub struct AutoClutterEntityIndex(pub hashbrown::HashMap<u64, AutoClutterRecord>);

#[cfg(test)]
mod tests;
