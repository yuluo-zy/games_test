//! systems/save/save_backup_metronome.rs — 备份节拍器（WP21）。
//!
//! O：`SaveBackupMetronome`：`add_active_game_time`（save_game 行 18 内联）、
//! `consider_backing_up`（行 23 内联 + closure$0）、`new_from_backup_list`（load_game
//! 行 77 内联）。语义：累积活动游戏时间，达阈值触发备份（关联
//! `SaveBackupList::backup_metronome_counter`，wp13）。内部字段/阈值 U → typed gap。

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// `SaveBackupMetronome`（O：备份节流节拍器；内部布局/阈值 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SaveBackupMetronome {
    /// 自上次备份以来累计活动游戏时间（I：字段语义；U 布局）。
    pub active_game_time: f32,
    /// 未映射区（U；备份列表引用/计数）。
    pub _gap: Gap,
}

impl SaveBackupMetronome {
    /// `new_from_backup_list`（O：load_game 内联 0x140146d030 行 77）。
    pub fn new_from_backup_list(_backup_list: &Gap) -> Self {
        Self::default()
    }
    /// `add_active_game_time`（O：save_game 内联 0x140a3f470 行 18）。
    pub fn add_active_game_time(&mut self, dt: f32) {
        self.active_game_time += dt;
    }
    /// `consider_backing_up`（O：save_game 内联 0x140a3f470 行 23；阈值 U）。
    pub fn consider_backing_up(&self) -> bool {
        // O 语义：达阈值建议备份；具体阈值/间隔 U。
        false
    }
}
