//! B GUI state and menu policies. Native tags are exposed only through the view;
//! owned Rust enums preserve behavior without promising the original compiler ABI.
use std::{collections::HashMap, sync::Arc};

use bevy_ecs::resource::Resource;
use bevy_ecs::{event::Event, event::EventWriter, system::ResMut};
use glam::Vec2;
use parking_lot::Mutex;

use super::session::SessionId;
use crate::systems::camera_rig::main_camera::CameraMode;

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

impl PhotomodeScreen {
    /// The transition tested by B do_photomode before the frame-end copy.
    pub fn just_enabled(&self) -> bool {
        self.enabled && !self.was_enabled
    }

    /// B 0x14234c104. Call after consumers have inspected the transition.
    pub fn finish_frame(&mut self) {
        self.was_enabled = self.enabled;
    }
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
    /// B 0x14093fcd0 machine: `cmp byte [rcx],0; setne al; ret` — returns
    /// `tag != 0`, i.e. any submenu other than the top-level Menu.
    pub fn is_in_submenu(self) -> bool {
        !matches!(self, Self::Menu)
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

/// Per-TOD-entry editor state. B 0x1400bff20 inserts one false byte under the
/// entry's TempUiId; 0x140144590 toggles it with the `...` button. It is not a
/// deletion-confirmation flag: deletion uses egui's double-click response.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TodSnapshotGuiState {
    pub expanded: bool,
}

impl TodSnapshotGuiState {
    pub fn show_toggle(&mut self, ui: &mut egui::Ui) {
        if ui.button("...").clicked() {
            self.expanded = !self.expanded;
        }
    }
}

/// Owned controls whose meaning is established by the original widgets. Field
/// spellings are descriptive; the offset comments refer to B, not this layout.
/// The three flags at B +0xfb/+0xfc/+0xff and the snapshot request at +0 remain
/// outside this semantic model until their consumers have been identified.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct DebugGui {
    /// B +0xf0, toggled by the debug-window input action.
    pub enabled: bool,
    /// B +0xf1.
    pub show_spatial_grid: bool,
    /// B +0xf2.
    pub show_trigger_proxies: bool,
    /// B +0xf3.
    pub show_threshold_curve: bool,
    /// B +0xf4.
    pub show_lazy_grid: bool,
    /// B +0xf5.
    pub show_sheep_controls: bool,
    /// B +0xf6.
    pub show_music_info: bool,
    /// B +0xf7, labelled `Show 2D UI`; also gates the screen-log window.
    pub show_2d_ui: bool,
    /// B +0xf8.
    pub human_scale_reference: bool,
    /// B +0xf9.
    pub disable_terrain_hover_highlight: bool,
    /// B +0xfa.
    pub hide_brush_previews: bool,
    /// B +0xfd.
    pub disable_wall_snapping: bool,
    /// B +0xfe, labelled `Disable eraser higlight` in the original UI.
    pub suppress: bool,
    /// B +0x100.
    pub force_music_richness: bool,
    /// B +0x101, the independently toggled experimental window.
    pub experimental_window: bool,
    /// B +0x102.
    pub trigger_proxy_visualization: DebugTriggerProxyVis,
    /// B +0x103. This is CameraMode, not PhotoModeCameraMode; photomode's
    /// conversion writes its tag plus one here and writes Normal on exit.
    pub camera_mode: CameraMode,
    /// B +0x104.
    pub show_raycast_geometry: ShowDebugRaycastGeo,
    /// B +0xe8, edited only for MeshesAndNormals (tag 3).
    pub normal_length: f32,
    /// B +0xec, used by camera_snapshot as `GameTime.delta * speed`.
    pub camera_snapshot_interpolation_speed: f32,
    /// B +0xb8: original TempUiId keys are u64. The owning TOD domain supplies
    /// IDs; this map stores only GUI expansion state and never allocates IDs.
    pub tod_snapshot_states: HashMap<u64, TodSnapshotGuiState>,
    pub screen_log: ScreenLog,
}

