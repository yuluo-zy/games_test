//! `compute_path_masks.rs` — W10 `compute_path_masks`（O 0x1409014e0，7779 B）与 `blur_path_mask`（O 0x140903350，2364 B）。
//!
//! ## compute_path_masks（参数 O：`(ctx, world)`）
//! 1. 依次 `get_resource_id`（任一缺失 → unwrap panic，**全部先于任何判断**）：
//!    A = [`PathRaster`]、B = [`Cobblestone1Raster`]、C = [`Cobblestone2Raster`]、D = [`Cobblestone3Raster`]、
//!    E = [`PathStrokeQueues`]（TypeId 0xcda6af2f6afb130e/0x036e88a927bd70aa；只读 `+0x10`、`+0x30`）。
//! 2. 运行条件（O：`cmp [A+0x29b],0; jne run; cmp [B+0x29b],1; je run; cmp [C+0x29b],0; jne run;
//!    (E+0x30 | E+0x10) != 0 → run; D+0x29b == 0 → return`）：
//!    `A.mask_dirty || B.mask_dirty || C.mask_dirty || E.nonempty || D.mask_dirty`。
//! 3. 清理：A、B、C、D 的 `+0x29b = 0` 且 `memset(+0x98, 0, 0x200)`，各自 `changed_tick(+0x5c) = world.change_tick`（**D 即使不是触发原因也清**）。
//! 4. pass "Combine path exlusion masks"（原文拼写 "exlusion"，`/paths/combine_path_exclusions.cs.hlsl`）：
//!    读 preset 1/2/3/4 → `path_mask` / `cobblestone1_mask` / `cobblestone2_mask` / `cobblestone3_mask`（0x19），
//!    写 preset 5 → `paths_exclusion`（0x32）；`dispatch_2d(512, 512)`。
//! 5. 临时纹理 "blur temp 0"、"blur temp 1"（各 `new_2d(512×512, R16_SFLOAT /*0x4c*/)`）。
//! 6. pass "Directional contraction"（`/paths/directional_contract.cs.hlsl`）：读 preset 9 `terrain_heightmap`、
//!    0xa `visual_terrain_heightmap`、0xd `terrain_sdg`、5 `path_mask`（0x19），写 **temp1** → `img_out`（0x32）。
//! 7. 两个 "Blur Path Stroke"（`/gauss_blur_r16f.cs.hlsl`，`is_horizontal` = true / false，`sigma` = 2.0，`dispatch_2d(512,512)`）：
//!    水平：读 temp1(`img_in`) → 写 temp0(`img_out`)；垂直：读 temp0 → 写 **preset 5**。
//! 8. `blur_path_mask(preset)` ×4（preset 1..4，各返回其模糊后的纹理）。
//! 9. pass "Path Mask: Visual"（`/paths/path_mask_visual.cs.hlsl`）：读 preset 0x13 `water_mask`；
//!    四个模糊结果 → `path_mask_blurred` / `cobblestone1_mask_blurred` / `cobblestone2_mask_blurred` / `cobblestone3_mask_blurred`；
//!    读 preset 9 / 0xa；写 preset 6 `paths_visual`、preset 7 `dirt_mask`（0x32）；`dispatch_2d(512, 512)`。
//!
//! ## blur_path_mask（参数 O：`(sret, ctx, world, preset: u8)`；返回模糊后的 RG 纹理句柄）
//! 建 "blur temp 0"/"blur temp 1"（同上）；两个 "Blur Path Stroke"：水平 读 `preset` → 写 temp0；垂直 读 temp0 → 写 temp1；返回 **temp1**。

use crate::render::pass_builder::pass_resource_access::RgTexture;
use crate::render::pass_builder::pass_resources::TypeId128;
use crate::resources::texture_presets::TexturePreset;
use crate::rhapsody_types::TextureDesc;
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, ResourceView, TEX_READ, TEX_WRITE, binding_hash,
};
use crate::systems::render_loop::compute_passes::upload_world_raster::{
    Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster, PathRaster, RasterKind,
};

pub const GRID: u32 = 512;
/// `ash::vk::Format::R16_SFLOAT`（O：`new_2d(.., 0x4c)`）。
pub const TEMP_FORMAT: u32 = 0x4c;
pub const TEMP0_NAME: &str = "blur temp 0";
pub const TEMP1_NAME: &str = "blur temp 1";
pub const BLUR_NAME: &str = "Blur Path Stroke";
pub const BLUR_URI: &str = "/gauss_blur_r16f.cs.hlsl";
pub const BLUR_SIGMA: f32 = 2.0;
pub const COMBINE_NAME: &str = "Combine path exlusion masks";
pub const COMBINE_URI: &str = "/paths/combine_path_exclusions.cs.hlsl";
pub const CONTRACT_NAME: &str = "Directional contraction";
pub const CONTRACT_URI: &str = "/paths/directional_contract.cs.hlsl";
pub const VISUAL_NAME: &str = "Path Mask: Visual";
pub const VISUAL_URI: &str = "/paths/path_mask_visual.cs.hlsl";

pub const KEY_HORIZONTAL: u64 = binding_hash("is_horizontal"); // 0xd91ff8372094bc22
pub const KEY_SIGMA: u64 = binding_hash("sigma"); // 0xdc022c4f3092a794
pub const HASH_IMG_IN: u64 = binding_hash("img_in"); // 0xd82d7a0adb9164c2
pub const HASH_IMG_OUT: u64 = binding_hash("img_out"); // 0x1ca51a73373208db
pub const HASH_TERRAIN: u64 = binding_hash("terrain_heightmap");
pub const HASH_VISUAL_TERRAIN: u64 = binding_hash("visual_terrain_heightmap");

/// 资源 E：两个队列的"非空"判断（O：`or [E+0x30], [E+0x10]`；其余布局 U）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PathStrokeQueues {
    _a: [u8; 0x10],
    pub first_word: u64,
    _b: [u8; 0x18],
    pub second_word: u64,
}

impl PathStrokeQueues {
    pub fn with_words(first: u64, second: u64) -> Self {
        PathStrokeQueues {
            _a: [0; 0x10],
            first_word: first,
            _b: [0; 0x18],
            second_word: second,
        }
    }
    pub fn nonempty(&self) -> bool {
        (self.second_word | self.first_word) != 0
    }
}

// SAFETY: 布局按 `+0x10` / `+0x30` 的读偏移复刻。
unsafe impl ResourceView for PathStrokeQueues {
    const TYPE_ID: TypeId128 = TypeId128 {
        lo: 0xcda6af2f6afb130e,
        hi: 0x036e88a927bd70aa,
    };
}

fn create_blur_temps(env: &mut PassEnv<'_>) -> (RgTexture, RgTexture) {
    let desc = TextureDesc::new_2d([GRID, GRID], TEMP_FORMAT).mip_levels(1);
    let t0 = env.create_texture(TEMP0_NAME, desc);
    let t1 = env.create_texture(TEMP1_NAME, desc);
    (t0, t1)
}

/// 一个 "Blur Path Stroke" pass。`src`/`dst` 为 `TexturePreset` 或 RG 纹理（经闭包选择绑定方式）。
fn blur_stroke(
    env: &mut PassEnv<'_>,
    horizontal: bool,
    bind_src: impl FnOnce(
        crate::render::pass_builder::compute_pass_builder::ComputePass,
    ) -> crate::render::pass_builder::compute_pass_builder::ComputePass,
    bind_dst: impl FnOnce(
        crate::render::pass_builder::compute_pass_builder::ComputePass,
    ) -> crate::render::pass_builder::compute_pass_builder::ComputePass,
) {
    let pass = env
        .pass(BLUR_NAME, BLUR_URI)
        .uniform(KEY_HORIZONTAL, horizontal)
        .uniform(KEY_SIGMA, BLUR_SIGMA);
    let pass = bind_dst(bind_src(pass));
    let recorded = pass.dispatch_2d(GRID, GRID);
    env.record(recorded);
}

/// O 0x140903350：`blur_path_mask`。
pub fn blur_path_mask(env: &mut PassEnv<'_>, preset: u8) -> RgTexture {
    let (tex0, tex1) = create_blur_temps(env);
    // 水平：preset → temp0
    blur_stroke(
        env,
        true,
        |p| p.bind_texture(HASH_IMG_IN, &TexturePreset(preset), TEX_READ),
        |p| p.bind_texture(HASH_IMG_OUT, &&tex0, TEX_WRITE),
    );
    // 垂直：temp0 → temp1
    blur_stroke(
        env,
        false,
        |p| p.bind_texture(HASH_IMG_IN, &&tex0, TEX_READ),
        |p| p.bind_texture(HASH_IMG_OUT, &&tex1, TEX_WRITE),
    );
    tex1
}

/// 条件判断 + 清理（O：步骤 1~3）；返回是否运行。
fn should_run_and_consume(env: &PassEnv<'_>) -> bool {
    let a = env.resource::<PathRaster>();
    let b = env.resource::<Cobblestone1Raster>();
    let c = env.resource::<Cobblestone2Raster>();
    let d = env.resource::<Cobblestone3Raster>();
    let e = env.resource::<PathStrokeQueues>();
    let run = a.flags().mask_dirty.get()
        || b.flags().mask_dirty.get()
        || c.flags().mask_dirty.get()
        || e.nonempty()
        || d.flags().mask_dirty.get();
    if run {
        for r in [a.flags(), b.flags(), c.flags(), d.flags()] {
            r.consume_mask_dirty();
        }
    }
    run
}

