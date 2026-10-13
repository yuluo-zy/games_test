//! `systems::save::save_game` — 主存档系统（B 骨架对齐，WP13）。
//!
//! O 证据（合同见 `contracts/save-systems.md`）：
//!   - `SaveFileSection` 10 变体 + json_key（0x140a43810 机器码 + .rdata 串块
//!     0x142ace218）与 padding_bytes（0x142ace3c8 表）。
//!   - `save_game` 0x140a3f470（反编译）；`save_game_to_file` 0x141750c90；
//!     `add_history_edit_to_save_file` 0x140a439b0；`AutoSaveState`。
//!   - 事件：`SaveGameCmd` / `AsyncSaveCmd` / `DeleteSessionCmd` / `SavingGameComplete`。

use crate::resources::history::History;
use crate::typed_gap::Gap;

/// 存档文件节（O：10 变体，tag 字节在枚举首字节；json_key/padding 已从机器码恢复）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SaveFileSection {
    Version = 0,
    Identity = 1,
    BackwardCompatibility = 2,
    Camera = 3,
    TimeOfDay = 4,
    Sheep = 5,
    Colors = 6,
    HistoryChainIndex = 7,
    HistoryStart = 8,
    HistorySnapshot = 9,
}

impl SaveFileSection {
    /// O：`json_key` 0x140a43810（tag 字节 → 两张 8B 表；len 表 @0x142ace328，
    /// ptr 表 @0x142ace378，串块 @0x142ace218）。
    pub fn json_key(self) -> &'static str {
        match self {
            SaveFileSection::Version => "Version",
            SaveFileSection::Identity => "Identity",
            SaveFileSection::BackwardCompatibility => "BackwardCompatibility",
            SaveFileSection::Camera => "Camera",
            SaveFileSection::TimeOfDay => "Time of day",
            SaveFileSection::Sheep => "Sheep",
            SaveFileSection::Colors => "Colors",
            SaveFileSection::HistoryChainIndex => "chain_index",
            SaveFileSection::HistoryStart => "history_start",
            SaveFileSection::HistorySnapshot => "snapshot",
        }
    }
    /// O：`padding_bytes` 0x140a43830（@0x142ace3c8 表）：
    /// [0, 16, 0, 128, 32, 128, 64, 8, 8, 2048]。
    pub fn padding_bytes(self) -> u64 {
        match self {
            SaveFileSection::Version => 0,
            SaveFileSection::Identity => 16,
            SaveFileSection::BackwardCompatibility => 0,
            SaveFileSection::Camera => 128,
            SaveFileSection::TimeOfDay => 32,
            SaveFileSection::Sheep => 128,
            SaveFileSection::Colors => 64,
            SaveFileSection::HistoryChainIndex => 8,
            SaveFileSection::HistoryStart => 8,
            SaveFileSection::HistorySnapshot => 2048,
        }
    }
    /// O：方法存在（`save_cooldown_seconds` / `should_trigger_backup` /
    /// `should_trigger_debug_log`）；数值/语义 U。
    pub fn save_cooldown_seconds(self) -> f32 {
        0.0
    }
    pub fn should_trigger_backup(self) -> bool {
        false
    }
    pub fn should_trigger_debug_log(self) -> bool {
        false
    }
}

/// O：`AutoSaveState`（close_file 0x140a43840 / last_error_state 0x140a439a0 /
/// create_backup_of 0x141e59860）。
#[derive(Clone, Debug, Default)]
pub struct AutoSaveState {
    /// 错误状态；is_ok 判定 U。
    pub error: Option<Gap>,
    /// 其余状态（当前打开文件/节）。布局 U。
    pub rest: Gap,
}

impl AutoSaveState {
    pub fn close_file(&mut self) {}
    pub fn last_error_state(&self) -> Option<&Gap> {
        self.error.as_ref()
    }
    pub fn create_backup_of(&self, _folder: &std::path::Path) {}
}

/// O：`GameSaveData`（存档数据载体；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct GameSaveData(pub Gap);

/// O：`SaveFileStructure`（节结构描述；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct SaveFileStructure(pub Gap);

/// O：`DeferredJsonSerialize`（延迟 JSON 序列化；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct DeferredJsonSerialize(pub Gap);

/// O：`SaveGameReplayModTracker`（回放修改跟踪；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct SaveGameReplayModTracker(pub Gap);

/// O：`SaveGameCmd`（存档命令事件）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SaveGameCmd(pub Gap);

/// O：`AsyncSaveCmd`（异步存档命令）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AsyncSaveCmd(pub Gap);

/// O：`DeleteSessionCmd`（删除会话命令）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DeleteSessionCmd(pub Gap);

/// O：`SavingGameComplete`（存档完成事件）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SavingGameComplete(pub Gap);

/// O：`add_history_edit_to_save_file` 0x140a439b0：把当前历史 edit 计数值压入待写
/// 列表（增量写 history 子节）。
pub fn add_history_edit_to_save_file(_history: &History, _pending: &mut Vec<u64>) {
    let _ = _history;
}

/// O：`write_json_sections` / `save_game_to_file` 0x141750c90：逐 SaveFileSection
/// 写 JSON。节序列化细节 U。
pub fn write_json_sections<W: std::io::Write>(
    writer: &mut W,
    sections: &[SaveFileSection],
) -> std::io::Result<()> {
    for section in sections {
        let _ = section.json_key();
        writer.write_all(section.json_key().as_bytes())?;
    }
    Ok(())
}

/// O：`save_game` 0x140a3f470 主入口（反编译）；完整资源清单/节序 U。
pub fn save_game(_history: &History) -> Result<(), Gap> {
    Ok(())
}

/// O：`save_session_meta_to_file`（SessionMeta 落盘）。
pub fn save_session_meta_to_file(_path: &std::path::Path) -> Result<(), std::io::Error> {
    Ok(())
}
