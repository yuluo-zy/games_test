//! `generate_hzb.rs` — W26 层级深度缓冲（FidelityFX SPD，O：`generate_hzb` 0x1409286f0，2666 B）。
//!
//! 参数（O）：`(out: Hzb*, ctx, world, depth: &Handle<Texture>(+desc), name: &str)`。流程：
//! 1. `format!("SPD Atomic: {name}")` → `get_or_create_temporal(…, new_gpu_only(0x18, 0x20))`（`Err` → `Result::unwrap` panic）。
//! 2. `(w, h) = depth.desc.extent_2d()`；HZB 尺寸 `(np2(w/2), np2(h/2))`，其中 **`w < 4`（`h < 4`）取 1**
//!    （`cmp eax,4; cmovb` / `shr rax,0x22; cmove`）；`np2` = `next_power_of_two`（`bsr` + `0xFFFFFFFF >> lz`）。
//! 3. `TextureDesc::new_2d(extent, R16_SFLOAT /*0x4c*/).all_mip_levels()`，`mip_levels = min(all, 13)`；
//!    `create_raw_resource(desc, "HZB")`。
//! 4. `groups = (div_ceil(hzb.w, 64), div_ceil(hzb.h, 64))`。
//! 5. `ComputePass::new("Generate HZB", "/generate_hzb.cs.hlsl")`：
//!    - 深度读（0x19）→ `imgSrc`（无视图）；
//!    - HZB 写视图 B：`bind_individual_mips(true).base_mip_level(0).view_type(5)`（0x37）→ `imgDst`
//!      （与 generate_mips 的差别：基 mip 为 **0**）；
//!    - HZB 写视图 C：`view_type(5).base_mip_level(min(mips-1,6)).level_count(Some(1))`（0x37）→ `imgDst6`；
//!    - `write_rg_storage_buffer(spdGlobalAtomic, atomic)`。
//! 6. push constants 24 B：`[mips-1][groups.x*groups.y][0: u64][depth.w][depth.h]`；`dispatch_groups(gx, gy, 1)`。
//! 7. 返回 `{half_extent: (w>>1, h>>1), hzb 资源}`（O：`[out+0]=w>>1, [out+4]=h>>1, [out+0x28]=资源号`）。

use crate::render::pass_builder::compute_pass_builder::ComputeTexture;
use crate::render::pass_builder::pass_resource_access::RgTexture;
use crate::rhapsody_types::{TextureDesc, TextureViewDesc};
use crate::systems::render_loop::compute_passes::generate_mips::{
    HASH_ATOMIC, HASH_MIP_LAST, HASH_MIP0, HASH_MIPS, MAX_MIPS, VIEW_TYPE_2D_ARRAY, bind_with_view,
    div_ceil_tile, last_mip_view, spd_atomic_buffer, spd_push_constants,
};
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, TEX_READ, TEX_STORAGE_WRITE};

pub const PASS_NAME: &str = "Generate HZB";
pub const SHADER_URI: &str = "/generate_hzb.cs.hlsl";
/// HZB 资源名（O：`create_raw_resource(desc, "HZB")`）。
pub const HZB_NAME: &str = "HZB";
/// `ash::vk::Format::R16_SFLOAT`（O：`new_2d(.., 0x4c)`）。
pub const HZB_FORMAT: u32 = 0x4c;

/// 返回给调用方的 HZB 信息。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hzb {
    /// `(w >> 1, h >> 1)`（O：`[out+0]`、`[out+4]`）。
    pub half_extent: [u32; 2],
    pub texture: RgTexture,
}

/// `next_power_of_two`，`< 4` 取 1（O：`cmp eax, 4; cmovb esi, 1`；对 `n = dim>>1`：`dim<4` ⇒ `n<2` ⇒ 1）。
pub fn hzb_dim(dim: u32) -> u32 {
    if dim < 4 {
        return 1;
    }
    let n = dim >> 1;
    // `0xFFFFFFFF >> lz(n-1)` + 1（O：`dec; bsr; not; shr; inc`）。
    (u32::MAX >> (n - 1).leading_zeros()) + 1
}

