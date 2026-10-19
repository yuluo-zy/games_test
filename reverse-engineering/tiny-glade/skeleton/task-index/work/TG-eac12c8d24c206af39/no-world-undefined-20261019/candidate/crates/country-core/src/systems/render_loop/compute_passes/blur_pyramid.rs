//! `blur_pyramid.rs` — 可复用的模糊金字塔原语（非独立帧 pass）。
//! O：`blur_pyramid` 0x1409ee1e0（3623 B）、`bidirectional_blur_pyramid` 0x1409ed860（2423 B；内联了 `rev_blur_pyramid`，
//! PDB 里没有它的独立过程）。
//!
//! ## blur_pyramid（参数 O：`(sret, ctx, world, src: &Handle<Texture>)`；返回 "partial blur pyramid" 句柄）
//! 1. `desc = src.desc.half_res().format(B10G11R11_UFLOAT_PACK32 /*0x7a*/).storage().sampled()`；`m = desc.all_mip_levels()`；
//!    **`v = m - 1`（u8）；`mips = if v >= 2 { v } else { 1 }`**（O：`dec al; cmp al,2; mov ecx,1; cmovae ecx,eax`）。
//!    `create_raw_resource(desc{mip_levels = mips}, "partial blur pyramid")`（20 B）。
//! 2. pass "blur0"（`/blur_pyramid/blur.cs.hlsl`）：读 `src` → `input_tex`（0x19，无视图）；
//!    写 partial（视图 `base_mip_level(0).level_count(Some(1))`）→ `output_tex`（0x37）；
//!    `write_uniform<[u32;2]>`(`input_mip_dims`, = `src` 的 `extent_2d`)；`dispatch_2d(half.w, half.h)`。
//! 3. `for i in 1..mips`：pass **"blurN"**（字面 "blurN"，不是格式化出来的）：
//!    读 partial（视图 `usage(4).base_mip_level(i-1).level_count(Some(1))`）→ `input_tex`（0x19）；
//!    写 partial（视图 `usage(8).base_mip_level(i).level_count(Some(1))`）→ `output_tex`（0x37）；
//!    `input_mip_dims` = `extent_2d(half.div_extent((2^(i-1), 2^(i-1), 1)))`；`dispatch_2d(extent_2d(half.div_extent((2^i, 2^i, 1))))`。
//!    `div_extent`（O 0x140bd56a0）= 逐轴 **向下取整** 除并 `max(_, 1)`，除数为 0 → `panic_const_div_by_zero`。
//!
//! ## rev_blur_pyramid（`bidirectional_blur_pyramid` 的后半，参数 `(ctx, world, partial)`；返回 "full blur pyramid" 句柄）
//! 1. `full = create_raw_resource(partial.desc.sampled().storage(), "full blur pyramid")`（17 B），`N = full.mip_levels`。
//! 2. `for i in (0..N).rev()`（N-1 → 0）：`ext_i = extent_2d(full.desc.div_extent((2^i, 2^i, 1)))`；pass **"rev_blurN"**
//!    （`/blur_pyramid/rev_blur.cs.hlsl`）：
//!    - 读 partial 的 mip `i`（视图 `usage(4).base_mip_level(i).level_count(1)`）→ `input_tail_tex`（0x19）；
//!    - 读 full 的 mip `min(i+1, N-1)`（视图 `usage(4)…`）→ `input_tex`（0x19）；
//!    - 写 full 的 mip `i`（视图 `usage(8)…`）→ `output_tex`（0x37）；
//!    - `write_uniform<[u32;2]>`(`output_extent`, `ext_i`)、`write_uniform<bool>`(`sample_self`, `i + 1 != N`)；
//!    - `dispatch`：模式 1，`(ext_i.w, ext_i.h, depth)`（depth 恒 1，2D 纹理）。
//! 3. 返回 full。`bidirectional_blur_pyramid(src)` = `rev_blur_pyramid(blur_pyramid(src))`。
//!
//! 视图经 A4 `bind_texture_with_view` 进入 `PassBinding::ImageView.view`；A4 缺口：`dispatch_3d`（Threads 模式带 z）无公开构造，
//! 这里 z 恒 1（2D 纹理）。`TextureDesc::all_mip_levels` 为 rhapsody 内部（I：`floor(log2(max(w,h)))+1`）。

