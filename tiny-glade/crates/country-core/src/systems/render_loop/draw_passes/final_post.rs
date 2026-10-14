//! `final_post.rs` — 最终后处理 / 解析到交换链（B_d_gap：`final_post` 资源句柄接线完整恢复）。
//!
//! - `resolve_to_swapchain`（0x14090d850，335 B）：完整恢复（见下）。
//! - `final_post`（0x14090d000，2119 B）+ 其 `raster_single_object` 实例（`final_post::closure_env$1`，0x1409ce780，6417 B）：
//!   **完整恢复**——`final_post` 把输入打包成 0x98 B 的闭包环境 [`FinalPostEnv`]（偏移即机器码里 `rbp-0x40 + k`），
//!   闭包按固定序列读环境字段做绑定（见 [`final_blit_body`]）。
//!
//! # `resolve_to_swapchain`（O）
//! 参数：`r8` = 源纹理 `&Texture`（`+0x21` = `sample_count`），`r9` = 交换链纹理（`extent_2d(r9+4)` 作视口），
//! 第 5 个（栈）参数 = 一个字节 `flag`。`name = sample_count != 1 ? "raster_resolve_msaa_to_swapchain"(32) : "raster_resolve_to_swapchain"(27)`；
//! `Res<AssetShaderLibrary>::get_raster_handle_by_name(name)`（未命中 `expect`）；
//! `raster_single_object::<1>("Resolve to swapchain"(0x14), [tag0 = 交换链], shader, 闭包)`；闭包（0x1409d00a0）：
//! `bind_texture_to_draw_builder(&Handle<Texture> = 源, 0x11ba5697cc40b3dd, 0x1d, 无 view)` → `write_uniform<u32>(0x7b01c6bb382d8849, flag)`；
//! 随后 `draw_no_vao(3)` + `render`（raster_single 公共骨架）。
//!
//! # `final_post` 参数（O：Win64；栈参数 `[rbp + 0x1b0 + 8*i]`，`rbp = rsp + 0x80`，`sub rsp, 0x1c8`）
//! | 位置 | 内容 | 用途 |
//! |---|---|---|
//! | `rcx` | 返回槽（0x24 B `Handle<Texture>`）| 写回 "msaa color out" 的描述 + 句柄（`[rsi+0..0x24]`）|
//! | `rdx` | `&mut RenderGraph` | `create_raw_resource("msaa color out"(0xe), desc)` |
//! | `r8` | `&World` | 资源读取（见下）|
//! | `r9` | 目标尺寸（低 32 位宽、高 32 位高）| `TextureDesc::new_2d(extent, 0x2b).sample_count(4)` |
//! | `[0x1b0]` | `&Handle<Texture>` src0 | 环境 `+0x30`；其宽 / 目标宽 = 缩放比 |
//! | `[0x1b8]` | `&Handle<Texture>` src1 | 环境 `+0x54` |
//! | `[0x1c0]` | `u8` | 环境 `+0x94` |
//! | `[0x1c8]` | `&Option<LutOverride>`（`i64::MIN` niche，见 [`LutSelect`]）| 环境 `+0x00..0x18`、`+0x7c`、`+0x80`、`+0x96` |
//! | `[0x1d0]` / `[0x1d8]` | `f32` | 环境 `+0x90` / `+0x8c` |
//!
//! 资源（`Components::get_resource_id`，O）：`0x7e3bf54835a54748/0x56b9895f6e41fabb` → `Option<Handle<RasterShaderProgram>>`
//! （`test byte [p],1` 否则 `Option::unwrap` panic；`[p+8]` = 着色器句柄，作为 `raster_single_object` 的着色器）；
//! `0x4a142e09642375b9/0x43f6154cfa8c847f` → 当前档 `cfg = res + (u8@+0xc0)*0x40`（`+0x28` 字节 = 命名标志，`+0` String 名，
//! `+0x20` 两个 f32；`+0x30/+0x34/+0x38` 三个 f32 在闭包里读）；`0xc2b8e941c91a6428/0x59f3c6d1d3da43a0` = `GladeTheme`（首 u32）；
//! `0x5f06b298510fb768/0x054e14bacecb858d` → 自定义 LUT：`[+0] == i64::MIN`（None niche）→ 空，否则 `+0x18` 处的纹理引用 → 环境 `+0x28`。
//!
//! # 缩放比分类（O：0x14090d462..0x14090d4a6）
//! `ratio = src0.width as f32 / out_width as f32`：`ratio < 0.95` → 4；`0.95 <= ratio < 1.05` → 1；`ratio >= 1.05` → 2
//! （环境 `+0x78` u32 → uniform `0x4655…`）。
//!
//! # 偏差（I）
//! `RenderGraph::create_raw_resource` 在 A4 里没有 API（`PassHost` 无该方法），由调用方给出已创建的 "msaa color out" 纹理
//! （[`msaa_color_desc`] 给出描述）；`ColorAttachment::Tag2` 已由 i_api 提供（O：`mov dword [..], 2`）。

