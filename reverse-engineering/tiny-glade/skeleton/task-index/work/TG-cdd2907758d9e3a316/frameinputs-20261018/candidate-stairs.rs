//! `stairs.rs` — W20 岩质地形台阶生成（O：`stairs` 0x140a68a50，2286 B）。
//!
//! 参数（O）：`(ctx, world)`，无资源读取。两个 pass：
//!
//! 1. **"Stairs Marching Squares"**（`/rocky_terrain/stairs/stairs.cs.hlsl`，23 / 36 B）：
//!    - `write_storage_buffer<StairsTransformSsbo>`（`stairs_buffer` = 0xd2133b40ce812cca）；
//!    - `write_uniform<[u32;2]>`（`contouring_grid_dims` = 0xf32096f91a47fc71，值 **(80, 80)**）；
//!    - `write_uniform<i32>`（`pebble_buffer_offset` = 0x0c87906f4f6dd692，值 **0x9c40 = 40000**）；
//!    - `write_uniform<i32>`（`pebble_indirect_draw_idx` = 0x3d4b337c85484553，值 **2**）；
//!    - 纹理（访问码均 0x19）：preset 9 `terrain_heightmap`、0xa `visual_terrain_heightmap`、0xe `rocky_terrain`、
//!      5 `path_mask`、0x13 `water_mask`；
//!    - `write_storage_buffer_concurrent<DrawIndexedIndirectBuffer>`（`draw_commands`，组 1）；
//!    - `write_uniform<i32>`（`draw_indirect_buffer_offset`，值 **1**）；
//!    - `dispatch_2d(80, 80)`（O：常量 `{1, 80, 80, 1}`）。
//! 2. **"Stairs counter"**（`/rocky_terrain/stairs/stairs_counter.cs.hlsl`，14 / 44 B）：
//!    `read_storage_buffer<DrawIndexedIndirectBuffer>`（`draw_commands`）、`write_uniform<i32>`（`draw_indirect_buffer_offset` = 1）、
//!    `write_storage_buffer<StairsCounterSsbo>`（`stairs_counter` = 0xa6e4734c9bd10774）、`dispatch_1d(1)`（O：常量 `{1,1,1,1}`）。
//!
//! SSBO 名（O：SsboId 与 xxh3_64 比对）："Stairs transforms"（`StairsTransformSsbo`）、"Stairs Counter"（`StairsCounterSsbo`）。

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::ssbo::{rhapsody_boundary::GraphicsDevice, StorageElement, TypedStorageBuffer};
use crate::resources::texture_presets::TexturePreset;
use crate::systems::render_loop::compute_passes::pass_kit::{
    binding_hash, PassEnv, StairsCounterBuf, TEX_READ,
};
use crate::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer;

/// Original fixed48B affine storage, not an InstanceBuffer. The add_new
/// wrapper@1412c37d0 allocates40B; ctor@141612cb0 checks whole48B rows and
/// size<=0x1e0780, then forwards None without uploading zeroes.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StairsTransformSsbo(pub crate::resources::render::Affine3Packed);
// SAFETY: repr(C) Affine3Packed consists only of four contiguous Vec3 columns,
//48B of f32 fields with no padding. Allzero bits are valid f32 values.
unsafe impl bytemuck::Pod for StairsTransformSsbo {}
unsafe impl bytemuck::Zeroable for StairsTransformSsbo {}
impl StorageElement for StairsTransformSsbo {
    const NAME: &'static str = "Stairs transforms";
    const BUFFER_LEN: usize = 41000;
}
impl LibrarySsbo for StairsTransformSsbo {
    type Storage = TypedStorageBuffer<Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        TypedStorageBuffer::new_storage_buffer(device, initial)
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for StairsTransformSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
             .0
    }
}

pub const MARCHING_NAME: &str = "Stairs Marching Squares";
pub const MARCHING_URI: &str = "/rocky_terrain/stairs/stairs.cs.hlsl";
pub const COUNTER_NAME: &str = "Stairs counter";
pub const COUNTER_URI: &str = "/rocky_terrain/stairs/stairs_counter.cs.hlsl";

pub const HASH_STAIRS: u64 = binding_hash("stairs_buffer");
pub const KEY_GRID: u64 = binding_hash("contouring_grid_dims");
pub const KEY_PEBBLE_OFFSET: u64 = binding_hash("pebble_buffer_offset");
pub const KEY_PEBBLE_DRAW_IDX: u64 = binding_hash("pebble_indirect_draw_idx");
pub const HASH_DRAW_COMMANDS: u64 = binding_hash("draw_commands");
pub const KEY_DRAW_OFFSET: u64 = binding_hash("draw_indirect_buffer_offset");
pub const HASH_COUNTER: u64 = binding_hash("stairs_counter");
pub const GRID: u32 = 80;
pub const PEBBLE_OFFSET: i32 = 0x9c40;
pub const PEBBLE_DRAW_IDX: i32 = 2;
pub const DRAW_OFFSET: i32 = 1;
/// 纹理绑定（O：`(绑定名, preset)`，按调用顺序）。
pub const TEXTURES: [(&str, u8); 5] = [
    ("terrain_heightmap", 9),
    ("visual_terrain_heightmap", 0xa),
    ("rocky_terrain", 0xe),
    ("path_mask", 5),
    ("water_mask", 0x13),
];

