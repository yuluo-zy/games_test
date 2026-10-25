//! `resources/steam.rs` — Steam 动作集定义与 SteamInput 资源。
//!
//! 原文件（B）：`crates/country-core/src/resources/steam.rs`（22 原函数：业务 9 / 内联 9 / 胶水 4）。
//! 本文件为逐函数 fresh 反编译后的重建（2026-10-17，rebuild-steam）。
//!
//! 结构（O，fresh 反编译 + RNE 机器码）：
//! - `SteamMainActionSet`（0xf0 B）：`loaded u64 @0` + `action_set_handle @8` +
//!   `analog [u64;3] @0x10` + `digital [u64;25] @0x28`。try_load 0x1417c3720 按动作名表
//!   （GameControls：CameraPan/cursor_move/camera_height + 25 数字）逐名查句柄。
//! - `SteamMainActionSetState`（0x31 B）：`analog [Vec2;3] @0` + `digital [bool;25] @0x18`。
//!   read_state 0x1409e86b0 逐句柄读模拟/数字数据写入。
//! - `SteamMenuActionSet`（0x68 B）：`loaded u64 @0` + `action_set_handle @8` +
//!   `analog [u64;1] @0x10` + `digital [u64;9] @0x18`。try_load 0x1417c5770 按 Menu 集
//!   （cursor_move + main_menu/ui_cancel/ui_confirm/ui_up/ui_left/ui_right/ui_down/
//!   ui_tab_prev/ui_tab_next）。
//! - `SteamMenuActionSetState`（0x11 B）：`analog [Vec2;1] @0` + `digital [bool;9] @8`。
//! - `SteamInput`：手柄状态、两个动作集、核心动作→Steam来源映射。旧上帧位图/计数解释已纠正。
//!
//! `is_any_active` 0x1409e8aa0（RNE 机器码核验）：任一 analog `max(|x|,|y|) >= 0.25`
//! （阈值 0x142925950 = `0000803e` = f32 0.25；NaN 不触发 ucomiss 无序遍历）或任一
//! digital 为真 → true；全不触发 → false。
//!
//! 库外 Steam SDK 绑定（原 Cargo.lock `steamworks` 0.11.0 git 依赖）经 [`SteamSdk`]
//! trait 注入，本模块只审调用语义。

use std::collections::HashMap;
use std::sync::Arc;

use bevy_ecs::resource::Resource;
use glam::Vec2;

/// 库外 steamworks SDK 注入边界（原 `steamworks::ClientManager` 的等价面）。
/// 主机提供实现（实际 SDK 绑定在库外接线）。
pub trait SteamSdk: Send + Sync {
    /// `client.user().steam_id()`（0x140ae96f0）。
    fn user_steam_id(&self) -> u64;
    /// `input.run_frame()`（0x140ae9740）。
    fn run_frame(&self);
    /// `input.get_connected_controllers()`（0x140ae9740）。
    fn connected_controllers(&self) -> Vec<u64>;
    /// `input.get_action_set_handle(name)`。
    fn action_set_handle(&self, name: &str) -> Option<u64>;
    /// `input.get_analog_action_handle(name)`。
    fn analog_action_handle(&self, name: &str) -> Option<u64>;
    /// `input.get_digital_action_handle(name)`。
    fn digital_action_handle(&self, name: &str) -> Option<u64>;
    /// `input.get_analog_action_data(controller, handle)`。
    fn analog_action_data(&self, controller: u64, handle: u64) -> AnalogActionData;
    /// `input.get_digital_action_data(controller, handle)`。
    fn digital_action_data(&self, controller: u64, handle: u64) -> DigitalActionData;
    /// `input.activate_action_set(set, controller)`。
    fn activate_action_set(&self, set: u64, controller: u64);
    /// `input.init()`（SteamAPI_ISteamInput_Init）。
    fn init_input(&self) -> bool;
    /// `utils().is_steam_running_on_steam_deck()`。
    fn is_steam_running_on_steam_deck(&self) -> bool;
    /// `input.show_binding_panel(controller)`。
    fn show_binding_panel(&self, controller: u64);
    /// `friends().activate_game_overlay_to_web_page(url)`。
    fn activate_game_overlay_to_web_page(&self, url: &str);
    /// `screenshots().hook_screenshots(hook)`。
    fn hook_screenshots(&self, hook: bool);
    /// `register_callback::<ScreenshotRequested>`（add_steam_integration 0x140945320，回调体
    /// 0x14091ad50）。库外 SDK 绑定在收到 Steam 截图键事件时调用 `callback`。默认实现不注册
    /// （宿主无该绑定时 Steam 截图键不会产生请求，不伪造回调）。
    fn register_screenshot_requested(&self, _callback: Arc<dyn Fn() + Send + Sync>) {}
    /// `screenshots().add_screenshot_to_library(path, None, width, height)`
    /// （`AddScreenshotToLibrary`，0x1402c13d0 内调用；`path_to_absolute_cstring` 0x140b84ff0）。
    /// 默认实现返回 Err：宿主无 Steam 截图绑定时不得伪造成功。
    fn add_screenshot_to_library(
        &self,
        _path: &std::path::Path,
        _width: u32,
        _height: u32,
    ) -> Result<(), String> {
        Err("host steam SDK has no AddScreenshotToLibrary binding".to_string())
    }
    /// `utils().show_floating_gamepad_text_input(mode, x, y, width, height)`。
    fn show_floating_gamepad_text_input(&self, mode: u32, x: i32, y: i32, width: i32, height: i32);
    /// `input.get_analog_action_origins(controller, set, handle)`。
    fn analog_action_origins(&self, controller: u64, set: u64, handle: u64) -> Vec<u64>;
    /// `input.get_digital_action_origins(controller, set, handle)`。
    fn digital_action_origins(&self, controller: u64, set: u64, handle: u64) -> Vec<u64>;
    /// `input.get_glyph_for_action_origin(origin)`（C 串→String）。
    fn glyph_for_action_origin(&self, origin: u64) -> String;
}

