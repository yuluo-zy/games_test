//! `ui_clutter/decoclutter_show_pins.rs` — show/hide pinned deco-clutter pins.
//!
//! PDB: `decoclutter_show_pins @0x1412357c0` (6719 B),
//! `register_required_components @0x141237200`, `clone_behavior @0x140002370`.
//! Named entities `DecoClutterLock`, `ShowPinnedDecoClutter`.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::Gap;

/// `DecoClutterLock` (O named entity; component).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct DecoClutterLock {
    pub _gap: Gap,
}

/// `ShowPinnedDecoClutter` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct ShowPinnedDecoClutter {
    pub _gap: Gap,
}

/// `decoclutter_show_pins` @0x1412357c0 (O).
pub fn decoclutter_show_pins(
    mut _locks: Query<(&DecoClutterLock, &mut Gap)>,
    mut _show: EventReader<ShowPinnedDecoClutter>,
    _commands: Commands,
) {
    todo!("recover decoclutter_show_pins (pin visibility)")
}
