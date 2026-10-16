//! events.rs — `country_core::resources::events` 命令事件族（组 A restorer 接线）。
//!
//! 原模块承载跨域命令事件（构造/模式/灯光等）；本处只落地组 A 六 crate 消费的
//! 事件类型合同，载荷语义 U（typed gap，不造数据）。

use bevy_ecs::event::Event;

/// 木框架构造命令（O：`resources::events::ConstructTimberframeCmd`；forest
/// manage_tree_prefabs 0x141bf16c0 的 EventReader 面 + timberframe crate 三构造
/// 系统消费（0x28B 条目——事件载荷 0x28B，beam_count 游标按 0x28 步进）。
/// 载荷 = 墙形状（ShapeParameters 域）+ 屋顶参数族，语义 U。
#[derive(Event, Clone, Debug, Default)]
pub struct ConstructTimberframeCmd {
    /// 墙形状记录（O：ShapeParameters 28B 域；语义 U，typed gap 表达）。
    pub shape: Vec<u8>,
    /// typed gap：屋顶参数等。
    pub _gap: u64,
}

/// 模式刚切换事件（O：`resources::events::ModeJustChanged`；system_color
/// autoclose_color_menu 0x142298d30 消费。载荷语义 U）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct ModeJustChanged;

/// 菜单刚打开事件（O：`resources::gui::EscapeMenuJustOpened` 族——原模块
/// `resources::gui`；autoclose_color_menu 消费。载荷语义 U）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct EscapeMenuJustOpened;

/// 点灯命令（O：`resources::events::LightLanternCmd`；color_menu 0x142291230
/// 的 EventWriter 面。载荷语义 U）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct LightLanternCmd;

/// 窗灯命令（O：`resources::events::LightWindowCmd`；color_menu 的 EventWriter 面。
/// 载荷语义 U）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct LightWindowCmd;

/// 输入记录收尾命令（O：`resources::history::FinishInputRecordingCmd` 族——原
/// 模块 `resources::history`；color_menu 的 EventWriter 面。载荷语义 U）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct FinishInputRecordingCmd;
