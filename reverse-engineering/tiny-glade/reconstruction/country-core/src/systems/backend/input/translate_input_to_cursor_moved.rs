//! `backend/input/translate_input_to_cursor_moved.rs` — replay path: convert
//! recorded mouse-move records into `InputCursorMoved` events with accumulated
//! deltas.
//!
//! PDB: `translate_input_to_cursor_moved` @0x140b17a80 (635 B). Named entities
//! `InputCursorDelta`, `InputCursorMoved`.
//!
//! O: walks the recorded event deques (stride 0x18; `tag @+8`: 0 = mouse
//! moved, 1 = moved, 2 = skip), accumulates the cursor delta (packed Vec2
//! @+0xc) with NaN-safe clamp (maxps/minps on the moved tag), then pushes an
//! `InputCursorMoved {tag, delta}` event (stride 0x10) and adds the delta into
//! the cursor-state accumulators @+0x58 (delta) and @+0x60 (position delta).

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;
use glam::Vec2;

use crate::input::InputCursorMoved;
use crate::input::input_stream::*;
use crate::Gap;

/// `InputCursorDelta` (O named entity).
#[derive(Clone, Copy, Debug, Default)]
pub struct InputCursorDelta {
    pub _gap: Gap,
}

/// `translate_input_to_cursor_moved` @0x140b17a80 (O).
pub fn translate_input_to_cursor_moved(
    mut _recorded: Res<Gap>,
    mut _out: EventWriter<InputCursorMoved>,
    mut _cursor: ResMut<Gap>,
) {
    // Recovered (O): per-record tag dispatch + delta accumulation + clamp;
    // emits InputCursorMoved and updates +0x58/+0x60 accumulators.
    todo!("recover translate_input_to_cursor_moved (0x18-strided records, NaN-safe clamp)")
}
