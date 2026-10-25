//! `systems::camera_rig::free_camera` — Free 模式相机（dolly `CameraRig` 加三个访问器）。
//!
//! 原版里 Free body 就是一份 dolly 0.6.0 的 `CameraRig`：`{ drivers: Vec<Box<dyn RigDriver>>, final_transform }`
//! （Vec 在 +0、`final_transform` 在 +0x18，对 `FreeCamera::update` 0x140993d30 的写回偏移）。游戏自己的
//! 逻辑只有"怎么搭这副 rig"，纯 dolly 数学不重实现。
//!
//! | 函数 | 地址 | 级别 |
//! |---|---|---|
//! | `MainCamera::match_free_camera_to_normal` | 0x14090e6f0 | O：`Position(pos) + YawPitch{yaw=[+0x154], pitch=[+0x150]} + Smooth(1, 1)`，`dt = 0` 跑一遍 |
//! | `FreeCamera::get_position` | 0x140992f10 | O：按 TypeId `0x6b8463577afc1a66` 找 `Position`，缺失 → dolly 的 panic |
//! | `FreeCamera::get_yaw_pitch` | 0x140993010 | O：TypeId `0x718e4fd258330fe5` 找 `YawPitch`，返回前 8 字节 |
//! | `FreeCamera::update` | 0x140993d30 | O：从单位变换起逐个 driver 更新，结果写回 `final_transform` 并返回 |
//!
//! 单位变换（0x142aece40）：旋转 `(0,0,0,1)` + 零位置，与 dolly 的 `Transform::IDENTITY` 一致。
//! translate_screen_space1409938b0 is restored below against the actual CPU height owner.
//!
//! dolly 0.6.0 依赖 glam 0.27（原版 `Cargo.lock` 如此），与工作区的 glam 0.29 是两个类型；
//! 数值在边界处按分量转换。

use dolly::drivers::{Position, Smooth, YawPitch};
use dolly::prelude::CameraRig;
use glam::{Quat, Vec3};

/// Free 相机。
#[derive(Debug)]
pub struct FreeCamera {
    pub rig: CameraRig,
}

impl FreeCamera {
    /// O 0x14090e6f0：用 Normal 相机的位置/朝向重建 Free rig。
    /// 参数顺序与原版读取一致：位置、`pitch`（`+0x150`）、`yaw`（`+0x154`）。
    pub fn match_normal(position: Vec3, pitch_degrees: f32, yaw_degrees: f32) -> Self {
        let mut rig = CameraRig::builder()
            .with(Position::new([position.x, position.y, position.z]))
            .with(
                YawPitch::new()
                    .yaw_degrees(yaw_degrees)
                    .pitch_degrees(pitch_degrees),
            )
            .with(Smooth::new_position_rotation(1.0, 1.0))
            .build();
        rig.update(0.0);
        Self { rig }
    }

    /// O 0x140992f10。
    pub fn get_position(&mut self) -> Vec3 {
        let p = self.rig.driver_mut::<Position>().position;
        Vec3::new(p.x, p.y, p.z)
    }

    /// O 0x140993010：`(yaw_degrees, pitch_degrees)`。
    pub fn get_yaw_pitch(&mut self) -> (f32, f32) {
        let d = self.rig.driver_mut::<YawPitch>();
        (d.yaw_degrees, d.pitch_degrees)
    }

