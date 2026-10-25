//! `ButterflyCamera`（B `systems::camera_rig::butterfly`）。
//!
//! 证据（camera-time 反编译 + `MainCamera::new` 0x14090ec70 实读）：
//! * rig：`Position(0,0,0) + Rotation(identity) + Smooth(2, predictive) +
//!   Arm(0,1.5,3.5) + Smooth(4) + LookAt(0,0,0, tracking 4)`。
//! * `update` 0x140a0d090：`selected(+0x3c) != applied(+0x38)` →
//!   `applied = selected` 且 `dt = 1000`（瞬时吸附），再迭代 rig。
//! * rotate/constrain/translate 三个操作折叠到 0x14003bb80（仅 `ret`），
//!   不约束地形。
use dolly::drivers::{Arm, LookAt, Position, Rotation, Smooth};
use dolly::prelude::CameraRig;
use glam::{Quat, Vec3};

#[derive(Debug)]
pub struct ButterflyCamera {
    pub rig: CameraRig,
    /// 原 +0x38：已应用的选中索引。
    pub applied_index: u32,
    /// 原 +0x3c：选中索引。
    pub selected_index: u32,
}

impl ButterflyCamera {
    pub fn new() -> Self {
        let rig = CameraRig::builder()
            .with(Position::new([0.0, 0.0, 0.0]))
            .with(Rotation::new([0.0, 0.0, 0.0, 1.0]))
            .with(Smooth::new_position(2.0).predictive(true))
            .with(Arm::new([0.0, 1.5, 3.5]))
            .with(Smooth::new_position(4.0))
            .with(LookAt::new([0.0, 0.0, 0.0]).tracking_smoothness(4.0))
            .build();
        Self {
            rig,
            applied_index: 0,
            selected_index: 0,
        }
    }

    /// O 0x140a0d090：索引变化时 dt 提到 1000 后迭代。
    pub fn update(&mut self, dt: f32) -> (Vec3, Quat) {
        let mut dt = dt;
        if self.selected_index != self.applied_index {
            self.applied_index = self.selected_index;
            dt = 1000.0;
        }
        let t = self.rig.update(dt);
        let (p, q) = (t.position, t.rotation);
        (
            Vec3::new(p.x, p.y, p.z),
            Quat::from_xyzw(q.v.x, q.v.y, q.v.z, q.s),
        )
    }
}

impl Default for ButterflyCamera {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rig_has_the_six_original_drivers() {
        let camera = ButterflyCamera::new();
        assert_eq!(camera.rig.drivers.len(), 6);
    }

    #[test]
    fn selecting_a_new_butterfly_snaps_the_applied_index() {
        let mut camera = ButterflyCamera::new();
        camera.selected_index = 3;
        let (position, rotation) = camera.update(0.016);
        assert_eq!(camera.applied_index, 3);
        assert!(position.is_finite() && rotation.is_finite());
        camera.selected_index = 3;
        let _ = camera.update(0.016);
        assert_eq!(camera.applied_index, 3);
    }
}
