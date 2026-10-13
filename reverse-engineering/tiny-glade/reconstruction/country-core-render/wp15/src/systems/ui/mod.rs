//! `systems/ui/mod.rs` — the country-core UI surface: `GlobalId`, the
//! `UiSystems` plugin (`add_ui`), `signifier_config_hash`, and the UI module
//! tree (`resources`, `signifiers`, `hover_intersection`, `ui_update`,
//! `debug_visualize_volumes`).
//!
//! Source: B skeleton `crates/country-core/src/systems/ui/mod.rs`.
//! PDB named entity: `GlobalId`.
//! O evidence (Ghidra, B patched EXE):
//! - `UiSystems::add_ui` (App plugin method) @0x140945c60: schedules the UI
//!   systems in `Update` (`chain_ignore_deferred` + `in_set_inner`), inserts
//!   resources `UiState`, `UiNavState` (with a 1e9-ns timeout), and
//!   `UiSignifierStream` (0x170 B), and registers events `PlayUiSound` /
//!   `PlayPhotomodeSound`.
//! - `GlobalId` — `impl$2::eq` / `impl$4::hash`; a u64 identity (`WidgetId` /
//!   `GlobalId` are name-hash based — `WidgetId::from_name` =
//!   `utils::calculate_hash(name)`).
//! - `signifier_config_hash` (O inline into `signfier_mark_for_delete`).

#![allow(dead_code, unused_imports)]

pub mod debug_visualize_volumes;
pub mod hover_intersection;
pub mod resources;
pub mod signifiers;
pub mod ui_update;

use crate::Gap;

/// `systems::ui::GlobalId` (O named entity; name-hash identity, 8 B).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct GlobalId(pub u64);

impl GlobalId {
    /// `WidgetId::from_name` — `utils::calculate_hash(name)` (O).
    pub fn from_name(name: &str) -> GlobalId {
        GlobalId(crate::utils::calculate_hash(name))
    }
}

/// `signifier_config_hash` (O inline into `signfier_mark_for_delete`).
pub fn signifier_config_hash() -> u64 {
    0
}

/// `UiSystems` (O trait/plugin on `bevy_app::App`): `add_ui` @0x140945c60.
pub trait UiSystems {
    fn add_ui(&mut self);
}

/// Skeleton registration surface of `add_ui` (O resource/event set).
pub fn add_ui(_app: &mut Gap) {
    todo!("recover add_ui (UiState/UiNavState/UiSignifierStream resources + PlayUiSound/PlayPhotomodeSound events + Update schedule)")
}
