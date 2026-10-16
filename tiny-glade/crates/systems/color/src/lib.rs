//! system_color — 调色系统族（组 A restorer，2026-10-16）。
//!
//! 原 crate（B skeleton）：`crates/systems/color/`（17 文件；77 符号 = 业务 26 /
//! 内联 11 / 胶水 40；rhapsody `GraphicsDevice::create_buffer` 0x14288e990 判
//! dependency_reuse 不恢复）。
//! 契约：`systems-recovery/groupA/color/RE-ANALYSIS.md`（SSBO 写链、实体颜色
//! 记录 0x20B {id, 打包中心, max_y×0.4922, tag}（tag 6 墙/7 门/0xc 半木/10 装饰）、
//! 图标网格路径全表、IconInfo{String, kind@+0x18, u32@+0x24}）。

pub mod autoclose_color_menu;
pub mod color_menu;
pub mod radial_menu;
pub mod replay_color;
pub mod set_decorator_color_id;
pub mod set_flag_connection_n_chimney_smoke;
pub mod set_gate_color_id;
pub mod set_halftimber_color_id;
pub mod set_roof_customization;
pub mod set_wall_color_id;
pub mod set_window_light;
pub mod show_color_menu;
pub mod ui_color_decorator;
pub mod ui_color_gates;
pub mod ui_color_wall_and_roof;
pub mod write_roof_color_id_to_gpu;

use bevy_ecs::prelude::*;

// ---------------------------------------------------------------- 事件族

/// 显示颜色菜单命令（O：`system_color::show_color_menu::ShowColorMenuCmd`；
/// ui_color_wall_and_roof / ui_color_gates / ui_color_windows_and_flags 发射、
/// show_color_options 0x14229a7f0 消费）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct ShowColorMenuCmd {
    /// typed gap。
    pub _gap: u64,
}

/// 隐藏颜色菜单命令（O：`show_color_menu::HideColorMenuCmd`；hide_color_menu
/// 0x1422adcf0 消费）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct HideColorMenuCmd;

/// 屋顶颜色命令（O：`set_roof_customization::SetRoofColorIdCmd`；消费
/// set_roof_customization 0x142299dd0 / write_roof_color_id_to_gpu 0x142298520）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetRoofColorIdCmd {
    /// typed gap。
    pub _gap: u64,
}

/// 平顶样式命令（权威定义在 `roofs::commands::SetFlatRoofStyleCmd`——
/// 原模块 `set_roof_customization` 侧消费该核心事件）。
pub use country_core::resources::roofs::commands::SetFlatRoofStyleCmd;

/// 装饰物颜色命令（O：`set_decorator_color_id::SetDecoratorColorIdCmd`；
/// 消费 set_decorator_color_id 0x1422af0c0）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetDecoratorColorIdCmd {
    /// typed gap。
    pub _gap: u64,
}

/// 装饰亮度命令（O：`SetDecoratorBrightnessCmd`；ColorMenuBrightnessValue
/// 三变体 {0.25, 1.0, 4.0}（0x140b06770 判别表，coordinator 契约 #1））。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetDecoratorBrightnessCmd {
    /// typed gap。
    pub _gap: u64,
}

/// 梯子颜色命令（O：`SetLadderColorIdCmd`）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetLadderColorIdCmd {
    /// typed gap。
    pub _gap: u64,
}

/// 楼梯扶手命令（O：`SetStairRailingCmd`）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetStairRailingCmd {
    /// typed gap。
    pub _gap: u64,
}

/// 魔法颜色命令（O：`SetMagicColorIdCmd`）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetMagicColorIdCmd {
    /// typed gap。
    pub _gap: u64,
}

/// 窗灯命令（O：`set_window_light::SetWindowLightCmd`；消费 set_window_light
/// 0x142298260）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetWindowLightCmd {
    /// typed gap。
    pub _gap: u64,
}

