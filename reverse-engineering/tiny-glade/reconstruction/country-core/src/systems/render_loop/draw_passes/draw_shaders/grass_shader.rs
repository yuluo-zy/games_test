//! grass_shader.rs — 草地 LOD0 / LOD1（`_grass.raster` / `_grass_lod1.raster`）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/grass_shader.rs`（编译单元 589）。机器码（O）：
//! - `draw_common`（line 18）：内联进两个入口，没有独立过程。
//! - `draw_grass_shader` 0x140b4af70（600 B）、`draw_grass_lod1_shader` 0x140b4b1d0（600 B）；两者函数体逐条相同，
//!   只差最后一次调用。
//!
//! 逐条（O，`DrawBuilder` 由 `rcx` 按指针传入，终结操作无返回值）：
//! 1. `bind_texture<TexturePreset(9)>`   名 0x61b08b5e0470c5f2，访问 0x1d
//! 2. `bind_texture<TexturePreset(10)>`  名 0x297d1e4cdae8f657，访问 0x1d
//! 3. `bind_texture<TexturePreset(8)>`   名 0x77160113ac8dca3f，访问 0x1d
//! 4. `read_storage_buffer<GrassBladeDataSsbo>`  名 0xe88d970968e0288c
//! 5. `read_storage_buffer<AtomicCounterBuffer>` 名 0x83cdc731ea452909
//! 6. 常量块存在 → `write_uniform<i32>(0xb06e4bc7fd9f1694, 0x90000)`
//! 7. LOD0：`multi_draw_indexed_indirect(idx = 0, arg_50 = 1, arg_58 = 0)`；
//!    LOD1：`multi_draw_indirect(arg_50 = 1, arg_58 = 0)`（非索引间接）。
//!
//! shader URI（O，pass-builder shader-uri-list）：`/grass.raster.hlsl`、`/grass_lod1.raster.hlsl`。
//! 间接参数缓冲 = `DrawIndexedIndirectBuffer`（LOD0）/ `DrawIndirectBuffer`（LOD1），由 W29 grass compute 写。
//! U：预设 9/10/8 的真实名字；`0x90000` 的含义（I：草叶数量上限 768²）。

use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
    bind_preset, bind_terrain_heightmap, bind_terrain_lighting,
};
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
    GrassBladeDataTag, ReadSsboExt,
};
use crate::systems::render_loop::indirect_params::{
    AtomicCounterBuffer, DrawIndexedIndirectBuffer, DrawIndirectBuffer,
};

/// 第三张预设纹理（O：`TexturePreset(8)`，名 0x77160113ac8dca3f）。
pub const GRASS_EXTRA_PRESET: u8 = 8;
pub const GRASS_EXTRA_PRESET_NAME: u64 = 0x7716_0113_ac8d_ca3f;
pub const GRASS_BLADES_BINDING: u64 = 0xe88d_9709_68e0_288c;
pub const GRASS_ATOMIC_COUNTER_BINDING: u64 = 0x83cd_c731_ea45_2909;
/// `write_uniform<i32>` 的名字与常量值（O：`mov r9d, 0x90000`）。
pub const GRASS_UNIFORM_NAME: u64 = 0xb06e_4bc7_fd9f_1694;
pub const GRASS_UNIFORM_VALUE: i32 = 0x9_0000;
/// 间接绘制的 `arg_50`（O：`mov r8d, 1`）与 `arg_58`（O：`xor r9d, r9d`）。
pub const GRASS_INDIRECT_ARG_50: u64 = 1;
pub const GRASS_INDIRECT_ARG_58: u64 = 0;

/// `draw_common`（O：line 18，内联）。
pub fn draw_common<'p>(b: DrawBuilder<'p>) -> DrawBuilder<'p> {
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = bind_preset(b, GRASS_EXTRA_PRESET, GRASS_EXTRA_PRESET_NAME);
    let b = b.read_ssbo::<GrassBladeDataTag>(GRASS_BLADES_BINDING);
    let b = b.read_ssbo::<AtomicCounterBuffer>(GRASS_ATOMIC_COUNTER_BINDING);
    b.uniform(GRASS_UNIFORM_NAME, GRASS_UNIFORM_VALUE)
}

