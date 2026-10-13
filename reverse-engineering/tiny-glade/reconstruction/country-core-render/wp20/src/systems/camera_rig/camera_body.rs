//! `camera_rig/camera_body.rs` — the `CameraBody` trait and shared camera
//! types.
//!
//! PDB: `CameraBody` impls on `FreeCamera`/`NormalCamera`/`WalkCamera`/
//! `ButterflyCamera` (rotate_yaw_pitch / constrain_to_be_above_terrain /
//! translate_screen_space / update). Motion math reconciled with the verified
//! `reconstruction/camera` crate (O constants); terrain/collision queries are
//! consumed through traits so the game's collision (wp16 rapier glue) can be
//! wired at integration.

#![allow(dead_code, unused_imports)]

use glam::{Quat, Vec2, Vec3};
use serde::{Deserialize, Serialize};

/// `CameraMode` (I: 6 modes; O u32 discriminant jumptable in `main_camera` /
/// `camera_snapshot`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum CameraMode {
    Normal = 0,
    TiltShift = 1,
    Free = 2,
    Walk = 3,
    TopDown = 4,
    Butterfly = 5,
}

/// `CameraTransform` (O: 28 B position+rotation output written by body
/// `update`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraTransform {
    pub position: Vec3,
    pub rotation: Quat,
}

impl CameraTransform {
    pub fn forward(self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }
}

/// Terrain-height queries consumed by the bodies (game collision/terrain
/// facade — wp16/terrain wiring; O call sites).
pub trait TerrainQueries {
    fn sample_height(&mut self, xz: Vec2) -> f32;
    fn walk_height_blend(&mut self, xz: Vec2) -> f32;
}

/// Sphere-cast / ray-cast queries consumed by the walk body (rapier glue,
/// wp16; O call sites).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CastRequest {
    pub origin: Vec3,
    pub direction: Vec3,
    pub radius: f32,
    pub max_toi: f32,
    pub filter: u32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CastHit {
    pub toi: f32,
    pub normal: Vec3,
    pub observed_collision_kind: u8,
}
pub trait CollisionQueries {
    fn spherecast(&mut self, request: CastRequest) -> Option<CastHit>;
    fn raycast(&mut self, origin: Vec3, direction: Vec3, max_toi: f32, filter: u32) -> Option<Vec3>;
}

/// Motion context passed to `translate_screen_space` (O: camera matrices +
/// demo-mode flag).
#[derive(Clone, Copy, Debug)]
pub struct MotionContext {
    pub matrices_transform: CameraTransform,
    pub demo_mode: bool,
}

/// `CameraBody` (O trait): the country-core camera-body interface.
pub trait CameraBody {
    fn rotate_yaw_pitch(&mut self, yaw: f32, pitch: f32);
    fn update(&mut self, dt: f32) -> CameraTransform;
}

/// Shared helpers (O: lerp / exp smoothing / XZ bounds used by every body).
pub(crate) fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (b - a) * t + a
}
pub(crate) fn smooth(dt: f32) -> f32 {
    1.0 - (dt * -8.0).exp()
}
/// B XZ-bounds: demo keeps points inside a radius-2*extent disc; normal mode
/// clamps to the `[-extent, extent]` square with a radial correction.
pub(crate) fn bounds_xz(p: Vec2, extent: f32, demo: bool) -> Vec2 {
    if demo {
        let d = p.length_squared();
        if extent * extent * 4.0 < d {
            p / p.length() * (extent * 2.0)
        } else {
            p
        }
    } else {
        let c = Vec2::new(p.x.max(-extent).min(extent), p.y.max(-extent).min(extent));
        let d = p - c;
        if extent * extent < d.length_squared() {
            c + d * (extent / d.length())
        } else {
            p
        }
    }
}
