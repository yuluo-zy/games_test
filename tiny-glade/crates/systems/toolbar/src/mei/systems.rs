//! Original B MEI frame systems: mei_pre_update1406842e0 (set6) and
//! mei_post_update1406884c0 (set44, before flush and after text fragments).
//!
//! Pre processes prior-frame input then swaps/clears the current node table.
//! Authoritative digital19..23 fields also produce confirm/navigation state.
//! Post prepares actual content, lays out frame-event roots/children, updates
//! navigation, draws real text/shapes and the controller focus rectangle, then
//! clears one-frame input/state. Contracts and raw RNE instructions are retained
//! under task-index/work/TG-70a753eeccd8998d55/meihost-20261018.

use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::prelude::{Res, ResMut};
use bevy_input::mouse::MouseWheel;
use bevy_time::prelude::Time;
use glam::Vec2;

use country_core::asset_libraries::texture_library::AssetTextureLibrary;
use country_core::game_config::GameConfig;
use country_core::input::ActiveInputModeTracker;
use country_core::input::action_set::{
    ActionSetAction, ActionSetStates, DigitalActionState, MainActionSet as CoreMainActionSet,
};
use country_core::render::device::GraphicsDeviceResource;
use country_core::resources::mouse_terrain_raycast::CursorPositionSS;
use country_core::resources::steam::SteamInput;
use country_core::resources::window::WindowSize;
use country_core::systems::audio::PlayMenuUiSound;
use country_core::systems::steam::SteamClientResource;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::resources::widget_params::{
    UiFocus, UiNavState, UiSounds, WidgetParams, WidgetVolumeShape,
};
use tg_text::renderer::TextRenderer;

use crate::mei::core::{
    GlobalId, MeiContext, MeiInputEvent, NodeDrawContext, NodeFlags, NodePrepareContext,
    NodeStateBits,
};

