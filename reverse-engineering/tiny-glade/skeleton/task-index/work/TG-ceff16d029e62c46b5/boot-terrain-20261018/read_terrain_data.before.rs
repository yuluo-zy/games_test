//! read_terrain_data.rs — W5 CPU 地形/水面/花园回读（O：`read_terrain_data` 0x14098c680，5917 B）。
//!
//! # 流程（O）
//! 0. 取预设纹理 10（`AssetTextureLibrary::get_rg_texture(.., 10)`）的 extent，建光栅 pass
//!    `"Visual terrain height"`（`RasterPass::new_impl`），在其中依次用两个 `DrawBuilder`
//!    （`"terrain_shader"` → `draw_floor_shader`、`"rocky_terrain"` → `rocky_terrain_shader`）绘制地形实体，
//!    然后 `RasterPass::render`。（光栅部分依赖 ECS 实体查询 + A4 draw_builder，这里只固化其**参数**，见
//!    [`VisualHeightRasterPlan`]，执行经 [`VisualHeightRaster`] 接缝。）
//! 1. `"Read terrain height"`（`/terrain_editing/read_terrain_height.cs.hlsl`）：`terrain_heightmap`（预设 9）、
//!    `visual_terrain_heightmap`（预设 10）读 0x19；写 `height_out`（`TerrainHeightSsbo`）、`visual_height_out`
//!    （`VisualTerrainHeightSsbo`）；`dispatch_1d(0x20000)`。
//! 2. `"Read terrain rock mask"`（`/terrain_editing/read_terrain_rock_mask.cs.hlsl`）：`rocky_terrain`（预设 0xe）读；
//!    写 `mask_out`（`TerrainRockMaskSsbo`）；`dispatch_1d(0x2000)`。
//! 3. `"Read water"`（`/water/read_water_mask.cs.hlsl`）**仅当 `_DEV_OVERRIDE_IS_DEMO_MODE`（`*0x143634d90`）为假**：
//!    `water_mask`（预设 0x13）读；写 `mask_out`（`WaterMaskSsbo`）、`mask_out_clutter`
//!    （`WaterExcludeClutterMaskSsbo`）；`dispatch_1d(0x2000)`。
//! 4. `"Read garden"`（`/garden/read_garden_mask.cs.hlsl`）：`garden_mask`（预设 0x14）读；写 `mask_out`
//!    （`GardenMaskSsbo`）；`dispatch_1d(0x2000)`。
//! 5. `ScheduledTerrainReadback`（type-id `0x24ccb35ae3644351…`，缺失 `Option::unwrap`）：
//!    `ScheduledTerrainReadback::on_gpu_copy_issued`（见 [`ScheduledTerrainReadback`]）。
//!
//! 所有 dispatch 描述符读自 `.rdata`（`0x142ab8410 = {1, 0x20000, 1, 1}`、`0x142ab8420 = {1, 0x2000, 1, 1}`），
//! 即 `Threads` 模式、一维。
//!
//! # 偏离
//! 六个 SSBO 载体在 `resources::*` 里是 typed gap（不满足 `LibrarySsbo`），经 [`ReadbackSsbos`] 关联类型注入。

use std::sync::atomic::{AtomicU64, Ordering};

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::resources::texture_presets::TexturePreset;
use crate::resources::water_mask::{CLUTTER_BIT_THRESHOLD, WATER_BIT_THRESHOLD};

use super::pass_kit::{PassEnv, binding_hash};

// ---------------------------------------------------------------- 绑定名 / 预设槽 / 维度

pub const H_TERRAIN_HEIGHTMAP: u64 = binding_hash("terrain_heightmap");
pub const H_VISUAL_TERRAIN_HEIGHTMAP: u64 = binding_hash("visual_terrain_heightmap");
pub const H_HEIGHT_OUT: u64 = binding_hash("height_out");
pub const H_VISUAL_HEIGHT_OUT: u64 = binding_hash("visual_height_out");
pub const H_ROCKY_TERRAIN: u64 = binding_hash("rocky_terrain");
pub const H_MASK_OUT: u64 = binding_hash("mask_out");
pub const H_WATER_MASK: u64 = binding_hash("water_mask");
pub const H_MASK_OUT_CLUTTER: u64 = binding_hash("mask_out_clutter");
pub const H_GARDEN_MASK: u64 = binding_hash("garden_mask");

