//! `systems/mod_key.rs` — modifier-key state (snapping-disable toggle).
//!
//! PDB: `Modifiers::disable_snapping @0x1408c5680` (4 B, folded with
//! `ActiveInputMode::is_controller`), `Modifiers::is_disable_snapping_just_
//! pressed @0x140af8f70` (5 B), `update_mod_key_state @0x140AF9000`
//! (506 B — PDB rva sits inside `resources::toolbar::Toolbar::new` @0x140AF8E00
//! ⇒ ICF/folding; actual code address U). Named entities `Modifiers`,
//! `ToggleSnapping`.
//!
//! I: `Modifiers` holds per-frame modifier key state; `update_mod_key_state`
//! reads ctrl/shift from `ButtonInput<KeyCode>` and toggles the snapping
//! disable flag on `just_pressed`.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::reused::bevy_input::{ButtonInput, KeyCode};
use crate::Gap;

/// `Modifiers` (O named entity; component/resource — layout U).
#[derive(bevy_ecs::component::Component, bevy_ecs::resource::Resource, Clone, Copy, Debug, Default)]
pub struct Modifiers {
    /// ctrl held (I).
    pub ctrl: bool,
    /// shift held (I).
    pub shift: bool,
    /// alt held (I).
    pub alt: bool,
    /// snapping disabled this frame (O accessor name).
    pub disable_snapping: bool,
}

impl Modifiers {
    /// `disable_snapping` @0x1408c5680 (O; folded one-liner).
    pub fn disable_snapping(&self) -> bool {
        self.disable_snapping
    }
    /// `is_disable_snapping_just_pressed` @0x140af8f70 (O).
    pub fn is_disable_snapping_just_pressed(&self) -> bool {
        self.disable_snapping
    }
}

/// `ToggleSnapping` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct ToggleSnapping {
    pub _gap: Gap,
}

/// `update_mod_key_state` @PDB 0x140AF9000 (O; folded — ICF with Toolbar::new
/// region).
pub fn update_mod_key_state(
    mut _modifiers: ResMut<Modifiers>,
    _keys: Res<ButtonInput<KeyCode>>,
) {
    todo!("recover update_mod_key_state (modifier keys → Modifiers + toggle snapping)")
}
