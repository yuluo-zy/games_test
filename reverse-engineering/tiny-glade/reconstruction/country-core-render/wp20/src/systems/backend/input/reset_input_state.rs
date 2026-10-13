//! `backend/input/reset_input_state.rs` — reset the recorded input stream to
//! the last sync point and clear keyboard state.
//!
//! PDB: `reset_input_state` @0x140942b00.
//!
//! O: reorder/zero a region (11 qwords — the `InputTranslationState` area of
//! the InputStream / action-set snapshot), then trim both input VecDeques
//! (frames head @+0x10/count @+0x18, sync head @+0x30/count @+0x38, total
//! @+0x40) down to the last sync point, for the mouse+keyboard and controller
//! sources; finally `bevy_input::ButtonInput::reset_all<KeyCode>()`.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::input::input_stream::*;
use crate::reused::bevy_input::{ButtonInput, KeyCode};
use crate::Gap;

/// `reset_input_state` @0x140942b00 (O).
pub fn reset_input_state(
    mut _state: ResMut<InputRecordReplayState>,
    _kb: ResMut<Gap>,
    _gamepad: ResMut<Gap>,
    mut _keys: ResMut<ButtonInput<KeyCode>>,
) {
    // Recovered (O): reorder+zero translation snapshot; trim the two input
    // deques to the last sync point; ButtonInput::reset_all.
    todo!("recover reset_input_state (InputTranslationState permutation + deque trim + reset_all)")
}