/// 预设纹理槽（O：`mov byte ptr [..], N`，随后 `bind_texture_to_compute_pass<TexturePreset>`）。
pub const PRESET_TERRAIN_HEIGHT: TexturePreset = TexturePreset(9);
pub const PRESET_VISUAL_TERRAIN_HEIGHT: TexturePreset = TexturePreset(10);
pub const PRESET_ROCKY_TERRAIN: TexturePreset = TexturePreset(0xe);
pub const PRESET_WATER_MASK: TexturePreset = TexturePreset(0x13);
pub const PRESET_GARDEN_MASK: TexturePreset = TexturePreset(0x14);

pub const ACCESS_READ: u8 = 0x19;
/// 高度读回线程数（O：`.rdata 0x142ab8410` 的 x 分量）。
pub const HEIGHT_READBACK_THREADS: u32 = 0x20000;
/// 其余掩码读回线程数（O：`.rdata 0x142ab8420`）；水掩码 512×512 / 32 texel 每线程。
pub const MASK_READBACK_THREADS: u32 = 0x2000;
/// `_DEV_OVERRIDE_IS_DEMO_MODE` 的静态地址（O：`cmp byte ptr [0x143634d90], 0`）。
pub const DEMO_MODE_FLAG_ADDR: u64 = 0x1_4363_4d90;

/// 水回读打包位判定（O：`read_water_mask.cs.hlsl`：`water_mask[coord].y > 0.5 → water bit`；`> 0.2 → clutter bit`；
/// 每线程 32 texel，一行 512 宽）。
pub fn pack_water_mask_bits(coverage_y: f32) -> (bool, bool) {
    (
        coverage_y > WATER_BIT_THRESHOLD,
        coverage_y > CLUTTER_BIT_THRESHOLD,
    )
}

/// 把 32 个 texel 的 `y` 值打成两个 u32（HLSL 的 `i & 31` 位序）。
pub fn pack_water_mask_word(texels_y: &[f32; 32]) -> (u32, u32) {
    let (mut water, mut clutter) = (0u32, 0u32);
    for (i, y) in texels_y.iter().enumerate() {
        let (w, c) = pack_water_mask_bits(*y);
        water |= u32::from(w) << i;
        clutter |= u32::from(c) << i;
    }
    (water, clutter)
}

// ---------------------------------------------------------------- 光栅预渲染参数（O，执行经接缝）

/// 光栅 pass `"Visual terrain height"` 的固化参数（O）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VisualHeightRasterPlan {
    pub pass_name: &'static str,
    /// 深度附件预设（O：`get_rg_texture(.., 10)`；pass extent = 该纹理 `extent_2d`）。
    pub depth_preset: TexturePreset,
    /// 依次绘制的 `(DrawBuilder 名, 着色器函数)`（O：`DrawBuilder::new` + `bind_common` + draw_*）。
    pub draws: [(&'static str, &'static str); 2],
}

pub const VISUAL_HEIGHT_RASTER: VisualHeightRasterPlan = VisualHeightRasterPlan {
    pass_name: "Visual terrain height",
    depth_preset: PRESET_VISUAL_TERRAIN_HEIGHT,
    draws: [
        ("terrain_shader", "draw_floor_shader::draw_floor_shader"),
        ("rocky_terrain", "rocky_terrain_draw::rocky_terrain_shader"),
    ],
};

/// 光栅预渲染接缝（`RasterPass::new_impl` / `DrawBuilder::new` / `RasterPass::render`，A4 draw 域）。
pub trait VisualHeightRaster {
    fn render_visual_height(&mut self, plan: &VisualHeightRasterPlan);
}

// ---------------------------------------------------------------- ScheduledTerrainReadback

/// `ScheduledTerrainReadback`（O：type-id `0xfc8691076ab4d826`/`0x24ccb35ae3644351` 一族；
/// `on_gpu_copy_issued` = 对 `[+0]` 做 `cmpxchg 0` 取走请求，旧值非 0 则 `[+8] |= 1`）。
#[derive(bevy_ecs::resource::Resource, Debug, Default)]
pub struct ScheduledTerrainReadback {
    /// `[+0]`：待处理的读回请求（取走式）。
    pub pending: AtomicU64,
    /// `[+8]`：本帧已发出 GPU 拷贝。
    pub copy_issued: bool,
    /// 组件变更 tick（O：`ecx = [world+0x5b0]` 写入 `[row+0x5c]`）。
    pub changed_tick: u32,
}