/// O 0x1406842e0 `toolbar::mei::core::mei_pre_update`。
///
/// 逐段（fresh 反编译行号见 `evidence/0x1406842e0-mei_pre_update.c`）：
/// 1. `mei+0x270 = time.elapsed_secs_wrapped`（L161）。
/// 2. 清空输入 handler 表（L163-175）。
/// 3. 节点子 id 快照集合（L176-249；原集合消费语义 U——本实现仅取快照）。
/// 4. `window_height / 1000.0` 缩放（L259/L402）；光标读写（L254-258）；
///    `EventReader<MouseWheel>` 累计 `fVar46/fVar47`（L259-314）。
/// 5. `MainActionSet::deconstruct` → 逐 12B 记录
///    `process_input_action_handlers`，未处理者压实保留（L400-445）。
/// 6. `Vec::retain<ActionSetAction, closure_env$2>`（L446-451）：命中
///    `just_pressed(0x13..0x17)` 时仅当 `UiNavState.hold` 保留；随后按
///    `hold = elapsed < 120 s` 刷新（逐指令见实现注释）。
/// 7. `MainActionSet::reconstruct` 后 `memcpy(mei+0x30, set, 0x170)`
///    → `mei.actions`（L472-475）。
/// 8. 每节点调用 `content.pre_update(cursor_units, &mut state_bits)`
///    （L476-539；vtable +0x58）。
/// 9. 每节点命中测试 → hover 位（L540-891）：`interactive = !ignore_input &&
///    (force_interactive || content.interactive())`；矩形 =
///    `(pos..pos+size) * scale`；hover = 光标在内 **或**（focus.tag==Mei &&
///    focus.id==node && hold）。
/// 10. `interactive && hover` → `UiState` **unwrap/panic** + `WidgetParams`
///     `{new, no_history_recording, occlude_by_world(true),
///     allow_negative_distance(true), sounds(empty)}` → `UiState::add<String>`；
///     drag 状态位（bit1/2/3）= `slider3d::into_drag_state()` 的三个谓词；
///     `clicked` → 状态 bit4（焦点二项待 `MeiContext+0xc8` 槽）；焦点位 +
///     输入事件 → `content.process_input_action`；在内且滚轮非零 →
///     派发 `MeiInputEvent::scroll`。
/// 11. 悬停/点击上升沿 → `EventWriter<PlayMenuUiSound>`（声音 id != 0x0d 时，
///     L855-890）。
/// 12. Clear frame events, swap node tables and clear the new current table.
#[allow(clippy::too_many_arguments)]
pub fn mei_pre_update(
    mut mei: ResMut<MeiContext>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut nav_state: ResMut<UiNavState>,
    window_size: Res<WindowSize>,
    time: Res<Time>,
    actions: Res<ActionSetStates>,
    ui_state: Option<ResMut<UiState>>,
    cursor: Option<Res<CursorPositionSS>>,
    mut sounds: EventWriter<PlayMenuUiSound>,
) {
    // O L259/L402：`scale = window_height / 1000.0`（cvtsi2ss + divss）。
    let scale = window_size.height as f32 / 1000.0;

    // B14068459f: Time+0x58 is delta_secs, not elapsed_secs_wrapped.
    // Keep the legacy source field name until callers migrate together.
    mei.elapsed_wrapped = time.delta_secs();

    // O L163-175：handler 表 hashbrown clear + 容量字段重写。
    mei.frame_begin();

    // O L176-249：帧首清空父子链表，随后遍历节点表把每个 `children` 条目
    // `insert(child, node_id)`（0x14068463b..0x140684709）。fresh 2026-10-18
    // 判等：表为 `HashMap<GlobalId, GlobalId>`（stride 0x10），非动作位集。
    mei.build_parent_links();

    // O L254-258：Option<Res<CursorPositionSS>> unwrap（None → (0,0)）。
    let cursor_ss = cursor.as_ref().map(|c| c.0).unwrap_or(Vec2::ZERO);

    // O L259-314：`EventReader<MouseWheel>` 双缓冲遍历累计滚轮 (x,y)。
    let mut scroll = Vec2::ZERO;
    for ev in mouse_wheel.read() {
        scroll += Vec2::new(ev.x, ev.y);
    }

    // O L403-411：`*local_100(mei+0x268) = cursor / scale`（divps）。
    mei.cursor_units = cursor_ss / scale;

    // O L400-445：`MainActionSet::deconstruct()` 得到 12B 记录 Vec；
    // **仅当 `nav.focus.tag == 2` 且记录非空**时，以焦点 id 为起点
    // `process_input_action_handlers`（0x140684874 门控），未 handled 的记录
    // 压实保留（retain 语义）。
    let mut records = actions.deconstruct();
    if nav_state.focus.tag == 2 && !records.is_empty() {
        let focus_id = GlobalId(nav_state.focus.payload);
        records.retain(|record| {
            let action = ActionSetAction::from_bytes(record)
                .expect("internally generated MEI action record must be valid");
            !mei.process_input_action_handlers(focus_id, &action)
        });
    }

    // O L446-451：`Vec::retain<ActionSetAction, closure_env$2>`（0x140fcdae0）。
    // fresh 机器码：closure 捕获 `UiNavState+0x30`（hold 字节，`movzx r15d,[r8]`），
    // 对每个 action 调 `ActionSetAction::just_pressed(id)`，id = 0x13..0x17
    // （ui_confirm/ui_left/ui_right/ui_down/ui_up）。命中任一 just_pressed 时
    // 置 flag=1；保留当且仅当 `hold`。flag=1 时随后刷新 timer=now，
    // `hold = timer.map_or(false, |t| t.elapsed().as_secs() < 120)`（0x1406849bd..a45）。
    let mut nav_action_pressed = false;
    records.retain(|record| {
        let action = ActionSetAction::from_bytes(record)
            .expect("internally generated MEI action record must be valid");
        let pressed = [0x13u8, 0x14, 0x15, 0x16, 0x17]
            .into_iter()
            .any(|slot| action.just_pressed(slot));
        if pressed {
            nav_action_pressed = true;
            nav_state.hold
        } else {
            true
        }
    });
    nav_state.update_hold(nav_action_pressed);

    // O L472-475：`reconstruct` → `memcpy(mei+0x30, set, 0x170)`。
    mei.actions = CoreMainActionSet::reconstruct(&records)
        .expect("filtered MEI action records must be valid");
    // Original memcpy into MeiContext+30 also writes digital19 at+c8 and
    // digital20..23 at+d0..ef. These are the same native action-set fields;
    // the Rust source projection stores them separately, so copy full states.
    mei.click_action = mei.actions.digital[19];
    mei.nav_actions = mei.actions.digital[20..24].try_into().unwrap();

    // ------------------------------------------------------------------
    // O L476-539：每节点 pre_update（vtable +0x58）。
    // 实参：`(&mut cursor_units, &mut node.state_bits)` + `&mei+0x30`(actions)。
    // ------------------------------------------------------------------
    let cursor_units = mei.cursor_units;
    let frame_actions = mei.actions;
    let ids: Vec<GlobalId> = {
        let mut ids: Vec<GlobalId> = mei.nodes.keys().copied().collect();
        // 原生为 hashbrown 桶序（order-sensitive 仅影响声音/事件发射次序，U）。
        ids.sort_unstable();
        ids
    };
    for id in &ids {
        if let Some(node) = mei.nodes.get_mut(id) {
            node.content
                .pre_update(&frame_actions, cursor_units, &mut node.state_bits);
        }
    }

    // ------------------------------------------------------------------
    // O L540-891：命中测试与交互状态机。
    // ------------------------------------------------------------------
    for id in &ids {
        let (old_state, interactive, has_content, rect_min, rect_max, hover_sound, click_sound) = {
            let node = mei.nodes.get(id).unwrap();
            let interactive = (node.flags & NodeFlags::IGNORE_INPUT) == 0
                && ((node.flags & NodeFlags::FORCE_INTERACTIVE) != 0 || node.content.interactive());
            let min = node.pos * scale;
            let max = (node.pos + node.size) * scale;
            (
                node.state_bits,
                interactive,
                !node.content.is_empty(),
                min,
                max,
                node.sound_hover,
                node.sound_click,
            )
        };

        // O L620-639：`movmskpd` 四界比较 → 任一界外则 `iVar19 != 0`。
        let inside = cursor_ss.x >= rect_min.x
            && cursor_ss.x <= rect_max.x
            && cursor_ss.y >= rect_min.y
            && cursor_ss.y <= rect_max.y;

        // O L640-647 / L720-723：`hover = inside || (focus==Mei && focused==id && hold)`。
        // fresh：`cmp dword [nav+0x10],2; cmp [nav+0x18],node_id; cmp byte [nav+0x30],0`。
        let focused_by_nav = nav_state.focused_id() == Some(id.0) && nav_state.hold;
        let hover = inside || focused_by_nav;

        {
            let node = mei.nodes.get_mut(id).unwrap();
            if hover {
                node.state_bits |= NodeStateBits::HOVER;
            } else {
                node.state_bits &= !NodeStateBits::HOVER;
            }
        }

        // O L651-854：仅内容非空节点进入 widget 注册/事件分发。
        if has_content && interactive && hover {
            // O L656-675：`Option<ResMut<UiState>>` 缺失 → panic_fmt
            // （原版为 unwrap/expect，串 U）。
            let state = ui_state
                .as_ref()
                .expect("mei_pre_update: UiState resource missing");

            // O L676-683：`alloc::fmt::format::format_inner`（id → 名字串）。
            // **分歧（I）**：格式串/字段面 U；本实现用 GlobalId 数值十进制。
            let name = format!("{}", id.0);

            let node_state = mei.nodes.get(id).unwrap().state_bits;
            let rect_size = rect_max - rect_min;
            // O L687-692：volume = {tag 3, layer = node 状态字节, size =
            // 扩张后矩形尺寸}；`extra_78` 由 `{fmt 串, node+0xe}` 拼出（U）。
            let params = WidgetParams::new(
                WidgetVolumeShape {
                    tag: 3,
                    layer: node_state,
                    size: rect_size,
                    ..Default::default()
                },
                [0; 2],
            )
            .no_history_recording()
            .occlude_by_world(true)
            // O L699：`allow_negative_distance()`——原生为带参设置，mei 传 true。
            .allow_negative_distance(true)
            .sounds(UiSounds::empty());

            // O L706：`UiState::add<String>`。
            let ctx = state.add(&name, params);

            // O L707-717：`slider3d::into_drag_state()`（0x14092df10）的三个状态位
            // （is_some→bit2、is_started→bit1、is_finished→bit3），每位置位前先清位
            // （`and 0xfb/0xfd/0xf7`，fresh 0x140684f20..f66）。
            let drag = ctx.into_drag_state();
            {
                let node = mei.nodes.get_mut(id).unwrap();
                if drag.is_some() {
                    node.state_bits |= NodeStateBits::DRAG;
                } else {
                    node.state_bits &= !NodeStateBits::DRAG;
                }
                if drag.is_started() {
                    node.state_bits |= NodeStateBits::DRAG_STARTED;
                } else {
                    node.state_bits &= !NodeStateBits::DRAG_STARTED;
                }
                if drag.is_finished() {
                    node.state_bits |= NodeStateBits::DRAG_FINISHED;
                } else {
                    node.state_bits &= !NodeStateBits::DRAG_FINISHED;
                }
            }

            // O L718-724：`clicked()` → 状态 bit4。**else 分支**（fresh
            // 0x140684f90..0x140684fcd）：`nav.focus.tag == 2 &&
            // nav.focus.payload == node_id && just_pressed(mei+0xc8)` 也置 bit4。
            let click_by_nav = !ctx.clicked()
                && nav_state.focus.tag == 2
                && nav_state.focus.payload == id.0
                && mei.click_action.just_pressed();
            if ctx.clicked() || click_by_nav {
                let node = mei.nodes.get_mut(id).unwrap();
                node.state_bits |= NodeStateBits::CLICKED;
            }

            // 取出本帧输入事件（仅焦点节点消费；未消费则留给 post_update 清空）。
            let focused = mei.nodes.get(id).unwrap().state_bits & NodeStateBits::FOCUS != 0;
            if focused {
                let events = std::mem::take(&mut mei.input_events);
                let node = mei.nodes.get_mut(id).unwrap();
                for ev in events {
                    // O L748-814：tag 0xc6 起剩余事件被丢弃（不转发）。
                    if ev.tag() == 0xc6 {
                        break;
                    }
                    // O L790：vtable +0x48 = process_input_action。
                    node.content.process_input_action(&ev);
                }
            }

            // O L829-853：`inside && scroll != 0` → 派发滚轮事件
            // （tag 0xc3，载荷 = 累计滚轮 Vec2）。
            if inside && scroll != Vec2::ZERO {
                let node = mei.nodes.get_mut(id).unwrap();
                node.content
                    .process_input_action(&MeiInputEvent::scroll(scroll));
            }
        } else if has_content && !interactive && inside {
            // 非交互节点仍参与悬停位的写入（上面已写）；不注册 widget。
        }

        // O L855-890：上升沿声音。
        // 悬停上升沿：`(old & 1) == 0 && (new & 1) != 0`，声音 id != 0x0d。
        let new_state = mei.nodes.get(id).unwrap().state_bits;
        if (old_state & NodeStateBits::HOVER) == 0
            && (new_state & NodeStateBits::HOVER) != 0
            && hover_sound != 0x0d
        {
            sounds.write(PlayMenuUiSound { kind: hover_sound });
        }
        // 点击/焦点上升沿：`(old & 0x30) == 0 && (new & 0x30) != 0`，id != 0x0d。
        if (old_state & 0x30) == 0 && (new_state & 0x30) != 0 && click_sound != 0x0d {
            sounds.write(PlayMenuUiSound { kind: click_sound });
        }
    }

    // Original14068548e clears frame events; after swap clear the new
    // current table, preserving previous nodes for the UI-building systems.
    mei.recycle_frame_nodes();
}

