//! transparency_pass.rs — W44 折射拷贝 / W45 水·冰表面 / W46 pre-TAA 与 W53 post-TAA 透明。
//!
//! 地址（O）：`refraction_copy_pass` 0x1409bf320（1239 B）、`water_surface_pass` 0x1409bf800（4292 B）、
//! `ice_surface_pass` 0x1409c08d0（12711 B）、`pre_taa_transparency_pass` 0x1409c3a80（1575 B）、
//! `post_taa_transparency_pass` 0x1409c40b0（945 B）。`select_water_surface`：`render_world` 0x1404ec110 按
//! `GladeType::is_winter`（0x140ae8320）派发。
//!
//! # 列表驱动的三个 pass（完整恢复，见函数文档）
//! 都是 [`d_main_support::run_list_pass`] 的实例：
//! | pass | 名字 | 颜色 | 深度 | 列表 | flag | 额外 bind | 状态改写 |
//! |---|---|---|---|---|---|---|---|
//! | refraction_copy | "Copy refraction target"(0x16) | tag1(视图字 0) | 只读(tag1) | 14（`+0x158`） | false | RG 句柄 → `0xd48bea13951c1ddc`（0x1d） | `byte0 &= 0xbf` |
//! | pre_taa | "Pre-TAA transparency"(0x14) | tag0 | 只读 | 16（`+0x188`） | false | 深度 → `0x4861579da049fbfc`、B → `0x2a333c3a28028230`、C → `0x04175185826c09d0` | `byte0 &= 0xbf` + `byte3 field2` |
//! | post_taa | "Post-TAA transparency"(0x15) | tag0 | 无 | 17（`+0x1a0`），**空列表直接返回** | true | — | 同上 |
//!
//! # 水/冰表面（W45）：真实实现（B_d_water）
//! [`water_surface_pass`]（[`surface_water`]）与 [`ice_surface_pass`]（[`surface_ice`]）现在是真实 pass：RG 资源创建
//! （`RgResources`）、`clear_rg_buffer` / `clear_rg_color_image` / `rg_buffer_copy_from_slice<u32>`、一个 `RasterPass`
//! （`Water plane` / `Ice plane`，列表 15）与 2 / 4 个 `ComputePass`（着色器 URI 见 [`WATER_ARGS_SHADER_URI`] /
//! [`ICE_COMPUTE_PASSES`]）。每一步的资源来源、格式码、尺寸公式、绑定名（FNV-1a 反射名）见两个文件的模块头与 [`surface_support`]。
//! [`WATER_OPS`] / [`ICE_OPS`] 保留为**黄金序列**：`SurfaceEnv::trace` 记录真实调用序列，测试断言（一个绘制项）与表一致。
//! 旧的“只返回计划”的 `water_surface_pass()` / `ice_surface_pass()` 改名为 [`water_surface_plan`] / [`ice_surface_plan`]。
//!
//! deferred_raster / final_post 已按同样方法完整接线（见各自文件）。

#![allow(dead_code, unused_imports)]

#[path = "transparency_pass/surface_ice.rs"]
pub mod surface_ice;
#[path = "transparency_pass/surface_support.rs"]
pub mod surface_support;
#[cfg(test)]
#[path = "transparency_pass/surface_tests.rs"]
mod surface_tests;
#[path = "transparency_pass/surface_water.rs"]
pub mod surface_water;

pub use surface_ice::ice_surface_pass;
pub use surface_support::{SurfaceCompute, SurfaceEnv, SurfaceInputs};
pub use surface_water::water_surface_pass;

use crate::render::pass_builder::draw_builder::BindTexture;
use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
use crate::resources::session::GladeType;
use crate::systems::render_loop::draw_passes::d_main_support::{
    ColorSpec, DepthSpec, DrawPassCtx, ItemMode, ListPass, run_list_pass,
};
use crate::systems::render_loop::draw_passes::draw_lists::DrawLists;

/// 水面变体（W45 派发结果；O：`GladeType::is_winter` 选冰）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WaterSurfaceKind {
    Water,
    Ice,
}

/// W45 表面选择。
pub fn select_water_surface(glade: GladeType) -> WaterSurfaceKind {
    if glade.is_winter() {
        WaterSurfaceKind::Ice
    } else {
        WaterSurfaceKind::Water
    }
}

