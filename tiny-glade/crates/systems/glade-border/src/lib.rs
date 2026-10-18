//! system_glade_border — 谷地边界显示与越界反馈（restorer-eco-gladeborder，2026-10-17）。
//!
//! 原 crate（B skeleton）：`crates/systems/glade-border/`（3 文件；5 符号 =
//! 业务 2 / 内联 1 / 胶水 2）。
//! - `display_glade_border`（0x141be0ad0 + closure$0/closure$1）：`DisplayOutOfBorder`
//!   事件触发时把谷地边界曲线以 debug 线段可视化，`DEV_OVERRIDE_IS_DEMO_MODE`（B 构建
//!   = 0）时发射 `ShowDemoBorderOverlay`；`GameConfig[+0x121] == 0` 时经 `SignifierFactory`
//!   + `closure$1` 写边界周长 signifier（虚线实体，`DashedLineLength = 周长×0.7`）。
//! - `feedback_input_out_of_border`（行 12，PDB inlinee）：越界反馈上升沿 → `PlayUiSound`
//!   变体 #4（读末条事件 + 1 字节 Local 去抖）。

pub mod display_glade_border;
pub mod feedback_input_out_of_border;

use bevy_ecs::prelude::*;

use country_core::resources::glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder};

use crate::display_glade_border::display_glade_border;
use crate::feedback_input_out_of_border::feedback_input_out_of_border;

/// `system_glade_border::impl$0::build` @0x140650ab0（O 面，fresh 反编译 2026-10-17）：
/// `chain_ignore_deferred((display_glade_border, feedback_input_out_of_border))` →
/// `intern<dyn SystemSet>`（命名集）→ `in_set_inner` → `add_systems<Update>`；再按 TypeId
/// 存在性检查注册 `DisplayOutOfBorder` 与 `FeedbackInputOutOfBorder` 两个事件（不存在才
/// `register_event`）。ShowDemoBorderOverlay/PlayUiSound 事件由其他插件注册，本 build 不注册。
///
/// 原实现经 Bevy App 机制注册；本 crate 按 core garden 域约定以 `(world, schedule)` 落地。
/// 属 bevy 插件胶水（glue-legitimate），不重建。
pub fn register_glade_border_events(world: &mut World) {
    world.init_resource::<Events<DisplayOutOfBorder>>();
    world.init_resource::<Events<FeedbackInputOutOfBorder>>();
}

/// 注册 CPU 系统（O：build 0x140650ab0 的 add_systems 面——两系统按顺序链
/// `chain_ignore_deferred` 加入 `Update`）。
pub fn add_glade_border_cpu_systems(schedule: &mut Schedule) {
    schedule.add_systems(
        (display_glade_border, feedback_input_out_of_border).chain_ignore_deferred(),
    );
}

/// `impl$0::build`（0x140650ab0，O 面）。
pub fn build(world: &mut World, schedule: &mut Schedule) {
    register_glade_border_events(world);
    add_glade_border_cpu_systems(schedule);
}
