use bevy_ecs::prelude::Resource;
use glam::{Vec2, Vec3};

/// Screen-space f32 position read by B 0x140b17a80. This is distinct from the
/// world-space CursorTerrainRaycast, and from integer WindowSize dimensions.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct CursorPositionSS(pub Vec2);

/// 相机光标射线（O：`resources/mouse_terrain_raycast.rs`；`intersect_with_xz_plane`
/// @0x1408ef180，组 A restorer 接线——`system_water::draw_water` 的 `Res<CameraCursorRay>`
/// 参数面与 `system_tree` 各 UI 系统消费）。
///
/// 布局（O，0x1408ef180 读取面）：`origin: Vec3 @+0x00`、`dir: Vec3 @+0x0c`
/// （字段名 I）；`intersect_with_xz_plane(plane_y)` 计算 `origin + t·dir`（`t = (plane_y - o.y)/d.y`，
/// 机器码 `fVar5 = (in_XMM2_Da - fVar3) / fVar4`；无除零保护，调用点保证 `d.y != 0`）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct CameraCursorRay {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl CameraCursorRay {
    /// `CameraCursorRay::intersect_with_xz_plane`（0x1408ef180，O）。
    pub fn intersect_with_xz_plane(&self, plane_y: f32) -> Vec3 {
        let t = (plane_y - self.origin.y) / self.dir.y;
        Vec3::new(
            self.origin.x + t * self.dir.x,
            self.origin.y + t * self.dir.y,
            self.origin.z + t * self.dir.z,
        )
    }
}
