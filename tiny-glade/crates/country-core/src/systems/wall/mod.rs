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
pub mod set_wall_style;

// ---------------------------------------------------------------- 组 A 接线

/// 墙样式设置命令（O：`systems::wall::set_wall_style::SetWallStyleCmd` 族；
/// color_menu 0x142291230 的 EventWriter 面；载荷语义 U——样式/模式字节族）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetWallStyleCmd {
    pub wall_id: crate::resources::walls::WallId,
    pub from: u8,
    pub to: u8,
    pub record_history: u8,
    pub play_sound: u8,
}

/// 自由手高度类型设置命令（O：`set_wall_style::SetFreehandHeightTypeCmd` 族）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetFreehandHeightTypeCmd {
    pub wall_id: crate::resources::walls::WallId,
    pub from: u8,
    pub to: u8,
    pub record_history: u8,
}

/// 灰泥剥离量命令（O：`set_wall_style::SetPlasterPeelAmountCmd` 族）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetPlasterPeelAmountCmd {
    pub wall_id: crate::resources::walls::WallId,
    pub from: f32,
    pub to: f32,
    pub record_history: u8,
}

pub mod construction;

pub mod height_command;
pub mod draw_wall;
pub mod deferred_wall_history;
pub mod ui_draw_wall;
pub mod finish_wall;
pub mod ui_continue_wall;
