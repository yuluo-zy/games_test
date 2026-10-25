//! `compute_garden.rs` — W12 `garden_rasterize_from_cpu`（O 0x140a22140，2616 B）、
//! W16 `garden_spawn_vegetation`（O 0x140a22b80，5234 B）。
//!
//! ## garden_rasterize_from_cpu（参数 O：`(ctx, world)`）
//! 1. `get_resource_id(0x979ffa6288aa259a, 0x09791d5b46ed449d)`（= `GardenRaster`，见 `upload_world_raster`；缺失 → unwrap panic）。
//!    `world.change_tick(+0x5b0)` 读出；**遮罩标志 `res+0x29b == 0` → 直接返回**（该标志由 W1 `upload_world_raster` 置位）。
//!    否则：资源的 `changed_tick(entry+0x5c) = world.change_tick`、`res+0x29b = 0`、`memset(res+0x98, 0, 0x200)`。
//! 2. 临时纹理 "garden blur temp"：`new_2d((512, 512), R16G16B16A16_SFLOAT /*0x61*/)`（+ sampled/storage，I），`create_raw_resource`。
//! 3. 两个 "Blur garden mask"（`/gauss_blur_mask.cs.hlsl`）pass，各自：
//!    `write_uniform<bool>`（键 0xd91ff8372094bc22：**第一次 true（水平）、第二次 false（垂直）**）、
//!    读 0x19 / 写 0x32 两个纹理（绑定名 0x6ad63ccd18c8f041 = 源，0xe8f7c3a7b57d0768 = 目标）、
//!    `write_uniform<f32>`（键 0xdc022c4f3092a794，值 **5.0**）、`write_uniform<i32>`（键 0x4ad76c5b9d4fae28，值 1）、
//!    `dispatch_2d(512, 512)`。
//!    - 第一次：源 = preset 0x14（花园遮罩），目标 = 临时纹理；
//!    - 第二次：源 = 临时纹理，目标 = preset 0x14。
//!
//! ## garden_spawn_vegetation（参数 O：`(ctx, world)`，无资源读取）
//! 四个 pass，每个的 `[u32;2]` uniform（键 0xfb539f7243dbb832，stones 为 0xf32096f91a47fc71）= 其 dispatch 尺寸：
//!
//! | pass | URI | 输出 SSBO（哈希，组） | 尺寸 | 其余 |
//! |---|---|---|---|---|
//! | Spawn bushes | spawn_bushes_garden | Bush(0x5baefac4a34b3dee,1) + Flower(0xf3aa86819743c1d8,1) | 90 | 预设 9,0xa,exclusion,0x14,0x16；indirect(0xf1c07a2b8012765a,1)；i32 uniform 10 |
//! | Spawn flowers | spawn_flowers_garden | Flower(0x633d7e34d2dd23f1,1) | 150 | 同上；i32 11 |
//! | Garden Marching Squares | marching_squares | exclusion,0x14 → GardenStone(0x6d9256d4ac7887ad) | 180 | 预设 9,0xa；indirect；i32 14 |
//! | Spawn small flowers | spawn_small_flowers_garden | Flower03(0x7369bcad63f3af6e) | 400 | 预设 9,0xa,exclusion,0x14,0x15；indirect；i32 16 |
//!
//! 预设绑定名哈希：9 → 0x61b08b5e0470c5f2、0xa → 0x297d1e4cdae8f657、0x14 → 0x49f43a487883738d、
//! 0x16 → 0x6570181649ac6420、0x15 → 0x4a9aecb3eff0f013（访问码均 0x19）。`i32` uniform（键 0x0e9511a616b75789）
//! 值 = 10/11/14/16（I：该 pass 在 `DrawIndexedIndirectBuffer` 里的槽位）。
//!
//! SSBO 名（O：SsboId 与 xxh3_64 比对）：BushSsbo → "Bushes"、FlowerSsbo → "FlowerSsbo"、GardenStoneSsbo → "Garden Stone"；
//! `BushSsbo` / `FlowerSsbo` 是真实库侧类型（元素 48 / 64 B，固定容量，见各类型文档），`GardenStoneSsbo` / `Flower03Ssbo` 仍是载体；
//! `Flower03Ssbo` 的 NAME 未能核实（U，占位 "Flower03Ssbo"）。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::manual_repeat_n, clippy::useless_vec)]
use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::compute_pass_builder::ComputePass;
use crate::render::pass_builder::pass_resource_access::RgTexture;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{StorageElement, TypedStorageBuffer};
use crate::resources::render::Affine3Packed;
use crate::resources::texture_presets::TexturePreset;
use crate::rhapsody_types::TextureDesc;
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, TEX_READ, TEX_WRITE, binding_hash, fixed_len_typed_storage, ssbo_carrier,
};
use crate::systems::render_loop::compute_passes::upload_world_raster::GardenRaster;
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes_compute::bind_garden_exclusion_compute;
use crate::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer;
use crate::typed_gap::Gap;
use glam::Vec3;

