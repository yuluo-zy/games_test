//! system_glade_border — 谷地边界显示与越界反馈（组 A restorer，2026-10-16）。
//!
//! 原 crate（B skeleton）：`crates/systems/glade-border/`（3 文件；5 符号 =
//! 业务 2 / 内联 1 / 胶水 2）。
//! - `display_glade_border`（0x141be0ad0）：`DisplayOutOfBorder` 事件触发时把谷地
//!   边界曲线以 debug 线段 + signifier 可视化，进程全局 `*0x14362c098 == 1` 时发射
//!   `ShowDemoBorderOverlay`。
//! - `feedback_input_out_of_border`（行 12，PDB inlinee）：越界输入 → `PlayUiSound`
//!   （读末条事件 + 写事件，事件体并入调用点）。

pub mod display_glade_border;
pub mod feedback_input_out_of_border;

use bevy_ecs::prelude::*;

use country_core::resources::glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder, ShowDemoBorderOverlay};
use country_core::systems::ui::resources::widget_params::PlayUiSound;

use crate::display_glade_border::display_glade_border;
use crate::feedback_input_out_of_border::feedback_input_out_of_border;

/// `system_glade_border::impl$0::build` @0x140650ab0（O 面：1 次 add_systems +
/// in_set + register_event×1；资源操作 0x1400dd6f0 哈希查找 ×2 区 = 条件性资源插入，
/// TypeId 映射 U，re-analyst §2）。
///
/// 原实现经 Bevy App 机制注册；本 crate 按 core garden 域约定以 `(world, schedule)`
/// 落地。
pub fn register_glade_border_events(world: &mut World) {
    world.init_resource::<Events<DisplayOutOfBorder>>();
    world.init_resource::<Events<FeedbackInputOutOfBorder>>();
    world.init_resource::<Events<ShowDemoBorderOverlay>>();
    world.init_resource::<Events<PlayUiSound>>();
}

/// 注册 CPU 系统（O：build 0x140650ab0 的 add_systems 面）。
pub fn add_glade_border_cpu_systems(schedule: &mut Schedule) {
    schedule.add_systems(display_glade_border);
    schedule.add_systems(feedback_input_out_of_border);
}

/// `impl$0::build`（0x140650ab0，O 面）。
pub fn build(world: &mut World, schedule: &mut Schedule) {
    register_glade_border_events(world);
    add_glade_border_cpu_systems(schedule);
}
