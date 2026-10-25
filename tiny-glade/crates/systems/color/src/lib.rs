//! system_color — 调色系统族（restorer-state-color，2026-10-17 fresh 反编译重建）。
//!
//! 原 crate（B skeleton）：`crates/systems/color/`（17 文件；21 业务函数）。
//! 事件族布局按 fresh 反编译消费面恢复（`EventInstance = {event_id u64, event E}`，
//! bevy 0.16 fork 的 `EventInstance` 为 id 前置，见 base.rs:181）：
//! - 墙/门/半木事件（country_core 已定义）：E={u64 id, u32 色, u8 custom}，stride 0x18。
//! - 本模块事件：见下方各结构注释。

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

/// 显示颜色菜单命令（O：`show_color_menu::ShowColorMenuCmd`；ui_color_* 发射、
/// show_color_options 0x14229a7f0 消费）。字段名 U（show_color_options 反编译失败）；
/// autoclose_color_menu 0x142298d30 读 kind 字段：`kind == 0xc` 直接关闭。
#[repr(C)]
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct ShowColorMenuCmd {
    /// kind（O：autoclose 读 `*event == 0xc`）。
    pub kind: u8,
    /// DecoratorId uses the native +4 payload, independently of wall ids.
    pub decorator_id: u32,
    /// Native +8 WallId payload.
    pub payload: u64,
    /// Native +0x10 screen-space menu center, supplied by the clicked widget.
    pub position: glam::Vec2,
}

const _: () = assert!(std::mem::size_of::<ShowColorMenuCmd>() == 24);

/// 隐藏颜色菜单命令（O：`show_color_menu::HideColorMenuCmd`；hide_color_menu
/// 0x1422adcf0 消费）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct HideColorMenuCmd {
    /// Native event byte: true restores the saved selection before closing.
    pub cancel: bool,
}

/// 屋顶颜色命令（O：`set_roof_customization::SetRoofColorIdCmd`；消费
/// set_roof_customization 0x142299dd0 / write_roof_color_id_to_gpu 0x142298520）。
/// E={id u64, color u32, custom u8}，stride 0x18（set_roof_customization 外循环
/// 读 event+0x10 色、+0x14 custom）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetRoofColorIdCmd {
    /// 目标 id。
    pub id: u64,
    /// 颜色 id。
    pub color: u32,
    /// custom 标志（+0x12 触发布局重算）。
    pub custom: u8,
}

/// 平顶样式命令（权威定义在 `roofs::commands::SetFlatRoofStyleCmd`——
/// 原模块 `set_roof_customization` 侧消费该核心事件）。
pub use country_core::resources::roofs::commands::SetFlatRoofStyleCmd;

/// 装饰物颜色命令（O：`set_decorator_color_id::SetDecoratorColorIdCmd`；
/// 消费 set_decorator_color_id 0x1422af0c0 外循环）。E={id u32, color u32,
/// custom u8}，stride 0x18（读 event+0xc 色、+0x10 custom）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetDecoratorColorIdCmd {
    /// 装饰 id。
    pub id: u32,
    /// 颜色 id（写 DecoratorInfo+0x28）。
    pub color: u32,
    /// custom 标志。
    pub custom: u8,
}

/// 装饰亮度命令（O：`SetDecoratorBrightnessCmd`；ColorMenuBrightnessValue
/// 三变体 {0.25, 1.0, 4.0}（0x140b06770 判别表，coordinator 契约 #1））。
/// E={id u32, value u32, custom u8}，stride 0x18（set_decorator_color_id 第 2 层
/// 写灯笼 +0x0、tag 0xe）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetDecoratorBrightnessCmd {
    /// 装饰 id。
    pub id: u32,
    /// 亮度值。
    pub value: u32,
    /// custom 标志。
    pub custom: u8,
}

/// 梯子颜色命令（O：`SetLadderColorIdCmd`；set_decorator_color_id 第 3 层写
/// 楼梯连接节点 +0x8、tag 10）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetLadderColorIdCmd {
    /// 楼梯装饰 id。
    pub id: u32,
    /// 颜色值。
    pub value: u32,
    /// custom 标志。
    pub custom: u8,
}

/// 楼梯扶手命令（O：`SetStairRailingCmd`；set_decorator_color_id 第 4 层
/// （stride 0x10）写楼梯连接节点 +0x10、tag 10）。E={id u32, value u8, custom u8}
/// = 8B → EventInstance stride 0x10。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetStairRailingCmd {
    /// 楼梯装饰 id。
    pub id: u32,
    /// 扶手值。
    pub value: u8,
    /// custom 标志。
    pub custom: u8,
}

/// 魔法颜色命令（O：`SetMagicColorIdCmd`；set_decorator_color_id 最内层写
/// 楼梯连接节点 +0xc、tag 10）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetMagicColorIdCmd {
    /// 楼梯装饰 id。
    pub id: u32,
    /// 颜色值。
    pub value: u32,
    /// custom 标志。
    pub custom: u8,
}

/// 窗灯命令（O：`set_window_light::SetWindowLightCmd`；消费 set_window_light
/// 0x142298260）。E={id u32, value u32, custom u8}，stride 0x18（读 event+0xc 值、
/// +0x10 custom）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct SetWindowLightCmd {
    /// 装饰 id。
    pub id: u32,
    /// 窗灯值（u32，写 WindowExtra.f 位模式）。
    pub value: u32,
    /// custom 标志。
    pub custom: u8,
}

/// Share the original core event identity with replay, UI, and the consumer.
pub use country_core::resources::walls::decorator::{SetChimneySmokeCmd, SetFlagConnectionCmd};

