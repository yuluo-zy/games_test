//! B `systems/history/replay_mode.rs` — 回放模式进出检测（RE-HISTORY-WRAPUP §3，O）。
//!
//! replay_mode：AppMode 进出 0xd（Replay）检测 + HoldToFastReplay 长按快进 → redo
//! HistoryCmd。纯判定在此；bevy 系统接线（AppMode 变化事件 + 快进输入）由宿主装配。
use crate::systems::mode_manager::AppMode;

/// AppMode 原生 tag == 0xd（Replay）。
pub fn is_replay_mode(mode: AppMode) -> bool {
    mode.native_tag() == 13
}

/// 进出回放模式（O：进入/退出判定；返回 `Some(true)` 进入、`Some(false)` 退出、
/// `None` 未变化）。
pub fn replay_mode_transition(previous: AppMode, current: AppMode) -> Option<bool> {
    let was = is_replay_mode(previous);
    let is = is_replay_mode(current);
    if was == is {
        None
    } else {
        Some(is)
    }
}

/// HoldToFastReplay 长按快进（输入资源由宿主提供；此处登记阈值语义，I）。
pub const FAST_REPLAY_HOLD_MS: u64 = 500;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::mode_manager::AppMode;

    #[test]
    fn replay_mode_tag_is_thirteen() {
        assert!(is_replay_mode(AppMode::Replay));
        assert!(!is_replay_mode(AppMode::LoadGame));
        assert_eq!(AppMode::Replay.native_tag(), 13);
    }

    #[test]
    fn transitions_detect_entry_and_exit() {
        assert_eq!(
            replay_mode_transition(AppMode::LoadGame, AppMode::Replay),
            Some(true)
        );
        assert_eq!(
            replay_mode_transition(AppMode::Replay, AppMode::LoadGame),
            Some(false)
        );
        assert_eq!(
            replay_mode_transition(AppMode::LoadGame, AppMode::Water),
            None
        );
    }
}
