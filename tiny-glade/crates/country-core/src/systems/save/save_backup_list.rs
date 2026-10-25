//! B 0x140ae8330, 0x140ae8470 and filter-map closure 0x14166df80.
use crate::resources::session::SessionId;
use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackupSaveFileKind {
    Auto = 0,
    Manual = 1,
    Recovered = 2,
    Other = 3,
    Corrupted = 4,
}
impl BackupSaveFileKind {
    pub const fn is_auto(self) -> bool {
        matches!(self, Self::Auto)
    }
    pub const fn is_corrupted(self) -> bool {
        matches!(self, Self::Corrupted)
    }
    fn from_path(path: &Path) -> Self {
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            return Self::Other;
        };
        if name.starts_with("history-manual-") {
            Self::Manual
        } else if name.starts_with("history-recovered-") {
            Self::Recovered
        } else if name.starts_with("history-auto-") {
            Self::Auto
        } else if name.starts_with("corrupted-") {
            Self::Corrupted
        } else {
            Self::Other
        }
    }
}

#[derive(Clone, Debug)]
pub struct BackupSaveFile {
    pub path: PathBuf,
    pub modified: SystemTime,
    pub kind: BackupSaveFileKind,
}

#[derive(Clone, Debug, Default)]
pub struct SaveBackupList {
    files: Vec<BackupSaveFile>,
    backup_metronome_counter: u32,
}

/// 提取文件名末尾的数字计数器（E/reviewer：每个入选条目的数字后缀取 max(n+1)）。
/// 先剥已知前缀（history-auto-/history-manual-/history-recovered-/corrupted-），再取末尾
/// 连续数字。无数字后缀返回 None。
fn trailing_counter(stem: &str) -> Option<u32> {
    let rest = stem
        .strip_prefix("history-auto-")
        .or_else(|| stem.strip_prefix("history-manual-"))
        .or_else(|| stem.strip_prefix("history-recovered-"))
        .or_else(|| stem.strip_prefix("corrupted-"))
        .unwrap_or(stem);
    // 末尾连续数字（解析 u32；非法数字名返回 None，与元数据跳过语义一致）。
    let digits_end = rest.len();
    let digits_start = rest
        .char_indices()
        .rev()
        .take_while(|(_, c)| c.is_ascii_digit())
        .last()
        .map(|(i, _)| i)?;
    rest.get(digits_start..digits_end)?.parse::<u32>().ok()
}
impl SaveBackupList {
    /// read_dir failure propagates; unreadable entries/metadata/modification
    /// times are skipped. Only regular .json and .bak files are accepted.
    pub fn iter_backup_files(directory: &Path) -> Result<impl Iterator<Item = BackupSaveFile>> {
        let entries = fs::read_dir(directory).context("Error reading save backup directory")?;
        Ok(entries.filter_map(|entry| {
            let entry = entry.ok()?;
            let metadata = entry.metadata().ok()?;
            if !metadata.is_file() {
                return None;
            }
            let path = entry.path();
            if !matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("json" | "bak")
            ) {
                return None;
            }
            let modified = metadata.modified().ok()?;
            let kind = BackupSaveFileKind::from_path(&path);
            Some(BackupSaveFile {
                path,
                modified,
                kind,
            })
        }))
    }
    pub fn new_from_session(session: SessionId) -> Self {
        Self::new_from_directory(&session.backup_folder_path())
    }
    pub fn new_from_directory(directory: &Path) -> Self {
        let mut list = Self::default();
        if !directory.exists() {
            if let Err(error) = fs::create_dir_all(directory) {
                log::error!("Error creating backup save folder: {error}");
            }
            return list;
        }
        let files = match Self::iter_backup_files(directory) {
            Ok(files) => files,
            Err(error) => {
                log::error!("{error:#}");
                return list;
            }
        };
        for file in files {
            if let Some(stem) = file.path.file_stem() {
                let stem = stem.to_str().expect("backup save filename is not UTF-8");
                // E/reviewer 定案：原版对**每个入选条目**的数字后缀取 max(n+1)，不限于
                // history-auto-（0x140ae8470 机器码；uStack_74=max(prev, parsed+1)）。
                if let Some(counter) = trailing_counter(stem) {
                    list.backup_metronome_counter =
                        list.backup_metronome_counter.max(counter.wrapping_add(1));
                }
            }
            list.files.push(file);
        }
        list
    }
    pub fn files(&self) -> &[BackupSaveFile] {
        &self.files
    }
    pub fn into_files(self) -> Vec<BackupSaveFile> {
        self.files
    }
    pub const fn backup_metronome_counter(&self) -> u32 {
        self.backup_metronome_counter
    }
}
