//! `deferred_raster.rs` — W43 延迟光照（`deferred_raster` 0x1409d20b0，6203 B）。**完整恢复**（B_d_gap：资源句柄接线）。
//!
//! # 参数（O：Win64 ABI；栈参数在 `rbp + 0xdf0 + 8*i`，`rbp = rsp + 0x80`，`sub rsp, 0xe08`）
//! | 位置 | 内容 |
//! |---|---|
//! | `rcx` | `&mut RenderGraph`（传给 `RasterPass::new_impl` 的 `[rsp+0x40]`）|
//! | `rdx` | `&World`（`+0x38` = `&Components`）|
//! | `r8` = arg3 | `&[Handle<Texture>; 2]`（0x24 B 一个）：`[0]` = 视口来源 + 绑定 `0x41aa…`，`[1]`（`+0x24`）= **只读深度附件**（tag1）+ 绑定 `0x4861…` |
//! | `r9` = arg4 | `&Handle<Texture>` 场景颜色附件（tag0 LOAD）；其 `extent_2d` → `[u32;2]` uniform |
//! | `[0xdf0]` = arg5 | `&Handle<Texture>` → `0x0417…` |
//! | `[0xdf8]` = arg6 | `u8` → `write_uniform<u32>(0x5788…)` |
//! | `[0xe00]` = arg7 | `&Handle<Texture>` → `0xe003…` |
//! | `[0xe08]` = arg8 | `&Handle<Texture>` → `0x2a33…` |
//! | `[0xe10]` = arg9 | `&[Handle<Texture>; 3]`：`[0]` → `0xfe8d…`、`[1]`（`+0x24`）→ `0x0d36…`、`[2]`（`+0x48`）→ `0x4b1b…` |
//! | `[0xe18]` = arg10 | `&Handle<Texture>` → `0xba3e…` |
//! | `[0xe20]` = arg11 | `&Handle<Texture>` → `0x9941…` |
//! | `[0xe28]` = arg12 | `&Handle<Texture>` → `0x5a67…` |
//! | `[0xe30]` = arg13 | `&[Handle<Buffer>; 2]`（0x18 B 一个）：`[0]` → `read_rg_storage_buffer(0x2bd0…)`，`[1]`（`+0x18`）→ `(0x129f…)` |
//!
//! # 资源（`Components::get_resource_id` 的 TypeId，O）
//! - `0x78cac557b9afa382/0x4199801d7a83a055` → `[Option<Handle<RasterShaderProgram>>; 5]`（按条目的 `material` 下标，`None` → `Option::unwrap` panic）；
//! - `0x4bc362195b80f729/-0x67d867317f343105` = `Res<Time>`：`secs as f32 + nanos / 1e9`（即 `Duration::as_secs_f32`）→ `optional_uniform<f32>(0x1e54…)`；
//! - `-0x7dd9338791986cab/0x65cfda6a55a841ac` = `AssetSsboLibrary`：`get::<LanternSsbo>`（失败 `expect`：
//!   `country_core::resources::render::lantern…`）→ 缓冲首字低 32 位 → `write_uniform<u32>(0x9b88…)`。
//!
//! # 绘制（O）
//! `RasterPass::new_impl("Apply deferred lighting"(0x17), [tag0 = arg4], 深度 = arg3[1] (tag1 只读), 视口 = arg3[1])`；然后 5 个条目
//! （[`ENTRIES`]，下标/标志取自 `[rbp+0xcc8..0xcf4]`）。每个条目一次 `DrawPacket`（头 `[0, _]`、`blend = 1.0`、`DrawKind` 默认）+
//! `DrawBuilder::new(variant 0)`：
//! - 默认状态 `s = shader::default()`；`flag == false`：`byte0 = s & 0xbf`，`byte3` 不变，`packet.+0xb1`（模板参考值）= `material`；
//!   `flag == true`：同 byte0，`byte3 = (s >> 24 & 0x9f) | 0x20`，模板参考值 = `material | 0x80`；
//! - 绑定序列见 [`bind_all`]（逐项 = 机器码调用序，hash/来源均由脚本抽取）；
//! - 收尾状态改写（`0x1409d2e4c`）：`byte0 &= 0x7f`、`byte1 |= 1`、`byte2 = (byte2 & 3) | 0xe8`、`byte3 = (byte3 & 0xe0) | 0x10`；
//! - `draw_no_vao(3)`。循环结束后 `render`。
//!
//! 对 A4 的适配（I）：`bind_ray_tracing<DrawBuilder>`（0x1416289d0）用 A4 的 `DrawBuilder::bind_ray_tracing::<Tri, Meta, Bvh, Wall>`
//! （本函数对四个 `SsboType` 载体泛型）；Lantern 的 uniform 值 = `PassHost::ssbo_instance_count(LanternSsbo)`（实际实例数量）；
//! `LanternSsbo` 在 crate 里是 typed gap（无 `SsboType`），这里用本文件的 [`LanternBuffer`] / [`SpatialResolveOffsetsBuffer`]
//! 载体（`rg_buffer_hash` = `get_rg_buffer<T>` 的立即数）。

