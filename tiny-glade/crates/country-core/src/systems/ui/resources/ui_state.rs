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
    pub native_tag: u8,
    pub native_phase: u8,
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
    clearing_canvases: RwLock<HashMap<WidgetId, Arc<ClearingCanvasWidget>>>,
    /// +0x68：`HashMap<WidgetId, Arc<WidgetParams>>`（O：ui_state 章/DI-§2.4；
    /// `add` 0x1409236d0 / `add_with_id` 0x14092e3e0 插表）。
    widgets: RwLock<HashMap<WidgetId, Arc<WidgetParams>>>,
}
impl UiState {
    pub fn new(interaction: UiInteraction) -> Self {
        Self {
            interaction,
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
                no_history_recording: true,
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
    pub fn dragged(&self) -> u8 {
        let active = self.state.interaction;
        if active.native_tag >= 2 && active.widget == self.id {
            active.native_phase
        } else {
            2
        }
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

    /// `is_active` @0x14092e680（O，review-audit 纠错：含 [+0x28]!=2 门控）：
    /// `hover!=0 && id 匹配 && 交点载荷判别 != 2`（交点判别 = native_tag 面近似；
    /// 原交点载荷 @+0x28..0x58 7 qword 由宿主写入，U）。
    pub fn is_active(&self) -> bool {
        let active = self.state.interaction;
        active.native_tag != 0
            && active.widget == self.id
            && active.native_tag != 2
    }
}