use alloc::string::String;

use crate::name_hash::NameHash;
use crate::render::pass_builder::draw_builder::{ACCESS_SAMPLED_IMAGE, BindTexture, DrawBuilder};
use crate::render::pass_builder::framebuffer_binding::{AttachmentTexture, ColorAttachment};
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::render::raster_single::raster_single_object;
use crate::render::shader::RasterShaderProgram;
use crate::resources::texture_presets::TextureFromDisk;
use crate::rhapsody_contract::{Handle, ManagedHandle};
use crate::systems::render_loop::draw_passes::d_main_support::{DrawPassCtx, UNWRAP_NONE};

pub const RESOLVE_PASS_NAME: &str = "Resolve to swapchain";
pub const RESOLVE_SHADER: &str = "raster_resolve_to_swapchain";
pub const RESOLVE_MSAA_SHADER: &str = "raster_resolve_msaa_to_swapchain";
pub const RESOLVE_SOURCE_BINDING: u64 = 0x11ba_5697_cc40_b3dd;
pub const RESOLVE_FLAG_UNIFORM: u64 = 0x7b01_c6bb_382d_8849;
pub const FINAL_BLIT_PASS_NAME: &str = "Final blit";
pub const MSAA_COLOR_OUT_NAME: &str = "msaa color out";
/// `TextureDesc::new_2d` 的格式参数（O：`mov r8d, 0x2b`）与 `sample_count(4)`（`mov r8b, 4`）。
pub const MSAA_COLOR_FORMAT: u32 = 0x2b;
pub const MSAA_SAMPLE_COUNT: u8 = 4;
/// 缩放比阈值（O：`.rdata` f32 0x3f866666 / 0x3f733333）。
pub const EXPOSURE_UP: f32 = 1.05;
pub const EXPOSURE_DOWN: f32 = 0.95;

// ---- 闭包里的绑定 / uniform 名哈希（O：`movabs r8`；负立即数已转无符号）----
pub const B_SRC0: u64 = 0x11ba_5697_cc40_b3dd;
pub const B_SRC1: u64 = 0xabc5_542c_e079_e241; // -0x543aabd31f861dbf
pub const B_DISK: u64 = 0x815b_f503_6966_978c; // -0x7ea40afc96996874
pub const DISK_FILE: u64 = 0x579f_39ab_91a0_1865;
pub const U_FLAG94: u64 = 0x0d95_04ec_f7d9_5823;
pub const U_SCALE: u64 = 0x4655_303d_1f6e_22d5;
pub const U_BLEND: u64 = 0x988a_abd8_c91c_f129; // -0x6775542736e30ed7
pub const U_MODE: u64 = 0x225a_a70a_248d_92bb;
pub const U_GRADING_A: u64 = 0x1490_b7b3_33b4_194a;
pub const U_ARG_F1: u64 = 0x9348_3428_0264_e6e5; // -0x6cb7cbd7fd9b191b
pub const U_GRADING_B: u64 = 0x78c9_e92e_56b2_bf10;
pub const U_CFG_30: u64 = 0x8a73_7337_db9d_d3a8; // -0x758c8cc824622c58
pub const U_ARG_F0: u64 = 0xa902_d50b_6752_1d49; // -0x56fd2af498ade2b7
pub const U_CFG_38: u64 = 0x1de6_d1ab_b825_2ddb;
pub const U_CFG_34: u64 = 0x8327_a369_02ae_7c8e; // -0x7cd85c96fd518372
pub const U_DIMS: u64 = 0x531b_b931_8eea_4971;
pub const B_LUT_BY_NAME: u64 = 0x0422_b14c_27be_4803;
pub const B_LUT_FIXED: u64 = 0x0465_daf5_0c6e_be10;
/// `-0x443d203b397dbdaf`：固定的 `TextureFromDisk` 文件键。
pub const LUT_FIXED_FILE: u64 = 0xbbc2_dfc4_c682_4251;
pub const S_SHADOW_MASK_MEAN: u64 = 0xf7bf_1be6_7867_176a; // 绑定名 hash("shadow_mask_mean")
pub const B_LUT_CUSTOM: u64 = 0x15b8_3356_dc5c_e998;
pub const U_LUT_FLAG: u64 = 0x1ec3_8fd9_a628_a516;
/// LUT 资产名前缀（O：`grading_lut:{}`，grading_lut.rs 的 `format!`）。
pub const LUT_ASSET_PREFIX: &str = "grading_lut:";
/// 缺省 LUT 名（O：7 字节 `default`）。
pub const DEFAULT_LUT_NAME: &str = "default";
/// 闭包里 12 次 `write_uniform`（不含 `optional_uniform`）的顺序（O）。
pub const FINAL_POST_UNIFORMS: [u64; 12] = [
    U_FLAG94,
    U_SCALE,
    U_BLEND,
    U_MODE,
    U_GRADING_A,
    U_ARG_F1,
    U_GRADING_B,
    U_CFG_30,
    U_ARG_F0,
    U_CFG_38,
    U_CFG_34,
    U_LUT_FLAG,
];