pub const REFRACTION_PASS_NAME: &str = "Copy refraction target";
pub const REFRACTION_BINDING: u64 = 0xd48b_ea13_951c_1ddc; // -0x2b7415ec6ae3e224
pub const REFRACTION_LIST: usize = 14;
pub const PRE_TAA_PASS_NAME: &str = "Pre-TAA transparency";
pub const PRE_TAA_LIST: usize = 16;
pub const PRE_TAA_DEPTH_BINDING: u64 = 0x4861_579d_a049_fbfc;
pub const PRE_TAA_B_BINDING: u64 = 0x2a33_3c3a_2802_8230;
pub const PRE_TAA_C_BINDING: u64 = 0x0417_5185_826c_09d0;
pub const POST_TAA_PASS_NAME: &str = "Post-TAA transparency";
pub const POST_TAA_LIST: usize = 17;

/// W44 — 折射拷贝（O：0x1409bf320，外部门 `*0x14362c098 == 0` 在 `render_world`）。
///
/// 参数（O 寄存器）：`r8` = 颜色（tag1，视图字全 0）、`r9` = 被采样的折射源（RG 句柄）、栈参 5 = 深度（只读，视口取其 extent）。
pub fn refraction_copy_pass(
    ctx: &mut DrawPassCtx<'_>,
    color: &AttachmentTexture,
    source: &AttachmentTexture,
    depth: &AttachmentTexture,
    lists: &DrawLists,
) {
    let colors = [ColorSpec::tag1(*color)];
    let spec = ListPass {
        name: REFRACTION_PASS_NAME,
        colors: &colors,
        depth: DepthSpec::ReadOnly { tex: *depth },
        viewport: *depth,
        flag: false,
        mode: ItemMode::Transparent {
            byte3_field2: false,
        },
        variant: 0,
        extra: Some(&|b| {
            b.bind_texture_to_draw_builder(
                BindTexture::RgHandle(source),
                REFRACTION_BINDING,
                crate::render::pass_builder::draw_builder::ACCESS_SAMPLED_IMAGE,
                None,
            )
        }),
    };
    run_list_pass(ctx, &spec, lists.list(REFRACTION_LIST));
}

/// W46 — pre-TAA 透明（O：0x1409c3a80）。`depth` 同时是只读深度附件与第一个被采样纹理；`b` / `c` 是栈参 5/6 的两张 RG 纹理。
pub fn pre_taa_transparency_pass(
    ctx: &mut DrawPassCtx<'_>,
    color: &AttachmentTexture,
    depth: &AttachmentTexture,
    b: &AttachmentTexture,
    c: &AttachmentTexture,
    lists: &DrawLists,
) {
    use crate::render::pass_builder::draw_builder::ACCESS_SAMPLED_IMAGE;
    let colors = [ColorSpec::tag0(*color)];
    let spec = ListPass {
        name: PRE_TAA_PASS_NAME,
        colors: &colors,
        depth: DepthSpec::ReadOnly { tex: *depth },
        viewport: *depth,
        flag: false,
        mode: ItemMode::Transparent { byte3_field2: true },
        variant: 0,
        extra: Some(&|bld| {
            bld.bind_texture_to_draw_builder(
                BindTexture::RgHandle(depth),
                PRE_TAA_DEPTH_BINDING,
                ACCESS_SAMPLED_IMAGE,
                None,
            )
            .bind_texture_to_draw_builder(
                BindTexture::RgHandle(b),
                PRE_TAA_B_BINDING,
                ACCESS_SAMPLED_IMAGE,
                None,
            )
            .bind_texture_to_draw_builder(
                BindTexture::RgHandle(c),
                PRE_TAA_C_BINDING,
                ACCESS_SAMPLED_IMAGE,
                None,
            )
        }),
    };
    run_list_pass(ctx, &spec, lists.list(PRE_TAA_LIST));
}

/// W53 — post-TAA 透明（O：0x1409c40b0）：列表（`+0x1a0/+0x1a8`）为空 → 整个 pass 不创建。
pub fn post_taa_transparency_pass(
    ctx: &mut DrawPassCtx<'_>,
    color: &AttachmentTexture,
    lists: &DrawLists,
) {
    if lists.list(POST_TAA_LIST).is_empty() {
        return;
    }
    let colors = [ColorSpec::tag0(*color)];
    let spec = ListPass {
        name: POST_TAA_PASS_NAME,
        colors: &colors,
        depth: DepthSpec::None,
        viewport: *color,
        flag: true,
        mode: ItemMode::Transparent { byte3_field2: true },
        variant: 0,
        extra: None,
    };
    run_list_pass(ctx, &spec, lists.list(POST_TAA_LIST));
}

