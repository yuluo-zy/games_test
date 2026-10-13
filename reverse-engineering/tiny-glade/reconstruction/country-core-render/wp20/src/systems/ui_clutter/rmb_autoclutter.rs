//! `ui_clutter/rmb_autoclutter.rs` — right-click auto-clutter removal.
//!
//! PDB: `handle_rmb_autoclutter @0x140976ca0` (1359 B).

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::Gap;

/// `handle_rmb_autoclutter` @0x140976ca0 (O).
pub fn handle_rmb_autoclutter(
    mut _clutter: Query<&mut Gap>,
    _cursor: Res<Gap>,
    _input: Res<Gap>,
    _commands: Commands,
) {
    todo!("recover handle_rmb_autoclutter (RMB remove auto-clutter)")
}