    /// O 0x140993d30：返回更新后的 `(位置, 旋转)`。
    pub fn update(&mut self, dt: f32) -> (Vec3, Quat) {
        let t = self.rig.update(dt);
        let (p, q) = (t.position, t.rotation);
        (
            Vec3::new(p.x, p.y, p.z),
            Quat::from_xyzw(q.v.x, q.v.y, q.v.z, q.s),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_normal_reproduces_position_and_angles() {
        let mut free = FreeCamera::match_normal(Vec3::new(1.0, 2.0, 3.0), -20.0, 90.0);
        assert_eq!(free.get_position(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(free.get_yaw_pitch(), (90.0, -20.0));
    }

    #[test]
    fn dt_zero_pass_puts_the_rig_at_the_target_pose() {
        let free = FreeCamera::match_normal(Vec3::new(4.0, 5.0, 6.0), 0.0, 0.0);
        let t = free.rig.final_transform;
        assert_eq!((t.position.x, t.position.y, t.position.z), (4.0, 5.0, 6.0));
        let q = t.rotation;
        let identity = Quat::from_xyzw(q.v.x, q.v.y, q.v.z, q.s);
        assert!(identity.abs_diff_eq(Quat::IDENTITY, 1e-6));
    }

    #[test]
    #[should_panic(expected = "driver found in the CameraRig")]
    fn missing_driver_panics_with_dollys_message() {
        let mut free = FreeCamera {
            rig: CameraRig::builder().build(),
        };
        free.get_position();
    }

    #[test]
    fn update_moves_the_smoothed_pose_toward_the_driver() {
        let mut free = FreeCamera::match_normal(Vec3::ZERO, 0.0, 0.0);
        free.rig.driver_mut::<Position>().position = [10.0, 0.0, 0.0].into();
        let (position, _) = free.update(0.016);
        assert!(position.x > 0.0 && position.x < 10.0);
    }
}

/// B Free/Walk share this original driver operation@140993750.
/// Dolly owns the yaw720 remainder/pitch90 clamp and missing-driver panic.
pub(crate) fn rotate_rig_yaw_pitch(rig: &mut CameraRig, yaw: f32, pitch: f32) {
    rig.driver_mut::<YawPitch>()
        .rotate_yaw_pitch(yaw * 0.5, pitch * 0.5);
}

pub(crate) fn rig_position(rig: &CameraRig) -> Vec3 {
    let p = rig.driver::<Position>().position;
    Vec3::new(p.x, p.y, p.z)
}
pub(crate) fn rig_rotation(rig: &CameraRig) -> Quat {
    let q = rig.final_transform.rotation;
    Quat::from_xyzw(q.v.x, q.v.y, q.v.z, q.s)
}
pub(crate) fn set_rig_position(rig: &mut CameraRig, p: Vec3) {
    rig.driver_mut::<Position>().position = [p.x, p.y, p.z].into();
}

/// B FreeCamera::translate_screen_space@1409938b0, full1146B.
/// Uses the real existing Dolly rig and the same CPU height owner. The floor
/// is sampled at the smoothed horizontal target, not at the hard-clamped point.
pub(crate) fn translate_rig_screen_space(
    rig: &mut CameraRig,
    movement: Vec3,
    dt: f32,
    heights: &crate::resources::terrain_height::TerrainHeightsData,
) {
    let p = rig_position(rig);
    let rotated = rig_rotation(rig) * movement;
    let target = p + rotated;
    let alpha = 1.0 - (dt * -8.0).exp();
    let max = |a: f32, b: f32| if a > b { a } else { b };
    let min = |a: f32, b: f32| if a < b { a } else { b };
    let x = target.x + (min(91.0, max(-91.0, target.x)) - target.x) * alpha;
    let z = target.z + (min(91.0, max(-91.0, target.z)) - target.z) * alpha;
    let floor = max(heights.sample_at_world_xz(x, z), 0.0) + 0.75;
    let y = target.y + (min(91.0, max(0.0, target.y)) - target.y) * alpha;
    let y = if y.is_nan() { floor } else { max(floor, y) };
    let delta = Vec3::new(
        (x - p.x) * 20.0 / 60.0,
        (y - p.y) * 20.0 / 60.0,
        (z - p.z) * 20.0 / 60.0,
    );
    let driver = rig.driver_mut::<Position>();
    driver.translate([delta.x, delta.y, delta.z]);
}

#[cfg(test)]
mod original_motion_tests {
    use super::*;
    #[test]
    fn original_free_translation_samples_the_same_height_grid() {
        let heights = crate::resources::terrain_height::TerrainHeightsData::new();
        let mut free = FreeCamera::match_normal(Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0);
        translate_rig_screen_space(&mut free.rig, Vec3::ZERO, 0.0, &heights);
        assert_eq!(free.get_position(), Vec3::new(0.0, 0.25, 0.0));
    }
}
