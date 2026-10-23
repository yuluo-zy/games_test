//! toolbar 交互 UI（B 版恢复，restorer-wave3-toolbar-layout，2026-10-17）。
//!
//! 恢复范围（wave3 fresh 反编译 + caller SystemParam 签名）：
//! - 事件类型：`UiJustBecameInFocusEvent`/`UiClickEvent`/`UiDragEvent`
//!   （`toolbar::ui_interaction`，add_toolbar_item 的 UI 交互事件族）。
//! - `ui_animation_state_update`（0x14233b3f0）：逐工具栏项更新 `UiAnimationState`
//!   外观/选择进度（公式+字节判别判等，见下）。
//! - `ui_sounds`（0x14233a7f0，真地址）：**未实现（分歧见模块尾与 RESTORE.md）**。
//! - `render_taffy_ui_to_widgets`（0x142338990）：焦点交换 / keep 面 / 事件写面
//!   已恢复实际屏幕矩形与距离、回放输入屏蔽及真实 icon_cross 闭包。
//!
//! gap-utils-toolbar（2026-10-17）补落库类型：`UiNav`/`HistoryButton` 组件（见
//! `resouces_and_components`）；`WindowFocusEvent` 事件已由 country-core
//! `resources::window` 落库（mode_manager 组在途，本 crate 直接引用）——render_taffy 的
//! `Option<&HistoryButton>, Option<&UiNav>` Query 面与 `grow_one<EventInstance<
//! WindowFocusEvent>>` 写面目标类型已具备，13 参接线仍待落库（见模块尾）。

use bevy_ecs::entity::Entity;
use bevy_ecs::event::{Event, EventReader, EventWriter};
use bevy_ecs::prelude::{Commands, Query, Res, ResMut};
use bevy_time::prelude::Time;

use country_core::systems::ui::resources::widget_params::{UiNavState, UiSounds, WidgetParams};

use crate::resouces_and_components::{
    HistoryButton, TaffyNode, UiAnimationState, UiInteractionState, UiLocked, UiMouseInteractable,
    UiNav, UiPlayLockedAnimation, UiPlaySelectAnimation,
};

/// B 0x14233e960: hide only UiMouseInteractable entities by zeroing size.
/// Position and entities without this exact core component remain untouched.
pub fn debug_hide_ui(
    debug: Res<country_core::resources::gui::DebugGui>,
    mut drawables: Query<(
        &mut country_core::components::Ui2dExtent,
        &country_core::components::UiMouseInteractable,
    )>,
) {
    if !debug.show_2d_ui {
        for (mut extent, _) in &mut drawables {
            extent.size = glam::Vec2::ZERO;
        }
    }
}

#[cfg(test)]
mod native_debug_hide_tests {
    use super::*;
    use bevy_ecs::prelude::{Schedule, World};
    use country_core::{components::Ui2dExtent, resources::gui::DebugGui};
    use glam::Vec2;

    #[test]
    fn toolbar_producer_and_core_consumer_share_interactable_identity() {
        let mut world = World::new();
        world.insert_resource(DebugGui {
            show_2d_ui: false,
            ..Default::default()
        });
        // Use the toolbar producer's path, consumed by the canonical core Query.
        let affected = world
            .spawn((
                Ui2dExtent {
                    pos: Vec2::new(5.0, 7.0),
                    size: Vec2::new(20.0, 10.0),
                },
                crate::resouces_and_components::UiMouseInteractable::default(),
            ))
            .id();
        let ordinary = world
            .spawn(Ui2dExtent {
                pos: Vec2::ZERO,
                size: Vec2::ONE,
            })
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(debug_hide_ui);
        schedule.run(&mut world);
        assert_eq!(world.get::<Ui2dExtent>(affected).unwrap().size, Vec2::ZERO);
        assert_eq!(
            world.get::<Ui2dExtent>(affected).unwrap().pos,
            Vec2::new(5.0, 7.0)
        );
        assert_eq!(world.get::<Ui2dExtent>(ordinary).unwrap().size, Vec2::ONE);
    }

    /// O 0x14233c300: pulse formula, progress clamp and completion removal.
    #[test]
    fn select_animation_pulses_scale_then_removes_component() {
        use crate::resouces_and_components::{TaffyNode, UiPlaySelectAnimation};
        use bevy_ecs::prelude::World;
        use std::time::Duration;

        let mut world = World::new();
        world.insert_resource(Time::<()>::default());
        world
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f32(0.1));
        let entity = world
            .spawn((
                TaffyNode::default(),
                UiPlaySelectAnimation {
                    a: 0.2,
                    b: 0.2,
                    progress: 0.0,
                },
            ))
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_select_animation);
        schedule.run(&mut world);
        let animation = *world.get::<UiPlaySelectAnimation>(entity).unwrap();
        assert_eq!(animation.progress, 0.5);
        let pulse = ((1.0 - 0.5) * 0.1 + 0.5) * 4.0 * (1.0 - 0.5);
        let scale = world.get::<TaffyNode>(entity).unwrap().scale();
        assert!((scale - (pulse * pulse * 0.2 + 1.0)).abs() < 1e-6);
        // Second step clamps at 1.0 (pulse 0) and the command removes it.
        schedule.run(&mut world);
        world.flush();
        assert!(world.get::<UiPlaySelectAnimation>(entity).is_none());
    }

    /// O: the original sinf/divss sequence preserves the zero singularity.
    #[test]
    fn sinc_preserves_original_zero_singularity() {
        assert!(sinc(0.0).is_nan());
        assert!(sinc(-0.0).is_nan());
        assert!((sinc(1.0) - 1.0f32.sin()).abs() < 1e-6);
    }
}

/// UI 刚聚焦事件（O：`toolbar::ui_interaction::UiJustBecameInFocusEvent`；ui_sounds
/// 的 EventReader 目标）。**重建（I）**：载荷面 U（EventInstance 步长 0x10 读面，
/// 实体 @+8）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct UiJustBecameInFocusEvent {
    /// 实体（I；EventInstance 记录 @+8）。
    pub entity: Entity,
}

