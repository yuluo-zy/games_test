//! leaf_particle_draw.rs — 落叶粒子（`_leaf_particle.raster`）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/leaf_particle_draw.rs`。机器码（O）：
//! `leaf_particle_draw` 0x140a8a090（628 B）。ABI：`rcx = &mut DrawBuilder`、`r8 = &World`。
//!
//! 顺序（O）：
//! 1. 世界资源（TypeId 0x3dce36237fb50dff/0x93a27cf4512d885c，缺失 → `Option::unwrap` panic）的 `u64 @+0` = 粒子数
//! 2. `read_storage_buffer<LeafParticleSsbo>`（槽 "LeafParticles"，名 0xb7b51d286b577eac）
//! 3. `TextureFromDisk(0x4bb0467de05d40d2)`（名 0x0ffe919c88323006）
//! 4. `TextureFromDisk(0xefa7a5fecb959f12)`（名 0x1645417ef0f4e1f6）
//! 5. `draw_instanced_w_gl_draw_flag(count)`
//!
//! 粒子数资源的类型名与两张纹理的文件名 U。

use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::bind_disk;
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
    LeafParticleTag, ReadSsboExt,
};

pub const LEAF_PARTICLE_BINDING: u64 = 0xb7b5_1d28_6b57_7eac;
pub const LEAF_TEXTURE_A: (u64, u64) = (0x4bb0_467d_e05d_40d2, 0x0ffe_919c_8832_3006);
pub const LEAF_TEXTURE_B: (u64, u64) = (0xefa7_a5fe_cb95_9f12, 0x1645_417e_f0f4_e1f6);

/// `leaf_particle_draw`（O 0x140a8a090）。`particle_count` = 世界资源 `u64 @+0`（实例数按 `u32` 传给 draw）。
pub fn leaf_particle_draw(b: DrawBuilder<'_>, particle_count: u64) {
    let b = b.read_ssbo::<LeafParticleTag>(LEAF_PARTICLE_BINDING);
    let b = bind_disk(b, LEAF_TEXTURE_A.0, LEAF_TEXTURE_A.1);
    let b = bind_disk(b, LEAF_TEXTURE_B.0, LEAF_TEXTURE_B.1);
    b.draw_instanced_w_gl_draw_flag(particle_count as u32);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::draw_stream::DrawKind;
    use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{Call, Rig, rg};

    #[test]
    fn ssbo_then_two_disk_textures_then_instanced_draw_with_the_count() {
        let mut rig = Rig::new(
            &[LEAF_PARTICLE_BINDING, LEAF_TEXTURE_A.1, LEAF_TEXTURE_B.1],
            &[],
        );
        leaf_particle_draw(rig.builder(), 321);
        assert_eq!(
            rig.calls(),
            vec![
                rg::<LeafParticleTag>(),
                Call::Disk(0x4bb0467de05d40d2),
                Call::Disk(0xefa7a5fecb959f12)
            ]
        );
        let d = rig.finish();
        assert_eq!(d[0].kind, DrawKind::InstancedGlDrawFlag { flag: 321 });
    }

    #[test]
    fn zero_particles_still_issue_a_draw() {
        let mut rig = Rig::new(&[], &[]);
        leaf_particle_draw(rig.builder(), 0);
        assert_eq!(
            rig.finish()[0].kind,
            DrawKind::InstancedGlDrawFlag { flag: 0 }
        );
    }
}
