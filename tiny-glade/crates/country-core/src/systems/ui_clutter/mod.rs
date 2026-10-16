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

#[cfg(test)]
mod tests;