use crate::render::pass_builder::pass_resource_access::RgTexture;
use crate::rhapsody_types::{TextureDesc, TextureViewDesc};
use crate::systems::render_loop::compute_passes::generate_hzb::all_mip_levels;
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, TEX_READ, TEX_STORAGE_WRITE, binding_hash,
};

pub const PARTIAL_NAME: &str = "partial blur pyramid";
pub const FULL_NAME: &str = "full blur pyramid";
pub const BLUR_URI: &str = "/blur_pyramid/blur.cs.hlsl";
pub const REV_URI: &str = "/blur_pyramid/rev_blur.cs.hlsl";
pub const BLUR0_NAME: &str = "blur0";
/// 字面 "blurN"（O：`rodata`，不做格式化）。
pub const BLUR_N_NAME: &str = "blurN";
pub const REV_N_NAME: &str = "rev_blurN";
/// `ash::vk::Format::B10G11R11_UFLOAT_PACK32`（O：`format(0x7a)`）。
pub const PYRAMID_FORMAT: u32 = 0x7a;
/// `TextureViewDesc::usage` 的两个值（O：4 = 采样读，8 = 存储写）。
pub const USAGE_READ: u32 = 4;
pub const USAGE_WRITE: u32 = 8;

pub const HASH_INPUT_TEX: u64 = binding_hash("input_tex"); // 0x11ba5697cc40b3dd
pub const HASH_OUTPUT_TEX: u64 = binding_hash("output_tex"); // 0x88219a3ae69892b2
pub const KEY_INPUT_MIP_DIMS: u64 = binding_hash("input_mip_dims"); // 0x4b60ad975c278bdb
pub const HASH_INPUT_TAIL: u64 = binding_hash("input_tail_tex"); // 0xdcdfcf289eb02e68
pub const KEY_OUTPUT_EXTENT: u64 = binding_hash("output_extent"); // 0x51fcf121d36542ad
pub const KEY_SAMPLE_SELF: u64 = binding_hash("sample_self"); // 0xbea683204a46bd12

/// `TextureDesc::half_res`（O 0x140bd5730）：逐轴 `max(v - v/2, 1)`（= 向上取整的一半）。
pub fn half_res(extent: [u32; 2]) -> [u32; 2] {
    extent.map(|v| (v - v / 2).max(1))
}

/// `TextureDesc::div_extent`（O 0x140bd56a0）：逐轴 `max(v / d, 1)`（向下取整）；除数为 0 → panic。
pub fn div_extent(extent: [u32; 2], divisor: u32) -> [u32; 2] {
    assert!(divisor != 0, "attempt to divide by zero");
    extent.map(|v| (v / divisor).max(1))
}

/// 金字塔 mip 数（O：见文件头）。
pub fn pyramid_mips(half_extent: [u32; 2]) -> u8 {
    let v = (all_mip_levels(half_extent) as u8).wrapping_sub(1);
    if v >= 2 { v } else { 1 }
}

fn view(usage: u32, mip: u32) -> TextureViewDesc {
    TextureViewDesc::new()
        .usage(usage)
        .base_mip_level(mip)
        .level_count(Some(1))
}

/// W-外围：`blur_pyramid`（O 0x1409ee1e0）。`src` 为 RG 句柄（含 desc）。返回 "partial blur pyramid"。
pub fn blur_pyramid(env: &mut PassEnv<'_>, src: &RgTexture) -> RgTexture {
    blur_pyramid_with_input_policy(env, src, false)
}

