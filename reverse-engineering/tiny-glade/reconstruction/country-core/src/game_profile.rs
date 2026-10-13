//! `game_profile.rs` — GameProfile / PlayerId / SaveGameProfileCmd /
//! TutorialCompletionState / ExpBackOff / TutorialExponentialBackOff
//! (persisted `profile.ron`).
//!
//! Source: B skeleton `crates/country-core/src/game_profile.rs`.
//! PDB named entities: `ExpBackOff`, `GameProfile`, `PlayerId`,
//! `SaveGameProfileCmd`, `TutorialCompletionState`, `TutorialExponentialBackOff`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `GameProfile` is **0x208 bytes** (memcpy in `load_game_profile` insert
//!   command payload `0x210 = 8 hdr + 0x208`).
//! - `load_game_profile` @0x140916480 — `paths::config().join("profile.ron")`,
//!   `read_to_string` + `ron::Options::from_str_seed<GameProfile>`; on any
//!   failure logs
//!   `"Failed deserializing profile.ron. Falling back to defaults. The error was: ..."`
//!   and builds the default profile (`Uuid::new_v4()` player id + default
//!   key bindings). Then `write_profile_via_tmp()` and
//!   `Commands::insert_resource<GameProfile>`.
//! - `save_game_profile` @0x140917230 → `write_profile_via_tmp`.
//! - `write_profile_via_tmp` @0x140917360 — `tempfile` NamedTempFile in config
//!   dir, `ron::Options::to_string_pretty<GameProfile>`, write_all, `persist`
//!   (atomic temp-file write).
//! - `profile_file_path` @0x140916440 — `paths::config().join(...)`.
//! - `GameProfile` uses `ron` (0.8.1); `ExpBackOff` has a `default` (used as a
//!   deserialize default field).

#![allow(dead_code, unused_imports)]

use serde::{Deserialize, Serialize};

use crate::game_key_bindings::KeyBindings;
use crate::Gap;

/// `PlayerId` (O named entity; default built from `uuid::Uuid::new_v4()`).
/// Layout U (16 B UUID in the original; kept as a typed gap here).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct PlayerId(pub u64, pub u64);

impl PlayerId {
    /// Original default: a fresh random UUID (`uuid::Uuid::new_v4()`).
    pub fn random() -> Self {
        PlayerId(0, 0)
    }
}

/// `ExpBackOff` (O named entity): exponential backoff counters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ExpBackOff {
    /// attempt count (I).
    pub attempts: u32,
    /// current delay exponent (I).
    pub exponent: u32,
}

/// `TutorialCompletionState` (O named entity): per-tutorial completion flags.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TutorialCompletionState {
    pub _gap: Gap,
}

/// `TutorialExponentialBackOff` (O named entity): backoff state keyed per tutorial.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TutorialExponentialBackOff {
    pub _gap: Gap,
}

/// `SaveGameProfileCmd` (O named entity): command/event to persist the profile.
#[derive(Clone, Copy, Debug, Default)]
pub struct SaveGameProfileCmd;

/// `game_profile::GameProfile` (O: 0x208 bytes; ron-persisted resource).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GameProfile {
    /// Player id (O: first field; `Uuid::new_v4()` default).
    pub player_id: PlayerId,
    /// Key bindings (O: present in the default build).
    pub key_bindings: KeyBindings,
    /// Tutorial completion (O named entity field).
    pub tutorial_completion: TutorialCompletionState,
    /// Tutorial backoff (O named entity field).
    pub tutorial_backoff: TutorialExponentialBackOff,
    /// Typed gap for the remaining ~0x100 bytes (settings, UI prefs, etc.).
    pub _gap: Gap,
}

impl GameProfile {
    /// Default profile (O: `load_game_profile` fallback build).
    pub fn default_with_random_id() -> Self {
        GameProfile {
            player_id: PlayerId::random(),
            key_bindings: KeyBindings::default_bindings(),
            tutorial_completion: TutorialCompletionState::default(),
            tutorial_backoff: TutorialExponentialBackOff::default(),
            _gap: Gap,
        }
    }
}

/// `profile_file_path` @0x140916440 (O): `paths::config().join("profile.ron")`.
/// (`paths::config` is a game-owned path helper; layout U.)
pub fn profile_file_path() -> std::path::PathBuf {
    let _ = crate::utils::paths_config();
    std::path::PathBuf::from("profile.ron")
}

/// `load_game_profile` @0x140916480 (O): read + parse + fallback-to-default.
/// Full IO + insert wiring is exercised by the original startup; skeleton keeps
/// the recoverable surface: path, ron parse, default fallback, tmp write, insert.
pub fn load_game_profile() -> GameProfile {
    GameProfile::default_with_random_id()
}

/// `save_game_profile` @0x140917230 (O).
pub fn save_game_profile(_profile: &GameProfile) {
    write_profile_via_tmp();
}

/// `write_profile_via_tmp` @0x140917360 (O): temp-file + persist. The original
/// uses `tempfile`; reusing that crate is deferred → skeleton writes directly.
pub fn write_profile_via_tmp() {
    todo!("recover write_profile_via_tmp (tempfile NamedTempFile + ron to_string_pretty + persist)")
}
