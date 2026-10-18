//! `compute_path_masks.rs` — W10 `compute_path_masks`（O 0x1409014e0，7779 B）与 `blur_path_mask`（O 0x140903350，2364 B）。
//!
//! ## compute_path_masks（参数 O：`(ctx, world)`）
//! 1. 依次 `get_resource_id`（任一缺失 → unwrap panic，**全部先于任何判断**）：
//!    A = canonical PathRaster、B = Cobblestone1Raster、C = Cobblestone2Raster、D = Cobblestone3Raster、
//!    E = `Events<RecalculateHeightMapCmd>`（TypeId 0xcda6af2f6afb130e/0x036e88a927bd70aa；只读 `+0x10`、`+0x30`）。
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
use crate::resources::texture_presets::TexturePreset;
use crate::resources::world_raster_defs::{
    Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster, PathRaster,
};
use crate::rhapsody_types::TextureDesc;
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, TEX_READ, TEX_WRITE, binding_hash,
};
use crate::systems::render_loop::compute_passes::upload_world_raster::LiveRasterKind;
use bevy_ecs::change_detection::{DetectChangesMut, Mut};

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

/// Native lookup/gate/consume uses the real four ECS owners. The host must
/// resolve all four plus Events<RecalculateHeightMapCmd> before this function,
/// including on a clean frame. Bypass reads do not mark a skipped frame changed.
fn should_run_and_consume(
    a: &mut Mut<'_, PathRaster>,
    b: &mut Mut<'_, Cobblestone1Raster>,
    c: &mut Mut<'_, Cobblestone2Raster>,
    d: &mut Mut<'_, Cobblestone3Raster>,
    recalculate_event_nonempty: bool,
) -> bool {
    let run = a.bypass_change_detection().reupload_pending()
        || b.bypass_change_detection().reupload_pending()
        || c.bypass_change_detection().reupload_pending()
        || recalculate_event_nonempty
        || d.bypass_change_detection().reupload_pending();
    if !run {
        return false;
    }
    fn consume<R: LiveRasterKind>(r: &mut Mut<'_, R>) {
        r.set_changed();
        r.bypass_change_detection().consume_reupload();
    }
    // Original 1409017b5..1847 marks each actual owner and clears +29b plus
    // all512 bitmap bytes, including D when another source caused the run.
    consume(a);
    consume(b);
    consume(c);
    consume(d);
    true
}

/// Original W10 thirteen real compute passes, with canonical raster state.
/// State consumption precedes pass construction. A later declaration failure
/// cancels the frame; it does not roll the native dirty bitmap back.
pub fn compute_path_masks(
    env: &mut PassEnv<'_>,
    a: &mut Mut<'_, PathRaster>,
    b: &mut Mut<'_, Cobblestone1Raster>,
    c: &mut Mut<'_, Cobblestone2Raster>,
    d: &mut Mut<'_, Cobblestone3Raster>,
    recalculate_event_nonempty: bool,
    world_change_tick: u32,
) -> Option<u32> {
    if !should_run_and_consume(a, b, c, d, recalculate_event_nonempty) {
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
    use crate::resources::world_raster::DirtyTilePublisher;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;
    use bevy_ecs::{change_detection::DetectChanges, world::World};
    fn owners() -> World {
        let mut w = World::new();
        w.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
        w.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
        w.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
        w.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
        w
    }
    fn run(w: &mut World, event: bool) -> (Option<u32>, TestFrame) {
        let hashes = [
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
        .map(binding_hash);
        let view = TestWorld::new(
            Reflection::new()
                .bind(&hashes)
                .uniforms(&[KEY_HORIZONTAL, KEY_SIGMA]),
        );
        let tick = w.read_change_tick().get();
        let mut frame = TestFrame::default();
        let result = w.resource_scope(|w, mut a: Mut<PathRaster>| {
            w.resource_scope(|w, mut b: Mut<Cobblestone1Raster>| {
                w.resource_scope(|w, mut c: Mut<Cobblestone2Raster>| {
                    w.resource_scope(|_, mut d: Mut<Cobblestone3Raster>| {
                        with_env(&view, &mut frame, |env| {
                            compute_path_masks(env, &mut a, &mut b, &mut c, &mut d, event, tick)
                        })
                    })
                })
            })
        });
        (result, frame)
    }
    fn changed(w: &World) -> [bevy_ecs::component::Tick; 4] {
        [
            w.get_resource_ref::<PathRaster>().unwrap().last_changed(),
            w.get_resource_ref::<Cobblestone1Raster>()
                .unwrap()
                .last_changed(),
            w.get_resource_ref::<Cobblestone2Raster>()
                .unwrap()
                .last_changed(),
            w.get_resource_ref::<Cobblestone3Raster>()
                .unwrap()
                .last_changed(),
        ]
    }
    #[test]
    fn clean_skip_does_not_change_or_consume_the_canonical_owners() {
        let mut w = owners();
        w.resource_mut::<Cobblestone3Raster>().dirty_mut().set_all();
        let before = changed(&w);
        w.increment_change_tick();
        let (result, frame) = run(&mut w, false);
        assert!(result.is_none());
        assert!(frame.passes.is_empty());
        assert_eq!(changed(&w), before);
        assert!(
            !w.resource::<Cobblestone3Raster>().dirty().is_empty(),
            "bitmap alone is not the native gate"
        );
    }
    #[test]
    fn fourth_flag_alone_and_recalculate_event_each_run_thirteen_passes_and_consume_all_four() {
        for event in [false, true] {
            let mut w = owners();
            w.resource_mut::<PathRaster>().dirty_mut().set_all();
            w.resource_mut::<Cobblestone3Raster>().dirty_mut().set_all();
            if !event {
                w.resource_mut::<Cobblestone3Raster>()
                    .mark_staging_uploaded();
            }
            w.increment_change_tick();
            let tick = w.read_change_tick();
            let (result, frame) = run(&mut w, event);
            assert_eq!(result, Some(tick.get()));
            assert_eq!(changed(&w), [tick; 4]);
            assert!(!w.resource::<Cobblestone3Raster>().reupload_pending());
            assert!(w.resource::<PathRaster>().dirty().is_empty());
            assert!(w.resource::<Cobblestone3Raster>().dirty().is_empty());
            assert_eq!(frame.passes.len(), 13);
            let names: Vec<_> = frame.passes.iter().map(|p| p.name).collect();
            assert_eq!(names[0], COMBINE_NAME);
            assert_eq!(names[1], CONTRACT_NAME);
            assert!(names[2..12].iter().all(|x| *x == BLUR_NAME));
            assert_eq!(names[12], VISUAL_NAME);
        }
    }
}