/// UI 点击事件（O：`toolbar::ui_interaction::UiClickEvent`；switch_*_button /
/// tod_v2_button / ui_sounds 的 EventReader 目标）。
///
/// **重建（B 读面，wave3 fresh）**：EventInstance 记录 stride 0x10，
/// `{event_id u64@+0, entity Entity@+8}` ——事件载荷 = 被点击实体。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct UiClickEvent {
    /// 被点击实体（@+8）。
    pub entity: Entity,
}

/// UI 拖拽事件（O：`toolbar::ui_interaction::UiDragEvent`；ui_sounds 的
/// EventReader 目标）。
///
/// **fresh 2026-10-18 判等（render_taffy 0x14233998b..0x1423399d2 / 写入
/// 0x142338f20）**：payload = `{entity u64 @+0x08, state u8 @+0x10}`，
/// EventInstance stride **0x18**（事件 id @+0）。Ghidra 该 `grow_one` 符号为
/// `tuple<&str, SettingsMenuState>`，与实测载荷（Entity + 单字节判别）不符，
/// 以机器码为准（符号串 U）。`state` 原样保存 `into_drag_state()` 判别
/// （0/1/3，`!=2` 时发射）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct UiDragEvent {
    /// 实体（@+0x08）。
    pub entity: Entity,
    /// 拖拽判别原始字节（@+0x10；`SettingsMenuState` 语义 U）。
    pub state: u8,
}

/// O 0x14233b3f0 `toolbar::ui_interaction::ui_animation_state_update`。
///
/// 签名（caller mangled 名）：`Query<(&UiInteractionState, &mut UiAnimationState)>,
/// Res<bevy_time::time::Time<>>`。
///
/// **判等（per-widget 公式，gap-utils-toolbar 2026-10-17 补字节判别，逐语句自 fresh
/// 反编译 0x14233b3f0）**：
/// - `keep-byte`（per-widget stride-3 记录 `+1`）为 0 → 离开态：
///   `select += dt/-0.2`、state→1；否则进入态：`select += dt/0.2`、state→0/2。
/// - `select = clamp01(select)`；`appear = clamp01(appear + 4·dt·sign)`，
///   sign = (stride-3 记录 `+0` byte) 为 0 → -1 否则 +1。
///
/// **判等（公式+state 写面；reviewer-A-toolbar FIX-T1）**：`{f32@0,+4·dt},{f32@4,±dt/0.2},
/// byte@0xc{0,1,2}` 逐语句对 fresh 反编译；`update_progress` 含 state 0/1/2 转写。
/// keep/appear 符号源 = `interaction.keep`/`interaction.state`（**B 组件本身 3B**，
/// 非独立 WidgetByteState）；bevy `&mut` 自动记 changed-tick（原生 stride-4 u32 写面）。
pub fn ui_animation_state_update(
    mut query: Query<(&UiInteractionState, &mut UiAnimationState)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (interaction, mut anim) in &mut query {
        // 进入/离开判别源 = interaction+1（keep）。
        anim.update_progress(dt, interaction.keep != 0);
        // appear：`+= 4·dt·sign`，sign = interaction+0（`==0` → -1 否则 +1）。
        let sign = if interaction.state == 0 { -1.0 } else { 1.0 };
        anim.appear = (anim.appear + 4.0 * dt * sign).clamp(0.0, 1.0);
    }
}

/// O `sinc` (inline, 2 instances; host ui_locked_animation): `sin(x) / x`.
/// The original 0x14233cd34..0x14233cd39 has no zero branch: preserve its
/// NaN at the removable singularity instead of changing the game algorithm.
pub fn sinc(x: f32) -> f32 {
    x.sin() / x
}

/// O 0x14233c300 `ui_select_animation` (fresh decompile
/// `evidence/core-recovery-20261015/gui/0x14233c300.c`):
/// - `p = clamp01(p + dt / b)` stored back at component +8;
/// - `pulse = ((1 - p) * 0.1 + p) * 4 * (1 - p)`;
/// - `scale *= pulse * pulse * a + 1` at TaffyNode+0x10;
/// - `p == 1` removes the component through the entity command queue.
///
/// Parameter A is the pop amplitude and B the duration
/// (`UiPlaySelectAnimation{0.1/0.2, 0.2}` at the original producers).
pub fn ui_select_animation(
    time: Res<Time>,
    mut query: Query<(Entity, &mut TaffyNode, &mut UiPlaySelectAnimation)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (entity, mut taffy, mut animation) in &mut query {
        let progress = (animation.progress + dt / animation.b).clamp(0.0, 1.0);
        animation.progress = progress;
        let pulse = ((1.0 - progress) * 0.1 + progress) * 4.0 * (1.0 - progress);
        let scale = (pulse * pulse * animation.a + 1.0) * taffy.scale();
        taffy.set_scale(scale);
        if progress == 1.0 {
            commands.entity(entity).remove::<UiPlaySelectAnimation>();
        }
    }
}

/// O 0x14233c8b0 `ui_locked_animation`.
///
/// Exact five-parameter PDB caller spelling is retained in skeleton/B
/// ui_interaction.rs, function information da21103e06ec109a0ec604e3.
/// First drain clicks and queue `UiPlayLockedAnimation { a: 0.1, b: 0.3,
/// progress: 0.0 }` only for live entities carrying UiLocked. Then animate
/// components already in the query: deferred click inserts become visible
/// at the original command-application boundary, not during this loop.
/// Original 0x14233cd34..0x14233cd39 performs sinf(x)/x with no zero guard.
pub fn ui_locked_animation(
    mut clicks: EventReader<UiClickEvent>,
    locked: Query<&UiLocked>,
    mut query: Query<(Entity, &mut UiPlayLockedAnimation, &mut TaffyNode)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for click in clicks.read() {
        if locked.get(click.entity).is_ok() {
            commands.entity(click.entity).insert(UiPlayLockedAnimation {
                a: 0.1,
                b: 0.3,
                progress: 0.0,
            });
        }
    }
    let dt = time.delta_secs();
    for (entity, mut animation, mut taffy) in &mut query {
        let progress = (dt / animation.b + animation.progress).clamp(0.0, 1.0);
        animation.progress = progress;
        let x = (progress * 5.0 + -1.0) * std::f32::consts::PI;
        let amplitude = sinc(x);
        let scale = (amplitude * animation.a + 1.0) * taffy.scale();
        taffy.set_scale(scale);
        if progress == 1.0 {
            // Native removal command uses the warning handler (insertion uses panic).
            commands.entity(entity).remove::<UiPlayLockedAnimation>();
        }
    }
}