/// O 0x1406884c0 `toolbar::mei::core::mei_post_update`。
///
/// 逐段（`evidence/0x1406884c0-mei_post_update.c` + 2026-10-18 fresh
/// `work/restorer-mei-final/evidence/mei_post_update.asm.txt`）：
/// 1. `WindowSize::as_vec2`、`fVar77 = height/1000.0`（L300/L321）。
/// 2. 每节点 `content.render`（vtable +0x60），返回判别 `!= 2` 时把计算结果
///    写回节点（L380-407）。
/// 3. `compute_z_order_and_toposort`（L409-750）：bumpalo `StackEntry`
///    stride 0x40，LIFO；父节点累加 `z`，标志 `&2 | b`；子节点按 `children`
///    压栈。
/// 4. 全体节点清焦点位 `state &= 0xbf`（0x140689bb0）。
/// 5. `update_nav`（0x140689bb9..0x14068aca8，见 `update_nav` 文档）：
///    4 × `DigitalActionState::just_pressed` 方向、默认焦点、父链兄弟搜索，
///    命中写 `nav.focus = Mei(id)`。
/// 6. `UiFocus::as_mei` 命中 → 焦点节点 `state |= 0x40`（0x140689d1c）。
/// 7. `Vec<&mut Node>` + `sort_unstable_by`（**`f32::total_cmp` 升序**，
///    比较键 Node+0xac）；逐节点 `content.draw`（vtable +0x68）。
/// 8. `pack_srgba_u32`（L1597-1600）；焦点矩形链（未实现，见模块头）。
/// 9. 清空输入事件 Vec；全体 `state &= 0xcf`（清 bit4/bit5）。
#[allow(clippy::too_many_arguments)]
pub fn mei_post_update(
    mut mei: ResMut<MeiContext>,
    mut text: ResMut<TextRenderer>,
    mut nav_state: ResMut<UiNavState>,
    mut textures: ResMut<AssetTextureLibrary>,
    device: Res<GraphicsDeviceResource>,
    window_size: Res<WindowSize>,
    _steam_client: Option<Res<SteamClientResource>>,
    _steam_input: Option<Res<SteamInput>>,
    input_mode: Res<ActiveInputModeTracker>,
) {
    let scale = window_size.height as f32 / 1000.0;
    let window_points = Vec2::new(window_size.width as f32 / scale, 1000.0);
    let active_mode = input_mode.mode();
    // Native 0x14068881c..0x140688847: only expose the Steam client to
    // content preparation when SteamInput exists and controller mode is active.
    let steam_client = if _steam_input.is_some() && active_mode.is_controller() {
        _steam_client.as_deref()
    } else {
        None
    };

    // ------------------------------------------------------------------
    // O 0x14068899f: prepare_layout (vtable +0x60). Niche tag 2 means
    // no replacement; a present 16-byte width/height result is copied to
    // Node+0x60..0x70 at 0x1406889ba. It has no relation to z-order flags.
    // ------------------------------------------------------------------
    let ids: Vec<GlobalId> = {
        let mut ids: Vec<GlobalId> = mei.nodes.keys().copied().collect();
        ids.sort_unstable();
        ids
    };
    let actions = mei.actions;
    let cursor = mei.cursor_units;
    let elapsed = mei.elapsed_wrapped;
    for id in &ids {
        if let Some(node) = mei.nodes.get_mut(id) {
            // O L380：原生仅对内容指针非空（`node+0x80 != 0`）的节点调用。
            if !node.content.is_empty() {
                let mut ctx = NodePrepareContext {
                    text: &mut text,
                    device: &device,
                    textures: &mut textures,
                    state: &mut node.state_bits,
                    actions: &actions,
                    window_points,
                    steam_client,
                    steam_input: _steam_input.as_deref(),
                    pixels_per_point: scale,
                    cursor,
                    elapsed,
                };
                if let Some(size) = node.content.prepare_layout(&mut ctx) {
                    node.intrinsic_size = size;
                }
            }
        }
    }

    // Original140688c10..140689b60: roots are frame events, then
    // bottom-up minimum sizes and forward frame/child placement.
    mei.perform_layout(window_points);

    // ------------------------------------------------------------------
    // O L954-958：全体节点清焦点位（state byte & 0xbf）。
    // ------------------------------------------------------------------
    for node in mei.nodes.values_mut() {
        node.state_bits &= !NodeStateBits::FOCUS;
    }

    // ------------------------------------------------------------------
    // O L1056-2093：update_nav（fresh 2026-10-18 逐指令重建，
    // `work/restorer-mei-final/evidence/mei_post_update.asm.txt`
    // 0x140689bb9..0x14068aca8）。
    //
    // 1. 现有 Mei 焦点节点缺失 → 清焦点（qword `nav+0x10` = 0，payload 不动）。
    // 2. `nav.focus.is_toolbar()` → 跳过导航输入（仍由尾部写焦点位）。
    // 3. 方向 = 4 个 `DigitalActionState`（`mei+0xd0/d8/e0/e8` = left/right/down/up）
    //    的 pairwise `just_pressed`：
    //      left  = js(d0) && !js(d8)   right = js(d8) && !js(d0)
    //      down  = js(e0) && !js(e8)   up    = js(e8) && !js(e0)
    //      dir = (right-left, down-up)
    //    `[0x38]` = up|left（负向），`[0x98]` = left|right（水平）。
    // 4. 全节点扫描求 default-focus 候选 / any_with_nav / first_with_nav /
    //    current_found / blocked / 焦点中心。
    // 5. 焦点修复：缺失→清；候选子树含 with_nav → 设焦点；无焦点且有 with_nav →
    //    设首节点。
    // 6. 仅 `current_found && !blocked`、恰好一个方向、焦点为 Mei 时，沿
    //    `mei.parents` 上溯：父节点 swizzle 轴向与输入轴一致时，取当前节点在
    //    父 children 中的前/后兄弟段（负向取前段），逆序逐个做子树搜索
    //    （`closure_env$2`：with_nav 节点中心沿 dir 方向、距离最近者）。
    // 7. 命中 → `nav.focus = Mei(candidate)`。
    update_nav(&mut mei, &mut nav_state);

    // O L1424-1485 / L1602：`UiFocus::as_mei` 命中 → 焦点节点 `state |= 0x40`
    // （fresh 0x140689d1c..0x140689e59 / 0x14068a9e8 段；节点缺失 unwrap panic）。
    if let Some(id) = nav_state.focus.as_mei() {
        let node = mei
            .nodes
            .get_mut(&GlobalId(id))
            .expect("mei_post_update: focused node missing");
        node.state_bits |= NodeStateBits::FOCUS;
    }

    // ------------------------------------------------------------------
    // O L1544-1591：`Vec<&mut Node>` + sort_unstable_by + 逐节点 draw。
    // fresh 比较器（内联于 insertion_sort_shift_left
    // `0x140fe4040`）：`[node+0xac]` 的 f32 位按 `v ^ ((v>>31)>>1)` 变换后
    // 无符号比较 —— 即 `f32::total_cmp` 升序（NaN 也有全序）。**判等**。
    // ------------------------------------------------------------------
    let mut draw_ids: Vec<GlobalId> = mei.nodes.keys().copied().collect();
    draw_ids.sort_by(|a, b| {
        let za = mei.nodes.get(a).map(|n| n.z).unwrap_or(0.0);
        let zb = mei.nodes.get(b).map(|n| n.z).unwrap_or(0.0);
        za.total_cmp(&zb)
    });
    for id in &draw_ids {
        if let Some(node) = mei.nodes.get_mut(id) {
            if !node.content.is_empty() {
                // O L1573-1588：`node+0x80` 内容指针非空；
                // 实参 {pos, size, opacity, state 字节, scale}。
                let mut ctx = NodeDrawContext {
                    text: &mut text,
                    center: node.pos + node.size * 0.5,
                    extent: node.size,
                    opacity: node.opacity,
                    pixels_per_point: scale,
                    cursor,
                    elapsed,
                    state: node.state_bits,
                };
                node.content.draw(&mut ctx);
            }
        }
    }

    // Original14068a9e8: controller focus rectangle, unless native flag0x40
    // suppresses it. ShapeStream helpers were previously misclassified as an
    // unrelated edit-sequence API; this is actual tg-text shape output.
    if active_mode.is_controller() {
        if let Some(id) = nav_state.focus.as_mei() {
            let node = mei
                .nodes
                .get(&GlobalId(id))
                .expect("MEI focus rectangle node missing");
            if node.flags & 0x40 == 0 {
                let stream = tg_text::ShapeStream::new()
                    .fill_color(pack_srgba_u32(1., 1., 1., 0.15))
                    .corner_radius(3.)
                    .encode_no_images()
                    .expect("encode MEI focus rectangle");
                text.rect(node.pos.to_array(), node.size.to_array(), 1., &stream)
                    .expect("render MEI focus rectangle");
            }
        }
    }

    // O L1710-1739：清空输入事件 Vec（drop KeyboardInput/字符串载荷）。
    mei.input_events.clear();

    // O L1765-1770：全体节点 `state &= 0xcf`（清 bit4/bit5）。
    for node in mei.nodes.values_mut() {
        node.state_bits &= 0xcf;
    }
}