/// `system_color::impl$0::add_color` @0x140651870（O 面：17 系统注册 + 资源/
/// 事件注册；re-analyst §6：17 个 0x140ecxxx 注册调用的系统名映射 U）。
pub fn register_color_events(world: &mut World) {
    use country_core::resources::color_id::{
        GateColorIds, HalftimberColorIds, RoofColorIds, SetGateColorIdCmd,
        SetHalftimberColorIdCmd, SetWallColorIdCmd, WallColorIds,
    };
    use country_core::resources::glade_border::NotInDemoHint;
    use country_core::resources::walls::decorator::{SetChimneySmokeCmd, SetFlagConnectionCmd};
    use country_core::systems::ui::resources::widget_params::PlayUiSound;
    use country_core::systems::wall::{SetFreehandHeightTypeCmd, SetPlasterPeelAmountCmd, SetWallStyleCmd};
    use country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd;

    // O：三色表资源注册（`init_color_ids_buffer` 0x142290750 的 insert_resource
    // ×3 + 菜单登记 4×0x140987ca0）。
    world.init_resource::<WallColorIds>();
    world.init_resource::<GateColorIds>();
    world.init_resource::<HalftimberColorIds>();
    world.init_resource::<RoofColorIds>();
    world.init_resource::<Events<ShowColorMenuCmd>>();
    world.init_resource::<Events<HideColorMenuCmd>>();
    world.init_resource::<Events<SetRoofColorIdCmd>>();
    world.init_resource::<Events<SetFlatRoofStyleCmd>>();
    world.init_resource::<Events<SetDecoratorColorIdCmd>>();
    world.init_resource::<Events<SetDecoratorBrightnessCmd>>();
    world.init_resource::<Events<SetLadderColorIdCmd>>();
    world.init_resource::<Events<SetStairRailingCmd>>();
    world.init_resource::<Events<SetMagicColorIdCmd>>();
    world.init_resource::<Events<SetWindowLightCmd>>();
    world.init_resource::<Events<SetWallColorIdCmd>>();
    world.init_resource::<Events<SetGateColorIdCmd>>();
    world.init_resource::<Events<SetHalftimberColorIdCmd>>();
    world.init_resource::<Events<SetFlagConnectionCmd>>();
    world.init_resource::<Events<SetChimneySmokeCmd>>();
    world.init_resource::<Events<SetFreehandHeightTypeCmd>>();
    world.init_resource::<Events<SetPlasterPeelAmountCmd>>();
    world.init_resource::<Events<SetWallStyleCmd>>();
    world.init_resource::<Events<SetFlatRoofRimStyleCmd>>();
    world.init_resource::<Events<PlayUiSound>>();
    world.init_resource::<Events<NotInDemoHint>>();
}

/// 注册 CPU 系统（O：add_color 0x140651870 的 add_systems 面）。
pub fn add_color_cpu_systems<S: Send + Sync + 'static>(schedule: &mut Schedule) {
    // 分两批注册（O：add_color 0x140651870 的 add_systems 族；bevy 元组上限）。
    schedule.add_systems((
        show_color_menu::show_color_options::<S>,
        show_color_menu::hide_color_menu,
        autoclose_color_menu::autoclose_color_menu,
        set_wall_color_id::init_color_ids_buffer,
        set_wall_color_id::set_wall_color_id,
        set_gate_color_id::set_gate_color_id,
        set_halftimber_color_id::set_halftimber_color_id,
        set_roof_customization::set_roof_customization::<S>,
        write_roof_color_id_to_gpu::write_roof_color_id_to_gpu,
        set_decorator_color_id::set_decorator_color_id,
        set_window_light::set_window_light,
        set_flag_connection_n_chimney_smoke::set_flag_connection_and_chimney_smoke,
        ui_color_wall_and_roof::ui_color_wall_and_roof,
        ui_color_wall_and_roof::color_cursor_icon,
    ));
    schedule.add_systems((
        ui_color_gates::ui_color_gates,
        ui_color_decorator::ui_color_windows_and_flags,
        color_menu::color_menu::<S>,
        color_menu::color_button_animations,
        color_menu::delete_color_icon_entities,
        replay_color::replay_color::<S>,
    ));
}

/// `system_color::impl$0::add_color`（0x140651870，O 面）。
pub fn build<S: Send + Sync + 'static>(world: &mut World, schedule: &mut Schedule) {
    register_color_events(world);
    add_color_cpu_systems::<S>(schedule);
}
