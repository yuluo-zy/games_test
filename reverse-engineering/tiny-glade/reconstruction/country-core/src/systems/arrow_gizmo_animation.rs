//! `systems/arrow_gizmo_animation.rs` — arrow-gizmo stretch animation.
//!
//! PDB: `arrow_gizmo_animation @0x140a92f80` (2056 B). Named entity
//! `EnableArrowGizmoStretch`.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::Gap;

/// `EnableArrowGizmoStretch` (O named entity; component).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct EnableArrowGizmoStretch {
    pub _gap: Gap,
}

/// `arrow_gizmo_animation` @0x140a92f80 (O).
pub fn arrow_gizmo_animation(
    mut _gizmos: Query<(&EnableArrowGizmoStretch, &mut Gap)>,
    _time: Res<Gap>,
) {
    let _scope = crate::reused::puffin::Scope::new("arrow_gizmo_animation");
    todo!("recover arrow_gizmo_animation (stretch animation)")
}
