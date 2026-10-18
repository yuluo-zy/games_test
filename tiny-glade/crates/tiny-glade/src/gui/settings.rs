//! B `crates/tiny-glade/src/gui/settings.rs`（设置菜单 / 视频设置应用判定 / Konami）。
//!
//! fresh Ghidra shared 2026-10-17（证据在
//! `recovery/agent-team/work/restorer-tinyglade-C/`，正确 VA 带 0x140 前缀）：
//! - `SettingsMenuPlugin::build` `0x140245570`（189 B）
//! - `LastAppliedVideoSettings::needs_apply` `0x140245630`（350 B）
//! - `do_gui_escape_menu_settings` `0x140245790`（4,290 B）
//! - `ui_settings_game` `0x140247060`、`ui_settings_audio` `0x1402475d0`、
//!   `ui_settings_video` `0x140247870`（10,058 B）、`ui_settings_ui`
//!   `0x14024aa80`、`ui_settings_developer` `0x14024ae60`、
//!   `ui_settings_controls` `0x14024b120`
//! - `detect_konami_code` `0x14024b370`（330 B）
//!
//! PDB 另有本文件内的 `ui_settings_keybindings`（源行 462 起，9 个闭包）、
//! `ui_settings_video_main/display/image`、`get_quailty_presets` 等函数，
//! 均属同一未恢复 UI 子树，本批未列入任务。
//!
//! ## `SettingsMenuPlugin::build`（判等，胶水-注册）
//! ```text
//! app.insert_resource::<LastAppliedVideoSettings>(
//!        LastAppliedVideoSettings::default())
//!    .insert_resource::<AvailableMonitors>(AvailableMonitors::default())
//!    .add_event::<ApplyVideoSettingsCmd>();
//! ```
//! 事件注册前的 resource-id 检查是 Bevy `EventRegistry` 内联胶水。
//!
//! ## `LastAppliedVideoSettings::needs_apply`（fresh 0x140245630，未实现）
//! 反汇编级结构（`self`=rcx, `other`=rdx）：
//! ```text
//! self.field0.unwrap();                      // [self+0]==i64::MIN → unwrap_failed
//! al = (self.byte@0xa8 != other.byte@0xa8) << 1;
//! self.opt@0x38 / other.opt@0x38             // Option 的 None 有 2 个哨兵：
//!                                            // 0x8000_0000_0000_0000 / ..._0001
//! 若任一为 None →  若恰一个 None ⇒ true
//!                  若都 None ⇒ 比较 len@0x48 + memcmp(ptr@0x40)
//! 若都 Some   ⇒ 依次比较 0x68(u32)、0x6c(u64)、0x78(u16)、0x74(u32)、
//!               0x7c(bool)、0x7d(bool)；不等 ⇒ true
//! 结果还并入 al 的 bit0/bit1/bit2（`and al,6`）。
//! ```
//! 缺 `LastAppliedVideoSettings` 的 PDB 字段名/字段语义（结构体仅类型名，
//! TPI 占位），无法命名字段与判定 payload；不写近似结构。
//!
//! ## `detect_konami_code`（fresh 0x14024b370，未实现）
//! PDB 系统签名：`fn(Res<ButtonInput<KeyCode>>, Local<usize>)`。
//! 反汇编：扫描 `ButtonInput<KeyCode>` 的 HashMap 控制字节（16 桶/组，
//! 取每个控制字节的 bit7），对置位桶读取 12 B 桶条目中的 KeyCode；每帧按
//! 桶顺序取一个键与目标序列 `[0x53,0x50,0x50,0x51,0x52,0x51,0x52,0x15,
//! 0x14,0x3a]` 比较；命中则 `progress += 1`，满 0xb → 置
//! `SHOW_DEVELOPER_SETTINGS = 1`；不匹配清零。
//! **已解出键值语义**：0x53/0x50/0x51/0x52/0x15/0x14/0x3a 正是 bevy_input
//! 0.16 `KeyCode` 的判别式 —— ArrowUp/ArrowDown/ArrowLeft/ArrowRight/KeyB/
//! KeyA/Enter（Konami 码）。序列第 11 项与 0 值“跳过项”的栈记录布局
//! （12 B/记录、判零分支的跳表）尚未逐条闭合；另有哈希桶顺序依赖。
//!
//! ## 6 个 `ui_settings_*` / `do_gui_escape_menu_settings`（未实现）
//! egui/MEI 面板；反汇编中每个控件都是 `TweakableFloatWithStep/Enum/Bool` +
//! `HintTracker::text` + `lang_lib::lookup` 的内联调用，但描述符（步长/范围）、
//! 语言键与 `SettingsMenuState` 字段绑定在 `.rdata` 常量与闭包环境中，未逐项
//! 取证。保留为未实现，不以平面近似 UI 代替。

use bevy_app::{App, Plugin};
use bevy_ecs::system::Local;
use bevy_input::ButtonInput;
use bevy_input::keyboard::KeyCode;
use bevy_ecs::change_detection::Res;