impl Default for DebugGui {
    fn default() -> Self {
        Self {
            enabled: false,
            show_spatial_grid: false,
            show_trigger_proxies: false,
            show_threshold_curve: false,
            show_lazy_grid: false,
            show_sheep_controls: false,
            show_music_info: false,
            show_2d_ui: true,
            human_scale_reference: false,
            disable_terrain_hover_highlight: false,
            hide_brush_previews: false,
            disable_wall_snapping: false,
            suppress: false,
            force_music_richness: false,
            experimental_window: false,
            trigger_proxy_visualization: DebugTriggerProxyVis::ShowAll,
            camera_mode: CameraMode::NORMAL,
            show_raycast_geometry: ShowDebugRaycastGeo::No,
            normal_length: 1.0,
            camera_snapshot_interpolation_speed: 1.5,
            tod_snapshot_states: HashMap::new(),
            screen_log: ScreenLog::default(),
        }
    }
}

impl DebugGui {
    /// B 0x1400bff20: existing state survives repeated visits. No retention or
    /// cleanup policy is added when the source TOD entry is removed.
    pub fn tod_snapshot_state(&mut self, id: u64) -> &mut TodSnapshotGuiState {
        self.tod_snapshot_states.entry(id).or_default()
    }

    /// Resource-owned part of the original debug panels. GPU, audio and world
    /// controls remain owned by their respective consumers.
    pub fn show_controls(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Mode")
            .selected_text(camera_mode_label(self.camera_mode))
            .show_ui(ui, |ui| {
                for mode in [
                    CameraMode::NORMAL,
                    CameraMode::TILT_SHIFT,
                    CameraMode::FREE,
                    CameraMode::WALK,
                    CameraMode::TOP_DOWN,
                    CameraMode::BUTTERFLY,
                ] {
                    ui.selectable_value(&mut self.camera_mode, mode, camera_mode_label(mode));
                }
            });
        ui.add(
            egui::Slider::new(&mut self.camera_snapshot_interpolation_speed, 0.0..=10.0)
                .text("Interpolation Speed"),
        );
        ui.checkbox(&mut self.show_2d_ui, "Show 2D UI");
        ui.checkbox(&mut self.show_sheep_controls, "Show sheep controls");
        ui.checkbox(&mut self.show_music_info, "Show music info");
        ui.checkbox(
            &mut self.force_music_richness,
            "Force Music Richness to 1.0",
        );
        ui.checkbox(
            &mut self.disable_terrain_hover_highlight,
            "Disable terrain hover highlight",
        );
        ui.checkbox(&mut self.disable_wall_snapping, "Disable wall snapping");
        ui.checkbox(&mut self.suppress, "Disable eraser higlight");
        ui.checkbox(&mut self.hide_brush_previews, "Hide brush previews");
        ui.checkbox(
            &mut self.human_scale_reference,
            "Enable human scale ref (R)",
        );
        ui.checkbox(&mut self.show_threshold_curve, "Show debug threshold curve");
        ui.checkbox(&mut self.show_spatial_grid, "Show debug spatial grid");
        egui::ComboBox::from_label("Show debug raycast geo")
            .selected_text(self.show_raycast_geometry.as_string())
            .show_ui(ui, |ui| {
                for value in enum_iterator::all::<ShowDebugRaycastGeo>() {
                    ui.selectable_value(&mut self.show_raycast_geometry, value, value.as_string());
                }
            });
        ui.checkbox(&mut self.show_trigger_proxies, "Show debug trigger proxies");
        ui.checkbox(&mut self.show_lazy_grid, "Show lazy grid");
        if self.show_trigger_proxies {
            egui::ComboBox::from_label("Viz presets")
                .selected_text(self.trigger_proxy_visualization.as_string())
                .show_ui(ui, |ui| {
                    for value in enum_iterator::all::<DebugTriggerProxyVis>() {
                        ui.selectable_value(
                            &mut self.trigger_proxy_visualization,
                            value,
                            value.as_string(),
                        );
                    }
                });
            if self.trigger_proxy_visualization == DebugTriggerProxyVis::MeshesAndNormals {
                ui.add(egui::Slider::new(&mut self.normal_length, 0.2..=4.0).text("Normal length"));
            }
        }
    }