/// Complete native sound consumer, sharing the canonical event/component owners.
pub use crate::ui_presentation::ui_sounds;

/// O 0x142338990 `toolbar::ui_interaction::render_taffy_ui_to_widgets`。
///
/// **13 参接线判等（caller `run_unsafe<FunctionSystem<...>>` mangled 名逐参）**：
/// 1. `Query<(Entity, &Ui2dExtent, &UiMouseInteractable, &TaffyNode,
///    &mut UiInteractionState, Option<&HistoryButton>, Option<&UiNav>)>`
/// 2. `Res<UiState>`
/// 3. `ResMut<UiNavState>`
/// 4. `EventWriter<UiClickEvent>`
/// 5. `EventWriter<UiDragEvent>`
/// 6. `Res<AppMode>`
/// 7. `Res<UiSignifierStream>`
/// 8. `Res<CursorPositionSS>`
/// 9. `Res<GameConfig>`
/// 10. `Res<ActionSetStates>`
/// 11. `EventWriter<UiHistoryButtonPressed>`
/// 12. `Commands`
/// 13. `Res<ToolbarSizeMult>`
///
/// 主体（fresh 2026-10-18 复扫，
/// `work/restorer-mei-final/evidence/render_taffy_ui_to_widgets.asm.txt`）：
/// - 前段焦点交换 **判等**（0x142338c60..0x142338cc6）：
///   `nav.focus.is_toolbar() && (actions.digital[0].pressed() ||
///   actions.analog[1].length() > 1.0)` → `staged_focus = focus`，焦点 qword 清零。
/// - `QueryIterationCursor::init` 后逐 widget：
///   1. 名字 = `format!("toolbar_button_{:?}", taffy_node)`（L494-502）。
///   2. 矩形扩张：`expand = game_config.ui.icon_size_mult * 10.0 * size_mult`
///      （fresh 0x142339522 读 `GameConfig+0xe8`）；`rect = extent ± expand/2`。
///   3. `WidgetParams` 链 → `UiState::add::<String>`。
///   4. per-widget keep（`UiInteractionState+1`）**判等**：清零后若
///      `nav.focus.is_toolbar() && payload == entity` 置 1。
///   5. 有 `HistoryButton`：`into_drag_state()`（is_some→
///      `UiHistoryButtonPressed{kind}`；==3→`Commands::entity.insert(
///      UiPlaySelectAnimation{0.2,0.2,0})`）；无 HistoryButton：Replay 且 active → 实际 icon_cross factory，随后跳过事件尾。
///   6. 无焦点且 state==1 的首个 UiNav 按钮接管 Toolbar 焦点（**判等**）。
///   7. 所有 widget 共享尾：`clicked()`→`UiClickEvent{entity}`；
///      `into_drag_state() != 2`→`UiDragEvent{entity,state}`（**判等**）。
///
/// ScreenAabb and replay-denied factory were rechecked against original
///1423395cb..637 and1423397b0..884 on2026-10-21. The prior field/tag
/// projections omitted bounds/depth and confused Replay13 with Photomode19.
#[allow(clippy::too_many_arguments)]
pub fn render_taffy_ui_to_widgets(
    mut query: Query<(
        Entity,
        &country_core::components::Ui2dExtent,
        &UiMouseInteractable,
        &TaffyNode,
        &mut UiInteractionState,
        Option<&HistoryButton>,
        Option<&UiNav>,
    )>,
    ui_state: Res<country_core::systems::ui::resources::ui_state::UiState>,
    mut nav_state: ResMut<UiNavState>,
    mut click_events: EventWriter<UiClickEvent>,
    mut drag_events: EventWriter<UiDragEvent>,
    app_mode: Res<country_core::systems::mode_manager::AppMode>,
    mut signifiers: ResMut<
        country_core::systems::ui::signifiers::signifier_create::UiSignifierStream,
    >,
    cursor: Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
    game_config: Res<country_core::game_config::GameConfig>,
    actions: Res<country_core::input::action_set::ActionSetStates>,
    mut history_events: EventWriter<
        country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed,
    >,
    mut commands: Commands,
    size_mult: Res<crate::toolbar_update::ToolbarSizeMult>,
) {
    let _ = &cursor;

    // ------------------------------------------------------------------
    // O L240-241 / 0x142338c4a..0x142338cc6：焦点交换。
    // fresh：`nav.focus.is_toolbar() && (digital[0].pressed() ||
    // analog[1].length() > 1.0)` 时
    //   `nav.staged_focus = nav.focus; *(qword*)(nav+0x10) = 0;`
    // （+0x18 payload 不写；ResMut 变更 tick = &mut 语义）。
    // `digital[0]` = "click"，`analog[1]` = "hardware_window_space_cursor_move"。
    // **判等**（2026-10-18 fresh 0x142338c60..0x142338cc6）。
    // ------------------------------------------------------------------
    if nav_state.focus.is_toolbar() {
        let pressed = actions.digital[0].pressed();
        let moved = actions.analog[1].length() > 1.0;
        if pressed || moved {
            nav_state.staged_focus = nav_state.focus;
            nav_state.focus.tag = 0;
            nav_state.focus._pad = 0;
        }
    }
    // 焦点是否为空：原生 `qword [nav+0x10] == 0`（tag+pad），在迭代前取一次
    // （0x142338cd5..0x142338cda）。
    let focus_was_none = nav_state.focus.tag == 0 && nav_state.focus._pad == 0;

    // O L494-513：`fVar26 = game_config[+0xe8] * 10.0 * size_mult`。
    // fresh 0x142339522：`[GameConfig+0xe8]` = `UiSettings.icon_size_mult`（f32，
    // +0xe8 为 UiSettings 首字段）。**判等**。
    let config_scale = game_config.ui.icon_size_mult;
    let expand = config_scale * 10.0 * size_mult.multiplier;

    for (entity, extent, interactable, taffy, mut interaction, history, nav) in &mut query {
        // O L494-502：名字 = `format!("toolbar_button_{:?}", node)`。
        let name = format!("toolbar_button_{:?}", taffy.node);

        // O L503-513：`fVar25 = (extent.size + expand) * 0.5`；
        // `rect = {extent.pos - half, extent.pos + half}`；
        // WidgetParams volume size = `extent.size + expand`
        // （fresh 0x142338990：`+0x00` pos Vec2、`+0x08` size Vec2）。
        let rect_size = extent.size + glam::Vec2::splat(expand);

        // Original1423395cb/5e6: preserve both screen bounds and the
        // UiMouseInteractable f32 distance. It is not an interaction layer.
        let half_size = rect_size * 0.5;
        let params = WidgetParams::default()
            .screen_aabb(
                extent.pos - half_size,
                extent.pos + half_size,
                f32::from_le_bytes(interactable.bytes),
            )
            .no_history_recording()
            // Original142339637 clears the bool argument before this call.
            .occlude_by_world(false)
            .allow_negative_distance(true)
            .sounds(UiSounds::empty());
        let ctx = ui_state.add(&name, params);

        // ------------------------------------------------------------------
        // O 0x1423394c3..0x142339515：per-widget keep 字节（UiInteractionState+1）。
        // `keep = 0`；若 `nav.focus.is_toolbar() && nav.focus.payload == entity`
        // → `keep = 1`（`&mut` 自动记 changed tick）。**判等**。
        // ------------------------------------------------------------------
        interaction.keep = 0;
        let nav_focused_this =
            nav_state.focus.tag == 1 && nav_state.focus.payload == entity.to_bits();
        if nav_focused_this {
            interaction.keep = 1;
        }

        // O L559-595：`slider3d::into_drag_state()`（0x14092df10；纯读取，
        // 原生在 HistoryButton/共享尾部分别调用，本实现复用同值）。
        let drag = ctx.into_drag_state();

        if let Some(history) = history {
            // is_some → push `UiHistoryButtonPressed{kind}`（1B 载荷，
            // L562-577 写 EventInstance stride 0x10 的 +8 字节）。
            if drag.is_some() {
                history_events.write(
                    country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed(
                        history.kind,
                    ),
                );
            }
            // O L578-595：`into_drag_state() == 3` →
            // `Commands::entity(e).insert(UiPlaySelectAnimation{0.2,0.2,0})`
            // （命令记录 0x28 B：fn/handler/entity/payload）。
            if drag.is_started() {
                commands.entity(entity).insert(
                    crate::resouces_and_components::UiPlaySelectAnimation {
                        a: 0.2,
                        b: 0.2,
                        progress: 0.,
                    },
                );
            }
        } else if *app_mode == country_core::systems::mode_manager::AppMode::Replay {
            // Original1423397b7 tests native AppMode13 (Replay), not
            // Photomode19. Both active and inactive paths continue directly
            // to the next item: ordinary replay toolbar widgets cannot emit
            // focus, click or drag events. History controls keep their path.
            if ctx.is_active() {
                signifiers.write_factory(&replay::denied_input_signifier(
                    cursor.0,
                    config_scale * 60.0,
                ));
            }
            continue;
        }

        // ------------------------------------------------------------------
        // O 0x142339890..0x142339904：无焦点的首个活跃可导航按钮接管焦点。
        // `focus_was_none && interaction.state == 1 && ctx.is_active() &&
        // Option<&UiNav>.is_some()` → `keep = 1`；若当前焦点仍为空 →
        // `nav.focus = Toolbar(entity)`（内层重查防止后续项覆盖）。
        // **判等**（UiNav 存在性 = fresh Option 槽标志；位置在分支之后）。
        // ------------------------------------------------------------------
        if focus_was_none && interaction.state == 1 && ctx.is_active() && nav.is_some() {
            interaction.keep = 1;
            if nav_state.focus.tag == 0 && nav_state.focus._pad == 0 {
                nav_state.focus.tag = 1;
                nav_state.focus._pad = 0;
                nav_state.focus.payload = entity.to_bits();
            }
        }

        // O L609-624 / 0x142339910..0x142339976：`clicked()` →
        // `EventWriter<UiClickEvent>`（事件 stride 0x10：event id@+0，entity@+8）。
        // 对所有 widget 生效（分支汇合后在共享尾部执行）。
        if ctx.clicked() {
            click_events.write(UiClickEvent { entity });
        }
        // O L625-642 / 0x14233997e..0x1423399d2：`into_drag_state() != 2` →
        // `UiDragEvent{entity, state}`（事件 stride 0x18：id@+0、entity@+8、
        // 判别字节@+0x10）。**判等**（符号名 tuple<&str,...> 不符，见类型注释）。
        if drag.0 != 2 {
            drag_events.write(UiDragEvent {
                entity,
                state: drag.0,
            });
        }
    }
}