/// `system_color::impl$0::add_color` @0x140651870（注册面；FIX-4 reviewer：
/// 事件**集合**一致 15 个，注册面按 workspace 约定 `init_resource` 胶水落地——
/// 未逐事件对齐反编译的 `register_event` 顺序，另含本模块系统写出的他域事件）。
pub fn register_color_events(world: &mut World) {
    use country_core::resources::color_id::{
        GateColorIds, HalftimberColorIds, RoofColorIds, SetGateColorIdCmd, SetHalftimberColorIdCmd,
        SetWallColorIdCmd, WallColorIds,
    };
    use country_core::resources::glade_border::NotInDemoHint;
    use country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd;
    use country_core::systems::ui::resources::widget_params::PlayUiSound;
    use country_core::systems::wall::{
        SetFreehandHeightTypeCmd, SetPlasterPeelAmountCmd, SetWallStyleCmd,
    };

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

/// 仅供本 crate 独立运行/测试的 `(world, schedule)` 落地（旧约定）。真实游戏经 [`ColorSystems::add_color`]。
pub fn build<S: Send + Sync + 'static>(world: &mut World, schedule: &mut Schedule) {
    register_color_events(world);
    add_color_cpu_systems::<S>(schedule);
}

/// `<bevy_app::App as system_color::ColorSystems>::add_color`（0x140651870）。
///
/// 机器码（PyGhidra 逐指令 + 各系统 `Box<dyn System>` 类型名串核对）：
/// 1. 15 次 `register_event`：ShowColorMenuCmd、HideColorMenuCmd、SetWallColorIdCmd、
///    SetRoofColorIdCmd、SetDecoratorColorIdCmd、SetDecoratorBrightnessCmd、SetGateColorIdCmd、
///    SetHalftimberColorIdCmd、SetFlagConnectionCmd、SetWindowLightCmd、SetChimneySmokeCmd、
///    SetFlatRoofStyleCmd、SetLadderColorIdCmd、SetStairRailingCmd、SetMagicColorIdCmd；
/// 2. `Schedules::add_systems(<启动类调度>, init_color_ids_buffer)`（0x140ece460，创建四个颜色
///    SSBO 与 CPU 表）。**调度标签烤进单体实例，未识别**；按 Startup 注册，PreStartup/Startup 对
///    首帧 Update 等价；
/// 3. `SubApp::add_systems(Update, (replay_color.in_set(0x28), (18 个系统).chain_ignore_deferred()
///    .in_set(0x1f)))`；链内顺序取自 `chain_ignore_deferred` 单体（0x140ed1780）里
///    `into_configs` 的调用顺序。
///
/// 与重建 crate 旧 `add_color_cpu_systems` 的差异：原版有 `set_wall_style`、没有 `color_cursor_icon`。
/// 未闭合：原版 `set_*_color_id` 内含 SSBO 上传段，重建版未恢复，宿主用独立的
/// `write_wall_color_ids_to_gpu` 补这一段。
pub trait ColorSystems {
    fn add_color(&mut self) -> &mut Self;
}

impl ColorSystems for bevy_app::App {
    fn add_color(&mut self) -> &mut Self {
        use bevy_app::{Startup, Update};
        use country_core::resources::history::MaybePendingSnapshotId as S;
        use country_core::systems::GameSystemSet;

        self.add_event::<ShowColorMenuCmd>();
        self.add_event::<HideColorMenuCmd>();
        self.add_event::<country_core::resources::color_id::SetWallColorIdCmd>();
        self.add_event::<SetRoofColorIdCmd>();
        self.add_event::<SetDecoratorColorIdCmd>();
        self.add_event::<SetDecoratorBrightnessCmd>();
        self.add_event::<country_core::resources::color_id::SetGateColorIdCmd>();
        self.add_event::<country_core::resources::color_id::SetHalftimberColorIdCmd>();
        self.add_event::<SetFlagConnectionCmd>();
        self.add_event::<SetWindowLightCmd>();
        self.add_event::<SetChimneySmokeCmd>();
        self.add_event::<SetFlatRoofStyleCmd>();
        self.add_event::<SetLadderColorIdCmd>();
        self.add_event::<SetStairRailingCmd>();
        self.add_event::<SetMagicColorIdCmd>();

        self.add_systems(Startup, set_wall_color_id::init_color_ids_buffer);
        self.add_systems(
            Update,
            (
                replay_color::replay_color::<S>.in_set(GameSystemSet(0x28)),
                (
                    ui_color_wall_and_roof::ui_color_wall_and_roof,
                    ui_color_decorator::ui_color_windows_and_flags,
                    ui_color_gates::ui_color_gates,
                    autoclose_color_menu::autoclose_color_menu,
                    show_color_menu::hide_color_menu,
                    color_menu::delete_color_icon_entities,
                    show_color_menu::show_color_options::<S>,
                    color_menu::color_menu::<S>,
                    color_menu::color_button_animations,
                    set_wall_color_id::set_wall_color_id,
                    set_roof_customization::set_roof_customization::<S>,
                    set_decorator_color_id::set_decorator_color_id,
                    set_gate_color_id::set_gate_color_id,
                    set_halftimber_color_id::set_halftimber_color_id,
                    country_core::systems::wall::set_wall_style::set_wall_style,
                    set_flag_connection_n_chimney_smoke::set_flag_connection_and_chimney_smoke,
                    set_window_light::set_window_light,
                    write_roof_color_id_to_gpu::write_roof_color_id_to_gpu,
                )
                    .chain_ignore_deferred()
                    .in_set(GameSystemSet(0x1f)),
            ),
        );
        self
    }
}
