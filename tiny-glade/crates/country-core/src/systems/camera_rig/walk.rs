//! `WalkCamera`（B `systems::camera_rig::walk`）。
//!
//! 证据（PDB v2 + camera-time 反编译）：
//! * `WalkCamera::new` 0x140993e20：`Position(0,1,0) + YawPitch(0,0)`，无 Smooth。
//! * `WalkCamera::teleport` 0x140994130：Position driver 写坐标；方向向量转
//!   yaw/pitch 度。
//! * `WalkCamera::rotation` 0x140994310：16 B 拷贝分叉 `CameraTransform`
//!   rotation-first 的四元数。
//! * `update` 0x140993d30：dolly rig 迭代（共享）。
//!
//! teleport 的两个辅助经 PDB 原库身份核实：dolly::util::look_at<RightHanded>
//! 与 glam::EulerRot::YXZ，直接复用锁内依赖；无需重写通用数学。
use dolly::drivers::{Position, YawPitch};
use dolly::prelude::CameraRig;
use glam::{Quat, Vec3};

#[derive(Debug)]
pub struct WalkCamera {
    pub rig: CameraRig,
    /// 原 +0x54：保留竖直分量。
    pub allow_vertical: bool,
    /// 原 +0x50：上次平移 dt。
    pub last_translation_dt: f32,
}

impl WalkCamera {
    /// O 0x140993e20。
    pub fn new() -> Self {
        let rig = CameraRig::builder()
            .with(Position::new([0.0, 1.0, 0.0]))
            .with(YawPitch::new())
            .build();
        Self {
            rig,
            allow_vertical: false,
            last_translation_dt: 0.0,
        }
    }

    pub fn position(&mut self) -> Vec3 {
        let p = self.rig.driver_mut::<Position>().position;
        Vec3::new(p.x, p.y, p.z)
    }

    /// O 0x140994130：写位置；方向向量转 yaw/pitch 度。
    pub fn teleport(&mut self, position: Vec3, direction: Vec3) {
        teleport_rig(&mut self.rig, position, direction);
    }

    /// O 0x140994310：返回分叉 rotation-first 变换的四元数。
    pub fn rotation(&self) -> Quat {
        let q = self.rig.final_transform.rotation;
        Quat::from_xyzw(q.v.x, q.v.y, q.v.z, q.s)
    }

    /// O 0x140993d30。
    pub fn update(&mut self, dt: f32) -> (Vec3, Quat) {
        let t = self.rig.update(dt);
        let (p, q) = (t.position, t.rotation);
        (
            Vec3::new(p.x, p.y, p.z),
            Quat::from_xyzw(q.v.x, q.v.y, q.v.z, q.s),
        )
    }
}

impl Default for WalkCamera {
    fn default() -> Self {
        Self::new()
    }
}

/// 直接对已存在的 walk rig 执行 teleport（`MainCamera` 的 rig 由
/// `startup_rigs.walk` 持有，不需要移动所有权）。
pub fn teleport_rig(rig: &mut CameraRig, position: Vec3, direction: Vec3) {
    rig.driver_mut::<Position>().position = [position.x, position.y, position.z].into();
    let rotation = dolly::drivers::Rotation {
        rotation: dolly::util::look_at::<dolly::handedness::RightHanded, _, _>(
            [direction.x, direction.y, direction.z],
        ),
    }.rotation;
    let rotation = Quat::from_xyzw(rotation.v.x, rotation.v.y, rotation.v.z, rotation.s);
    let (yaw, pitch, _) = rotation.to_euler(glam::EulerRot::YXZ);
    let driver = rig.driver_mut::<YawPitch>();
    driver.yaw_degrees = yaw.to_degrees();
    driver.pitch_degrees = pitch.to_degrees();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_position_0_1_0_with_no_smoothing() {
        let camera = WalkCamera::new();
        let t = camera.rig.final_transform;
        assert_eq!((t.position.x, t.position.y, t.position.z), (0.0, 1.0, 0.0));
        assert_eq!(camera.rig.drivers.len(), 2);
    }

    #[test]
    fn teleport_writes_position_and_looks_down_negative_z() {
        let mut camera = WalkCamera::new();
        camera.teleport(Vec3::new(3.0, 4.0, 5.0), Vec3::NEG_Z);
        assert_eq!(camera.position(), Vec3::new(3.0, 4.0, 5.0));
        let (yaw, pitch, _) = camera.rotation().to_euler(glam::EulerRot::YXZ);
        assert!(yaw.abs() < 1e-5, "yaw {yaw}");
        assert!(pitch.abs() < 1e-5, "pitch {pitch}");
    }
}

#[cfg(feature = "native-physics")]
mod native_motion {
    //! Original B WalkCamera motion on MainCamera's existing Dolly rig.
    //! Full routines1409947002084B and140994320990B. Native sphere sweeps
    //! reuse the one RaycastWorld+TerrainHeightsData; no second physics owner.