fn blur_pyramid_with_input_policy(
    env: &mut PassEnv<'_>,
    src: &RgTexture,
    allow_undefined_input: bool,
) -> RgTexture {
    let src_extent = [src.desc.extent[0], src.desc.extent[1]];
    let half = half_res(src_extent);
    let mips = pyramid_mips(half);
    let desc = TextureDesc::new_2d(half, PYRAMID_FORMAT).mip_levels(mips);
    let partial = env.create_texture(PARTIAL_NAME, desc);
    let partial_tex = &partial;

    // blur0：输出视图 base_mip 0、level_count 1。
    let v0 = TextureViewDesc::new()
        .base_mip_level(0)
        .level_count(Some(1));
    let mut pass = env
        .pass(BLUR0_NAME, BLUR_URI)
        .bind_texture(HASH_INPUT_TEX, &src, TEX_READ)
        .bind_texture_with_view(HASH_OUTPUT_TEX, &partial_tex, TEX_STORAGE_WRITE, Some(&v0));
    if allow_undefined_input {
        // 仅原 no-world tail 显式请求；真实采样和同步、后续 mip producer 均保留。
        pass = pass.allow_undefined_texture_contents(HASH_INPUT_TEX);
    }
    let rec = pass
        .uniform(KEY_INPUT_MIP_DIMS, src_extent)
        .dispatch_2d(half[0], half[1]);
    env.record(rec);

    for i in 1..u32::from(mips) {
        let vr = view(USAGE_READ, i - 1);
        let vw = view(USAGE_WRITE, i);
        let prev = div_extent(half, 1 << (i - 1));
        let cur = div_extent(half, 1 << i);
        let rec = env
            .pass(BLUR_N_NAME, BLUR_URI)
            .bind_texture_with_view(HASH_INPUT_TEX, &partial_tex, TEX_READ, Some(&vr))
            .bind_texture_with_view(HASH_OUTPUT_TEX, &partial_tex, TEX_STORAGE_WRITE, Some(&vw))
            .uniform(KEY_INPUT_MIP_DIMS, prev)
            .dispatch_2d(cur[0], cur[1]);
        env.record(rec);
    }
    partial
}

/// `rev_blur_pyramid`（`bidirectional_blur_pyramid` 的后半，O 内联）。
pub fn rev_blur_pyramid(env: &mut PassEnv<'_>, partial: &RgTexture) -> RgTexture {
    let desc = partial.desc;
    let full = env.create_texture(FULL_NAME, desc);
    let n = u32::from(desc.mip_levels);
    let (partial_tex, full_tex) = (partial, &full);
    let extent = desc.extent_2d();
    for i in (0..n).rev() {
        let ext = div_extent(extent, 1 << i);
        let next = (i + 1).min(n - 1);
        let (v_tail, v_in, v_out) = (
            view(USAGE_READ, i),
            view(USAGE_READ, next),
            view(USAGE_WRITE, i),
        );
        let rec = env
            .pass(REV_N_NAME, REV_URI)
            .bind_texture_with_view(HASH_INPUT_TAIL, &partial_tex, TEX_READ, Some(&v_tail))
            .bind_texture_with_view(HASH_INPUT_TEX, &full_tex, TEX_READ, Some(&v_in))
            .bind_texture_with_view(HASH_OUTPUT_TEX, &full_tex, TEX_STORAGE_WRITE, Some(&v_out))
            .uniform(KEY_OUTPUT_EXTENT, ext)
            .uniform(KEY_SAMPLE_SELF, i + 1 != n)
            .dispatch_2d(ext[0], ext[1]);
        env.record(rec);
    }
    full
}

/// `bidirectional_blur_pyramid`（O 0x1409ed860）：`rev_blur_pyramid(blur_pyramid(src))`。
pub fn bidirectional_blur_pyramid(env: &mut PassEnv<'_>, src: &RgTexture) -> RgTexture {
    let partial = blur_pyramid(env, src);
    rev_blur_pyramid(env, &partial)
}

