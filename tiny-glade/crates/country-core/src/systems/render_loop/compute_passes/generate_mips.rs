//! `generate_mips.rs` — W50 通用 mip 链（FidelityFX SPD 单 pass，两个单态化：
//! `generate_mips<TexturePreset>` 0x140a3a8f0 / `generate_mips<WriteTextureUnchecked>` 0x1400fb890，机器码同形）。
//!
//! 参数（O，TexturePreset 实例）：`(ctx, world, &TextureDesc, texture, name: &str)`。流程：
//! 1. `assert!(desc.mip_levels <= 13)`（"assertion failed: texture_desc.mip_levels <= 13"）；
//!    `assert_eq!(desc.format, B10G11R11_UFLOAT_PACK32 /*0x7a*/)`（`assert_failed<Format,Format>`）。
//! 2. `format!("SPD Atomic: {name}")` → `Ustr` → `get_or_create_temporal(name, BufferDesc::new_gpu_only(0x18, 0x20))`，
//!    `Err` → `Result::unwrap` panic。
//! 3. `(w, h) = desc.extent_2d()`；`groups = (div_ceil(w, 64), div_ceil(h, 64))`。
//! 4. `ComputePass::new("Generate mips", "/generate_mips.cs.hlsl")`，三个存储图像视图（访问码均 0x37）：
//!    - 视图 A：`base_mip_level(0).level_count(Some(1))` → 绑定名 `imgSrc`；
//!    - 视图 B：`bind_individual_mips(true).base_mip_level(1).view_type(2D_ARRAY=5)` → `imgDst`；
//!    - 视图 C：`view_type(5).base_mip_level(min(mips-1, 6)).level_count(Some(1))` → `imgDst6`；
//!    `write_rg_storage_buffer(spdGlobalAtomic, atomic)`。
//! 5. push constants 16 B：`[mips-1: u32][groups.x*groups.y: u32][0: u64]`；`dispatch_groups(gx, gy, 1)`（模式 0）。
//!
//! 绑定走 A4 的 `bind_texture_with_view`（RG 路径，整块 `TextureViewDesc` 进入 `PassBinding::ImageView.view`）。

// Rust 1.88 enables newer style suggestions; retain original nested branches and modulo expressions.
#![allow(clippy::manual_is_multiple_of)]
// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::doc_lazy_continuation)]
use crate::render::pass_builder::compute_pass_builder::RgBuffer;
use crate::render::pass_builder::compute_pass_builder::{ComputePass, ComputeTexture};
use crate::rhapsody_types::{TextureDesc, TextureViewDesc};
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, TEX_STORAGE_WRITE, binding_hash,
};

pub const PASS_NAME: &str = "Generate mips";
pub const SHADER_URI: &str = "/generate_mips.cs.hlsl";
/// `assert!(texture_desc.mip_levels <= 13)` 的文案（O，rodata 原文）。
pub const MIP_ASSERT_MSG: &str = "assertion failed: texture_desc.mip_levels <= 13";
/// `ash::vk::Format::B10G11R11_UFLOAT_PACK32`（O：`cmp dword ptr [desc+8], 0x7a`）。
pub const SPD_FORMAT: u32 = 0x7a;
/// `ash::vk::ImageViewType::TYPE_2D_ARRAY`（O：`view_type(5)`）。
pub const VIEW_TYPE_2D_ARRAY: u32 = 5;
/// SPD 一次最多处理的 mip 数。
pub const MAX_MIPS: u8 = 13;
/// SPD 每线程组覆盖 64×64 像素。
pub const TILE: u32 = 64;
/// 绑定名哈希（O，movabs 立即数）。
pub const HASH_MIP0: u64 = binding_hash("imgSrc"); // 0xb466b30ac6fc0292
pub const HASH_MIPS: u64 = binding_hash("imgDst"); // 0xed9d1f0a5683f687
pub const HASH_MIP_LAST: u64 = binding_hash("imgDst6"); // 0x45f26f91023c2ec3
pub const HASH_ATOMIC: u64 = binding_hash("spdGlobalAtomic"); // 0x068d4413b4de4f24
/// 原子计数缓冲：`BufferDesc::new_gpu_only(0x18, 0x20)`（O）。
pub const ATOMIC_SIZE: u32 = 0x18;
pub const ATOMIC_USAGE: u32 = 0x20;

/// `div_ceil(v, 64)`（O：`shr 6` + `and 0x3f; cmp 1; sbb -1`）。
pub fn div_ceil_tile(v: u32) -> u32 {
    v / TILE + u32::from(v % TILE != 0)
}

/// 视图 C 的起始 mip：`min(mips - 1, 6)`（O：`dec ebx; cmp ebx,6; cmovb`）。
pub fn last_view_base_mip(mips: u8) -> u32 {
    (u32::from(mips).wrapping_sub(1)).min(6)
}

