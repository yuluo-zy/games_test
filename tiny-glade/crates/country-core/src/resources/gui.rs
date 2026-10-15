//! B GUI state and menu policies. Native tags are exposed only through the view;
//! owned Rust enums preserve behavior without promising the original compiler ABI.
use std::sync::Arc;

use bevy_ecs::resource::Resource;
use bevy_ecs::{event::Event, event::EventWriter, system::ResMut};
use glam::Vec2;
use parking_lot::Mutex;

use super::session::SessionId;

#[derive(Resource, Clone)]
pub struct EguiContextResource(pub egui::Context);

impl std::ops::Deref for EguiContextResource {
    type Target = egui::Context;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Resource)]
pub struct PendingEguiContextResource(pub egui::Context);

impl PendingEguiContextResource {
    /// B 0x14067dd80 moves the existing context without cloning its Arc.
    pub fn into_egui_context_resource(self) -> EguiContextResource {
        EguiContextResource(self.0)
    }
}

/// B toolbar::photomode::do_photomode reads byte 0 as the previous value,
/// writes enable/disable to byte 1, and copies byte 1 to byte 0 at frame end.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhotomodeScreen {
    pub was_enabled: bool,
    pub enabled: bool,
}

#[derive(Event, Clone, Copy, Debug, Default)]
pub struct EscapeMenuJustClosed;

/// Names describe the B menu writers; original variant spellings are unavailable.
#[derive(Resource, Clone, Debug, Default)]
pub enum EscMenuState {
    /// B menu plugin initializes the current state to word 2.
    #[default]
    Closed,
    Main,
    Quit,
    NewSession,
    Load(LoadMenuState),
    Settings(SettingsMenuState),
    /// The original flag is retained; its UI-specific meaning is not yet named.
    ReportBug(bool),
    Credits,
    /// The flag opens the LUT gallery; false leaves the world/menu policy closed.
    Photomode(bool),
}

impl EscMenuState {
    pub fn view(&self) -> EscMenuStateView {
        let (tag, flag) = match self {
            Self::Closed => (2, 0),
            Self::Main => (3, 0),
            Self::Quit => (4, 0),
            Self::NewSession => (5, 0),
            // B uses the hovered Option<usize> tag as the load variant's niche.
            Self::Load(state) => (u32::from(state.hovered_index.is_some()), 0),
            Self::Settings(state) => (7, state.native_tag()),
            Self::ReportBug(flag) => (8, u8::from(*flag)),
            Self::Credits => (9, 0),
            Self::Photomode(flag) => (10, u8::from(*flag)),
        };
        EscMenuStateView {
            tag,
            payload: 0,
            flag,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.view().is_closed()
    }
    pub fn is_open(&self) -> bool {
        self.view().is_open()
    }
    pub fn cast_ui_shadows(&self) -> bool {
        self.view().cast_ui_shadows()
    }
    pub fn allow_time_passage(&self) -> bool {
        self.view().allow_time_passage()
    }
    pub fn should_show_world(&self) -> bool {
        self.view().should_show_world()
    }
    pub fn can_limit_frame_rate(&self) -> bool {
        self.view().can_limit_frame_rate()
    }
    pub fn can_use_static_world_capture(&self) -> bool {
        self.view().can_use_static_world_capture()
    }
    pub fn allow_painterly_overlay(&self) -> bool {
        self.view().allow_painterly_overlay()
    }
}

#[derive(Resource, Debug, Default)]
pub struct NextEscMenuState(pub Option<EscMenuState>);

impl NextEscMenuState {
    /// B 0x14093fad0: release the old payload, then move the entire new state.
    pub fn set(&mut self, state: EscMenuState) {
        self.0 = Some(state);
    }
}

#[derive(Resource, Debug, Default)]
pub struct PrevEscMenuState(pub Option<EscMenuState>);

impl PrevEscMenuState {
    /// This setter shares the original NextEscMenuState::set body.
    pub fn set(&mut self, state: EscMenuState) {
        self.0 = Some(state);
    }

