//! `surface_ice.rs` — W45 冰表面 `ice_surface_pass`（0x1409c08d0，12711 B）。**真实实现**（B_d_water）。
//!
//! 标注：O = 机器码观测，I = 推断，U = 未知。栈布局 / 绑定名见 [`surface_support`](super::surface_support)。
//! 局部变量下标是 `rbp+X` 栈槽（资源句柄 = `{id: u32, desc}`，id 在 +0；缓冲 = `{desc 16 B, id}`，id 在 +0x10）。
//!
//! # 步骤（O）
//! 1. 0x1409c0909.. 纹理（`new_2d(extent_2d(color.desc), 格式)`，随后 `create_raw_resource`）：
//!    `"ice roughness tex"`（格式 9，slot 0x20c）、`"ice refr hits"`（0x62，0x1a0）、`"ice refr hit radiance"`（0x62 + `sampled_with_different_format`，0x17c）。
//! 2. 0x1409c0af3.. 缓冲：`"ice_refl_count_buffer"`、`"ice_rt_count_buffer"`（各 `new_gpu_only(0x10, 0x20)`，slot 0xbc0 / 0x6a0），各经
//!    `rg_buffer_copy_from_slice<u32>` 写入 `[0, 1, 1, 0]`（rodata 0x142abfce4，I：间接 dispatch 参数的初值，x = 计数）；
//!    `n = A.w*A.h`（32 位），`"ice_refl_px_buffer"` = `new_gpu_only(4*(n + n>>4), 0x20)`（slot 0x260，`lea rdx,[4*rsi]`），
//!    `"ice_rt_ray_buffer"` = `new_gpu_only((n + n>>4) << 4, 0x20)`（slot 0x160）。
//! 3. 0x1409c0d80 `get_or_create_temporal_pair("Ice shadows", new_2d(extent(color), 9).sampled().color_attachment()).unwrap()`：
//!    当前帧 → slot 0x130（栅格第 5 颜色），上一帧 → slot 0x100（`shadows_history_tex`）；标志字节 → `[0xd8c]`。
//! 4. 0x1409c0f2d `RasterPass::new_impl("Ice plane", 5 个颜色, 深度 tag0 可写 layer 0 无清除, 视口 = extent_2d(depth))`：
//!    颜色 0 = `color`（tag0）；1 = `ice roughness tex`（tag1，视图字 = 四个 f32 1.0）；2 = `ice refr hits`（tag1，视图字 0）；
//!    3 = `ice refr hit radiance`（**tag2**）；4 = `Ice shadows` 当前帧（tag1，视图字 0）。
//! 5. 每个列表 15 元素：`bind_common` → `write_uniform<bool>(bool_render_shadows, true)` → `shadow_map`（preset 0x11）→ `half_depth_tex←A` →
//!    `taa_history_tex←B` → `shadows_history_tex←上一帧` → `write_uniform<u32>(history_valid)`（值 = `[0xe20] != 0 ? 标志字节 : 0`，
//!    O：`cmp byte [0xe20],0; cmove ecx,0`）→ `reprojection_tex←D` → `refraction_tex←C` → `write_rg(ice_refl_count_buffer)` →
//!    `write_rg(ice_refl_px_buffer)` → `byte3 = (b3 & 0x9f) | 0x40` → 回调。`RasterPass::render`。
//! 6. 0x1409c1479 半分辨率纹理（`half_res(color.desc)` → `new_2d`）：`"water refl hit radiance"`（0x62 + different_format，slot 0x1e8）、
//!    `"water refl hits"`（0x53，slot 0x1c4），随后 `clear_rg_color_image(water refl hits)`。
//! 7. 四个计算 pass（顺序、绑定见各段注释）；`Ice refl march` / `Ice refl trace` 间接 dispatch，两个 resolve 为 `dispatch_2d(color 的 extent)`
//!    （闭包环境 `disc=1, x=w, y=h, z=1`；`r12 = extent_2d(color)` 在 0x1409c0d32 存入 `[rbp+0xba8]`，I：整段共用）。
//!    两个 temporal pair：`"water refr temporal"`（`new_2d(extent(color), 0x7a).sampled().storage()`）、
//!    `"ice refl ray len"`（`new_2d(extent(color), 0x53).sampled().storage()`），标志字节相与（`and ebx, [..+0x7fc]`）。
//!
//! 与 `ICE_OPS` 的对应见 [`surface_support`](super::surface_support) 模块头“黄金序列”。

