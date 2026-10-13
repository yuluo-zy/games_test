//! mouse_terrain_raycast.rs — 鼠标→地形射线与相机光标射线（WP8 恢复）。
//!
//! 源：B skeleton `resources/mouse_terrain_raycast.rs`。
//! 反编译：`CameraCursorRay::intersect_with_xz_plane`@0x1408ef180（行 27）、`camera_mouse_ray`@0x1408ef1c0
//! （行 35）、`mouse_terrain_raycast`@0x1408ef320（行 63）、`closure$0`@0x1408efa70（行 77）。
//! 布局（O）：`CameraCursorRay { tag: u32@0x00(=1 有效), pos: Vec3@0x04, dir: Vec3@0x10, length: f32@0x1c }`。
//! `mouse_terrain_raycast`：已有效→xz 平面；否则 `TerrainHeightsData::intersect_ray<TerrainHeightSsbo>`。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;
use glam::Vec3;

use crate::typed_gap::Gap;

/// 相机光标射线（O 布局）。`tag==1` 表示已有效；length 为命中距离（SQRT/…，O）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct CameraCursorRay {
    pub tag: u32,
    pub pos: Vec3,
    pub dir: Vec3,
    pub length: f32,
}

impl CameraCursorRay {
    /// 与 y 高度的 xz 平面求交（O：`t=(y-pos.y)/dir.y; pos+t*dir`）。
    pub fn intersect_with_xz_plane(&self, y: f32) -> Vec3 {
        let t = (y - self.pos.y) / self.dir.y;
        self.pos + self.dir * t
    }
}

/// 屏幕空间光标位置（O 名；`camera_mouse_ray` 读取）。布局 U。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct CursorPositionSS(pub Gap);

/// 从鼠标屏幕位置构建世界射线（O：`utils::geometry::ray::Ray::from_mouse_ss`）。
pub fn camera_mouse_ray(
    _cursor: &CursorPositionSS,
    _camera: &Gap,
    _raycast: &mut CameraCursorRay,
    _frame_tick: u32,
) {
    unimplemented!("recover camera_mouse_ray via utils::geometry::ray::Ray::from_mouse_ss")
}
