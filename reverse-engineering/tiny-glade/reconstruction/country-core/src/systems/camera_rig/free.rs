//! `camera_rig/free.rs` — `FreeCamera` (dolly `Position`+`YawPitch`+`Smooth`).
//!
//! PDB: `FreeCamera::{get_position @0x140992f10, get_yaw_pitch @0x140993010,
//! set @0x140993100}`; `CameraBody for FreeCamera::{rotate_yaw_pitch,
//! translate_screen_space @0x1409938b0, update @0x140993d30}`.
//!
//! O: driver lookup by `dolly::Rig::driver::<Position/YawPitch>` (type-id
//! search); `translate_screen_space` samples terrain height (max 0) + 0.75 and
//! clamps XZ to ±91, moving the `Position` driver at `20/60 * lerp`; `set`
//! rebuilds the rig. Math reconciled with `reconstruction/camera/free.rs`
//! (machine-code verified).

#![allow(dead_code, unused_imports)]

use dolly::drivers::{Position, Smooth, YawPitch};
use dolly::rig::CameraRig;
use glam::{Vec2, Vec3};

use super::camera_body::{
    bounds_xz, lerp, smooth, CameraBody, CameraTransform, MotionContext, TerrainQueries,
};

/// `FreeCamera` (O named entity; dolly rig).
#[derive(Debug)]
pub struct FreeCamera {
    pub rig: CameraRig,
}

impl FreeCamera {
    pub fn new() -> Self {
        Self::from_position_angles(Vec3::Y, 0.0, 0.0)
    }
    /// O: Position + YawPitch (degrees) + Smooth(1,1) rig.
    pub fn from_position_angles(position: Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            rig: CameraRig::builder()
                .with(Position::new(position))
                .with(YawPitch::new().yaw_degrees(yaw).pitch_degrees(pitch))
                .with(Smooth::new_position_rotation(1.0, 1.0))
                .build(),
        }
    }
    /// `set` @0x140993100 (O): rebuild the rig from position/yaw/pitch.
    pub fn set(&mut self, position: Vec3, yaw: f32, pitch: f32) {
        *self = Self::from_position_angles(position, yaw, pitch);
    }
    /// `get_position` @0x140992f10 (O).
    pub fn get_position(&self) -> Vec3 {
        self.rig.driver::<Position>().position.into()
    }
    /// `get_yaw_pitch` @0x140993010 (O).
    pub fn get_yaw_pitch(&self) -> Vec2 {
        let p = self.rig.driver::<YawPitch>();
        Vec2::new(p.yaw_degrees, p.pitch_degrees)
    }
    pub fn transform(&self) -> CameraTransform {
        CameraTransform {
            position: self.rig.final_transform.position.into(),
            rotation: self.rig.final_transform.rotation.into(),
        }
    }
    /// `translate_screen_space` @0x1409938b0 (O): screen-space pan with terrain
    /// follow (+0.75 clearance), XZ clamp ±91.
    pub fn translate_screen_space(
        &mut self,
        movement: Vec3,
        dt: f32,
        terrain: &mut dyn TerrainQueries,
    ) {
        let p = self.get_position();
        let target = p + self.transform().rotation * movement;
        let a = smooth(dt);
        let x = target.x.max(-91.0).min(91.0);
        let z = target.z.max(-91.0).min(91.0);
        let height = terrain.sample_height(Vec2::new(x, z)).max(0.0) + 0.75;
        let y = lerp(target.y, target.y.max(0.0).min(91.0), a).max(height);
        let delta = Vec3::new(
            (lerp(target.x, x, a) - p.x) * 20.0 / 60.0,
            (y - p.y) * 20.0 / 60.0,
            (lerp(target.z, z, a) - p.z) * 20.0 / 60.0,
        );
        self.rig.driver_mut::<Position>().translate(delta);
    }
}

impl Default for FreeCamera {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraBody for FreeCamera {
    fn rotate_yaw_pitch(&mut self, yaw: f32, pitch: f32) {
        self.rig
            .driver_mut::<YawPitch>()
            .rotate_yaw_pitch(yaw * 0.5, pitch * 0.5);
    }
    fn update(&mut self, dt: f32) -> CameraTransform {
        let t = self.rig.update(dt);
        CameraTransform {
            position: t.position.into(),
            rotation: t.rotation.into(),
        }
    }
}
