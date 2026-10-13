//! `systems::history::boundary` — wp13 History 核心边界 facade（WP17 独立门禁）。
//!
//! 真实 `History`（edit_chain/snapshots/pending/cursor/history_start/nonce）
//! 在共享 `country-core::resources::history`（wp13 恢复）。本独立 crate 不重复
//! 实现，仅以 typed-gap facade 供回放系统引用；集成时由 owner 替换为共享类型。

use crate::typed_gap::Gap;

/// wp13 `History` 资源边界（U：真实字段在共享 crate）。
#[derive(Clone, Debug, Default)]
pub struct HistoryFacade(pub Gap);

/// 应用模式（O：观察 0xd=Replay）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AppMode {
    #[default]
    Unknown,
    Replay,
    Other(u32),
}

impl AppMode {
    pub const REPLAY_TAG: u32 = 0xd;
    pub fn tag(&self) -> u32 {
        match self {
            AppMode::Replay => Self::REPLAY_TAG,
            AppMode::Other(t) => *t,
            AppMode::Unknown => u32::MAX,
        }
    }
    pub fn from_tag(tag: u32) -> Self {
        if tag == Self::REPLAY_TAG {
            AppMode::Replay
        } else {
            AppMode::Other(tag)
        }
    }
    pub fn is_replay(&self) -> bool {
        *self == AppMode::Replay
    }
}

/// `HistoryCmd` 命令字边界（O：4 字节；byte[1]==0 Undo / ==1 Redo / ==4 或
/// byte[0]==2 Interrupt；wp13 已实现 0x140af5b30）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct HistoryCmdFacade(pub u32);

impl HistoryCmdFacade {
    pub fn undo() -> Self {
        Self(0)
    }
    pub fn redo() -> Self {
        Self(1)
    }
    pub fn interrupt() -> Self {
        Self(0x0200_0200)
    }
    pub fn sub(&self) -> u8 {
        self.0.to_le_bytes()[1]
    }
    pub fn family(&self) -> u8 {
        self.0.to_le_bytes()[0]
    }
}

/// `PendingHistoryCommands` 边界（wp13：待处理回放命令）。
#[derive(Clone, Debug, Default)]
pub struct PendingHistoryCommandsFacade(pub Vec<Gap>);

impl PendingHistoryCommandsFacade {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// 快照负载边界（wp13 `Arc<Mutex<Gap>>`；U）。
pub type SnapshotPayloadBoundary = Gap;