/// `UiFocus` 的 qword 空判别（O 0x140985f30：`cmp qword [rcx],0; setz al`）——
/// 与 `UiFocus::is_none()`（只比 tag）不同，原生同时要求 `+0x04` 填充为 0。
fn focus_qword_is_none(focus: &UiFocus) -> bool {
    focus.tag == 0 && focus._pad == 0
}

/// 写 `UiFocus` 的 Mei 变体（O：`mov qword [nav+0x10],2; mov [nav+0x18],id`）。
fn set_mei_focus(nav: &mut UiNavState, id: u64) {
    nav.focus.tag = 2;
    nav.focus._pad = 0;
    nav.focus.payload = id;
}

/// 清焦点只写 tag+pad 的 qword（O：`mov qword [nav+0x10],0`；payload 不写）。
fn clear_focus_qword(nav: &mut UiNavState) {
    nav.focus.tag = 0;
    nav.focus._pad = 0;
}

/// O `iter_subtree<GlobalId, update_nav::closure_env$0>`（0x140f9e3e0）：
/// `nodes.get(id).expect(...)`；闭包 = `node.flags & WITH_NAV != 0`，命中即返回
/// true（短路），否则递归 children。
fn iter_subtree_has_nav(mei: &MeiContext, id: GlobalId) -> bool {
    let node = mei
        .nodes
        .get(&id)
        .expect("mei_post_update: iter_subtree node missing");
    if node.flags & NodeFlags::WITH_NAV != 0 {
        return true;
    }
    for child in &node.children {
        if iter_subtree_has_nav(mei, *child) {
            return true;
        }
    }
    false
}