/// 模拟动作数据（steamworks `AnalogActionData`：Vec2 值 + active 位）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AnalogActionData {
    pub value: Vec2,
    pub active: bool,
}

/// 数字动作数据（steamworks `DigitalActionData`）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DigitalActionData {
    pub state: bool,
    pub active: bool,
}

/// 原数字动作源二元组的安全枚举：低字节0=GameControls、1=Menu，
/// 高字节为该动作集原句柄表下标。原9e8da0加1转换为图标查询码。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SteamActionSetDigitalAction {
    LeftClick = 0x0000,
    RightClick = 0x0100,
    Undo = 0x0200,
    Redo = 0x0300,
    ToolBuilding = 0x0400,
    ToolPath = 0x0500,
    ToolDecorator = 0x0600,
    ToolCustomize = 0x0700,
    ToolEraser = 0x0800,
    ToolTerrain = 0x0900,
    ToolGarden = 0x0a00,
    ToolTree = 0x0b00,
    ToolWater = 0x0c00,
    TimeOfDay = 0x0d00,
    Photomode = 0x0e00,
    ToggleSnapping = 0x0f00,
    ZoomIn = 0x1000,
    ZoomOut = 0x1100,
    MainMenu = 0x1200,
    UiCancel = 0x1300,
    UiConfirm = 0x1400,
    UiUp = 0x1500,
    UiLeft = 0x1600,
    UiRight = 0x1700,
    UiDown = 0x1800,
    MenuMainMenu = 0x0001,
    MenuUiCancel = 0x0101,
    MenuUiConfirm = 0x0201,
    MenuUiUp = 0x0301,
    MenuUiLeft = 0x0401,
    MenuUiRight = 0x0501,
    MenuUiDown = 0x0601,
    MenuUiTabPrev = 0x0701,
    MenuUiTabNext = 0x0801,
}

impl SteamActionSetDigitalAction {
    /// 压入码（u16 判别值）。
    pub fn code(self) -> u16 {
        self as u16
    }
}

/// 动作类型→本帧动作 分组的键（原 `country_core::input::action_set::DigitalActionSetActionType`）。
/// 本域重建用 u16 承载；消费端按码对账后并入 `input::action_set`。
pub type DigitalActionSetActionType = u16;

/// 主动作集句柄表（GameControls）。
#[derive(Clone, Copy, Debug, Default)]
pub struct SteamMainActionSet {
    /// 0：未加载成功（try_load 失败时 `*out = 0`）。
    pub loaded: u64,
    pub action_set_handle: u64,
    pub analog: [u64; 3],
    pub digital: [u64; 25],
}

