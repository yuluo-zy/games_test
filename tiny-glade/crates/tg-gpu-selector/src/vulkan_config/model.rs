//! Configuration values passed between caller and editor.
#[derive(Debug, Clone, PartialEq, Eq)]
/// A symbolic layer or vendor entry; equal names in different variants remain independent.
pub enum AllowlistItem {
    Layer(String),
    Vendor(String),
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Caller-owned enablement entries. Editing toggles entries without expanding vendor names into layers.
pub struct Allowlist {
    pub items: Vec<AllowlistItem>,
    /// Opaque caller policy value, preserved unchanged by this editor.
    pub policy_word: u32,
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// Rendering switches returned to the caller; enabling them does not alter a device in this crate.
pub struct Workarounds {
    pub sync_before_each_pass: bool,
    pub sync_before_each_draw: bool,
    pub sync_before_each_dispatch: bool,
    pub serial_recording: bool,
    pub skip_loading_screen: bool,
    pub vendor_crash_breadcrumbs: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// One available layer and its optional grouping label, supplied by external Vulkan enumeration.
pub struct Layer {
    pub name: String,
    pub vendor: Option<String>,
    pub description: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Complete editable value returned on Apply, including the uninterpreted policy word.
pub struct Configuration {
    pub allowlist: Allowlist,
    pub workarounds: Workarounds,
}

/// Stable display order and semantic switch names, independent of numeric message tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Workaround {
    SyncBeforeEachPass,
    SyncBeforeEachDraw,
    SyncBeforeEachDispatch,
    SerialRecording,
    SkipLoadingScreen,
    VendorCrashBreadcrumbs,
}
impl Workaround {
    /// UI order shared by the editor and switch presentation.
    pub const ALL: [Self; 6] = [
        Self::SyncBeforeEachPass,
        Self::SyncBeforeEachDraw,
        Self::SyncBeforeEachDispatch,
        Self::SerialRecording,
        Self::SkipLoadingScreen,
        Self::VendorCrashBreadcrumbs,
    ];
    /// Display text for this switch, independent of its stored boolean value.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SyncBeforeEachPass => "Sync before each render graph pass",
            Self::SyncBeforeEachDraw => "Sync before each draw",
            Self::SyncBeforeEachDispatch => "Sync before each compute shader dispatch",
            Self::SerialRecording => "Serial command buffer recording",
            Self::SkipLoadingScreen => "Skip loading screen",
            Self::VendorCrashBreadcrumbs => {
                "Use vendor-specific crash breadcrumbs (NVIDIA Aftermath)"
            }
        }
    }
}
impl Workarounds {
    /// Read the named switch without exposing numeric field indices.
    pub fn get(&self, workaround: Workaround) -> bool {
        match workaround {
            Workaround::SyncBeforeEachPass => self.sync_before_each_pass,
            Workaround::SyncBeforeEachDraw => self.sync_before_each_draw,
            Workaround::SyncBeforeEachDispatch => self.sync_before_each_dispatch,
            Workaround::SerialRecording => self.serial_recording,
            Workaround::SkipLoadingScreen => self.skip_loading_screen,
            Workaround::VendorCrashBreadcrumbs => self.vendor_crash_breadcrumbs,
        }
    }
    /// Flip exactly one switch; callers persist or apply the resulting value themselves.
    pub fn toggle(&mut self, workaround: Workaround) {
        let field = match workaround {
            Workaround::SyncBeforeEachPass => &mut self.sync_before_each_pass,
            Workaround::SyncBeforeEachDraw => &mut self.sync_before_each_draw,
            Workaround::SyncBeforeEachDispatch => &mut self.sync_before_each_dispatch,
            Workaround::SerialRecording => &mut self.serial_recording,
            Workaround::SkipLoadingScreen => &mut self.skip_loading_screen,
            Workaround::VendorCrashBreadcrumbs => &mut self.vendor_crash_breadcrumbs,
        };
        *field = !*field;
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Layers sharing an optional vendor, preserving first-seen group order and input order within the group.
pub struct LayerGroup {
    pub vendor: Option<String>,
    pub layers: Vec<Layer>,
}