/// 原 no-world tail 最终 flag0 丢弃该输入的计算值，但仍真实执行全部 blur fetch。
/// 其它消费者使用默认严格入口，不能把此许可当作源初始化。
pub fn bidirectional_blur_pyramid_allowing_undefined_input(
    env: &mut PassEnv<'_>,
    src: &RgTexture,
) -> RgTexture {
    let partial = blur_pyramid_with_input_policy(env, src, true);
    rev_blur_pyramid(env, &partial)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    fn world() -> TestWorld {
        TestWorld::new(
            Reflection::new()
                .bind(&[HASH_INPUT_TEX, HASH_OUTPUT_TEX, HASH_INPUT_TAIL])
                .uniforms(&[KEY_INPUT_MIP_DIMS, KEY_OUTPUT_EXTENT, KEY_SAMPLE_SELF]),
        )
    }
    fn src(w: u32, h: u32) -> RgTexture {
        RgTexture {
            resource: 1,
            desc: TextureDesc::new([w, h, 1], 0, 0),
        }
    }
    /// pass 里每个 `ImageView` 绑定的视图描述（按绑定顺序）。
    fn views(
        p: &crate::render::pass_builder::compute_pass_builder::RecordedComputePass,
    ) -> Vec<TextureViewDesc> {
        use crate::render::pass_builder::pass_resource_access::PassBinding;
        p.bindings
            .iter()
            .filter_map(|b| match b {
                PassBinding::ImageView { view, .. } => Some(*view),
                _ => None,
            })
            .collect()
    }
    fn dispatch_xy(
        p: &crate::render::pass_builder::compute_pass_builder::RecordedComputePass,
    ) -> (u32, u32) {
        match p.dispatch {
            DispatchMode::Threads { x, y, z: 1 } => (x, y),
            _ => panic!("expected 2d thread dispatch"),
        }
    }
    fn u32s(
        p: &crate::render::pass_builder::compute_pass_builder::RecordedComputePass,
        o: usize,
    ) -> (u32, u32) {
        (uniform_u32(p, o), uniform_u32(p, o + 4))
    }

    #[test]
    fn extent_helpers() {
        assert_eq!(half_res([5, 1]), [3, 1]);
        assert_eq!(half_res([0, 2]), [1, 1]);
        assert_eq!(div_extent([960, 540], 4), [240, 135]);
        assert_eq!(div_extent([3, 1], 8), [1, 1]);
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn div_extent_by_zero_panics() {
        div_extent([4, 4], 0);
    }

    #[test]
    fn mip_count_rule() {
        // all = floor(log2(max))+1；v = all-1；mips = v>=2 ? v : 1。
        assert_eq!(pyramid_mips([960, 540]), 9);
        assert_eq!(pyramid_mips([1, 1]), 1);
        assert_eq!(pyramid_mips([2, 2]), 1); // all=2 → v=1 → 1
        assert_eq!(pyramid_mips([4, 4]), 2); // all=3 → v=2
        assert_eq!(pyramid_mips([8, 8]), 3);
    }

    #[test]
    fn forward_pyramid_pass_sequence_and_dims() {
        let w = world();
        let mut f = TestFrame::default();
        let partial = with_env(&w, &mut f, |env| blur_pyramid(env, &src(1920, 1080)));
        assert_eq!(f.textures.len(), 1);
        assert_eq!(f.textures[0].0, "partial blur pyramid");
        assert_eq!(
            f.textures[0].1,
            TextureDesc::new_2d([960, 540], 0x7a).mip_levels(9)
        );
        assert_eq!(partial.desc.mip_levels, 9);
        assert_eq!(f.order[0], "blur0");
        assert!(f.order[1..].iter().all(|n| n == "blurN"));
        assert_eq!(f.passes.len(), 9);
        assert_eq!(
            w.backend.paths.borrow()[0],
            "/blur_pyramid/blur.cs.hlsl:cs_main"
        );
        // blur0：uniform = src 尺寸；dispatch = half。
        assert_eq!(u32s(&f.passes[0], 0), (1920, 1080));
        assert_eq!(dispatch_xy(&f.passes[0]), (960, 540));
        // blurN(i)：uniform = half/2^(i-1)，dispatch = half/2^i。
        for i in 1..9u32 {
            let p = &f.passes[i as usize];
            assert_eq!(
                u32s(p, 0),
                (960 / (1 << (i - 1)), (540 / (1 << (i - 1))).max(1))
            );
            assert_eq!(
                dispatch_xy(p),
                ((960 / (1 << i)).max(1), (540 / (1 << i)).max(1))
            );
        }
        // 视图：blur0 只有输出视图；blurN 读 mip i-1、写 mip i。
        // blur0：输入 src 无视图（缺省描述），输出带视图。
        let v0 = views(&f.passes[0]);
        assert_eq!(v0.len(), 2);
        assert_eq!(v0[0], TextureViewDesc::default());
        assert_eq!(
            v0[1],
            TextureViewDesc::new()
                .base_mip_level(0)
                .level_count(Some(1))
        );
        let v3 = views(&f.passes[3]);
        assert_eq!(
            v3[0],
            TextureViewDesc {
                usage: Some(4),
                base_mip_level: 2,
                level_count: Some(1),
                ..Default::default()
            }
        );
        assert_eq!(
            v3[1],
            TextureViewDesc {
                usage: Some(8),
                base_mip_level: 3,
                level_count: Some(1),
                ..Default::default()
            }
        );
        // 访问：color library、读 src(0x19)、写 partial(0x37)。
        let acc: Vec<u8> = f.passes[0]
            .builder
            .accesses
            .iter()
            .map(|a| a.access)
            .collect();
        assert_eq!(acc, [0x1a, 0x19, 0x37]);
    }

    #[test]
    fn tiny_source_gets_a_single_pass() {
        let w = world();
        let mut f = TestFrame::default();
        with_env(&w, &mut f, |env| blur_pyramid(env, &src(4, 4)));
        // half = 2×2 → all=2 → v=1 → mips=1 → 仅 blur0。
        assert_eq!(f.passes.len(), 1);
        assert_eq!(f.order, ["blur0"]);
    }

    #[test]
    fn reverse_pyramid_walks_mips_top_down_with_sample_self_flag() {
        let w = world();
        let mut f = TestFrame::default();
        let partial = RgTexture {
            desc: TextureDesc::new_2d([64, 32], 0x7a).mip_levels(4),
            resource: 77,
        };
        let full = with_env(&w, &mut f, |env| rev_blur_pyramid(env, &partial));
        assert_eq!(f.textures.len(), 1);
        assert_eq!(
            (f.textures[0].0.as_str(), f.textures[0].1),
            ("full blur pyramid", partial.desc)
        );
        assert_eq!(full.desc, partial.desc);
        assert_eq!(f.passes.len(), 4);
        assert!(f.order.iter().all(|n| n == "rev_blurN"));
        assert_eq!(
            w.backend.paths.borrow()[0],
            "/blur_pyramid/rev_blur.cs.hlsl:cs_main"
        );
        // i = 3,2,1,0。
        let dims: Vec<(u32, u32)> = f.passes.iter().map(dispatch_xy).collect();
        assert_eq!(dims, [(8, 4), (16, 8), (32, 16), (64, 32)]);
        // output_extent @0（声明顺序：input_mip_dims 0、output_extent 16、sample_self 32）。
        for (k, p) in f.passes.iter().enumerate() {
            assert_eq!(u32s(p, 16), dims[k]);
        }
        let flags: Vec<u32> = f.passes.iter().map(|p| uniform_u32(p, 32)).collect();
        assert_eq!(
            flags,
            [0, 1, 1, 1],
            "sample_self = (i + 1 != N)：最顶层（i = N-1）为 false"
        );
        // 视图：i=3 时 full 的读 mip = min(4, 3) = 3；i=1 时 = 2。
        assert_eq!(views(&f.passes[0])[1].base_mip_level, 3);
        assert_eq!(views(&f.passes[2])[1].base_mip_level, 2);
        assert_eq!(views(&f.passes[2])[0].base_mip_level, 1, "partial 读 mip i");
        assert_eq!(views(&f.passes[2])[2].usage, Some(8));
        // 访问：color、partial 读、full 读、full 写。
        let acc: Vec<u8> = f.passes[0]
            .builder
            .accesses
            .iter()
            .map(|a| a.access)
            .collect();
        assert_eq!(acc, [0x1a, 0x19, 0x19, 0x37]);
    }

    #[test]
    fn bidirectional_is_forward_then_reverse() {
        let w = world();
        let mut f = TestFrame::default();
        let full = with_env(&w, &mut f, |env| {
            bidirectional_blur_pyramid(env, &src(64, 64))
        });
        // half = 32 → all = 6 → mips = 5：blur0 + 4×blurN，然后 5×rev_blurN。
        let names: Vec<&str> = f.order.iter().map(|s| s.as_str()).collect();
        let mut want = vec!["blur0"];
        want.extend(["blurN"; 4]);
        want.extend(["rev_blurN"; 5]);
        assert_eq!(names, want);
        assert_eq!(full.desc.mip_levels, 5);
        assert_eq!(
            f.textures.iter().map(|t| t.0.as_str()).collect::<Vec<_>>(),
            ["partial blur pyramid", "full blur pyramid"]
        );
    }

    #[test]
    fn no_world_permission_is_only_on_first_blur_input_and_keeps_every_dispatch() {
        use crate::render::pass_builder::pass_resource_access::PassBinding;
        let w = world();
        let mut strict = TestFrame::default();
        with_env(&w, &mut strict, |env| {
            bidirectional_blur_pyramid(env, &src(64, 64))
        });
        let mut permitted = TestFrame::default();
        with_env(&w, &mut permitted, |env| {
            bidirectional_blur_pyramid_allowing_undefined_input(env, &src(64, 64))
        });
        assert_eq!(strict.order, permitted.order);
        assert_eq!(strict.passes.len(), permitted.passes.len());
        for (index, (ordinary, original_no_world)) in
            strict.passes.iter().zip(&permitted.passes).enumerate()
        {
            assert!(ordinary.undefined_texture_bindings.is_empty());
            assert!(original_no_world.inactive_texture_bindings.is_empty());
            assert_eq!(dispatch_xy(ordinary), dispatch_xy(original_no_world));
            assert_eq!(
                ordinary.builder.accesses,
                original_no_world.builder.accesses
            );
            if index == 0 {
                assert_eq!(original_no_world.undefined_texture_bindings.len(), 1);
                let slot = original_no_world.undefined_texture_bindings[0] as usize;
                assert!(matches!(
                    original_no_world.bindings[slot],
                    PassBinding::ImageView {
                        resource: 1,
                        write: false,
                        ..
                    }
                ));
            } else {
                assert!(original_no_world.undefined_texture_bindings.is_empty());
            }
        }
    }

    #[test]
    fn names_and_hashes_match_machine_code() {
        assert_eq!(HASH_INPUT_TEX, 0x11ba5697cc40b3dd);
        assert_eq!(HASH_OUTPUT_TEX, (-0x77de65c519676d4e_i64) as u64);
        assert_eq!(KEY_INPUT_MIP_DIMS, 0x4b60ad975c278bdb);
        assert_eq!(HASH_INPUT_TAIL, (-0x232030d7614fd198_i64) as u64);
        assert_eq!(KEY_OUTPUT_EXTENT, 0x51fcf121d36542ad);
        assert_eq!(KEY_SAMPLE_SELF, (-0x41597cdfb5b942ee_i64) as u64);
        assert_eq!(PARTIAL_NAME.len(), 0x14);
        assert_eq!(FULL_NAME.len(), 0x11);
        assert_eq!(BLUR_URI.len(), 0x1a);
        assert_eq!(REV_URI.len(), 0x1e);
        assert_eq!(REV_N_NAME.len(), 9);
    }
}