impl SteamMainActionSet {
    /// 动作名表（try_load 0x1417c3720 逐名查句柄，串长已按反编译 local_f8 核对）。
    pub const ANALOG_NAMES: [&'static str; 3] = ["CameraPan", "cursor_move", "camera_height"];
    pub const DIGITAL_NAMES: [&'static str; 25] = [
        "left_click",
        "right_click",
        "undo",
        "redo",
        "tool_building",
        "tool_path",
        "tool_decorator",
        "tool_customize",
        "tool_eraser",
        "tool_terrain",
        "tool_garden",
        "tool_tree",
        "tool_water",
        "time_of_day",
        "photomode",
        "toggle_snapping",
        "zoom_in",
        "zoom_out",
        "main_menu",
        "ui_cancel",
        "ui_confirm",
        "ui_up",
        "ui_left",
        "ui_right",
        "ui_down",
    ];

    /// try_load 0x1417c3720：动作集句柄查不到 → `loaded = 0`（false）；否则逐名查
    /// 3 analog + 25 digital 句柄（查不到仅告警，不回滚），`loaded = 1`。
    pub fn try_load(&mut self, input: &dyn SteamSdk) -> bool {
        let Some(set_handle) = input.action_set_handle("GameControls") else {
            return false;
        };
        self.loaded = 1;
        self.action_set_handle = set_handle;
        for (i, name) in Self::ANALOG_NAMES.iter().enumerate() {
            let handle = input.analog_action_handle(name);
            if handle.is_none() {
                log::error!("Failed to get analog action handle {}", name);
            }
            self.analog[i] = handle.unwrap_or(0);
        }
        for (i, name) in Self::DIGITAL_NAMES.iter().enumerate() {
            let handle = input.digital_action_handle(name);
            if handle.is_none() {
                log::error!("Failed to get digital action handle {}", name);
            }
            self.digital[i] = handle.unwrap_or(0);
        }
        true
    }

    /// read_state 0x1409e86b0：逐句柄读 3 模拟 + 25 数字 → 状态结构。
    pub fn read_state(&self, input: &dyn SteamSdk, controller: u64, out: &mut SteamMainActionSetState) {
        for (i, handle) in self.analog.iter().enumerate() {
            out.analog[i] = input.analog_action_data(controller, *handle).value;
        }
        for (i, handle) in self.digital.iter().enumerate() {
            out.digital[i] = input.digital_action_data(controller, *handle).state;
        }
    }
}

/// 主动作集状态。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SteamMainActionSetState {
    pub analog: [Vec2; 3],
    pub digital: [bool; 25],
}

impl SteamMainActionSetState {
    /// is_any_active 0x1409e8aa0（RNE 机器码核验）：任一 analog `max(|x|,|y|) >= 0.25`
    /// 或任一 digital 为真 → true。
    pub fn is_any_active(&self) -> bool {
        self.analog
            .iter()
            .any(|v| v.x.abs().max(v.y.abs()) >= 0.25)
            || self.digital.iter().any(|&b| b)
    }
}

/// 菜单动作集句柄表（Menu）。
#[derive(Clone, Copy, Debug, Default)]
pub struct SteamMenuActionSet {
    pub loaded: u64,
    pub action_set_handle: u64,
    pub analog: [u64; 1],
    pub digital: [u64; 9],
}

impl SteamMenuActionSet {
    /// try_load 0x1417c5770：Menu 集，1 模拟（cursor_move）+ 9 数字。
    pub const ANALOG_NAMES: [&'static str; 1] = ["cursor_move"];
    pub const DIGITAL_NAMES: [&'static str; 9] = [
        "main_menu",
        "ui_cancel",
        "ui_confirm",
        "ui_up",
        "ui_left",
        "ui_right",
        "ui_down",
        "ui_tab_prev",
        "ui_tab_next",
    ];

    pub fn try_load(&mut self, input: &dyn SteamSdk) -> bool {
        let Some(set_handle) = input.action_set_handle("Menu") else {
            return false;
        };
        self.loaded = 1;
        self.action_set_handle = set_handle;
        for (i, name) in Self::ANALOG_NAMES.iter().enumerate() {
            let handle = input.analog_action_handle(name);
            if handle.is_none() {
                log::error!("Failed to get analog action handle {}", name);
            }
            self.analog[i] = handle.unwrap_or(0);
        }
        for (i, name) in Self::DIGITAL_NAMES.iter().enumerate() {
            let handle = input.digital_action_handle(name);
            if handle.is_none() {
                log::error!("Failed to get digital action handle {}", name);
            }
            self.digital[i] = handle.unwrap_or(0);
        }
        true
    }

    /// read_state 0x1409e8bf0：1 模拟 + 9 数字。
    pub fn read_state(&self, input: &dyn SteamSdk, controller: u64, out: &mut SteamMenuActionSetState) {
        out.analog[0] = input.analog_action_data(controller, self.analog[0]).value;
        for (i, handle) in self.digital.iter().enumerate() {
            out.digital[i] = input.digital_action_data(controller, *handle).state;
        }
    }
}

/// 菜单动作集状态。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SteamMenuActionSetState {
    pub analog: [Vec2; 1],
    pub digital: [bool; 9],
}

