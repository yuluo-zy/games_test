//! 游戏输入的动作模型、同步快照和完整录制协议。
use core::fmt;
use core::time::Duration;

use bevy_ecs::resource::Resource;

pub mod action_set;
pub mod input_stream;
pub mod panic_repro_state;
pub mod recording;

/// 避免设备提示在短时间内来回跳动。
pub const INPUT_MODE_COOLDOWN: Duration = Duration::from_millis(250);

/// 当前输入方式。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum ActiveInputMode {
    #[default]
    MouseKeyboard = 0,
    Controller = 1,
}

impl ActiveInputMode {
    /// 是否使用手柄提示。
    pub fn is_controller(self) -> bool {
        self == Self::Controller
    }
}

/// 同一单调时间起点上的设备活动记录；相等时优先键鼠。
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveInputModeTracker {
    pub controller_deadline: Duration,
    pub mouse_keyboard_deadline: Duration,
}

impl ActiveInputModeTracker {
    /// 默认显示手柄提示。
    pub fn new_default_controller(now: Duration) -> Self {
        Self {
            controller_deadline: now.saturating_add(INPUT_MODE_COOLDOWN),
            mouse_keyboard_deadline: now,
        }
    }

    /// 默认显示键鼠提示。
    pub fn new_default_mouse_keyboard(now: Duration) -> Self {
        Self {
            controller_deadline: now,
            mouse_keyboard_deadline: now.saturating_add(INPUT_MODE_COOLDOWN),
        }
    }

    /// 最近获得优先权的输入设备。
    pub fn mode(&self) -> ActiveInputMode {
        if self.controller_deadline > self.mouse_keyboard_deadline {
            ActiveInputMode::Controller
        } else {
            ActiveInputMode::MouseKeyboard
        }
    }
}

/// 输入录制/回放模式。原生编码转换只保留为兼容工具，不用作 Rust 内存布局。
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
