//! toolbar 交互 UI（B 版恢复，restorer-wave3-toolbar-layout，2026-10-17）。
//!
//! 恢复范围（wave3 fresh 反编译 + caller SystemParam 签名）：
//! - 事件类型：`UiJustBecameInFocusEvent`/`UiClickEvent`/`UiDragEvent`
//!   （`toolbar::ui_interaction`，add_toolbar_item 的 UI 交互事件族）。
//! - `ui_animation_state_update`（0x14233b3f0）：逐工具栏项更新 `UiAnimationState`
//!   外观/选择进度（公式+字节判别判等，见下）。
//! - `ui_sounds`（0x14233a7f0，真地址）：**未实现（分歧见模块尾与 RESTORE.md）**。
//! - `render_taffy_ui_to_widgets`（0x142338990）：焦点交换 / keep 面 / 事件写面
//!   已按 fresh 判等落库；Signifier 闭包标记与焦点矩形链仍未实现（见函数注释）。
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

use country_core::systems::ui::resources::widget_params::{
    UiNavState, UiSounds, WidgetParams, WidgetVolumeShape,
};

use crate::resouces_and_components::{
    HistoryButton, TaffyNode, UiAnimationState, UiInteractionState, UiLocked, UiMouseInteractable, UiNav,
    UiPlayLockedAnimation, UiPlaySelectAnimation,
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
///      UiPlaySelectAnimation{0.2,0.2,0})`）；无 HistoryButton：非 Photomode 且
///      active → Signifier 载荷（`icon_size_mult*60`）。
///   6. 无焦点且 state==1 的首个 UiNav 按钮接管 Toolbar 焦点（**判等**）。
///   7. 所有 widget 共享尾：`clicked()`→`UiClickEvent{entity}`；
///      `into_drag_state() != 2`→`UiDragEvent{entity,state}`（**判等**）。
///
/// **未实现/分歧**：
/// - `SignifierFactory::new` 的 inner/marker（闭包胖指针）未在反编译展开，
///   载荷已判等但不伪造闭包。
/// - `WidgetParams::new` 的 volume/`extra_78` 字段身份 U；按
///   `WidgetVolumeShape{tag=3,layer=interaction.state,size=rect}` 投影（I）。
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

    for (entity, extent, _interactable, taffy, mut interaction, history, nav) in &mut query {
        // O L494-502：名字 = `format!("toolbar_button_{:?}", node)`。
        let name = format!("toolbar_button_{:?}", taffy.node);

        // O L503-513：`fVar25 = (extent.size + expand) * 0.5`；
        // `rect = {extent.pos - half, extent.pos + half}`；
        // WidgetParams volume size = `extent.size + expand`
        // （fresh 0x142338990：`+0x00` pos Vec2、`+0x08` size Vec2）。
        let rect_size = extent.size + glam::Vec2::splat(expand);

        // O L514-531：`WidgetParams::new` + 4 个 builder + `UiState::add<String>`。
        let params = WidgetParams::new(
            WidgetVolumeShape {
                tag: 3,
                layer: interaction.state,
                size: rect_size,
                ..Default::default()
            },
            [0; 2],
        )
        .no_history_recording()
        .occlude_by_world(true)
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
        } else {
            // O L534-557：无 HistoryButton 分支。
            // `*local_1e8 != 0xd`（AppMode != Photomode）&& `is_active()`。
            let not_photomode = !matches!(
                *app_mode,
                country_core::systems::mode_manager::AppMode::Photomode
            );
            if not_photomode && ctx.is_active() {
                // O L539-555：分配 0x20 B 载荷 `{1u64, 1u64, cursor(Vec2),
                // f32 = config_scale * 60.0}` → `SignifierFactory::new` →
                // `UiSignifierStream::write`。
                // **U**：`inner`/`marker`（闭包胖指针）未在反编译中展开；这里以
                // null/0 构造 40B 记录并真实写入流（无闭包记录走 5-qword 原始
                // 视图），不再计算后丢弃。
                let cursor_bits = cursor.0;
                let mut payload = [0u8; 0x20];
                payload[0..8].copy_from_slice(&1u64.to_le_bytes());
                payload[8..16].copy_from_slice(&1u64.to_le_bytes());
                payload[16..20].copy_from_slice(&cursor_bits.x.to_le_bytes());
                payload[20..24].copy_from_slice(&cursor_bits.y.to_le_bytes());
                payload[24..28].copy_from_slice(&(config_scale * 60.0).to_le_bytes());
                let factory = country_core::systems::ui::signifiers::SignifierFactory::new(
                    &payload,
                    std::ptr::null(),
                    0,
                );
                signifiers.write_factory(&factory);
            }
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
mod ordering;
pub use ordering::sort_contextual_buttons_draw_order;
pub use focus::{ui_focus_parent_if_self_in_focus, ui_show_if_parent_in_focus,
    ui_send_focus_begin_events, ui_notify_hints_if_toolbar_in_focus};
