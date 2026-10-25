//! `tiny_glade::crash_pickup::crash_payload` — crash-pickup wire payloads.
//!
//! Field names and types are recovered from the B serializers (public symbols):
//! * `Serialize for RecoveredSave` `0x14003eb80`:
//!   keys `temp_save_path` (`0x14292c48d`) and `session` (`0x14292c49b`);
//! * `Serialize for RecoveredSession` `0x14003ec80`:
//!   keys `session`, `folder`, `backup_folder` (`0x14292c49b`…`0x14292c4de`);
//! * `Serialize for BacktracePayload` `0x14003e250`:
//!   keys `backtrace` (`0x14292c328`) and `message` (`0x14292c331`), with types
//!   `backtrace::Backtrace` and `String`;
//! * `Serialize for CrashRecovered` `0x14003e7c0`:
//!   `breadcrumbs`, `report_folder`, `screenshot`, `save`, `inputs`, `log`,
//!   `profile`, `settings`, `gpu_crash_dump`, `rhapsody_log`
//!   (`0x14292c386`…`0x14292c3d2`), where every path is
//!   `Result<PathBuf, String>` and `save` is
//!   `Result<RecoveredSave, SaveRecoveryError>`;
//! * `Serialize for SaveRecoveryError` `0x14003e350`: variants
//!   `GameNotYetInitialized` and `Other` (`0x14292c338`/`0x14292c34d`);
//! * `Serialize for CrashPickupMessage` `0x14003eda0`: variants `Panic` and
//!   `recovered`, with field `payload` (`0x14292c520`…`0x14292c52c`).
//!
//! Runtime-only types that are not modelled here (evidence: `drop_in_place`
//! symbols `0x1402e5d80`, `0x1405fff90`, `0x1405d0980`, `0x1402ad6c0` and
//! `server::run` `0x1402aee80`):
//! * `CrashMiscInfo` — 8-byte payload inside `Arc<Mutex<…>>`; two `u8` fields
//!   are initialised to `2` at `+0x11/+0x12` (field names not recovered).
//! * `ResolvedCrash` — `CrashRecovered` (0x1e8) + `Option<CrashPayload>` at
//!   `+0x1e8`, total 0x228.
//! * `CrashPayload` — runtime enum; one variant is `BacktracePayload`, the other
//!   holds the received `minidumper::MinidumpBinary` (file handle at `+0x38`).
//! These are SDK-side transport state; the crash-pickup server keeps them behind
//! its event-loop seam.

use country_core::resources::session::ActiveSession;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Recovered save slot.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecoveredSave {
    pub temp_save_path: PathBuf,
    pub session: Option<RecoveredSession>,
}

/// Session snapshot carried inside [`RecoveredSave`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecoveredSession {
    pub session: ActiveSession,
    pub folder: PathBuf,
    pub backup_folder: PathBuf,
}

/// Backtrace payload from the panic hook.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BacktracePayload {
    pub backtrace: backtrace::Backtrace,
    pub message: String,
}

impl BacktracePayload {
    /// Build a payload containing only a panic message.
    pub fn message(message: String) -> Self {
        Self {
            backtrace: backtrace::Backtrace::new(),
            message,
        }
    }
}

/// Full crash report bundle sent by the crash server.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrashRecovered {
    pub breadcrumbs: Vec<sentry_types::protocol::v7::Breadcrumb>,
    pub report_folder: Result<PathBuf, String>,
    pub screenshot: Result<PathBuf, String>,
    pub save: Result<RecoveredSave, SaveRecoveryError>,
    pub inputs: Result<PathBuf, String>,
    pub log: Result<PathBuf, String>,
    pub profile: Result<PathBuf, String>,
    pub settings: Result<PathBuf, String>,
    pub gpu_crash_dump: Result<PathBuf, String>,
    pub rhapsody_log: Result<PathBuf, String>,
}

/// Save recovery failure.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SaveRecoveryError {
    GameNotYetInitialized,
    Other(String),
}

impl std::fmt::Display for SaveRecoveryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GameNotYetInitialized => formatter.write_str("Game not yet initialized"),
            Self::Other(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for SaveRecoveryError {}

/// Message sent over the crash pickup IPC channel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CrashPickupMessage {
    Panic { payload: BacktracePayload },
    #[serde(rename = "recovered")]
    Recovered(CrashRecovered),
}
