//! path_pebble_shader.rs — 路径石子（`_path_pebble.raster`）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/path_pebble_shader.rs`（编译单元 498）。机器码（O）：
//! `path_pebble_draw` 0x1408e5a10（227 B）：
//! 1. `read_storage_buffer<PebblePlacementSsbo>`（槽 "PebblePlacement"，名 0xe853ba5b3b695a54）
//! 2. `TextureFromDisk(0x84b51135b59095ba)`（名 0xbbe03b6e0596993b；同一个文件键也出现在 `floor_shader`，I：石子共用石材纹理）
//! 3. `multi_draw_indexed_indirect(idx = 3, arg_50 = 1, arg_58 = 0)`（静态表 [3] = 0x39 个索引）
//!
//! shader URI（O）：`/path_pebble.raster.hlsl`。间接参数缓冲 = `DrawIndexedIndirectBuffer`（W17 `_path_pebbles.cs` 写）。

use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::bind_disk;
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
    PebblePlacementTag, ReadSsboExt,
};
use crate::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer;

pub const PEBBLE_PLACEMENT_BINDING: u64 = 0xe853_ba5b_3b69_5a54;
pub const PEBBLE_STONE_TEXTURE: (u64, u64) = (0x84b5_1135_b590_95ba, 0xbbe0_3b6e_0596_993b);
/// `multi_draw_indexed_indirect` 的静态表下标（O：`mov edx, 3`）。
pub const PEBBLE_MESH_INDEX: u32 = 3;

/// `path_pebble_draw`（O 0x1408e5a10）。
pub fn path_pebble_draw(b: DrawBuilder<'_>) {
    let b = b.read_ssbo::<PebblePlacementTag>(PEBBLE_PLACEMENT_BINDING);
    let b = bind_disk(b, PEBBLE_STONE_TEXTURE.0, PEBBLE_STONE_TEXTURE.1);
    b.multi_draw_indexed_indirect::<DrawIndexedIndirectBuffer>(PEBBLE_MESH_INDEX, 1, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::draw_stream::DrawKind;
    use crate::render::pass_builder::pass_resource_access::SsboType;
    use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{Call, Rig, rg};

    #[test]
    fn reads_the_pebbles_then_the_stone_texture_then_draws_mesh_3() {
        let mut rig = Rig::new(&[PEBBLE_PLACEMENT_BINDING, PEBBLE_STONE_TEXTURE.1], &[]);
        path_pebble_draw(rig.builder());
        assert_eq!(
            rig.calls()[..2],
            [rg::<PebblePlacementTag>(), Call::Disk(0x84b51135b59095ba)]
        );
        let d = rig.finish();
        let buf = rig.rg_id(DrawIndexedIndirectBuffer::rg_buffer_hash());
        assert_eq!(
            d[0].kind,
            DrawKind::IndexedIndirect {
                buffer: buf,
                arg_48: 3 * 0x14,
                arg_50: 1,
                arg_58: 0,
                first_index_count: Some(0x39)
            }
        );
        assert_eq!(
            d[0].accesses.len(),
            1,
            "只有 SSBO 进了 packet.accesses；磁盘纹理走描述符绑定"
        );
    }

    #[test]
    fn the_stone_texture_is_the_same_file_as_the_floor_shader() {
        assert_eq!(PEBBLE_STONE_TEXTURE.0, 0x84b5_1135_b590_95ba);
    }
}
