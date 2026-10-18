//! Original B W4 @140a3ee80/1518B, canonical vertex-count and GPU owners.
//! The original count resource is RockyTerrainMeshVertCount. Its value / 3,
//! truncated to u32, feeds both push constants and dispatch; it is not the
//! current retained CPU mesh layout or a ResourceView of a native header.

use crate::resources::rocky_terrain::{
    RockyTerrainMeshVertCount, RockyTerrainTriangleSsbo, RockyTerrainVertexSsbo,
};
use crate::resources::texture_presets::TexturePreset;
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, TEX_READ, binding_hash};

pub const PASS_NAME: &str = "Displace Rocky Terrain";
pub const SHADER_URI: &str = "/rocky_terrain/displace_rocky_terrain.cs.hlsl";
pub const HASH_VERTICES: u64 = binding_hash("vertices"); // 0x12ef39a9447f1f0e
pub const HASH_TRIANGLES: u64 = binding_hash("triangles"); // 0x480a6146ae76b082
/// 五个纹理绑定（O：`(绑定名哈希, preset)`，按调用顺序）。
pub const TEXTURES: [(u64, u8); 5] = [
    (binding_hash("terrain_heightmap"), 9),
    (binding_hash("visual_terrain_heightmap"), 0xa),
    (binding_hash("rocky_terrain"), 0xe),
    (binding_hash("path_mask"), 5),
    (binding_hash("water_mask"), 0x13),
];

/// `x / 3` 截成 u32（O：`mul` 魔数 + `shr` 取高半，再 `mov [rax], esi` 只写低 32 位）。
pub fn group_count(first_word: u64) -> u32 {
    (first_word / 3) as u32
}

/// W4 系统（O 0x140a3ee80）。
pub fn displace_rocky_terrain(env: &mut PassEnv<'_>, vertices: &RockyTerrainMeshVertCount) {
    let groups = group_count(vertices.0 as u64);
    let mut pass = env
        .pass(PASS_NAME, SHADER_URI)
        .write_storage_buffer::<RockyTerrainVertexSsbo>(HASH_VERTICES)
        .write_storage_buffer::<RockyTerrainTriangleSsbo>(HASH_TRIANGLES);
    for (hash, preset) in TEXTURES {
        pass = pass.bind_texture(hash, &TexturePreset(preset), TEX_READ);
    }
    let recorded = pass
        .push_constants(&groups.to_le_bytes())
        .dispatch_1d(groups);
    env.record(recorded);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::render::pass_builder::pass_resource_access::PassBinding;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    fn world() -> TestWorld {
        let mut hashes = vec![HASH_VERTICES, HASH_TRIANGLES];
        hashes.extend(TEXTURES.iter().map(|t| t.0));
        TestWorld::new(Reflection::new().bind(&hashes))
            .with_ssbo::<RockyTerrainVertexSsbo>()
            .with_ssbo::<RockyTerrainTriangleSsbo>()
    }

    #[test]
    fn divides_first_word_by_three_and_truncates() {
        assert_eq!(group_count(0), 0);
        assert_eq!(group_count(2), 0);
        assert_eq!(group_count(3), 1);
        assert_eq!(group_count(3000), 1000);
        assert_eq!(
            group_count(0x1_0000_0000 * 3 + 5),
            1,
            "高 32 位被截断（`mov [rax], esi`）"
        );
    }

    #[test]
    fn records_one_pass_with_declared_resources_in_order() {
        let w = world();
        let mut f = TestFrame::default();
        with_env(&w, &mut f, |env| {
            displace_rocky_terrain(env, &RockyTerrainMeshVertCount(3 * 77))
        });
        assert_eq!(f.passes.len(), 1);
        let p = &f.passes[0];
        assert_eq!(p.name, "Displace Rocky Terrain");
        assert_eq!(
            w.backend.paths.borrow()[0],
            "/rocky_terrain/displace_rocky_terrain.cs.hlsl:cs_main"
        );
        assert!(matches!(
            p.dispatch,
            DispatchMode::Threads { x: 77, y: 1, z: 1 }
        ));
        assert_eq!(p.push_constants, 77u32.to_le_bytes());
        // 访问：color library 读(0x1a)、两个 SSBO 写(0x32)、五个纹理读(0x19)。
        let acc: Vec<u8> = p.builder.accesses.iter().map(|a| a.access).collect();
        assert_eq!(acc, [0x1a, 0x32, 0x32, 0x19, 0x19, 0x19, 0x19, 0x19]);
        // 绑定表：color/vertex/triangle 为缓冲，其后 5 个图像视图（只读）。
        assert!(matches!(p.bindings[1], PassBinding::Buffer { .. }));
        assert!(
            p.bindings[3..]
                .iter()
                .all(|b| matches!(b, PassBinding::ImageView { write: false, .. }))
        );
    }

    #[test]
    fn preset_slots_and_hashes_match_machine_code() {
        let presets: Vec<u8> = TEXTURES.iter().map(|t| t.1).collect();
        assert_eq!(presets, [9, 0xa, 0xe, 5, 0x13]);
        assert_eq!(TEXTURES[0].0, 0x61b08b5e0470c5f2);
        assert_eq!(HASH_VERTICES, 0x12ef39a9447f1f0e);
        assert_eq!(HASH_TRIANGLES, 0x480a6146ae76b082);
        assert_eq!(TEXTURES[3].0, (-0x12afd000923bfde1_i64) as u64);
    }

    #[test]
    fn zero_triangles_still_records_a_zero_sized_dispatch() {
        let w = world();
        let mut f = TestFrame::default();
        with_env(&w, &mut f, |env| {
            displace_rocky_terrain(env, &RockyTerrainMeshVertCount(0))
        });
        assert!(matches!(
            f.passes[0].dispatch,
            DispatchMode::Threads { x: 0, .. }
        ));
    }
}
