//! cursor_terrain_raycast.rs — 光标地形射线（WP8 恢复）。
//!
//! 源：B skeleton `resources/cursor_terrain_raycast.rs`。
//! 反编译：`CursorTerrainRaycast::position`@0x140a12650（行 35）：`(*tag & 1)!=0` 时返回
//! 位置 Vec3（+0x04/+0x0c），否则 {0,0,0}。方法行号：`update`@20、`new`@39。
//! 布局（O）：`{ tag: u8@0x00, pos: Vec3@0x04 }`，Option<Vec3> 语义（I）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;
use glam::Vec3;

/// 光标地形射线（O 布局；`tag` 为 Some 标记）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct CursorTerrainRaycast {
    pub tag: u8,
    pub pos: Vec3,
}

impl CursorTerrainRaycast {
    /// 新建（O 名；`new`@39）。
    pub fn new() -> Self {
        Self { tag: 0, pos: Vec3::ZERO }
    }

    /// 更新（O 名；`update`@20）。输入语义 U。
    pub fn update(&mut self, _hit: Option<Vec3>) {
        match _hit {
            Some(pos) => {
                self.tag = 1;
                self.pos = pos;
            }
            None => {
                self.tag = 0;
                self.pos = Vec3::ZERO;
            }
        }
    }

    /// 返回位置（O：`position`@0x140a12650 读 +4/+0xc）。
    pub fn position(&self) -> Vec3 {
        if (self.tag & 1) != 0 {
            self.pos
        } else {
            Vec3::ZERO
        }
    }
}
