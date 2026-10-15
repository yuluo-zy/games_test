//! 顶层 `input` 模块（B `input::{action_set, input_stream, mod, panic_repro_state}`）。
//!
//! 证据（`input/ALGORITHMS.md` §2.6、§3，汇编逐条核对）：
//! - `ActiveInputModeTracker`（0x20 B）= `controller_deadline: Instant @0` + `mouse_keyboard_deadline: Instant @0x10`；
//!   `new_default_controller` 0x140958eb0 = `{controller: now + 250ms, mouse_keyboard: now}`，
//!   `new_default_mouse_keyboard` 0x140958ef0 对调；冷却常量是 `Duration { secs: 0, nanos: 0x0ee6b280 }`；
//!   `mode` 0x140958f30 = `controller_deadline > mouse_keyboard_deadline`（先比秒再比纳秒）；
//! - `ActiveInputMode`（u8）：`MouseKeyboard = 0`、`Controller = 1`；`is_controller` 0x1408c5680 返回判别字节；
//! - `InputRecordReplayMode` 5 个变体名取自 `Display`（0x14094f2d0）：`AutoRecordPanicRepro`、`Records`、`Replay`、
//!   `DisableInput`、`None`（此前文档的 Idle/Replaying/Recording/Finished 命名不对）。
//!
//! `Instant` 在原版里是 Windows 的 `{secs, nanos}`；这里用相对某个起点的 `Duration`，由调用方提供 `now`，便于测试。
use core::fmt;
use core::time::Duration;

pub mod action_set;
pub mod input_stream;
pub mod panic_repro_state;

/// 控制器/键鼠切换的冷却时间（O：`0x0ee6b280` ns = 250 ms）。
pub const INPUT_MODE_COOLDOWN: Duration = Duration::from_nanos(0x0ee6_b280);

/// 当前输入方式（O：u8 判别）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum ActiveInputMode {
    #[default]
    MouseKeyboard = 0,
    Controller = 1,
}

impl ActiveInputMode {
    /// 0x1408c5680。
    pub fn is_controller(self) -> bool {
        self == Self::Controller
    }
}

/// 输入方式跟踪器。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveInputModeTracker {
    pub controller_deadline: Duration,
    pub mouse_keyboard_deadline: Duration,
}

impl ActiveInputModeTracker {
    /// 0x140958eb0。
    pub fn new_default_controller(now: Duration) -> Self {
        Self {
            controller_deadline: now + INPUT_MODE_COOLDOWN,
            mouse_keyboard_deadline: now,
        }
    }

    /// 0x140958ef0。
    pub fn new_default_mouse_keyboard(now: Duration) -> Self {
        Self {
            controller_deadline: now,
            mouse_keyboard_deadline: now + INPUT_MODE_COOLDOWN,
        }
    }

    /// 0x140958f30：截止更晚的一方是当前输入方式；相等时是键鼠。
    pub fn mode(&self) -> ActiveInputMode {
        if self.controller_deadline > self.mouse_keyboard_deadline {
            ActiveInputMode::Controller
        } else {
            ActiveInputMode::MouseKeyboard
        }
    }
}

/// 输入录制/回放模式（O：niche 存储，变体 i = `i64::MIN + i`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputRecordReplayMode {
    AutoRecordPanicRepro = 0,
    Records = 1,
    Replay = 2,
    DisableInput = 3,
    None = 4,
}

impl InputRecordReplayMode {
    /// 从 niche 存储值还原。
    pub fn from_niche(raw: u64) -> Option<Self> {
        match raw.wrapping_sub(0x8000_0000_0000_0000) {
            0 => Some(Self::AutoRecordPanicRepro),
            1 => Some(Self::Records),
            2 => Some(Self::Replay),
            3 => Some(Self::DisableInput),
            4 => Some(Self::None),
            _ => Option::None,
        }
    }

    pub fn to_niche(self) -> u64 {
        0x8000_0000_0000_0000 + self as u64
    }
}

impl fmt::Display for InputRecordReplayMode {
    /// 0x14094f2d0 的名字串（0x142ab3ad0 起，长 20/7/6/12/4）。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::AutoRecordPanicRepro => "AutoRecordPanicRepro",
            Self::Records => "Records",
            Self::Replay => "Replay",
            Self::DisableInput => "DisableInput",
            Self::None => "None",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooldown_is_250_milliseconds() {
        assert_eq!(INPUT_MODE_COOLDOWN, Duration::from_millis(250));
    }

    #[test]
    fn default_controller_tracker_reports_controller_and_the_mirror_reports_mouse() {
        let now = Duration::from_secs(100);
        let controller = ActiveInputModeTracker::new_default_controller(now);
        assert_eq!(controller.mode(), ActiveInputMode::Controller);
        assert!(controller.mode().is_controller());
        let mouse = ActiveInputModeTracker::new_default_mouse_keyboard(now);
        assert_eq!(mouse.mode(), ActiveInputMode::MouseKeyboard);
        assert!(!mouse.mode().is_controller());
    }

    #[test]
    fn equal_deadlines_mean_mouse_keyboard_and_nanos_break_second_ties() {
        let t = Duration::new(5, 100);
        let tie = ActiveInputModeTracker {
            controller_deadline: t,
            mouse_keyboard_deadline: t,
        };
        assert_eq!(tie.mode(), ActiveInputMode::MouseKeyboard);
        let later_nanos = ActiveInputModeTracker {
            controller_deadline: Duration::new(5, 101),
            mouse_keyboard_deadline: t,
        };
        assert_eq!(later_nanos.mode(), ActiveInputMode::Controller);
        let later_secs = ActiveInputModeTracker {
            controller_deadline: Duration::new(4, 999_999_999),
            mouse_keyboard_deadline: t,
        };
        assert_eq!(later_secs.mode(), ActiveInputMode::MouseKeyboard);
    }

    #[test]
    fn replay_modes_use_the_niche_encoding_and_display_names() {
        for (i, name) in [
            "AutoRecordPanicRepro",
            "Records",
            "Replay",
            "DisableInput",
            "None",
        ]
        .iter()
        .enumerate()
        {
            let mode = InputRecordReplayMode::from_niche(0x8000_0000_0000_0000 + i as u64).unwrap();
            assert_eq!(mode.to_string(), *name);
            assert_eq!(mode.to_niche(), 0x8000_0000_0000_0000 + i as u64);
        }
        assert_eq!(
            InputRecordReplayMode::from_niche(0x8000_0000_0000_0005),
            Option::None
        );
        assert_eq!(InputRecordReplayMode::Replay.to_string().len(), 6);
    }
}
