//! toolbar 系统：`toolbar_alignment`（O 0x142344560）与 `toolbar_auto_hide`
//! （I 0x142343a10）。
//!
//! 第 11 轮 TB-D（2026-10-19）fresh 证据：
//! - `toolbar_alignment` 0x142344560：PDB body 201 B，prepare + 反编译 +
//!  冷块反汇编（`evidence/round11/tbd-jobs/decomp/142344560.c`、
//!  `dis-align-cold-result.json`、`dis-align-gaps-result.json`）。
//!  PDB 精确签名：
//!  `fn(Res<GameConfig>, Res<ToolbarLayout>, Res<PhotomodeToolbarLayout>,
//!  ResMut<TaffyLayout>)`。
//!  1. 先遍历 `TaffyLayout.anchored`（stride 0x28）：`side(+0x24) != 4`
//!     时按 `0x1020300 >> ((alignment & 3) << 3)` 的低字节重写 side；
//!     `alignment = GameConfig.ui.toolbar_alignment`（native +0xF0）。
//!  2. 对 **两个布局各一次**（ToolbarLayout、PhotomodeToolbarLayout）：
//!     case 0..3 的 4 个 setter 序列作用在 `node_a(+0x18)` / `node_b(+0x20)`：
//!     - 0：A=env12(justify center), A=env11(align flex-end),
//!          B=env10(flex row), A=env11；
//!     - 1：A=env0(justify flex-start), A=undo_env1(align center),
//!          B=env2(flex column), A=env13(align flex-start)；
//!     - 2：A=undo_env0(justify flex-end), A=undo_env1, B=env2, A=env11；
//!     - 3：A=env12, A=env13, B=env10, A=env13。
//!     第四调用的目标与参数由冷块 0x142344687/0x1423446df/0x14234473f/
//!     0x14234479f 的 `MOV RCX,RSI; MOV EDX,EDI; MOV R8D,R12D; CALL ...`
//!     逐条确认（0x142375ce0=env11、0x142376300=env13）。
//!  3. 每个 contextual menu 记录（stride 0x20，`node(+0x10) != 0`）：
//!     align 0/1 → env2(column)，align≥2 → env10(row)；随后偶数 align
//!     (0/2) → env11、奇数 (1/3) → env13（次跳表 0x14304e6e8）。
//! - `toolbar_auto_hide` 0x142343a10：PDB body 2883 B（hot 1675 B 已反编译 +
//!  冷块反汇编，`decomp/142343a10.c`、`list-auto-hide-result.json`）。
//!  PDB 精确签名（14 参数）：
//!  `fn(Res<AppMode>, Res<CursorPositionSS>, Res<EscMenuState>, Res<GameConfig>,
//!  Res<Time>, Query<(&ToolbarButtonCooldown, &UiInteractionState,
//!  Option<&ShowOnlyInPhotomode>)>, Res<WindowSize>, Res<ActionSetStates>,
//!  ResMut<ToolbarLayout>, ResMut<PhotomodeToolbarLayout>, ResMut<TaffyLayout>,
//!  Query<(&mut Ui2dExtent, &DebugToolbarActivationZone)>,
//!  EventWriter<PlayUiSound>, Res<ToolbarSizeMult>)`。
//!  已重建：布局选择（`AppMode.native_tag() == 0x13` = Photomode）、
//!  `auto_hide_toolbar == false → progress = 0`、
//!  `scale = icon_size_mult * ToolbarSizeMult`、
//!  `thin = scale·110·0.3`、`total = ((count-1)·10 + count·110)·scale`、
//!  progress `±dt/0.3` + `clamp01` + PlayUiSound 0x10/0x11 边沿、
//!  `CubicInOut::apply(progress)` 与 `offset = scale·200·ease`、anchored offset
//!  写面（alignment 0:(0,+o)、1:(-o,0)、2:(+o,0)、3:(0,-o)）。
//!  2026-10-20 原版 RNE 指令差分补闭：四向中心、缩放后双轴边距、真实 debug
//!  extent 写入，以及第一数字动作 click。0x10/0x11 是无载荷声音变体，native
//!  记录 +0x18 是事件队列自增 EventId，不是声音业务载荷；安全表示的未使用载荷清零。
//!  证据：progress-20261020/toolbar-layout。
//!
//! `undo_redo_alignment`/`undo_redo_auto_hide` 的 PDB 签名引用
//! `toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout`，
//! 该模块当前 crate 不存在且不在本组文件权内 → 本轮不落，登记为跨文件阻塞。