/// SPD push constants（16 B；hzb 版再追加 8 B 源尺寸）。
pub fn spd_push_constants(mips_minus_one: u32, groups: [u32; 2]) -> Vec<u8> {
    let mut b = Vec::with_capacity(16);
    b.extend_from_slice(&mips_minus_one.to_le_bytes());
    b.extend_from_slice(&groups[0].wrapping_mul(groups[1]).to_le_bytes());
    b.extend_from_slice(&0u64.to_le_bytes());
    b
}

/// `get_or_create_temporal("SPD Atomic: {name}", new_gpu_only(0x18, 0x20))`；`Err` → `Result::unwrap` panic。
pub fn spd_atomic_buffer(env: &mut PassEnv<'_>, name: &str) -> RgBuffer {
    let label = format!("SPD Atomic: {name}");
    env.temporal_buffer(&label, ATOMIC_SIZE, ATOMIC_USAGE)
        .buffer
}

/// 绑定一个带视图描述的存储图像（O：`bind_texture_to_compute_pass` 带视图实例，访问码 0x37）。
pub fn bind_with_view<A: ComputeTexture>(
    pass: ComputePass,
    hash: u64,
    texture: &A,
    access: u8,
    view: TextureViewDesc,
) -> ComputePass {
    pass.bind_texture_with_view(hash, texture, access, Some(&view))
}

/// SPD 的两个 mip 写视图（B、C）的描述。
pub fn individual_mips_view() -> TextureViewDesc {
    TextureViewDesc::new()
        .bind_individual_mips(true)
        .base_mip_level(1)
        .view_type(VIEW_TYPE_2D_ARRAY)
}
pub fn last_mip_view(mips: u8) -> TextureViewDesc {
    TextureViewDesc::new()
        .view_type(VIEW_TYPE_2D_ARRAY)
        .base_mip_level(last_view_base_mip(mips))
        .level_count(Some(1))
}

