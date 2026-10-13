//! `DirectionalLight` — sun light with a look-at view basis (layer A1).
//!
//! 证据（O，逐条核对机器码，见 `evidence/country-core-audit/L0_NOTES.md`）：
//!  * `look_at` 0x140b18550（725 B）：**构造函数**，`sret=rcx, eye=&Vec3(rdx), target=&Vec3(r8)`。
//!  * `set_direction_towards_light` 0x140b18830（695 B）：`self=rcx, dir=&Vec3(rdx)`。
//!  * 二者的矩阵部分展开后都是 glam `Mat4::look_at_rh(eye, target, Vec3::Y)`：
//!    `f = normalize(target-eye)`（乘倒数）、`s = normalize(f × Y)`、`u = s × f`，
//!    列 = `(s.x,u.x,-f.x,0) (s.y,u.y,-f.y,0) (s.z,u.z,-f.z,0) (-s·eye,-u·eye, f·eye, 1)`，
//!    与指令序列（含 `f·eye` 取正号、第三行取 `-f`）一致。
//!  * 布局（sret 的 movaps/movlps/movss 偏移）：view Mat4 @0x00，forward Vec3 @0x40，position Vec3 @0x4c，
//!    (1,1,1) Vec3 @0x58，target Vec3 @0x64，distance f32 @0x70；movaps 要求 16 对齐，尺寸 0x80。
//!    旧版把 +0x40 当 position、+0x60 当 `1+target`，字段与偏移都不对。

use glam::{Mat4, Vec3};

/// 太阳方向光。
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectionalLight {
    /// +0x00：`look_at_rh(position, target, Y)`。
    pub view: Mat4,
    /// +0x40：`normalize(target - position)`（光线传播方向）。
    pub forward: Vec3,
    /// +0x4c：光源位置（eye）。
    pub position: Vec3,
    /// +0x58：常量 (1,1,1)，`look_at` 写入、`set_direction_towards_light` 不动。含义 U（颜色/缩放之一）。
    pub tint: Vec3,
    /// +0x64：被照射的目标点。
    pub target: Vec3,
    /// +0x70：`|eye - target|`。
    pub distance: f32,
}

const _: () = assert!(core::mem::size_of::<DirectionalLight>() == 0x80);

/// 两处共用的矩阵与方向：`(view, forward)`。
fn look_basis(eye: Vec3, target: Vec3) -> (Mat4, Vec3) {
    let forward = (target - eye).normalize();
    (Mat4::look_at_rh(eye, target, Vec3::Y), forward)
}

impl DirectionalLight {
    /// `look_at(eye, target)`（O 0x140b18550）。`distance` 取 `|eye - target|`（独立计算，与 forward 的长度同源）。
    pub fn look_at(eye: Vec3, target: Vec3) -> Self {
        let (view, forward) = look_basis(eye, target);
        Self { view, forward, position: eye, tint: Vec3::ONE, target, distance: (eye - target).length() }
    }

    /// `set_direction_towards_light(dir)`（O 0x140b18830）：
    /// `position = target + normalize(dir) * distance`，随后按新位置重建 `view`/`forward`；
    /// `target`、`distance`、`tint` 不变。
    pub fn set_direction_towards_light(&mut self, direction: Vec3) {
        let eye = self.target + direction.normalize() * self.distance;
        let (view, forward) = look_basis(eye, self.target);
        self.position = eye;
        self.view = view;
        self.forward = forward;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: Vec3, b: Vec3) -> bool {
        (a - b).abs().max_element() < 1e-4
    }

    #[test]
    fn layout_matches_original() {
        use core::mem::{offset_of, size_of};
        assert_eq!(size_of::<DirectionalLight>(), 0x80);
        assert_eq!(offset_of!(DirectionalLight, forward), 0x40);
        assert_eq!(offset_of!(DirectionalLight, position), 0x4c);
        assert_eq!(offset_of!(DirectionalLight, tint), 0x58);
        assert_eq!(offset_of!(DirectionalLight, target), 0x64);
        assert_eq!(offset_of!(DirectionalLight, distance), 0x70);
    }

    #[test]
    fn look_at_maps_eye_to_origin_and_target_down_minus_z() {
        let eye = Vec3::new(3.0, 8.0, -2.0);
        let target = Vec3::new(-1.0, 0.5, 4.0);
        let l = DirectionalLight::look_at(eye, target);
        assert!(close(l.view.transform_point3(eye), Vec3::ZERO));
        assert!(close(l.view.transform_point3(target), Vec3::new(0.0, 0.0, -(eye - target).length())));
        assert!(close(l.forward, (target - eye).normalize()));
        assert_eq!(l.tint, Vec3::ONE);
        assert!((l.distance - (eye - target).length()).abs() < 1e-5);
    }

    #[test]
    fn set_direction_keeps_distance_and_target() {
        let mut l = DirectionalLight::look_at(Vec3::new(0.0, 10.0, 0.0), Vec3::new(1.0, 0.0, 1.0));
        let (t, d, tint) = (l.target, l.distance, l.tint);
        l.set_direction_towards_light(Vec3::new(2.0, 2.0, 0.0));
        assert_eq!((l.target, l.distance, l.tint), (t, d, tint));
        assert!(close(l.position, t + Vec3::new(2.0, 2.0, 0.0).normalize() * d));
        assert!(close(l.forward, -Vec3::new(2.0, 2.0, 0.0).normalize()));
        assert!(close(l.view.transform_point3(l.position), Vec3::ZERO));
    }
}
