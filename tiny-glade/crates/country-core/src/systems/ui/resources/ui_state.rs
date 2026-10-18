//! B clearing_canvas_w_layer_and_sound 0x14092df50 and dragged 0x14092df10.
//! This owner represents the recovered canvas registry and interaction query
//! fields, not the complete native UiState memory layout.
use bevy_ecs::resource::Resource;
use glam::Vec2;
use hashbrown::HashMap;
use parking_lot::RwLock;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
};

use super::widget_params::WidgetParams;

/// 拖拽状态（O：`country_core::systems::ui::resources::ui_state::DragState`，
/// **1 B 判别枚举**）。
///
/// 2026-10-17 fresh 机器码（B patched, shared worker）：
/// - `is_some`   0x14092e6b0：`cmp byte [rcx],2; setne al` → 判别 != 2；
/// - `is_started` 0x14092e6c0：`cmp byte [rcx],3; sete al` → 判别 == 3；
/// - `is_finished` 0x14092e6d0：`cmp byte [rcx],2; setb al` → 判别 < 2（0 或 1）。
///
/// 因此判别值：`0/1` = 两种 finished 变体（`UiWidgetContext::clicked`
/// 0x14092e5f0 只认 1，即 `<2 && &1`），`2` = None，`3` = Started。
/// 原生是 Rust enum；用 1 B 透明新类型承载，保留任意原始判别值（`into_drag_state`
/// 0x14092df10 直接把 `UiState+0x19` 字节作为返回值）。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DragState(pub u8);

impl DragState {
    /// 判别 0：finished 变体（`is_finished` 真、`clicked` 假）。
    pub const CANCELLED: Self = Self(0);
    /// 判别 1：finished 变体（`clicked` 只认此值，`0x14092e5f0` `&1`）。
    pub const FINISHED: Self = Self(1);
    /// 判别 2：无拖拽（`dragged` 未命中时返回）。
    pub const NONE: Self = Self(2);
    /// 判别 3：拖拽已开始。
    pub const STARTED: Self = Self(3);

    /// 0x14092e6b0：`byte != 2`。
    pub fn is_some(self) -> bool {
        self.0 != 2
    }

    /// 0x14092e6c0：`byte == 3`。
    pub fn is_started(self) -> bool {
        self.0 == 3
    }

    /// 0x14092e6d0：`byte < 2`（无符号）。
    pub fn is_finished(self) -> bool {
        self.0 < 2
    }
}

impl Default for DragState {
    fn default() -> Self {
        Self::NONE
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WidgetId(pub u64);
impl WidgetId {
    /// B 0x1405bdda0 uses zero-key SipHasher13 and the str Hash terminator.
    /// Reuse std's implementation; this is unrelated to NameHash/xxh3.
    pub fn from_name(name: &str) -> Self {
        let mut hash = DefaultHasher::new();
        name.hash(&mut hash);
        Self(hash.finish())
    }
    pub fn clearing_canvas(name: &str) -> Self {
        Self::from_name(&format!("{name}_canvas"))
    }
}

/// Fields observed at UiState +0x18, +0x19 and +0x38. An input owner must
/// supply these explicitly; missing input must not imply a dragging widget.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiInteraction {
    /// +0x18: hover byte (`CMP byte [state+0x18],1/2` in 0x14092e5f0/620/680).
    pub native_tag: u8,
    /// +0x19: click byte (returned by `hovered`; tested by `clicked`).
    pub native_phase: u8,
    /// +0x38: active widget id compared against `UiWidgetContext` id.
    pub widget: WidgetId,
}

/// The exact clearing-canvas specialization registered by the garden caller.
/// The original constructor stores volume tag 1, argument at +0x79, shape tag
/// 4, layer at +0x89, extent (130,130), occlusion=false and flag byte=1.
/// All four sound slots and the signifier vector are empty in this caller.
#[derive(Clone, Debug)]
pub struct ClearingCanvasWidget {
    pub argument_79: u8,
    pub layer: u8,
    pub extent: Vec2,
    pub occlude_by_world: bool,
    pub no_history_recording: bool,
}

#[derive(Resource)]
pub struct UiState {
    interaction: UiInteraction,
    /// +0x28: intersection payload discriminant. `is_active` 0x14092e680
    /// rejects value 2 (`CMP dword ptr [RAX+0x28],2; SETNZ`). The remaining
    /// intersection bytes are host-owned and not modelled here.
    intersection_tag: u32,
    clearing_canvases: RwLock<HashMap<WidgetId, Arc<ClearingCanvasWidget>>>,
    /// +0x68：`HashMap<WidgetId, Arc<WidgetParams>>`（O：ui_state 章/DI-§2.4；
    /// `add` 0x1409236d0 / `add_with_id` 0x14092e3e0 插表）。The parking-lot
    /// `RwLock` at native +0x60 guards this map (0x14092e3e0 LOCK word at
    /// `[state+0x60]`, `memcpy` 0xb0 params, `Arc` 0xc0 insertion).
    widgets: RwLock<HashMap<WidgetId, Arc<WidgetParams>>>,
}
impl UiState {
    /// B add_ui @140945d25..d70 installs empty registries and an inactive
    /// interaction (native +0x18=0). Inactive payload bytes are not initialized
    /// by that constructor; NONE/id0 below are safe owned projections only.
    pub fn at_startup() -> Self {
        let mut state = Self::new(UiInteraction {
            native_tag: 0,
            native_phase: DragState::NONE.0,
            widget: WidgetId(0),
        });
        state.intersection_tag = 2;
        state
    }