/// W50 入口（O：两个单态化同形）。`texture` 为 `TexturePreset` / `WriteTextureUnchecked`。
pub fn generate_mips<A: ComputeTexture>(
    env: &mut PassEnv<'_>,
    desc: &TextureDesc,
    texture: &A,
    name: &str,
) {
    assert!(
        desc.mip_levels <= MAX_MIPS,
        "assertion failed: texture_desc.mip_levels <= 13"
    );
    assert_eq!(desc.format, SPD_FORMAT);
    let atomic = spd_atomic_buffer(env, name);
    let groups = [div_ceil_tile(desc.extent[0]), div_ceil_tile(desc.extent[1])];
    let mips = desc.mip_levels;
    let pass = env.pass(PASS_NAME, SHADER_URI);
    let pass = bind_with_view(
        pass,
        HASH_MIP0,
        texture,
        TEX_STORAGE_WRITE,
        TextureViewDesc::new()
            .base_mip_level(0)
            .level_count(Some(1)),
    );
    let pass = bind_with_view(
        pass,
        HASH_MIPS,
        texture,
        TEX_STORAGE_WRITE,
        individual_mips_view(),
    );
    let pass = bind_with_view(
        pass,
        HASH_MIP_LAST,
        texture,
        TEX_STORAGE_WRITE,
        last_mip_view(mips),
    );
    // Original packaged shader declares imgSrc readonly despite its legacy
    // RW synchronization code. Preserve only the exact mip0 binding range.
    let pass = pass.preserve_storage_texture_contents(HASH_MIP0)
        .write_rg_storage_buffer(HASH_ATOMIC, &atomic);
    // SPD receives (mip_levels-1). Its cross-workgroup atomic path is
    // uniformly skipped when that push value is <=6. Above that threshold its original atomicAdd consumes existing contents;
    // a missing actual initial atomic producer must remain a strict graph error.
    let pass = if u32::from(mips).wrapping_sub(1) > 6 {
        pass.preserve_storage_buffer_contents(HASH_ATOMIC)
    } else { pass };
    let recorded = pass
        .push_constants(&spd_push_constants(u32::from(mips).wrapping_sub(1), groups))
        .dispatch_groups(groups[0], groups[1], 1);
    env.record(recorded);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::resources::texture_presets::TexturePreset;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    fn refl() -> Reflection {
        Reflection::new().bind(&[HASH_MIP0, HASH_MIPS, HASH_MIP_LAST, HASH_ATOMIC])
    }
    fn desc(w: u32, h: u32, mips: u8) -> TextureDesc {
        TextureDesc::new_2d([w, h], SPD_FORMAT).mip_levels(mips)
    }
    /// `ImageView` 绑定里的 `(写标志, 视图描述)`（颜色库是缓冲，不算）。
    fn views(
        p: &crate::render::pass_builder::compute_pass_builder::RecordedComputePass,
    ) -> Vec<(bool, TextureViewDesc)> {
        use crate::render::pass_builder::pass_resource_access::PassBinding;
        p.bindings
            .iter()
            .filter_map(|b| match b {
                PassBinding::ImageView { write, view, .. } => Some((*write, *view)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn binding_names_match_exe_immediates() {
        assert_eq!(HASH_MIP0, 0xb466b30ac6fc0292);
        assert_eq!(HASH_MIPS, 0xed9d1f0a5683f687);
        assert_eq!(HASH_MIP_LAST, 0x45f26f91023c2ec3);
        assert_eq!(HASH_ATOMIC, 0x068d4413b4de4f24);
    }

    #[test]
    fn tile_division_rounds_up() {
        assert_eq!(div_ceil_tile(0), 0);
        assert_eq!(div_ceil_tile(1), 1);
        assert_eq!(div_ceil_tile(64), 1);
        assert_eq!(div_ceil_tile(65), 2);
        assert_eq!(div_ceil_tile(1920), 30);
        assert_eq!(div_ceil_tile(1080), 17);
    }

    #[test]
    fn last_view_base_is_min_of_mips_minus_one_and_six() {
        assert_eq!(last_view_base_mip(1), 0);
        assert_eq!(last_view_base_mip(4), 3);
        assert_eq!(last_view_base_mip(7), 6);
        assert_eq!(last_view_base_mip(13), 6);
    }

    #[test]
    fn push_constants_layout() {
        let b = spd_push_constants(5, [3, 4]);
        assert_eq!(b.len(), 16);
        assert_eq!(&b[0..4], &5u32.to_le_bytes());
        assert_eq!(&b[4..8], &12u32.to_le_bytes());
        assert_eq!(&b[8..16], &[0u8; 8]);
    }

    #[test]
    fn records_spd_pass_with_three_views_atomic_and_group_dispatch() {
        let w = TestWorld::new(refl());
        let mut f = TestFrame::default();
        with_env(&w, &mut f, |env| {
            generate_mips(env, &desc(300, 129, 6), &TexturePreset(8), "terrain")
        });
        assert_eq!(f.temporal_buffers.len(), 1);
        assert_eq!(f.temporal_buffers[0].0, "SPD Atomic: terrain");
        assert_eq!(
            (f.temporal_buffers[0].1.size, f.temporal_buffers[0].1.usage),
            (0x18, 0x20)
        );
        let p = &f.passes[0];
        assert_eq!(p.name, "Generate mips");
        assert_eq!(
            w.backend.paths.borrow()[0],
            "/generate_mips.cs.hlsl:cs_main"
        );
        assert!(matches!(
            p.dispatch,
            DispatchMode::Groups { x: 5, y: 3, z: 1 }
        ));
        assert_eq!(p.push_constants, spd_push_constants(5, [5, 3]));
        let v = views(p);
        assert_eq!(v.len(), 3);
        assert!(v.iter().all(|(w, _)| *w), "三个视图都是写");
        assert_eq!(
            v[0].1,
            TextureViewDesc::new()
                .base_mip_level(0)
                .level_count(Some(1))
        );
        assert_eq!(
            v[1].1,
            TextureViewDesc {
                base_mip_level: 1,
                view_type: Some(5),
                bind_individual_mips: true,
                ..Default::default()
            }
        );
        assert_eq!(
            v[2].1,
            TextureViewDesc {
                base_mip_level: 5,
                level_count: Some(1),
                view_type: Some(5),
                ..Default::default()
            }
        );
        // 访问：color library 读(0x1a)、三个视图写(0x37)、atomic 写(0x37)。
        let acc: Vec<u8> = p.builder.accesses.iter().map(|a| a.access).collect();
        assert_eq!(acc, [0x1a, 0x37, 0x37, 0x37, 0x37]);
    }

    #[test]
    #[should_panic(expected = "assertion failed: texture_desc.mip_levels <= 13")]
    fn more_than_13_mips_asserts_before_anything_is_created() {
        let w = TestWorld::new(refl());
        let mut f = TestFrame::default();
        with_env(&w, &mut f, |env| {
            generate_mips(env, &desc(64, 64, 14), &TexturePreset(8), "x")
        });
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn wrong_format_asserts() {
        let w = TestWorld::new(refl());
        let mut f = TestFrame::default();
        let mut d = desc(64, 64, 4);
        d.format = 37;
        with_env(&w, &mut f, |env| {
            generate_mips(env, &d, &TexturePreset(8), "x")
        });
    }

    #[test]
    fn exactly_13_mips_is_allowed_and_atomic_is_created_before_the_pass() {
        let w = TestWorld::new(refl());
        let mut f = TestFrame::default();
        with_env(&w, &mut f, |env| {
            generate_mips(env, &desc(4096, 4096, 13), &TexturePreset(8), "big")
        });
        assert_eq!(f.passes.len(), 1);
        assert_eq!(views(&f.passes[0])[2].1.base_mip_level, 6);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn temporal_creation_failure_unwraps() {
        let w = TestWorld::new(refl());
        let mut f = TestFrame {
            fail_temporal: true,
            ..Default::default()
        };
        with_env(&w, &mut f, |env| {
            generate_mips(env, &desc(64, 64, 4), &TexturePreset(8), "x")
        });
    }
}