/// `ShadowMaskMeanSsbo`（`apply_screen_shadows`）作为 `read_storage_buffer` 的载体；`rg_buffer_hash` = `get_rg_buffer<T>`（0x1412cab80）立即数。
pub struct ShadowMaskMeanBuffer;
impl SsboType for ShadowMaskMeanBuffer {
    fn rg_buffer_hash() -> u64 {
        0x480e_afc7_a5a8_a812
    }
}

/// 当前档的三个 f32（`cfg + 0x30/0x34/0x38`，语义 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GradingFloats {
    pub f30: f32,
    pub f34: f32,
    pub f38: f32,
}

/// `[rbp+0x1c8]` 指向的 LUT 选择（O：`Option` niche = `i64::MIN`，`+8` 字节 == 1 的变体，其余）。
#[derive(Clone, Debug, PartialEq)]
pub enum LutSelect {
    /// `[+0] != i64::MIN`：自定义 `(name: String @ +0/+8/+0x10, blend: f32 @ +0x18, flag: u8 @ +0x1c)`；模式 0。
    Custom { name: String, blend: f32, flag: u8 },
    /// niche 且 `byte [+8] == 1`：模式 1，`blend = 0`，`flag = 0`，沿用当前档配置。
    Keep,
    /// niche 且 `byte [+8] != 1`：模式 3；当前档名被替换为 `"default"`，`grading` 对重置为 `(1, 1)`。
    Default,
}

/// 当前档（`cfg`）被读取的字段。
#[derive(Clone, Debug, PartialEq)]
pub struct GradingConfig {
    /// `+0x28`。
    pub named: bool,
    /// `+0x20` 两个 f32（仅 `named` 时读取，否则 `(1, 1)`）。
    pub pair: [f32; 2],
    pub floats: GradingFloats,
}

/// `final_post` 闭包环境（0x98 B；偏移 = `rbp - 0x40 + k`，闭包里 = `rbp + 0xb80 + k`）。
#[derive(Clone, Debug)]
pub struct FinalPostEnv {
    /// `+0x00..0x18`：自定义 LUT 名（`None` = niche → `"default"`）。
    pub lut_name: Option<String>,
    /// `+0x18`：当前档（读 `+0x30/+0x34/+0x38`）。
    pub cfg: GradingFloats,
    /// `+0x20`：目标尺寸 `[w, h]`（`optional_uniform<[u32;2]>`）。
    pub extent: [u32; 2],
    /// `+0x28`：自定义 LUT 纹理（`Some` → 原生纹理绑定；`None` → 缺省 LUT 资产）。
    pub lut_texture: Option<(ManagedHandle, AttachmentTexture)>,
    /// `+0x30`：src0（0x24 B）。
    pub src0: AttachmentTexture,
    /// `+0x54`：src1。
    pub src1: AttachmentTexture,
    /// `+0x78`：缩放比分类（1/2/4）。
    pub scale_class: u32,
    /// `+0x7c`：混合系数。
    pub blend: f32,
    /// `+0x80`：LUT 选择模式（0/1/3）。
    pub mode: u32,
    /// `+0x84` / `+0x88`：分级系数对。
    pub grading: [f32; 2],
    /// `+0x8c` / `+0x90`：栈参数 `[0x1d8]` / `[0x1d0]`。
    pub arg_f1: f32,
    pub arg_f0: f32,
    /// `+0x94`：栈参数 `[0x1c0]`。
    pub flag94: u8,
    /// `+0x95`：`GladeTheme == 0x13`。
    pub theme_is_19: bool,
    /// `+0x96`：`Custom.flag`（其它变体 0）。
    pub lut_flag: u8,
}