use bevy_ecs::prelude::*;
mod input {
    //! Toolbar background input volumes, recovered from the original implementation.
    //! Original B toolbar_steal_mouse_input142344df0/2706B, body SHA256
    //! 9b00afb82fd97223222be7f4c04718ec342a75df07675620972b92f8882e115c.
    //! Six original SystemParams; this producer never mutates ActionSetStates.
    use crate::{
        resouces_and_components::{
            taffy_layout::TaffyLayout,
            toolbar_layout::{PhotomodeToolbarLayout, ToolbarContext, ToolbarLayout},
        },
        toolbar_update::ToolbarSizeMult,
    };
    use bevy_ecs::prelude::*;
    use country_core::{
        resources::gui::DebugGui,
        systems::ui::resources::{
            ui_state::UiState,
            widget_params::{UiSounds, WidgetParams},
        },
    };
    use glam::Vec2;

    /// Original set8: after toolbar_size_mult_update, before the button tuple.
    /// Registrations feed the following frame's set6 picker. This is an input
    /// volume for toolbar backgrounds, not an Escape-menu input-reset system.
    pub fn toolbar_steal_mouse_input(
        toolbar: Res<ToolbarLayout>,
        photomode: Res<PhotomodeToolbarLayout>,
        layout: Res<TaffyLayout>,
        ui: Res<UiState>,
        debug: Res<DebugGui>,
        multiplier: Res<ToolbarSizeMult>,
    ) {
        // Original142344ecf reads DebugGui+f7, established Show 2D UI.
        if !debug.show_2d_ui {
            return;
        }
        for context in [&*toolbar as &dyn ToolbarContext, &*photomode] {
            // Original chained iterator first yields node_b@20 unconditionally,
            // then each non-null menu node@10 in the stride20 definition Vec.
            for node in std::iter::once(context.node_b()).chain(
                context
                    .contextual_menus()
                    .iter()
                    .filter(|menu| menu.node != taffy::node::Node::default())
                    .map(|menu| menu.node),
            ) {
                // Native Taffy::layout Result unwrap is deliberate: a malformed
                // registered node is an invariant failure, not silently omitted.
                let actual = layout.layout(node);
                let size = Vec2::new(actual.size.width, actual.size.height);
                let location = Vec2::new(actual.location.x, actual.location.y);
                // 14234526e..2b1: keep multiplication/addition and center rounding.
                // This expands from location toward max, not symmetrically about
                // the old rectangle. Do not simplify min to location (f32 changes).
                let padding = multiplier.multiplier * 10.0;
                let doubled = padding + padding;
                let half = (size + Vec2::splat(doubled)) * 0.5;
                let center = location + half;
                let min = center - half;
                let max = center + half;
                let name = format!("steal_input_{node:?}");
                let params = WidgetParams::default()
                    .screen_aabb(min, max, f32::from_bits(0x3ca3d70a))
                    .no_history_recording()
                    .sounds(UiSounds::empty());
                // Native makes no occlude_by_world / allow_negative_distance call:
                // preserve true / false defaults, unlike the button producers.
                ui.add(&name, params);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::resouces_and_components::toolbar_layout::ContextualMenu;
        use country_core::systems::ui::resources::{ui_state::WidgetId, widget_params::WidgetGeometry};
        use taffy::{
            geometry::Size,
            style::{AvailableSpace, Dimension, Style},
        };

        fn fixture() -> (World, Schedule, Vec<taffy::node::Node>) {
            let mut layout = TaffyLayout::default();
            let mut nodes = Vec::new();
            for _ in 0..3 {
                let node = layout
                    .add_node(Style {
                        size: Size {
                            width: Dimension::Points(40.),
                            height: Dimension::Points(20.),
                        },
                        ..Default::default()
                    })
                    .root();
                layout
                    .taffy
                    .compute_layout(
                        node,
                        Size {
                            width: AvailableSpace::Definite(100.),
                            height: AvailableSpace::Definite(100.),
                        },
                    )
                    .unwrap();
                nodes.push(node);
            }
            let mut world = World::new();
            world.insert_resource(ToolbarLayout {
                node_b: nodes[0],
                menus: vec![
                    ContextualMenu::default(),
                    ContextualMenu {
                        node: nodes[2],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            });
            world.insert_resource(PhotomodeToolbarLayout {
                node_b: nodes[1],
                ..Default::default()
            });
            world.insert_resource(layout);
            world.insert_resource(UiState::at_startup());
            world.insert_resource(DebugGui::default());
            world.insert_resource(ToolbarSizeMult { multiplier: 2.0 });
            let mut schedule = Schedule::default();
            schedule.add_systems(toolbar_steal_mouse_input);
            (world, schedule, nodes)
        }

        #[test]
        fn normal_photo_and_existing_contextual_roots_register_real_layout_backgrounds() {
            let (mut world, mut schedule, nodes) = fixture();
            // There deliberately is no ActionSetStates, GameConfig or AppMode:
            // these are not original dependencies and both layouts participate.
            schedule.run(&mut world);
            let ui = world.resource::<UiState>();
            assert_eq!(ui.registry_snapshot().len(), 3);
            for node in nodes {
                let params = ui
                    .widget(WidgetId::from_name(&format!("steal_input_{node:?}")))
                    .unwrap();
                match params.resolved_geometry().as_ref() {
                    WidgetGeometry::ScreenAabb { min, max, distance } => {
                        assert_eq!(*min, Vec2::ZERO);
                        assert_eq!(*max, Vec2::new(80., 60.));
                        assert_eq!(distance.to_bits(), 0x3ca3d70a);
                    }
                    other => panic!("unexpected geometry {other:?}"),
                }
                assert!(params.occlude_by_world);
                assert!(params.flags.no_history_recording());
                assert!(!params.flags.allow_negative_distance());
                assert_eq!(params.sounds, UiSounds::empty());
            }
        }

        #[test]
        fn show_2d_ui_false_gates_before_even_invalid_layout_access() {
            let (mut world, mut schedule, _) = fixture();
            world.resource_mut::<DebugGui>().show_2d_ui = false;
            world.resource_mut::<ToolbarLayout>().node_b = Default::default();
            schedule.run(&mut world);
            assert!(world.resource::<UiState>().registry_snapshot().is_empty());
        }

        #[test]
        #[should_panic]
        fn invalid_unconditional_toolbar_node_retains_original_unwrap_failure() {
            let (mut world, mut schedule, _) = fixture();
            world.resource_mut::<ToolbarLayout>().node_b = Default::default();
            schedule.run(&mut world);
        }
    }
}
pub use input::toolbar_steal_mouse_input;
use bevy_time::Time;
use country_core::{
    game_config::GameConfig,
    input::action_set::ActionSetStates,
    resources::{
        gui::EscMenuState,
        mouse_terrain_raycast::CursorPositionSS,
        window::WindowSize,
    },
    systems::{
        mode_manager::AppMode,
        ui::resources::widget_params::PlayUiSound,
    },
};
use glam::Vec2;
use utils::{easing_functions::EasingFunction, geometry::aabb::Aabb2};

use crate::{
    resouces_and_components::{
        DebugToolbarActivationZone, ShowOnlyInPhotomode, ToolbarButtonCooldown, UiInteractionState,
        taffy_layout::TaffyLayout,
        toolbar_layout::{PhotomodeToolbarLayout, ToolbarContext, ToolbarLayout},
    },
    toolbar_update::ToolbarSizeMult,
};

/// `taffy_mut_style<T>` 通用模式（O 0x142375b40 等十单态 fresh 反编译逐语句）：
/// `let mut s = taffy.style(node).unwrap(); f(&mut s); taffy.set_style(node, s);`。
pub fn taffy_mut_style<T: FnOnce(&mut taffy::style::Style)>(
    taffy: &mut taffy::node::Taffy,
    node: taffy::node::Node,
    f: T,
) {
    let mut style = taffy.style(node).unwrap().clone();
    f(&mut style);
    taffy.set_style(node, style).unwrap();
}

/// `toolbar_alignment` 使用、且 B 中确有 `taffy_mut_style` 单态的 8 个 closure。
///
/// 偏移证据（workspace taffy 0.3.13 实测 `Style` 344B）：`flex_direction@0x14b`、
/// `align_items@0x14d`、`justify_content@0x152`。
pub mod alignment_closures {
    use taffy::style::{AlignContent, AlignItems, FlexDirection, Style};

    /// `closure_env$12`（0x142376050）：`[+0x1c2]=4` → justify_content Center。
    pub fn toolbar_env_12(style: &mut Style) {
        style.justify_content = Some(AlignContent::Center);
    }
    /// `closure_env$11`（0x142375ce0）：`[+0x1bd]=3` → align_items FlexEnd。
    pub fn toolbar_env_11(style: &mut Style) {
        style.align_items = Some(AlignItems::FlexEnd);
    }
    /// `closure_env$10`（0x142375b40）：`[+0x1bb]=0` → flex_direction Row。
    pub fn toolbar_env_10(style: &mut Style) {
        style.flex_direction = FlexDirection::Row;
    }
    /// `closure_env$0`（0x142375c10）：`[+0x1c2]=2` → justify_content FlexStart。
    pub fn toolbar_env_0(style: &mut Style) {
        style.justify_content = Some(AlignContent::FlexStart);
    }
    /// `closure_env$2`（0x142375eb0）：`[+0x1bb]=1` → flex_direction Column。
    pub fn toolbar_env_2(style: &mut Style) {
        style.flex_direction = FlexDirection::Column;
    }
    /// `closure_env$13`（0x142376300）：`[+0x1bd]=2` → align_items FlexStart。
    pub fn toolbar_env_13(style: &mut Style) {
        style.align_items = Some(AlignItems::FlexStart);
    }
    /// `undo_redo_alignment::closure_env$0`（0x142376230）：justify FlexEnd。
    pub fn undo_redo_env_0(style: &mut Style) {
        style.justify_content = Some(AlignContent::FlexEnd);
    }
    /// `undo_redo_alignment::closure_env$1`（0x142375f80）：align_items Center。
    pub fn undo_redo_env_1(style: &mut Style) {
        style.align_items = Some(AlignItems::Center);
    }
}

/// `0x1020300 >> ((alignment & 3) << 3)` 的低字节（O：0x142344600 前置清位）。
pub fn alignment_side_mask(alignment: u8) -> u8 {
    (0x0102_0300u32 >> ((alignment & 3) << 3)) as u8
}

fn apply_node(layout: &mut TaffyLayout, node: taffy::node::Node, f: fn(&mut taffy::style::Style)) {
    if node == taffy::node::Node::default() {
        return;
    }
    layout.style_mut(node, f);
}

/// O 0x142344560 `toolbar::toolbar_systems::toolbar_alignment`。
pub fn toolbar_alignment(
    config: Res<GameConfig>,
    toolbar: Res<ToolbarLayout>,
    photomode: Res<PhotomodeToolbarLayout>,
    mut layout: ResMut<TaffyLayout>,
) {
    use alignment_closures::*;

    let alignment = config.ui.toolbar_alignment as u8;
    let mask = alignment_side_mask(alignment);

    // 前置：清非 Window anchored 的 side 字节（O，0x142344600..0x14234462a）。
    for entry in layout.anchored.iter_mut() {
        if entry.side != crate::resouces_and_components::taffy_layout::AnchorSide::Window {
            entry.side = match mask {
                0 => crate::resouces_and_components::taffy_layout::AnchorSide::Above,
                1 => crate::resouces_and_components::taffy_layout::AnchorSide::Below,
                2 => crate::resouces_and_components::taffy_layout::AnchorSide::Left,
                _ => crate::resouces_and_components::taffy_layout::AnchorSide::Right,
            };
        }
    }

    // Native executes the switch once per layout resource (two-iteration loop).
    for context in [
        &*toolbar as &dyn ToolbarContext,
        &*photomode as &dyn ToolbarContext,
    ] {
        let a = context.node_a();
        let b = context.node_b();
        match alignment {
            0 => {
                apply_node(&mut layout, a, toolbar_env_12);
                apply_node(&mut layout, a, toolbar_env_11);
                apply_node(&mut layout, b, toolbar_env_10);
                apply_node(&mut layout, a, toolbar_env_11);
            }
            1 => {
                apply_node(&mut layout, a, toolbar_env_0);
                apply_node(&mut layout, a, undo_redo_env_1);
                apply_node(&mut layout, b, toolbar_env_2);
                apply_node(&mut layout, a, toolbar_env_13);
            }
            2 => {
                apply_node(&mut layout, a, undo_redo_env_0);
                apply_node(&mut layout, a, undo_redo_env_1);
                apply_node(&mut layout, b, toolbar_env_2);
                apply_node(&mut layout, a, toolbar_env_11);
            }
            3 => {
                apply_node(&mut layout, a, toolbar_env_12);
                apply_node(&mut layout, a, toolbar_env_13);
                apply_node(&mut layout, b, toolbar_env_10);
                apply_node(&mut layout, a, toolbar_env_13);
            }
            // 4..19 的 setter 目标落在共享冷块（跨函数尾部合并），未取证 → U。
            _ => {}
        }
        // 次跳表 0x14304e6e8：偶 align → env11；奇 align → env13。
        for menu in context.contextual_menus() {
            if menu.node == taffy::node::Node::default() {
                continue;
            }
            if alignment < 2 {
                apply_node(&mut layout, menu.node, toolbar_env_2);
            } else {
                apply_node(&mut layout, menu.node, toolbar_env_10);
            }
            if alignment & 1 == 0 {
                apply_node(&mut layout, menu.node, toolbar_env_11);
            } else {
                apply_node(&mut layout, menu.node, toolbar_env_13);
            }
        }
    }
}

/// Original activation-zone center and unpadded dimensions (0x142343f59..0x142343fd8).
/// Alignment tags are bottom, left, right, top. Both vertical branches use
/// the thickness for their distance from the edge, not the toolbar length.
pub fn activation_rect(alignment: u8, window: Vec2, thin: f32, total: f32) -> (Vec2, Vec2) {
    match alignment {
        0 => (
            Vec2::new(window.x * 0.5, window.y + thin * -0.5),
            Vec2::new(total, thin),
        ),
        1 => (
            Vec2::new(thin * 0.5, window.y * 0.5),
            Vec2::new(thin, total),
        ),
        2 => (
            Vec2::new(window.x + thin * -0.5, window.y * 0.5),
            Vec2::new(thin, total),
        ),
        3 => (
            Vec2::new(window.x * 0.5, thin * 0.5),
            Vec2::new(total, thin),
        ),
        _ => unreachable!("invalid toolbar alignment"),
    }
}

/// Full original geometry (0x142343f2f..0x142343ff9), shared by the real
/// cursor hit test and DebugToolbarActivationZone drawable producer.
/// Preserve the original f32 evaluation order, including count == 0.
pub fn toolbar_activation_geometry(
    alignment: u8,
    window: Vec2,
    scale: f32,
    count: i32,
) -> (Vec2, Vec2) {
    let count = count as f32;
    let button = scale * 110.0;
    let total = (count + -1.0) * 10.0 * scale + button * count;
    let thin = button * 0.3;
    let (center, dimensions) = activation_rect(alignment, window, thin, total);
    let margin = if alignment == 1 || alignment == 2 {
        Vec2::new(scale * 80.0, scale * 40.0)
    } else {
        Vec2::new(scale * 40.0, scale * 80.0)
    };
    (center, dimensions + (margin + margin))
}

/// `toolbar_auto_hide` 的可见进度更新（从 0x142343a10 的 hot block 逐句译出）。
///
/// `keep_bit` 是 `Query<(&ToolbarButtonCooldown, &UiInteractionState,
/// Option<&ShowOnlyInPhotomode>)>` 中与当前 AppMode 匹配的项的
/// `UiInteractionState.keep` 按位或结果（native `OR`）。
pub fn auto_hide_progress(
    old: f32,
    dt: f32,
    inside_or_keep: bool,
    menu_closed: bool,
    action_pressed: bool,
) -> f32 {
    let next = if !inside_or_keep || !menu_closed {
        // Cursor outside the activation zone, or a menu is open → hide.
        old + dt / 0.3
    } else if !action_pressed {
        // Cursor inside, menu closed and click not held → show.
        old + dt / (-0.3)
    } else {
        old
    };
    next.clamp(0.0, 1.0)
}

/// Native PlayUiSound edge emission: 0x11 when 1 → <1, 0x10 when 0 → >0.
pub fn auto_hide_sound_kind(old: f32, new: f32) -> Option<u32> {
    if old == 0.0 && new > 0.0 {
        Some(0x10)
    } else if old == 1.0 && new < 1.0 {
        Some(0x11)
    } else {
        None
    }
}

/// O 0x142343a10 `toolbar::toolbar_systems::toolbar_auto_hide`.
///
/// Geometry is independently checked against the original SSE/jump-table range;
/// the first digital action is click. Sound variants 0x10/0x11 have no payload.
pub fn toolbar_auto_hide(
    app_mode: Res<AppMode>,
    cursor: Res<CursorPositionSS>,
    esc_menu: Res<EscMenuState>,
    config: Res<GameConfig>,
    time: Res<Time>,
    buttons: Query<(
        &ToolbarButtonCooldown,
        &UiInteractionState,
        Option<&ShowOnlyInPhotomode>,
    )>,
    window: Res<WindowSize>,
    actions: Res<ActionSetStates>,
    mut toolbar: ResMut<ToolbarLayout>,
    mut photomode: ResMut<PhotomodeToolbarLayout>,
    mut layout: ResMut<TaffyLayout>,
    mut zones: Query<(&mut country_core::components::Ui2dExtent, &DebugToolbarActivationZone)>,
    mut sounds: EventWriter<PlayUiSound>,
    size_mult: Res<ToolbarSizeMult>,
) {
    let is_photomode = app_mode.native_tag() == AppMode::Photomode.native_tag();
    let context: &mut dyn ToolbarContext = if is_photomode {
        &mut *photomode
    } else {
        &mut *toolbar
    };
    let scale = config.ui.icon_size_mult * size_mult.multiplier;

    if !config.ui.auto_hide_toolbar {
        context.set_hide_progress(0.0);
    } else {
        // Native counts only buttons belonging to the active mode
        // (Option<ShowOnlyInPhotomode> presence == is_photomode) and ORs
        // UiInteractionState.keep (+1 of the 3-byte component).
        let mut keep = 0u8;
        let mut count = 0i32;
        for (_cooldown, state, photomode_only) in &buttons {
            if photomode_only.is_some() == is_photomode {
                keep |= state.keep;
                count = count.wrapping_add(1);
            }
        }
        let (center, dims) = toolbar_activation_geometry(
            config.ui.toolbar_alignment as u8,
            window.as_vec2(),
            scale,
            count,
        );
        // Original 0x142344103..0x14234419b writes every matching debug extent
        // in either Bevy table or sparse-set storage before the hit test.
        for (mut extent, _) in &mut zones {
            extent.pos = center;
            extent.size = dims;
        }
        let inside_or_keep =
            (keep & 1) != 0 || Aabb2::from_center_dims(center, dims).contains(cursor.0);

        let old = context.hide_progress();
        let next = auto_hide_progress(
            old,
            time.delta_secs(),
            inside_or_keep,
            esc_menu.is_closed(),
            actions.digital[0].pressed(),
        );
        context.set_hide_progress(next);
        if let Some(kind) = auto_hide_sound_kind(old, next) {
            sounds.write(PlayUiSound {
                kind,
                _payload: [0; 20],
            });
        }
    }

    // CubicInOut easing (`0x14304e668` discriminant 1) and the anchored
    // offset write (hot block 0x142344410..0x1423444ca).
    let ease = EasingFunction::CubicInOut.apply(context.hide_progress());
    let offset = scale * 200.0 * ease;
    let (dx, dy) = match config.ui.toolbar_alignment as u8 {
        0 => (0.0, offset),
        1 => (-offset, 0.0),
        2 => (offset, 0.0),
        3 => (0.0, -offset),
        _ => (0.0, 0.0),
    };
    if let Some(entry) = layout.anchored.iter_mut().find(|entry| entry.node == context.node_a()) {
        entry.offset = Vec2::new(dx, dy);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use taffy::style::{AlignContent, AlignItems, FlexDirection};

    #[test]
    fn alignment_closures_write_expected_style_fields() {
        use alignment_closures::*;
        let mut s = taffy::style::Style::default();
        toolbar_env_0(&mut s);
        assert_eq!(s.justify_content, Some(AlignContent::FlexStart));
        let mut s = taffy::style::Style::default();
        toolbar_env_2(&mut s);
        assert_eq!(s.flex_direction, FlexDirection::Column);
        let mut s = taffy::style::Style::default();
        toolbar_env_10(&mut s);
        assert_eq!(s.flex_direction, FlexDirection::Row);
        let mut s = taffy::style::Style::default();
        toolbar_env_11(&mut s);
        assert_eq!(s.align_items, Some(AlignItems::FlexEnd));
        let mut s = taffy::style::Style::default();
        toolbar_env_12(&mut s);
        assert_eq!(s.justify_content, Some(AlignContent::Center));
        let mut s = taffy::style::Style::default();
        toolbar_env_13(&mut s);
        assert_eq!(s.align_items, Some(AlignItems::FlexStart));
        let mut s = taffy::style::Style::default();
        undo_redo_env_0(&mut s);
        assert_eq!(s.justify_content, Some(AlignContent::FlexEnd));
        let mut s = taffy::style::Style::default();
        undo_redo_env_1(&mut s);
        assert_eq!(s.align_items, Some(AlignItems::Center));
    }

    #[test]
    fn side_mask_matches_native_rotation() {
        assert_eq!(alignment_side_mask(0), 0x00);
        assert_eq!(alignment_side_mask(1), 0x03);
        assert_eq!(alignment_side_mask(2), 0x02);
        assert_eq!(alignment_side_mask(3), 0x01);
        // `& 3` is explicit in the original before the shift.
        assert_eq!(alignment_side_mask(4), 0x00);
        assert_eq!(alignment_side_mask(7), 0x01);
    }

    #[test]
    fn progress_integration_and_sound_edges_match_native() {
        // Cursor outside -> +dt/0.3
        assert!((auto_hide_progress(0.0, 0.15, false, true, true) - 0.5).abs() < 1e-6);
        // Inside + menu closed + hotkey not pressed -> -dt/0.3
        assert!((auto_hide_progress(0.9, 0.15, true, true, false) - 0.4).abs() < 1e-6);
        // Inside + menu open -> +dt/0.3
        assert!((auto_hide_progress(0.0, 0.15, true, false, false) - 0.5).abs() < 1e-6);
        // clamp01
        assert_eq!(auto_hide_progress(0.95, 1.0, false, true, true), 1.0);
        assert_eq!(auto_hide_progress(0.05, 1.0, true, true, false), 0.0);
        // Edges: 1 -> <1 emits 0x11; 0 -> >0 emits 0x10.
        assert_eq!(auto_hide_sound_kind(1.0, 0.9), Some(0x11));
        assert_eq!(auto_hide_sound_kind(0.0, 0.1), Some(0x10));
        assert_eq!(auto_hide_sound_kind(0.5, 0.6), None);
        assert_eq!(auto_hide_sound_kind(0.5, 0.0), None);
        assert_eq!(auto_hide_sound_kind(0.0, 0.0), None);
    }

    #[test]
    fn activation_rect_places_all_four_alignments() {
        let window = Vec2::new(1000.0, 600.0);
        let (c0, d0) = activation_rect(0, window, 10.0, 200.0);
        assert_eq!(c0, Vec2::new(500.0, 595.0));
        assert_eq!(d0, Vec2::new(200.0, 10.0));
        let (c1, d1) = activation_rect(1, window, 10.0, 200.0);
        assert_eq!(c1, Vec2::new(5.0, 300.0));
        assert_eq!(d1, Vec2::new(10.0, 200.0));
        let (c2, _) = activation_rect(2, window, 10.0, 200.0);
        assert_eq!(c2, Vec2::new(995.0, 300.0));
        let (c3, d3) = activation_rect(3, window, 10.0, 200.0);
        assert_eq!(c3, Vec2::new(500.0, 5.0));
        assert_eq!(d3, Vec2::new(200.0, 10.0));
    }

    #[test]
    fn alignment_case_zero_applies_sequence_to_node_a_and_b() {
        use crate::resouces_and_components::taffy_layout::TaffyLayout;
        let mut layout = TaffyLayout::default();
        let a = layout
            .taffy
            .new_leaf(taffy::style::Style::default())
            .unwrap();
        let b = layout
            .taffy
            .new_leaf(taffy::style::Style::default())
            .unwrap();
        // Case 0 sequence (native 0x142344648 block).
        apply_node(&mut layout, a, alignment_closures::toolbar_env_12);
        apply_node(&mut layout, a, alignment_closures::toolbar_env_11);
        apply_node(&mut layout, b, alignment_closures::toolbar_env_10);
        apply_node(&mut layout, a, alignment_closures::toolbar_env_11);
        assert_eq!(
            layout.taffy.style(a).unwrap().justify_content,
            Some(AlignContent::Center)
        );
        assert_eq!(
            layout.taffy.style(a).unwrap().align_items,
            Some(AlignItems::FlexEnd)
        );
        assert_eq!(
            layout.taffy.style(b).unwrap().flex_direction,
            FlexDirection::Row
        );
    }
}