/// `draw_grass_shader`（O 0x140b4af70）：草 LOD0，索引间接绘制。
pub fn draw_grass_shader(b: DrawBuilder<'_>) {
    draw_common(b).multi_draw_indexed_indirect::<DrawIndexedIndirectBuffer>(
        0,
        GRASS_INDIRECT_ARG_50,
        GRASS_INDIRECT_ARG_58,
    );
}

/// `draw_grass_lod1_shader`（O 0x140b4b1d0）：草 LOD1，非索引间接绘制。
pub fn draw_grass_lod1_shader(b: DrawBuilder<'_>) {
    draw_common(b)
        .multi_draw_indirect::<DrawIndirectBuffer>(GRASS_INDIRECT_ARG_50, GRASS_INDIRECT_ARG_58);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::draw_stream::DrawKind;
    use crate::render::pass_builder::pass_resource_access::SsboType;
    use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
        TERRAIN_HEIGHTMAP_NAME, TERRAIN_LIGHTING_NAME,
    };
    use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::DrawSsbo;
    use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{
        Call, Rig, as_i32, rg, uniform_at,
    };

    fn names() -> Vec<u64> {
        vec![
            TERRAIN_HEIGHTMAP_NAME,
            TERRAIN_LIGHTING_NAME,
            GRASS_EXTRA_PRESET_NAME,
            GRASS_BLADES_BINDING,
            GRASS_ATOMIC_COUNTER_BINDING,
        ]
    }

    #[test]
    fn common_binds_three_presets_two_ssbos_and_the_constant() {
        let mut rig = Rig::new(&names(), &[GRASS_UNIFORM_NAME]);
        let b = draw_common(rig.builder());
        let data = b.constants_data().expect("反射有 uniform 块").to_vec();
        assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 0x90000);
        assert_eq!(
            b.packet().accesses.len(),
            5,
            "3 个预设 + 2 个 SSBO 都命中反射"
        );
        b.draw();
        assert_eq!(
            rig.calls(),
            vec![
                Call::Preset(9),
                Call::Preset(10),
                Call::Preset(8),
                rg::<GrassBladeDataTag>(),
                rg::<AtomicCounterBuffer>(),
            ]
        );
        rig.finish();
    }

    #[test]
    fn lod0_is_multi_draw_indexed_indirect_index_zero() {
        let mut rig = Rig::new(&names(), &[GRASS_UNIFORM_NAME]);
        draw_grass_shader(rig.builder());
        let draws = rig.finish();
        assert_eq!(draws.len(), 1);
        let buf = rig.rg_id(DrawIndexedIndirectBuffer::rg_buffer_hash());
        assert_eq!(
            draws[0].kind,
            DrawKind::IndexedIndirect {
                buffer: buf,
                arg_48: 0,
                arg_50: 1,
                arg_58: 0,
                first_index_count: Some(0x1b)
            },
            "静态表 [0] = 0x1b"
        );
    }

    #[test]
    fn lod1_is_multi_draw_indirect_not_indexed() {
        let mut rig = Rig::new(&names(), &[GRASS_UNIFORM_NAME]);
        draw_grass_lod1_shader(rig.builder());
        let draws = rig.finish();
        let buf = rig.rg_id(DrawIndirectBuffer::rg_buffer_hash());
        assert_eq!(
            draws[0].kind,
            DrawKind::MultiDrawIndirect {
                buffer: buf,
                arg_50: 1,
                arg_58: 0
            }
        );
    }

    #[test]
    fn without_a_uniform_block_the_constant_is_silently_skipped() {
        // 原版 `cmp byte ptr [rbp+0x17e], 8; je`：常量块为 None 时不写。
        let mut rig = Rig::new(&names(), &[]);
        let b = draw_common(rig.builder());
        assert!(b.constants_data().is_none());
        b.draw();
        rig.finish();
    }

    #[test]
    fn slot_names_are_the_original_ones() {
        assert_eq!(GrassBladeDataTag::SLOT_NAME, "GrassBlades");
        assert_eq!(AtomicCounterBuffer::SLOT_NAME, "AtomicCounterBuffer");
    }
}
