//! system-onboarding（B 主基线）：新手引导提示系统。
//!
//! 类型/函数地址与布局证据：`upper-modules-plan/systems/onboarding/ANALYSIS.md` +
//! 本模块 evidence/（fresh Ghidra shared 反编译，get_functions fields=signature,
//! decompiled_code）。恢复轮次按用户硬标准（2026-10-16）：真业务函数 fresh 反编译 →
//! 重建原函数完整控制流图 → Rust body 逐语句判等；重建不了标 `未实现`（列出分歧点），
//! 不以近似体通过。
//!
//! 共享基础缺口（country-core）——**wave3 已闭合（2026-10-17，coregap-2）**：
//! - `GameProfile` 提示区（0x1bc/0x1c0/0x1c4 字形模式、0x1e0..0x1f0 计时/冷却/累加器、
//!   0x1f4..0x205 提示完成标志、0x206）→ 已落 country-core `game_profile.rs`；
//! - `NotifyHintSystem*`（16 事件）、`DisplayRollercoasterPrompt`、`TutorialOp`、
//!   `WriteSettingsFileCmd`/`SaveGameProfileCmd`、`ModeJustChanged` → 已落
//!   country-core `resources::onboarding`/`events`；SteamInput → `resources::steam`。
//! - 仍缺（登记上游缺口）：toolbar/mei（hint_draw/render_hints 渲染 DSL）、
//!   geo_src_to_edit_subject（hint_focused_edit_hint/hint_clutter_focused_edit_hint/
//!   hint_roof_shape 展示链）、`PreviewCircle+0x20`/`PreviewRectangle+0x28` 放置点 Y
//!   （hint_position_update 7/8 的 Y 占位 U）、`ToolbarUiIsInFocus`/`ClutterMeshes`
//!   的 country-core 类型（本地同名事件/省略参数，接线 U）。
//! - 已闭合（2026-10-17 consume-hint）：hint_orchestrator 全控制流（收集/排序/门控/
//!   实体增删迁移/冷却/三条文本模板）与 hint_position_update case 6/7/8/10。

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
    // O：`ToolbarUiIsInFocus` 属 toolbar plugin 的注册面（country-core 尚未落该类型）；
    // 本模块以本地同名事件承接 `hint_orchestrator` 的 reader，故在此注册（接线 U）。
    world.init_resource::<Events<hint_orchestrator::ToolbarUiIsInFocus>>();
}

/// NotifyHintSystem* 事件宿主载荷字节 seam（O：native 各 hint_* 读 `事件+8` 载荷字节
/// 分支展示/完成；country-core 事件为 unit，**载荷字节 U** 未落字段）。本重建以常量 0
/// （展示路径）建模——完成路径不触发，登记 U 待 core 补载荷字段后替换。
pub fn event_payload_byte_seam() -> u8 {
    0
}

/// prompt_rollercoaster（O 0x14227bb70，395）：EventReader<DisplayRollercoasterPrompt> →
/// `Steam(packed_set_plus_one) → alternate(Option<KeyCode>)` 写 DisplayHintRequest
/// （**placement ScreenCenter(10) @+0 字节**；+0x48 u16 = **0x1200** → flags 0、
/// id@+0x49 = **0x12** = `HintType::Rollercoaster`，orchestrator 对 0x12 使用独立
/// 文本模板）。提示文本 lang key "Rollercoaster.Ride"。
///
/// **已重建（2026-10-17，coregap-2 DisplayRollercoasterPrompt 落库 + id 判等）**；
/// 分歧点：Steam 动作码 index / 键码来源（GameKeyBindings 槽位）U → 本重建以
/// `steam_query(0x101)` + `Key(None)` 占位（结构/事件流判等，载荷内容待补证）。
pub fn prompt_rollercoaster(
    mut events: EventReader<country_core::resources::events::DisplayRollercoasterPrompt>,
    mut writer: EventWriter<DisplayHintRequest>,
) {
    if events.read().next().is_some() {
        // O：`impl$31::from()` = `packed_set_plus_one(index)`（Steam 动作码）→
        // `alternate<Option<KeyCode>>`（提示键帽）。
        let display = BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(0x101))
            .alternate(None::<KeyCode>);
        writer.write(
            DisplayHintRequest::new(
                display,
                crate::hint_orchestrator::HintType::Rollercoaster,
                0,
            )
            .with_placement(crate::hint_orchestrator::HintPlacement::ScreenCenter),
        );
    }
}
