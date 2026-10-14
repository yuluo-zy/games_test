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
//! `translate_screen_space`（0x1409938b0）依赖未解的地形采样函数 0x1409e9640，不在这里。
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
