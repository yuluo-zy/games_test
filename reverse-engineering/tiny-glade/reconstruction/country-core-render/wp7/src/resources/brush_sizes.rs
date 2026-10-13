//! brush_sizes.rs — 笔刷尺寸资源的水笔刷切片（WP7 恢复）。
//!
//! 源（B skeleton）：`crates/country-core/src/resources/brush_sizes.rs`。
//! `BrushSizes` 为通用笔刷尺寸资源（多工具共用）；本模块只恢复水笔刷字段
//! （`draw_water` 0x141ba9130 读取，O）。
//!
//! O 证据：
//! - `set_water`（0x1409f18a0）：`self+4 = clamp(value, 1.0, 5.0)`
//!   （0x3f800000 / 0x40a00000）。
//! - `reset_water`（0x1409f1890）：`self+4 = 2.0`（0x40000000）。
//! - `draw_water` 0x141ba9130 水工具分支：`fVar11 = *(float*)(brush + 4)`（读水笔刷）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

/// 笔刷尺寸资源（O：`BrushSizes::water` 位于 **+4** 的 f32；其余槽位 U）。
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct BrushSizes {
    /// 水笔刷尺寸（O：self+4；默认 2.0，clamp[1.0, 5.0]）。+0 槽位属其他工具（U）。
    pub water: f32,
    /// typed gap：其他工具笔刷尺寸（+0/+8…）U。
    pub _gap: crate::typed_gap::Gap,
}

impl Default for BrushSizes {
    /// 默认水笔刷 2.0（O：`reset_water` 0x40000000）。
    fn default() -> Self {
        BrushSizes {
            water: 2.0,
            _gap: crate::typed_gap::Gap,
        }
    }
}

impl BrushSizes {
    /// 设水笔刷（O：`clamp(value, 1.0, 5.0)`）。
    pub fn set_water(&mut self, value: f32) {
        self.water = value.clamp(1.0, 5.0);
    }
    /// 重置水笔刷（O：2.0）。
    pub fn reset_water(&mut self) {
        self.water = 2.0;
    }
}