// ---------------------------------------------------------------------------------------------
// 水 / 冰：B_d_gap 补充的机器码事实（字符串 / 尺寸 / 列表 / 参数布局）
// ---------------------------------------------------------------------------------------------

/// 水面 / 冰面表面 pass 读取的列表：`+0x170`（ptr）/ `+0x178`（len）= 下标 15（O：`water_surface_pass` 0x1409bfaae、
/// `ice_surface_pass` 0x1409c0ffd；元素步长 `shl 7` = 0x80）。即 `DesignatedRenderPass(0xf)` 的实体
/// （`startup_water` 的水面 / 冰面 mesh）。
pub const SURFACE_LIST: usize =
    crate::systems::render_loop::draw_passes::draw_lists::list::WATER_SURFACE;

/// `water_surface_pass` 的栈布局（O：Win64，`sub rsp, 0xc28`，`rbp = rsp + 0x80`，栈参数 `[rbp + 0xc10 + 8*i]`）：
/// `rcx` = `&mut RenderGraph`、`rdx` = `&World`、`r8` = 颜色（tag0）、`r9` = 深度（tag0 可写；`extent_2d(r9+4)` = 视口）、
/// `[0xc10]` = `&Handle<Texture>`（其 `extent_2d(+4)` 决定射线缓冲大小）、`[0xc20]` = u8（`bind_common` 的标志）、
/// `[0xc38]` = `&DrawLists`。
pub const WATER_PASS_NAME: &str = "Water plane";
pub const WATER_COUNT_BUFFER_NAME: &str = "water_rt_count_buffer";
pub const WATER_RAY_BUFFER_NAME: &str = "water_rt_ray_buffer";
/// `BufferDesc::new_gpu_only(0x10, 0x20)`（`water_rt_count_buffer`；创建后 `clear_rg_buffer`）。
pub const WATER_COUNT_BUFFER_SIZE: u64 = 0x10;
/// 两个缓冲的用途标志（O：`mov r8d, 0x20`）。
pub const WATER_BUFFER_USAGE: u32 = 0x20;
pub const WATER_ARGS_PASS_NAME: &str = "Water deferred rt args";
pub const WATER_ARGS_SHADER_URI: &str = "/water/calc_deferred_rt_args.cs.hlsl";
pub const WATER_RT_PASS_NAME: &str = "Water deferred rt";
pub const WATER_RT_SHADER_URI: &str = "/water/deferred_rt.cs.hlsl";

/// `water_rt_ray_buffer` 的字节数（O：0x1409bf8c8..0x1409bf8e4：`n = w*h`（32 位乘）；`size = (n + (n >> 4)) << 4`，
/// 加法 32 位回绕、左移在 64 位）。
pub fn water_ray_buffer_size(width: u32, height: u32) -> u64 {
    let n = width.wrapping_mul(height);
    (n.wrapping_add(n >> 4) as u64) << 4
}

/// 冰面 pass 创建的 RG 资源名与 pass 名（O：`ice_surface_pass` 内 `create_raw_resource` 的字符串 + 长度）。
pub const ICE_PASS_NAME: &str = "Ice plane";
pub const ICE_RESOURCE_NAMES: [&str; 13] = [
    "ice roughness tex",
    "ice refr hits",
    "ice refr hit radiance",
    "ice_refl_count_buffer",
    "ice_rt_count_buffer",
    "ice_refl_px_buffer",
    "ice_rt_ray_buffer",
    "water refl hit radiance",
    "water refl hits",
    "water refr temporal",
    "ice refl ray len",
    "Ice shadows",
    "Ice plane",
];
/// 冰面的四个计算 pass：`(名字, 着色器 URI)`（O：`ComputePass::new` 的字符串）。
pub const ICE_COMPUTE_PASSES: [(&str, &str); 4] = [
    ("Ice refl march", "/water/ice_refl_march.cs.hlsl"),
    ("Ice refl trace", "/water/ice_deferred_rt.cs.hlsl"),
    ("Ice refr resolve", "/water/ice_refr_resolve.cs.hlsl"),
    ("Ice refl resolve", "/water/ice_refl_resolve.cs.hlsl"),
];

// ---------------------------------------------------------------------------------------------
// 水 / 冰：按机器码提取的操作计划
// ---------------------------------------------------------------------------------------------

