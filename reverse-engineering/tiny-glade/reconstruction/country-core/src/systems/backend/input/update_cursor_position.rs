//! `backend/input/update_cursor_position.rs` — merge `InputCursorMoved`
//! events into the cursor position resource.
//!
//! PDB: **no standalone procedure** — fully inlined (business logic observed
//! through the callers in `translate_input` / `translate_input_to_cursor_moved`).
//! O: the cursor position resource keeps a head `Vec2` position and the
//! per-frame accumulated delta; this system folds the event stream into it.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;
use glam::Vec2;

use crate::input::InputCursorMoved;
use crate::Gap;

/// Cursor-position resource (O shape; exact layout U).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct CursorPosition {
    pub position: Vec2,
    pub _gap: Gap,
}

/// `update_cursor_position` (O; inlined — no PDB procedure).
pub fn update_cursor_position(
    mut _moved: EventReader<InputCursorMoved>,
    mut _cursor: ResMut<CursorPosition>,
) {
    todo!("recover update_cursor_position (fold InputCursorMoved deltas into position)")
}
