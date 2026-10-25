//! `systems::save` — 存档系统族（B 骨架对齐，WP13）。
//!
//! O：`add_save_load_systems` / `add_save_load_events_and_resources`（+闭包）。
//! 各子系统见子模块。

use crate::typed_gap::Gap;

pub mod load_completion_checker;
pub mod load_game;
pub mod save_backup_list;
pub mod save_file_identity;
pub mod save_game;
pub mod save_thread;
pub mod shadow_file;
pub mod snapshot;
pub mod snapshot_gc;

/// O：`add_save_load_systems`（注册存档系统）。
pub fn add_save_load_systems() {
    let _ = (
        save_game::save_game,
        load_game::load_game,
        save_thread::save_thread,
        load_completion_checker::check_load_completion,
    );
}

/// O：`add_save_load_events_and_resources`（注册事件/资源）。
pub fn add_save_load_events_and_resources() {
    let _ = (
        save_game::SaveGameCmd(Gap),
        save_game::SavingGameComplete(Gap),
        load_game::LoadingGameComplete(Gap),
    );
}
