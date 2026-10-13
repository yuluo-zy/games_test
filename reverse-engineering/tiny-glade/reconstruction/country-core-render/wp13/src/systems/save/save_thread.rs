//! `systems::save::save_thread` — 异步写盘线程（B 骨架对齐，WP13）。
//!
//! O：`save_thread` 0x141720680 / `snapshot_gc_thread` 0x14172b280 /
//! `CurrentSaveFile`（backup_save / replace_edits / perform_manual_backup /
//! change_section / flush_deferred_and_sync）/ `UnsavedFileList::contains_file`
//! 0x1417201a0 / `AutosaveErrorState::is_ok` 0x140b92210。

use crate::typed_gap::Gap;

/// O：`CurrentSaveFile`（当前打开存档；`change_section` / `replace_edits` /
/// `backup_save` / `perform_manual_backup` / `flush_deferred_and_sync`）。
#[derive(Clone, Debug, Default)]
pub struct CurrentSaveFile(pub Gap);

impl CurrentSaveFile {
    pub fn backup_save(&mut self) {}
    pub fn replace_edits(&mut self, _edits: &[Gap]) {}
    pub fn perform_manual_backup(&mut self) {}
    pub fn change_section(&mut self, _section: Gap) {}
    pub fn flush_deferred_and_sync(&mut self) {}
}

/// O：`DeferredSection`（延迟写入节；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct DeferredSection(pub Gap);

/// O：`SnapshotGcCmd`（快照 GC 命令）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SnapshotGcCmd(pub Gap);

/// O：`UnsavedFileList`（未落盘文件集合；`add` / `contains_file` 0x1417201a0）。
#[derive(Clone, Debug, Default)]
pub struct UnsavedFileList(pub Vec<Gap>);

impl UnsavedFileList {
    pub fn add(&mut self, _file: Gap) {}
    pub fn contains_file(&self, _file: &Gap) -> bool {
        false
    }
}

/// O：`AutosaveErrorState::is_ok` 0x140b92210（错误状态判空）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AutosaveErrorState(pub bool);

impl AutosaveErrorState {
    pub fn is_ok(&self) -> bool {
        self.0
    }
}

/// O：`save_thread` 0x141720680 主循环（execute_open_file_cmd / execute_save_cmd /
/// full_write_history_file / capture_snapshot / nuke_unreferenced_shapshots）。
pub fn save_thread(_current: &mut CurrentSaveFile) {}

/// O：`snapshot_gc_thread` 0x14172b280（清理未引用快照）。
pub fn snapshot_gc_thread() {}

/// O：`full_write_history_file`（整写历史文件）。
pub fn full_write_history_file(_history: &Gap) {}

/// O：`nuke_unreferenced_shapshots`（[sic] 原符号；删除未引用快照）。
pub fn nuke_unreferenced_shapshots(_gc: &mut Gap) {}

/// O：`capture_snapshot`（异步快照捕获）。
pub fn capture_snapshot(_snapshot: Gap) {}
