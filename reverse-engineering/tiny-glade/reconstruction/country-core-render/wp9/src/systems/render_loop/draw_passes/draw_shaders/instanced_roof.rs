//! draw_passes/draw_shaders/instanced_roof.rs — 屋顶实例化 draw 族（WP9）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs`。机器码：
//! `draw_instanced_roof_shader_nani` 0x1409d9070（line 24）、`draw_instanced_roof_spire_shader`
//! 0x1409d96a0（line 58）、`draw_instanced_roof_support_beam_shader` 0x1409d98a0（line 73）。
//! shader：`nani_instanced_roof.raster.hlsl` / `instanced_roof_spire.raster.hlsl` /
//! `instanced_roof_beams.raster.hlsl`。

#![allow(dead_code, unused_imports)]

use crate::resources::roofs::roof_buffers::RoofSupportBeamSsbo;
use crate::resources::roof_tile_ssbo::{RoofSpireSsbo, RoofTileSsbo};
use crate::typed_gap::Gap;

/// draw_instanced_roof_shader_nani（O：0x1409d9070）。
/// 绑定：2×TexturePreset（set9/set2）+ 4×TextureFromDisk（Roof 纹理 4 族）+ 2×uniform<i32>
/// + 读 DormerHoleSsbo / DormerHoleCountPerCellSsbo / RoofColorIdSsbo / **RoofTileSsbo** +
/// `nani_draw()`。
pub fn draw_instanced_roof_shader_nani(_draw_builder: &mut Gap, _tiles: &RoofTileSsbo) {
    let _ = _draw_builder;
    let _ = _tiles;
}

/// draw_instanced_roof_spire_shader（O：0x1409d96a0）。读 **RoofSpireSsbo** +
/// `draw_instanced_w_gl_draw_flag`。
pub fn draw_instanced_roof_spire_shader(_draw_builder: &mut Gap, _spires: &RoofSpireSsbo) {
    let _ = _draw_builder;
    let _ = _spires;
}

/// draw_instanced_roof_support_beam_shader（O：0x1409d98a0）。2×TexturePreset +
/// 读 **RoofSupportBeamSsbo** + `draw_instanced_w_gl_draw_flag`。
pub fn draw_instanced_roof_support_beam_shader(
    _draw_builder: &mut Gap,
    _beams: &RoofSupportBeamSsbo,
) {
    let _ = _draw_builder;
    let _ = _beams;
}
