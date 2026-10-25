//! system-onboarding（B 主基线）：新手引导提示系统。
//!
//! 类型/函数地址与布局证据：`upper-modules-plan/systems/onboarding/ANALYSIS.md` +
//! 本模块 evidence/（fresh Ghidra shared 反编译，get_functions fields=signature,
//! decompiled_code）。恢复轮次按用户硬标准（2026-10-16）：真业务函数 fresh 反编译 →
//! 重建原函数完整控制流图 → Rust body 逐语句判等；重建不了标 `未实现`（列出分歧点），
//! 不以近似体通过。
//!
//! 本轮公共提示接线（2026-10-24）：真实 typed 键／鼠标身份、原版 MEI 提示模板、
//! Graphic 静态／动画、吸附 bool 通知与完成保存闭环已接入现有 owner。
//! GameProfile 仍为唯一权威，持久化使用原19项 TutorialCompletionState 与嵌套冷却结构。
//! position 系统读取真实 Preview 放置平面与 ClutterMeshes，并恢复主体高亮及实体卸载。
//! 具体原证、有限差分范围与尚待核对的提示条件／构造器见本轮业务报告；
//! 这里不以事件类型存在或 CPU 建树成功宣称 OS 输入／GPU 画面验收完成。

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

/// 保留原Bevy物理键及鼠标身份；原生平台载荷与Other按钮不得退化为数字别名。
pub use bevy_input::{keyboard::KeyCode, mouse::MouseButton};

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
/// `configure` 是既有 CPU 宿主的地形提示适配入口；完整游戏注册使用下方
/// `OnboardingSystems::add_onboarding`，包括生产 MEI draw 及事件生命周期。
pub fn configure(world: &mut World, schedule: &mut bevy_ecs::schedule::Schedule) {
    // Reuse the existing original startup constructor once, when the host has
    // provided its profile and no terrain hint conditions were supplied.
    if world.contains_resource::<country_core::game_profile::GameProfile>()
        && !world.contains_resource::<hint_terrain::HintTerrainConditions>()
        && !world.contains_resource::<hint_terrain_focus::HintFocusTerrainConditions>()
        && !world.contains_resource::<hint_terrain_steepness::HintTerrainSteepnessConditions>()
    {
        use bevy_ecs::system::RunSystemOnce;
        world
            .run_system_once(hints_startup::hints_startup)
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
    world.init_resource::<Events<country_core::game_profile::SaveGameProfileCmd>>();
    world
        .init_resource::<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>(
        );
    world.init_resource::<Events<DisplayHintRequest>>();
    world.init_resource::<Events<LingerHintRequest>>();
    world.init_resource::<Events<tutorial_completion::TutorialHintComplete>>();
    world.init_resource::<Events<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>>();
    // The CPU-only configuration needs the same core-owned event storage as
    // the toolbar producer. The full App registers it through ToolbarPlugin.
    world.init_resource::<Events<hint_orchestrator::ToolbarUiIsInFocus>>();
    // Current CPU consumer order. The upper host supplies profile/AppMode and
    // the actual core terrain producers; this does not create a core->plugin dependency.
    schedule.add_systems(
        (
            hint_terrain::hint_terrain_layering,
            hint_terrain_focus::hint_terrain_focus,
            hint_terrain_steepness::hint_terrain_steepness,
            tutorial_completion::tutorial_completion,
        )
            .chain()
            .after(country_core::systems::terrain::TerrainCpuStage::Changed),
    );
}

/// NotifyHintSystem* 事件宿主载荷字节 seam（O：native 各 hint_* 读 `事件+8` 载荷字节
/// 分支展示/完成；country-core 事件为 unit，**载荷字节 U** 未落字段）。本重建以常量 0
/// （展示路径）建模——完成路径不触发，登记 U 待 core 补载荷字段后替换。
pub fn event_payload_byte_seam() -> u8 {
    0
}

/// 原 B `prompt_rollercoaster`（0x14227bb70）：整批提示事件合并成一个
/// ScreenCenter 请求。Steam 动作 20 的打包查询是 0x1401；键盘候选取玩家档案
/// 中的 `ui_confirm`（槽 29，原 GameProfile+0x15c），因此改键后立即生效。
pub fn prompt_rollercoaster(
    mut events: EventReader<country_core::resources::events::DisplayRollercoasterPrompt>,
    mut writer: EventWriter<DisplayHintRequest>,
    profile: Res<country_core::game_profile::GameProfile>,
) {
    // 原版把 reader 游标直接推进至事件队尾；不能只 next() 留下旧事件反复展示。
    if events.read().count() != 0 {
        let confirm = profile.key_bindings.bindings[29];
        let display = BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(0x1401))
            .alternate(confirm);
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

#[cfg(test)]
mod rollercoaster_prompt_tests {
    use super::*;
    use country_core::{game_profile::GameProfile, resources::events::DisplayRollercoasterPrompt};

    #[test]
    fn original_prompt_uses_rebound_confirm_and_consumes_the_whole_event_batch() {
        let mut world = World::new();
        world.init_resource::<Events<DisplayRollercoasterPrompt>>();
        world.init_resource::<Events<DisplayHintRequest>>();
        world.insert_resource(GameProfile::default());
        let mut schedule = Schedule::default();
        schedule.add_systems(prompt_rollercoaster);
        let rebound = world.resource::<GameProfile>().key_bindings.bindings[0];
        for binding in [rebound, None] {
            world.resource_mut::<GameProfile>().key_bindings.bindings[29] = binding;
            world.send_event(DisplayRollercoasterPrompt);
            world.send_event(DisplayRollercoasterPrompt);
            schedule.run(&mut world);
            let requests = world.resource::<Events<DisplayHintRequest>>();
            assert_eq!(requests.len(), 1);
            let mut cursor = requests.get_cursor();
            let request = cursor.read(requests).next().unwrap();
            assert_eq!(request.id, crate::HintType::Rollercoaster as u8);
            assert_eq!(request.placement, crate::HintPlacement::ScreenCenter);
            assert_eq!(request.flags, 0);
            assert_eq!(
                request.display,
                BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(0x1401),)
                    .alternate(binding)
            );
            world.resource_mut::<Events<DisplayHintRequest>>().clear();
            schedule.run(&mut world);
            assert!(world.resource::<Events<DisplayHintRequest>>().is_empty());
        }
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
        self.add_event::<country_core::resources::events::DisplayBetaWIPWarning>();
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
