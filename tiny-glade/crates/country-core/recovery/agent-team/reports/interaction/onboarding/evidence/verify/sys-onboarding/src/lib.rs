//! system-onboarding（B 主基线）：新手引导提示系统。
//!
//! 类型/函数地址与布局证据：`upper-modules-plan/systems/onboarding/ANALYSIS.md` +
//! 本模块 evidence/（fresh Ghidra shared 反编译，get_functions fields=signature,
//! decompiled_code）。恢复轮次按用户硬标准（2026-10-16）：真业务函数 fresh 反编译 →
//! 重建原函数完整控制流图 → Rust body 逐语句判等；重建不了标 `未实现`（列出分歧点），
//! 不以近似体通过。
//!
//! 共享基础缺口（country-core，他组在途）阻断的字段/类型在本模块统一登记：
//! - `GameProfile` 提示区（0x1bc/0x1c0/0x1c4 字形模式、0x1e0..0x1f0 计时/冷却/累加器、
//!   0x1f4..0x205 提示完成标志、0x206）—— country-core `GameProfile` 仅承载
//!   `tutorial`（=0x1f4）；其余未落库。
//! - `country_core::resources::onboarding::NotifyHintSystem*`（除 ToolWasJustUsed 外 18 个）、
//!   `country_core::resources::events::DisplayRollercoasterPrompt`、`TutorialOp`、
//!   `tiny_glade::app_config::{WriteSettingsFileCmd,SaveGameProfileCmd}`、SteamInput。
//! - toolbar/mei（hint_draw/render_hints 渲染 DSL；批次 3 未恢复）。

pub mod exponential_back_off;
pub mod hint_camera_controls;
pub mod hint_camera_progress;
pub mod hint_clutter_focus_edit;
pub mod hint_deselect_tool;
pub mod hint_disable_snapping;
pub mod hint_display_system;
pub mod hint_focused_edit;
pub mod hint_orchestrator;
pub mod hint_roof_shape;
pub mod hint_stairs;
pub mod hint_terrain;
pub mod hint_terrain_focus;
pub mod hint_terrain_steepness;
pub mod hint_undo;
pub mod hint_unlink_windows;
pub mod hints_startup;
pub mod reset_hints;
pub mod tutorial_completion;

pub use hint_orchestrator::{
    BindingHintDisplay, DisplayHintRequest, HintGraphicParams, HintLayoutMemo, HintPlacement,
    HintProgressBar, HintType, InteractionPrompt, LingerHintRequest,
};

use bevy_ecs::prelude::*;

/// 键码（O：`bevy_input::keyboard::KeyCode` 同域；workspace game_key_bindings 以 u32
/// 承载，`KeyBinding.key`）。未绑哨兵 `0xc3` = `game_key_bindings::UNBOUND_KEY`
/// （country-core button_input.rs）。
pub type KeyCode = u32;

/// 鼠标按钮（O：bevy `MouseButton` 判别 Left=0 / Right=1 / Middle=2；country-core
/// `button_input::MouseButtonInput` 同空间）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Steam 覆盖层输入句柄（O：`country_core::resources::steam::SteamInput`；
/// **country-core 未落库**，本处为不透明 host seam。凡依赖其方法
/// （`get_action_origin_glyphs`）的函数标 `未实现`）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SteamInput;

/// add_onboarding（O 0x140650d50，2693）：Bevy 插件注册胶水。
///
/// 真注册（glue-legitimate，不重建）：`Components::get_resource_id` 幂等检查后
/// `EventRegistry::register_event` 注册 21 个事件（O：DisplayHintRequest /
/// LingerHintRequest / TutorialHintComplete / TutorialOp /
/// NotifyHintSystem{CameraMoved,Snapping,Terrain,TerrainSteepness,RoofShape,
/// ClutterWasMoved,ShapeEditWasPerformed,Window,ToolWasJustUsed,SomethingWasErased,
/// StairsPlaced,HoveringOverStairs,StartedStairsContinue,CanContinueStairs,
/// StairsDetachedFromWall,StairsCanDetachFromWall} / DisplayRollercoasterPrompt）；
/// Startup 加 `hints_startup`，Update 加提示系统（chain_ignore_deferred + in_set）。
///
/// 依赖 bevy_app/SubApp（本 workspace 仅 bevy_ecs）：以 `configure` 表达可落部分；
/// 未落库事件（country-core `resources::onboarding` 除 ToolWasJustUsed 外、TutorialOp、
/// DisplayRollercoasterPrompt）登记待落。
pub fn configure(world: &mut World, _schedule: &mut bevy_ecs::schedule::Schedule) {
    world.init_resource::<hint_camera_progress::CameraHintProgress>();
    world.init_resource::<hint_camera_controls::CameraHintsState>();
    world.init_resource::<hint_deselect_tool::DeselectToolHintConditions>();
    world.init_resource::<hint_focused_edit::FocusedEditHintConditions>();
    world.init_resource::<hint_terrain::HintTerrainConditions>();
    world.init_resource::<hint_terrain_steepness::HintTerrainSteepnessConditions>();
    world.init_resource::<hint_disable_snapping::HintDisableSnappingConditions>();
    world.init_resource::<hint_undo::UndoHintConditions>();
    world.init_resource::<hint_roof_shape::HintRoofShapeConditions>();
    world.init_resource::<hint_clutter_focus_edit::FocusedClutterHintConditions>();
    world.init_resource::<hint_terrain_focus::HintFocusTerrainConditions>();
    world.init_resource::<hint_stairs::StairsHintConditions>();

    world.init_resource::<Events<DisplayHintRequest>>();
    world.init_resource::<Events<LingerHintRequest>>();
    world.init_resource::<Events<tutorial_completion::TutorialHintComplete>>();
    world.init_resource::<Events<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>>();
}

/// prompt_rollercoaster（O 0x14227bb70，395）：EventReader<DisplayRollercoasterPrompt> →
/// alternate(Option<KeyCode>) 写 DisplayHintRequest（id 10，flags 0x1200）；
/// 常量 "Rollercoaster.Ride"。
/// **未实现**：DisplayRollercoasterPrompt（country-core resources::events）未落库。
pub fn prompt_rollercoaster(
    _events: EventReader<DisplayRollercoasterPromptSeam>,
    _writer: EventWriter<DisplayHintRequest>,
) {
    // 未实现：见函数注释分歧点。
}

/// DisplayRollercoasterPrompt seam（O：`country_core::resources::events::
/// DisplayRollercoasterPrompt`，**未落库**；本处为等价事件占位）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayRollercoasterPromptSeam;
