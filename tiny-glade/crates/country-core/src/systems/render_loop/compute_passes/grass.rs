//! `grass.rs` — W29 GPU 草叶生成（O：`grass_compute` 0x140a60280，8537 B；`ObjectGrassPush` 的 `Component` 实现
//! `ObjectGrassPush` 是**单个 f32**（O：`object_grass_push_pass` 的 map 闭包 `movss xmm1,[r8]`；`register_component<ObjectGrassPush>`
//! 0x140a5fc10 的 `Layout` = size 4 / align 4）。
//!
//! `impl$4::register_required_components` 0x140a66970 / `clone_behavior` 0x140002370）。
//!
//! ## grass_compute（参数 O：`(ctx, world, hzb: &Hzb)`；`hzb` = `generate_hzb` 的返回值：`[+0]` 半尺寸、`[+8..]` 纹理句柄）
//! 缓冲（均 `new_gpu_only(size, usage)`，usage 除 tile 缓冲为 0x20 外均为 0）：
//! `tile_buffer`（**temporal** "grass_tile_buffer"，0x10000 B，usage 0x20；`Err` → `Result::unwrap` panic）、
//! `blade_prefix_buffer`（0x10000）、`blade_prefix_subtotal_buffer`（0x404）、`tile_bundle_prefix_buffer`（0x40000）、
//! `tile_bundle_subtotal_buffer`（0x404）、`bundle_buffer`（0x80000）、`bundle_grasslet_index_buffer`（0x80000）、
//! `grass_dispatch_args_buffer`（0x20）、`grasslet_bundle_ranges`（0x40000，随即 `clear_rg_buffer`）。
//!
//! 八个 pass（顺序即机器码顺序；`⟵` 读、`⟶` 写）：
//! 1. **"Grass calc tiles"**（`/grass/calc_tiles.cs.hlsl`）：纹理读 preset 9 `terrain_heightmap`、0xa `visual_terrain_heightmap`、
//!    `hzb_tex`（hzb）；`write_uniform<[u32;2]>`(`hzb_tex_inner_dims`, hzb 半尺寸)；`write_uniform<bool>`(`freeze_occlusion_culling`,
//!    静态调试开关 `[0x143634da0]`)；⟶ `tile_buffer`、`blade_prefix_buffer`、`blade_prefix_subtotal_buffer`；`dispatch_1d(0x4000)`。
//! 2. **"Grass prefix tiles"**（`tiles_prefix`）：⟶ `blade_prefix_subtotal_buffer`；`dispatch_groups(1,1,1)`。
//! 3. **"Grass tile bundle count"**（`tile_bundle_count`）：⟶ `tile_bundle_prefix_buffer`、`tile_bundle_subtotal_buffer`；
//!    ⟵ `tile_buffer`、`blade_prefix_buffer`、`blade_prefix_subtotal_buffer`；`dispatch_1d(0x4000)`。
//! 4. **"Grass prefix bundles"**（`bundles_prefix`）：⟶ `tile_bundle_subtotal_buffer`；`dispatch_groups(1,1,1)`。
//! 5. **"Grass tile bundle gen"**（`tile_bundle_gen`）：⟶ `bundle_buffer`、`bundle_grasslet_index_buffer`；
//!    ⟵ `tile_bundle_prefix_buffer`、`tile_bundle_subtotal_buffer`、`tile_buffer`、`blade_prefix_buffer`、`blade_prefix_subtotal_buffer`；
//!    `dispatch_1d(0x4000)`。随后 `prefix_scan::inclusive_prefix_scan_u32_1m(ctx, world, bundle_grasslet_index_buffer)`。
//! 6. **"Grass dispatch args"**（`calc_dispatch_args`）：⟶ `grass_dispatch_args_buffer`；⟵ `bundle_grasslet_index_buffer`、
//!    `tile_bundle_subtotal_buffer`；`dispatch_1d(1)`。
//! 7. **"Grass bundle ranges"**（`calc_grasslet_bundle_ranges`）：⟶ `grasslet_bundle_ranges`；⟵ `grass_dispatch_args_buffer`、
//!    `bundle_grasslet_index_buffer`；`dispatch_indirect_raw(grass_dispatch_args_buffer, 0x10)`。
//! 8. **"Grass"**（`/generate_grass.cs.hlsl`）：⟵ `tile_buffer`、`bundle_buffer`、`grasslet_bundle_ranges`；
//!    `write_storage<GrassBladeDataSsbo>`(`grass_blade_buffer`)；`write_storage_concurrent<DrawIndexedIndirectBuffer>`(`draw_commands`, 组 1) +
//!    `i32 draw_indirect_buffer_offset = 0`；`write_storage<DrawIndirectBuffer>`(`lod1_commands`) + `i32 lod1_commands_offset = 0`；
//!    `write_storage<AtomicCounterBuffer>`(`atomics`) + `i32 grass_buffer_size = 0x90000`；
//!    取实际 `AppMode`（TypeId 0xc2b8e941c91a6428/0x59f3c6d1d3da43a0，缺失 → unwrap panic）首个 u32 `t`：
//!    `brush_index = BRUSH_TABLE[t - 7]`（`t ∈ 7..=9`），其余 0 → `i32 brush_index`；
//!    纹理读：preset 0xe `rocky_terrain`、0x13 `water_mask`、0x15 `buildings_mask`、0x10 `foliage_exclusion_mask`、9、0xa、
//!    5 `path_mask`、0x14 `garden_mask`、0xf `grass_object_push`，以及 `TextureFromDisk(0x8991bdb39a048ac5)` → `bg_terrain_heightmap`；
//!    `dispatch_indirect_raw(grass_dispatch_args_buffer, 0)`。
//!
//! SSBO 名（O：SsboId 与 xxh3_64 比对）："GrassBlades"（`GrassBladeDataSsbo`）。
//! `TextureFromDisk` 经 A4 真实的 `bind_texture`（外部描述符 `ExternalSlot::Image`，不登记 RG 访问）。
//! 预扫描 `inclusive_prefix_scan_u32_1m`（`compute_passes::prefix_scan`，B 阶段 i_infra 接线）：用
//! `SizedRgBuffer { buffer: bundle_index, size_bytes: 0x80000 }` 直接调用，并按 `pass1 → pass2 → merge` 顺序 `env.record`
//! （原版该函数在内部把三个 pass 交给 recorder；重建的 `prefix_scan` 返回 pass，登记由调用方做）。
//!
//! ## ObjectGrassPush 的 Component 实现
//! `ObjectGrassPush` 是**单个 f32**（O：`object_grass_push_pass` 的 map 闭包 `movss xmm1,[r8]`；`register_component<ObjectGrassPush>`
//! 0x140a5fc10 的 `Layout` = size 4 / align 4）。
//!
//! `impl$4::register_required_components`（O）：`enforce_no_required_components_recursion` → `register_component::<ObjectGrassPush>`
//! → 压入递归检查栈 → 弹出（无 `#[require]`）= `#[derive(Component)]` 对无 require 的类型的展开；`clone_behavior`（3 B）= 默认。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::bool_assert_comparison)]
use bevy_ecs::component::Component;

use crate::render::pass_builder::compute_pass_builder::{ComputePass, RgBuffer};
use crate::render::pass_builder::pass_resources::TypeId128;
use crate::render::rg_helpers::clear_rg_buffer;
use crate::resources::texture_presets::{TextureFromDisk, TexturePreset};
use crate::systems::render_loop::compute_passes::generate_hzb::Hzb;
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, ResourceView, TEX_READ, binding_hash,
};
use crate::systems::render_loop::compute_passes::prefix_scan::{
    SizedRgBuffer, inclusive_prefix_scan_u32_1m,
};
use crate::systems::render_loop::indirect_params::{
    AtomicCounterBuffer, DrawIndexedIndirectBuffer, DrawIndirectBuffer,
};

/// B ctor@141611980: 32-byte GPU-produced record, 0x90000 elements.
/// Word payload preserves the recovered storage layout without guessing CPU
/// field names. Grass generation and its draw shader own the actual contents.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GrassBladeDataSsbo {
    pub words: [u32; 8],
}
impl crate::render::ssbo::StorageElement for GrassBladeDataSsbo {
    const NAME: &'static str = "GrassBlades";
    const BUFFER_LEN: usize = 0x90000;
}
impl crate::asset_libraries::ssbo_library::SsboType for GrassBladeDataSsbo {
    type Storage = crate::render::ssbo::TypedStorageBuffer<Self>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        use crate::render::ssbo::rhapsody_boundary::BufferDesc;
        if let Some(bytes) = initial {
            assert_eq!(bytes.len() % 32, 0);
            assert_eq!(bytes.as_ptr() as usize % 4, 0);
            assert!(
                bytes.len() < 0x1200020,
                "Excessive data size supplied for GrassBladeDataSsbo"
            );
        }
        // None is uninitialized; the original skips temporary zero data.
        let buffer = device
            .create_buffer(
                &BufferDesc::new(0x1200000, 0x22, 1).alignment(4),
                "GrassBlades",
                initial,
            )
            .expect("creating original GrassBlades buffer");
        crate::render::ssbo::TypedStorageBuffer::from_created_buffer(buffer)
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for GrassBladeDataSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
            .0
    }
}

/// 物体草 push 数据（O：单个 f32——`movss xmm1,[r8]`、`Layout` 4/4；`#[derive(Component)]`，无 require）。
/// 查询项 `(&Transform, &ObjectGrassPush)` → `Vec4(translation, push)`（见 `draw_passes::object_grass_push`）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct ObjectGrassPush(pub f32);

/// 旧原生偏移兼容视图；实际身份为 AppMode，不在当前 App 插入第二资源（O：TypeId 0xc2b8e941c91a6428/0x59f3c6d1d3da43a0；首个 u32 = 模式 tag）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GladeThemeId(pub u32);

// SAFETY: 首个 u32 被读取（O：`mov ecx, [rax]`）。
unsafe impl ResourceView for GladeThemeId {
    const TYPE_ID: TypeId128 = TypeId128 {
        lo: 0xc2b8e941c91a6428,
        hi: 0x59f3c6d1d3da43a0,
    };
}

pub const TILE_THREADS: u32 = 0x4000;
pub const TEMPORAL_TILE_NAME: &str = "grass_tile_buffer";
pub const TILE_BUFFER_SIZE: u32 = 0x10000;
pub const TILE_BUFFER_USAGE: u32 = 0x20;
/// 主题 → 笔刷下标表（O：`.rodata` 0x142ad48d8，3 个 u32 = `{1, 1, 2}`），索引 = 主题 id - 7。
pub const BRUSH_TABLE: [u32; 3] = [1, 1, 2];
/// `grass_buffer_size` uniform（O：`mov r9d, 0x90000`）。
pub const GRASS_BUFFER_SIZE: i32 = 0x90000;
/// 磁盘纹理 `bg_terrain_heightmap`（O：`movabs rax, -0x766e424c65fb753b`）。
pub const BG_TERRAIN_HEIGHTMAP: TextureFromDisk = TextureFromDisk(0x8991bdb39a048ac5);
pub const DISPATCH_ARGS_BYTES: u32 = 0x20;
/// 第 7 个 pass 在 `grass_dispatch_args_buffer` 里的间接参数偏移（O：`mov r8d, 0x10`），第 8 个为 0。
pub const RANGES_INDIRECT_OFFSET: u64 = 0x10;

pub const HASH_TERRAIN: u64 = binding_hash("terrain_heightmap");
pub const HASH_VISUAL_TERRAIN: u64 = binding_hash("visual_terrain_heightmap");
pub const HASH_HZB: u64 = binding_hash("hzb_tex"); // 0x62b6812f89a8a569
pub const KEY_HZB_DIMS: u64 = binding_hash("hzb_tex_inner_dims"); // 0xe8d23613960341d3
pub const KEY_FREEZE: u64 = binding_hash("freeze_occlusion_culling"); // 0x6d7707561ec0da2b
pub const HASH_TILE: u64 = binding_hash("tile_buffer"); // 0x67b935e4d4a7dd6c
pub const HASH_BLADE_PREFIX: u64 = binding_hash("blade_prefix_buffer"); // 0x8c6079871c46ed39
pub const HASH_BLADE_SUBTOTAL: u64 = binding_hash("blade_prefix_subtotal_buffer"); // 0x3f3ee7377aa977ba
pub const HASH_BUNDLE_PREFIX: u64 = binding_hash("tile_bundle_prefix_buffer"); // 0x3fc2c9e8ccc0b77e
pub const HASH_BUNDLE_SUBTOTAL: u64 = binding_hash("tile_bundle_subtotal_buffer"); // 0x73d3886611f6f8de
pub const HASH_BUNDLE: u64 = binding_hash("bundle_buffer"); // 0x49447d0bbff37d38
pub const HASH_BUNDLE_INDEX: u64 = binding_hash("bundle_grasslet_index_buffer"); // 0x14b3d567dc6b16d9
pub const HASH_DISPATCH_ARGS: u64 = binding_hash("grass_dispatch_args_buffer"); // 0xdabaf5ab2db30c87
pub const HASH_RANGES: u64 = binding_hash("grasslet_bundle_ranges"); // 0x1a0da9d576285352
pub const HASH_BLADES: u64 = binding_hash("grass_blade_buffer"); // 0x9d0168095bc840f3
pub const HASH_DRAW_COMMANDS: u64 = binding_hash("draw_commands"); // 0xf1c07a2b8012765a
pub const KEY_DRAW_OFFSET: u64 = binding_hash("draw_indirect_buffer_offset"); // 0x0e9511a616b75789
pub const HASH_LOD1: u64 = binding_hash("lod1_commands"); // 0x6932ed479ccd8292
pub const KEY_LOD1_OFFSET: u64 = binding_hash("lod1_commands_offset"); // 0x3957088263abad4c
pub const HASH_ATOMICS: u64 = binding_hash("atomics"); // 0x83cdc731ea452909
pub const KEY_BUFFER_SIZE: u64 = binding_hash("grass_buffer_size"); // 0xb06e4bc7fd9f1694
pub const KEY_BRUSH: u64 = binding_hash("brush_index"); // 0xdadac8f5a39d5a40
pub const HASH_BG_TERRAIN: u64 = binding_hash("bg_terrain_heightmap"); // 0x5b725035d0873550

/// 主题 id → `brush_index`（O：`ecx - 6 < 15` 时 `idx = ecx - 7`（64 位回绕），`idx <= 2` 取表，否则 0）。
pub fn brush_index(theme: u32) -> u32 {
    let idx: u64 = if theme.wrapping_sub(6) < 15 {
        u64::from(theme).wrapping_sub(7)
    } else {
        u64::MAX - 1
    };
    if idx <= 2 {
        BRUSH_TABLE[idx as usize]
    } else {
        0
    }
}

fn buf(env: &mut PassEnv<'_>, name: &'static str, size: u32) -> RgBuffer {
    env.create_buffer(name, size, 0)
}

/// 记录一个 pass（`PassBuilder::render`）。
fn record(
    env: &mut PassEnv<'_>,
    pass: crate::render::pass_builder::compute_pass_builder::RecordedComputePass,
) {
    env.record(pass);
}

/// `bundle_grasslet_index_buffer` 的字节数（预扫描入参；O：`create_raw_resource` 的大小 0x80000）。
pub const BUNDLE_INDEX_BYTES: u64 = 0x80000;

/// W29 入口（O 0x140a60280）。在 "Grass tile bundle gen" 之后调用 `prefix_scan::inclusive_prefix_scan_u32_1m`
/// 对 `bundle_grasslet_index_buffer` 做前缀和（三个 pass 依次登记）。
pub fn grass_compute(env: &mut PassEnv<'_>, hzb: &Hzb, freeze_occlusion_culling: bool) {
    grass_compute_impl(env, hzb, freeze_occlusion_culling, |env| env.resource::<GladeThemeId>().0)
}

/// Current typed AppMode projection; no partial-layout resource is installed.
pub fn grass_compute_with_app_mode(
    env: &mut PassEnv<'_>, hzb: &Hzb, freeze_occlusion_culling: bool, app_mode: u32,
) {
    grass_compute_impl(env, hzb, freeze_occlusion_culling, |_| app_mode)
}

fn grass_compute_impl(
    env: &mut PassEnv<'_>, hzb: &Hzb, freeze_occlusion_culling: bool, mode: impl FnOnce(&PassEnv<'_>) -> u32,
) {
    // 缓冲（先 temporal，再两个，pass 1 之前）。
    let tile = env
        .temporal_buffer(TEMPORAL_TILE_NAME, TILE_BUFFER_SIZE, TILE_BUFFER_USAGE)
        .buffer;
    let blade_prefix = buf(env, "blade_prefix_buffer", 0x10000);
    let blade_subtotal = buf(env, "blade_prefix_subtotal_buffer", 0x404);
    let hzb_tex = hzb.texture;

    // 1. calc tiles
    let p = env
        .pass("Grass calc tiles", "/grass/calc_tiles.cs.hlsl")
        .bind_texture(HASH_TERRAIN, &TexturePreset(9), TEX_READ)
        .bind_texture(HASH_VISUAL_TERRAIN, &TexturePreset(0xa), TEX_READ)
        .bind_texture(HASH_HZB, &&hzb_tex, TEX_READ)
        .uniform(KEY_HZB_DIMS, hzb.half_extent)
        .uniform(KEY_FREEZE, freeze_occlusion_culling)
        .write_rg_storage_buffer(HASH_TILE, &tile)
        .write_rg_storage_buffer(HASH_BLADE_PREFIX, &blade_prefix)
        .write_rg_storage_buffer(HASH_BLADE_SUBTOTAL, &blade_subtotal)
        .dispatch_1d(TILE_THREADS);
    record(env, p);

    // 2. prefix tiles
    let p = env
        .pass("Grass prefix tiles", "/grass/tiles_prefix.cs.hlsl")
        .write_rg_storage_buffer(HASH_BLADE_SUBTOTAL, &blade_subtotal)
        .preserve_storage_buffer_contents(HASH_BLADE_SUBTOTAL)
        .dispatch_groups(1, 1, 1);
    record(env, p);

    // 3. tile bundle count
    let bundle_prefix = buf(env, "tile_bundle_prefix_buffer", 0x40000);
    let bundle_subtotal = buf(env, "tile_bundle_subtotal_buffer", 0x404);
    let p = env
        .pass(
            "Grass tile bundle count",
            "/grass/tile_bundle_count.cs.hlsl",
        )
        .write_rg_storage_buffer(HASH_BUNDLE_PREFIX, &bundle_prefix)
        .write_rg_storage_buffer(HASH_BUNDLE_SUBTOTAL, &bundle_subtotal)
        .read_rg_storage_buffer(HASH_TILE, &tile)
        .read_rg_storage_buffer(HASH_BLADE_PREFIX, &blade_prefix)
        .read_rg_storage_buffer(HASH_BLADE_SUBTOTAL, &blade_subtotal)
        .dispatch_1d(TILE_THREADS);
    record(env, p);

    // 4. prefix bundles
    let p = env
        .pass("Grass prefix bundles", "/grass/bundles_prefix.cs.hlsl")
        .write_rg_storage_buffer(HASH_BUNDLE_SUBTOTAL, &bundle_subtotal)
        .preserve_storage_buffer_contents(HASH_BUNDLE_SUBTOTAL)
        .dispatch_groups(1, 1, 1);
    record(env, p);

    // 5. tile bundle gen
    let bundles = buf(env, "bundle_buffer", 0x80000);
    let bundle_index = buf(env, "bundle_grasslet_index_buffer", 0x80000);
    let p = env
        .pass("Grass tile bundle gen", "/grass/tile_bundle_gen.cs.hlsl")
        .write_rg_storage_buffer(HASH_BUNDLE, &bundles)
        .write_rg_storage_buffer(HASH_BUNDLE_INDEX, &bundle_index)
        .read_rg_storage_buffer(HASH_BUNDLE_PREFIX, &bundle_prefix)
        .read_rg_storage_buffer(HASH_BUNDLE_SUBTOTAL, &bundle_subtotal)
        .read_rg_storage_buffer(HASH_TILE, &tile)
        .read_rg_storage_buffer(HASH_BLADE_PREFIX, &blade_prefix)
        .read_rg_storage_buffer(HASH_BLADE_SUBTOTAL, &blade_subtotal)
        .dispatch_1d(TILE_THREADS);
    record(env, p);
    let scan = inclusive_prefix_scan_u32_1m(
        env,
        &SizedRgBuffer {
            buffer: bundle_index,
            size_bytes: BUNDLE_INDEX_BYTES,
        },
    );
    env.record(scan.pass1);
    env.record(scan.pass2);
    env.record(scan.merge);

    // 6. dispatch args
    let args = buf(env, "grass_dispatch_args_buffer", DISPATCH_ARGS_BYTES);
    let p = env
        .pass("Grass dispatch args", "/grass/calc_dispatch_args.cs.hlsl")
        .write_rg_storage_buffer(HASH_DISPATCH_ARGS, &args)
        .read_rg_storage_buffer(HASH_BUNDLE_INDEX, &bundle_index)
        .read_rg_storage_buffer(HASH_BUNDLE_SUBTOTAL, &bundle_subtotal)
        .dispatch_1d(1);
    record(env, p);

    // 7. bundle ranges（先清零）
    let ranges = buf(env, "grasslet_bundle_ranges", 0x40000);
    clear_rg_buffer(&mut *env.frame, ranges.resource);
    let p = env
        .pass(
            "Grass bundle ranges",
            "/grass/calc_grasslet_bundle_ranges.cs.hlsl",
        )
        .write_rg_storage_buffer(HASH_RANGES, &ranges)
        .preserve_storage_buffer_contents(HASH_RANGES)
        .read_rg_storage_buffer(HASH_DISPATCH_ARGS, &args)
        .read_rg_storage_buffer(HASH_BUNDLE_INDEX, &bundle_index)
        .dispatch_indirect_raw(&args, RANGES_INDIRECT_OFFSET);
    record(env, p);

    // 8. generate grass
    let theme = mode(env);
    let mut p = env
        .pass("Grass", "/generate_grass.cs.hlsl")
        .read_rg_storage_buffer(HASH_TILE, &tile)
        .read_rg_storage_buffer(HASH_BUNDLE, &bundles)
        .read_rg_storage_buffer(HASH_RANGES, &ranges)
        .write_storage_buffer::<GrassBladeDataSsbo>(HASH_BLADES)
        .write_storage_buffer_concurrent::<DrawIndexedIndirectBuffer>(HASH_DRAW_COMMANDS, 1)
        .preserve_storage_buffer_contents(HASH_DRAW_COMMANDS)
        .uniform(KEY_DRAW_OFFSET, 0i32)
        .write_storage_buffer::<DrawIndirectBuffer>(HASH_LOD1)
        .preserve_storage_buffer_contents(HASH_LOD1)
        .uniform(KEY_LOD1_OFFSET, 0i32)
        .write_storage_buffer::<AtomicCounterBuffer>(HASH_ATOMICS)
        .preserve_storage_buffer_contents(HASH_ATOMICS)
        .uniform(KEY_BUFFER_SIZE, GRASS_BUFFER_SIZE)
        .uniform(KEY_BRUSH, brush_index(theme) as i32);
    for (name, preset) in [
        ("rocky_terrain", 0xe),
        ("water_mask", 0x13),
        ("buildings_mask", 0x15),
        ("foliage_exclusion_mask", 0x10),
        ("terrain_heightmap", 9),
        ("visual_terrain_heightmap", 0xa),
        ("path_mask", 5),
        ("garden_mask", 0x14),
        ("grass_object_push", 0xf),
    ] {
        p = bind_preset(p, name, preset);
    }
    // 磁盘纹理（O：`bind_texture_to_compute_pass<TextureFromDisk>`，外部描述符，读）排在 9 个 preset 之后、dispatch 之前。
    let p = p.bind_texture(HASH_BG_TERRAIN, &BG_TERRAIN_HEIGHTMAP, TEX_READ);
    let recorded = p.dispatch_indirect_raw(&args, 0);
    env.record(recorded);
}

