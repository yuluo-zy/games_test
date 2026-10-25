//! `systems::save::save_backup_list` — 备份存档列表（B 骨架对齐，WP13）。
//!
//! O：`SaveBackupList::new_from_session` 0x140ae8470（文件名前缀 `history_auto`；
//! 错误日志 “Error reading save backup directory”）、`iter_backup_files`
//! 0x140ae8330、`into_files`、`backup_metronome_counter`；`BackupSaveFileKind::
//! is_corrupted` 0x140ae8320（与 GladeType::is_winter 同址折叠，tag==4 判定）/
//! `is_auto`。

use crate::typed_gap::Gap;

/// 备份文件种类（O：`is_corrupted` 0x140ae8320 与 GladeType::is_winter 同址折叠 =
/// tag 字节 == 4 判定；`is_auto` 判定文件名前缀）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BackupSaveFileKind {
    Auto,
    Manual,
    Corrupted,
    Unknown,
}

impl BackupSaveFileKind {
    /// O：`is_corrupted`（tag==4 判定，O）。
    pub fn is_corrupted(self) -> bool {
        matches!(self, BackupSaveFileKind::Corrupted)
    }
    /// O：`is_auto`（PDB 名；判定规则 U）。
    pub fn is_auto(self) -> bool {
        matches!(self, BackupSaveFileKind::Auto)
    }
}

/// O：`SaveBackupList`（备份文件列表；`new_from_session` 0x140ae8470 /
/// `iter_backup_files` 0x140ae8330 / `into_files` / `backup_metronome_counter`）。
#[derive(Clone, Debug, Default)]
pub struct SaveBackupList {
    /// O：文件名前缀 `history_auto`（备份节流节奏 U）。
    pub files: Vec<Gap>,
}

impl SaveBackupList {
    /// O：`new_from_session` 0x140ae8470（扫备份目录）。
    pub fn new_from_session(_session: &Gap) -> Self {
        Self::default()
    }
    /// O：`iter_backup_files` 0x140ae8330。
    pub fn iter_backup_files(&self) -> impl Iterator<Item = &Gap> {
        self.files.iter()
    }
    /// O：`into_files`。
    pub fn into_files(self) -> Vec<Gap> {
        self.files
    }
    /// O：`backup_metronome_counter`（备份节流计数；语义 U）。
    pub fn backup_metronome_counter(&self) -> u64 {
        0
    }
}