    pub fn new(interaction: UiInteraction) -> Self {
        Self {
            interaction,
            intersection_tag: 0,
            clearing_canvases: RwLock::new(HashMap::new()),
            widgets: RwLock::new(HashMap::new()),
        }
    }
    pub fn replace_interaction(&mut self, interaction: UiInteraction) {
        self.interaction = interaction;
    }
    pub fn interaction(&self) -> UiInteraction {
        self.interaction
    }

    /// +0x18 hover byte.
    pub fn hover_byte(&self) -> u8 {
        self.interaction.native_tag
    }

    /// +0x19 click byte.
    pub fn click_byte(&self) -> u8 {
        self.interaction.native_phase
    }

    /// +0x28 intersection discriminant (read by `is_active`).
    pub fn intersection_tag(&self) -> u32 {
        self.intersection_tag
    }

    /// Writes the +0x28 intersection discriminant (input owner).
    pub fn set_intersection_tag(&mut self, tag: u32) {
        self.intersection_tag = tag;
    }

    /// +0x38 active widget id.
    pub fn active_widget(&self) -> WidgetId {
        self.interaction.widget
    }

    /// `UiState::add<&str>` @0x1409236d0（O）：名字 → WidgetId::from_name →
    /// add_with_id（0xb0B WidgetParams 拷贝 + Arc 0xc0B 插表）。
    pub fn add(&self, name: &str, params: WidgetParams) -> UiWidgetContext<'_> {
        self.add_with_id(WidgetId::from_name(name), params)
    }

    /// `add_with_id` @0x14092e3e0（O）：写锁（1e9 ns 超时）→ 拷贝 0xb0B
    /// WidgetParams → Arc 0xc0B → HashMap@+0x68 insert（旧 Arc 递减）。
    pub fn add_with_id(&self, id: WidgetId, params: WidgetParams) -> UiWidgetContext<'_> {
        self.widgets.write().insert(id, Arc::new(params));
        UiWidgetContext { state: self, id }
    }

    /// `UiState::widget` @0x14067dd80（O：1177 callers ICF 折叠）：按 WidgetId
    /// 查 Arc<WidgetParams>。
    pub fn widget(&self, id: WidgetId) -> Option<Arc<WidgetParams>> {
        self.widgets.read().get(&id).cloned()
    }

    /// 构造交互上下文（I：装饰 UI 系统族以活动 widget id 查询相位）。
    pub fn widget_context(&self, id: WidgetId) -> UiWidgetContext<'_> {
        UiWidgetContext { state: self, id }
    }

    /// Specialization with UiSounds::empty, as supplied at both Garden call
    /// sites. Insertion replaces and drops the previous Arc on every call.
    pub fn clearing_canvas_w_layer_and_sound(
        &self,
        name: &str,
        argument_79: u8,
        layer: u8,
    ) -> UiWidgetContext<'_> {
        let id = WidgetId::clearing_canvas(name);
        self.clearing_canvases.write().insert(
            id,
            Arc::new(ClearingCanvasWidget {
                argument_79,
                layer,
                extent: Vec2::splat(130.0),
                occlude_by_world: false,
                no_history_recording: false,
            }),
        );
        UiWidgetContext { state: self, id }
    }
    pub fn clearing_canvas_params(&self, id: WidgetId) -> Option<Arc<ClearingCanvasWidget>> {
        self.clearing_canvases.read().get(&id).cloned()
    }
}