use crate::asset_libraries::color_library::ColorLibrarySsbo;
use crate::render::pass_builder::draw_builder::{ACCESS_SAMPLED_IMAGE, BindTexture, DrawBuilder};
use crate::render::pass_builder::draw_stream::DrawPacket;
use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::render::shader::RasterShaderProgram;
use crate::resources::texture_presets::{TextureFromDisk, TexturePreset};
use crate::rhapsody_contract::Handle;
use crate::systems::render_loop::draw_passes::d_main_support::{
    ColorSpec, DepthSpec, DrawPassCtx, UNWRAP_NONE,
};
use crate::systems::render_loop::draw_passes::material_id::MaterialId;

/// Real per-App material shader owner (PDB DeferredRasterResources).
/// B insert_rendering_resources 140940afa..b50 initializes all five Options
/// to None; startup_deferred_raster fills only material slots 1,3,0,2.
#[derive(bevy_ecs::resource::Resource, Default)]
pub struct DeferredRasterResources {
    pub shaders: [Option<crate::asset_libraries::Handle<RasterShaderProgram>>; 5],
}

pub const PASS_NAME: &str = "Apply deferred lighting";

/// 材质 → 着色器 URI（O：字符串池 0x142c2e230 附近 / `sequence.md`）。
pub fn shader_uri(m: MaterialId) -> Option<&'static str> {
    match m {
        MaterialId::Solid => Some("/deferred/solid_light_sun_sky.raster.hlsl"),
        MaterialId::Grass => Some("/deferred/grass_light_sun_sky.raster.hlsl"),
        MaterialId::Plant => Some("/deferred/plant_light_sun_sky.raster.hlsl"),
        MaterialId::Canopy => Some("/deferred/canopy_light_sun_sky.raster.hlsl"),
        MaterialId::Forward | MaterialId::COUNT => None,
    }
}

/// 一次完整的延迟光照由哪些材质组成。
pub fn materials() -> &'static [MaterialId] {
    &MaterialId::DEFERRED
}

/// `get_rg_buffer<SpatialResolveOffsetsSsbo>`（0x1412d57a0）的 `SsboId` 立即数。
pub use crate::resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo as SpatialResolveOffsetsBuffer;
/// `get_rg_buffer<LanternSsbo>`（0x1412d5470）的 `SsboId` 立即数（`-0x74d25e10e40069e7`）。
pub use crate::resources::render_ssbo::LanternSsbo as LanternBuffer;