impl SteamMenuActionSetState {
    /// is_any_active 0x1409e8aa0（同 Main 语义；Menu 实例内联）。
    pub fn is_any_active(&self) -> bool {
        self.analog
            .iter()
            .any(|v| v.x.abs().max(v.y.abs()) >= 0.25)
            || self.digital.iter().any(|&b| b)
    }
}

/// SteamInput 资源（startup_steam 0x140a9bbf0 插入；update_steam_inputs 0x140ae9740 消费）。
///
/// 原布局（O）：`active_controller u64 @0` + `? u64 @8` + `SteamMainActionSet @0x10`
/// （0xf0 B）+ `SteamMenuActionSet @0x100`（0x68 B）+ 动作源HashMap @0x160。
/// 其+168/170/178是容器元数据，不能当输入释放/按住计数。本重建按逻辑分域承载，
/// 字段序仅供消费端接线参照。
#[derive(Resource)]
pub struct SteamInput {
    pub client: Arc<dyn SteamSdk>,
    /// 当前手柄句柄（无手柄 = 0；@0x00）。
    pub active_controller: u64,
    /// 上一手柄句柄（连接匹配用；@0x08）。
    pub last_controller: u64,
    /// GameControls 动作集（@0x10）。
    pub main_action_set: SteamMainActionSet,
    /// Menu 动作集（@0x100）。
    pub menu_action_set: SteamMenuActionSet,
    /// 动作类型→本帧动作 分组（O：0x140ae9740 reserve_rehash）。
    pub actions_by_type: HashMap<DigitalActionSetActionType, Vec<SteamActionSetDigitalAction>>,
}

impl SteamInput {
    /// 当前是否连接指定手柄。
    pub fn has_controller(&self, controller: u64) -> bool {
        controller != 0
            && (self.active_controller == controller || self.last_controller == controller)
    }
}

/// 原动作源查询码：kind0=主模拟，1=主数字，2=菜单（index9为菜单模拟）。
/// get_action_origin_glyphs 0x1409e6280 的 R9 载荷。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionOriginGlyphQuery(pub u16);

impl ActionOriginGlyphQuery {
    /// 低8位的动作集分支。
    pub fn kind(self) -> u8 {
        (self.0 & 0xff) as u8
    }

    /// 高 8 位：动作下标。
    pub fn index(self) -> u8 {
        (self.0 >> 8) as u8
    }
}

impl SteamInput {
    /// get_action_origin_glyphs 0x1409e6280：查动作源，把每个非零源经
    /// `GetGlyphForActionOrigin_Legacy` → `to_string_lossy` 映射为字形串。
    ///
    /// 返回 None = 无字形（tag 0/1/2 细分在本重建中合并）；Some = 字形串表。
    pub fn get_action_origin_glyphs(&self, action: ActionOriginGlyphQuery) -> Option<Vec<String>> {
        // 原首项是 Option<控制器>；当前 API 用非零句柄表达 Some。
        if self.active_controller == 0 { return None; }
        let index = usize::from(action.index());
        let (set, handle, analog) = match action.kind() {
            0 if self.main_action_set.loaded == 1 =>
                (self.main_action_set.action_set_handle, self.main_action_set.analog.get(index).copied(), true),
            1 if self.main_action_set.loaded == 1 =>
                (self.main_action_set.action_set_handle, self.main_action_set.digital.get(index).copied(), false),
            2 if self.menu_action_set.loaded == 1 =>
                (self.menu_action_set.action_set_handle, if index == 9 {
                    self.menu_action_set.analog.first().copied()
                } else { self.menu_action_set.digital.get(index).copied() }, index == 9),
            _ => return None,
        };
        let Some(handle) = handle else { return None };
        let origins = if analog {
            self.client.analog_action_origins(self.active_controller, set, handle)
        } else {
            self.client.digital_action_origins(self.active_controller, set, handle)
        };
        let mut glyphs = Vec::new();
        for origin in origins {
            if origin != 0 {
                glyphs.push(self.client.glyph_for_action_origin(origin));
            }
        }
        if glyphs.is_empty() {
            None
        } else {
            Some(glyphs)
        }
    }
}

