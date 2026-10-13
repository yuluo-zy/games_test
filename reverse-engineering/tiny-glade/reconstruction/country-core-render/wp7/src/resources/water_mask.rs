//! water_mask.rs — `country_core::resources::water_mask` 水 mask 资源（WP7 恢复）。
//!
//! 源（B skeleton）：`crates/country-core/src/resources/water_mask.rs`（metadata-only，
//! 5 记录，0 procedure）。PDB 命名实体 `WaterMaskSsbo` `name_and_relations_only`。
//! 语义（I，`reconstruction/shader-source/shaders/water/read_water_mask.cs.hlsl` O）：
//! 512×512 水 mask 纹理 `.y` 通道打包回读为水/排除位（`>0.5` water bit / `>0.2` clutter bit）。
//! 生命周期（O，泛型胶水）：`AssetSsboLibrary::add_new/get_rg_buffer`、
//! `TypedStorageBuffer::new_storage_buffer`、`ComputePass::write_storage_buffer`、
//! `pass_resource_access::write_access_buffer<WaterMaskSsbo>`（0x140974a20）。

#![allow(dead_code, unused_imports)]

use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::typed_gap::Gap;

/// 水 mask 回读缓冲（O 名，布局 U）。
pub struct WaterMaskSsbo(pub Gap);

impl SsboType for WaterMaskSsbo {
    /// typed gap：原值 = `utils::name_hash("…WaterMaskSsbo")`。
    fn rg_buffer_hash() -> u64 {
        0
    }
}

/// 水 mask 栅格尺寸（O：512×512）。
pub const WATER_MASK_GRID: usize = 512;
/// 水位阈值（O：`read_water_mask.cs.hlsl` `water_mask[coord].y > 0.5f`）。
pub const WATER_BIT_THRESHOLD: f32 = 0.5;
/// 排除 clutter 阈值（O：`water_mask[coord].y > 0.2f`）。
pub const CLUTTER_BIT_THRESHOLD: f32 = 0.2;
/// 世界→mask 采样缩放（O：`water_mask_sampler.inc.hlsl`
/// `world_xz * 0.0076923077 + 0.5`，即 1/130 与地形高度图同映射）。
pub const WATER_MASK_SAMPLE_SCALE: f32 = 1.0 / 130.0;
/// 水 mask 世界半宽（O：`draw_water_mask.cs.hlsl` `(uv*2-1)*65.0`）。
pub const WATER_WORLD_HALF_EXTENT: f32 = 65.0;
/// 水笔刷覆盖半径（O：`draw_water_mask.cs.hlsl` `2.0 - clamp(dist, 0, 2)`）。
pub const WATER_BRUSH_RADIUS: f32 = 2.0;