/// W10（O 0x1409014e0）。返回 `Some(world_change_tick)`（A/B/C/D 的 `changed_tick` 被刷新）或 `None`（未运行）。
pub fn compute_path_masks(env: &mut PassEnv<'_>, world_change_tick: u32) -> Option<u32> {
    if !should_run_and_consume(env) {
        return None;
    }
    // 4. combine
    let combine = env
        .pass(COMBINE_NAME, COMBINE_URI)
        .bind_texture(binding_hash("path_mask"), &TexturePreset(1), TEX_READ)
        .bind_texture(
            binding_hash("cobblestone1_mask"),
            &TexturePreset(2),
            TEX_READ,
        )
        .bind_texture(
            binding_hash("cobblestone2_mask"),
            &TexturePreset(3),
            TEX_READ,
        )
        .bind_texture(
            binding_hash("cobblestone3_mask"),
            &TexturePreset(4),
            TEX_READ,
        )
        .bind_texture(
            binding_hash("paths_exclusion"),
            &TexturePreset(5),
            TEX_WRITE,
        )
        .dispatch_2d(GRID, GRID);
    env.record(combine);

    // 5. 临时纹理
    let (tex0, tex1) = create_blur_temps(env);

    // 6. directional contraction：preset 5 → temp1
    let contract = env
        .pass(CONTRACT_NAME, CONTRACT_URI)
        .bind_texture(HASH_TERRAIN, &TexturePreset(9), TEX_READ)
        .bind_texture(HASH_VISUAL_TERRAIN, &TexturePreset(0xa), TEX_READ)
        .bind_texture(binding_hash("terrain_sdg"), &TexturePreset(0xd), TEX_READ)
        .bind_texture(binding_hash("path_mask"), &TexturePreset(5), TEX_READ)
        .bind_texture(HASH_IMG_OUT, &&tex1, TEX_WRITE)
        .dispatch_2d(GRID, GRID);
    env.record(contract);

    // 7. 水平 temp1 → temp0；垂直 temp0 → preset 5
    blur_stroke(
        env,
        true,
        |p| p.bind_texture(HASH_IMG_IN, &&tex1, TEX_READ),
        |p| p.bind_texture(HASH_IMG_OUT, &&tex0, TEX_WRITE),
    );
    blur_stroke(
        env,
        false,
        |p| p.bind_texture(HASH_IMG_IN, &&tex0, TEX_READ),
        |p| p.bind_texture(HASH_IMG_OUT, &TexturePreset(5), TEX_WRITE),
    );

    // 8. 四个 blur_path_mask
    let blurred = [
        blur_path_mask(env, 1),
        blur_path_mask(env, 2),
        blur_path_mask(env, 3),
        blur_path_mask(env, 4),
    ];
    let [b1, b2, b3, b4] = blurred;

    // 9. visual
    let visual = env
        .pass(VISUAL_NAME, VISUAL_URI)
        .bind_texture(binding_hash("water_mask"), &TexturePreset(0x13), TEX_READ)
        .bind_texture(binding_hash("path_mask_blurred"), &&b1, TEX_READ)
        .bind_texture(binding_hash("cobblestone1_mask_blurred"), &&b2, TEX_READ)
        .bind_texture(binding_hash("cobblestone2_mask_blurred"), &&b3, TEX_READ)
        .bind_texture(binding_hash("cobblestone3_mask_blurred"), &&b4, TEX_READ)
        .bind_texture(HASH_TERRAIN, &TexturePreset(9), TEX_READ)
        .bind_texture(HASH_VISUAL_TERRAIN, &TexturePreset(0xa), TEX_READ)
        .bind_texture(binding_hash("paths_visual"), &TexturePreset(6), TEX_WRITE)
        .bind_texture(binding_hash("dirt_mask"), &TexturePreset(7), TEX_WRITE)
        .dispatch_2d(GRID, GRID);
    env.record(visual);
    Some(world_change_tick)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;
    use crate::systems::render_loop::compute_passes::upload_world_raster::RasterFlags;

    #[derive(Clone, Copy, Default)]
    struct St {
        a: bool,
        b: bool,
        c: bool,
        d: bool,
        e: (u64, u64),
    }

    fn all_hashes() -> Vec<u64> {
        [
            "path_mask",
            "cobblestone1_mask",
            "cobblestone2_mask",
            "cobblestone3_mask",
            "paths_exclusion",
            "terrain_sdg",
            "terrain_heightmap",
            "visual_terrain_heightmap",
            "img_in",
            "img_out",
            "water_mask",
            "path_mask_blurred",
            "cobblestone1_mask_blurred",
            "cobblestone2_mask_blurred",
            "cobblestone3_mask_blurred",
            "paths_visual",
            "dirt_mask",
        ]
        .iter()
        .map(|n| binding_hash(n))
        .collect()
    }

    fn world(st: St) -> TestWorld {
        TestWorld::new(
            Reflection::new()
                .bind(&all_hashes())
                .uniforms(&[KEY_HORIZONTAL, KEY_SIGMA]),
        )
        .with_resource(PathRaster(RasterFlags::new(false, st.a)))
        .with_resource(Cobblestone1Raster(RasterFlags::new(false, st.b)))
        .with_resource(Cobblestone2Raster(RasterFlags::new(false, st.c)))
        .with_resource(Cobblestone3Raster(RasterFlags::new(false, st.d)))
        .with_resource(PathStrokeQueues::with_words(st.e.0, st.e.1))
    }

    fn raster<R: ResourceView>(w: &TestWorld) -> &R {
        use crate::render::pass_builder::pass_resources::ResourceLookup;
        unsafe { &*(w.get_resource_ptr(R::TYPE_ID).unwrap() as *const R) }
    }

    fn run(st: St) -> (TestWorld, TestFrame, Option<u32>) {
        let w = world(st);
        let mut f = TestFrame::default();
        let r = with_env(&w, &mut f, |env| compute_path_masks(env, 99));
        (w, f, r)
    }

    #[test]
    fn nothing_dirty_does_nothing() {
        let (w, f, r) = run(St::default());
        assert_eq!(r, None);
        assert!(f.passes.is_empty() && f.textures.is_empty());
        assert_eq!(w.backend.loads.get(), 0);
    }

    #[test]
    fn each_trigger_alone_runs_the_pipeline() {
        for st in [
            St {
                a: true,
                ..Default::default()
            },
            St {
                b: true,
                ..Default::default()
            },
            St {
                c: true,
                ..Default::default()
            },
            St {
                d: true,
                ..Default::default()
            },
            St {
                e: (1, 0),
                ..Default::default()
            },
            St {
                e: (0, 7),
                ..Default::default()
            },
        ] {
            let (_, f, r) = run(st);
            assert_eq!(r, Some(99));
            assert!(!f.passes.is_empty());
        }
    }

    #[test]
    fn full_pass_order_and_counts() {
        let (_, f, _) = run(St {
            a: true,
            ..Default::default()
        });
        let mut want = vec![
            "Combine path exlusion masks",
            "Directional contraction",
            "Blur Path Stroke",
            "Blur Path Stroke",
        ];
        for _ in 0..4 {
            want.extend(["Blur Path Stroke", "Blur Path Stroke"]);
        }
        want.push("Path Mask: Visual");
        assert_eq!(f.order, want);
        assert_eq!(f.passes.len(), 13);
        // 临时纹理：compute_path_masks 自己 2 个 + 每个 blur_path_mask 2 个。
        assert_eq!(f.textures.len(), 2 + 4 * 2);
        assert!(
            f.textures
                .iter()
                .all(|t| t.1 == TextureDesc::new_2d([512, 512], 0x4c).mip_levels(1))
        );
        assert_eq!(f.textures[0].0, "blur temp 0");
        assert_eq!(f.textures[1].0, "blur temp 1");
        for p in &f.passes {
            assert!(matches!(
                p.dispatch,
                DispatchMode::Threads {
                    x: 512,
                    y: 512,
                    z: 1
                }
            ));
        }
    }

    #[test]
    fn all_four_rasters_are_consumed_even_if_only_one_triggered() {
        let (w, _, _) = run(St {
            a: true,
            d: false,
            ..Default::default()
        });
        // D 不是触发原因但仍被清（O：无条件清 A/B/C/D）。
        for r in [
            &raster::<PathRaster>(&w).0,
            &raster::<Cobblestone1Raster>(&w).0,
            &raster::<Cobblestone2Raster>(&w).0,
            &raster::<Cobblestone3Raster>(&w).0,
        ] {
            assert!(!r.mask_dirty.get());
            assert!(r.dirty_rows.iter().all(|b| b.get() == 0));
        }
    }

    #[test]
    fn blur_wiring_contract_to_preset5_and_blur_path_mask_returns_temp1() {
        let (_, f, _) = run(St {
            b: true,
            ..Default::default()
        });
        // 指令流：0 combine，1 contraction，2 blur H，3 blur V，4.. blur_path_mask。
        let acc = |i: usize| {
            f.passes[i]
                .builder
                .accesses
                .iter()
                .map(|a| (a.access, a.resource))
                .collect::<Vec<_>>()
        };
        // contraction 写的 temp1 = blur H 读的资源；blur H 写的 temp0 = blur V 读的资源。
        let contract_w = acc(1).last().unwrap().1;
        let h = acc(2);
        assert_eq!(h[1], (0x19, contract_w));
        let h_w = h[2].1;
        let v = acc(3);
        assert_eq!(v[1], (0x19, h_w));
        // blur V 最终写 preset 5：与 combine 写 preset 5 是同一个资源。
        assert_eq!(v[2], (0x32, acc(0).last().unwrap().1));
        // blur_path_mask #1：水平读 preset 1（= combine 读的第 1 个资源）。
        assert_eq!(acc(4)[1].1, acc(0)[1].1);
        // visual 的 4 个模糊输入依次是各 blur_path_mask 的 temp1（= 垂直 pass 写入的资源）。
        let vis = acc(12);
        for k in 0..4 {
            let blur_v_write = acc(5 + 2 * k)[2].1;
            assert_eq!(vis[2 + k], (0x19, blur_v_write), "blur result {k}");
        }
        // visual 末两项写（preset 6、7）。
        assert_eq!(vis[vis.len() - 2].0, 0x32);
        assert_eq!(vis[vis.len() - 1].0, 0x32);
        // 水平/垂直标志。
        assert_eq!(uniform_u32(&f.passes[2], 0), 1);
        assert_eq!(uniform_u32(&f.passes[3], 0), 0);
        assert_eq!(f32::from_bits(uniform_u32(&f.passes[2], 16)), 2.0);
    }

    #[test]
    fn standalone_blur_path_mask_returns_the_vertical_target() {
        let w = world(St::default());
        let mut f = TestFrame::default();
        let t = with_env(&w, &mut f, |env| blur_path_mask(env, 3));
        assert_eq!(f.textures.len(), 2);
        assert_eq!(f.passes.len(), 2);
        assert_eq!(t.resource, f.passes[1].builder.accesses[2].resource);
        assert_ne!(t.resource, f.passes[0].builder.accesses[2].resource);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_queue_resource_unwraps_before_any_decision() {
        let w = TestWorld::new(Reflection::new())
            .with_resource(PathRaster(RasterFlags::new(false, true)))
            .with_resource(Cobblestone1Raster(RasterFlags::new(false, false)))
            .with_resource(Cobblestone2Raster(RasterFlags::new(false, false)))
            .with_resource(Cobblestone3Raster(RasterFlags::new(false, false)));
        let mut f = TestFrame::default();
        with_env(&w, &mut f, |env| compute_path_masks(env, 0));
    }

    #[test]
    fn constants_match_machine_code() {
        assert_eq!(HASH_IMG_IN, (-0x27d285f5246e9b3e_i64) as u64);
        assert_eq!(HASH_IMG_OUT, 0x1ca51a73373208db);
        assert_eq!(KEY_HORIZONTAL, (-0x26e007c8df6b43de_i64) as u64);
        assert_eq!(
            binding_hash("terrain_sdg"),
            (-0x54dbd78cbb54573f_i64) as u64
        );
        assert_eq!(
            binding_hash("paths_exclusion"),
            (-0x4a1e510a3c5cc65c_i64) as u64
        );
        assert_eq!(
            binding_hash("cobblestone1_mask"),
            (-0x6decae1bf9404ad7_i64) as u64
        );
        assert_eq!(
            binding_hash("path_mask_blurred"),
            (-0x72e0669636f2acb8_i64) as u64
        );
        assert_eq!(
            binding_hash("paths_visual"),
            (-0x280b7ecfc4efb544_i64) as u64
        );
        assert_eq!(binding_hash("dirt_mask"), 0x3ec9710e6411d8d1);
        assert_eq!(
            PathStrokeQueues::TYPE_ID.lo,
            (-0x325950d09504ecf2_i64) as u64
        );
        assert_eq!(core::mem::offset_of!(PathStrokeQueues, first_word), 0x10);
        assert_eq!(core::mem::offset_of!(PathStrokeQueues, second_word), 0x30);
        assert_eq!(COMBINE_NAME.len(), 0x1b);
        assert_eq!(COMBINE_URI.len(), 0x26);
        assert_eq!(CONTRACT_NAME.len(), 0x17);
        assert_eq!(CONTRACT_URI.len(), 0x23);
        assert_eq!(VISUAL_NAME.len(), 0x11);
        assert_eq!(VISUAL_URI.len(), 0x1f);
        assert_eq!(BLUR_NAME.len(), 0x10);
        assert_eq!(BLUR_URI.len(), 0x18);
        assert_eq!(TEMP0_NAME.len(), 0xb);
    }
}