mod focus;
mod hide;
mod ordering;
mod replay;
mod navigation {
    //! Native toolbar navigation: B 14233eed0, subtree 141ea8c60.
    //! Jump tables and exact bounded instructions are recorded in
    //! reports/business-module-recovery-2026-10-21/toolbar-input-evidence.
    use super::UiClickEvent;

    #[cfg(test)]
    mod native_fixtures {
        //! Recorded original-executable fixtures; no native business helper is replaced.
        //! Source: reports/business-module-recovery-2026-10-23-toolbar/native_nav_trees.json.
        //! EXE SHA256 cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323.
        use super::*;
        use crate::resouces_and_components::taffy_layout::AnchoredNode;
        use country_core::input::action_set::{DigitalActionPhase, DigitalActionState};
        use taffy::style::{FlexDirection, Style};
        struct Case {
            name: &'static str,
            positions: &'static [(u32, f32, f32)],
            children: &'static [(u32, &'static [u32])],
            anchors: &'static [(u32, u32, u8)],
            visibility: &'static [(u32, u8)],
            vertical: &'static [u32],
            current: u32,
            direction: usize,
            native_selected: u32,
        }
        const CASES: &[Case] = &[
            Case {
                name: "sibling_root_priority",
                positions: &[(10, 0.0, 0.0), (11, 100.0, 0.0), (12, 10.0, 0.0)],
                children: &[(1, &[10, 11, 12])],
                anchors: &[(1, 0, 4)],
                visibility: &[],
                vertical: &[],
                current: 10,
                direction: 1,
                native_selected: 11,
            },
            Case {
                name: "hidden_candidate_is_eligible",
                positions: &[(10, 0.0, 0.0), (11, 5.0, 0.0)],
                children: &[(1, &[10, 11])],
                anchors: &[(1, 0, 4)],
                visibility: &[(11, 0)],
                vertical: &[],
                current: 10,
                direction: 1,
                native_selected: 11,
            },
            Case {
                name: "subtree_nearest",
                positions: &[(10, 0.0, 0.0), (11, 100.0, 0.0), (12, 10.0, 0.0)],
                children: &[(1, &[10, 2]), (2, &[11, 12])],
                anchors: &[(1, 0, 4)],
                visibility: &[],
                vertical: &[],
                current: 10,
                direction: 1,
                native_selected: 12,
            },
            Case {
                name: "subtree_tie_keeps_first",
                positions: &[(10, 0.0, 0.0), (11, 10.0, 1.0), (12, 10.0, -1.0)],
                children: &[(1, &[10, 2]), (2, &[11, 12])],
                anchors: &[(1, 0, 4)],
                visibility: &[],
                vertical: &[],
                current: 10,
                direction: 1,
                native_selected: 11,
            },
            Case {
                name: "left_reverses_siblings",
                positions: &[(10, 0.0, 0.0), (11, 5.0, 0.0), (12, 100.0, 0.0)],
                children: &[(1, &[10, 11, 12])],
                anchors: &[(1, 0, 4)],
                visibility: &[],
                vertical: &[],
                current: 12,
                direction: 0,
                native_selected: 11,
            },
            Case {
                name: "no_forward_candidate_keeps_current",
                positions: &[(10, 0.0, 0.0), (11, -1.0, 0.0)],
                children: &[(1, &[10, 11])],
                anchors: &[(1, 0, 4)],
                visibility: &[],
                vertical: &[],
                current: 10,
                direction: 1,
                native_selected: 10,
            },
            Case {
                name: "enter_above_anchor",
                positions: &[(10, 0.0, 0.0), (11, 0.0, -10.0)],
                children: &[(1, &[10]), (2, &[11])],
                anchors: &[(1, 0, 4), (2, 10, 0)],
                visibility: &[],
                vertical: &[],
                current: 10,
                direction: 3,
                native_selected: 11,
            },
            Case {
                name: "return_from_above_anchor",
                positions: &[(10, 0.0, 0.0), (11, 0.0, -10.0)],
                children: &[(1, &[10]), (2, &[11])],
                anchors: &[(1, 0, 4), (2, 10, 0)],
                visibility: &[],
                vertical: &[],
                current: 11,
                direction: 2,
                native_selected: 10,
            },
            Case {
                name: "enter_below_anchor",
                positions: &[(10, 0.0, 0.0), (11, 0.0, 10.0)],
                children: &[(1, &[10]), (2, &[11])],
                anchors: &[(1, 0, 4), (2, 10, 1)],
                visibility: &[],
                vertical: &[],
                current: 10,
                direction: 2,
                native_selected: 11,
            },
            Case {
                name: "enter_left_anchor",
                positions: &[(10, 0.0, 0.0), (11, -10.0, 0.0)],
                children: &[(1, &[10]), (2, &[11])],
                anchors: &[(1, 0, 4), (2, 10, 2)],
                visibility: &[],
                vertical: &[],
                current: 10,
                direction: 0,
                native_selected: 11,
            },
            Case {
                name: "enter_right_anchor",
                positions: &[(10, 0.0, 0.0), (11, 10.0, 0.0)],
                children: &[(1, &[10]), (2, &[11])],
                anchors: &[(1, 0, 4), (2, 10, 3)],
                visibility: &[],
                vertical: &[],
                current: 10,
                direction: 1,
                native_selected: 11,
            },
            Case {
                name: "vertical_sibling",
                positions: &[(10, 0.0, 0.0), (11, 0.0, 10.0)],
                children: &[(1, &[10, 11])],
                anchors: &[(1, 0, 4)],
                visibility: &[],
                vertical: &[1],
                current: 10,
                direction: 2,
                native_selected: 11,
            },
            Case {
                name: "return_from_below_anchor",
                positions: &[(10, 0.0, 0.0), (11, 0.0, 10.0)],
                children: &[(1, &[10]), (2, &[11])],
                anchors: &[(1, 0, 4), (2, 10, 1)],
                visibility: &[],
                vertical: &[],
                current: 11,
                direction: 3,
                native_selected: 10,
            },
            Case {
                name: "return_from_left_anchor",
                positions: &[(10, 0.0, 0.0), (11, -10.0, 0.0)],
                children: &[(1, &[10]), (2, &[11])],
                anchors: &[(1, 0, 4), (2, 10, 2)],
                visibility: &[],
                vertical: &[],
                current: 11,
                direction: 1,
                native_selected: 10,
            },
            Case {
                name: "return_from_right_anchor",
                positions: &[(10, 0.0, 0.0), (11, 10.0, 0.0)],
                children: &[(1, &[10]), (2, &[11])],
                anchors: &[(1, 0, 4), (2, 10, 3)],
                visibility: &[],
                vertical: &[],
                current: 11,
                direction: 0,
                native_selected: 10,
            },
            Case {
                name: "wrong_anchor_return_direction",
                positions: &[(10, 0.0, 0.0), (11, 0.0, -10.0)],
                children: &[(1, &[10]), (2, &[11])],
                anchors: &[(1, 0, 4), (2, 10, 0)],
                visibility: &[],
                vertical: &[],
                current: 11,
                direction: 3,
                native_selected: 11,
            },
            Case {
                name: "sibling_precedes_closer_anchor",
                positions: &[(10, 0.0, 0.0), (11, 100.0, 0.0), (12, 1.0, 0.0)],
                children: &[(1, &[10, 11]), (2, &[12])],
                anchors: &[(1, 0, 4), (2, 10, 3)],
                visibility: &[],
                vertical: &[],
                current: 10,
                direction: 1,
                native_selected: 11,
            },
            Case {
                name: "nested_ascent_finds_outer_sibling",
                positions: &[(10, 0.0, 0.0), (11, 5.0, 0.0)],
                children: &[(1, &[2, 11]), (2, &[10])],
                anchors: &[(1, 0, 4)],
                visibility: &[],
                vertical: &[],
                current: 10,
                direction: 1,
                native_selected: 11,
            },
            Case {
                name: "vertical_parent_blocks_horizontal_sibling",
                positions: &[(10, 0.0, 0.0), (11, 5.0, 0.0)],
                children: &[(1, &[10, 11])],
                anchors: &[(1, 0, 4)],
                visibility: &[],
                vertical: &[1],
                current: 10,
                direction: 1,
                native_selected: 10,
            },
            Case {
                name: "visited_anchor_cycle_terminates",
                positions: &[(10, 0.0, 0.0)],
                children: &[(1, &[10])],
                anchors: &[(1, 10, 4), (1, 10, 0)],
                visibility: &[],
                vertical: &[],
                current: 10,
                direction: 0,
                native_selected: 10,
            },
        ];
        #[test]
        fn production_navigation_matches_recorded_nonempty_native_trees() {
            for case in CASES {
                let mut layout = TaffyLayout::default();
                let mut nodes = HashMap::new();
                nodes.insert(0, Node::default());
                let ids = case
                    .positions
                    .iter()
                    .map(|entry| entry.0)
                    .chain(case.children.iter().flat_map(|(node, children)| {
                        std::iter::once(*node).chain(children.iter().copied())
                    }))
                    .chain(
                        case.anchors
                            .iter()
                            .flat_map(|(node, target, _)| [*node, *target]),
                    );
                for id in ids {
                    nodes.entry(id).or_insert_with(|| {
                        layout
                            .taffy
                            .new_leaf(Style {
                                flex_direction: if case.vertical.contains(&id) {
                                    FlexDirection::Column
                                } else {
                                    FlexDirection::Row
                                },
                                ..Default::default()
                            })
                            .unwrap()
                    });
                }
                for &(parent, children) in case.children {
                    for child in children {
                        layout
                            .taffy
                            .add_child(nodes[&parent], nodes[child])
                            .unwrap();
                    }
                }
                for &(node, relative_to, side) in case.anchors {
                    layout.anchored.push(AnchoredNode {
                        node: nodes[&node],
                        relative_to: nodes[&relative_to],
                        side: match side {
                            0 => AnchorSide::Above,
                            1 => AnchorSide::Below,
                            2 => AnchorSide::Left,
                            3 => AnchorSide::Right,
                            4 => AnchorSide::Window,
                            _ => unreachable!(),
                        },
                        order: 0,
                        offset: Vec2::ZERO,
                        gap: 0.0,
                    });
                }
                let mut world = World::new();
                let mut entities = HashMap::new();
                for &(id, x, y) in case.positions {
                    let entity = world
                        .spawn((
                            TaffyNode {
                                node: nodes[&id],
                                ..Default::default()
                            },
                            UiNav::default(),
                            Ui2dExtent {
                                pos: Vec2::new(x, y),
                                size: Vec2::ONE,
                            },
                            UiInteractionState {
                                state: case
                                    .visibility
                                    .iter()
                                    .find(|(node, _)| *node == id)
                                    .map_or(1, |(_, state)| *state),
                                keep: 0,
                                selected: 0,
                            },
                        ))
                        .id();
                    entities.insert(id, entity);
                }
                world.insert_resource(layout);
                let mut actions = ActionSetStates::default();
                actions.digital[20 + case.direction] = DigitalActionState {
                    phase: DigitalActionPhase::JustPressed,
                    ..Default::default()
                };
                world.insert_resource(actions);
                let mut navigation = UiNavState::default();
                navigation.focus = toolbar(entities[&case.current]);
                world.insert_resource(navigation);
                world.init_resource::<Events<UiClickEvent>>();
                let mut schedule = Schedule::default();
                schedule.add_systems(ui_nav_update);
                schedule.run(&mut world);
                assert_eq!(
                    world.resource::<UiNavState>().focus,
                    toolbar(entities[&case.native_selected]),
                    "native fixture {}",
                    case.name
                );
            }
        }
    }
    use crate::resouces_and_components::{
        TaffyNode, UiNav,
        taffy_layout::{AnchorSide, TaffyLayout},
    };
    use bevy_ecs::prelude::*;
    use country_core::{
        components::{Ui2dExtent, UiInteractionState},
        input::action_set::ActionSetStates,
        systems::ui::resources::widget_params::{UiFocus, UiNavState},
    };
    use glam::Vec2;
    use std::collections::{HashMap, HashSet};
    use taffy::node::Node;

    fn clear(focus: &mut UiFocus) {
        focus.tag = 0;
        focus._pad = 0;
    }
    fn toolbar(entity: Entity) -> UiFocus {
        UiFocus {
            tag: 1,
            _pad: 0,
            payload: entity.to_bits(),
        }
    }

    /// Native recursively visits a node before its children. Visibility is an
    /// entry-selection condition, not a subtree predicate. Strict distance ties
    /// preserve the first tree-order candidate, independently for each root.
    fn nearest(
        root: Node,
        layout: &TaffyLayout,
        nodes: &HashMap<Node, Entity>,
        items: &Query<(
            Entity,
            Option<&Ui2dExtent>,
            Option<&UiInteractionState>,
            &UiNav,
        )>,
        origin: Vec2,
        direction: Vec2,
    ) -> Option<Entity> {
        let direction = direction.normalize_or_zero();
        let mut best_distance = f32::MAX;
        let mut best = None;
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let Some(&entity) = nodes.get(&node) {
                if let Ok((_, Some(extent), _, _)) = items.get(entity) {
                    let delta = extent.pos - origin;
                    let distance = delta.length();
                    if delta.normalize_or_zero().dot(direction) > 0.0 && distance < best_distance {
                        best_distance = distance;
                        best = Some(entity);
                    }
                }
            }
            stack.extend(layout.children(node).into_iter().rev());
        }
        best
    }

    pub fn ui_nav_update(
        items: Query<(
            Entity,
            Option<&Ui2dExtent>,
            Option<&UiInteractionState>,
            &UiNav,
        )>,
        widgets: Query<(Entity, &TaffyNode, &UiNav)>,
        actions: Res<ActionSetStates>,
        layout: Res<TaffyLayout>,
        mut state: ResMut<UiNavState>,
        mut clicks: EventWriter<UiClickEvent>,
    ) {
        if state.focus.is_mei() {
            clear(&mut state.staged_focus);
            return;
        }
        let raw = [20usize, 21, 22, 23].map(|index| actions.digital[index].just_pressed());
        let directions = [
            raw[0] && !raw[1],
            raw[1] && !raw[0],
            raw[2] && !raw[3],
            raw[3] && !raw[2],
        ];
        let any_direction = directions.iter().any(|&pressed| pressed);
        let confirm = actions.digital[19].just_pressed();
        if !state.focus.is_toolbar() {
            if !any_direction && !confirm {
                return;
            }
            if state.staged_focus.is_toolbar() {
                state.focus = state.staged_focus;
                clear(&mut state.staged_focus);
                return;
            }
            // Query iteration order is the original Bevy entry-selection order.
            let first = items
                .iter()
                .find(|(_, _, visibility, _)| visibility.is_none_or(|value| value.state != 0))
                .map(|(entity, _, _, _)| entity);
            clear(&mut state.staged_focus);
            if let Some(entity) = first {
                state.focus = toolbar(entity);
            } else {
                clear(&mut state.focus);
            }
            return;
        }
        if actions.digital[18].just_released() {
            state.staged_focus = state.focus;
            clear(&mut state.focus);
            return;
        }
        let current = Entity::from_bits(state.focus.payload);
        let Ok((_, extent, _, navigation)) = items.get(current) else {
            clear(&mut state.focus);
            return;
        };
        if confirm {
            clicks.write(UiClickEvent { entity: current });
            state.staged_focus = state.focus;
            clear(&mut state.focus);
            return;
        }
        let Some(slot) = directions.iter().position(|&pressed| pressed) else {
            return;
        };
        let link = navigation.links[slot];
        match link.tag {
            1 => return,
            3 => {
                // The original checks the Entity generation niche before writing.
                if link.value >> 32 != 0 {
                    state.focus = toolbar(Entity::from_bits(link.value));
                }
                return;
            }
            2 => {
                state.focus = toolbar(current);
                return;
            }
            0 => {}
            _ => panic!("invalid native UiNav variant"),
        }
        let Some(extent) = extent else {
            state.focus = toolbar(current);
            return;
        };
        let origin = extent.pos;
        let direction = Vec2::new(
            directions[1] as u8 as f32 - directions[0] as u8 as f32,
            directions[2] as u8 as f32 - directions[3] as u8 as f32,
        );

        // Rebuild the two native hash maps only for spatial fallback. Parent
        // discovery starts at anchored roots, with the original LIFO traversal.
        let nodes: HashMap<_, _> = widgets
            .iter()
            .map(|(entity, node, _)| (node.node, entity))
            .collect();
        let mut parents = HashMap::new();
        let mut stack: Vec<_> = layout.anchored.iter().map(|entry| entry.node).collect();
        while let Some(node) = stack.pop() {
            for child in layout.children(node) {
                parents.insert(child, node);
                stack.push(child);
            }
        }
        let (_, current_node, _) = widgets
            .get(current)
            .expect("native focused toolbar TaffyNode unwrap");
        let mut node = current_node.node;
        let mut visited = HashSet::new();
        while visited.insert(node) {
            let mut candidates = Vec::new();
            let next;
            if let Some(&parent) = parents.get(&node) {
                next = parent;
                // Native tests flex_direction != Row against horizontal movement;
                // preserve this even for the dependency's reverse-direction tags.
                let vertical = layout.style(parent).flex_direction != taffy::style::FlexDirection::Row;
                if vertical != (directions[0] || directions[1]) {
                    let children = layout.children(parent);
                    let index = children
                        .iter()
                        .position(|&child| child == node)
                        .expect("native navigation child missing from parent");
                    if directions[0] || directions[3] {
                        candidates.extend(children[..index].iter().rev().copied());
                    } else {
                        candidates.extend_from_slice(&children[index + 1..]);
                    }
                }
            } else {
                let Some(anchor) = layout
                    .anchored
                    .iter()
                    .find(|entry| entry.node == node && entry.side != AnchorSide::Window)
                else {
                    break;
                };
                next = anchor.relative_to;
                let returning = match anchor.side {
                    AnchorSide::Above => directions[2],
                    AnchorSide::Below => directions[3],
                    AnchorSide::Left => directions[1],
                    AnchorSide::Right => directions[0],
                    AnchorSide::Window => false,
                };
                if returning {
                    if let Some(entity) = nearest(next, &layout, &nodes, &items, origin, direction) {
                        state.focus = toolbar(entity);
                        return;
                    }
                }
                node = next;
                continue;
            }
            for anchor in &layout.anchored {
                if anchor.relative_to != node {
                    continue;
                }
                let entering = match anchor.side {
                    AnchorSide::Above => directions[3],
                    AnchorSide::Below => directions[2],
                    AnchorSide::Left => directions[0],
                    AnchorSide::Right => directions[1],
                    AnchorSide::Window => false,
                };
                if entering {
                    candidates.push(anchor.node);
                }
            }
            for root in candidates {
                if let Some(entity) = nearest(root, &layout, &nodes, &items, origin, direction) {
                    state.focus = toolbar(entity);
                    return;
                }
            }
            node = next;
        }
        state.focus = toolbar(current);
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::resouces_and_components::{UiNavLink, taffy_layout::AnchoredNode};
        use country_core::input::action_set::{DigitalActionPhase, DigitalActionState};
        use taffy::style::Style;

        fn fixture() -> (World, Schedule, Node) {
            let mut world = World::new();
            let mut layout = TaffyLayout::default();
            let root = layout.add_node(Style::default()).root();
            layout.anchored.push(AnchoredNode {
                node: root,
                relative_to: Node::default(),
                side: AnchorSide::Window,
                order: 0,
                offset: Vec2::ZERO,
                gap: 0.0,
            });
            world.insert_resource(layout);
            world.insert_resource(ActionSetStates::default());
            world.insert_resource(UiNavState::default());
            world.init_resource::<Events<UiClickEvent>>();
            let mut schedule = Schedule::default();
            schedule.add_systems(ui_nav_update);
            (world, schedule, root)
        }
        fn item(world: &mut World, parent: Node, pos: Vec2, state: u8) -> Entity {
            let mut layout = world.resource_mut::<TaffyLayout>();
            let node = layout.taffy.new_leaf(Style::default()).unwrap();
            layout.taffy.add_child(parent, node).unwrap();
            world
                .spawn((
                    UiNav::default(),
                    TaffyNode {
                        node,
                        ..Default::default()
                    },
                    Ui2dExtent {
                        pos,
                        size: Vec2::ONE,
                    },
                    UiInteractionState {
                        state,
                        keep: 0,
                        selected: 0,
                    },
                ))
                .id()
        }
        fn press(world: &mut World, slot: usize) {
            world.resource_mut::<ActionSetStates>().digital = [DigitalActionState::default(); 40];
            world.resource_mut::<ActionSetStates>().digital[slot] = DigitalActionState {
                phase: DigitalActionPhase::JustPressed,
                ..Default::default()
            };
        }
        #[test]
        fn sibling_priority_and_hidden_subtree_candidates_match_native() {
            let (mut world, mut schedule, root) = fixture();
            let current = item(&mut world, root, Vec2::ZERO, 1);
            let first = item(&mut world, root, Vec2::new(100.0, 0.0), 0);
            let _closer_later_root = item(&mut world, root, Vec2::new(10.0, 0.0), 1);
            world.resource_mut::<UiNavState>().focus = toolbar(current);
            press(&mut world, 21);
            schedule.run(&mut world);
            assert_eq!(world.resource::<UiNavState>().focus, toolbar(first));
        }
        #[test]
        fn explicit_links_confirm_and_staged_restore_keep_entity_identity() {
            let (mut world, mut schedule, root) = fixture();
            let current = item(&mut world, root, Vec2::ZERO, 1);
            let target = item(&mut world, root, Vec2::X, 1);
            world.get_mut::<UiNav>(current).unwrap().links[1] = UiNavLink {
                tag: 3,
                value: target.to_bits(),
            };
            world.resource_mut::<UiNavState>().focus = toolbar(current);
            press(&mut world, 21);
            schedule.run(&mut world);
            assert_eq!(world.resource::<UiNavState>().focus, toolbar(target));
            press(&mut world, 19);
            schedule.run(&mut world);
            assert_eq!(world.resource::<Events<UiClickEvent>>().len(), 1);
            assert_eq!(world.resource::<UiNavState>().focus.tag, 0);
            assert_eq!(world.resource::<UiNavState>().staged_focus, toolbar(target));
            press(&mut world, 19);
            schedule.run(&mut world);
            assert_eq!(world.resource::<UiNavState>().focus, toolbar(target));
            assert_eq!(world.resource::<Events<UiClickEvent>>().len(), 1);
        }
        #[test]
        fn opposite_directions_cancel_and_missing_entity_invalidates_focus() {
            let (mut world, mut schedule, root) = fixture();
            let current = item(&mut world, root, Vec2::ZERO, 1);
            world.resource_mut::<UiNavState>().staged_focus = toolbar(current);
            press(&mut world, 20);
            world.resource_mut::<ActionSetStates>().digital[21] = DigitalActionState {
                phase: DigitalActionPhase::JustPressed,
                ..Default::default()
            };
            schedule.run(&mut world);
            assert_eq!(world.resource::<UiNavState>().focus.tag, 0);
            world.resource_mut::<UiNavState>().focus = toolbar(current);
            world.despawn(current);
            press(&mut world, 21);
            schedule.run(&mut world);
            assert_eq!(world.resource::<UiNavState>().focus.tag, 0);
        }

        #[test]
        fn anchored_menu_entry_and_return_search_real_taffy_subtrees() {
            let (mut world, mut schedule, root) = fixture();
            let current = item(&mut world, root, Vec2::ZERO, 1);
            let anchor_target = world.get::<TaffyNode>(current).unwrap().node;
            let menu = world
                .resource_mut::<TaffyLayout>()
                .taffy
                .new_leaf(Style::default())
                .unwrap();
            world
                .resource_mut::<TaffyLayout>()
                .anchored
                .push(AnchoredNode {
                    node: menu,
                    relative_to: anchor_target,
                    side: AnchorSide::Above,
                    order: 1,
                    offset: Vec2::ZERO,
                    gap: 0.0,
                });
            let farther = item(&mut world, menu, Vec2::new(30.0, -10.0), 1);
            let nearer = item(&mut world, menu, Vec2::new(1.0, -10.0), 1);
            let _equal_distance_later = item(&mut world, menu, Vec2::new(-1.0, -10.0), 1);
            world.resource_mut::<UiNavState>().focus = toolbar(current);
            press(&mut world, 23);
            schedule.run(&mut world);
            assert_eq!(world.resource::<UiNavState>().focus, toolbar(nearer));
            assert_ne!(world.resource::<UiNavState>().focus, toolbar(farther));
            press(&mut world, 22);
            schedule.run(&mut world);
            assert_eq!(world.resource::<UiNavState>().focus, toolbar(current));
        }

        #[test]
        fn real_cyclic_link_producer_matches_horizontal_and_vertical_inputs() {
            for (horizontal, slot) in [(true, 21), (false, 22)] {
                let (mut world, mut schedule, root) = fixture();
                let current = item(&mut world, root, Vec2::ZERO, 1);
                let target = item(&mut world, root, Vec2::ONE, 1);
                let links = crate::resouces_and_components::link_up_nav(
                    &[current.to_bits(), target.to_bits()],
                    horizontal,
                );
                for (entity, navigation) in links {
                    world
                        .entity_mut(Entity::from_bits(entity))
                        .insert(navigation);
                }
                world.resource_mut::<UiNavState>().focus = toolbar(current);
                press(&mut world, slot);
                schedule.run(&mut world);
                assert_eq!(world.resource::<UiNavState>().focus, toolbar(target));
                schedule.run(&mut world);
                assert_eq!(world.resource::<UiNavState>().focus, toolbar(current));
            }
        }
    }
}
pub use navigation::ui_nav_update;
pub use focus::{
    ui_focus_parent_if_self_in_focus, ui_notify_hints_if_toolbar_in_focus,
    ui_send_focus_begin_events, ui_show_if_parent_in_focus,
};
pub use hide::hide_ui;
pub use ordering::sort_contextual_buttons_draw_order;

#[cfg(test)]
#[path = "ui_interaction/widget_geometry_tests.rs"]
mod widget_geometry_tests;
