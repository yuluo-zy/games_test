//! compute_water.rs — W2 water_rasterize_from_cpu, W13 water_spawn_vegetation（WP7 恢复）。
//!
//! 反编译（O）：`water_rasterize_from_cpu` = 0x140a12680（size 3223）；`water_spawn_vegetation`
//! = 0x140a13970（size 2269）。帧接线：W2 / W13（frame-wiring.md）。
//!
//! - **W2**：CPU 水位栅格 → 水 mask 纹理。`WaterRasterStagingSsbo`（type-id
//!   `0x1ad182936286a0ee`）存在且 `WaterRaster+0x29b` 脏 → 写 `ssbo+0x5c=world+0x5b0`、
//!   清 0x29b、`memset(0x98,0,0x200)` → 4 个 compute pass（`_water_draw_water_mask` 族，
//!   每 pass `write_uniform<f32>` + `write_uniform<i32>` + dispatch）。
//! - **W13**：两个 ComputePass（lilypad+lilypad_flower；reed）——`write_storage_buffer` +
//!   纹理绑定（preset 槽 9/10/0x13/5/0x14/0x17 与 9/10/0x13/5/0x16）+
//!   `write_storage_buffer_concurrent<DrawIndexedIndirectBuffer>` +
//!   `write_uniform<bool>`（`ActiveSession::is_autumn`）+ `write_uniform<i32>` + dispatch。
//!   boats/waterwheels 的 URI 动态拼接、位置并入本 pass 或独立（U，见 spawn_boats）。

#![allow(dead_code, unused_imports)]

use crate::consume::buffers::DrawIndexedIndirectBuffer;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::resources::session::ActiveSession;
use crate::resources::water_mask::{WATER_MASK_GRID, WATER_MASK_SAMPLE_SCALE};
use crate::resources::world_raster::WorldRaster;
use crate::resources::world_raster_defs::{WaterRaster, WaterRasterStagingSsbo};
use crate::typed_gap::Gap;

/// 水植被实例 SSBO（O 名；布局 U，compute_water 域）。
pub struct LilypadSsbo(pub Gap);
pub struct LilypadFlowerSsbo(pub Gap);
pub struct ReedSsbo(pub Gap);

impl SsboType for LilypadSsbo {
    /// typed gap：原值 = `utils::name_hash("…LilypadSsbo")`。
    fn rg_buffer_hash() -> u64 {
        0
    }
}
impl SsboType for LilypadFlowerSsbo {
    fn rg_buffer_hash() -> u64 {
        0
    }
}
impl SsboType for ReedSsbo {
    fn rg_buffer_hash() -> u64 {
        0
    }
}

/// 世界→水栅格变换（O：`(world / 130.0 + 0.5) * 512.0`，`rasterize_stroke<512>`
/// 0x142261d10；世界窗口约 130 单位 → 512 格，原点对应格 256）。
pub fn world_to_water_grid(world_xz: glam::Vec2) -> glam::Vec2 {
    (world_xz / 130.0 + 0.5) * WATER_MASK_GRID as f32
}

/// W2 — CPU staging → 水 mask 纹理（O：0x140a12680）。
/// 若 `WaterRasterStagingSsbo` 存在且 `WaterRaster` 再上传脏字节（+0x29b）：
/// 清 staging（0x200B）+ 写 `ssbo+0x5c = world+0x5b0`，经 4 个 `_water_draw_water_mask`
/// compute pass 上传为 512×512 水 mask 纹理。
pub fn water_rasterize_from_cpu(
    _world: &mut Gap,
    _staging: &WaterRasterStagingSsbo,
    _water: &WaterRaster,
) {
    // O：get_resource_id(0x1ad182936286a0ee) → unwrap_failed；+0x29b 脏判定 →
    //   ComputePass::new + write_uniform<f32>/<i32> + dispatch×4。
    // 纹理槽：preset 9/10/0x13（水 mask 输出）。rhapsody PassBuilder 接入边界 todo。
    let _ = WATER_MASK_SAMPLE_SCALE;
    unimplemented!()
}

/// W13 — 水面植被生成（O：0x140a13970）。
/// Pass 1（lilypad + lilypad_flower）：`write_storage_buffer<LilypadSsbo>/
/// <LilypadFlowerSsbo>` + `ActiveSession` 断言（type-id 0x74430dfcc6d8719b）+
/// `write_uniform<bool>`（= `is_autumn`）+ 纹理 9/10/0x13/5/0x14/0x17 +
/// `write_storage_buffer_concurrent<DrawIndexedIndirectBuffer>` + `write_uniform<i32>` + dispatch。
/// Pass 2（reed）：同形，`write_storage_buffer<ReedSsbo>` + 纹理 9/10/0x13/5/0x16 + dispatch。
pub fn water_spawn_vegetation(_world: &mut Gap, _session: &ActiveSession) {
    // O：demo 守卫；两 ComputePass；dispatch 常量 0xe100000001/0x1000000e1 与
    //   0x9600000001/0x100000096（1D 间接 dispatch 形）。
    unimplemented!()
}

/// 水位双线性采样入口（O：`bilinear_sample_at_world_xz<RasterBufferRef<512>>`，
/// u8 单元 512 列；`<=0.01` 判无水——`nani_update_preplaced_flower`）。
/// 数值语义（0..255 水位码值）U；此处以 `WorldRaster` typed-gap 呈现。
pub fn water_level_at_xz(_raster: &WorldRaster<512>, _world_xz: glam::Vec2) -> f32 {
    0.0
}