// ---- 绑定名哈希（O：`movabs r8`；负立即数已转无符号）----
pub const B_LIGHTING_0: u64 = 0x41aa_4a0d_2dea_0b36;
pub const B_LIGHTING_1: u64 = 0x4861_579d_a049_fbfc;
pub const B_ARG8: u64 = 0x2a33_3c3a_2802_8230;
pub const B_ARG5: u64 = 0x0417_5185_826c_09d0;
/// `TextureFromDisk(0x579f39ab91a01865)` 的绑定名（`-0x7ea40afc96996874`）。
pub const B_DISK: u64 = 0x815b_f503_6966_978c;
pub const DISK_FILE: u64 = 0x579f_39ab_91a0_1865;
pub const B_ARG7: u64 = 0xe003_e2bb_6439_9af9;
pub const B_PRESET_8: u64 = 0x7716_0113_ac8d_ca3f;
pub const B_PRESET_C: u64 = 0x8f87_0a5c_7060_b0f4;
pub const U_TRUE: u64 = 0x418a_9844_1f59_a54c;
pub const B_PRESET_11: u64 = 0x1165_1f4d_c984_1aa4;
pub const B_ARG11: u64 = 0x9941_437e_56d6_6d25;
pub const U_TIME: u64 = 0x1e54_85ef_2e9f_9384;
pub const U_FLAG_INT: u64 = 0xc5ab_673f_b086_51f5;
pub const U_DEPTH_DIMS: u64 = 0x531b_b931_8eea_4971;
pub const U_LANTERN: u64 = 0x9b88_329e_408f_f69f;
pub const B_ARG10: u64 = 0xba3e_6caf_0257_74d7;
pub const B_GBUFFER_0: u64 = 0xfe8d_a59f_9774_9daa;
pub const B_GBUFFER_1: u64 = 0x0d36_dd99_2ba2_9df1;
pub const B_GBUFFER_2: u64 = 0x4b1b_8bd2_36f1_4df0;
pub const S_SPATIAL: u64 = 0xbc5a_d80c_0ba9_15c3; // -0x43a527f3f456ea3d
pub const S_LANTERN: u64 = 0x0c6b_33eb_2099_2aad;
pub const B_ARG12: u64 = 0x5a67_3470_2611_6ffe;
pub const R_RT_BUFFER_0: u64 = 0x2bd0_ac28_9087_c8e1;
pub const R_RT_BUFFER_1: u64 = 0x129f_01ce_8d54_06a0;
pub const B_PRESET_9: u64 = 0x61b0_8b5e_0470_c5f2;
pub const B_PRESET_A: u64 = 0x297d_1e4c_dae8_f657;
pub const B_PRESET_0: u64 = 0xb79d_1a8a_6a25_4145;
/// `write_uniform<u32>(0x5788…, arg6)`。
pub const U_ARG6: u64 = 0x5788_8394_3452_a027;

/// 五个条目：`(material 下标, flag)`（O：`[rbp+0xcc8..0xcf4]` 的 dword/byte 对）。
pub const ENTRIES: [(u32, bool); 5] = [(1, false), (3, false), (0, false), (0, true), (2, false)];

/// 调用方给出的输入（见模块头的 ABI 表）。
pub struct DeferredInputs<'a> {
    /// 资源 `0x78cac557…`：按 material 下标的着色器。
    pub shaders: [Option<Handle<RasterShaderProgram>>; 5],
    /// arg3：`[0]` G-buffer，`[1]` 只读深度与视口来源。
    pub lighting: [AttachmentTexture; 2],
    /// arg4：场景颜色附件（历史字段名保留；原调用方传 color）。
    pub depth: AttachmentTexture,
    pub arg5: AttachmentTexture,
    pub arg6: u8,
    pub arg7: AttachmentTexture,
    pub arg8: AttachmentTexture,
    /// arg9：三个相邻句柄。
    pub gbuffer: [AttachmentTexture; 3],
    pub arg10: AttachmentTexture,
    pub arg11: AttachmentTexture,
    pub arg12: AttachmentTexture,
    /// arg13：两个 RG 缓冲句柄（`read_rg_storage_buffer` 的资源号）。
    pub rt_buffers: [u32; 2],
    #[doc(hidden)]
    pub _marker: core::marker::PhantomData<&'a ()>,
}

