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
//! U：teleport 的欧拉转换辅助（FUN_140334d80/FUN_141766a90）未逐指令恢复，
//! 这里用等价的 `from_rotation_arc(-Z, dir)` + YXZ 分解（I）；球投滑行/
//! 地形约束属于碰撞域，不在本文件。
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
    let length_squared = direction.length_squared();
    if length_squared > 0.0 && length_squared.is_finite() {
        let dir = direction / length_squared.sqrt();
        let rotation = Quat::from_rotation_arc(Vec3::NEG_Z, dir);
        let (yaw, pitch, _) = rotation.to_euler(glam::EulerRot::YXZ);
        let driver = rig.driver_mut::<YawPitch>();
        driver.yaw_degrees = yaw.to_degrees();
        driver.pitch_degrees = pitch.to_degrees();
    }
    // 与原作一致：方向为零/非有限时保持当前朝向。
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
