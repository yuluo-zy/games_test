//! Original B W5 @14098c680/5917B: actual floor/rocky entities first render
//! visual depth, then height/rock/water(non-demo)/garden mapped SSBO passes.
//! The same ScheduledTerrainReadback tail is committed by the typed host only
//! after graph admission succeeds, not before its recorded passes are admitted.

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

/// Original direct raster body. Items are snapshots of the two actual
/// TerrainEntities and their typed Vao/Shader/Transform/optional topology.
/// No shaders are selected by a guessed query, and no fake geometry is created.
pub fn render_visual_height(
    ctx: &mut crate::systems::render_loop::draw_passes::d_main_support::DrawPassCtx<'_>,
    floor: &crate::render::pass_builder::draw_list::DrawListItem,
    rocky: &crate::render::pass_builder::draw_list::DrawListItem,
    vertex_count: u32,
) {
    use crate::{
        asset_libraries::color_library::ColorLibrarySsbo,
        render::pass_builder::draw_builder::{CommonDrawFlag, CommonDrawParams, DrawBuilder},
        systems::render_loop::draw_passes::{
            d_main_support::DepthSpec,
            draw_shaders::{floor_shader, rocky_terrain_draw},
        },
    };
    let depth = ctx.host().rg_texture(10);
    // Native depth descriptor @14098c74b..75d: tag0, preset10, Some(0).
    let mut pass = ctx.raster_pass(
        "Visual terrain height",
        &[],
        DepthSpec::Write {
            tex: depth,
            layer: 0,
            clear: Some(0.0),
        },
        depth,
    );
    // Both native bind_common calls use static142ab9238 = {time0,flag0}.
    // This pass must not reuse the ordinary scene's elapsed-time parameters.
    let params = CommonDrawParams {
        time: 0.0,
        flag: CommonDrawFlag::Off,
    };
    let state = pass.host.default_raster_state();
    let floor_builder = DrawBuilder::new(&mut pass, floor.to_draw_packet(state), 1)
        .bind_common::<ColorLibrarySsbo>(&params, &floor.params.transform);
    floor_shader::draw_floor_shader(floor_builder, true);
    let state = pass.host.default_raster_state();
    let rocky_builder = DrawBuilder::new(&mut pass, rocky.to_draw_packet(state), 1)
        .bind_common::<ColorLibrarySsbo>(&params, &rocky.params.transform);
    rocky_terrain_draw::rocky_terrain_shader(rocky_builder, true, vertex_count);
    pass.render();
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

impl Clone for ScheduledTerrainReadback {
    fn clone(&self) -> Self {
        Self {
            pending: AtomicU64::new(self.pending.load(core::sync::atomic::Ordering::Acquire)),
            copy_issued: self.copy_issued,
            changed_tick: self.changed_tick,
        }
    }
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

/// Compute portion after render_visual_height. The actual same-world host
/// records these passes, waits for declaration/admission success, then calls
/// ScheduledTerrainReadback::on_gpu_copy_issued with the actual Bevy tick.
/// Delayed CPU copies remain driven by that existing canonical owner.
pub fn read_terrain_data<S: ReadbackSsbos>(
    env: &mut PassEnv<'_>,
    demo_mode: bool,
) -> Vec<RecordedComputePass> {
    let mut passes = vec![
        read_terrain_height::<S>(env),
        read_terrain_rock_mask::<S>(env),
    ];
    if !demo_mode {
        passes.push(read_water_mask::<S>(env));
    }
    passes.push(read_garden_mask::<S>(env));
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
    fn non_demo_compute_tail_declares_four_passes_in_original_order() {
        let mut e = make_env();
        let passes = e.run(|penv| read_terrain_data::<Ssbos>(penv, false));
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
    }

    #[test]
    fn demo_mode_skips_only_the_water_pass() {
        let mut e = make_env();
        let passes = e.run(|penv| read_terrain_data::<Ssbos>(penv, true));
        let names: Vec<_> = passes.iter().map(|p| p.name).collect();
        assert_eq!(
            names,
            [
                "Read terrain height",
                "Read terrain rock mask",
                "Read garden"
            ]
        );
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
    fn visual_height_records_clear_and_both_real_callbacks_with_static_common_params() {
        use crate::{
            asset_libraries::color_library::ColorLibrarySsbo,
            render::pass_builder::{
                bind_resources::WallColorIdSsbo,
                draw_builder::{STONE_FLOOR_TEXTURES, WALL_COLOR_TEXTURES},
                draw_list::DrawListItem,
                draw_stream::{
                    DrawKind, ShaderReflection,
                    test_host::{Log, MockHost},
                },
                framebuffer_binding::{AttachmentTexture, DepthClear},
                pass_resource_access::SsboType,
            },
            systems::render_loop::draw_passes::{
                d_main_support::{DrawPassCtx, NoShaders},
                draw_shaders::{
                    floor_shader, rocky_terrain_draw,
                    ssbo_tags::{RockyTerrainTriangleTag, RockyTerrainVertexTag},
                },
            },
        };
        use std::{cell::RefCell, rc::Rc, sync::Arc, time::Duration};
        let log = Rc::new(RefCell::new(Log::default()));
        let mut factory = || {
            let (mut host, _) = MockHost::new();
            host.log = log.clone();
            for id in [11, 22] {
                let mut reflection = ShaderReflection::default();
                reflection.program_name = "w5_actual_callbacks";
                reflection.has_uniform_block = true;
                reflection.uniform_block_size = 16;
                reflection.uniform_block_binding = 7;
                for (name, offset) in [
                    (0x418a98441f59a54c, 0), // common flag == Off
                    (0xc5ab673fb08651f5, 4), // original flag byte
                    (0x1e5485ef2e9f9384, 8), // static time, not ctx elapsed
                    (floor_shader::FLOOR_FLAG_UNIFORM, 12),
                ] {
                    reflection.uniform_offsets.insert(name, offset);
                }
                host.shaders.insert(id, Arc::new(reflection));
            }
            for (index, hash) in [
                ColorLibrarySsbo::rg_buffer_hash(),
                WallColorIdSsbo::rg_buffer_hash(),
                RockyTerrainVertexTag::rg_buffer_hash(),
                RockyTerrainTriangleTag::rg_buffer_hash(),
            ]
            .into_iter()
            .enumerate()
            {
                host.rg_buffers.insert(hash, index as u32 + 1);
            }
            for preset in 0..=0x15 {
                host.presets.insert(
                    preset,
                    AttachmentTexture {
                        id: 100 + preset as u32,
                        extent: (512, 512),
                        format: 126,
                        sample_count: 1,
                    },
                );
            }
            let disks = STONE_FLOOR_TEXTURES
                .iter()
                .chain(WALL_COLOR_TEXTURES.iter())
                .chain(floor_shader::FLOOR_DISK_MID.iter())
                .copied()
                .chain([
                    floor_shader::FLOOR_DISK_HEAD,
                    floor_shader::FLOOR_DISK_TAIL,
                    rocky_terrain_draw::ROCKY_DISK_TEXTURE,
                ]);
            for (key, _) in disks {
                host.textures_by_name.insert(
                    key,
                    (
                        crate::rhapsody_contract::ManagedHandle(key),
                        AttachmentTexture {
                            id: 500,
                            extent: (8, 8),
                            format: 37,
                            sample_count: 1,
                        },
                    ),
                );
            }
            host.boxed()
        };
        let mut world = bevy_ecs::world::World::new();
        let mut ctx = DrawPassCtx::new(
            &mut world,
            Duration::from_secs(73),
            &mut factory,
            &NoShaders,
        );
        let mut floor = DrawListItem::new(11, 0);
        floor.vao = 101;
        let mut rocky = DrawListItem::new(22, 0);
        rocky.vao = 202;
        render_visual_height(&mut ctx, &floor, &rocky, 123);
        let log = log.borrow();
        assert_eq!(log.submissions.len(), 1);
        let pass = &log.submissions[0];
        assert_eq!(pass.name, "Visual terrain height");
        assert_eq!(pass.color_count, 0);
        assert_eq!(pass.viewport, [512, 512]);
        let depth = pass.depth.unwrap();
        assert_eq!(depth.texture_id, 110);
        assert_eq!(depth.layer, 0);
        assert!(depth.writable);
        assert_eq!(depth.clear, DepthClear::Clear(0.0));
        assert_eq!(pass.draws.len(), 2);
        assert_eq!(pass.draws[0].shader.0.0, 11);
        assert_eq!(pass.draws[1].shader.0.0, 22);
        assert_eq!(pass.draws[0].head, [1, 101]);
        assert_eq!(pass.draws[1].head, [1, 202]);
        assert_eq!(
            pass.draws.iter().map(|d| d.variant).collect::<Vec<_>>(),
            [1, 1]
        );
        assert_eq!(
            pass.draws[0].kind,
            DrawKind::InstancedGlDrawFlag { flag: 1 }
        );
        assert_eq!(
            pass.draws[1].kind,
            DrawKind::NoIndexBuffer { vertex_count: 123 }
        );
        assert_eq!(log.flushed_constants.len(), 2);
        for (_, bytes) in &log.flushed_constants {
            let words: Vec<_> = bytes
                .chunks_exact(4)
                .map(|v| u32::from_le_bytes(v.try_into().unwrap()))
                .collect();
            assert_eq!(words, [1, 0, 0, 1]);
        }
    }
}