    use super::super::free_camera::{rig_position, rig_rotation, set_rig_position};
    use crate::{
        resources::{terrain_height::TerrainHeightsData, world_raster_defs::PathRaster},
        systems::collision::world::RaycastWorld,
    };
    use dolly::prelude::CameraRig;
    use glam::{Vec2, Vec3};
    fn max(a: f32, b: f32) -> f32 {
        if a > b { a } else { b }
    }
    fn min(a: f32, b: f32) -> f32 {
        if a < b { a } else { b }
    }
    fn options(limit: f32) -> rapier3d::parry::query::ShapeCastOptions {
        rapier3d::parry::query::ShapeCastOptions {
            max_time_of_impact: limit,
            target_distance: 0.0,
            stop_at_penetration: false,
            compute_impact_geometry_on_penetration: false,
        }
    }
    /// Mirror the original byte-only/three-word source update at body+38.
    /// MainCamera retains unspecified payload bytes when only its tag is written.
    pub(crate) enum GroundSourceUpdate {
        NoHit,
        Terrain,
        Collider([u64; 3]),
    }

    pub(crate) fn translate_rig_screen_space(
        rig: &mut CameraRig,
        movement: Vec3,
        dt: f32,
        allow_vertical: bool,
        world: &RaycastWorld,
        heights: &TerrainHeightsData,
        demo: bool,
    ) {
        let squared = movement.length_squared();
        if 1.0e-10 > squared {
            return;
        }
        let original = rig_rotation(rig) * movement;
        let mut flat = original;
        if !allow_vertical {
            flat.y = 0.0;
        }
        let mut step = (flat.normalize_or_zero() * squared.sqrt()) * 8.0 / 60.0;
        let initial = rig_position(rig);
        let mut p = initial + Vec3::new(0.0, -0.5, 0.0);
        let mask = if allow_vertical { 0x20 } else { 0x30ef };
        for _ in 0..8 {
            let hit = world.spherecast_w_terrain(heights, p, step, 0.5, options(1.0), mask);
            if hit.is_miss() {
                p += step;
                break;
            }
            let t = min(1.0, max(0.0, hit.toi));
            let traversed = step * t;
            p += traversed * 0.9;
            let remainder = step * (1.0 - t);
            let normal = hit.normal.normalize();
            // B0x142aba? stores the exact (1-.9f) rounded constant, not .1f.
            step = remainder - normal * normal.dot(remainder)
                + traversed * f32::from_bits(0x3dcc_ccd0);
            if original.dot(step) <= 0.0 || 1.0e-10 > step.length_squared() {
                break;
            }
        }
        p += Vec3::new(0.0, 0.5, 0.0);
        let point = Vec2::new(p.x, p.z);
        let target = if demo {
            let squared = point.length_squared();
            if squared > 4624.0 {
                point / squared.sqrt() * 68.0
            } else {
                point
            }
        } else {
            let clamp = |v: f32| {
                if v.is_nan() {
                    -34.0
                } else {
                    min(34.0, max(-34.0, v))
                }
            };
            let clamped = Vec2::new(clamp(point.x), clamp(point.y));
            let excess = point - clamped;
            let squared = excess.length_squared();
            if squared > 1156.0 {
                clamped + excess * (34.0 / squared.sqrt())
            } else {
                point
            }
        };
        let a = 1.0 - (dt * -8.0).exp();
        let adjusted = Vec3::new(
            p.x + (target.x - p.x) * a,
            p.y + (p.y - p.y) * a,
            p.z + (target.y - p.z) * a,
        );
        // The source obtains Position again before applying the translation.
        let current = rig_position(rig);
        set_rig_position(rig, current + (adjusted - current));
    }

    pub(crate) fn constrain_rig_to_be_above_terrain(
        rig: &mut CameraRig,
        last_translation_dt: f32,
        allow_vertical: bool,
        world: &RaycastWorld,
        heights: &TerrainHeightsData,
        path: &PathRaster,
        skip_path_blend: bool,
    ) -> GroundSourceUpdate {
        let p = rig_position(rig);
        let origin = p + Vec3::new(0.0, -0.5, 0.0);
        let down = Vec3::new(-0.0, -1.0, -0.0);
        let mut ground = (p.y - 1.9) - 1.4;
        let mut source = GroundSourceUpdate::NoHit;
        let hit = world.spherecast_w_terrain(
            heights,
            Vec3::new(
                min(64.9, max(-64.9, p.x)),
                origin.y,
                min(64.9, max(-64.9, p.z)),
            ),
            down,
            0.45,
            options(f32::MAX),
            0x20,
        );
        if hit.is_hit() {
            let height = (origin.y - hit.toi) + -0.45;
            ground = if ground.is_nan() {
                height
            } else {
                max(height, ground)
            };
            source = GroundSourceUpdate::Terrain;
        }
        let mask = if allow_vertical { 0 } else { 0x30cf };
        let hit = world.spherecast_w_terrain(heights, origin, down, 0.45, options(f32::MAX), mask);
        if hit.is_hit() {
            let height = (origin.y - hit.toi) + -0.45;
            if height > ground {
                ground = height;
                source = GroundSourceUpdate::Collider(hit.handle);
            }
        }
        let blend = if skip_path_blend {
            1.9
        } else {
            let v = path
                .as_ref()
                .bilinear_sample_at_world_xz(p.x, p.z, 130.0, 130.0);
            0.0 * (1.0 - v) + v * 1.9
        };
        let mut target = max(ground + 1.9, max(blend, 0.4));
        if allow_vertical {
            target = max(p.y, target);
        }
        let y = p.y + (target - p.y) * (1.0 - (-last_translation_dt * 20.0).exp());
        set_rig_position(rig, Vec3::new(p.x, y, p.z));
        source
    }
}
#[cfg(feature = "native-physics")]
pub(crate) use native_motion::{
    GroundSourceUpdate, constrain_rig_to_be_above_terrain, translate_rig_screen_space,
};
