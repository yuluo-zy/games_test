//! rocky_terrain_draw.rs — 岩石地形（`_rocky_terrain_rocky_terrain.raster`）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs`
//! （编译单元 530）。机器码（O）：
//! - `rocky_terrain_shader` 0x1409cb4e0（size 501，line 16）。
//!
//! 逻辑（O）：置 `+0xa8 |= 4`（cull，I）→
//! `read_storage_buffer<RockyTerrainVertexSSBO>` →
//! `read_storage_buffer<RockyTerrainTriangleSSBO>`（=W4 displace 写的顶点/三角
//! SSBO，I）→ `TextureFromDisk(0x84a037e697de0324)` → `TexturePreset(0xc)` →
//! `write_uniform<bool>` → 资源 `0xfa44538b95ae76c6`（U，岩石网格实例计数）→
//! `draw_no_index_buffer`（无索引直绘，O）。
//!
//! shader URI（O）：`/rocky_terrain/rocky_terrain.raster.hlsl`（compiled
//! `_rocky_terrain_rocky_terrain`，b903…/f0ad…）。
//! typed gap：`0x84a037e697de0324` 纹理名 U；`0xfa44538b95ae76c6` 资源语义 U。

#![allow(dead_code, unused_imports)]

use crate::resources::texture_presets::{TextureFromDisk, TexturePreset};
use country_core::consume::pass_builder::DrawBuilder;
use country_core::resources::rocky_terrain::{RockyTerrainTriangleSSBO, RockyTerrainVertexSSBO};
use country_core::typed_gap::Gap;

/// `rocky_terrain_shader`（O：0x1409cb4e0）。顶点/三角 SSBO + 岩石纹理 →
/// `draw_no_index_buffer`。
pub fn rocky_terrain_shader(
    _builder: DrawBuilder,
    _vertices: &RockyTerrainVertexSSBO,
    _triangles: &RockyTerrainTriangleSSBO,
    _world: &Gap,
) -> DrawBuilder {
    let _ = (TextureFromDisk(0), TexturePreset(0xc));
    unimplemented!()
}
