//! B Roof component and its observed 0x58-byte game fields.
//! Geometry is shared with the recovered system-roof crate. The byte projection
//! preserves appearance fields whose original Rust enum schema is not yet known.

use crate::resources::walls::WallId;
use bevy_ecs::component::Component;
use glam::Vec3;

#[derive(Component, Clone, Debug)]
pub struct Roof(pub(crate) [u8; 88]);

impl Roof {
    pub fn from_observed(bytes: [u8; 88]) -> Self {
        Self(bytes)
    }
    pub fn observed(&self) -> &[u8; 88] {
        &self.0
    }
    pub fn wall_id(&self) -> WallId {
        WallId(u64::from_le_bytes(self.0[..8].try_into().unwrap()))
    }
    pub fn floor_y(&self) -> f32 {
        self.float_at(0x4c)
    }
    pub fn height_01(&self) -> f32 {
        self.float_at(0x30)
    }
    pub fn is_pitched(&self) -> bool {
        ty_observed(self.height_01())
    }
    pub fn is_flat(&self) -> bool {
        self.height_01() <= f32::from_bits(0x3dcccccd)
    }
    pub fn is_flat_w_no_rim(&self) -> bool {
        self.is_flat() && self.0[0x3d] == 0
    }
    pub fn is_gable(&self) -> bool {
        is_gable_observed(self.height_01(), self.float_at(0x34))
    }
    pub fn gable_sides(&self) -> Option<[u8; 2]> {
        self.is_gable().then_some([self.0[0x3c], self.0[0x3c] | 2])
    }
    pub fn height(&self) -> f32 {
        system_roof::surface::height(&self.0)
    }
    pub fn pivot_3d(&self) -> Vec3 {
        let tag = u32::from_le_bytes(self.0[8..12].try_into().unwrap()) as usize;
        assert!(tag < 2, "unsupported Roof enclosure");
        Vec3::new(
            self.float_at(12 + tag * 8),
            self.floor_y(),
            self.float_at(16 + tag * 8),
        )
    }
    pub fn shape_dims(&self) -> [f32; 2] {
        if self.0[8] & 1 == 0 {
            let diameter = self.float_at(0x14) + self.float_at(0x14);
            [diameter; 2]
        } else {
            [self.float_at(0x1c), self.float_at(0x20)]
        }
    }
    pub fn shape_at(&self, coordinate: f32) -> system_roof::surface::RoofSection {
        system_roof::surface::section(&self.0, coordinate)
    }
    pub fn parameters(&self) -> system_roof::parameters::RoofParameterBlock {
        system_roof::parameters::RoofParameterBlock::from_roof(&self.0)
    }
    fn float_at(&self, offset: usize) -> f32 {
        f32::from_le_bytes(self.0[offset..offset + 4].try_into().unwrap())
    }
}

pub fn ty_observed(value_at_0x30: f32) -> bool {
    let threshold = f32::from_bits(0x3dcccccd);
    // UCOMISS + SETC: 无序比较（NaN）也设置 CF。
    threshold < value_at_0x30 || threshold.is_nan() || value_at_0x30.is_nan()
}

pub fn is_gable_observed(value_at_0x30: f32, value_at_0x34: f32) -> bool {
    // CMPNLEPS（含无序比较）与 CMPEQPS 的低 lane 按位相与。
    ty_observed(value_at_0x30) && value_at_0x34 == 1.0
}

// 原入口的八参数观察合同保持不变。
#[allow(clippy::too_many_arguments)]
pub fn new_observed(
    output: &mut [u8; 0x58],
    arg2: &[u8; 8],
    arg3: &[u8; 28],
    arg4: &[u8; 28],
    arg5: &[u8; 4],
    arg6: &[u8; 4],
    arg7: u8,
    arg8: &[u8; 12],
) {
    output[0..8].copy_from_slice(arg2);
    output[0x24..0x40].copy_from_slice(arg3);
    output[8..0x24].copy_from_slice(arg4);
    output[0x4c..0x50].copy_from_slice(arg5);
    output[0x50..0x54].copy_from_slice(arg6);
    output[0x54] = arg7;
    output[0x40..0x4c].copy_from_slice(arg8);
    // 原作没有写入 +0x55..+0x57。0x58 来自调用者组件步长，非完整 Rust 类型证明。
}