/// 操作种类（O：被调用的 rhapsody / A4 函数）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanKind {
    NewGpuOnlyBuffer,
    NewTexture2d,
    CreateRawResource,
    ClearRgBuffer,
    RasterPassNew,
    BindTextureDraw,
    BindTextureCompute,
    WriteUniform,
    WriteRgBuffer,
    ReadRgBuffer,
    ReadStorageBuffer,
    WriteStorageBuffer,
    DrawNoVao,
    RasterRender,
    ComputePassNew,
    ComputeRender,
    BindRayTracing,
    BindColorLibrary,
    DispatchIndirect,
    Dispatch1d,
    Dispatch2d,
    DispatchGroups,
    WriteOptionalUniform,
}

/// 一步操作：种类 + 绑定名哈希（若有，O：紧邻调用前的 `movabs r8`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlanOp {
    pub kind: PlanKind,
    pub key: Option<u64>,
}

impl PlanOp {
    pub const fn new(kind: PlanKind, key: Option<u64>) -> PlanOp {
        PlanOp { kind, key }
    }
}

/// `water_surface_pass`（0x1409bf800）的操作序列。
pub const WATER_OPS: &[PlanOp] = &[
    PlanOp::new(PlanKind::NewGpuOnlyBuffer, None),
    PlanOp::new(PlanKind::CreateRawResource, None),
    PlanOp::new(PlanKind::ClearRgBuffer, None),
    PlanOp::new(PlanKind::NewGpuOnlyBuffer, None),
    PlanOp::new(PlanKind::CreateRawResource, None),
    PlanOp::new(PlanKind::RasterPassNew, None),
    PlanOp::new(PlanKind::BindTextureDraw, Some(0x2a333c3a28028230)),
    PlanOp::new(PlanKind::BindTextureDraw, Some(0x04175185826c09d0)),
    PlanOp::new(PlanKind::WriteUniform, Some(0xa9a109b760eed902)),
    PlanOp::new(PlanKind::BindTextureDraw, Some(0xb8a74ccd835a6be3)),
    PlanOp::new(PlanKind::WriteRgBuffer, Some(0x1863d10d6efe0b4e)),
    PlanOp::new(PlanKind::WriteRgBuffer, Some(0x8f7b94e37966c87b)),
    PlanOp::new(PlanKind::BindTextureDraw, Some(0xe9208e8a28884e3c)),
    PlanOp::new(PlanKind::RasterRender, None),
    PlanOp::new(PlanKind::ComputePassNew, None),
    PlanOp::new(PlanKind::WriteRgBuffer, Some(0x1863d10d6efe0b4e)),
    PlanOp::new(PlanKind::ComputeRender, None),
    PlanOp::new(PlanKind::ComputePassNew, None),
    PlanOp::new(PlanKind::ReadRgBuffer, Some(0x1863d10d6efe0b4e)),
    PlanOp::new(PlanKind::ReadRgBuffer, Some(0x8f7b94e37966c87b)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x2a333c3a28028230)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x04175185826c09d0)),
    PlanOp::new(PlanKind::WriteUniform, Some(0xa9a109b760eed902)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xb8a74ccd835a6be3)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x815bf5036966978c)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x61b08b5e0470c5f2)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x297d1e4cdae8f657)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x77160113ac8dca3f)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x815bf5036966978c)),
    PlanOp::new(PlanKind::BindRayTracing, None),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xd9978e541ce9a95e)),
    PlanOp::new(PlanKind::DispatchIndirect, None),
];