/// O `iter_subtree<(), update_nav::closure_env$2>`（0x140f9e610）的节点访问：
/// 仅 `flags & WITH_NAV` 的节点参与候选；`delta = (pos + size*0.5) - focus_center`，
/// `len = |delta|`，`dot = delta.normalize_or_zero() · dir.normalize_or_zero()`；
/// `0.0 < dot && len < best` → `out = Some(id)`、`best = len`。
/// 子节点**全部**访问（无短路）。**判等**（逐 SSE 指令，2026-10-18 fresh）。
fn subtree_search(
    mei: &MeiContext,
    root: GlobalId,
    focus_center: Vec2,
    dir: Vec2,
    best: &mut f32,
    out: &mut Option<GlobalId>,
) {
    let node = mei
        .nodes
        .get(&root)
        .expect("mei_post_update: iter_subtree node missing");
    if node.flags & NodeFlags::WITH_NAV != 0 {
        let center = node.pos + node.size * 0.5;
        let delta = center - focus_center;
        let len = delta.length();
        let dot = delta.normalize_or_zero().dot(dir.normalize_or_zero());
        if dot > 0.0 && len < *best {
            *best = len;
            *out = Some(root);
        }
    }
    for child in &node.children {
        subtree_search(mei, *child, focus_center, dir, best, out);
    }
}

