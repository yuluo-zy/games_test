//! `camera_rig/butterfly.rs` — `ButterflyCamera` (dolly rig with
//! `Position`+`Rotation`+`Smooth`+`Arm`+`LookAt`; camera-bookmark zoom).
//!
//! PDB: `CameraBody for ButterflyCamera::{rotate_yaw_pitch,
//! constrain_to_be_above_terrain, translate_screen_space, update @0x140a0d090}`.
//!
//! O: `update` — when `selected_index != applied_index`, snap
//! (`applied_index = selected_index`, `dt = 1000.0`) then run the dolly rig
//! update (driver vtable @+0x30) and write the 32 B Transform output. The
//! `rotate_yaw_pitch`/`constrain_to_be_above_terrain`/`translate_screen_space`
//! fold to the shared 1-byte trampoline @0x14003bb80 (empty for the butterfly
//! body). Math reconciled with `reconstruction/camera/butterfly.rs`
//! (machine-code verified).

#![allow(dead_code, unused_imports)]

use dolly::drivers::{Arm, LookAt, Position, Rotation, Smooth};
use dolly::rig::CameraRig;
use glam::{Quat, Vec3};

use super::camera_body::{CameraBody, CameraTransform};

/// `ButterflyCamera` (O named entity).
#[derive(Debug)]
pub struct ButterflyCamera {
    pub rig: CameraRig,
    pub selected_index: u32,
    pub applied_index: u32,
}

impl ButterflyCamera {
    /// `new` (O: Position(0) + Rotation(identity) + Smooth(2,predictive) +
    /// Arm((0,1.5,3.5)) + Smooth(4) + LookAt tracking_smoothness(4)).
    pub fn new() -> Self {
        Self {
            rig: CameraRig::builder()
                .with(Position::new(Vec3::ZERO))
                .with(Rotation::new(Quat::IDENTITY))
                .with(Smooth::new_position(2.0).predictive(true))
                .with(Arm::new(Vec3::new(0.0, 1.5, 3.5)))
                .with(Smooth::new_position(4.0))
                .with(LookAt::new(Vec3::ZERO).tracking_smoothness(4.0))
                .build(),
            selected_index: 0,
            applied_index: u32::MAX,
        }
    }

    /// `follow` (O inline; Query<Transform, With<Butterfly>> feeds Position /
    /// Rotation / LookAt target).
    pub fn follow(&mut self, t: CameraTransform) {
        self.rig.driver_mut::<Position>().position = t.position.into();
        self.rig.driver_mut::<Rotation>().rotation = t.rotation.into();
        self.rig.driver_mut::<LookAt>().target = (t.position + Vec3::Y).into();
    }

    /// `advance` (O inline; camera-bookmark zoom direction changes selection).
    pub fn advance(&mut self, zoom: f32) {
        if zoom != 0.0 {
            let sign = if zoom.is_sign_negative() { -1i32 } else { 1 };
            self.selected_index = (self.selected_index as i32).wrapping_sub(sign).max(0) as u32;
        }
    }
}

impl Default for ButterflyCamera {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraBody for ButterflyCamera {
    /// `rotate_yaw_pitch` @0x14003bb80 (O): empty for the butterfly body.
    fn rotate_yaw_pitch(&mut self, _: f32, _: f32) {}
    /// `update` @0x140a0d090 (O): snap dt to 1000.0 on index change, then rig
    /// update.
    fn update(&mut self, dt: f32) -> CameraTransform {
        let dt = if self.selected_index != self.applied_index {
            self.applied_index = self.selected_index;
            1000.0
        } else {
            dt
        };
        let t = self.rig.update(dt);
        CameraTransform {
            position: t.position.into(),
            rotation: t.rotation.into(),
        }
    }
}
