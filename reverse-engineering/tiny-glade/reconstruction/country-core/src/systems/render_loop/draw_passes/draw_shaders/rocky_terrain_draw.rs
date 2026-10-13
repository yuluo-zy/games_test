//! rocky_terrain_draw.rs — 岩石地形（`_rocky_terrain.raster`）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs`。机器码（O）：
//! `rocky_terrain_shader` 0x1409cb4e0（718 B）。ABI：`rcx = &mut DrawBuilder`、`r8 = &World`、`r9b = bool`（`rdx` 未读）。
//!
//! 顺序（O）：
//! 0. `packet.state` 第 0 字节 `(b & 0xf3) | 4`
//! 1. `read_storage_buffer<RockyTerrainVertexSSBO>`（槽 "Rocky Terrain Vertices"，名 0x12ef39a9447f1f0e）
//! 2. `read_storage_buffer<RockyTerrainTriangleSSBO>`（槽 "Rocky Terrain Triangles"，名 0x480a6146ae76b082）
//! 3. `TextureFromDisk(0x84a037e697de0324)`（名 0x089488658ef70e65；与 `stairs_draw` 同一文件）
//! 4. `TexturePreset(0xc)`（名 0x8f870a5c7060b0f4）
//! 5. 常量块存在 → `write_uniform<bool>(0x0be4740b2c6814ba, flag)`（与 `floor_shader` 同名）
//! 6. 资源（TypeId 0xfa44538b95ae76c6/0x12146c4ea814069b，缺失 → `Option::unwrap` panic）的 `u32 @+0` = 顶点数
//!    → `draw_no_index_buffer(vertex_count)`
//!
//! shader URI 由 pass 注册；顶点数资源的类型名 U。

use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
    bind_disk, bind_preset,
};
use crate::systems::render_loop::draw_passes::draw_shaders::floor_shader::FLOOR_FLAG_UNIFORM;
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
    ReadSsboExt, RockyTerrainTriangleTag, RockyTerrainVertexTag,
};

pub const ROCKY_VERTEX_BINDING: u64 = 0x12ef_39a9_447f_1f0e;
pub const ROCKY_TRIANGLE_BINDING: u64 = 0x480a_6146_ae76_b082;
pub const ROCKY_DISK_TEXTURE: (u64, u64) = (0x84a0_37e6_97de_0324, 0x0894_8865_8ef7_0e65);
pub const ROCKY_PRESET: (u8, u64) = (0x0c, 0x8f87_0a5c_7060_b0f4);

/// `packet.state` 第 0 字节：`(b & 0xf3) | 4`（O：`and al, -0xd; or al, 4`）。
pub fn rocky_state(state: u32) -> u32 {
    (state & !0x0c) | 0x04
}

/// `rocky_terrain_shader`（O 0x1409cb4e0）。`flag` = `r9b`；`vertex_count` = 世界资源 `u32 @+0`
/// （资源缺失时调用方应 `Option::unwrap` panic，文案 "called `Option::unwrap()` on a `None` value"）。
pub fn rocky_terrain_shader(mut b: DrawBuilder<'_>, flag: bool, vertex_count: u32) {
    let p = b.packet_mut();
    p.state.0 = rocky_state(p.state.0);
    let b = b.read_ssbo::<RockyTerrainVertexTag>(ROCKY_VERTEX_BINDING);
    let b = b.read_ssbo::<RockyTerrainTriangleTag>(ROCKY_TRIANGLE_BINDING);
    let b = bind_disk(b, ROCKY_DISK_TEXTURE.0, ROCKY_DISK_TEXTURE.1);
    let b = bind_preset(b, ROCKY_PRESET.0, ROCKY_PRESET.1);
    let b = b.uniform(FLOOR_FLAG_UNIFORM, flag);
    b.draw_no_index_buffer(vertex_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::draw_stream::DrawKind;
    use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{Call, Rig, rg};

    fn names() -> Vec<u64> {
        vec![
            ROCKY_VERTEX_BINDING,
            ROCKY_TRIANGLE_BINDING,
            ROCKY_DISK_TEXTURE.1,
            ROCKY_PRESET.1,
        ]
    }

    #[test]
    fn state_rewrite_sets_cull_bit_2() {
        assert_eq!(rocky_state(0x8000_008c), 0x8000_0084);
        assert_eq!(rocky_state(0), 4);
    }

    #[test]
    fn order_and_draw_without_index_buffer() {
        let mut rig = Rig::new(&names(), &[FLOOR_FLAG_UNIFORM]);
        rocky_terrain_shader(rig.builder(), true, 0x1234);
        assert_eq!(
            rig.calls(),
            vec![
                rg::<RockyTerrainVertexTag>(),
                rg::<RockyTerrainTriangleTag>(),
                Call::Disk(0x84a037e697de0324),
                Call::Preset(0xc)
            ]
        );
        let d = rig.finish();
        assert_eq!(
            d[0].kind,
            DrawKind::NoIndexBuffer {
                vertex_count: 0x1234
            }
        );
        assert_eq!(d[0].state.0 & 0x0c, 0x04);
    }

    #[test]
    fn slot_names_come_from_the_binary() {
        use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::DrawSsbo;
        assert_eq!(RockyTerrainVertexTag::SLOT_NAME, "Rocky Terrain Vertices");
        assert_eq!(
            RockyTerrainTriangleTag::SLOT_NAME,
            "Rocky Terrain Triangles"
        );
    }
}
