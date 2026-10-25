//! `startup/startup_session.rs` — session boot/load (`load_game_session`).
//!
//! Source: B skeleton `crates/country-core/src/startup/startup_session.rs`.
//! O evidence (Ghidra, B patched EXE; `load_game_session` @0x14166e7d0):
//! - removes `resources::session::NextSession` (a queued session change),
//! - evaluates the next session (`SessionLoadAttempt::from_session_id`,
//!   a `VecDeque<SessionLoadAttempt>`), gated by
//!   `systems::save::load_completion_checker::is_session_ok_to_load`,
//! - on success emits `tiny_glade::app_config::WriteSettingsFileCmd` and uses
//!   `resources::session::SessionId::save_folder_path().join(...)`,
//! - `copy_dir_all` @0x141740c00 copies the session save folder.
//! - `load_session_meta` (O inline), `MaybeExists` (O: get helper).

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `SessionLoadAttempt` (O named; queued session-load attempt state).
#[derive(Clone, Debug, Default)]
pub struct SessionLoadAttempt(pub Gap);

impl SessionLoadAttempt {
    /// `SessionLoadAttempt::from_session_id` (O inline).
    pub fn from_session_id(_id: &Gap) -> Self {
        SessionLoadAttempt(Gap)
    }
}

/// `MaybeExists` (O: `get` helper for optional file/dir).
#[derive(Clone, Copy, Debug, Default)]
pub struct MaybeExists;

/// `load_game_session` @0x14166e7d0 (O).
pub fn load_game_session(_session: &mut Gap, _commands: &mut Gap) {
    todo!("recover load_game_session (NextSession -> is_session_ok_to_load -> WriteSettingsFileCmd)")
}

/// `copy_dir_all` @0x141740c00 (O).
pub fn copy_dir_all(_src: &std::path::Path, _dst: &std::path::Path) -> std::io::Result<()> {
    todo!("recover copy_dir_all")
}

/// `load_session_meta` (O inline).
pub fn load_session_meta(_path: &std::path::Path) {
    todo!("recover load_session_meta")
}
