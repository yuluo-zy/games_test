//! roof_tile_ssbo.rs — 屋顶 SSBO 元素（RoofShingle / RoofSpireSsbo）（WP9）。
//!
//! PDB 命名空间 `country_core::resources::roof_tile_ssbo`（无独立源文件；类型被子模块
//! 引用，如 sparse_instance_buffer、ssbo_library、roof_tiles_gravity）。64B RoofShingle
//! 与 evidence/roof-tiles + shader `roof/roof_tile.inc.hlsl` 一致（O）。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

/// 屋瓦实例记录（64B；RoofTileSsbo = RoofShingle）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct RoofShingle {
    pub base_position: [f32; 3], // +0
    pub current_height: f32,     // +12
    pub clipping_plane: [u32; 2], // +16
    pub scale_t: [u32; 2],       // +24（half2 打包：尺寸×、t）
    pub rotation_quat: [u32; 2], // +32（half4 四元数）
    pub roof_id: u32,            // +40
    pub velocity_rotation: u32,  // +44（half2x16：速度/旋转）
    pub animation_t: f32,        // +48
    pub seed: i32,               // +52
    pub is_circle: i32,          // +56
    pub is_main_roof_body: i32,  // +60
}

impl RoofShingle {
    pub const SIZE: usize = 64;
}

/// 屋顶 SSBO 元素类型（裸别名）。
pub type RoofTileSsbo = RoofShingle;

/// 屋顶尖顶（spire）实例（draw_instanced_roof_spire_shader 读；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct RoofSpireSsbo(pub Gap);