/// `ice_surface_pass`（0x1409c08d0）的操作序列。
pub const ICE_OPS: &[PlanOp] = &[
    PlanOp::new(PlanKind::NewTexture2d, None),
    PlanOp::new(PlanKind::CreateRawResource, None),
    PlanOp::new(PlanKind::NewTexture2d, None),
    PlanOp::new(PlanKind::CreateRawResource, None),
    PlanOp::new(PlanKind::NewTexture2d, None),
    PlanOp::new(PlanKind::CreateRawResource, None),
    PlanOp::new(PlanKind::NewGpuOnlyBuffer, None),
    PlanOp::new(PlanKind::CreateRawResource, None),
    PlanOp::new(PlanKind::NewGpuOnlyBuffer, None),
    PlanOp::new(PlanKind::CreateRawResource, None),
    PlanOp::new(PlanKind::NewGpuOnlyBuffer, None),
    PlanOp::new(PlanKind::CreateRawResource, None),
    PlanOp::new(PlanKind::NewGpuOnlyBuffer, None),
    PlanOp::new(PlanKind::CreateRawResource, None),
    PlanOp::new(PlanKind::NewTexture2d, None),
    PlanOp::new(PlanKind::RasterPassNew, None),
    PlanOp::new(PlanKind::WriteUniform, Some(0x418a98441f59a54c)),
    PlanOp::new(PlanKind::BindTextureDraw, Some(0x11651f4dc9841aa4)),
    PlanOp::new(PlanKind::BindTextureDraw, Some(0x2a333c3a28028230)),
    PlanOp::new(PlanKind::BindTextureDraw, Some(0x04175185826c09d0)),
    PlanOp::new(PlanKind::BindTextureDraw, Some(0xfdfab2518469de1b)),
    PlanOp::new(PlanKind::WriteUniform, Some(0xa9a109b760eed902)),
    PlanOp::new(PlanKind::BindTextureDraw, Some(0xb8a74ccd835a6be3)),
    PlanOp::new(PlanKind::BindTextureDraw, Some(0xe9208e8a28884e3c)),
    PlanOp::new(PlanKind::WriteRgBuffer, Some(0x2a3415eab2e8de29)),
    PlanOp::new(PlanKind::WriteRgBuffer, Some(0xfc902035b75f0652)),
    PlanOp::new(PlanKind::RasterRender, None),
    PlanOp::new(PlanKind::NewTexture2d, None),
    PlanOp::new(PlanKind::CreateRawResource, None),
    PlanOp::new(PlanKind::NewTexture2d, None),
    PlanOp::new(PlanKind::CreateRawResource, None),
    PlanOp::new(PlanKind::ComputePassNew, None),
    PlanOp::new(PlanKind::ReadRgBuffer, Some(0x2a3415eab2e8de29)),
    PlanOp::new(PlanKind::ReadRgBuffer, Some(0xfc902035b75f0652)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x2a333c3a28028230)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x04175185826c09d0)),
    PlanOp::new(PlanKind::WriteUniform, Some(0xa9a109b760eed902)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xb8a74ccd835a6be3)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x815bf5036966978c)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x53902bebe45ab883)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x61b08b5e0470c5f2)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x297d1e4cdae8f657)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x77160113ac8dca3f)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x815bf5036966978c)),
    PlanOp::new(PlanKind::BindRayTracing, None),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x2c5901276b7f6bd6)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xecc9f908d1aab342)),
    PlanOp::new(PlanKind::WriteRgBuffer, Some(0xf0512b889439e93e)),
    PlanOp::new(PlanKind::WriteRgBuffer, Some(0x06541d6cfdf253ab)),
    PlanOp::new(PlanKind::DispatchIndirect, None),
    PlanOp::new(PlanKind::ComputePassNew, None),
    PlanOp::new(PlanKind::ReadRgBuffer, Some(0xf0512b889439e93e)),
    PlanOp::new(PlanKind::ReadRgBuffer, Some(0x06541d6cfdf253ab)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x2a333c3a28028230)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x04175185826c09d0)),
    PlanOp::new(PlanKind::WriteUniform, Some(0xa9a109b760eed902)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xb8a74ccd835a6be3)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x815bf5036966978c)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x61b08b5e0470c5f2)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x297d1e4cdae8f657)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x77160113ac8dca3f)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x815bf5036966978c)),
    PlanOp::new(PlanKind::BindRayTracing, None),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x2c5901276b7f6bd6)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xecc9f908d1aab342)),
    PlanOp::new(PlanKind::DispatchIndirect, None),
    PlanOp::new(PlanKind::NewTexture2d, None),
    PlanOp::new(PlanKind::NewTexture2d, None),
    PlanOp::new(PlanKind::ComputePassNew, None),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x1128f800d215bbe4)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xde479e07731179b4)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x4861579da049fbfc)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xb8c260c7c1df9cab)),
    PlanOp::new(PlanKind::ComputeRender, None),
    PlanOp::new(PlanKind::ComputePassNew, None),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xb79d1a8a6a254145)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x2c5901276b7f6bd6)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xecc9f908d1aab342)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x4861579da049fbfc)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x815bf5036966978c)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xb8a74ccd835a6be3)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x53902bebe45ab883)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xd9978e541ce9a95e)),
    PlanOp::new(PlanKind::WriteUniform, Some(0xa9a109b760eed902)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xf459974fb83cd293)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0xb8c260c7c1df9cab)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x69d7bdb75a2f4016)),
    PlanOp::new(PlanKind::BindTextureCompute, Some(0x96c7a43d54eae8fa)),
    PlanOp::new(PlanKind::ComputeRender, None),
];

