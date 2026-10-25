//! brush_sizes.rs — 路径/水/花园三个工具的笔刷尺寸（`resources/brush_sizes.rs`）。
//!
//! 6 个函数全部直读汇编（0x1409f1860/70/90/a0/c0/d0）：`reset_*` 把默认值写进字段，
//! `set_*` 先 `max(下限, x)` 再 `min(上限, ·)` 后写入。
//! 字段偏移 path@+0、water@+4、garden@+8（代码此前把 water 放首位，是错的）。

use bevy_ecs::resource::Resource;
use ::utils::scalar::clamp;

/// 路径工具范围：默认 0.8（0x3f4ccccd），[0.32, 1.2]（0x142ac56c0 / 0x1429cd128）。
/// 下限的位模式是 0x3ea3d70b，比字面量 `0.32_f32`（0x3ea3d70a）大 1 ULP，所以按位写。
pub const PATH_DEFAULT: f32 = 0.8;
pub const PATH_RANGE: (f32, f32) = (f32::from_bits(0x3ea3_d70b), f32::from_bits(0x3f99_999a));
/// 水工具：默认 2.0（0x40000000），[1.0, 5.0]（0x142925904 / 0x142925988）。
pub const WATER_DEFAULT: f32 = 2.0;
pub const WATER_RANGE: (f32, f32) = (1.0, 5.0);
/// 花园工具：默认 3.0（0x40400000），[2.0, 4.0]（0x14292e6f0 / 0x142934694）。
pub const GARDEN_DEFAULT: f32 = 3.0;
pub const GARDEN_RANGE: (f32, f32) = (2.0, 4.0);

fn clamp_like_exe((lower, upper): (f32, f32), value: f32) -> f32 {
    clamp(value, lower, upper)
}

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct BrushSizes {
    pub path: f32,
    pub water: f32,
    pub garden: f32,
}

impl BrushSizes {
    /// O：0x1409f1860。
    pub fn reset_path(&mut self) {
        self.path = PATH_DEFAULT;
    }

    /// O：0x1409f1870。
    pub fn set_path(&mut self, size: f32) {
        self.path = clamp_like_exe(PATH_RANGE, size);
    }

    /// O：0x1409f1890。
    pub fn reset_water(&mut self) {
        self.water = WATER_DEFAULT;
    }

    /// O：0x1409f18a0。
    pub fn set_water(&mut self, size: f32) {
        self.water = clamp_like_exe(WATER_RANGE, size);
    }

    /// O：0x1409f18c0。
    pub fn reset_garden(&mut self) {
        self.garden = GARDEN_DEFAULT;
    }

    /// O：0x1409f18d0。
    pub fn set_garden(&mut self, size: f32) {
        self.garden = clamp_like_exe(GARDEN_RANGE, size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zero() -> BrushSizes {
        BrushSizes {
            path: 0.0,
            water: 0.0,
            garden: 0.0,
        }
    }

    #[test]
    fn layout_is_path_water_garden() {
        assert_eq!(std::mem::size_of::<BrushSizes>(), 12);
        assert_eq!(std::mem::offset_of!(BrushSizes, path), 0);
        assert_eq!(std::mem::offset_of!(BrushSizes, water), 4);
        assert_eq!(std::mem::offset_of!(BrushSizes, garden), 8);
    }

    #[test]
    fn resets_write_the_exe_immediates() {
        let mut sizes = zero();
        sizes.reset_path();
        sizes.reset_water();
        sizes.reset_garden();
        assert_eq!(sizes.path.to_bits(), 0x3f4c_cccd);
        assert_eq!(sizes.water.to_bits(), 0x4000_0000);
        assert_eq!(sizes.garden.to_bits(), 0x4040_0000);
    }

    #[test]
    fn setters_clamp_to_the_exe_ranges_and_touch_one_field() {
        let mut sizes = zero();
        sizes.set_path(10.0);
        assert_eq!(sizes.path, f32::from_bits(0x3f99_999a));
        sizes.set_path(-1.0);
        assert_eq!(sizes.path.to_bits(), 0x3ea3_d70b);
        assert_ne!(sizes.path.to_bits(), 0.32_f32.to_bits());
        sizes.set_water(0.0);
        assert_eq!(sizes.water, 1.0);
        sizes.set_water(9.0);
        assert_eq!(sizes.water, 5.0);
        sizes.set_garden(2.5);
        assert_eq!(sizes.garden, 2.5);
        sizes.set_garden(100.0);
        assert_eq!(sizes.garden, 4.0);
        assert_eq!(sizes.water, 5.0);
    }

    #[test]
    fn nan_survives_both_clamps_like_maxss_minss() {
        let mut sizes = zero();
        sizes.set_path(f32::NAN);
        sizes.set_water(f32::NAN);
        sizes.set_garden(f32::NAN);
        assert!(sizes.path.is_nan() && sizes.water.is_nan() && sizes.garden.is_nan());
    }

    #[test]
    fn range_constants_match_the_exe_float_bits() {
        assert_eq!(PATH_RANGE.0.to_bits(), 0x3ea3_d70b);
        assert_eq!(PATH_RANGE.1.to_bits(), 0x3f99_999a);
    }
}
