//! leaf_particle_draw.rs — 落叶粒子（`_leaf_particle.raster`）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/leaf_particle_draw.rs`
//! （编译单元 556）。机器码（O）：
//! - `leaf_particle_draw` 0x140a8a090（size 435，line 13）。
//!
//! 逻辑（O）：资源 `0x3dce36237fb50dff`（U，unwrap）→
//! `read_storage_buffer<LeafParticleSsbo>`（=`_leaf_particles.cs.hlsl` 生产者）→
//! `TextureFromDisk(0x4bb0467de05d40d2)` → `TextureFromDisk(0xefa7a5fecb959f12)` →
//! `draw_instanced_w_gl_draw_flag`。
//!
//! shader URI（O）：`/leaf_particle.raster.hlsl`（compiled `_leaf_particle`，
//! b903…/f0ad…）。typed gap：`0x3dce36237fb50dff` 资源语义 U；两磁盘纹理名 U。

#![allow(dead_code, unused_imports)]

use crate::resources::texture_presets::TextureFromDisk;
use crate::consume::pass_builder::DrawBuilder;
use crate::systems::render_loop::compute_passes::leaf_particles::LeafParticleSsbo;
use crate::typed_gap::Gap;

/// `leaf_particle_draw`（O：0x140a8a090）。叶粒子 SSBO + 两磁盘纹理 →
/// `draw_instanced_w_gl_draw_flag`。
pub fn leaf_particle_draw(
    _builder: DrawBuilder,
    _particles: &LeafParticleSsbo,
    _world: &Gap,
) -> DrawBuilder {
    let _ = TextureFromDisk(0);
    unimplemented!()
}
