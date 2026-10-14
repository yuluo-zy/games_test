//! stairs_draw.rs — 楼梯（`_stairs.raster`）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs`。机器码（O）：`stairs_draw` 0x14086a780（1507 B）。
//! ABI：`rcx = &mut DrawBuilder`、`rdx = &EntityDrawParams`（`[rdx+8]` = 实体）、`r8 = &World`。
//!
//! 实体侧（O）：`entity_fetch::fetch_ref(entity)` 后按组件 TypeId（0x775c6361f3425a3a/0x3bad471c3be19d00）经
//! archetype 表 / 稀疏集两条路径取组件，读其首字节 `flags`；两条路径合流后
//! `flag = (aux >> 32 == 0) && ((aux as u32 | flags as u32) & 1 != 0)`（`aux`：组件来自表时恒为 0；另一条路径
//! 里是从取值结果尾部拼出的 64 位，语义 U）。常规路径 `aux = 0`，于是 `flag = flags & 1`。
//!
//! 绘制序（O）：
//! 1. `read_storage_buffer<StairsTransformSsbo>`（槽 "Stairs transforms"，名 0xd2133b40ce812cca）
//! 2. `TextureFromDisk(0x84a037e697de0324)`（名 0x089488658ef70e65；`rocky_terrain` 同一文件）
//! 3. 常量块存在 → `write_uniform<i32>(0x0c87906f4f6dd692, 0x9c40)`
//! 4. 常量块存在 → `write_uniform<bool>(0x7263f507731849d2, flag)`
//! 5. `TexturePreset(0xc)`（名 0x8f870a5c7060b0f4）
//! 6. `multi_draw_indexed_indirect(idx = flag + 1, arg_50 = 1, arg_58 = 0)`（静态表 [1] = 300、[2] = 96 个索引）

use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
    bind_disk, bind_preset,
};
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
    ReadSsboExt, StairsTransformTag,
};
use crate::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer;

pub const STAIRS_TRANSFORM_BINDING: u64 = 0xd213_3b40_ce81_2cca;
pub const STAIRS_DISK_TEXTURE: (u64, u64) = (0x84a0_37e6_97de_0324, 0x0894_8865_8ef7_0e65);
pub const STAIRS_I32_UNIFORM: (u64, i32) = (0x0c87_906f_4f6d_d692, 0x9c40);
pub const STAIRS_FLAG_UNIFORM: u64 = 0x7263_f507_7318_49d2;
pub const STAIRS_PRESET: (u8, u64) = (0x0c, 0x8f87_0a5c_7060_b0f4);

/// 实体侧的取值（见模块注释）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StairsEntityData {
    /// 组件首字节（`movzx r14d, byte ptr [rsi]`）。
    pub flags: u8,
    /// `rdi`：来自表的组件恒为 0。
    pub aux: u64,
}

/// `bl = (aux >> 32 == 0) & ((aux as u32 | flags) & 1)`（O：`sete bl` … `or edi, r14d; and bl, dil`）。
pub fn stairs_flag(d: StairsEntityData) -> bool {
    (d.aux >> 32 == 0) && (((d.aux as u32) | d.flags as u32) & 1) != 0
}

/// `stairs_draw`（O 0x14086a780）。
pub fn stairs_draw(b: DrawBuilder<'_>, entity: StairsEntityData) {
    let flag = stairs_flag(entity);
    let b = b.read_ssbo::<StairsTransformTag>(STAIRS_TRANSFORM_BINDING);
    let b = bind_disk(b, STAIRS_DISK_TEXTURE.0, STAIRS_DISK_TEXTURE.1);
    let b = b.uniform(STAIRS_I32_UNIFORM.0, STAIRS_I32_UNIFORM.1);
    let b = b.uniform(STAIRS_FLAG_UNIFORM, flag);
    let b = bind_preset(b, STAIRS_PRESET.0, STAIRS_PRESET.1);
    b.multi_draw_indexed_indirect::<DrawIndexedIndirectBuffer>(flag as u32 + 1, 1, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::draw_stream::DrawKind;
    use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{
        Call, Rig, as_i32, rg, uniform_at,
    };

    fn names() -> Vec<u64> {
        vec![
            STAIRS_TRANSFORM_BINDING,
            STAIRS_DISK_TEXTURE.1,
            STAIRS_PRESET.1,
        ]
    }

    #[test]
    fn flag_truth_table() {
        let f = |flags, aux| stairs_flag(StairsEntityData { flags, aux });
        assert!(!f(0, 0));
        assert!(f(1, 0));
        assert!(!f(2, 0), "只看 bit0");
        assert!(f(0, 1), "aux 的低位也会置位");
        assert!(
            !f(1, 1 << 32),
            "aux 高 32 位非 0 → 恒为 false（`shr rax, 0x20; sete`）"
        );
    }

    #[test]
    fn sequence_uniforms_and_mesh_index_follow_the_flag() {
        for (flags, idx, count) in [(0u8, 1u32, 0x12cu32), (1, 2, 0x60)] {
            let mut rig = Rig::new(&names(), &[STAIRS_I32_UNIFORM.0, STAIRS_FLAG_UNIFORM]);
            stairs_draw(rig.builder(), StairsEntityData { flags, aux: 0 });
            assert_eq!(
                rig.calls(),
                vec![
                    rg::<StairsTransformTag>(),
                    Call::Disk(0x84a037e697de0324),
                    Call::Preset(0xc),
                    rg::<DrawIndexedIndirectBuffer>()
                ]
            );
            let d = rig.finish();
            let buf = rig.rg_id(<DrawIndexedIndirectBuffer as crate::render::pass_builder::pass_resource_access::SsboType>::rg_buffer_hash());
            assert_eq!(
                d[0].kind,
                DrawKind::IndexedIndirect {
                    buffer: buf,
                    arg_48: idx as u64 * 0x14,
                    arg_50: 1,
                    arg_58: 0,
                    first_index_count: Some(count)
                }
            );
        }
    }

    #[test]
    fn uniform_values() {
        let mut rig = Rig::new(&names(), &[STAIRS_I32_UNIFORM.0, STAIRS_FLAG_UNIFORM]);
        let b = rig
            .builder()
            .uniform(STAIRS_I32_UNIFORM.0, STAIRS_I32_UNIFORM.1)
            .uniform(STAIRS_FLAG_UNIFORM, true);
        let data = b.constants_data().unwrap().to_vec();
        assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 40000);
        assert_eq!(as_i32(&uniform_at(&data, 1, 4)), 1);
        b.draw();
        rig.finish();
    }
}
