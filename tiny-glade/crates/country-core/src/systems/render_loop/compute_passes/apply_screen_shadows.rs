//! `apply_screen_shadows.rs` — W42 `apply_screen_shadows`（O 0x140a39e70，2163 B）、
//! W47 `compute_shadow_mask_mean`（O 0x140a3a6f0，497 B）。
//!
//! ## apply_screen_shadows（参数 O：`(sret, ctx, world, inputs, 栈上第 5 个句柄)`，返回当前帧阴影遮罩句柄）
//! 1. `format = R8_UNORM(9)`、`extent = inputs.shadow_source.desc.extent_2d()`：
//!    `get_or_create_temporal_pair(ctx, "Screen shadows temporal", new_2d(extent, 9).sampled().storage())`；
//!    `Err` → `Result::unwrap` panic（"called `Result::unwrap()` on an `Err` value"）。
//! 2. `ComputePass::new("Apply screen shadows", "/apply_screen_shadows.cs.hlsl")`，依次：
//!    - `write_uniform<i32>`（`history_valid`，值 = 临时纹理对结果里的 1 字节 `flag`）；
//!    - 读（0x19）`inputs.a` → `gbuffer_tex`；`inputs.b` → `depth_tex`；
//!      **上一帧** 遮罩（`ReadOnlyHandle`）→ `history_tex`；`inputs.c`（栈参数）→ `reprojection_tex`；
//!    - `write_uniform<bool>`（`bool_render_shadows`，值 true）；读 preset 0x11 → `shadow_map`；
//!    - 写（0x32）**当前帧** 遮罩 → `output_tex`；`write_storage_buffer<ShadowMaskHistogramSsbo>`(`shadow_mask_histogram`)；
//!    - `dispatch_2d(cur.w, cur.h)`（模式 1，z=1，extent 取自当前遮罩 desc）。
//! 3. 返回当前遮罩（O：`[sret+0x20] = id`）。
//!
//! ## compute_shadow_mask_mean（参数 O：`(ctx, world)`）
//! `ComputePass::new("Shadow mask mean", "/shadow_mask_mean.cs.hlsl")`：`write_storage_buffer<ShadowMaskHistogramSsbo>`(`shadow_mask_histogram`)、
//! `write_storage_buffer<ShadowMaskMeanSsbo>`(`shadow_mask_mean`)、`dispatch_1d(256)`（O：常量 `{1, 0x100, 1, 1}`）。
//!
//! SSBO 名（O：SsboId 与 xxh3_64 比对一致）："ShadowMaskHistogramSsbo" / "ShadowMaskMeanSsbo"。

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::pass_resource_access::RgTexture;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{StorageElement, TypedStorageBuffer};
use crate::resources::texture_presets::TexturePreset;
use crate::rhapsody_types::TextureDesc;
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, TEX_READ, TEX_WRITE, binding_hash,
};

/// 屏幕阴影直方图（O：`apply_screen_shadows.cs` 的 `RWStructuredBuffer<uint>`，`InterlockedAdd(hist[clamp(.., 0, 255)], 1)`）。
///
/// 机器码（O，`new_storage_buffer<ShadowMaskHistogramSsbo>` 0x141612170）：`BufferDesc::new(0x400, 0x22, 1).alignment(4)`，
/// 元素 4（`cmp rbx, 0x404`）→ `BUFFER_LEN` = 256；`SsboId` 立即数 0xdec66ec37fabbb05 = xxh3("ShadowMaskHistogramSsbo")。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ShadowMaskHistogramSsbo(pub u32);
/// 屏幕阴影均值（O：`shadow_mask_mean.cs` 的 `RWByteAddressBuffer`，`Load<float>(0)`；`final_post` 读它缩放曝光）。
///
/// 机器码（O，0x141613720）：`BufferDesc::new(4, 0x22, 1).alignment(4)`——**`BUFFER_LEN == 1` 也是 0x22**（不是 uniform 0x12），
/// 元素 4（`cmp r14, 8`）；`SsboId` 立即数 0x480eafc7a5a8a812 = xxh3("ShadowMaskMeanSsbo")。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ShadowMaskMeanSsbo(pub f32);

impl StorageElement for ShadowMaskHistogramSsbo {
    const NAME: &'static str = "ShadowMaskHistogramSsbo";
    /// 0x400 / 4。
    const BUFFER_LEN: usize = 256;
}
impl StorageElement for ShadowMaskMeanSsbo {
    const NAME: &'static str = "ShadowMaskMeanSsbo";
    const BUFFER_LEN: usize = 1;
}
impl LibrarySsbo for ShadowMaskHistogramSsbo {
    type Storage = TypedStorageBuffer<ShadowMaskHistogramSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        TypedStorageBuffer::<ShadowMaskHistogramSsbo>::new_storage_buffer(device, initial)
    }
}
impl LibrarySsbo for ShadowMaskMeanSsbo {
    type Storage = TypedStorageBuffer<ShadowMaskMeanSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        TypedStorageBuffer::<ShadowMaskMeanSsbo>::new_storage_buffer(device, initial)
    }
}

pub const APPLY_NAME: &str = "Apply screen shadows";
pub const APPLY_URI: &str = "/apply_screen_shadows.cs.hlsl";
pub const MEAN_NAME: &str = "Shadow mask mean";
pub const MEAN_URI: &str = "/shadow_mask_mean.cs.hlsl";
pub const TEMPORAL_NAME: &str = "Screen shadows temporal";
/// `ash::vk::Format::R8_UNORM`（O：`new_2d(.., 9)`）。
pub const MASK_FORMAT: u32 = 9;
pub const MEAN_THREADS: u32 = 0x100;

pub const KEY_FLAG_I32: u64 = binding_hash("history_valid"); // 0xa9a109b760eed902
pub const KEY_BOOL: u64 = binding_hash("bool_render_shadows"); // 0x418a98441f59a54c
pub const HASH_IN_A: u64 = binding_hash("gbuffer_tex"); // 0x41aa4a0d2dea0b36
pub const HASH_IN_B: u64 = binding_hash("depth_tex"); // 0x4861579da049fbfc
pub const HASH_PREVIOUS: u64 = binding_hash("history_tex"); // 0x185a603c077dad19
pub const HASH_IN_C: u64 = binding_hash("reprojection_tex"); // 0xb8a74ccd835a6be3
pub const HASH_PRESET_11: u64 = binding_hash("shadow_map"); // 0x11651f4dc9841aa4
pub const HASH_CURRENT: u64 = binding_hash("output_tex"); // 0x88219a3ae69892b2
pub const HASH_HISTOGRAM: u64 = binding_hash("shadow_mask_histogram"); // 0x05ce23513a1cf9ff
pub const HASH_MEAN: u64 = binding_hash("shadow_mask_mean"); // 0xf7bf1be67867176a

/// `apply_screen_shadows` 的纹理输入（O：`inputs` 指针指向的两个相邻句柄 + 栈参数句柄；语义 U）。
#[derive(Clone, Copy, Debug)]
pub struct ScreenShadowInputs<'a> {
    /// 句柄 @+0x00 → 0x41aa4a0d2dea0b36。
    pub a: &'a RgTexture,
    /// 句柄 @+0x24 → 0x4861579da049fbfc；其 desc 决定遮罩尺寸。
    pub b: &'a RgTexture,
    /// 栈上第 5 个参数 → 0xb8a74ccd835a6be3。
    pub c: &'a RgTexture,
}

/// W42（O 0x140a39e70）。返回当前帧遮罩。
pub fn apply_screen_shadows(env: &mut PassEnv<'_>, inputs: &ScreenShadowInputs<'_>) -> RgTexture {
    let extent = [inputs.b.desc.extent[0], inputs.b.desc.extent[1]];
    // O：`get_or_create_temporal_pair(name, desc.sampled().storage())`，desc = `new_2d(extent, R8_UNORM).mip_levels(1)`。
    let desc = TextureDesc::new_2d(extent, MASK_FORMAT)
        .mip_levels(1)
        .sampled()
        .storage();
    let pair = env.temporal_pair(TEMPORAL_NAME, desc);
    let (cur, prev) = (pair.current, pair.previous);
    let recorded = env
        .pass(APPLY_NAME, APPLY_URI)
        .uniform(KEY_FLAG_I32, i32::from(pair.fresh))
        .bind_texture(HASH_IN_A, &inputs.a, TEX_READ)
        .bind_texture(HASH_IN_B, &inputs.b, TEX_READ)
        .bind_texture(HASH_PREVIOUS, &&prev, TEX_READ)
        .bind_texture(HASH_IN_C, &inputs.c, TEX_READ)
        .uniform(KEY_BOOL, true)
        .bind_texture(HASH_PRESET_11, &TexturePreset(0x11), TEX_READ)
        .bind_texture(HASH_CURRENT, &&cur, TEX_WRITE)
        .write_storage_buffer::<ShadowMaskHistogramSsbo>(HASH_HISTOGRAM)
        .dispatch_2d(cur.desc.extent[0], cur.desc.extent[1]);
    env.record(recorded);
    cur
}

/// W47（O 0x140a3a6f0）。
pub fn compute_shadow_mask_mean(env: &mut PassEnv<'_>) {
    let recorded = env
        .pass(MEAN_NAME, MEAN_URI)
        .write_storage_buffer::<ShadowMaskHistogramSsbo>(HASH_HISTOGRAM)
        .write_storage_buffer::<ShadowMaskMeanSsbo>(HASH_MEAN)
        .dispatch_1d(MEAN_THREADS);
    env.record(recorded);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::render::pass_builder::pass_resource_access::PassBinding;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    fn tex(resource: u32, w: u32, h: u32) -> RgTexture {
        RgTexture {
            resource,
            desc: TextureDesc::new([w, h, 1], 0, 0),
        }
    }
    fn world() -> TestWorld {
        let hashes = [
            HASH_IN_A,
            HASH_IN_B,
            HASH_PREVIOUS,
            HASH_IN_C,
            HASH_PRESET_11,
            HASH_CURRENT,
            HASH_HISTOGRAM,
            HASH_MEAN,
        ];
        TestWorld::new(
            Reflection::new()
                .bind(&hashes)
                .uniforms(&[KEY_FLAG_I32, KEY_BOOL]),
        )
        .with_ssbo::<ShadowMaskHistogramSsbo>()
        .with_ssbo::<ShadowMaskMeanSsbo>()
    }

    #[test]
    fn apply_binds_in_machine_code_order_and_returns_current() {
        let w = world();
        let mut f = TestFrame {
            pair_fresh: true,
            ..Default::default()
        };
        let (a, b, c) = (tex(1, 1920, 1080), tex(2, 960, 540), tex(3, 8, 8));
        let cur = with_env(&w, &mut f, |env| {
            apply_screen_shadows(
                env,
                &ScreenShadowInputs {
                    a: &a,
                    b: &b,
                    c: &c,
                },
            )
        });
        // 临时纹理对：名字、格式 R8_UNORM、尺寸取 b。
        assert_eq!(
            f.temporal_pairs,
            [(
                "Screen shadows temporal".to_string(),
                TextureDesc::new_2d([960, 540], 9)
                    .mip_levels(1)
                    .sampled()
                    .storage()
            )]
        );
        let p = &f.passes[0];
        assert_eq!(p.name, "Apply screen shadows");
        assert_eq!(
            w.backend.paths.borrow()[0],
            "/apply_screen_shadows.cs.hlsl:cs_main"
        );
        assert!(matches!(
            p.dispatch,
            DispatchMode::Threads {
                x: 960,
                y: 540,
                z: 1
            }
        ));
        assert_eq!(cur.desc.extent, [960, 540, 1]);
        // 访问：color library、4 个读、preset 读、当前写(0x32)、直方图写(0x32)。
        let acc: Vec<u8> = p.builder.accesses.iter().map(|a| a.access).collect();
        assert_eq!(acc, [0x1a, 0x19, 0x19, 0x19, 0x19, 0x19, 0x32, 0x32]);
        // 资源号：a=1, b=2, previous, c=3, preset, current。
        let res: Vec<u32> = p.builder.accesses.iter().map(|a| a.resource).collect();
        assert_eq!(&res[1..3], &[1, 2]);
        assert_eq!(res[3], 102, "previous（pair：当前 101，上一帧 102）");
        assert_eq!(res[4], 3);
        assert_eq!(res[6], cur.resource);
        assert!(matches!(
            p.bindings[6],
            PassBinding::ImageView { write: true, .. }
        ));
        // uniform：flag i32 @0，bool @16。
        assert_eq!(uniform_u32(p, 0), 1);
        assert_eq!(uniform_u32(p, 16), 1);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn temporal_pair_failure_unwraps() {
        let w = world();
        let mut f = TestFrame {
            fail_temporal: true,
            ..Default::default()
        };
        let (a, b, c) = (tex(1, 4, 4), tex(2, 4, 4), tex(3, 4, 4));
        with_env(&w, &mut f, |env| {
            apply_screen_shadows(
                env,
                &ScreenShadowInputs {
                    a: &a,
                    b: &b,
                    c: &c,
                },
            )
        });
    }

    #[test]
    fn mean_pass_is_a_256_thread_1d_dispatch_over_two_ssbos() {
        let w = world();
        let mut f = TestFrame::default();
        with_env(&w, &mut f, compute_shadow_mask_mean);
        let p = &f.passes[0];
        assert_eq!(p.name, "Shadow mask mean");
        assert_eq!(
            w.backend.paths.borrow()[0],
            "/shadow_mask_mean.cs.hlsl:cs_main"
        );
        assert!(matches!(
            p.dispatch,
            DispatchMode::Threads { x: 256, y: 1, z: 1 }
        ));
        let acc: Vec<u8> = p.builder.accesses.iter().map(|a| a.access).collect();
        assert_eq!(acc, [0x1a, 0x32, 0x32]);
    }

    #[test]
    fn constants_match_machine_code() {
        assert_eq!(KEY_FLAG_I32, (-0x565ef6489f1126fe_i64) as u64);
        assert_eq!(HASH_IN_C, (-0x4758b3327ca5941d_i64) as u64);
        assert_eq!(HASH_CURRENT, (-0x77de65c519676d4e_i64) as u64);
        assert_eq!(HASH_MEAN, (-0x840e4198798e896_i64) as u64);
        assert_eq!(HASH_IN_A, 0x41aa4a0d2dea0b36);
        assert_eq!(HASH_IN_B, 0x4861579da049fbfc);
        assert_eq!(HASH_PREVIOUS, 0x185a603c077dad19);
        assert_eq!(HASH_HISTOGRAM, 0x05ce23513a1cf9ff);
        assert_eq!(KEY_BOOL, 0x418a98441f59a54c);
        assert_eq!(HASH_PRESET_11, 0x11651f4dc9841aa4);
        assert_eq!(APPLY_NAME.len(), 0x14);
        assert_eq!(APPLY_URI.len(), 0x1d);
        assert_eq!(MEAN_NAME.len(), 0x10);
        assert_eq!(MEAN_URI.len(), 0x19);
        assert_eq!(TEMPORAL_NAME.len(), 0x17);
    }
}

#[cfg(test)]
mod ssbo_layout_tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::render::ssbo::tests::MockDevice;
    use core::mem::{align_of, size_of};
    use xxhash_rust::xxh3::xxh3_64;

    #[test]
    fn element_sizes_and_buffer_lens_match_buffer_desc() {
        assert_eq!(
            (
                size_of::<ShadowMaskHistogramSsbo>(),
                align_of::<ShadowMaskHistogramSsbo>()
            ),
            (4, 4)
        );
        assert_eq!(
            (
                size_of::<ShadowMaskMeanSsbo>(),
                align_of::<ShadowMaskMeanSsbo>()
            ),
            (4, 4)
        );
        // 256 个 u32 = 0x400 B（`InterlockedAdd(hist[clamp(.., 0, 255)], 1)`）。
        assert_eq!(ShadowMaskHistogramSsbo::BUFFER_LEN * 4, 0x400);
        assert_eq!(ShadowMaskMeanSsbo::BUFFER_LEN * 4, 4);
    }

    #[test]
    fn ssbo_ids_are_the_exe_immediates() {
        assert_eq!(
            xxh3_64(ShadowMaskHistogramSsbo::NAME.as_bytes()),
            0xdec6_6ec3_7fab_bb05
        );
        assert_eq!(
            xxh3_64(ShadowMaskHistogramSsbo::NAME.as_bytes()),
            (-0x2139_913c_8054_44fb_i64) as u64
        );
        assert_eq!(
            xxh3_64(ShadowMaskMeanSsbo::NAME.as_bytes()),
            0x480e_afc7_a5a8_a812
        );
    }

    #[test]
    fn buffers_are_storage_0x22_even_for_buffer_len_1() {
        let dev = MockDevice::default();
        let lib = AssetSsboLibrary::new();
        lib.add_new::<ShadowMaskHistogramSsbo>(&dev);
        lib.add_new::<ShadowMaskMeanSsbo>(&dev);
        let created = dev.created.borrow();
        let hist = &created[0].0;
        let mean = &created[1].0;
        assert_eq!(
            (hist.size, hist.usage, hist.memory, hist.alignment),
            (0x400, 0x22, 1, 4)
        );
        // `TypedStorageBuffer::new` 对 BUFFER_LEN == 1 会用 uniform 0x12；`new_storage_buffer<ShadowMaskMeanSsbo>` 是 0x22。
        assert_eq!(
            (mean.size, mean.usage, mean.memory, mean.alignment),
            (4, 0x22, 1, 4)
        );
        assert_eq!(created[0].1, "ShadowMaskHistogramSsbo");
        assert_eq!(created[1].1, "ShadowMaskMeanSsbo");
        assert!(created[0].2.is_none() && created[1].2.is_none());
    }
}
