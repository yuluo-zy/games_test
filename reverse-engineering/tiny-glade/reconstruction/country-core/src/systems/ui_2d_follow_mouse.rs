//! `systems/ui_2d_follow_mouse.rs` — 2D UI markers that follow the mouse.
//!
//! PDB: `ui_2d_follow_mouse @0x14092a100` (1265 B).
//!
//! O: puffin scope; two Query branches (flag @+0x108). For each entity, a
//! source index (`@+0x94` of the stride-0xa0 array / `@+0x10` of the
//! stride-0x48 array) selects a per-entity slot; the cursor position (Vec2)
//! and a u32 layer tag are written into the slot.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;
use glam::Vec2;

use crate::Gap;

/// `Ui2DFollowMouse` (O named entity; component).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Ui2DFollowMouse {
    pub _gap: Gap,
}

/// `ui_2d_follow_mouse` @0x14092a100 (O).
pub fn ui_2d_follow_mouse(
    mut _markers: Query<(&Ui2DFollowMouse, &mut Gap)>,
    _cursor: Res<Gap>,
    _layer: Res<Gap>,
) {
    let _scope = crate::reused::puffin::Scope::new("ui_2d_follow_mouse");
    todo!("recover ui_2d_follow_mouse (source index + cursor write per entity)")
}