/// 灌木实例（O：`compute_garden::BushSsbo`；着色器 `BushXform { Affine3Packed transform }`，48 B = 12 个 f32）。
///
/// 机器码（O 0x141610b60，`impl$0::new_storage_buffer<BushSsbo>`）：`BufferDesc::new(0x70800, 0x22, 1).alignment(4)`，名 "Bushes"；
/// 元素 48 B（`len % 48` 经乘法逆元）→ 容量 0x70800 / 48 = 9600；上限 `cmp r9, 0x70830`。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BushSsbo {
    pub transform: Affine3Packed,
}

/// 花实例（O：`compute_garden::FlowerSsbo`；着色器 `FlowerXform { Affine3Packed transform; float3 color; uint flower_type }`，64 B）。
///
/// 机器码（O 0x141613b90）：`BufferDesc::new(0x33e000, 0x22, 1).alignment(4)`，名 "FlowerSsbo"；元素 64 B（`and rax, 0x3f`）→
/// 容量 0x33e000 / 64 = 53120；上限 `cmp r14, 0x33e040`。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FlowerSsbo {
    pub transform: Affine3Packed,
    pub color: Vec3,
    pub flower_type: u32,
}

impl StorageElement for BushSsbo {
    const NAME: &'static str = "Bushes";
    /// 0x70800 / 48。
    const BUFFER_LEN: usize = 9600;
}
impl StorageElement for FlowerSsbo {
    const NAME: &'static str = "FlowerSsbo";
    /// 0x33e000 / 64。
    const BUFFER_LEN: usize = 53120;
}
impl LibrarySsbo for BushSsbo {
    type Storage = TypedStorageBuffer<BushSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        fixed_len_typed_storage::<BushSsbo>(device, initial)
    }
}
impl LibrarySsbo for FlowerSsbo {
    type Storage = TypedStorageBuffer<FlowerSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        fixed_len_typed_storage::<FlowerSsbo>(device, initial)
    }
}

/// 其余两个花园 SSBO 仍是载体（元素布局 U；`Flower03Ssbo` 的 NAME 也未核实）。
#[derive(Clone, Copy, Debug, Default)]
pub struct Flower03Ssbo(pub Gap);
#[derive(Clone, Copy, Debug, Default)]
pub struct GardenStoneSsbo(pub Gap);
ssbo_carrier!(Flower03Ssbo, "Flower03Ssbo");
ssbo_carrier!(GardenStoneSsbo, "Garden Stone");

// ------------------------------------------------------------------ rasterize

pub const BLUR_NAME: &str = "Blur garden mask";
pub const BLUR_URI: &str = "/gauss_blur_mask.cs.hlsl";
pub const TEMP_NAME: &str = "garden blur temp";
/// `ash::vk::Format::R16G16B16A16_SFLOAT`（O：`new_2d(.., 0x61)`）。
pub const TEMP_FORMAT: u32 = 0x61;
pub const MASK_GRID: u32 = 512;
pub const MASK_PRESET: u8 = 0x14;
pub const BLUR_SIGMA: f32 = 5.0;
/// `gauss_blur_mask.cs.hlsl` 的绑定名（O：FNV-1a-64 与 `movabs r8` 一致）。
pub const KEY_HORIZONTAL: u64 = binding_hash("is_horizontal"); // 0xd91ff8372094bc22
pub const KEY_SIGMA: u64 = binding_hash("sigma"); // 0xdc022c4f3092a794
pub const KEY_OUTPUT_CHANNEL: u64 = binding_hash("output_channel"); // 0x4ad76c5b9d4fae28
pub const HASH_SRC: u64 = binding_hash("water_mask"); // 0x6ad63ccd18c8f041
pub const HASH_DST: u64 = binding_hash("out_water_mask"); // 0xe8f7c3a7b57d0768

