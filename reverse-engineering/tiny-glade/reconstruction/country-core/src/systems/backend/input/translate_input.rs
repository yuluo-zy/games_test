//! `backend/input/translate_input.rs` — the main input-mapping system: raw
//! mouse/analog/digital input → `ActionSetStates` / `ActionSetValues`.
//!
//! PDB: `translate_input` @0x1417cd0a0 (4769 B). Named entity `MouseMoved`.
//!
//! O: consumes mouse-move delta events into the analog action values and
//! updates digital action states (bevy `ButtonInput` + controller axes);
//! `MouseMoved` is the cursor-delta event produced by the replay path.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::input::action_set::{ActionSetStates, ActionSetValues};
use crate::input::MouseMoved;
use crate::Gap;

/// `translate_input` @0x1417cd0a0 (O).
pub fn translate_input(
    mut _mouse_moved: EventReader<MouseMoved>,
    mut _states: ResMut<ActionSetStates>,
    mut _values: ResMut<ActionSetValues>,
    _controller_input: Res<Gap>,
) {
    // Recovered (O): raw mouse deltas + controller axes feed the per-action
    // analog values; digital states fold bevy ButtonInput / button mapping.
    todo!("recover translate_input analog+digital mapping")
}
