//! Owned GPU inputs and identifiers; device enumeration belongs to the caller.
use super::GraphicsInitError;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Vendor classification supplied by the device enumerator; it selects default support links.
pub enum Vendor {
    Nvidia,
    Intel,
    Other,
    Amd,
    Qualcomm,
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Driver assessment supplied by the caller. This crate displays it without probing the driver.
pub enum DriverSupport {
    Unsupported { minimum: String },
    UnsupportedLegacyIntel { minimum: String },
    Emulated,
    UnknownVendor(u32),
    KnownBad { known_good: String },
    Supported,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// One candidate in a stable input list, including display data and any initialization failure.
pub struct Device {
    pub name: String,
    pub vendor: Vendor,
    pub driver_version: String,
    pub driver_support: DriverSupport,
    pub init_error: Option<GraphicsInitError>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Why the caller is asking the player to choose a device; affects the heading only.
pub enum SelectionReason {
    Requested,
    PrimaryUnsupported,
    WhichOne,
    Changed,
    UnavailableDetected,
    GraphicsInitFailed,
}
impl SelectionReason {
    /// Localization key for the selection heading; this does not perform a locale lookup.
    pub fn key(self) -> &'static str {
        match self {
            Self::Requested => "Driver.Header.Reason.Requested",
            Self::PrimaryUnsupported => "Driver.Header.Reason.PrimaryUnsupported",
            Self::WhichOne => "Driver.Header.Reason.WhichOne",
            Self::Changed => "Driver.Header.Reason.Changed",
            Self::UnavailableDetected => "Driver.Header.Reason.UnavailableDetected",
            Self::GraphicsInitFailed => "GraphicsInitError.Prelude",
        }
    }
}

/// Index in the caller-provided candidate list; checked again when selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceId(usize);
impl DeviceId {
    /// Wrap a candidate-list index. Existence and availability are checked when the message is handled.
    pub const fn new(index: usize) -> Self {
        Self(index)
    }
    /// Recover the caller-provided list index; this value alone is not proof that a device exists.
    pub const fn index(self) -> usize {
        self.0
    }
}
impl Device {
    /// Driver warnings may be overridden; an initialization failure may not.
    pub fn is_selectable(&self) -> bool {
        self.init_error.is_none()
    }
}
