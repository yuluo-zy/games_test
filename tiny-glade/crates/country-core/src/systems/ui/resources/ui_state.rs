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
}
impl UiState {
    pub fn new(interaction: UiInteraction) -> Self {
        Self {
            interaction,
            clearing_canvases: RwLock::new(HashMap::new()),
        }
    }
    pub fn replace_interaction(&mut self, interaction: UiInteraction) {
        self.interaction = interaction;
    }
    pub fn interaction(&self) -> UiInteraction {
        self.interaction
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
}