    /// Core-hosted B gui_error_log policy. Hidden/context-less frames do not
    /// decrement timers; GameTime's delta is supplied by the schedule caller.
    pub fn show_screen_log(&mut self, context: Option<&EguiContextResource>, delta_seconds: f32) {
        if !self.show_2d_ui || !self.screen_log.entries.iter().any(Option::is_some) {
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

/// B CameraMode fmt (0x1400c48b0), length/relative-pointer tables
/// 0x142935590/0x1429355c0. The six selectable debug modes are the base modes.
fn camera_mode_label(mode: CameraMode) -> &'static str {
    match mode {
        CameraMode::NORMAL => "Normal",
        CameraMode::TILT_SHIFT => "TiltShift",
        CameraMode::FREE => "Free",
        CameraMode::WALK => "Walk",
        CameraMode::TOP_DOWN => "TopDown",
        CameraMode::BUTTERFLY => "Butterfly",
        _ => "Unresolved camera mode",
    }
}

#[cfg(test)]
mod recovery_tests {
    use super::*;

    #[test]
    fn debug_owner_defaults_and_persistent_per_entry_state_match_b() {
        let mut gui = DebugGui::default();
        assert!(gui.show_2d_ui);
        assert!(!gui.enabled);
        assert_eq!(gui.normal_length.to_bits(), 0x3f80_0000);
        assert_eq!(
            gui.camera_snapshot_interpolation_speed.to_bits(),
            0x3fc0_0000
        );
        assert_eq!(gui.camera_mode, CameraMode::NORMAL);
        gui.tod_snapshot_state(19).expanded = true;
        assert!(gui.tod_snapshot_state(19).expanded);
        assert!(!gui.tod_snapshot_state(20).expanded);
    }

    #[test]
    fn screen_log_overwrites_physical_slots_and_expires_strictly_below_zero() {
        let mut gui = DebugGui::default();
        for i in 0..6 {
            gui.log_to_screen(&i.to_string(), egui::Color32::RED);
        }
        assert_eq!(gui.screen_log.next, 1);
        assert_eq!(gui.screen_log.entries[0].as_ref().unwrap().message, "5");
        assert_eq!(gui.screen_log.entries[1].as_ref().unwrap().message, "1");
        let context = egui::Context::default();
        let _ = context.run(egui::RawInput::default(), |context| {
            egui::CentralPanel::default().show(context, |ui| gui.screen_log.show(ui, 5.0));
        });
        assert!(gui.screen_log.entries.iter().all(Option::is_some));
        let _ = context.run(egui::RawInput::default(), |context| {
            egui::CentralPanel::default().show(context, |ui| gui.screen_log.show(ui, f32::EPSILON));
        });
        assert!(gui.screen_log.entries.iter().all(Option::is_none));
    }
}

/// Shared GUI and time-of-day state. B `do_time_of_day_screen` @1405cc7f0
/// writes the open edge at +08, remembers visibility at +09, and uses +0a
/// for visibility. Its circle callback @140505780 publishes targets at +00/+04
/// and input flags at +0b/+0c; the actual TOD system consumes those same slots.
/// Startup clears all thirteen value bytes; Bevy registers size 16, alignment 4.
#[repr(C)]
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct TimeOfDayScreen {
    pub time_of_day: f32,
    pub sun_xz_rotation: f32,
    pub just_opened: bool,
    pub was_visible: bool,
    pub visible: bool,
    pub time_input_active: bool,
    pub angle_input_active: bool,
}
const _: () = assert!(core::mem::size_of::<TimeOfDayScreen>() == 16);
const _: () = assert!(core::mem::align_of::<TimeOfDayScreen>() == 4);
const _: () = assert!(core::mem::offset_of!(TimeOfDayScreen, just_opened) == 8);
const _: () = assert!(core::mem::offset_of!(TimeOfDayScreen, was_visible) == 9);
const _: () = assert!(core::mem::offset_of!(TimeOfDayScreen, visible) == 10);
const _: () = assert!(core::mem::offset_of!(TimeOfDayScreen, time_input_active) == 11);
const _: () = assert!(core::mem::offset_of!(TimeOfDayScreen, angle_input_active) == 12);
