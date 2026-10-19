//! B WindowSize stores two u32 values (set 0x140949c00). as_vec2
//! (0x140949c10) zero-extends and converts each to f32; it is not a bitcast.

use bevy_ecs::event::Event;
use bevy_ecs::resource::Resource;
use glam::Vec2;

use crate::render::render_config::{MonitorMatch, VideoMode};

/// B camera_mouse_ray@1408ef2f8 emits an EventId-only (8B) record.
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct AudioCursorMoved;

/// `AvailableMonitors`（B `country_core::resources::window::AvailableMonitors`）。
///
/// fresh 证据：`AvailableMonitors::clear` 0x140949C40 ——
/// `self+0x08 = Vec ptr`、`self+0x10 = Vec len`（清 len，元素 stride 0x40）；
/// PDB 关系 `grow_one<AvailableMonitor>`、`drop_in_place<AvailableMonitor>`。
/// 即 `struct AvailableMonitors { monitors: Vec<AvailableMonitor> }`。
#[derive(Resource, Clone, Debug, Default)]
pub struct AvailableMonitors {
    pub monitors: Vec<AvailableMonitor>,
}

impl AvailableMonitors {
    /// `AvailableMonitors::clear` 0x140949C40：`Vec::clear`（len=0，逐元素 drop）。
    pub fn clear(&mut self) {
        self.monitors.clear();
    }
}

/// `AvailableMonitor`（B `country_core::resources::window::AvailableMonitor`）。
///
/// fresh 证据（clear 0x140949C40 的 drop 循环，元素 0x40 B）：
/// `Vec<VideoMode>` 在 +0x00（cap @+0、ptr @+8、len @+0x10；元素 16 B），
/// `MonitorMatch` 在 +0x18（name String 0x18..0x30、size @+0x30、position @+0x38）。
/// 声明顺序按内存布局落库（原源码声明顺序 I）。
#[derive(Clone, Debug, Default)]
pub struct AvailableMonitor {
    pub video_modes: Vec<VideoMode>,
    pub monitor: MonitorMatch,
}


/// `WindowFocusEvent`（B `country_core::resources::window::WindowFocusEvent`，O：
/// switch_toolbar_category_button 0x1423427f0 / render_taffy_ui_to_widgets 0x142338990
/// 以 `grow_one<EventInstance<WindowFocusEvent>>` 写 `{id u64@+0, 载荷字节@+8}`，
/// stride 0x10）。载荷 = 单字节（I：类别/焦点字节，消费语义 U）。`finish_timelapse`
/// 0x140a8a460 写失焦 `{id, u8=0}`。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WindowFocusEvent(pub u8);

/// B main_loop drains this unit command into GameEvent tag 2; the OS handler
/// at 0x14026e9b5 toggles the current minimized state. Variant spelling is not
/// retained in PDB; the payload is zero sized.
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct OsWindowCmd;

/// 工具消费的输入空间光标位移；由现有 `ActionSetStates::analog[0]` 非零时产生。
/// B `update_cursor_position` inline @0x14173db3e 读取原+0x140 Vec2，
/// @0x14173db95 写入此事件；捕获期间仍发送位移，零向量不发送，NaN 按非零处理。
/// 窗口位置另由 analog[1] 更新到 CursorPositionSS；事件载荷不包含绝对位置。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct CursorMovedScreenSpace(pub Vec2);

#[repr(C)]
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl WindowSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn set(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn as_dims(self) -> [u32; 2] {
        [self.width, self.height]
    }

    pub fn as_vec2(self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }

    pub fn aspect_ratio(self) -> f32 {
        aspect_ratio_observed(self.width, self.height)
    }
}

/// 对应原函数从 RCX+0、RCX+4 读取的数值。
/// Preserve x86's negative quiet NaN for 0/0 on targets with a positive NaN.
#[inline(never)]
pub fn aspect_ratio_observed(field_0x00: u32, field_0x04: u32) -> f32 {
    let ratio = (field_0x00 as f32) / (field_0x04 as f32);
    if ratio.is_nan() {
        f32::from_bits(0xffc0_0000)
    } else {
        ratio
    }
}