fn sampled<'p>(b: DrawBuilder<'p>, t: &AttachmentTexture, hash: u64) -> DrawBuilder<'p> {
    b.bind_texture_to_draw_builder(BindTexture::RgHandle(t), hash, ACCESS_SAMPLED_IMAGE, None)
}

fn preset<'p>(b: DrawBuilder<'p>, p: u8, hash: u64) -> DrawBuilder<'p> {
    b.bind_texture_to_draw_builder(
        BindTexture::Preset(TexturePreset(p)),
        hash,
        ACCESS_SAMPLED_IMAGE,
        None,
    )
}

/// 一个条目内的完整绑定序列（O：0x1409d26da..0x1409d2dd8，逐调用）。
pub fn bind_all<'p, Tri: SsboType, Meta: SsboType, Bvh: SsboType, Wall: SsboType>(
    mut b: DrawBuilder<'p>,
    i: &DeferredInputs<'_>,
    time: f32,
    lantern_instance_count: u32,
) -> DrawBuilder<'p> {
    b = b.uniform(U_ARG6, i.arg6 as u32);
    b = sampled(b, &i.lighting[0], B_LIGHTING_0);
    b = sampled(b, &i.lighting[1], B_LIGHTING_1);
    b = sampled(b, &i.arg8, B_ARG8);
    b = sampled(b, &i.arg5, B_ARG5);
    b = b.bind_texture_to_draw_builder(
        BindTexture::FromDisk(TextureFromDisk(DISK_FILE)),
        B_DISK,
        ACCESS_SAMPLED_IMAGE,
        None,
    );
    b = sampled(b, &i.arg7, B_ARG7);
    b = preset(b, 8, B_PRESET_8);
    b = preset(b, 0xc, B_PRESET_C);
    b = b.uniform(U_TRUE, true);
    b = preset(b, 0x11, B_PRESET_11);
    b = sampled(b, &i.arg11, B_ARG11);
    b = b.optional_uniform(U_TIME, time);
    b = b.uniform(U_FLAG_INT, 0u32);
    b = b.optional_uniform(U_DEPTH_DIMS, [i.depth.extent.0, i.depth.extent.1]);
    b = b.uniform(U_LANTERN, lantern_instance_count);
    b = sampled(b, &i.arg10, B_ARG10);
    b = sampled(b, &i.gbuffer[0], B_GBUFFER_0);
    b = sampled(b, &i.gbuffer[1], B_GBUFFER_1);
    b = sampled(b, &i.gbuffer[2], B_GBUFFER_2);
    b = b.read_storage_buffer::<SpatialResolveOffsetsBuffer>("SpatialResolveOffsets", S_SPATIAL);
    b = b.read_storage_buffer::<LanternBuffer>("LanternSsbo", S_LANTERN);
    b = sampled(b, &i.arg12, B_ARG12);
    // 原版 *_light_sun_sky SPV：rdi_reservoir_tex 的所有 Load 都在 `lantern_count > 0`
    // 分支内；`restir_sample_direct_lights` 恰在 lantern_count == 0 时只创建从不写入的
    // 1x1 占位纹理（同一实例数来源）。保留绑定与同步，仅声明该读不要求内容。
    if lantern_instance_count == 0 {
        b = b.inactive_texture_contents(B_ARG12);
    }
    b = b.read_rg_storage_buffer(R_RT_BUFFER_0, i.rt_buffers[0]);
    b = b.read_rg_storage_buffer(R_RT_BUFFER_1, i.rt_buffers[1]);
    // 原版 *_light_sun_sky SPV：light_indirection_grid / grid_light_indices 的全部 Load
    // 都在 `lantern_count > 0` 分支内；`bin_lights` 在同一实例数为 0 时只建从不写入的
    // 4 字节占位缓冲。保留绑定与同步，仅声明该读不要求内容。
    if lantern_instance_count == 0 {
        b = b
            .inactive_storage_buffer_contents(R_RT_BUFFER_0)
            .inactive_storage_buffer_contents(R_RT_BUFFER_1);
    }
    b = b.bind_color_library::<ColorLibrarySsbo>();
    b = b.bind_ray_tracing::<Tri, Meta, Bvh, Wall>();
    b = preset(b, 9, B_PRESET_9);
    b = preset(b, 0xa, B_PRESET_A);
    preset(b, 0, B_PRESET_0)
}