use crate::render::pass_builder::framebuffer_binding::ColorAttachment;
use crate::rhapsody_types::{TextureDesc, TextureViewDesc};
use crate::systems::render_loop::draw_passes::d_main_support::DrawPassCtx;

use super::surface_support::{
    PRESET_BRDF_FG_LUT, PRESET_SHADOW_MAP, PRESET_TERRAIN_HEIGHTMAP, PRESET_TERRAIN_LIGHTING,
    PRESET_VISUAL_TERRAIN_HEIGHTMAP, RADIANCE_VIEW_FORMAT, RADIANCE_VIEW_USAGE, SurfaceEnv,
    SurfaceInputs, att, d_preset, d_sampled, d_uniform_bool, d_uniform_u32, d_write_rg, half_res,
    names, ray_units, run_surface_raster, sampled_with_different_format,
};
use super::{ICE_COMPUTE_PASSES, ICE_PASS_NAME, ICE_RESOURCE_NAMES, SURFACE_LIST};

/// `new_gpu_only(0x10, 0x20)` 的两个计数缓冲的初值（O：rodata 0x142abfce4，16 B）。
pub const ICE_COUNT_INIT: [u32; 4] = [0, 1, 1, 0];
/// 栅格颜色 1（roughness）的视图字：四个 `1.0f32`（O：`movaps xmm0, [0x142925930]` 存到 `[rbp+0x7d4]`）。
pub const ROUGHNESS_VIEW_WORDS: [u32; 4] = [0x3f80_0000; 4];

/// 格式码（O：`mov r8d, imm` 给 `TextureDesc::new_2d`）。
pub mod format {
    pub const ROUGHNESS: u32 = 9;
    pub const HITS: u32 = 0x62;
    pub const SHADOWS: u32 = 9;
    pub const REFL_HITS: u32 = 0x53;
    pub const REFR_TEMPORAL: u32 = 0x7a;
    pub const RAY_LEN: u32 = 0x53;
}

fn radiance_view() -> TextureViewDesc {
    TextureViewDesc::new()
        .format(RADIANCE_VIEW_FORMAT)
        .usage(RADIANCE_VIEW_USAGE)
}