// ---------------------------------------------------------------------------
// 小型 From 转换（O，RNE 机器码核验）。
// ---------------------------------------------------------------------------

/// impl$30::from 0x1409e8d80：`x << 8`（单字节 → 码高 8 位）。原泛型目标 U，按机器码承载。
#[inline]
pub fn packed_set(x: u8) -> u16 {
    (x as u16) << 8
}

/// impl$31::from 0x1409e8d90：`(x << 8) + 1`。
#[inline]
pub fn packed_set_plus_one(x: u8) -> u16 {
    ((x as u16) << 8) + 1
}

/// impl$37::from 0x1409e8da0：`a + (b << 8) + 1`（双字节打包）。
#[inline]
pub fn packed_pair(a: u8, b: u8) -> u16 {
    (a as u16) + ((b as u16) << 8) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_action_set_name_tables_match_try_load_order() {
        assert_eq!(SteamMainActionSet::ANALOG_NAMES, ["CameraPan", "cursor_move", "camera_height"]);
        assert_eq!(SteamMainActionSet::DIGITAL_NAMES.len(), 25);
        assert_eq!(SteamMainActionSet::DIGITAL_NAMES[0], "left_click");
        assert_eq!(SteamMainActionSet::DIGITAL_NAMES[24], "ui_down");
    }

    #[test]
    fn menu_action_set_name_tables_match_try_load_order() {
        assert_eq!(SteamMenuActionSet::DIGITAL_NAMES.len(), 9);
        assert_eq!(SteamMenuActionSet::DIGITAL_NAMES[0], "main_menu");
        assert_eq!(SteamMenuActionSet::DIGITAL_NAMES[8], "ui_tab_next");
    }

    #[test]
    fn is_any_active_threshold_and_predicates() {
        let state = SteamMainActionSetState {
            analog: [Vec2::new(0.2, 0.0), Vec2::ZERO, Vec2::ZERO],
            digital: [false; 25],
        };
        assert!(!state.is_any_active());
        let state = SteamMainActionSetState {
            analog: [Vec2::new(0.25, 0.0), Vec2::ZERO, Vec2::ZERO],
            digital: [false; 25],
        };
        assert!(state.is_any_active());
        // NaN 不触发。
        let state = SteamMainActionSetState {
            analog: [Vec2::new(f32::NAN, 0.0), Vec2::ZERO, Vec2::ZERO],
            digital: [false; 25],
        };
        assert!(!state.is_any_active());
        // 任一 digital 触发。
        let mut digital = [false; 25];
        digital[3] = true;
        assert!(SteamMainActionSetState { analog: [Vec2::ZERO; 3], digital }.is_any_active());
    }

    #[test]
    fn action_codes_are_unique_u16_discriminants() {
        let mut seen = std::collections::HashSet::new();
        for code in [
            0x400, 0x500, 0x600, 0x700, 0x900, 0x800, 0xa00, 0xb00, 0xc00, 0xe00, 0xd00, 0x200,
            0x300, 0x1300, 0x1400, 0x1600, 0x1700, 0x1800, 0x1500, 0x1200, 0xf00, 0x101,
        ] {
            assert!(seen.insert(code), "duplicate code {code:#x}");
        }
        assert_eq!(seen.len(), 22);
    }

    #[test]
    fn packed_conversions_match_the_machine_code() {
        assert_eq!(packed_set(5), 0x500);
        assert_eq!(packed_set_plus_one(0x12), 0x1201);
        assert_eq!(packed_pair(0x34, 0x12), 0x1234 + 1);
    }
}