/// Konami 目标序列（O：`0x14024b370` 栈上立即数，按帧顺序）。
///
/// 值 = bevy_input 0.16 `KeyCode` 判别式：
/// `0x53`=ArrowUp、`0x50`=ArrowDown、`0x51`=ArrowLeft、`0x52`=ArrowRight、
/// `0x15`=KeyB、`0x14`=KeyA、`0x3a`=Enter。
pub const KONAMI_SEQUENCE: [u32; 10] =
    [0x53, 0x50, 0x50, 0x51, 0x52, 0x51, 0x52, 0x15, 0x14, 0x3a];

/// 命中所需进度（O：`uVar8 == 0xb`；栈记录共 11 项，含 0 值“跳过”项）。
pub const KONAMI_LENGTH: usize = 0xb;

/// `SHOW_DEVELOPER_SETTINGS`（O：全局字节 `0x14364e690` 附近，命中置 1）。
pub const SHOW_DEVELOPER_SETTINGS_FLAG: u32 = 1;

/// `tiny_glade::gui::settings::SettingsMenuPlugin`。
#[derive(Debug, Default, Clone, Copy)]
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    /// `SettingsMenuPlugin::build` `0x140245570`（胶水-注册，判等）。
    fn build(&self, app: &mut App) {
        app.insert_resource(LastAppliedVideoSettings::default())
            .insert_resource(country_core::resources::window::AvailableMonitors::default())
            .add_event::<crate::app_config::ApplyVideoSettingsCmd>();
    }
}

/// `tiny_glade::gui::settings::LastAppliedVideoSettings`。
///
/// B 内存布局（needs_apply 反汇编）：`field0: Option<_> @0x00`（`unwrap`）、
/// `byte@0xa8`、`Option<String>` 样 @0x38（cap/ptr/len = 0x38/0x40/0x48）、
/// u32@0x68、u64@0x6c、u32@0x74、u16@0x78、bool@0x7c、bool@0x7d。
/// 字段名/语义未取证，故保留空壳，不编造字段。
#[derive(Debug, Default, bevy_ecs::resource::Resource)]
pub struct LastAppliedVideoSettings {
    _private: (),
}

impl LastAppliedVideoSettings {
    /// `LastAppliedVideoSettings::needs_apply` `0x140245630` —— 未实现。
    pub fn needs_apply(&self, _other: &Self) -> bool {
        todo!(
            "未实现: LastAppliedVideoSettings::needs_apply 0x140245630 —— 偏移/分支已解 \
             (field0.unwrap; byte@0xa8; Option@0x38 双 None 哨兵; len@0x48/memcmp@0x40; \
             0x68/0x6c/0x74/0x78/0x7c/0x7d)，字段名与 payload 语义未取证"
        )
    }

    /// `populate_empty_from`（源行 70，inline）—— 未实现。
    pub fn populate_empty_from(&mut self, _other: &Self) {
        todo!(
            "未实现: populate_empty_from 源行 70 —— PDB 仅有 Option::get_or_insert_with \
             内联证据，字段名未取证"
        )
    }
}

/// `do_gui_escape_menu_settings` `0x140245790` —— 未实现。
pub fn do_gui_escape_menu_settings() {
    todo!(
        "未实现-需重做: do_gui_escape_menu_settings 0x140245790 —— egui 设置菜单，\
         Tweakable*/lang 键/闭包环境未逐项取证"
    )
}

/// `ui_settings_game` `0x140247060` —— 未实现。
pub fn ui_settings_game() {
    todo!("未实现-需重做: ui_settings_game 0x140247060 —— egui UI")
}

/// `ui_settings_audio` `0x1402475d0` —— 未实现。
pub fn ui_settings_audio() {
    todo!("未实现-需重做: ui_settings_audio 0x1402475d0 —— 4×TweakableFloatWithStep，语言键未取证")
}

/// `ui_settings_video` `0x140247870` —— 未实现。
pub fn ui_settings_video() {
    todo!("未实现-需重做: ui_settings_video 0x140247870 —— egui UI（10 KB）")
}

/// `ui_settings_ui` `0x14024aa80` —— 未实现。
pub fn ui_settings_ui() {
    todo!("未实现-需重做: ui_settings_ui 0x14024aa80 —— Tweakable*/WindowFocusEvent，语言键未取证")
}

/// `ui_settings_developer` `0x14024ae60` —— 未实现。
pub fn ui_settings_developer() {
    todo!("未实现-需重做: ui_settings_developer 0x14024ae60 —— egui UI")
}

/// `ui_settings_controls` `0x14024b120` —— 未实现。
pub fn ui_settings_controls() {
    todo!("未实现-需重做: ui_settings_controls 0x14024b120 —— egui UI")
}

/// `detect_konami_code` `0x14024b370` —— 未实现（状态机已解，桶布局/0 项未闭合）。
#[allow(unused_variables)]
pub fn detect_konami_code(
    keys: Res<ButtonInput<KeyCode>>,
    progress: Local<usize>,
) {
    todo!(
        "未实现: detect_konami_code 0x14024b370 —— 序列=KeyCode 判别式 \
         ArrowUp/Down/Down/Left/Right/Left/Right/B/A/Enter，满 0xb 置 \
         SHOW_DEVELOPER_SETTINGS；HashMap 桶顺序与 0 值跳过项未逐条闭合"
    )
}