/// 缩放比分类（O：0x14090d462..0x14090d4a6）。
pub fn scale_class(src_width: u32, out_width: u32) -> u32 {
    let ratio = src_width as f32 / out_width as f32;
    if EXPOSURE_DOWN <= ratio {
        if EXPOSURE_UP <= ratio { 2 } else { 1 }
    } else {
        4
    }
}

/// "msaa color out" 的描述：`new_2d(extent, 0x2b).sample_count(4)`。
pub fn msaa_color_desc(extent: (u32, u32), id: u32) -> AttachmentTexture {
    AttachmentTexture {
        id,
        extent,
        format: MSAA_COLOR_FORMAT,
        sample_count: MSAA_SAMPLE_COUNT,
    }
}

/// `final_post` 的输入（已解析的参数与资源，见模块头）。
pub struct FinalPostArgs {
    pub out_extent: (u32, u32),
    pub src0: AttachmentTexture,
    pub src1: AttachmentTexture,
    pub flag94: u8,
    pub lut: LutSelect,
    pub arg_f0: f32,
    pub arg_f1: f32,
    /// 资源 `0x7e3bf548…`：`None` → `Option::unwrap` panic。
    pub shader: Option<Handle<RasterShaderProgram>>,
    pub cfg: GradingConfig,
    pub theme: u32,
    pub custom_lut: Option<(ManagedHandle, AttachmentTexture)>,
}

/// 打包环境（O：0x14090d462..0x14090d567）。
pub fn build_env(a: &FinalPostArgs) -> FinalPostEnv {
    let (lut_name, blend, mode, lut_flag) = match &a.lut {
        LutSelect::Custom { name, blend, flag } => (Some(name.clone()), *blend, 0, *flag),
        LutSelect::Keep => (None, 0.0, 1, 0),
        LutSelect::Default => (None, 0.0, 3, 0),
    };
    let grading = if matches!(a.lut, LutSelect::Default) || !a.cfg.named {
        [1.0, 1.0]
    } else {
        a.cfg.pair
    };
    FinalPostEnv {
        lut_name,
        cfg: a.cfg.floats,
        extent: [a.out_extent.0, a.out_extent.1],
        lut_texture: a.custom_lut.clone(),
        src0: a.src0,
        src1: a.src1,
        scale_class: scale_class(a.src0.extent.0, a.out_extent.0),
        blend,
        mode,
        grading,
        arg_f1: a.arg_f1,
        arg_f0: a.arg_f0,
        flag94: a.flag94,
        theme_is_19: a.theme == 0x13,
        lut_flag,
    }
}

/// `grading_lut:{name}` 的 `NameHash`（O：`format_inner` + `NameHash::compute`）。
pub fn lut_asset_hash(name: &str) -> u64 {
    NameHash::compute(&alloc::format!("{LUT_ASSET_PREFIX}{name}")).0
}

