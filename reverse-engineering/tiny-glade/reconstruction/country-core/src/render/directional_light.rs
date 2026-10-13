//! `DirectionalLight` — sun light with look-at basis (WP4-owned).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/render/directional_light.rs`
//! (source_sha256 `69166bac…`, 2 procs). Decompiles `A_0x140b18550.c`,
//! `A_0x140b18830.c`.
//!
//! Layout (O): +0x00 view-basis Mat4 (right/up/forward columns + translation
//! row w=1), +0x40 position Vec3, +0x50 target Vec3, +0x60 (1.0 + target),
//! +0x70 distance f32.

use glam::{Mat4, Vec3};

/// Directional (sun) light.
#[derive(Clone, Copy, Debug)]
pub struct DirectionalLight {
    /// View basis (O): column0 right, column1 up, column2 forward, column3
    /// translation.
    pub view: Mat4,
    /// Light position (O: +0x40).
    pub position: Vec3,
    /// Look-at target (O: +0x50).
    pub target: Vec3,
    /// Distance from target (O: +0x70).
    pub distance: f32,
}

impl DirectionalLight {
    /// `look_at(&mut self, eye, target)` (O: 0x140b18550, 725B): builds the
    /// look-at basis, translation = -dot(basis, eye), stores position + target
    /// + distance = |eye - target|.
    pub fn look_at(&mut self, eye: Vec3, target: Vec3) {
        let forward = (target - eye).normalize_or_zero();
        let right = forward.cross(Vec3::Y).normalize_or_zero();
        let up = right.cross(forward);

        let mut view = Mat4::IDENTITY;
        view.x_axis = right.extend(0.0);
        view.y_axis = up.extend(0.0);
        view.z_axis = forward.extend(0.0);
        // O: translation row w = 1, xyz = -dot(basis, eye).
        view.w_axis = Vec3::new(-eye.dot(right), -eye.dot(up), -eye.dot(forward)).extend(1.0);

        self.view = view;
        self.position = eye;
        self.target = target;
        self.distance = eye.distance(target);
    }

    /// `set_direction_towards_light(&mut self, direction)` (O: 0x140b18830,
    /// 695B): `position += normalize(direction) * distance`, then rebuilds the
    /// look-at basis toward the target.
    pub fn set_direction_towards_light(&mut self, direction: Vec3) {
        let dir = direction.normalize_or_zero();
        self.position += dir * self.distance;
        self.look_at(self.position, self.target);
    }

    /// `translate` (O name; U body — inline).
    pub fn translate(&mut self, _offset: Vec3) {
        // U
    }

    /// `update` (O name; U body — inline).
    pub fn update(&mut self) {
        // U
    }

    /// `view_to_clip` (O name; U body — inline).
    pub fn view_to_clip(&self) -> Mat4 {
        self.view
    }
}

impl Default for DirectionalLight {
    fn default() -> Self {
        DirectionalLight {
            view: Mat4::IDENTITY,
            position: Vec3::ZERO,
            target: Vec3::ZERO,
            distance: 0.0,
        }
    }
}
