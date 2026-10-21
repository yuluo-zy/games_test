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
pub fn configure(world: &mut World, schedule: &mut bevy_ecs::schedule::Schedule) {
    // Reuse the existing original startup constructor once, when the host has
    // provided its profile and no terrain hint conditions were supplied.
    if world.contains_resource::<country_core::game_profile::GameProfile>()
        && !world.contains_resource::<hint_terrain::HintTerrainConditions>()
        && !world.contains_resource::<hint_terrain_focus::HintFocusTerrainConditions>()
        && !world.contains_resource::<hint_terrain_steepness::HintTerrainSteepnessConditions>()
    {
        use bevy_ecs::system::RunSystemOnce;
        world.run_system_once(hints_startup::hints_startup)
            .expect("initializing hint conditions from the loaded profile");
    }
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

    world.init_resource::<Events<country_core::resources::onboarding::NotifyHintSystemTerrain>>();
    world.init_resource::<Events<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>>();
    world.init_resource::<Events<country_core::resources::events::WriteSettingsFileCmd>>();
    world.init_resource::<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>();
    world.init_resource::<Events<DisplayHintRequest>>();
    world.init_resource::<Events<LingerHintRequest>>();
    world.init_resource::<Events<tutorial_completion::TutorialHintComplete>>();
    world.init_resource::<Events<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>>();
    // The CPU-only configuration needs the same core-owned event storage as
    // the toolbar producer. The full App registers it through ToolbarPlugin.
    world.init_resource::<Events<hint_orchestrator::ToolbarUiIsInFocus>>();
    // Current CPU consumer order. The upper host supplies profile/AppMode and
    // the actual core terrain producers; this does not create a core->plugin dependency.
    schedule.add_systems((
        hint_terrain::hint_terrain_layering,
        hint_terrain_focus::hint_terrain_focus,
        hint_terrain_steepness::hint_terrain_steepness,
        tutorial_completion::tutorial_completion,
    ).chain().after(country_core::systems::terrain::TerrainCpuStage::Changed));
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

/// `<bevy_app::App as system_onboarding::OnboardingSystems>::add_onboarding`（0x140650d50）。
///
/// 机器码（PyGhidra 逐指令；每个系统名取自 `Box<dyn System>`/`into_configs` 单体内的
/// 类型名串，顺序取自单体内 into_configs 调用顺序）：
///
/// | 调用点 | 动作 |
/// |---|---|
/// | 0x140650de9 | `Schedules::add_systems(Startup, hints_startup)`（标签 vtable 0x142f006f0 = "Startup"） |
/// | 0x140650df2/0x140650dfe/0x140650e0a | 三条 `chain_ignore_deferred`（链 1 0x140eaf480、链 2 0x140eb14a0、链 3 0x140eb6830） |
/// | 0x140650f16 | 外层 `(链1, 链2, 链3).chain_ignore_deferred()` |
/// | 0x140650f49 | `.in_set(GameSystemSet(0x0b))` |
/// | 0x140650fa2 | `SubApp::add_systems(Update, 外层链)`（标签 vtable 0x142f00728 = "Update"） |
/// | 0x140650fb7 | `hint_draw.in_set(GameSystemSet(0x24))`（单系统，ZST `in_set` 单体 0x140eb7990） |
/// | 0x140650fc2 | `SubApp::add_systems(Update, hint_draw…)` |
/// | 0x14065101e…0x14065174e | 21 次 `register_event`（先 `get_resource_id` 判重，等价 `add_event`），顺序见下方实现 |
///
/// 原版没有 `insert_resource`/`init_resource`：各 `*HintConditions` 等资源由 `hints_startup`
/// 与系统参数的 `FromWorld`/宿主创建。未闭合：本仓库 `configure()` 里的
/// `ToolbarUiIsInFocus` 事件属 toolbar 插件，不在原 add_onboarding 面内，本 trait 不注册。
pub trait OnboardingSystems {
    fn add_onboarding(&mut self) -> &mut Self;
}

impl OnboardingSystems for bevy_app::App {
    fn add_onboarding(&mut self) -> &mut Self {
        use bevy_app::{Startup, Update};
        use country_core::resources::events::DisplayRollercoasterPrompt;
        use country_core::resources::onboarding::*;
        use country_core::systems::GameSystemSet;

        self.add_systems(Startup, hints_startup::hints_startup);

        // 链 1（0x140eaf480）。
        let chain1 = (
            reset_hints::reset_hints,
            hint_deselect_tool::hint_deselect_tool,
            hint_focused_edit::hint_focused_edit_condition,
            hint_focused_edit::hint_focused_edit_hint,
        )
            .chain_ignore_deferred();
        // 链 2（0x140eb14a0）。
        let chain2 = (
            hint_clutter_focus_edit::hint_clutter_focused_edit_condition,
            hint_clutter_focus_edit::hint_clutter_focused_edit_hint,
            hint_disable_snapping::hint_disable_snapping,
            hint_terrain_steepness::hint_terrain_steepness,
            hint_terrain::hint_terrain_layering,
            hint_terrain_focus::hint_terrain_focus,
            hint_roof_shape::hint_roof_shape,
            hint_unlink_windows::hint_unlink_windows,
            hint_camera_progress::hint_camera_progress_update,
            hint_camera_controls::hint_camera_enable_condition,
            hint_camera_controls::hint_camera_display_condition,
            hint_undo::hint_undo,
            hint_stairs::hint_stairs,
            prompt_rollercoaster,
            exponential_back_off::exponential_back_off,
            tutorial_completion::tutorial_completion,
            hint_orchestrator::hint_orchestrator,
            hint_camera_progress::hint_camera_progress_reset,
        )
            .chain_ignore_deferred();
        // 链 3（0x140eb6830）。
        let chain3 = (
            hint_display_system::hint_audio,
            hint_display_system::hint_deletion_cooldown_system,
            hint_display_system::hint_animation_system,
            hint_display_system::hint_deletion_system,
            hint_display_system::hint_position_update,
            hint_display_system::hint_highlight_shape,
            hint_display_system::progress_bar_update_system,
        )
            .chain_ignore_deferred();

        self.add_systems(
            Update,
            (chain1, chain2, chain3)
                .chain_ignore_deferred()
                .in_set(GameSystemSet(0x0b)),
        );
        self.add_systems(
            Update,
            hint_display_system::hint_draw.in_set(GameSystemSet(0x24)),
        );

        self.add_event::<NotifyHintSystemShapeEditWasPerformed>();
        self.add_event::<DisplayHintRequest>();
        self.add_event::<NotifyHintSystemCameraMoved>();
        self.add_event::<NotifyHintSystemSnapping>();
        self.add_event::<NotifyHintSystemTerrain>();
        self.add_event::<LingerHintRequest>();
        self.add_event::<TutorialOp>();
        self.add_event::<tutorial_completion::TutorialHintComplete>();
        self.add_event::<NotifyHintSystemWindow>();
        self.add_event::<NotifyHintSystemToolWasJustUsed>();
        self.add_event::<NotifyHintSystemSomethingWasErased>();
        self.add_event::<NotifyHintSystemTerrainSteepness>();
        self.add_event::<NotifyHintSystemRoofShape>();
        self.add_event::<NotifyHintSystemClutterWasMoved>();
        self.add_event::<NotifyHintSystemStairsPlaced>();
        self.add_event::<NotifyHintSystemHoveringOverStairs>();
        self.add_event::<NotifyHintSystemStartedStairsContinue>();
        self.add_event::<NotifyHintSystemCanContinueStairs>();
        self.add_event::<NotifyHintSystemStairsDetachedFromWall>();
        self.add_event::<NotifyHintSystemStairsCanDetachFromWall>();
        self.add_event::<DisplayRollercoasterPrompt>();
        self
    }
}
