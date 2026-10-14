//! `surface_water.rs` — W45 水表面 `water_surface_pass`（0x1409bf800，4292 B）。**真实实现**（B_d_water）。
//!
//! 标注：O = 机器码观测，I = 推断，U = 未知。栈布局 / 命名见 [`surface_support`](super::surface_support)。
//!
//! # 步骤（O：地址为该步第一条指令）
//! 1. 0x1409bf84b `BufferDesc::new_gpu_only(0x10, 0x20)` → `create_raw_resource("water_rt_count_buffer")` → 0x1409bf8ba `clear_rg_buffer`。
//! 2. 0x1409bf8bf `extent_2d(A+4)` → `n = w*h`（32 位）、`size = (n + (n>>4)) << 4` → `new_gpu_only(size, 0x20)` →
//!    `create_raw_resource("water_rt_ray_buffer")`（长度 0x13）。
//! 3. 0x1409bf9cf `RasterPass::new_impl("Water plane", [颜色 tag0], 深度 tag0 可写 layer 0 无清除, 视口 = extent_2d(depth))`；
//!    `Res<Time>`（TypeId `0x4bc362195b80f729`）→ `Duration::as_secs_f32` → `CommonDrawParams.time`，`flag = false`（`[rbp+0xb84] = 0`）。
//! 4. 列表 15（`+0x170/+0x178`）每个元素：`bind_common` →
//!    `half_depth_tex←A`、`taa_history_tex←B`、`write_uniform<u32>(history_valid ← [0xc20] u8)`、`reprojection_tex←D`、
//!    `write_rg_storage_buffer(water_rt_count_buffer)`、`write_rg_storage_buffer(water_rt_ray_buffer)`、`refraction_tex←C`
//!    （全部栅格绑定访问 0x1d）→ `state.byte3 = (b3 & 0x9f) | 0x40` → 回调。
//! 5. 0x1409bfe7a `RasterPass::render`。
//! 6. 0x1409bfeb7 `ComputePass::new("Water deferred rt args", "/water/calc_deferred_rt_args.cs.hlsl")` + `write_rg_storage_buffer(count)` +
//!    `PassBuilder::render`（闭包环境 `disc=1, (1,1,1)`，即 `dispatch_1d(1)`；常量 `0x142958fe0` = 四个 u32 1，反汇编器把它标成 float 是误标）。
//! 7. 0x1409c0062 `ComputePass::new("Water deferred rt", "/water/deferred_rt.cs.hlsl")`：`read_rg(count)`、`read_rg(ray)`、`half_depth_tex←A`、
//!    `taa_history_tex←B`、`write_uniform<u32>(history_valid)`、`reprojection_tex←D`、blue noise（磁盘）、`terrain_heightmap`(preset 9)、
//!    `visual_terrain_heightmap`(preset 0xa)、`terrain_lighting_texture`(preset 8)、blue noise（第二次，同一调用重复）、`bind_ray_tracing`、
//!    `color_tex←color`（访问 0x32）、`dispatch_indirect_raw(count, 0)`。
//!
//! 与 `WATER_OPS` 的对应：这些调用全部经 [`SurfaceEnv`] 助手落一个 `PlanOp`，测试断言一个绘制项时序列 == `WATER_OPS`。
//! 注意 `refraction_tex` 只在栅格里绑定（compute 里没有）。

use crate::render::pass_builder::framebuffer_binding::ColorAttachment;
use crate::systems::render_loop::draw_passes::d_main_support::DrawPassCtx;

use super::surface_support::{
    PRESET_TERRAIN_HEIGHTMAP, PRESET_TERRAIN_LIGHTING, PRESET_VISUAL_TERRAIN_HEIGHTMAP, SurfaceEnv,
    SurfaceInputs, att, d_sampled, d_uniform_u32, d_write_rg, names, run_surface_raster,
};
use super::{
    SURFACE_LIST, WATER_ARGS_PASS_NAME, WATER_ARGS_SHADER_URI, WATER_COUNT_BUFFER_NAME,
    WATER_COUNT_BUFFER_SIZE, WATER_PASS_NAME, WATER_RAY_BUFFER_NAME, WATER_RT_PASS_NAME,
    WATER_RT_SHADER_URI,
};

/// `water_surface_pass(rg, world, color, depth, A, B, history_valid, C, D, lists)`。
pub fn water_surface_pass(
    ctx: &mut DrawPassCtx<'_>,
    env: &mut SurfaceEnv<'_>,
    i: &SurfaceInputs<'_>,
) {
    // 1. 计数缓冲 + 清零。
    let (count, _) = env.create_buffer(WATER_COUNT_BUFFER_NAME, WATER_COUNT_BUFFER_SIZE);
    env.clear_buffer(&count);
    // 2. 射线缓冲：尺寸由 A 的 extent 决定（32 位算术，见 `water_ray_buffer_size`）。
    let [w, h] = i.half_depth.desc.extent_2d();
    let (ray, _) = env.create_buffer(WATER_RAY_BUFFER_NAME, super::water_ray_buffer_size(w, h));

    // 3-5. 栅格 pass "Water plane"。
    let colors = [ColorAttachment::Texture(att(i.color))];
    let history_valid = u32::from(i.history_valid);
    {
        let e: &SurfaceEnv<'_> = env;
        run_surface_raster(
            ctx,
            e,
            WATER_PASS_NAME,
            &colors,
            i.depth,
            i.lists.list(SURFACE_LIST),
            &|b| {
                let b = d_sampled(e, b, i.half_depth, names::HALF_DEPTH_TEX);
                let b = d_sampled(e, b, i.taa_history, names::TAA_HISTORY_TEX);
                let b = d_uniform_u32(e, b, names::HISTORY_VALID, history_valid);
                let b = d_sampled(e, b, i.reprojection, names::REPROJECTION_TEX);
                let b = d_write_rg(e, b, names::WATER_RT_COUNT_BUFFER, &count);
                let b = d_write_rg(e, b, names::WATER_RT_RAY_BUFFER, &ray);
                d_sampled(e, b, i.refraction, names::REFRACTION_TEX)
            },
        );
    }

    // 6. "Water deferred rt args"：计数缓冲写 + 单线程。
    let args = env
        .compute_pass(WATER_ARGS_PASS_NAME, WATER_ARGS_SHADER_URI)
        .write_rg(names::WATER_RT_COUNT_BUFFER, &count)
        .dispatch_1d(1);
    env.record(args);

    // 7. "Water deferred rt"：间接 dispatch（线程数来自 count 缓冲）。
    let rt = env
        .compute_pass(WATER_RT_PASS_NAME, WATER_RT_SHADER_URI)
        .read_rg(names::WATER_RT_COUNT_BUFFER, &count)
        .read_rg(names::WATER_RT_RAY_BUFFER, &ray)
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
        .tex_write(names::COLOR_TEX, i.color)
        .dispatch_indirect(&count);
    env.record(rt);
}