fn bind_preset(pass: ComputePass, name: &str, preset: u8) -> ComputePass {
    pass.bind_texture(binding_hash(name), &TexturePreset(preset), TEX_READ)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::render::pass_builder::pass_resource_access::RgTexture;
    use crate::rhapsody_types::TextureDesc;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    const PRESET_NAMES: [&str; 9] = [
        "rocky_terrain",
        "water_mask",
        "buildings_mask",
        "foliage_exclusion_mask",
        "terrain_heightmap",
        "visual_terrain_heightmap",
        "path_mask",
        "garden_mask",
        "grass_object_push",
    ];

    fn world(theme: Option<u32>) -> TestWorld {
        let mut h = vec![
            HASH_TERRAIN,
            HASH_VISUAL_TERRAIN,
            HASH_HZB,
            HASH_TILE,
            HASH_BLADE_PREFIX,
            HASH_BLADE_SUBTOTAL,
            HASH_BUNDLE_PREFIX,
            HASH_BUNDLE_SUBTOTAL,
            HASH_BUNDLE,
            HASH_BUNDLE_INDEX,
            HASH_DISPATCH_ARGS,
            HASH_RANGES,
            HASH_BLADES,
            HASH_DRAW_COMMANDS,
            HASH_LOD1,
            HASH_ATOMICS,
            // `inclusive_prefix_scan_u32_1m` 的绑定名（O：`prefix_scan` 的立即数）。
            crate::systems::render_loop::compute_passes::prefix_scan::INOUT_BUF,
            crate::systems::render_loop::compute_passes::prefix_scan::INPUT_BUF,
            crate::systems::render_loop::compute_passes::prefix_scan::SUBTOTAL_SUM_BUF,
            crate::systems::render_loop::compute_passes::prefix_scan::SUBTOTAL_OUT_BUF,
        ];
        h.extend(PRESET_NAMES.iter().map(|n| binding_hash(n)));
        let w = TestWorld::new(Reflection::new().bind(&h).uniforms(&[
            KEY_HZB_DIMS,
            KEY_FREEZE,
            KEY_DRAW_OFFSET,
            KEY_LOD1_OFFSET,
            KEY_BUFFER_SIZE,
            KEY_BRUSH,
        ]))
        .with_ssbo::<GrassBladeDataSsbo>()
        .with_ssbo::<DrawIndexedIndirectBuffer>()
        .with_ssbo::<DrawIndirectBuffer>()
        .with_ssbo::<AtomicCounterBuffer>();
        match theme {
            Some(t) => w.with_resource(GladeThemeId(t)),
            None => w,
        }
    }
    fn hzb() -> Hzb {
        Hzb {
            half_extent: [960, 540],
            texture: RgTexture {
                desc: TextureDesc::new_2d([1024, 1024], 0x4c).mip_levels(11),
                resource: 9,
            },
        }
    }
    fn run(theme: Option<u32>, freeze: bool) -> (TestWorld, TestFrame) {
        let w = world(theme);
        let mut f = TestFrame::default();
        with_env(&w, &mut f, |env| grass_compute(env, &hzb(), freeze));
        (w, f)
    }

    #[test]
    fn brush_table_and_theme_mapping() {
        assert_eq!(BRUSH_TABLE, [1, 1, 2]);
        // 主题 7/8/9 → 表；其余（含 6、10、5、0）→ 0。
        assert_eq!(
            (0..=25).map(brush_index).collect::<Vec<_>>(),
            [
                0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn eight_grass_passes_and_the_prefix_scan_in_machine_code_order() {
        let (_, f) = run(Some(8), false);
        assert_eq!(
            f.order,
            [
                "Grass calc tiles",
                "Grass prefix tiles",
                "Grass tile bundle count",
                "Grass prefix bundles",
                "Grass tile bundle gen",
                // `inclusive_prefix_scan_u32_1m(bundle_grasslet_index_buffer)`（O：gen 与 dispatch args 之间）。
                "_Prefix 1",
                "_Prefix 2",
                "_Prefix merge",
                "Grass dispatch args",
                "Clear buffer",
                "Grass bundle ranges",
                "Grass"
            ]
        );
        let d = |i: usize| match f.passes[i].dispatch {
            DispatchMode::Threads { x, y, z } => format!("t{x},{y},{z}"),
            DispatchMode::Groups { x, y, z } => format!("g{x},{y},{z}"),
            DispatchMode::IndirectRaw { offset, .. } => format!("i{offset:#x}"),
            _ => "other".into(),
        };
        let got: Vec<String> = (0..11).map(d).collect();
        assert_eq!(
            got,
            [
                "t16384,1,1",
                "g1,1,1",
                "t16384,1,1",
                "g1,1,1",
                "t16384,1,1",
                // prefix scan：half = 0x80000/4 - 0x80000/8；blocks = 0x80000>>12 = 128 → 128 - 64。
                "t65536,1,1",
                "t64,1,1",
                "t65536,1,1",
                "t1,1,1",
                "i0x10",
                "i0x0"
            ]
        );
    }

    #[test]
    fn buffers_are_created_with_sizes_and_usages() {
        let (_, f) = run(Some(7), false);
        assert_eq!(f.temporal_buffers.len(), 1);
        assert_eq!(f.temporal_buffers[0].0, "grass_tile_buffer");
        assert_eq!(
            (f.temporal_buffers[0].1.size, f.temporal_buffers[0].1.usage),
            (0x10000, 0x20)
        );
        let created: Vec<(&str, usize, u32)> = f
            .buffers
            .iter()
            .map(|b| (b.0.as_str(), b.1.size, b.1.usage))
            .collect();
        assert_eq!(
            created,
            [
                ("blade_prefix_buffer", 0x10000, 0),
                ("blade_prefix_subtotal_buffer", 0x404, 0),
                ("tile_bundle_prefix_buffer", 0x40000, 0),
                ("tile_bundle_subtotal_buffer", 0x404, 0),
                ("bundle_buffer", 0x80000, 0),
                ("bundle_grasslet_index_buffer", 0x80000, 0),
                // `inclusive_prefix_scan_u32_1m` 的子和缓冲：max(128,1) 补到 1024 项 × 4 B。
                ("subtotal_sum_buf", 0x1000, 0),
                ("grass_dispatch_args_buffer", 0x20, 0),
                ("grasslet_bundle_ranges", 0x40000, 0)
            ]
        );
        // 只有 grasslet_bundle_ranges 被清零。
        let ranges = f.passes[9].builder.accesses[1].resource; // "Grass bundle ranges"：写 ranges
        assert_eq!(f.rg_passes, [("Clear buffer", ranges, 0x34)]);
    }

    #[test]
    fn prefix_scan_runs_on_the_bundle_index_buffer_between_gen_and_dispatch_args() {
        let (_, f) = run(Some(7), false);
        assert_eq!(BUNDLE_INDEX_BYTES, 0x80000);
        let names: Vec<_> = f.passes[5..8].iter().map(|p| p.name).collect();
        assert_eq!(names, ["_Prefix 1", "_Prefix 2", "_Prefix merge"]);
        let idx = f.passes[4].builder.accesses[2].resource;
        // gen 的写访问里第二个就是它；三段扫描都在它上面做；dispatch args 读它。
        assert_eq!(
            f.passes[5].builder.accesses[1].resource, idx,
            "pass 1 原地写"
        );
        assert_eq!(
            f.passes[6].builder.accesses[1].resource, idx,
            "pass 2 读输入"
        );
        assert_eq!(
            f.passes[7].builder.accesses[1].resource, idx,
            "merge 原地写"
        );
        assert_eq!(f.passes[8].builder.accesses[2].resource, idx);
        // 子和缓冲是扫描自己创建的 RG 缓冲，由 pass 2 写、merge 读。
        let sub = f
            .buffers
            .iter()
            .find(|b| b.0 == "subtotal_sum_buf")
            .unwrap()
            .2
            .resource;
        assert_eq!(f.passes[6].builder.accesses[2].resource, sub);
        assert_eq!(f.passes[7].builder.accesses[2].resource, sub);
    }

    #[test]
    fn calc_tiles_binds_hzb_and_forwards_the_debug_flag() {
        let (_, f) = run(Some(7), true);
        let p = &f.passes[0];
        // hzb 半尺寸 uniform @0（8 B），freeze 位 @16。
        assert_eq!((uniform_u32(p, 0), uniform_u32(p, 4)), (960, 540));
        assert_eq!(uniform_u32(p, 16), 1);
        let acc: Vec<(u8, u32)> = p
            .builder
            .accesses
            .iter()
            .map(|a| (a.access, a.group))
            .collect();
        assert_eq!(
            acc,
            [
                (0x1a, 1),
                (0x19, 1),
                (0x19, 1),
                (0x19, 1),
                (0x37, 0),
                (0x37, 0),
                (0x37, 0)
            ]
        );
        assert_eq!(p.builder.accesses[3].resource, 9, "hzb 纹理");
        let (_, f2) = run(Some(7), false);
        assert_eq!(uniform_u32(&f2.passes[0], 16), 0);
    }

    #[test]
    fn final_pass_uniforms_theme_brush_and_disk_texture() {
        let (_, f) = run(Some(9), false);
        let p = &f.passes[10];
        // 声明顺序：hzb_dims(0)、freeze(16)、draw_offset(32)、lod1_offset(48)、buffer_size(64)、brush(80)。
        assert_eq!(uniform_u32(p, 32), 0);
        assert_eq!(uniform_u32(p, 48), 0);
        assert_eq!(uniform_u32(p, 64), 0x90000);
        assert_eq!(uniform_u32(p, 80), 2, "主题 9 → 表[2]");
        // 磁盘纹理 `bg_terrain_heightmap` 不进 RG 访问表；反射里没有该绑定名时也不产生外部绑定（O：注册表未命中 → 只写维度）。
        assert!(p.external_bindings.is_empty());
        assert!(
            f.passes[..10]
                .iter()
                .all(|q| q.external_bindings.is_empty()),
            "只有最后一个 pass 绑磁盘纹理"
        );
        // 访问：color、读 tile/bundle/ranges、blades 写、draw 并发写(0x37)、lod1 写、atomics 写、9 个 preset 读、indirect(0x02)。
        let acc: Vec<(u8, u32)> = p
            .builder
            .accesses
            .iter()
            .map(|a| (a.access, a.group))
            .collect();
        let mut want = vec![
            (0x1a, 1),
            (0x1a, 1),
            (0x1a, 1),
            (0x1a, 1),
            (0x32, 0),
            (0x37, 1),
            (0x32, 0),
            (0x32, 0),
        ];
        want.extend([(0x19, 1); 9]);
        want.push((0x02, 1));
        assert_eq!(acc, want);
    }

    /// 反射里有 `bg_terrain_heightmap` 时，磁盘纹理走外部描述符路径：按 `NameHash(0x8991bdb39a048ac5)` 查库
    /// （`get_by_name`；库里没有该名字 → "No such texture"），而不是登记 RG 访问。
    #[test]
    #[should_panic(expected = "No such texture")]
    fn disk_texture_binding_resolves_through_the_texture_library_by_name_hash() {
        let mut h = vec![HASH_BG_TERRAIN];
        h.extend(PRESET_NAMES.iter().map(|n| binding_hash(n)));
        h.extend([
            HASH_TILE,
            HASH_BUNDLE,
            HASH_RANGES,
            HASH_DISPATCH_ARGS,
            HASH_BLADES,
        ]);
        h.extend([HASH_DRAW_COMMANDS, HASH_LOD1, HASH_ATOMICS]);
        let w = TestWorld::new(Reflection::new().bind(&h).uniforms(&[
            KEY_HZB_DIMS,
            KEY_FREEZE,
            KEY_DRAW_OFFSET,
            KEY_LOD1_OFFSET,
            KEY_BUFFER_SIZE,
            KEY_BRUSH,
        ]))
        .with_ssbo::<GrassBladeDataSsbo>()
        .with_ssbo::<DrawIndexedIndirectBuffer>()
        .with_ssbo::<DrawIndirectBuffer>()
        .with_ssbo::<AtomicCounterBuffer>()
        .with_resource(GladeThemeId(7));
        let mut f = TestFrame::default();
        with_env(&w, &mut f, |env| grass_compute(env, &hzb(), false));
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_theme_resource_unwraps_before_the_last_pass() {
        run(None, false);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn temporal_failure_unwraps_first() {
        let w = world(Some(7));
        let mut f = TestFrame {
            fail_temporal: true,
            ..Default::default()
        };
        with_env(&w, &mut f, |env| grass_compute(env, &hzb(), false));
    }

    #[test]
    fn object_grass_push_registers_as_a_plain_component() {
        use bevy_ecs::world::World;
        // O：单个 f32（Layout 4/4）。
        assert_eq!(core::mem::size_of::<ObjectGrassPush>(), 4);
        assert_eq!(core::mem::align_of::<ObjectGrassPush>(), 4);
        let mut world = World::new();
        let id = world.register_component::<ObjectGrassPush>();
        assert_eq!(world.register_component::<ObjectGrassPush>(), id);
        assert_eq!(
            world
                .components()
                .get_info(id)
                .unwrap()
                .name()
                .to_string()
                .contains("ObjectGrassPush"),
            true
        );
    }

    #[test]
    fn constants_match_machine_code() {
        assert_eq!(HASH_HZB, 0x62b6812f89a8a569);
        assert_eq!(KEY_HZB_DIMS, (-0x172dc9ec69fcbe2d_i64) as u64);
        assert_eq!(KEY_FREEZE, 0x6d7707561ec0da2b);
        assert_eq!(HASH_TILE, 0x67b935e4d4a7dd6c);
        assert_eq!(HASH_BLADE_PREFIX, (-0x739f8678e3b912c7_i64) as u64);
        assert_eq!(HASH_BLADE_SUBTOTAL, 0x3f3ee7377aa977ba);
        assert_eq!(HASH_BUNDLE_PREFIX, 0x3fc2c9e8ccc0b77e);
        assert_eq!(HASH_BUNDLE_SUBTOTAL, 0x73d3886611f6f8de);
        assert_eq!(HASH_BUNDLE, 0x49447d0bbff37d38);
        assert_eq!(HASH_BUNDLE_INDEX, 0x14b3d567dc6b16d9);
        assert_eq!(HASH_DISPATCH_ARGS, (-0x25450a54d24cf379_i64) as u64);
        assert_eq!(HASH_RANGES, 0x1a0da9d576285352);
        assert_eq!(HASH_BLADES, 0x9d0168095bc840f3);
        assert_eq!(HASH_LOD1, 0x6932ed479ccd8292);
        assert_eq!(KEY_LOD1_OFFSET, 0x3957088263abad4c);
        assert_eq!(HASH_ATOMICS, (-0x7c3238ce15bad6f7_i64) as u64);
        assert_eq!(KEY_BUFFER_SIZE, (-0x4f91b4380260e96c_i64) as u64);
        assert_eq!(KEY_BRUSH, (-0x2525370a5c62a5c0_i64) as u64);
        assert_eq!(HASH_BG_TERRAIN, 0x5b725035d0873550);
        assert_eq!(BG_TERRAIN_HEIGHTMAP.0, (-0x766e424c65fb753b_i64) as u64);
        assert_eq!(GladeThemeId::TYPE_ID.lo, (-0x3d4716be36e59bd8_i64) as u64);
    }
}