/// 一次 blur（O：两次调用只差源/目标与 `horizontal` 位）。
fn blur_pass(env: &mut PassEnv<'_>, horizontal: bool, temp_is_dst: bool, temp: &RgTexture) {
    let pass = env
        .pass(BLUR_NAME, BLUR_URI)
        .uniform(KEY_HORIZONTAL, horizontal);
    let mask = TexturePreset(MASK_PRESET);
    let pass = if temp_is_dst {
        pass.bind_texture(HASH_SRC, &mask, TEX_READ)
            .bind_texture(HASH_DST, &temp, TEX_WRITE)
    } else {
        pass.bind_texture(HASH_SRC, &temp, TEX_READ)
            .bind_texture(HASH_DST, &mask, TEX_WRITE)
    };
    // Original image RMW keeps the uploaded red and other preset channels.
    // The temporary's first write has no live old channels.
    let pass = if temp_is_dst { pass } else {
        pass.preserve_storage_texture_contents(HASH_DST)
    };
    let recorded = pass
        .uniform(KEY_SIGMA, BLUR_SIGMA)
        .uniform(KEY_OUTPUT_CHANNEL, 1i32)
        .dispatch_2d(MASK_GRID, MASK_GRID);
    env.record(recorded);
}

/// W12（O 0x140a22140）。返回 `Some(world_change_tick)`：脏时资源的 `changed_tick` 被刷新为它；不脏返回 `None`。
pub fn garden_rasterize_from_cpu(env: &mut PassEnv<'_>, world_change_tick: u32) -> Option<u32> {
    let raster = env.resource::<GardenRaster>();
    if !raster.0.mask_dirty.get() {
        return None;
    }
    raster.0.consume_mask_dirty();
    declare_garden_blur(env);
    Some(world_change_tick)
}

/// Real ECS mask and tick adapter; no cast to a different compiler's layout.
/// B140a221ee..22207 marks the row and consumes +29b/bitmap before pass creation.
pub fn garden_rasterize_live(
    env: &mut PassEnv<'_>,
    raster: &mut bevy_ecs::change_detection::Mut<'_, crate::resources::world_raster_defs::GardenRaster>,
    world_change_tick: u32,
) -> Option<u32> {
    use bevy_ecs::change_detection::DetectChangesMut;
    if !raster.bypass_change_detection().reupload_pending() { return None; }
    raster.set_changed();
    raster.bypass_change_detection().consume_reupload();
    declare_garden_blur(env);
    Some(world_change_tick)
}

fn declare_garden_blur(env: &mut PassEnv<'_>) {
    let desc = TextureDesc::new_2d([MASK_GRID, MASK_GRID], TEMP_FORMAT).mip_levels(1);
    let temp_tex = env.create_texture(TEMP_NAME, desc);
    blur_pass(env, true, true, &temp_tex);
    blur_pass(env, false, false, &temp_tex);
}

// ------------------------------------------------------------------ spawn

pub const HASH_PRESET9: u64 = binding_hash("terrain_heightmap"); // 0x61b08b5e0470c5f2
pub const HASH_PRESET_A: u64 = binding_hash("visual_terrain_heightmap"); // 0x297d1e4cdae8f657
pub const HASH_PRESET_14: u64 = binding_hash("garden_mask"); // 0x49f43a487883738d
pub const HASH_PRESET_16: u64 = binding_hash("buildings_mask_non_blurred"); // 0x6570181649ac6420
pub const HASH_PRESET_15: u64 = binding_hash("buildings_mask"); // 0x4a9aecb3eff0f013
/// `[u32;2]` uniform 键（`BindingNameHash::new("grid_dims")` = fnv("grid") + 1）。
pub const KEY_GRID: u64 = binding_hash("grid_dims"); // 0xfb539f7243dbb832
pub const KEY_GRID_STONES: u64 = binding_hash("contouring_grid_dims"); // 0xf32096f91a47fc71
pub const HASH_INDIRECT: u64 = binding_hash("draw_commands"); // 0xf1c07a2b8012765a
/// `i32` uniform：该 pass 的绘制命令在 `DrawIndexedIndirectBuffer` 里的偏移（槽位）。
pub const KEY_SLOT: u64 = binding_hash("draw_indirect_buffer_offset"); // 0x0e9511a616b75789
pub const HASH_BUSH_BUFFER: u64 = binding_hash("bush_buffer"); // 0x5baefac4a34b3dee
pub const HASH_BUSH_FLOWERS: u64 = binding_hash("bush_flowers"); // 0xf3aa86819743c1d8
pub const HASH_FLOWER_BUFFER: u64 = binding_hash("flower_buffer"); // 0x633d7e34d2dd23f1
pub const HASH_STONES: u64 = binding_hash("instance_buffer"); // 0x6d9256d4ac7887ad
pub const HASH_FLOWER03_BUFFER: u64 = binding_hash("flower03_buffer"); // 0x7369bcad63f3af6e
pub const CONCURRENT_GROUP: u32 = 1;