/// O `toolbar::mei::core::update_nav`（mei_post_update 0x140689bb9..0x14068aca8）。
///
/// 全部控制流/比较/写面按 fresh 机器码重建；仅以下为登记项：
/// - Original action-set field aliases are mirrored by mei_pre_update.
/// - 默认焦点候选依赖 `HashMap` 迭代序（原生 hashbrown 桶序；本实现同构但
///   随机种子不同）→ 多候选时取胜者 U。
fn update_nav(mei: &mut MeiContext, nav: &mut UiNavState) {
    // 1. 现有 Mei 焦点节点缺失 → 清焦点（0x140689bc8..0x140689d01）。
    if nav.focus.tag == 2 && !mei.nodes.contains_key(&GlobalId(nav.focus.payload)) {
        clear_focus_qword(nav);
    }
    // 2. Toolbar 焦点跳过导航输入（0x140689d0f..0x140689e04）。
    if nav.focus.tag == 1 {
        return;
    }

    // 3. 方向输入（0x140689e04..0x14068a1fd）。
    let js = |s: DigitalActionState| s.just_pressed();
    let left = js(mei.nav_actions[0]) && !js(mei.nav_actions[1]);
    let right = js(mei.nav_actions[1]) && !js(mei.nav_actions[0]);
    let down = js(mei.nav_actions[2]) && !js(mei.nav_actions[3]);
    let up = js(mei.nav_actions[3]) && !js(mei.nav_actions[2]);
    let dir = Vec2::new(
        f32::from(right as u8) - f32::from(left as u8),
        f32::from(down as u8) - f32::from(up as u8),
    );

    // 4. 全节点扫描（0x14068a261..0x14068a35a）。
    let focus_is_none = focus_qword_is_none(&nav.focus);
    let mut default_focus: Option<GlobalId> = None;
    let mut any_with_nav = false;
    let mut first_with_nav = GlobalId(0);
    let mut current_found = false;
    let mut blocked = false;
    let mut focus_center = Vec2::ZERO;
    for (id, node) in mei.nodes.iter() {
        let flags = node.flags;
        let clicked = node.state_bits & NodeStateBits::CLICKED != 0;
        let with_nav = flags & NodeFlags::WITH_NAV != 0;
        if flags & 0x08 != 0 {
            // flags bit3：直接成为默认焦点并阻断导航。
            blocked = true;
            default_focus = Some(*id);
        } else if clicked {
            if flags & NodeFlags::DEFAULT_FOCUS != 0 {
                blocked |= with_nav;
                if with_nav || focus_is_none {
                    default_focus = Some(*id);
                }
            } else if with_nav {
                blocked = true;
                default_focus = Some(*id);
            }
        } else if flags & NodeFlags::DEFAULT_FOCUS != 0 && focus_is_none {
            default_focus = Some(*id);
        }
        if !any_with_nav {
            if with_nav {
                first_with_nav = *id;
            }
            any_with_nav = with_nav;
        }
        if nav.focus.tag == 2 && nav.focus.payload == id.0 {
            current_found = true;
            focus_center = node.pos + node.size * 0.5;
        }
    }

    // 5. 焦点修复（0x14068a35a..0x14068a3df）。
    if !current_found {
        clear_focus_qword(nav);
    }
    if let Some(id) = default_focus {
        if iter_subtree_has_nav(mei, id) {
            set_mei_focus(nav, id.0);
        }
    }
    if focus_qword_is_none(&nav.focus) && any_with_nav {
        set_mei_focus(nav, first_with_nav.0);
    }
    // 导航移动门：焦点节点存在且未被 flags bit3 / clicked+with_nav 阻断。
    if !(current_found && !blocked) {
        return;
    }

    // 6. 恰好一个方向（0x14068a3e5..0x14068a3f6）。
    let pressed_count = u8::from(left) + u8::from(right) + u8::from(down) + u8::from(up);
    if pressed_count != 1 {
        return;
    }
    if nav.focus.tag != 2 {
        return;
    }
    let mut current = GlobalId(nav.focus.payload);
    if mei.parents.is_empty() {
        return;
    }
    let horizontal = left || right;
    let negative = up || left;

    // 7. 沿父链上溯找兄弟组（0x14068a46f..0x14068aca8）。
    loop {
        let Some(&parent) = mei.parents.get(&current) else {
            return;
        };
        let Some(parent_node) = mei.nodes.get(&parent) else {
            return;
        };
        // 父布局主轴与输入轴不一致 → 上溯一层。
        if (parent_node.style.swizzle == 0) != horizontal {
            current = parent;
            continue;
        }
        let idx = parent_node
            .children
            .iter()
            .position(|c| *c == current)
            .expect("mei_post_update: current node missing from parent children");
        // 负向（left/up）取当前节点**之前**的兄弟段并逆序（由近到远）；
        // 正向（right/down）取**之后**的兄弟段，保持原序（不逆序）。
        // fresh 0x14068a65c 分支：`[0x38]`(negative) 非零 → memcpy [0..idx]
        // 后走 reverse；为零 → memcpy [idx+1..] 直接迭代。
        let siblings: Vec<GlobalId> = if negative {
            let mut before = parent_node.children[..idx].to_vec();
            before.reverse();
            before
        } else {
            parent_node.children[idx + 1..].to_vec()
        };
        let mut best = f32::MAX;
        let mut found: Option<GlobalId> = None;
        for sibling in siblings {
            subtree_search(mei, sibling, focus_center, dir, &mut best, &mut found);
            if found.is_some() {
                break;
            }
        }
        if let Some(candidate) = found {
            set_mei_focus(nav, candidate.0);
            return;
        }
        current = parent;
    }
}

