//! `systems::camera_rig` — WP20 camera rigs: `CameraBody` trait + the four
//! dolly-based bodies (`FreeCamera`/`NormalCamera`/`WalkCamera`/
//! `ButterflyCamera`) + the `MainCamera` (0x4e0 B resource) and
//! `CameraSnapshot`.
//!
//! Source: B skeleton `crates/country-core/src/systems/camera_rig/{mod,
//! camera_body,free,normal,walk,butterfly}.rs`.
//! Business logic contract: `contracts/camera.md`.
//!
//! The per-body motion math was machine-code-verified in the prior
//! `reconstruction/camera` crate (`tiny-glade-camera-b`); the WP20 skeleton
//! aligns to the country-core module tree and reuses dolly 0.6.0 (the game's
//! locked camera-rig dependency). No dolly internals are re-implemented.

#![allow(dead_code, unused_imports)]

pub mod butterfly;
pub mod camera_body;
pub mod free;
pub mod normal;
pub mod walk;

pub use camera_body::{CameraBody, CameraMode, CameraTransform};

use bevy_ecs::resource::Resource;

use crate::Gap;

/// `MainCamera` (O named entity; **0x4e0 B** resource). Layout (O):
/// per-mode camera bodies at +0x100 (Normal), +0x1b8 (TiltShift), +0x270
/// (Free), +0x2f0 (Walk), +0x390 (TopDown), +0x448 (Butterfly); mode byte
/// **@+0x4d0**; aspect ratio **@+0x4d8**.
#[derive(Resource, Clone, Debug, Default)]
pub struct MainCamera {
    pub raw: Gap,
}

impl MainCamera {
    /// `MainCamera::new` (O inline; reset_camera memcpy 0x4e0).
    pub fn new() -> Self {
        MainCamera { raw: Gap }
    }
    /// `store_parameters` (O call from record_and_replay).
    pub fn store_parameters(&mut self) {}
    /// `load_parameters` (O call from record_and_replay).
    pub fn load_parameters(&mut self) {}
    /// `immediately_finish_interpolating` (O call from record_and_replay).
    pub fn immediately_finish_interpolating(&mut self) {}
    /// `eye_position` (O).
    pub fn eye_position(&self) -> glam::Vec3 {
        glam::Vec3::ZERO
    }
}

/// `CameraSnapshot` (O named entity; per-mode camera snapshot resource).
#[derive(Resource, Clone, Debug, Default)]
pub struct CameraSnapshot {
    pub raw: Gap,
}

/// `CameraSnapshotInterpolationState` (O named entity).
#[derive(Clone, Debug, Default)]
pub struct CameraSnapshotInterpolationState {
    pub _gap: Gap,
}

/// `CameraRotationPivot` (O named entity).
#[derive(Clone, Debug, Default)]
pub struct CameraRotationPivot {
    pub _gap: Gap,
}

/// `CacheFreeCamRig` (O named entity).
#[derive(Clone, Debug, Default)]
pub struct CacheFreeCamRig {
    pub _gap: Gap,
}

/// `MainCameraMoved` event (O: registered in `MainCameraPlugin::build`).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct MainCameraMoved;