/// `raster_single_object` 闭包本体（O：0x1409cec1e..0x1409cf44b，逐调用）。
pub fn final_blit_body<'p>(b: DrawBuilder<'p>, env: &FinalPostEnv) -> DrawBuilder<'p> {
    let mut b = b
        .bind_texture_to_draw_builder(
            BindTexture::RgHandle(&env.src0),
            B_SRC0,
            ACCESS_SAMPLED_IMAGE,
            None,
        )
        .bind_texture_to_draw_builder(
            BindTexture::RgHandle(&env.src1),
            B_SRC1,
            ACCESS_SAMPLED_IMAGE,
            None,
        )
        .bind_texture_to_draw_builder(
            BindTexture::FromDisk(TextureFromDisk(DISK_FILE)),
            B_DISK,
            ACCESS_SAMPLED_IMAGE,
            None,
        )
        .uniform(U_FLAG94, env.flag94 as u32)
        .uniform(U_SCALE, env.scale_class)
        .uniform(U_BLEND, env.blend)
        .uniform(U_MODE, env.mode)
        .uniform(U_GRADING_A, env.grading[0])
        .uniform(U_ARG_F1, env.arg_f1)
        .uniform(U_GRADING_B, env.grading[1])
        .uniform(U_CFG_30, env.cfg.f30)
        .uniform(U_ARG_F0, env.arg_f0)
        .uniform(U_CFG_38, if env.theme_is_19 { env.cfg.f38 } else { 0.0 })
        .uniform(U_CFG_34, env.cfg.f34)
        .optional_uniform(U_DIMS, env.extent);
    let name = env.lut_name.as_deref().unwrap_or(DEFAULT_LUT_NAME);
    b = b
        .bind_texture_to_draw_builder(
            BindTexture::FromDisk(TextureFromDisk(lut_asset_hash(name))),
            B_LUT_BY_NAME,
            ACCESS_SAMPLED_IMAGE,
            None,
        )
        .bind_texture_to_draw_builder(
            BindTexture::FromDisk(TextureFromDisk(LUT_FIXED_FILE)),
            B_LUT_FIXED,
            ACCESS_SAMPLED_IMAGE,
            None,
        )
        .read_storage_buffer::<ShadowMaskMeanBuffer>("ShadowMaskMeanSsbo", S_SHADOW_MASK_MEAN);
    b = match &env.lut_texture {
        Some((texture, desc)) => b.bind_texture_to_draw_builder(
            BindTexture::Raw { texture, desc },
            B_LUT_CUSTOM,
            ACCESS_SAMPLED_IMAGE,
            None,
        ),
        None => b.bind_texture_to_draw_builder(
            BindTexture::FromDisk(TextureFromDisk(lut_asset_hash(DEFAULT_LUT_NAME))),
            B_LUT_CUSTOM,
            ACCESS_SAMPLED_IMAGE,
            None,
        ),
    };
    b.uniform(U_LUT_FLAG, env.lut_flag as u32)
}

/// `final_post(rg, world, …)`：`msaa_out` 是调用方（`RenderGraph::create_raw_resource`）创建的 "msaa color out"。
pub fn final_post(ctx: &mut DrawPassCtx<'_>, a: &FinalPostArgs, msaa_out: &AttachmentTexture) {
    // O：`test byte [p], 1; je unwrap_failed`。
    let shader = a.shader.clone().expect(UNWRAP_NONE);
    let env = build_env(a);
    let host = ctx.host();
    raster_single_object::<1, _>(
        FINAL_BLIT_PASS_NAME,
        &[ColorAttachment::Tag2(*msaa_out)],
        shader,
        host,
        |b| final_blit_body(b, &env),
    );
}

/// 选择 resolve 着色器名（O：`sample_count != 1` → msaa）。
pub fn resolve_shader_name(source_sample_count: u8) -> &'static str {
    if source_sample_count != 1 {
        RESOLVE_MSAA_SHADER
    } else {
        RESOLVE_SHADER
    }
}

/// `resolve_to_swapchain(ctx, world, source, swapchain, flag)`。
pub fn resolve_to_swapchain(
    ctx: &mut DrawPassCtx<'_>,
    source: &AttachmentTexture,
    swapchain: &AttachmentTexture,
    flag: u8,
) {
    let name = resolve_shader_name(source.sample_count);
    let shader = ctx
        .shader(name)
        .expect("called `Option::expect()` on a `None` value");
    let host = ctx.host();
    raster_single_object::<1, _>(
        RESOLVE_PASS_NAME,
        &[ColorAttachment::Texture(*swapchain)],
        shader,
        host,
        |b| {
            b.bind_texture_to_draw_builder(
                BindTexture::RgHandle(source),
                RESOLVE_SOURCE_BINDING,
                ACCESS_SAMPLED_IMAGE,
                None,
            )
            .uniform(RESOLVE_FLAG_UNIFORM, flag as u32)
        },
    );
}

/// 闭包里 `write_uniform` 的次数（O：12）。
pub fn final_post_uniform_writes() -> usize {
    FINAL_POST_UNIFORMS.len()
}
