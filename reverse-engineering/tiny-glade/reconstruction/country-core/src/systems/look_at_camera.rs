//! `systems/look_at_camera.rs` — the `LookAtCamera` component driver.
//!
//! PDB: `look_at_camera @0x140873de0`, `register_required_components
//! @0x1408756e0`, `clone_behavior @0x140002370`. Named entity `LookAtCamera`.
//!
//! O: a Query over `(LookAtCamera, Transform)` entities rotates the camera to
//! look at the recorded target each frame; `register_required_components`
//! inserts `Transform` (+ look-at marker) for spawned camera entities.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::utils::components::transform::Transform;
use crate::Gap;

/// `LookAtCamera` (O named entity; component).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct LookAtCamera {
    /// Target world position to look at (I).
    pub target: Vec3,
    /// Up direction (I).
    pub up: Vec3,
}

/// `look_at_camera` @0x140873de0 (O).
pub fn look_at_camera(
    mut _cameras: Query<(&LookAtCamera, &mut Transform)>,
) {
    // Rotate each camera's Transform toward its target each frame.
    todo!("recover look_at_camera per-entity look-at (Transform rotation)")
}

/// `register_required_components` @0x1408756e0 (O): ensure spawned
/// `LookAtCamera` entities carry the required components.
pub fn register_required_components(
    _commands: Commands,
    _cameras: Query<(Entity, Option<&Transform>), With<LookAtCamera>>,
) {
    todo!("recover register_required_components insert Transform")
}
