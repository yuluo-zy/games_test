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
#[derive(Resource, Clone, Copy, Debug)]
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

/// B insert_resources@140940ddc..e0b stores origin0 and direction(-0,-0,-1).
impl Default for CameraCursorRay {
    fn default() -> Self {
        Self::new()
    }
}
impl CameraCursorRay {
    pub fn new() -> Self {
        Self {
            origin: Vec3::ZERO,
            dir: Vec3::new(-0.0, -0.0, -1.0),
        }
    }
    pub fn ray(self) -> ::utils::geometry::Ray {
        ::utils::geometry::Ray {
            origin: self.origin,
            direction: self.dir,
        }
    }
}

/// Original camera_mouse_ray@1408ef1c0, Update10 first chain member.
/// Always refresh the unprojected ray, then drain this system's cursor reader.
/// Hardware movement or nonzero keyboard pan/height emits one unit audio event.
pub fn camera_mouse_ray(
    cursor: bevy_ecs::prelude::Res<CursorPositionSS>,
    camera: bevy_ecs::prelude::Res<crate::systems::camera_rig::MainCamera>,
    window: bevy_ecs::prelude::Res<super::window::WindowSize>,
    mut ray: bevy_ecs::prelude::ResMut<CameraCursorRay>,
    mut moved: bevy_ecs::prelude::EventReader<super::window::CursorMovedScreenSpace>,
    actions: bevy_ecs::prelude::Res<crate::input::action_set::MainActionSet>,
    mut audio: bevy_ecs::prelude::EventWriter<super::window::AudioCursorMoved>,
) {
    let updated = ::utils::geometry::Ray::from_mouse_ss(
        cursor.0,
        window.as_vec2(),
        camera.read_matrix(0x40),
        camera.read_matrix(0xc0),
    );
    *ray = CameraCursorRay {
        origin: updated.origin,
        dir: updated.direction,
    };
    if moved.read().last().is_some()
        || actions.analog[3].length() > 0.0
        || actions.analog[2].length() > 0.0
    {
        audio.write(super::window::AudioCursorMoved);
    }
}

/// Original cursor state writes1408ef6a0..700 /1408ef83a..89e. A new hit
/// after a miss does not clear old delta/speed; zero or NaN dt retains speed.
fn store_cursor_hit(
    cursor: &mut super::cursor_terrain_raycast::CursorTerrainRaycast,
    position: Vec3,
    delta_seconds: f32,
) {
    if cursor.tag == 1 {
        cursor.delta = position - cursor.pos;
        if delta_seconds > 0.0 || delta_seconds < 0.0 {
            cursor.speed = cursor.delta.length() / delta_seconds;
        }
    }
    cursor.tag = 1;
    cursor.pos = position;
}

/// Original mouse_terrain_raycast@1408ef320. The Option field is the
/// canonical TerrainEditingData +0/+4 editing-plane owner, not a new resource.
/// Its producer/constructor must be restored in that same owner before mounting.
pub fn mouse_terrain_raycast(
    mut followers: bevy_ecs::prelude::Query<(
        &mut crate::components::FollowMouse,
        &mut crate::utils::components::transform::Transform,
    )>,
    mut cursor: bevy_ecs::prelude::ResMut<super::cursor_terrain_raycast::CursorTerrainRaycast>,
    heights: bevy_ecs::prelude::Res<super::terrain_height::TerrainHeightsData>,
    editing: bevy_ecs::prelude::Res<super::terrain_editing::TerrainEditingData>,
    captured: bevy_ecs::prelude::Res<super::mouse_capture::CursorCapturedPosition>,
    ray: bevy_ecs::prelude::Res<CameraCursorRay>,
    time: bevy_ecs::prelude::Res<bevy_time::Time>,
) {
    if captured.0.is_some() {
        return;
    }
    let hit = if let Some(y) = editing.cursor_height_override {
        Some(ray.intersect_with_xz_plane(y))
    } else {
        heights.intersect_ray(ray.ray(), f32::MAX).map(|hit| {
            let plane = ray.intersect_with_xz_plane(0.0);
            if (hit.position - ray.origin).length() > (plane - ray.origin).length() {
                plane
            } else {
                hit.position
            }
        })
    };
    if let Some(position) = hit {
        store_cursor_hit(&mut cursor, position, time.delta_secs());
    }
    for (follow, mut transform) in &mut followers {
        if follow.0 {
            transform.translation = ray.origin + ray.dir * 45.0;
        } else if let Some(position) = hit {
            transform.translation = position;
        }
    }
}

#[cfg(test)]
mod original_ray_tests {
    use super::*;
    use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
    #[test]
    fn native_ray_constructor_preserves_negative_zero_direction() {
        let ray = CameraCursorRay::new();
        assert_eq!(ray.origin, Vec3::ZERO);
        assert_eq!(ray.dir.x.to_bits(), (-0.0f32).to_bits());
        assert_eq!(ray.dir.y.to_bits(), (-0.0f32).to_bits());
        assert_eq!(ray.dir.z, -1.0);
    }
    #[test]
    fn native_cursor_hit_keeps_speed_for_zero_or_nan_delta() {
        let mut cursor = CursorTerrainRaycast {
            tag: 1,
            pos: Vec3::ZERO,
            delta: Vec3::ZERO,
            speed: 7.0,
        };
        store_cursor_hit(&mut cursor, Vec3::X, 0.0);
        assert_eq!(cursor.delta, Vec3::X);
        assert_eq!(cursor.speed, 7.0);
        store_cursor_hit(&mut cursor, Vec3::X * 2.0, f32::NAN);
        assert_eq!(cursor.speed, 7.0);
        store_cursor_hit(&mut cursor, Vec3::X * 3.0, 0.5);
        assert_eq!(cursor.speed, 2.0);
    }
    #[test]
    fn first_native_hit_preserves_existing_delta_and_speed() {
        let mut cursor = CursorTerrainRaycast {
            tag: 0,
            pos: Vec3::ZERO,
            delta: Vec3::Y,
            speed: 7.0,
        };
        store_cursor_hit(&mut cursor, Vec3::X, 0.5);
        assert_eq!(cursor.delta, Vec3::Y);
        assert_eq!(cursor.speed, 7.0);
        assert_eq!(cursor.pos, Vec3::X);
    }
}
