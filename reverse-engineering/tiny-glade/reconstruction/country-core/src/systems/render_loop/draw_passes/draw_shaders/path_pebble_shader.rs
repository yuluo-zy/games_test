//! path_pebble_shader.rs — 路径石子（`_path_pebble.raster`）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/path_pebble_shader.rs`
//! （编译单元 498）。机器码（O）：
//! - `path_pebble_draw` 0x1408e5a10（size 178，line 14）。
//!
//! 逻辑（O）：`read_storage_buffer<PebblePlacementSsbo>`（=`_path_pebbles.cs.hlsl`
//! W17 生产者）→ `TextureFromDisk(0x84b51135b59095ba)`（与 floor_shader 相同——
//! 石子共用石材纹理，I）→ `multi_draw_indexed_indirect`（间接参数源 = W17 写
//! `DrawIndexedIndirectBuffer`，I）。
//!
//! shader URI（O）：`/path_pebble.raster.hlsl`（compiled `_path_pebble`，
//! b903…/f0ad…）。typed gap：`0x84b51135b59095ba` 纹理名 U（O：与 floor 共用）。

#![allow(dead_code, unused_imports)]

use crate::resources::texture_presets::TextureFromDisk;
use crate::consume::pass_builder::DrawBuilder;
use crate::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo;

/// `path_pebble_draw`（O：0x1408e5a10）。石子 SSBO + 石材纹理 →
/// `multi_draw_indexed_indirect`。
pub fn path_pebble_draw(_builder: DrawBuilder, _pebbles: &PebblePlacementSsbo) -> DrawBuilder {
    let _ = TextureFromDisk(0x84b51135b59095ba);
    unimplemented!()
}