    /// B 0x14093fbb0: the None niche (word 11) yields no borrowed state.
    pub fn get(&self) -> Option<&EscMenuState> {
        self.0.as_ref()
    }
}

/// B toolbar::menu::advance_menu_state (0x14068e5b0), hosted in core so the
/// owner/move policy is reusable without an upper-layer implementation. Closing
/// emits EscapeMenuJustClosed before replacing current/previous state.
/// The shared Vec growth body is ICF-aliased to WriteSettingsFileCmd; it does
/// not identify the event. The original system's generic signature does.
pub fn advance_menu_state(
    current: &mut EscMenuState,
    next: &mut NextEscMenuState,
    previous: &mut PrevEscMenuState,
    menu_just_closed: impl FnOnce(),
) -> bool {
    let Some(incoming) = next.0.take() else {
        return false;
    };
    // Only the Closed variant emits the event, not Photomode(false).
    if matches!(incoming, EscMenuState::Closed) {
        menu_just_closed();
    }
    previous.set(std::mem::replace(current, incoming));
    true
}

/// Core-hosted Bevy adapter for the original toolbar system. Startup supplies
/// resources, event maintenance, and the original schedule ordering.
pub fn advance_menu_state_system(
    mut current: ResMut<EscMenuState>,
    mut next: ResMut<NextEscMenuState>,
    mut previous: ResMut<PrevEscMenuState>,
    mut closed: EventWriter<EscapeMenuJustClosed>,
) {
    let Some(incoming) = next.0.take() else {
        return;
    };
    if matches!(incoming, EscMenuState::Closed) {
        closed.write(EscapeMenuJustClosed);
    }
    let previous = &mut *previous;
    previous.set(std::mem::replace(&mut current, incoming));
}

/// B clone copies the selection/modal IDs but shares the metrics Arc at +0x30.
#[derive(Clone, Debug, Default)]
pub struct LoadMenuState {
    pub hovered_index: Option<usize>,
    pub backups: Option<LoadMenuBackupState>,
    pub thumbnail_metrics: Arc<Mutex<LoadMenuThumbnailMetrics>>,
    /// B +0x38/+0x39: the confirmation overlay's session identity.
    pub warning_session: Option<SessionId>,
    /// B +0x49/+0x4a: the theme-switch overlay's session identity.
    pub switch_glade_type: Option<SessionId>,
}

#[derive(Clone, Debug)]
pub struct LoadMenuBackupState {
    pub session: SessionId,
    pub page: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingsMenuState {
    Audio,
    Video(VideoSettingsSubMenu),
    Game,
    Ui,
    Language,
    Developer,
    Controls,
    /// Native tag 10 is cloned but the B settings renderer panics on it.
    /// No producer or original variant name has been identified.
    UnreachableTag10,
}

impl SettingsMenuState {
    fn native_tag(self) -> u8 {
        match self {
            Self::Video(submenu) => submenu as u8,
            Self::Audio => 3,
            Self::Game => 5,
            Self::Ui => 6,
            Self::Language => 7,
            Self::Developer => 8,
            Self::Controls => 9,
            Self::UnreachableTag10 => 10,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoSettingsSubMenu {
    Menu = 0,
    Image = 1,
    Display = 2,
}

impl VideoSettingsSubMenu {
    /// B 0x14093fcd0 is exactly tag == 0, despite the counterintuitive name.
    pub fn is_in_submenu(self) -> bool {
        matches!(self, Self::Menu)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EscMenuStateView {
    pub tag: u32,
    pub payload: u32,
    /// Boolean for word 10; a nested settings enum (not a bool) for tag 7.
    pub flag: u8,
}

impl EscMenuStateView {
    fn word(self) -> u64 {
        u64::from(self.tag) | u64::from(self.payload) << 32
    }

    /// B 0x14093fbe0: the special word 10 consults flag; other tag-2 values close.
    pub fn is_closed(self) -> bool {
        if self.word() == 10 {
            self.flag == 0
        } else {
            self.tag == 2
        }
    }

    /// B 0x14093fbc0. This is the complement of is_closed for valid flags.
    pub fn is_open(self) -> bool {
        if self.word() == 10 {
            self.flag != 0
        } else {
            self.tag != 2
        }
    }

    /// B 0x14093fc00 uses the entire word for unit variants 3 and 4.
    pub fn cast_ui_shadows(self) -> bool {
        matches!(self.word(), 3 | 4) || self.tag == 7 && self.flag == 1
    }

    /// Inlined into B time_update, 0x140922cba..0x140922cf7.
    pub fn allow_time_passage(self) -> bool {
        self.is_closed() || self.tag == 7 && self.flag == 1
    }

    /// B 0x14093fc20. The comparison and fallback use the complete word.
    pub fn should_show_world(self) -> bool {
        let index = self.word().wrapping_sub(2);
        let index = if index < 9 { index as u32 } else { 4 };
        if (0xaf_u32 >> index) & 1 != 0 {
            true
        } else if (0x50_u32 >> index) & 1 != 0 {
            false
        } else {
            self.flag == 0
        }
    }

    /// These three original symbols share B 0x14093fc60.
    pub fn can_limit_frame_rate(self) -> bool {
        self.tag != 7 || self.flag != 1
    }

    pub fn can_use_static_world_capture(self) -> bool {
        self.can_limit_frame_rate()
    }

    pub fn allow_painterly_overlay(self) -> bool {
        self.can_limit_frame_rate()
    }
}

/// Shared by the load menu and LUT gallery. Field names describe observed roles;
/// the original Rust field spellings are not present in the B PDB.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LoadMenuThumbnailMetrics {
    /// B +0: `Option<u32>`, copied from the scroll widget's hovered index.
    pub highlighted_index: Option<u32>,
    /// B +8: first cell center, in normalized screen coordinates.
    pub origin: Vec2,
    /// B +0x10: thumbnail extent, consumed by the draw uniforms.
    pub cell_size: Vec2,
    /// B +0x18: distance between adjacent cell centers, including the gap.
    pub step: Vec2,
    /// B +0x20: scroll displacement measured in grid cells, not screen units.
    pub scroll_offset: Vec2,
    pub cell_count: u32,
    pub cells_per_line: u32,
    /// B +0x30: true fills a column before advancing to the next column.
    pub column_major: bool,
}

impl LoadMenuThumbnailMetrics {
    /// B 0x14093fc70 and its inlined map at 0x141304fb0. Collection is eager:
    /// zero cells do not divide, but nonempty grids with zero stride panic here.
    pub fn positions(&self) -> std::vec::IntoIter<Vec2> {
        (0..self.cell_count)
            .map(|i| {
                let (x, y) = if self.column_major {
                    (i / self.cells_per_line, i % self.cells_per_line)
                } else {
                    (i % self.cells_per_line, i / self.cells_per_line)
                };
                Vec2::new(
                    thumbnail_coordinate(x, self.step.x, self.origin.x),
                    thumbnail_coordinate(y, self.step.y, self.origin.y),
                )
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

fn thumbnail_coordinate(index: u32, step: f32, origin: f32) -> f32 {
    // B uses separate MULSS/ADDSS, with destination-first NaN propagation.
    // Preserve its quiet payloads and negative indefinite NaN on non-x86 hosts.
    if step.is_nan() {
        return f32::from_bits(step.to_bits() | 0x0040_0000);
    }
    if index == 0 && step.is_infinite() {
        return f32::from_bits(0xffc0_0000);
    }
    let offset = index as f32 * step;
    if origin.is_nan() {
        return f32::from_bits(origin.to_bits() | 0x0040_0000);
    }
    let result = offset + origin;
    if result.is_nan() {
        f32::from_bits(0xffc0_0000)
    } else {
        result
    }
}

/// Recovered descriptive variant names; no serialization contract is implied.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, enum_iterator::Sequence)]
pub enum DebugTriggerProxyVis {
    ShowAll,
    OnlyMeshes,
    RoofsWithUvs,
    MeshesAndNormals,
    ClutterCollisions,
}

impl DebugTriggerProxyVis {
    /// B 0x14093ffd0 returns an owned String, not a borrowed label.
    pub fn as_string(self) -> String {
        match self {
            Self::ShowAll => "Show All",
            Self::OnlyMeshes => "Only Meshes",
            Self::RoofsWithUvs => "Roofs with UVs",
            Self::MeshesAndNormals => "Meshes + Normals",
            Self::ClutterCollisions => "Clutter Collisions",
        }
        .to_owned()
    }
}

/// Recovered descriptive variant names, with the original numeric order.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, enum_iterator::Sequence)]
pub enum ShowDebugRaycastGeo {
    No,
    Walls,
    Roofs,
    DecoratorCollisions,
    DecoratorInteractions,
}

impl ShowDebugRaycastGeo {
    /// B 0x140940170 returns a static str through two lookup tables.
    pub fn as_string(self) -> &'static str {
        match self {
            Self::No => "No",
            Self::Walls => "Walls",
            Self::Roofs => "Roofs",
            Self::DecoratorCollisions => "Decorator collisions",
            Self::DecoratorInteractions => "Decorator interactions",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScreenLogEntry {
    pub message: String,
    pub color: egui::Color32,
    pub remaining_seconds: f32,
}

/// B has five Option entries and a wrapping insertion index, not a growing log.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScreenLog {
    pub entries: [Option<ScreenLogEntry>; 5],
    pub next: usize,
}

impl ScreenLog {
    /// B gui_error_log closure 0x14015e1c0 visits physical slots, renders first,
    /// then ages the entry. Zero and NaN timers survive the strict comparison.
    pub fn show(&mut self, ui: &mut egui::Ui, delta_seconds: f32) {
        for slot in &mut self.entries {
            if let Some(entry) = slot {
                ui.colored_label(entry.color, entry.message.clone());
                entry.remaining_seconds -= delta_seconds;
                if entry.remaining_seconds < 0.0 {
                    *slot = None;
                }
            }
        }
    }
}

/// Recovered log owner and the draw consumer's suppression field.
/// Remaining debug controls/hash table are tracked separately until identified.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct DebugGui {
    pub suppress: bool,
    pub show_screen_log: bool,
    pub screen_log: ScreenLog,
}

impl Default for DebugGui {
    fn default() -> Self {
        Self {
            suppress: false,
            show_screen_log: true,
            screen_log: ScreenLog::default(),
        }
    }
}

impl DebugGui {
    /// Core-hosted B gui_error_log policy. Hidden/context-less frames do not
    /// decrement timers; GameTime's delta is supplied by the schedule caller.
    pub fn show_screen_log(&mut self, context: Option<&EguiContextResource>, delta_seconds: f32) {
        if !self.show_screen_log || !self.screen_log.entries.iter().any(Option::is_some) {
            return;
        }
        let Some(context) = context else {
            return;
        };
        egui::Window::new(" ")
            .title_bar(false)
            .frame(egui::Frame::none())
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-20.0, 0.0))
            .show(context, |ui| self.screen_log.show(ui, delta_seconds));
    }

    /// B 0x14093fe50 copies even an empty message and unconditionally replaces
    /// the next slot. No severity filter or global debug-enable guard is present.
    pub fn log_to_screen(&mut self, message: &str, color: egui::Color32) {
        let entry = ScreenLogEntry {
            message: message.to_owned(),
            color,
            remaining_seconds: 5.0,
        };
        self.screen_log.entries[self.screen_log.next] = Some(entry);
        self.screen_log.next = (self.screen_log.next + 1) % 5;
    }
}
