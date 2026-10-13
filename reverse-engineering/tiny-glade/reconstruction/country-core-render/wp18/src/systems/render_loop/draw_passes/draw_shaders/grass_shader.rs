//! grass_shader.rs — 草地 LOD0/1（`_grass.raster` / `_grass_lod1.raster`）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/grass_shader.rs`
//! （编译单元 589）。机器码（O）：
//! - `draw_common`（line 18，内联共享）：`TexturePreset(9)` → 10 → 8 →
//!   `read_storage_buffer<GrassBladeDataSsbo>` →
//!   `read_storage_buffer<AtomicCounterBuffer>` → `write_uniform<i32>`。
//! - `draw_grass_shader` 0x140b4af70（size 412，line 37）：draw_common +
//!   `multi_draw_indexed_indirect`。
//! - `draw_grass_lod1_shader` 0x140b4b1d0（size 410，line 47）：draw_common +
//!   `multi_draw_indirect`（LOD1 用非索引间接）。
//!
//! shader URI（O）：`/grass.raster.hlsl`、`/grass_lod1.raster.hlsl`（compiled
//! `_grass`/`_grass_lod1`，b903…/f0ad…）。间接参数源 = W29 grass compute
//! 写的 `DrawIndexedIndirectBuffer`/`DrawIndirectBuffer` + `AtomicCounterBuffer`（I）。
//! typed gap：preset 9/10/8 槽语义 U（9/10=地面/水纹理族，I）。

#![allow(dead_code, unused_imports)]

use crate::resources::texture_presets::TexturePreset;
use country_core::consume::buffers::AtomicCounterBuffer;
use country_core::consume::pass_builder::DrawBuilder;
use country_core::systems::render_loop::compute_passes::grass::GrassBladeDataSsbo;

/// `draw_common`（O：line 18，内联共享）。草叶 SSBO + atomic counter + 时间/参数
/// uniform。本束以独立函数记录（原为内联宿主）。
pub fn draw_common(_builder: DrawBuilder) -> DrawBuilder {
    let _ = TexturePreset(0);
    unimplemented!()
}

/// `draw_grass_shader`（O：0x140b4af70）。`draw_common` +
/// `multi_draw_indexed_indirect`（草 LOD0 间接索引绘制）。
pub fn draw_grass_shader(
    _builder: DrawBuilder,
    _blades: &GrassBladeDataSsbo,
    _atomic: &AtomicCounterBuffer,
) -> DrawBuilder {
    unimplemented!()
}

/// `draw_grass_lod1_shader`（O：0x140b4b1d0）。`draw_common` +
/// `multi_draw_indirect`（草 LOD1 非索引间接）。
pub fn draw_grass_lod1_shader(
    _builder: DrawBuilder,
    _blades: &GrassBladeDataSsbo,
    _atomic: &AtomicCounterBuffer,
) -> DrawBuilder {
    unimplemented!()
}