/// 条目的默认状态改写（O：`0x1409d2507..0x1409d25f2` 与 `0x1409d25ed`）：返回 `(state, 模板参考值)`。
pub fn entry_state(default_state: u32, material: u32, flag: bool) -> (u32, u8) {
    let b0 = (default_state & 0xff) & 0xbf;
    let mid = default_state & 0x00ff_ff00;
    let mut b3 = default_state >> 24;
    let mut stencil = material as u8;
    if flag {
        stencil |= 0x80;
        b3 = (b3 & 0x9f) | 0x20;
    }
    (b0 | mid | (b3 << 24), stencil)
}

/// 收尾状态改写（O：0x1409d2e4c..0x1409d2e5b）。
pub fn final_state(s: u32) -> u32 {
    let b0 = (s & 0xff) & 0x7f;
    let b1 = ((s >> 8) & 0xff) | 1;
    let b2 = (((s >> 16) & 0xff) & 3) | 0xe8;
    let b3 = (((s >> 24) & 0xff) & 0xe0) | 0x10;
    b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
}

/// `deferred_raster(rg, world, …)`。
pub fn deferred_raster<Tri: SsboType, Meta: SsboType, Bvh: SsboType, Wall: SsboType>(
    ctx: &mut DrawPassCtx<'_>,
    i: &DeferredInputs<'_>,
) {
    let time = ctx.elapsed.as_secs_f32();
    // O：`AssetSsboLibrary::get::<LanternSsbo>` → 缺失 `expect`；值为实际灯笼实例数量。
    let lantern_instance_count = ctx
        .host()
        .ssbo_instance_count_typed(
            LanternBuffer::rg_buffer_hash(),
            core::any::type_name::<LanternBuffer>(),
        )
        .expect(UNWRAP_NONE);
    // B 1409d2216..2298: tag0 color = incoming r9, tag1 read-only
    // depth = incoming r8 + 0x24. render_world supplies {gbuffer, depth}
    // in r8 and the scene color in r9 (1404eca56..67).
    let colors = [ColorSpec::tag0(i.depth)];
    let depth = DepthSpec::ReadOnly {
        tex: i.lighting[1],
    };
    let mut pass = ctx.raster_pass(PASS_NAME, &colors, depth, i.lighting[1]);
    for (material, flag) in ENTRIES {
        // O：`cmp r13, 5; jae panic_bounds_check` / `test byte [..], 1; je unwrap_failed`。
        let shader = i.shaders[material as usize].clone().expect(UNWRAP_NONE);
        let (state, stencil) = entry_state(pass.host.default_raster_state().0, material, flag);
        let packet = DrawPacket::new(
            shader,
            [0, 0],
            crate::render::pass_builder::draw_stream::RasterState(state),
        );
        let mut b = DrawBuilder::new(&mut pass, packet, 0);
        b.packet_mut().unknown_b1 = stencil;
        let mut b = bind_all::<Tri, Meta, Bvh, Wall>(b, i, time, lantern_instance_count);
        let p = b.packet_mut();
        p.state.0 = final_state(p.state.0);
        b.draw_no_vao(3);
    }
    pass.render();
}
