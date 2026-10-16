//! Core wall consumers. These systems register no inferred original schedule.

pub mod wall_highlight;
pub mod wall_state;
pub mod calc_wall_bend;
pub mod delete_wall;
pub mod construct_wall_bricks;
pub mod decorator_wall_events;
pub mod registration;
pub mod adjust_wall_height;
pub mod history_recording;
pub mod valid_enclosure;
pub mod snapping;
pub mod eraser;
pub mod shapes;
pub mod structure_outline;

// ---------------------------------------------------------------- 组 A 接线

/// 墙样式设置命令（O：`systems::wall::set_wall_style::SetWallStyleCmd` 族；
/// color_menu 0x142291230 的 EventWriter 面；载荷语义 U——样式/模式字节族）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetWallStyleCmd {
    /// typed gap。
    pub _gap: u64,
}

/// 自由手高度类型设置命令（O：`set_wall_style::SetFreehandHeightTypeCmd` 族）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetFreehandHeightTypeCmd {
    /// typed gap。
    pub _gap: u64,
}

/// 灰泥剥离量命令（O：`set_wall_style::SetPlasterPeelAmountCmd` 族）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetPlasterPeelAmountCmd {
    /// typed gap。
    pub _gap: u64,
}