fn presets_9_a(pass: ComputePass) -> ComputePass {
    pass.bind_texture(HASH_PRESET9, &TexturePreset(9), TEX_READ)
        .bind_texture(HASH_PRESET_A, &TexturePreset(0xa), TEX_READ)
}

fn indirect_and_slot(pass: ComputePass, slot: i32) -> ComputePass {
    pass.write_storage_buffer_concurrent::<DrawIndexedIndirectBuffer>(
        HASH_INDIRECT,
        CONCURRENT_GROUP,
    )
    .uniform(KEY_SLOT, slot)
}

/// W16（O 0x140a22b80）：四个 pass。
pub fn garden_spawn_vegetation(env: &mut PassEnv<'_>) {
    // 1. bushes（90×90）
    let pass = env
        .pass("Spawn bushes", "/garden/spawn_bushes_garden.cs.hlsl")
        .write_storage_buffer_concurrent::<BushSsbo>(HASH_BUSH_BUFFER, CONCURRENT_GROUP)
        .write_storage_buffer_concurrent::<FlowerSsbo>(HASH_BUSH_FLOWERS, CONCURRENT_GROUP)
        .uniform(KEY_GRID, [90u32, 90]);
    let pass = bind_garden_exclusion_compute(presets_9_a(pass))
        .bind_texture(HASH_PRESET_14, &TexturePreset(0x14), TEX_READ)
        .bind_texture(HASH_PRESET_16, &TexturePreset(0x16), TEX_READ);
    let rec = indirect_and_slot(pass, 10).dispatch_2d(90, 90);
    env.record(rec);

    // 2. flowers（150×150）
    let pass = env
        .pass("Spawn flowers", "/garden/spawn_flowers_garden.cs.hlsl")
        .write_storage_buffer_concurrent::<FlowerSsbo>(HASH_FLOWER_BUFFER, CONCURRENT_GROUP)
        .uniform(KEY_GRID, [150u32, 150]);
    let pass = bind_garden_exclusion_compute(presets_9_a(pass))
        .bind_texture(HASH_PRESET_14, &TexturePreset(0x14), TEX_READ)
        .bind_texture(HASH_PRESET_16, &TexturePreset(0x16), TEX_READ);
    let rec = indirect_and_slot(pass, 11).dispatch_2d(150, 150);
    env.record(rec);

    // 3. marching squares / stones（180×180）：exclusion 与 0x14 在前
    let pass = bind_garden_exclusion_compute(env.pass(
        "Garden Marching Squares",
        "/garden/marching_squares.cs.hlsl",
    ))
    .bind_texture(HASH_PRESET_14, &TexturePreset(0x14), TEX_READ)
    .write_storage_buffer::<GardenStoneSsbo>(HASH_STONES)
    .uniform(KEY_GRID_STONES, [180u32, 180]);
    let rec = indirect_and_slot(presets_9_a(pass), 14).dispatch_2d(180, 180);
    env.record(rec);

    // 4. small flowers（400×400）
    let pass = env
        .pass(
            "Spawn small flowers",
            "/garden/spawn_small_flowers_garden.cs.hlsl",
        )
        .write_storage_buffer::<Flower03Ssbo>(HASH_FLOWER03_BUFFER)
        .uniform(KEY_GRID, [400u32, 400]);
    let pass = bind_garden_exclusion_compute(presets_9_a(pass))
        .bind_texture(HASH_PRESET_14, &TexturePreset(0x14), TEX_READ)
        .bind_texture(HASH_PRESET_15, &TexturePreset(0x15), TEX_READ);
    let rec = indirect_and_slot(pass, 16).dispatch_2d(400, 400);
    env.record(rec);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::systems::render_loop::compute_passes::pass_kit::ResourceView;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;
    use crate::systems::render_loop::compute_passes::upload_world_raster::RasterFlags;
    use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::GARDEN_EXCLUSION;

    fn rast_world(dirty: bool) -> TestWorld {
        TestWorld::new(Reflection::new().bind(&[HASH_SRC, HASH_DST]).uniforms(&[
            KEY_HORIZONTAL,
            KEY_SIGMA,
            KEY_OUTPUT_CHANNEL,
        ]))
        .with_resource(GardenRaster(RasterFlags::new(false, dirty)))
    }

    #[test]
    fn clean_staging_does_nothing() {
        let w = rast_world(false);
        let mut f = TestFrame::default();
        assert_eq!(
            with_env(&w, &mut f, |env| garden_rasterize_from_cpu(env, 77)),
            None
        );
        assert!(f.passes.is_empty() && f.textures.is_empty());
    }

    #[test]
    fn dirty_staging_clears_flags_then_blurs_horizontally_then_vertically() {
        let w = rast_world(true);
        let mut f = TestFrame::default();
        assert_eq!(
            with_env(&w, &mut f, |env| garden_rasterize_from_cpu(env, 4242)),
            Some(4242)
        );
        let staging = unsafe {
            &*(crate::render::pass_builder::pass_resources::ResourceLookup::get_resource_ptr(
                &w,
                GardenRaster::TYPE_ID,
            )
            .unwrap() as *const GardenRaster)
        };
        assert!(!staging.0.mask_dirty.get());
        assert!(
            staging.0.dirty_rows.iter().all(|b| b.get() == 0),
            "memset(+0x98, 0, 0x200)"
        );
        assert_eq!(f.textures.len(), 1);
        assert_eq!(f.textures[0].0, "garden blur temp");
        assert_eq!(
            f.textures[0].1,
            TextureDesc::new_2d([512, 512], 0x61).mip_levels(1)
        );
        assert_eq!(f.passes.len(), 2);
        for p in &f.passes {
            assert_eq!(p.name, "Blur garden mask");
            assert!(matches!(
                p.dispatch,
                DispatchMode::Threads {
                    x: 512,
                    y: 512,
                    z: 1
                }
            ));
            assert_eq!(f32::from_bits(uniform_u32(p, 16)), 5.0);
            assert_eq!(uniform_u32(p, 32), 1);
            // 访问：color library、读 0x19、写 0x32。
            let acc: Vec<u8> = p.builder.accesses.iter().map(|a| a.access).collect();
            assert_eq!(acc, [0x1a, 0x19, 0x32]);
        }
        assert_eq!(uniform_u32(&f.passes[0], 0), 1, "水平");
        assert_eq!(uniform_u32(&f.passes[1], 0), 0, "垂直");
        // 第一次写临时纹理，第二次读临时纹理。
        let temp = f.passes[0].builder.accesses[2].resource;
        assert_eq!(f.passes[1].builder.accesses[1].resource, temp);
        assert_ne!(
            f.passes[0].builder.accesses[1].resource, temp,
            "源是 preset 0x14"
        );
        assert_eq!(
            f.passes[1].builder.accesses[2].resource,
            f.passes[0].builder.accesses[1].resource
        );
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_staging_resource_unwraps() {
        let w = TestWorld::new(Reflection::new());
        let mut f = TestFrame::default();
        with_env(&w, &mut f, |env| garden_rasterize_from_cpu(env, 0));
    }

    fn spawn_world() -> TestWorld {
        let mut h = vec![
            0x5baefac4a34b3dee,
            0xf3aa86819743c1d8,
            0x633d7e34d2dd23f1,
            0x6d9256d4ac7887ad,
            0x7369bcad63f3af6e,
            HASH_PRESET9,
            HASH_PRESET_A,
            HASH_PRESET_14,
            HASH_PRESET_15,
            HASH_PRESET_16,
            HASH_INDIRECT,
        ];
        h.extend(GARDEN_EXCLUSION.map(|(_, hash)| hash));
        TestWorld::new(
            Reflection::new()
                .bind(&h)
                .uniforms(&[KEY_GRID, KEY_GRID_STONES, KEY_SLOT]),
        )
        .with_ssbo::<BushSsbo>()
        .with_ssbo::<FlowerSsbo>()
        .with_ssbo::<Flower03Ssbo>()
        .with_ssbo::<GardenStoneSsbo>()
        .with_ssbo::<DrawIndexedIndirectBuffer>()
    }

    #[test]
    fn four_spawn_passes_in_order_with_grid_sizes() {
        let w = spawn_world();
        let mut f = TestFrame::default();
        with_env(&w, &mut f, garden_spawn_vegetation);
        assert_eq!(
            f.order,
            [
                "Spawn bushes",
                "Spawn flowers",
                "Garden Marching Squares",
                "Spawn small flowers"
            ]
        );
        let paths = w.backend.paths.borrow().clone();
        assert_eq!(
            paths,
            [
                "/garden/spawn_bushes_garden.cs.hlsl:cs_main",
                "/garden/spawn_flowers_garden.cs.hlsl:cs_main",
                "/garden/marching_squares.cs.hlsl:cs_main",
                "/garden/spawn_small_flowers_garden.cs.hlsl:cs_main"
            ]
        );
        let dims: Vec<u32> = f
            .passes
            .iter()
            .map(|p| match p.dispatch {
                DispatchMode::Threads { x, y, z: 1 } if x == y => x,
                _ => panic!("expected square 2d dispatch"),
            })
            .collect();
        assert_eq!(dims, [90, 150, 180, 400]);
        // [u32;2] uniform @0 = (n, n)；i32 slot @32/…（声明顺序：KEY_GRID(0), KEY_GRID_STONES(16), KEY_SLOT(32)）。
        let u64_at =
            |p: &crate::render::pass_builder::compute_pass_builder::RecordedComputePass,
             o: usize| {
                let d = p.uniform_binder.as_ref().unwrap().data();
                (
                    u32::from_le_bytes(d[o..o + 4].try_into().unwrap()),
                    u32::from_le_bytes(d[o + 4..o + 8].try_into().unwrap()),
                )
            };
        assert_eq!(u64_at(&f.passes[0], 0), (90, 90));
        assert_eq!(u64_at(&f.passes[1], 0), (150, 150));
        assert_eq!(u64_at(&f.passes[2], 16), (180, 180), "stones 用另一把键");
        assert_eq!(u64_at(&f.passes[3], 0), (400, 400));
        let slots: Vec<u32> = f.passes.iter().map(|p| uniform_u32(p, 32)).collect();
        assert_eq!(slots, [10, 11, 14, 16]);
    }

    #[test]
    fn spawn_pass_access_sequences() {
        let w = spawn_world();
        let mut f = TestFrame::default();
        with_env(&w, &mut f, garden_spawn_vegetation);
        let acc = |i: usize| {
            f.passes[i]
                .builder
                .accesses
                .iter()
                .map(|a| (a.access, a.group))
                .collect::<Vec<_>>()
        };
        // bushes：color(0x1a,1)、两个并发写(0x37,1：A4 `write_storage_buffer_concurrent` 访问码，O 0x141608450)、
        // 预设 9/a、exclusion×4、0x14、0x16（均读 0x19,1）、indirect(0x37,1)。
        let mut want = vec![(0x1a, 1), (0x37, 1), (0x37, 1)];
        want.extend(std::iter::repeat((0x19, 1)).take(2 + 4 + 2));
        want.push((0x37, 1));
        assert_eq!(acc(0), want);
        // stones：exclusion×4、0x14 先于 GardenStone 写(0x32,0 = 非并发)。
        let a2 = acc(2);
        assert_eq!(
            &a2[..6],
            &[
                (0x1a, 1),
                (0x19, 1),
                (0x19, 1),
                (0x19, 1),
                (0x19, 1),
                (0x19, 1)
            ]
        );
        assert_eq!(a2[6], (0x32, 0));
        // small flowers：Flower03 写非并发；末尾 0x15 在 indirect 之前。
        let a3 = acc(3);
        assert_eq!(a3[1], (0x32, 0));
        assert_eq!(*a3.last().unwrap(), (0x37, 1));
    }

    #[test]
    fn constants_match_machine_code() {
        assert_eq!(KEY_HORIZONTAL, (-0x26e007c8df6b43de_i64) as u64);
        assert_eq!(HASH_DST, (-0x17083c584a82f898_i64) as u64);
        assert_eq!(KEY_SIGMA, (-0x23fdd3b0cf6d586c_i64) as u64);
        assert_eq!(KEY_GRID, (-0x4ac608dbc2447ce_i64) as u64);
        assert_eq!(KEY_GRID_STONES, (-0xcdf6906e5b8038f_i64) as u64);
        assert_eq!(HASH_INDIRECT, (-0xe3f85d47fed89a6_i64) as u64);
        assert_eq!(GardenRaster::TYPE_ID.lo, (-0x6860059d7755da66_i64) as u64);
        assert_eq!(GardenRaster::TYPE_ID.hi, 0x09791d5b46ed449d);
        assert_eq!(HASH_BUSH_BUFFER, 0x5baefac4a34b3dee);
        assert_eq!(HASH_BUSH_FLOWERS, 0xf3aa86819743c1d8);
        assert_eq!(HASH_FLOWER_BUFFER, 0x633d7e34d2dd23f1);
        assert_eq!(HASH_STONES, 0x6d9256d4ac7887ad);
        assert_eq!(HASH_FLOWER03_BUFFER, 0x7369bcad63f3af6e);
        assert_eq!(HASH_PRESET_14, 0x49f43a487883738d);
        assert_eq!(HASH_PRESET_15, 0x4a9aecb3eff0f013);
        assert_eq!(HASH_PRESET_16, 0x6570181649ac6420);
        assert_eq!(KEY_SLOT, 0x0e9511a616b75789);
        assert_eq!(KEY_OUTPUT_CHANNEL, 0x4ad76c5b9d4fae28);
        assert_eq!(HASH_SRC, 0x6ad63ccd18c8f041);
        assert_eq!(BLUR_NAME.len(), 0x10);
        assert_eq!(BLUR_URI.len(), 0x18);
        assert_eq!(TEMP_NAME.len(), 0x10);
    }

    // ---- BushSsbo / FlowerSsbo：真实库侧类型（O 0x141610b60 / 0x141613b90）

    #[test]
    fn bush_and_flower_elements_match_the_buffer_sizes_in_the_machine_code() {
        use core::mem::size_of;
        assert_eq!(size_of::<BushSsbo>(), 48);
        assert_eq!(size_of::<FlowerSsbo>(), 64);
        assert_eq!(BushSsbo::BUFFER_LEN * size_of::<BushSsbo>(), 0x70800);
        assert_eq!(FlowerSsbo::BUFFER_LEN * size_of::<FlowerSsbo>(), 0x33e000);
        assert_eq!((BushSsbo::NAME, FlowerSsbo::NAME), ("Bushes", "FlowerSsbo"));
    }

    #[test]
    fn bush_and_flower_buffers_are_fixed_size_in_the_library() {
        use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
        use crate::render::test_device::TestDevice as Device;
        let lib = AssetSsboLibrary::new();
        lib.add_new::<BushSsbo>(&Device(1));
        lib.add_new::<FlowerSsbo>(&Device(1));
        assert_eq!(lib.get::<BushSsbo>().unwrap().buffer.byte_size, 0x70800);
        assert_eq!(lib.get::<FlowerSsbo>().unwrap().buffer.byte_size, 0x33e000);
    }

    #[test]
    #[should_panic(expected = "Excessive data size supplied for buffer")]
    fn bush_initial_data_above_capacity_panics() {
        use crate::render::test_device::TestDevice as Device;
        // 9601 个元素（`len >= 0x70830` → panic）；u32 向量保证 4 字节对齐。
        let words = vec![0u32; (BushSsbo::BUFFER_LEN + 1) * 12];
        let bytes =
            unsafe { core::slice::from_raw_parts(words.as_ptr() as *const u8, words.len() * 4) };
        let _ = <BushSsbo as LibrarySsbo>::new_storage_buffer(&Device(1), Some(bytes));
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn flower_initial_data_must_be_a_whole_number_of_elements() {
        use crate::render::test_device::TestDevice as Device;
        let words = vec![0u32; 17];
        let bytes = unsafe { core::slice::from_raw_parts(words.as_ptr() as *const u8, 17 * 4) };
        let _ = <FlowerSsbo as LibrarySsbo>::new_storage_buffer(&Device(1), Some(bytes));
    }
}