/// W20 入口（O 0x140a68a50）。
pub fn stairs(env: &mut PassEnv<'_>) {
    let mut pass = env
        .pass(MARCHING_NAME, MARCHING_URI)
        .write_storage_buffer::<StairsTransformSsbo>(HASH_STAIRS)
        .uniform(KEY_GRID, [GRID, GRID])
        .uniform(KEY_PEBBLE_OFFSET, PEBBLE_OFFSET)
        .uniform(KEY_PEBBLE_DRAW_IDX, PEBBLE_DRAW_IDX);
    for (name, preset) in TEXTURES {
        pass = pass.bind_texture(binding_hash(name), &TexturePreset(preset), TEX_READ);
    }
    let rec = pass
        .write_storage_buffer_concurrent::<DrawIndexedIndirectBuffer>(HASH_DRAW_COMMANDS, 1)
        .uniform(KEY_DRAW_OFFSET, DRAW_OFFSET)
        .dispatch_2d(GRID, GRID);
    env.record(rec);

    let rec = env
        .pass(COUNTER_NAME, COUNTER_URI)
        .read_storage_buffer::<DrawIndexedIndirectBuffer>(HASH_DRAW_COMMANDS)
        .uniform(KEY_DRAW_OFFSET, DRAW_OFFSET)
        .write_storage_buffer::<StairsCounterBuf>(HASH_COUNTER)
        .dispatch_1d(1);
    env.record(rec);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    fn world() -> TestWorld {
        let mut h = vec![HASH_STAIRS, HASH_DRAW_COMMANDS, HASH_COUNTER];
        h.extend(TEXTURES.iter().map(|t| binding_hash(t.0)));
        TestWorld::new(Reflection::new().bind(&h).uniforms(&[
            KEY_GRID,
            KEY_PEBBLE_OFFSET,
            KEY_PEBBLE_DRAW_IDX,
            KEY_DRAW_OFFSET,
        ]))
        .with_ssbo::<StairsTransformSsbo>()
        .with_ssbo::<DrawIndexedIndirectBuffer>()
        .with_ssbo::<StairsCounterBuf>()
    }

    #[test]
    fn two_passes_with_expected_dispatches() {
        let w = world();
        let mut f = TestFrame::default();
        with_env(&w, &mut f, stairs);
        assert_eq!(f.order, ["Stairs Marching Squares", "Stairs counter"]);
        assert_eq!(
            *w.backend.paths.borrow(),
            [
                "/rocky_terrain/stairs/stairs.cs.hlsl:cs_main",
                "/rocky_terrain/stairs/stairs_counter.cs.hlsl:cs_main"
            ]
        );
        assert!(matches!(
            f.passes[0].dispatch,
            DispatchMode::Threads { x: 80, y: 80, z: 1 }
        ));
        assert!(matches!(
            f.passes[1].dispatch,
            DispatchMode::Threads { x: 1, y: 1, z: 1 }
        ));
    }

    #[test]
    fn marching_squares_uniforms_and_access_sequence() {
        let w = world();
        let mut f = TestFrame::default();
        with_env(&w, &mut f, stairs);
        let p = &f.passes[0];
        // 声明顺序：grid(0, 8 B)、pebble_offset(16)、draw_idx(32)、draw_offset(48)。
        assert_eq!((uniform_u32(p, 0), uniform_u32(p, 4)), (80, 80));
        assert_eq!(uniform_u32(p, 16), 0x9c40);
        assert_eq!(uniform_u32(p, 32), 2);
        assert_eq!(uniform_u32(p, 48), 1);
        // color(0x1a,1)、stairs 写(0x32,0)、5 纹理读(0x19,1)、draw 并发写(0x37,1：A4 `write_storage_buffer_concurrent` 的访问码，O 0x141608450)。
        let acc: Vec<(u8, u32)> = p
            .builder
            .accesses
            .iter()
            .map(|a| (a.access, a.group))
            .collect();
        let mut want = vec![(0x1a, 1), (0x32, 0)];
        want.extend([(0x19, 1); 5]);
        want.push((0x37, 1));
        assert_eq!(acc, want);
    }

    #[test]
    fn counter_pass_reads_draw_commands_and_writes_the_counter() {
        let w = world();
        let mut f = TestFrame::default();
        with_env(&w, &mut f, stairs);
        let p = &f.passes[1];
        let acc: Vec<(u8, u32)> = p
            .builder
            .accesses
            .iter()
            .map(|a| (a.access, a.group))
            .collect();
        assert_eq!(acc, [(0x1a, 1), (0x1a, 1), (0x32, 0)]);
        assert_eq!(uniform_u32(p, 48), 1);
    }

    #[test]
    fn constants_match_machine_code() {
        assert_eq!(HASH_STAIRS, 0xd2133b40ce812cca);
        assert_eq!(HASH_STAIRS, (-0x2decc4bf317ed336_i64) as u64);
        assert_eq!(KEY_GRID, (-0xcdf6906e5b8038f_i64) as u64);
        assert_eq!(KEY_PEBBLE_OFFSET, 0x0c87906f4f6dd692);
        assert_eq!(KEY_PEBBLE_DRAW_IDX, 0x3d4b337c85484553);
        assert_eq!(HASH_COUNTER, (-0x591b8cb3642ef88c_i64) as u64);
        assert_eq!(HASH_DRAW_COMMANDS, (-0xe3f85d47fed89a6_i64) as u64);
        assert_eq!(MARCHING_NAME.len(), 0x17);
        assert_eq!(MARCHING_URI.len(), 0x24);
        assert_eq!(COUNTER_NAME.len(), 0xe);
        assert_eq!(COUNTER_URI.len(), 0x2c);
        assert_eq!(PEBBLE_OFFSET, 40_000);
    }
}
