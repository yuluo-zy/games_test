//! steam.rs — Steamworks 输入行动集（WP21）。
//!
//! O：`SteamInput::get_action_origin_glyphs` 0x9e6280、`SteamMainActionSet::read_state`
//! 0x9e86b0（4×f32 模拟 + 24 数字字节 → SteamMainActionSetState 0x30B）、
//! `SteamMenuActionSet::read_state` 0x9e8bf0（SteamMenuActionSetState 0x10B）、
//! `is_any_active` 0x9e8aa0、`try_load<Input>` 0x17c3720/0x17c5770。
//! 复用边界：`steamworks` crate（缓存无 → typed gap）。

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 行动句柄（O：steamworks InputHandle；复用边界）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SteamActionSetAction(pub u64);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SteamActionSetDigitalAction(pub u64);

impl SteamActionSetDigitalAction {
    /// `is_analog`（O 名，1 内联）。
    pub fn is_analog(&self) -> bool {
        false
    }
}

/// `SteamMainActionSetState`（O：0x30B = 4×f32 模拟 + 24 数字字节）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SteamMainActionSetState {
    /// 2 个模拟行动（各 x/y；O：4×f32 @+0x0）。
    pub analog: [f32; 4],
    /// 24 个数字行动位（O：@+0x18..0x30）。
    pub digital: [u8; 24],
}

impl SteamMainActionSetState {
    /// `is_any_active`（O 0x9e8aa0）。
    pub fn is_any_active(&self) -> bool {
        self.digital.iter().any(|b| *b != 0) || self.analog.iter().any(|v| *v != 0.0)
    }
}

/// `SteamMenuActionSetState`（O：0x10B = 1×f32 + 9 数字字节）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SteamMenuActionSetState {
    /// 模拟行动（O：f32 @+0x0）。
    pub analog: f32,
    /// 9 个数字行动位（O：@+0x4..0x10）。
    pub digital: [u8; 9],
}

impl SteamMenuActionSetState {
    pub fn is_any_active(&self) -> bool {
        self.digital.iter().any(|b| *b != 0) || self.analog != 0.0
    }
}

/// `SteamMainActionSet`（O：主行动集；read_state 0x9e86b0）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SteamMainActionSet {
    /// 模拟行动句柄（O：action_handle 0x9e8??；steamworks 边界）。
    pub analog: [Gap; 2],
    /// 数字行动句柄（O）。
    pub digital: [Gap; 24],
    /// 行动名（O：name_c_str 内联表）。
    pub state: SteamMainActionSetState,
}

/// `SteamMenuActionSet`（O：菜单行动集；read_state 0x9e8bf0）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SteamMenuActionSet {
    pub state: SteamMenuActionSetState,
}

/// `SteamInput`（O：Steam 输入资源）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SteamInput(pub Gap);

impl SteamInput {
    /// `get_action_origin_glyphs`（O 0x9e6280）：steamworks 取行动原点字形 →
    /// `Vec<(String, …)>`。
    pub fn get_action_origin_glyphs(&self, _action: &SteamActionSetAction) -> Vec<String> {
        Vec::new()
    }
}

/// `SteamMainActionSetDigitalAction` / `AnalogAction` / `SteamMainActionSetAction`：
/// 行动名/句柄类型（O：name_c_str 内联表）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SteamMainActionSetDigitalAction(pub u8);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SteamMainActionSetAnalogAction(pub u8);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SteamMainActionSetAction(pub u8);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SteamMenuActionSetAction(pub u8);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SteamMenuActionSetDigitalAction(pub u8);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SteamMenuActionSetAnalogAction(pub u8);