impl ScheduledTerrainReadback {
    /// B 0x140aabdc0, LOCK INC; shared by draw and edit consumers.
    pub fn on_terrain_modified(&self) {
        self.pending.fetch_add(1, Ordering::SeqCst);
    }
    /// O（0x14098d88f..0x14098d8ac）：先标记变更 tick，再 `swap(0)`，旧值非 0 → `copy_issued = true`（只或不清）。
    pub fn on_gpu_copy_issued(&mut self, world_tick: u32) {
        self.changed_tick = world_tick;
        let old = self.pending.swap(0, Ordering::SeqCst);
        self.copy_issued |= old != 0;
    }
}

// ---------------------------------------------------------------- 回读 pass

/// 六个回读 SSBO（O：`write_storage_buffer<T>` 的 T）。
pub trait ReadbackSsbos {
    type Height: LibrarySsbo;
    type VisualHeight: LibrarySsbo;
    type RockMask: LibrarySsbo;
    type WaterMask: LibrarySsbo;
    type WaterClutter: LibrarySsbo;
    type Garden: LibrarySsbo;
}

/// W5 — `read_terrain_data`（O 0x14098c680）：返回按序声明的 compute pass（光栅部分已经 `raster` 接缝执行）。
/// `demo_mode` = `*0x143634d90`；`world_tick` = `[world+0x5b0]`。
pub fn read_terrain_data<S: ReadbackSsbos>(
    env: &mut PassEnv<'_>,
    raster: &mut dyn VisualHeightRaster,
    readback: &mut ScheduledTerrainReadback,
    demo_mode: bool,
    world_tick: u32,
) -> Vec<RecordedComputePass> {
    raster.render_visual_height(&VISUAL_HEIGHT_RASTER);

    let mut passes = vec![
        read_terrain_height::<S>(env),
        read_terrain_rock_mask::<S>(env),
    ];
    if !demo_mode {
        passes.push(read_water_mask::<S>(env));
    }
    passes.push(read_garden_mask::<S>(env));

    readback.on_gpu_copy_issued(world_tick);
    passes
}

/// `"Read terrain height"`。
pub fn read_terrain_height<S: ReadbackSsbos>(env: &mut PassEnv<'_>) -> RecordedComputePass {
    env.new_pass(
        "Read terrain height",
        &"/terrain_editing/read_terrain_height.cs.hlsl",
    )
    .bind_texture(H_TERRAIN_HEIGHTMAP, &PRESET_TERRAIN_HEIGHT, ACCESS_READ)
    .bind_texture(
        H_VISUAL_TERRAIN_HEIGHTMAP,
        &PRESET_VISUAL_TERRAIN_HEIGHT,
        ACCESS_READ,
    )
    .write_storage_buffer::<S::Height>(H_HEIGHT_OUT)
    .write_storage_buffer::<S::VisualHeight>(H_VISUAL_HEIGHT_OUT)
    .dispatch_1d(HEIGHT_READBACK_THREADS)
}

/// `"Read terrain rock mask"`。
pub fn read_terrain_rock_mask<S: ReadbackSsbos>(env: &mut PassEnv<'_>) -> RecordedComputePass {
    env.new_pass(
        "Read terrain rock mask",
        &"/terrain_editing/read_terrain_rock_mask.cs.hlsl",
    )
    .bind_texture(H_ROCKY_TERRAIN, &PRESET_ROCKY_TERRAIN, ACCESS_READ)
    .write_storage_buffer::<S::RockMask>(H_MASK_OUT)
    .dispatch_1d(MASK_READBACK_THREADS)
}

/// `"Read water"`（一次回读同时写水 / 排除杂物两个缓冲）。
pub fn read_water_mask<S: ReadbackSsbos>(env: &mut PassEnv<'_>) -> RecordedComputePass {
    env.new_pass("Read water", &"/water/read_water_mask.cs.hlsl")
        .bind_texture(H_WATER_MASK, &PRESET_WATER_MASK, ACCESS_READ)
        .write_storage_buffer::<S::WaterMask>(H_MASK_OUT)
        .write_storage_buffer::<S::WaterClutter>(H_MASK_OUT_CLUTTER)
        .dispatch_1d(MASK_READBACK_THREADS)
}

