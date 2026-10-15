//! B WindowSize stores two u32 values (set 0x140949c00). as_vec2
//! (0x140949c10) zero-extends and converts each to f32; it is not a bitcast.

use bevy_ecs::resource::Resource;
use glam::Vec2;

#[repr(C)]
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl WindowSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn set(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn as_dims(self) -> [u32; 2] {
        [self.width, self.height]
    }

    pub fn as_vec2(self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }

    pub fn aspect_ratio(self) -> f32 {
        aspect_ratio_observed(self.width, self.height)
    }
}

/// 对应原函数从 RCX+0、RCX+4 读取的数值。
/// Preserve x86's negative quiet NaN for 0/0 on targets with a positive NaN.
#[inline(never)]
pub fn aspect_ratio_observed(field_0x00: u32, field_0x04: u32) -> f32 {
    let ratio = (field_0x00 as f32) / (field_0x04 as f32);
    if ratio.is_nan() {
        f32::from_bits(0xffc0_0000)
    } else {
        ratio
    }
}