/// `ice_surface_pass(rg, world, color, depth, A, B, history_valid, C, D, lists)`。
pub fn ice_surface_pass(
    ctx: &mut DrawPassCtx<'_>,
    env: &mut SurfaceEnv<'_>,
    i: &SurfaceInputs<'_>,
) {
    let color_extent = i.color.desc.extent_2d();
    let [aw, ah] = i.half_depth.desc.extent_2d();
    let names_ = ICE_RESOURCE_NAMES;

    // 1. 纹理。
    let rough = env.create_texture(
        names_[0],
        TextureDesc::new_2d(color_extent, format::ROUGHNESS),
    );
    let refr_hits = env.create_texture(names_[1], TextureDesc::new_2d(color_extent, format::HITS));
    let refr_hit_radiance = env.create_texture(
        names_[2],
        sampled_with_different_format(TextureDesc::new_2d(color_extent, format::HITS)),
    );
    // 2. 缓冲。
    let (refl_count, refl_count_desc) =
        env.create_buffer(names_[3], super::WATER_COUNT_BUFFER_SIZE);
    let (rt_count, rt_count_desc) = env.create_buffer(names_[4], super::WATER_COUNT_BUFFER_SIZE);
    env.copy_u32(&refl_count, &refl_count_desc, &ICE_COUNT_INIT);
    env.copy_u32(&rt_count, &rt_count_desc, &ICE_COUNT_INIT);
    let units = u64::from(ray_units(aw, ah));
    let (refl_px, _) = env.create_buffer(names_[5], 4 * units);
    let (rt_ray, _) = env.create_buffer(names_[6], units << 4);
    // 3. "Ice shadows" temporal pair（`color_attachment()` 用途位在 `TextureDesc` 里无字段，U）。
    let shadows = env.temporal_pair(
        names_[11],
        TextureDesc::new_2d(color_extent, format::SHADOWS).sampled(),
    );
    let flag_in = i.history_valid != 0;
    let raster_history_valid = if flag_in { u32::from(shadows.fresh) } else { 0 };

    // 4-5. 栅格 pass "Ice plane"（5 个颜色附件）。
    let colors = [
        ColorAttachment::Texture(att(i.color)),
        ColorAttachment::View {
            view: ROUGHNESS_VIEW_WORDS,
            texture: att(&rough),
        },
        ColorAttachment::View {
            view: [0; 4],
            texture: att(&refr_hits),
        },
        ColorAttachment::Tag2(att(&refr_hit_radiance)),
        ColorAttachment::View {
            view: [0; 4],
            texture: att(&shadows.current),
        },
    ];
    {
        let e: &SurfaceEnv<'_> = env;
        run_surface_raster(
            ctx,
            e,
            ICE_PASS_NAME,
            &colors,
            i.depth,
            i.lists.list(SURFACE_LIST),
            &|b| {
                let b = d_uniform_bool(e, b, names::BOOL_RENDER_SHADOWS, true);
                let b = d_preset(e, b, PRESET_SHADOW_MAP, names::SHADOW_MAP);
                let b = d_sampled(e, b, i.half_depth, names::HALF_DEPTH_TEX);
                let b = d_sampled(e, b, i.taa_history, names::TAA_HISTORY_TEX);
                let b = d_sampled(e, b, &shadows.previous, names::SHADOWS_HISTORY_TEX);
                let b = d_uniform_u32(e, b, names::HISTORY_VALID, raster_history_valid);
                let b = d_sampled(e, b, i.reprojection, names::REPROJECTION_TEX);
                let b = d_sampled(e, b, i.refraction, names::REFRACTION_TEX);
                let b = d_write_rg(e, b, names::ICE_REFL_COUNT_BUFFER, &refl_count);
                d_write_rg(e, b, names::ICE_REFL_PX_BUFFER, &refl_px)
            },
        );
    }

    // 6. 半分辨率命中纹理。
    let half_extent = half_res(i.color.desc).extent_2d();
    let refl_hit_radiance = env.create_texture(
        names_[7],
        sampled_with_different_format(TextureDesc::new_2d(half_extent, format::HITS)),
    );
    let refl_hits = env.create_texture(
        names_[8],
        TextureDesc::new_2d(half_extent, format::REFL_HITS),
    );
    env.clear_image(&refl_hits);

    // 7a. "Ice refl march"。
    let (name, uri) = ICE_COMPUTE_PASSES[0];
    let history_valid = u32::from(i.history_valid);
    let p = env
        .compute_pass(name, uri)
        .read_rg(names::ICE_REFL_COUNT_BUFFER, &refl_count)
        .read_rg(names::ICE_REFL_PX_BUFFER, &refl_px)
        .tex(names::HALF_DEPTH_TEX, i.half_depth)
        .tex(names::TAA_HISTORY_TEX, i.taa_history)
        .uniform_u32(names::HISTORY_VALID, history_valid)
        .tex(names::REPROJECTION_TEX, i.reprojection)
        .disk(names::BLUE_NOISE_TEXTURE, names::BLUE_NOISE_FILE)
        .tex(names::ICE_ROUGHNESS_TEX, &rough)
        .preset(names::TERRAIN_HEIGHTMAP, PRESET_TERRAIN_HEIGHTMAP)
        .preset(
            names::VISUAL_TERRAIN_HEIGHTMAP,
            PRESET_VISUAL_TERRAIN_HEIGHTMAP,
        )
        .preset(names::TERRAIN_LIGHTING_TEXTURE, PRESET_TERRAIN_LIGHTING)
        .disk(names::BLUE_NOISE_TEXTURE, names::BLUE_NOISE_FILE)
        .ray_tracing()
        .tex_write(names::REFL_HIT_TEX, &refl_hits)
        .tex_write(names::REFL_HIT_RADIANCE_TEX, &refl_hit_radiance)
        .write_rg(names::ICE_RT_COUNT_BUFFER, &rt_count)
        .write_rg(names::ICE_RT_RAY_BUFFER, &rt_ray)
        .dispatch_indirect(&refl_count);
    env.record(p);

    // 7b. "Ice refl trace"（没有 roughness / 其余同 march）。
    let (name, uri) = ICE_COMPUTE_PASSES[1];
    let p = env
        .compute_pass(name, uri)
        .read_rg(names::ICE_RT_COUNT_BUFFER, &rt_count)
        .read_rg(names::ICE_RT_RAY_BUFFER, &rt_ray)
        .tex(names::HALF_DEPTH_TEX, i.half_depth)
        .tex(names::TAA_HISTORY_TEX, i.taa_history)
        .uniform_u32(names::HISTORY_VALID, history_valid)
        .tex(names::REPROJECTION_TEX, i.reprojection)
        .disk(names::BLUE_NOISE_TEXTURE, names::BLUE_NOISE_FILE)
        .preset(names::TERRAIN_HEIGHTMAP, PRESET_TERRAIN_HEIGHTMAP)
        .preset(
            names::VISUAL_TERRAIN_HEIGHTMAP,
            PRESET_VISUAL_TERRAIN_HEIGHTMAP,
        )
        .preset(names::TERRAIN_LIGHTING_TEXTURE, PRESET_TERRAIN_LIGHTING)
        .disk(names::BLUE_NOISE_TEXTURE, names::BLUE_NOISE_FILE)
        .ray_tracing()
        .tex_write(names::REFL_HIT_TEX, &refl_hits)
        .tex_write(names::REFL_HIT_RADIANCE_TEX, &refl_hit_radiance)
        .dispatch_indirect(&rt_count);
    env.record(p);

    // 两个 temporal pair（顺序：refr temporal、ray len；标志字节相与）。
    let refr_temporal = env.temporal_pair(
        names_[9],
        TextureDesc::new_2d(color_extent, format::REFR_TEMPORAL)
            .sampled()
            .storage(),
    );
    let ray_len = env.temporal_pair(
        names_[10],
        TextureDesc::new_2d(color_extent, format::RAY_LEN)
            .sampled()
            .storage(),
    );
    let resolve_valid = u32::from(refr_temporal.fresh & ray_len.fresh) & 1;
    let view = radiance_view();

    // 7c. "Ice refr resolve"。
    let (name, uri) = ICE_COMPUTE_PASSES[2];
    let p = env
        .compute_pass(name, uri)
        .tex(names::REFR_HIT_TEX, &refr_hits)
        .tex_view(names::REFR_HIT_RADIANCE_TEX, &refr_hit_radiance, &view)
        .tex(names::DEPTH_TEX, i.depth)
        .tex_write(names::REFL_REFR_RADIANCE_OUT_TEX, &refr_temporal.current)
        .dispatch_2d(color_extent[0], color_extent[1]);
    env.record(p);

    // 7d. "Ice refl resolve"。
    let (name, uri) = ICE_COMPUTE_PASSES[3];
    let p = env
        .compute_pass(name, uri)
        .preset(names::BRDF_FG_LUT, PRESET_BRDF_FG_LUT)
        .tex(names::REFL_HIT_TEX, &refl_hits)
        .tex_view(names::REFL_HIT_RADIANCE_TEX, &refl_hit_radiance, &view)
        .tex(names::DEPTH_TEX, i.depth)
        .disk(names::BLUE_NOISE_TEXTURE, names::BLUE_NOISE_FILE)
        .tex(names::REPROJECTION_TEX, i.reprojection)
        .tex(names::ICE_ROUGHNESS_TEX, &rough)
        .tex_write(names::COLOR_TEX, i.color)
        .uniform_u32(names::HISTORY_VALID, resolve_valid)
        .tex(
            names::REFL_REFR_RADIANCE_HISTORY_TEX,
            &refr_temporal.previous,
        )
        .tex_write(names::REFL_REFR_RADIANCE_OUT_TEX, &refr_temporal.current)
        .tex(names::RAY_LEN_HISTORY_TEX, &ray_len.previous)
        .tex_write(names::RAY_LEN_OUT_TEX, &ray_len.current)
        .dispatch_2d(color_extent[0], color_extent[1]);
    env.record(p);
}