/// `"Read garden"`。
pub fn read_garden_mask<S: ReadbackSsbos>(env: &mut PassEnv<'_>) -> RecordedComputePass {
    env.new_pass("Read garden", &"/garden/read_garden_mask.cs.hlsl")
        .bind_texture(H_GARDEN_MASK, &PRESET_GARDEN_MASK, ACCESS_READ)
        .write_storage_buffer::<S::Garden>(H_MASK_OUT)
        .dispatch_1d(MASK_READBACK_THREADS)
}

/// 生产回读 SSBO 关联类型（卡7 闭合：mask 四类资源 GAP 已建
/// `terrain_rock_mask`/`water_exclude_clutter_mask`/`garden_mask`/`water_mask` 的
/// `SsboType` 实现）。`read_terrain_data::<TerrainReadbackSsbos>` 供宿主调用。
pub struct TerrainReadbackSsbos;

impl ReadbackSsbos for TerrainReadbackSsbos {
    type Height = crate::resources::terrain_height::TerrainHeightSsbo;
    type VisualHeight = crate::resources::terrain_height::VisualTerrainHeightSsbo;
    type RockMask = crate::resources::terrain_rock_mask::TerrainRockMaskSsbo;
    type WaterMask = crate::resources::water_mask::WaterMaskSsbo;
    type WaterClutter = crate::resources::water_exclude_clutter_mask::WaterExcludeClutterMaskSsbo;
    type Garden = crate::resources::garden_mask::GardenMaskSsbo;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::systems::render_loop::compute_passes::pass_kit::imm;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    struct Ssbos;
    impl ReadbackSsbos for Ssbos {
        type Height = SsboA;
        type VisualHeight = SsboB;
        type RockMask = SsboC;
        type WaterMask = W;
        type WaterClutter = X;
        type Garden = Y;
    }
    // 另三个载体（test_env 只给了 3 个）。
    macro_rules! more {
        ($($n:ident),*) => {$(
            #[derive(Clone, Copy, Debug, Default)]
            struct $n;
            impl crate::render::ssbo::StorageElement for $n {
                const NAME: &'static str = stringify!($n);
                const BUFFER_LEN: usize = 1;
                fn default_value() -> Option<Self> { Some($n) }
            }
            impl LibrarySsbo for $n {
                type Storage = crate::render::ssbo::TypedStorageBuffer<$n>;
                fn new_storage_buffer(
                    device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
                    initial: Option<&[u8]>,
                ) -> Self::Storage {
                    match initial {
                        None => crate::render::ssbo::TypedStorageBuffer::<$n>::new(device, None),
                        Some(b) => crate::render::ssbo::TypedStorageBuffer::<$n>::new_storage_buffer(device, Some(b)),
                    }
                }
            }
        )*};
    }
    more!(W, X, Y);