/// 颜色打包（O：`utils::pack_srgba_u32`，L1597-1600 以
/// `{1.0,1.0,1.0}` + `{1.0,0.15}` 常量调用）。**重建（I）**：原函数在 utils
/// crate，未落库；此处按 sRGBA 线性打包语义镜像（去向 U）。
fn pack_srgba_u32(r: f32, g: f32, b: f32, a: f32) -> u32 {
    let q = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u32;
    (q(r) << 24) | (q(g) << 16) | (q(b) << 8) | q(a)
}

/// mei 系统共用的只读配置（O：`mei_pre_update` 的 `Res<GameConfig>` 出现在
/// `render_taffy_ui_to_widgets` 的 13 参中；本别名供接线层引用，避免重复导入）。
pub type MeiGameConfig = GameConfig;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mei::core::{EmptyContent, Node};
    use country_core::systems::ui::resources::widget_params::UiFocus;

    /// O：post_update L1597-1600 `pack_srgba_u32(1.0,1.0,1.0,0.15)`。
    #[test]
    fn pack_srgba_white_and_alpha() {
        let packed = pack_srgba_u32(1.0, 1.0, 1.0, 0.15);
        assert_eq!((packed >> 24) & 0xff, 0xff);
        assert_eq!((packed >> 16) & 0xff, 0xff);
        assert_eq!((packed >> 8) & 0xff, 0xff);
        assert_eq!(packed & 0xff, 38); // round(0.15 * 255) = 38
    }

    fn nav_node(x: f32) -> Node {
        let mut node = Node::new(Box::new(EmptyContent));
        node.pos = Vec2::new(x, 0.0);
        node.size = Vec2::new(10.0, 10.0);
        node.flags |= NodeFlags::WITH_NAV;
        node
    }

    fn pressed_state() -> DigitalActionState {
        DigitalActionState {
            phase: country_core::input::action_set::DigitalActionPhase::JustPressed,
            timer_seconds: 0.0,
        }
    }

    fn nav_with_children() -> (MeiContext, UiNavState) {
        let mut mei = MeiContext::default();
        let a = GlobalId(1);
        let b = GlobalId(2);
        let parent = GlobalId(9);
        let mut p = Node::new(Box::new(EmptyContent));
        p.children = vec![a, b];
        mei.nodes.insert(parent, p);
        mei.nodes.insert(a, nav_node(0.0));
        mei.nodes.insert(b, nav_node(100.0));
        mei.build_parent_links();
        let mut nav = UiNavState::default();
        set_mei_focus(&mut nav, a.0);
        (mei, nav)
    }

    /// O update_nav 0x14068a46f..0x14068aca8：右方向从子项 a 前进到兄弟 b。
    #[test]
    fn update_nav_moves_to_next_sibling_right() {
        let (mut mei, mut nav) = nav_with_children();
        mei.nav_actions[1] = pressed_state(); // right
        update_nav(&mut mei, &mut nav);
        assert_eq!(
            nav.focus,
            UiFocus {
                tag: 2,
                _pad: 0,
                payload: 2
            }
        );
    }

    /// 左方向从 b 回退到前一个兄弟 a（前段逆序）。
    #[test]
    fn update_nav_moves_to_previous_sibling_left() {
        let (mut mei, mut nav) = nav_with_children();
        set_mei_focus(&mut nav, 2);
        mei.nav_actions[0] = pressed_state(); // left
        update_nav(&mut mei, &mut nav);
        assert_eq!(
            nav.focus,
            UiFocus {
                tag: 2,
                _pad: 0,
                payload: 1
            }
        );
    }

    /// O 0x140689bc8：Mei 焦点节点缺失 → 清焦点（含默认焦点重建路径）。
    #[test]
    fn update_nav_clears_missing_focus() {
        let mut mei = MeiContext::default();
        let mut nav = UiNavState::default();
        set_mei_focus(&mut nav, 0xdead);
        update_nav(&mut mei, &mut nav);
        assert_eq!(nav.focus.tag, 0);
    }

    /// O 0x14068a3e5：非恰好一个方向时不动焦点。
    #[test]
    fn update_nav_requires_exactly_one_direction() {
        let (mut mei, mut nav) = nav_with_children();
        mei.nav_actions[1] = pressed_state(); // right
        mei.nav_actions[2] = pressed_state(); // down
        update_nav(&mut mei, &mut nav);
        assert_eq!(nav.focus.payload, 1);
    }
}
