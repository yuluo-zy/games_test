//! terrain_rock_mask.rs — 岩体掩码 512×512 位栅格（WP8 恢复）。
//!
//! 源：B skeleton `resources/terrain_rock_mask.rs`。
//! 反编译：`impl$1::sample_at_world_xz`@0x1409ea800（行 43）：
//!   `(x/130 + 0.5)` 截断 [0,1] → ×512 → 索引；`(y*512 | x) >> 5 < len` 边界检查。
//! 布局（O）：岩体掩码为按位打包 u32 数组（512×512 = 8192 word = 32KB）。世界范围 ±130。
//! `read_terrain_data`@0x14098c680 `write_storage_buffer<TerrainRockMaskSsbo>` 从 GPU 回拷。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 岩体掩码 SSBO 标记类型（O：`read_terrain_data` `write_storage_buffer<TerrainRockMaskSsbo>`）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainRockMaskSsbo;

/// 岩体掩码数据（O 语义：512×512 位，按位打包；世界范围 ±130，f32→像素 `x/130+0.5`）。
/// 布局 U → typed gap。
#[derive(Resource, Clone, Debug, Default)]
pub struct TerrainRockMask {
    /// typed gap：{u32 位图数组, 世界范围} 字段 U。
    pub _gap: Gap,
}

impl TerrainRockMask {
    /// 世界 XZ → 岩体存在位（O：`sample_at_world_xz`@0x1409ea800；位索引 `(z*512|x)`，word `idx>>5`）。
    pub fn sample_at_world_xz(&self, x: f32, z: f32) -> bool {
        let fx = (x / 130.0 + 0.5).clamp(0.0, 1.0) * 512.0;
        let fz = (z / 130.0 + 0.5).clamp(0.0, 1.0) * 512.0;
        let (ix, iz) = (fx as usize, fz as usize);
        let idx = iz * 512 + ix;
        let _ = idx >> 5; // 边界检查依赖 word 长度（O）；布局 U
        false
    }
}
