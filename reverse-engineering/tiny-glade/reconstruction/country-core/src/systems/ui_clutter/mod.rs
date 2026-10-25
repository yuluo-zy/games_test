//! `systems/ui_clutter` — the auto-clutter / deco-clutter UI family.
//!
//! Source: B skeleton `crates/country-core/src/systems/ui_clutter/*`.
//! PDB named entity `DeletedAutoClutter` (event; layout U).

pub mod debug_spawn_clutter;
pub mod decoclutter_show_pins;
pub mod defered_autoclutter_history;
pub mod rmb_autoclutter;
pub mod ui_autoclutter;
pub mod ui_focus_decoclutter;

use crate::Gap;

/// `DeletedAutoClutter` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct DeletedAutoClutter {
    pub _gap: Gap,
}