pub struct UiWidgetContext<'a> {
    state: &'a UiState,
    pub id: WidgetId,
}
impl UiWidgetContext<'_> {
    /// `dragged` 0x14092df10（与 `slider3d::impl$1::into_drag_state` 同址 ICF 折叠）：
    /// `byte[state+0x18] >= 2 && qword[state+0x38] == id` → `byte[state+0x19]`，
    /// 否则 `2`（None）。
    pub(crate) fn drag_state_raw(&self) -> DragState {
        let active = self.state.interaction;
        if active.native_tag >= 2 && active.widget == self.id {
            DragState(active.native_phase)
        } else {
            DragState::NONE
        }
    }

    /// 原始字节视图（历史调用方仍按 `u8` 比较；原生返回即该字节）。
    pub fn dragged(&self) -> u8 {
        self.drag_state_raw().0
    }

    /// `clicked` @0x14092e5f0（O，ui_state 章）：`hover>1 && click<2 && click&1`
    /// （尾含 id 匹配 sete）。hover = interaction.native_tag、click = native_phase
    /// （近似映射；native 偏移 hover@+0x18/click@+0x19/id@+0x38）。
    pub fn clicked(&self) -> bool {
        let active = self.state.interaction;
        active.native_tag > 1
            && active.native_phase < 2
            && active.native_phase & 1 != 0
            && active.widget == self.id
    }

    /// `hovered` @0x14092e620（O，review-audit 纠错版）：`hover==1 && id 匹配`，
    /// 返回值 = click 字节（native_phase）。
    pub fn hovered(&self) -> u8 {
        let active = self.state.interaction;
        if active.native_tag == 1 && active.widget == self.id {
            active.native_phase
        } else {
            0
        }
    }

    /// `is_active` @0x14092e680 (fresh disassembly 2026-10-17):
    /// `byte[state+0x18] != 0 && qword[state+0x38] == id &&
    /// dword[state+0x28] != 2` → `SETNZ AL`. The intersection discriminant is
    /// the +0x28 dword, not the hover byte (the previous approximation compared
    /// `native_tag != 2`).
    pub fn is_active(&self) -> bool {
        let active = self.state.interaction;
        active.native_tag != 0 && active.widget == self.id && self.state.intersection_tag != 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(hover: u8, click: u8, widget: WidgetId, intersection: u32) -> UiState {
        let mut state = UiState::new(UiInteraction {
            native_tag: hover,
            native_phase: click,
            widget,
        });
        state.set_intersection_tag(intersection);
        state
    }

    /// 0x14092e5f0: `hover >= 2 && click <= 1 && click & 1 && id match`.
    #[test]
    fn clicked_matches_fresh_bytes() {
        let id = WidgetId(5);
        assert!(state(2, 1, id, 0).widget_context(id).clicked());
        assert!(!state(2, 0, id, 0).widget_context(id).clicked());
        assert!(!state(1, 1, id, 0).widget_context(id).clicked());
        assert!(!state(2, 1, WidgetId(6), 0).widget_context(id).clicked());
    }

    /// 0x14092e620: `hover == 1 && id match` returns the click byte.
    #[test]
    fn hovered_returns_the_click_byte() {
        let id = WidgetId(5);
        assert_eq!(state(1, 7, id, 0).widget_context(id).hovered(), 7);
        assert_eq!(state(1, 7, WidgetId(6), 0).widget_context(id).hovered(), 0);
        assert_eq!(state(2, 7, id, 0).widget_context(id).hovered(), 0);
    }

    /// 0x14092e680: hover != 0, id match and intersection tag != 2.
    #[test]
    fn is_active_reads_the_intersection_tag_at_0x28() {
        let id = WidgetId(5);
        assert!(state(3, 0, id, 0).widget_context(id).is_active());
        assert!(state(3, 0, id, 1).widget_context(id).is_active());
        assert!(!state(3, 0, id, 2).widget_context(id).is_active());
        assert!(!state(0, 0, id, 0).widget_context(id).is_active());
        assert!(!state(3, 0, WidgetId(6), 0).widget_context(id).is_active());
    }

    /// `add_with_id` stores the WidgetId → Arc<WidgetParams> entry in the
    /// +0x68 map (0x14092e3e0).
    #[test]
    fn widget_map_round_trips_by_id() {
        let state = UiState::new(UiInteraction {
            native_tag: 0,
            native_phase: 0,
            widget: WidgetId(0),
        });
        let id = WidgetId::from_name("wheel");
        let context = state.add("wheel", WidgetParams::default());
        assert_eq!(context.id, id);
        assert!(state.widget(id).is_some());
        assert!(state.widget(WidgetId(999)).is_none());
    }

    /// 0x14092e6b0 / 0x14092e6c0 / 0x14092e6d0 的逐判别行为。
    #[test]
    fn drag_state_methods_match_fresh_bytes() {
        let cases = [
            (0, true, false, true),
            (1, true, false, true),
            (2, false, false, false),
            (3, true, true, false),
            (9, true, false, false),
        ];
        for (raw, some, started, finished) in cases {
            let d = DragState(raw);
            assert_eq!(d.is_some(), some, "raw={raw}");
            assert_eq!(d.is_started(), started, "raw={raw}");
            assert_eq!(d.is_finished(), finished, "raw={raw}");
        }
        assert_eq!(DragState::default(), DragState::NONE);
    }

    /// 0x14092df10：`native_tag >= 2` 且 id 匹配时返回 +0x19 原字节，否则 None(2)。
    #[test]
    fn into_drag_state_returns_raw_phase_or_none() {
        let id = WidgetId(5);
        assert_eq!(
            state(2, 3, id, 0).widget_context(id).drag_state_raw(),
            DragState::STARTED
        );
        assert_eq!(
            state(3, 0, id, 0).widget_context(id).drag_state_raw(),
            DragState(0)
        );
        assert_eq!(
            state(1, 3, id, 0).widget_context(id).drag_state_raw(),
            DragState::NONE
        );
        assert_eq!(
            state(2, 3, WidgetId(6), 0)
                .widget_context(id)
                .drag_state_raw(),
            DragState::NONE
        );
        // 兼容视图。
        assert_eq!(state(2, 3, id, 0).widget_context(id).dragged(), 3);
        assert_eq!(state(1, 3, id, 0).widget_context(id).dragged(), 2);
    }
}
