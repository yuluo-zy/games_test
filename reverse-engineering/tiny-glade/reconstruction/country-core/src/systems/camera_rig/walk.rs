//! `camera_rig/walk.rs` — `WalkCamera` (dolly `Position`+`YawPitch`,
//! sphere-cast collision + terrain blend).
//!
//! PDB: `WalkCamera::{new @0x140993e20, teleport @0x140994130, rotation
//! @0x140993e90}`; `CameraBody for WalkCamera::{rotate_yaw_pitch,
//! constrain_to_be_above_terrain @0x140994320, translate_screen_space
//! @0x140994700, update @0x140993d30}`.
//!
//! O: `translate_screen_space` walks a 0.5-radius sphere cast (8 iterations,
//! filter `0x30cf`, normal slide) and clamps XZ to ±34; `constrain_to_be_above_
//! terrain` sphere-casts down (filter `0x20`, radius 0.45) for ground + terrain
//! blend + exponential vertical smoothing. Math reconciled with
//! `reconstruction/camera/walk.rs` (machine-code verified).

#![allow(dead_code, unused_imports)]

use dolly::drivers::{Position, YawPitch};
use dolly::rig::CameraRig;
use glam::{Vec2, Vec3};

use super::camera_body::{
    bounds_xz, lerp, smooth, CameraBody, CameraTransform, CastHit, CastRequest, CollisionQueries,
    TerrainQueries,
};

/// `WalkCamera` (O named entity).
#[derive(Debug)]
pub struct WalkCamera {
    pub rig: CameraRig,
    pub last_translation_dt: f32,
    pub allow_vertical: bool,
    pub ground_hit: Option<CastHit>,
}

impl WalkCamera {
    /// `new` @0x140993e20 (O: Position(Vec3::Y) + YawPitch rig).
    pub fn new() -> Self {
        Self {
            rig: CameraRig::builder()
                .with(Position::new(Vec3::Y))
                .with(YawPitch::new())
                .build(),
            last_translation_dt: 0.0,
            allow_vertical: false,
            ground_hit: None,
        }
    }

    /// `teleport` @0x140994130 (O: reposition + set yaw/pitch from a facing
    /// direction).
    pub fn teleport(&mut self, p: Vec3, direction: Vec3) {
        self.rig.driver_mut::<Position>().position = p.into();
        let right = Vec3::Y.cross(-direction).normalize();
        let q = glam::Quat::from_mat3(&glam::Mat3::from_cols(
            right,
            (-direction).cross(right),
            -direction,
        ));
        self.rig.driver_mut::<YawPitch>().set_rotation_quat(q);
    }

    /// `rotation` @0x140993e90 (O).
    pub fn rotation(&self) -> glam::Quat {
        self.rig.final_transform.rotation.into()
    }

    /// `translate_screen_space` @0x140994700 (O).
    pub fn translate_screen_space(
        &mut self,
        movement: Vec3,
        dt: f32,
        collision: &mut dyn CollisionQueries,
        demo: bool,
    ) {
        self.last_translation_dt = dt;
        let len = movement.length_squared();
        if !(1e-10 <= len) {
            return;
        }
        let original = self.rotation() * movement;
        let mut flat = original;
        if !self.allow_vertical {
            flat.y = 0.0;
        }
        let mut step = flat.normalize_or_zero() * len.sqrt() * 8.0 / 60.0;
        let initial = self.position();
        let mut p = initial + Vec3::new(0.0, -0.5, 0.0);
        for _ in 0..8 {
            let hit = collision.spherecast(CastRequest {
                origin: p,
                direction: step,
                radius: 0.5,
                max_toi: 1.0,
                filter: 0x30cf,
            });
            match hit {
                None => {
                    p += step;
                    break;
                }
                Some(hit) => {
                    let t = hit.toi.max(0.0).min(1.0);
                    p += step * t * 0.9;
                    let remainder = step * (1.0 - t);
                    let normal = hit.normal.normalize();
                    step = remainder - normal * normal.dot(remainder) + step * t * 0.1;
                    if original.dot(step) <= 0.0 || step.length_squared() < 1e-10 {
                        break;
                    }
                }
            }
        }
        p += Vec3::new(0.0, 0.5, 0.0);
        let xz = bounds_xz(Vec2::new(p.x, p.z), 34.0, demo);
        let target = Vec3::new(xz.x, p.y, xz.y);
        let a = smooth(dt);
        let adjusted = Vec3::new(
            lerp(p.x, target.x, a),
            lerp(p.y, target.y, a),
            lerp(p.z, target.z, a),
        );
        self.rig.driver_mut::<Position>().translate(adjusted - initial);
    }

    pub fn position(&self) -> Vec3 {
        self.rig.driver::<Position>().position.into()
    }

    /// `constrain_to_be_above_terrain` @0x140994320 (O).
    pub fn constrain_to_be_above_terrain(
        &mut self,
        terrain: &mut dyn TerrainQueries,
        collision: &mut dyn CollisionQueries,
        skip_blend: bool,
    ) {
        let p = self.position();
        let origin = p + Vec3::new(0.0, -0.5, 0.0);
        let mut ground = p.y - 1.9 - 1.4;
        self.ground_hit = None;
        let clamped = Vec3::new(p.x.max(-64.9).min(64.9), origin.y, p.z.max(-64.9).min(64.9));
        let first = collision.spherecast(CastRequest {
            origin: clamped,
            direction: Vec3::NEG_Y,
            radius: 0.45,
            max_toi: f32::MAX,
            filter: 0x20,
        });
        if let Some(hit) = first {
            ground = (origin.y - hit.toi - 0.45).max(ground);
            self.ground_hit = Some(CastHit {
                observed_collision_kind: 0xb,
                ..hit
            });
        }
        if let Some(hit) = collision.spherecast(CastRequest {
            origin,
            direction: Vec3::NEG_Y,
            radius: 0.45,
            max_toi: f32::MAX,
            filter: if self.allow_vertical { 0 } else { 0x30cf },
        }) {
            let y = origin.y - hit.toi - 0.45;
            if ground < y {
                ground = y;
                self.ground_hit = Some(hit);
            }
        }
        let blend = if skip_blend {
            1.9
        } else {
            let v = terrain.walk_height_blend(Vec2::new(p.x, p.z));
            v * 1.9 + (1.0 - v) * 0.0
        };
        let mut target = (ground + 1.9).max(blend.max(0.4));
        if self.allow_vertical {
            target = p.y.max(target);
        }
        let y = lerp(p.y, target, 1.0 - (-self.last_translation_dt * 20.0).exp());
        let mut position = self.position();
        position.y = y;
        self.rig.driver_mut::<Position>().position = position.into();
    }
}

impl Default for WalkCamera {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraBody for WalkCamera {
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