/// `TextureDesc::all_mip_levels`（rhapsody，范围外；I：`floor(log2(max(w,h))) + 1`）。
pub fn all_mip_levels(extent: [u32; 2]) -> u32 {
    32 - extent[0].max(extent[1]).max(1).leading_zeros()
}

/// W26 入口。`depth` 为主深度（RG 句柄），`depth_desc` 为其描述。
pub fn generate_hzb<A: ComputeTexture>(
    env: &mut PassEnv<'_>,
    depth: &A,
    depth_desc: &TextureDesc,
    name: &str,
) -> Hzb {
    let atomic = spd_atomic_buffer(env, name);
    let [w, h] = depth_desc.extent_2d();
    let extent = [hzb_dim(w), hzb_dim(h)];
    let mips = all_mip_levels(extent).min(u32::from(MAX_MIPS)) as u8;
    let desc = TextureDesc::new_2d(extent, HZB_FORMAT).mip_levels(mips);
    let hzb = env.create_texture(HZB_NAME, desc);
    let groups = [div_ceil_tile(extent[0]), div_ceil_tile(extent[1])];
    let pass = env
        .pass(PASS_NAME, SHADER_URI)
        .bind_texture(HASH_MIP0, depth, TEX_READ);
    let hzb_ref = &hzb;
    let pass = bind_with_view(
        pass,
        HASH_MIPS,
        &hzb_ref,
        TEX_STORAGE_WRITE,
        TextureViewDesc::new()
            .bind_individual_mips(true)
            .base_mip_level(0)
            .view_type(VIEW_TYPE_2D_ARRAY),
    );
    let pass = bind_with_view(
        pass,
        HASH_MIP_LAST,
        &hzb_ref,
        TEX_STORAGE_WRITE,
        last_mip_view(mips),
    );
    let mut push = spd_push_constants(u32::from(mips).wrapping_sub(1), groups);
    push.extend_from_slice(&w.to_le_bytes());
    push.extend_from_slice(&h.to_le_bytes());
    let recorded = pass
        .write_rg_storage_buffer(HASH_ATOMIC, &atomic)
        .push_constants(&push)
        .dispatch_groups(groups[0], groups[1], 1);
    env.record(recorded);
    Hzb {
        half_extent: [w >> 1, h >> 1],
        texture: hzb,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    fn world() -> TestWorld {
        TestWorld::new(Reflection::new().bind(&[HASH_MIP0, HASH_MIPS, HASH_MIP_LAST, HASH_ATOMIC]))
    }
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
    fn depth(w: u32, h: u32) -> (RgTexture, TextureDesc) {
        (
            RgTexture {
                resource: 7,
                desc: TextureDesc::new([w, h, 1], 0, 0),
            },
            TextureDesc::new_2d([w, h], 126).mip_levels(1),
        )
    }

    #[test]
    fn hzb_dimension_rule() {
        // < 4 → 1；否则 next_power_of_two(dim / 2)。
        assert_eq!(hzb_dim(1), 1);
        assert_eq!(hzb_dim(3), 1);
        assert_eq!(hzb_dim(4), 2);
        assert_eq!(hzb_dim(5), 2);
        assert_eq!(hzb_dim(6), 4); // n=3 → 4
        assert_eq!(hzb_dim(1920), 1024);
        assert_eq!(hzb_dim(1080), 1024);
        assert_eq!(hzb_dim(2560), 2048);
        assert_eq!(hzb_dim(1440), 1024);
        for d in 4..5000u32 {
            assert_eq!(hzb_dim(d), (d / 2).next_power_of_two(), "dim {d}");
        }
    }

    #[test]
    fn all_mips_is_log2_plus_one() {
        assert_eq!(all_mip_levels([1, 1]), 1);
        assert_eq!(all_mip_levels([1024, 512]), 11);
        assert_eq!(all_mip_levels([1025, 8]), 11);
    }

    #[test]
    fn creates_hzb_texture_and_records_group_dispatch() {
        let w = world();
        let mut f = TestFrame::default();
        let (tex, desc) = depth(1920, 1080);
        let hzb = with_env(&w, &mut f, |env| generate_hzb(env, &&tex, &desc, "main"));
        assert_eq!(hzb.half_extent, [960, 540]);
        assert_eq!(f.textures.len(), 1);
        assert_eq!(f.textures[0].0, "HZB");
        assert_eq!(
            f.textures[0].1,
            TextureDesc::new_2d([1024, 1024], 0x4c).mip_levels(11)
        );
        assert_eq!(hzb.texture, f.textures[0].2);
        assert_eq!(f.temporal_buffers[0].0, "SPD Atomic: main");
        assert_eq!(
            (f.temporal_buffers[0].1.size, f.temporal_buffers[0].1.usage),
            (0x18, 0x20)
        );
        let p = &f.passes[0];
        assert_eq!(p.name, "Generate HZB");
        assert_eq!(w.backend.paths.borrow()[0], "/generate_hzb.cs.hlsl:cs_main");
        assert!(matches!(
            p.dispatch,
            DispatchMode::Groups { x: 16, y: 16, z: 1 }
        ));
        // push constants 24 B：mips-1=10，groups 16*16，0，源 1920×1080。
        let mut want = Vec::new();
        for v in [10u32, 256, 0, 0, 1920, 1080] {
            want.extend_from_slice(&v.to_le_bytes());
        }
        assert_eq!(p.push_constants, want);
        // 深度读 0x19，其余写 0x37；先 color library。
        let acc: Vec<u8> = p.builder.accesses.iter().map(|a| a.access).collect();
        assert_eq!(acc, [0x1a, 0x19, 0x37, 0x37, 0x37]);
        let v = views(p);
        // 深度读（无视图 = 缺省描述）+ 两个 HZB 写视图。
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], (false, TextureViewDesc::default()));
        assert_eq!(
            v[1],
            (
                true,
                TextureViewDesc {
                    base_mip_level: 0,
                    view_type: Some(5),
                    bind_individual_mips: true,
                    ..Default::default()
                }
            )
        );
        assert_eq!(v[2].1.base_mip_level, 6);
    }

    #[test]
    fn tiny_depth_buffers_clamp_to_one_texel() {
        let w = world();
        let mut f = TestFrame::default();
        let (tex, desc) = depth(2, 3);
        let hzb = with_env(&w, &mut f, |env| generate_hzb(env, &&tex, &desc, "t"));
        assert_eq!(hzb.half_extent, [1, 1]);
        assert_eq!(f.textures[0].1.extent, [1, 1, 1]);
        assert!(matches!(
            f.passes[0].dispatch,
            DispatchMode::Groups { x: 1, y: 1, z: 1 }
        ));
        assert_eq!(f.textures[0].1.mip_levels, 1);
    }

    #[test]
    fn mip_count_is_clamped_to_13() {
        let w = world();
        let mut f = TestFrame::default();
        let (tex, desc) = depth(65536, 4);
        with_env(&w, &mut f, |env| generate_hzb(env, &&tex, &desc, "t"));
        // 32768 × 2 → all = 16 → clamp 13；mips-1 = 12。
        assert_eq!(f.textures[0].1.mip_levels, 13);
        assert_eq!(&f.passes[0].push_constants[0..4], &12u32.to_le_bytes());
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn temporal_failure_unwraps_before_creating_the_texture() {
        let w = world();
        let mut f = TestFrame {
            fail_temporal: true,
            ..Default::default()
        };
        let (tex, desc) = depth(64, 64);
        with_env(&w, &mut f, |env| generate_hzb(env, &&tex, &desc, "t"));
    }
}