/// 一个表面 pass 的计划描述。
#[derive(Clone, Copy, Debug)]
pub struct SurfacePlan {
    pub kind: WaterSurfaceKind,
    /// 栅格 pass 名（O："Water plane"(0xb) / "Ice plane"）。
    pub raster_pass: &'static str,
    /// 计算 pass 名与着色器 URI（O：字符串池）。
    pub compute_passes: &'static [(&'static str, &'static str)],
    /// 创建的 RG 资源名（O：`create_raw_resource` 前的名字）。
    pub resources: &'static [&'static str],
    pub ops: &'static [PlanOp],
}

/// W45 水（O：0x1409bf800）。
pub const WATER_PLAN: SurfacePlan = SurfacePlan {
    kind: WaterSurfaceKind::Water,
    raster_pass: "Water plane",
    compute_passes: &[
        (
            "Water deferred rt args",
            "/water/calc_deferred_rt_args.cs.hlsl",
        ),
        ("Water deferred rt", "/water/deferred_rt.cs.hlsl"),
    ],
    resources: &["water_rt_count_buffer", "water_rt_ray_buffer"],
    ops: WATER_OPS,
};

/// W45 冰（O：0x1409c08d0）。
pub const ICE_PLAN: SurfacePlan = SurfacePlan {
    kind: WaterSurfaceKind::Ice,
    raster_pass: "Ice plane",
    compute_passes: &[
        ("Ice refl march", "/water/ice_refl_march.cs.hlsl"),
        ("Ice refl trace", "/water/ice_deferred_rt.cs.hlsl"),
        ("Ice refr resolve", "/water/ice_refr_resolve.cs.hlsl"),
        ("Ice refl resolve", "/water/ice_refl_resolve.cs.hlsl"),
    ],
    resources: &[
        "ice roughness tex",
        "ice refr hits",
        "ice refr hit radiance",
        "ice_refl_count_buffer",
        "ice_rt_count_buffer",
        "ice_refl_px_buffer",
        "ice_rt_ray_buffer",
        "water refl hit radiance",
        "water refl hits",
        "water refr temporal",
    ],
    ops: ICE_OPS,
};

/// W45 水表面的黄金计划（真实实现见 [`water_surface_pass`]）。
pub fn water_surface_plan() -> &'static SurfacePlan {
    &WATER_PLAN
}

/// W45 冰表面的黄金计划（真实实现见 [`ice_surface_pass`]）。
pub fn ice_surface_plan() -> &'static SurfacePlan {
    &ICE_PLAN
}

/// 按 `GladeType` 选择表面黄金计划（`select_water_surface` + 对应 pass）。
pub fn surface_plan_for(glade: GladeType) -> &'static SurfacePlan {
    match select_water_surface(glade) {
        WaterSurfaceKind::Water => water_surface_plan(),
        WaterSurfaceKind::Ice => ice_surface_plan(),
    }
}

#[cfg(test)]
mod gap_tests {
    use super::*;

    #[test]
    fn ray_buffer_size_follows_the_32bit_formula() {
        assert_eq!(
            water_ray_buffer_size(1920, 1080),
            (2_073_600u64 + 2_073_600 / 16) << 4
        );
        assert_eq!(water_ray_buffer_size(0, 5), 0);
        // 32 位回绕：65536 * 65536 = 0（imul eax, ecx）。
        assert_eq!(water_ray_buffer_size(65536, 65536), 0);
        assert_eq!(water_ray_buffer_size(16, 1), 17 << 4);
    }

    #[test]
    fn names_and_lengths_match_the_string_pool() {
        assert_eq!(WATER_COUNT_BUFFER_NAME.len(), 0x15);
        assert_eq!(WATER_RAY_BUFFER_NAME.len(), 0x13);
        assert_eq!(WATER_PASS_NAME.len(), 0xb);
        assert_eq!(WATER_ARGS_PASS_NAME.len(), 0x16);
        assert_eq!(WATER_RT_PASS_NAME.len(), 0x11);
        assert_eq!(ICE_PASS_NAME.len(), 9);
        assert_eq!(SURFACE_LIST, 15);
        assert_eq!(ICE_COMPUTE_PASSES.len(), 4);
    }
}
