//! `ui_clutter/ui_focus_decoclutter.rs` — focus-mode deco-clutter move/rotate.
//!
//! PDB: `ui_focus_decoclutter @0x1409a92c0` (11278 B) + closures `$0 @0x1409abed0`,
//! `$1 @0x1409ac140`, `$2 @0x1409ac3e0`, `$3 @0x1409ac640`. Named entities
//! `LastClonedClutter`, `RequestMoveClutter`, `RequestRotateDecorator`.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::Gap;

/// `LastClonedClutter` (O named entity; resource).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct LastClonedClutter {
    pub _gap: Gap,
}

/// `RequestMoveClutter` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct RequestMoveClutter {
    pub _gap: Gap,
}

/// `RequestRotateDecorator` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct RequestRotateDecorator {
    pub _gap: Gap,
}

/// `ui_focus_decoclutter` @0x1409a92c0 (O).
pub fn ui_focus_decoclutter(
    mut _last: ResMut<LastClonedClutter>,
    mut _move_requests: EventWriter<RequestMoveClutter>,
    mut _rotate_requests: EventWriter<RequestRotateDecorator>,
    _cursor: Res<Gap>,
    _commands: Commands,
) {
    todo!("recover ui_focus_decoclutter (focus move/rotate + clone tracking)")
}