    #[derive(Default)]
    struct Rec(Vec<&'static str>);
    impl VisualHeightRaster for Rec {
        fn render_visual_height(&mut self, plan: &VisualHeightRasterPlan) {
            self.0.push(plan.pass_name);
        }
    }

    fn make_env() -> Env {
        env_with(
            &[
                H_TERRAIN_HEIGHTMAP,
                H_VISUAL_TERRAIN_HEIGHTMAP,
                H_HEIGHT_OUT,
                H_VISUAL_HEIGHT_OUT,
                H_ROCKY_TERRAIN,
                H_MASK_OUT,
                H_WATER_MASK,
                H_MASK_OUT_CLUTTER,
                H_GARDEN_MASK,
            ],
            &[],
            |lib, dev| {
                lib.add_new::<SsboA>(dev);
                lib.add_new::<SsboB>(dev);
                lib.add_new::<SsboC>(dev);
                lib.add_new::<W>(dev);
                lib.add_new::<X>(dev);
                lib.add_new::<Y>(dev);
            },
        )
    }

    #[test]
    fn hashes_are_exe_immediates() {
        assert_eq!(H_TERRAIN_HEIGHTMAP, 0x61b08b5e0470c5f2);
        assert_eq!(H_VISUAL_TERRAIN_HEIGHTMAP, 0x297d1e4cdae8f657);
        assert_eq!(H_HEIGHT_OUT, imm(-0x4da8d638b0785ba7));
        assert_eq!(H_VISUAL_HEIGHT_OUT, 0x7703bd797f8b276e);
        assert_eq!(H_ROCKY_TERRAIN, 0x4ee098b38612d2b9);
        assert_eq!(H_MASK_OUT, 0x4ee7d41b19c88236);
        assert_eq!(H_WATER_MASK, 0x6ad63ccd18c8f041);
        assert_eq!(H_MASK_OUT_CLUTTER, 0x7864ea0bf1becbd8);
        assert_eq!(H_GARDEN_MASK, 0x49f43a487883738d);
    }

    #[test]
    fn full_readback_declares_four_passes_after_the_raster() {
        let mut e = make_env();
        let mut raster = Rec::default();
        let mut rb = ScheduledTerrainReadback::default();
        rb.pending.store(7, Ordering::SeqCst);
        let passes =
            e.run(|penv| read_terrain_data::<Ssbos>(penv, &mut raster, &mut rb, false, 42));
        assert_eq!(raster.0, ["Visual terrain height"]);
        let names: Vec<_> = passes.iter().map(|p| p.name).collect();
        assert_eq!(
            names,
            [
                "Read terrain height",
                "Read terrain rock mask",
                "Read water",
                "Read garden"
            ]
        );
        let uris = e.world.backend.paths.borrow().clone();
        assert_eq!(
            uris,
            [
                "/terrain_editing/read_terrain_height.cs.hlsl:cs_main",
                "/terrain_editing/read_terrain_rock_mask.cs.hlsl:cs_main",
                "/water/read_water_mask.cs.hlsl:cs_main",
                "/garden/read_garden_mask.cs.hlsl:cs_main"
            ]
        );
        assert!(matches!(
            passes[0].dispatch,
            DispatchMode::Threads {
                x: 0x20000,
                y: 1,
                z: 1
            }
        ));
        for p in &passes[1..] {
            assert!(matches!(
                p.dispatch,
                DispatchMode::Threads {
                    x: 0x2000,
                    y: 1,
                    z: 1
                }
            ));
        }
        // 水回读：1 个纹理 + 2 个缓冲（含 ctor 的 color library）。
        assert_eq!(passes[2].bindings.len(), 4);
        // 回读登记。
        assert!(rb.copy_issued);
        assert_eq!(rb.changed_tick, 42);
        assert_eq!(rb.pending.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn demo_mode_skips_only_the_water_pass() {
        let mut e = make_env();
        let mut rb = ScheduledTerrainReadback::default();
        let passes =
            e.run(|penv| read_terrain_data::<Ssbos>(penv, &mut Rec::default(), &mut rb, true, 1));
        let names: Vec<_> = passes.iter().map(|p| p.name).collect();
        assert_eq!(
            names,
            [
                "Read terrain height",
                "Read terrain rock mask",
                "Read garden"
            ]
        );
        // 没有待处理请求：不置位。
        assert!(!rb.copy_issued);
    }

    #[test]
    fn on_gpu_copy_issued_only_ors_never_clears() {
        let mut rb = ScheduledTerrainReadback {
            copy_issued: true,
            ..Default::default()
        };
        rb.on_gpu_copy_issued(5);
        assert!(rb.copy_issued, "|= 语义：旧值为 0 时不清除");
    }

    #[test]
    fn water_bits_follow_the_shader_thresholds() {
        assert_eq!(pack_water_mask_bits(0.5), (false, true));
        assert_eq!(pack_water_mask_bits(0.51), (true, true));
        assert_eq!(pack_water_mask_bits(0.2), (false, false));
        let mut t = [0.0f32; 32];
        t[0] = 0.9;
        t[31] = 0.3;
        assert_eq!(pack_water_mask_word(&t), (1, 0x8000_0001));
        // 512 宽 / 32 texel 每线程 * 0x2000 线程 = 512*512。
        assert_eq!(MASK_READBACK_THREADS * 32, 512 * 512);
    }

    #[test]
    fn raster_plan_constants() {
        assert_eq!(VISUAL_HEIGHT_RASTER.pass_name.len(), 0x15);
        assert_eq!(VISUAL_HEIGHT_RASTER.depth_preset, TexturePreset(10));
        assert_eq!(VISUAL_HEIGHT_RASTER.draws[0].0.len(), 0xe);
        assert_eq!(VISUAL_HEIGHT_RASTER.draws[1].0.len(), 0xd);
    }
}
