//! read_terrain_data.rs — W5 CPU 地形/水面回读（WP7 补充水部分）。
//!
//! 反编译（O）：0x14098c680（size 4998）。URIs：`/terrain_editing/read_terrain_height.cs.hlsl`、
//! `/terrain_editing/read_terrain_rock_mask.cs.hlsl`、`/water/read_water_mask.cs.hlsl`、
//! `/garden/read_garden_mask.cs.hlsl`。`read_terrain_data` 组合各回读 +
//! `ScheduledTerrainReadback::on_gpu_copy_issued`（type-id 0xfc8691076ab4d826，O）。
//!
//! **水部分（O）**：`if (_DEV_OVERRIDE_IS_DEMO_MODE == 0)` 分支，ComputePass 绑
//! 水 mask 纹理槽 **0x13**，`write_storage_buffer<WaterMaskSsbo>` +
//! `write_storage_buffer<WaterExcludeClutterMaskSsbo>`，dispatch
//! （`0x200000000001 / 0x100000001`，1D 回读）。即 `_water_read_water_mask.cs`
//! 一次回读打包水/排除两缓冲（O：SPIR-V `__alias_mask_out`/`__alias_mask_out_clutter`）。

#![allow(dead_code, unused_imports)]

use crate::resources::water_exclude_clutter_mask::WaterExcludeClutterMaskSsbo;
use crate::resources::water_mask::{WaterMaskSsbo, WATER_BIT_THRESHOLD, CLUTTER_BIT_THRESHOLD};
use crate::typed_gap::Gap;

/// 水 mask 回读打包位判定（O：`read_water_mask.cs.hlsl`
/// `water_mask[coord].y > 0.5 → water bit`；`> 0.2 → clutter bit`；32 texel/thread）。
pub fn pack_water_mask_bits(coverage_y: f32) -> (bool, bool) {
    (
        coverage_y > WATER_BIT_THRESHOLD,
        coverage_y > CLUTTER_BIT_THRESHOLD,
    )
}

/// W5 — 组合地形/水面回读（O：0x14098c680）。
pub fn read_terrain_data(_world: &mut Gap) {
    // O：地形高度（槽 9/10 → TerrainHeightSsbo/VisualTerrainHeightSsbo）、
    //   rock mask（槽 0xe → TerrainRockMaskSsbo）、水 mask（槽 0x13 →
    //   WaterMaskSsbo/WaterExcludeClutterMaskSsbo，demo 守卫）、garden（槽 0x14 →
    //   GardenMaskSsbo）；尾部 on_gpu_copy_issued。
    unimplemented!()
}

fn read_terrain_height(_world: &Gap, _height: &Gap) {
    // O：bind_terrain_heightmap + dispatch_1d。
    unimplemented!()
}

fn read_terrain_rock_mask(_world: &Gap, _mask: &Gap) {
    // O：read_texture + dispatch_1d。
    unimplemented!()
}

fn read_water_mask(_world: &Gap, _water: &WaterMaskSsbo, _exclude: &WaterExcludeClutterMaskSsbo) {
    // O：read_texture(水 mask) + write_storage_buffer×2 + dispatch_1d。
    unimplemented!()
}

fn read_garden_mask(_world: &Gap, _mask: &Gap) {
    // O：read_texture + dispatch_1d。
    unimplemented!()
}
