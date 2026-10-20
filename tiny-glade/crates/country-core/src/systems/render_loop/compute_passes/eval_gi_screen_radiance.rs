//! `eval_gi_screen_radiance.rs` — 屏幕辐射度求值 / 重投影 / 滤波（W39，0x140a44cd0，7066 B）。
//!
//! 机器码（O）：入参 `(recorder, world, half_depth, half_normal, ssgi, restir: &RestirTextures, reprojection)`
//! （`restir` 是 3 个 0x24 B 句柄：`+0` radiance、`+0x24` ray_dir、`+0x48` reservoir）。
//! 1. `half = extent_2d(half_depth.desc)`；`create_raw_resource("Screen indirect radiance")`（名长 0x18）：`new_2d((half.w * 3, half.h), 0x61)`
//!    （宽 ×3：每像素三个通道平铺）。
//! 2. **"Eval screen radiance"** `/gi/eval_screen_radiance.cs.hlsl`（名长 0x14、URI 长 0x20）：`bool_render_shadows = true` →
//!    `shadow_map`（预设 0x11）→ `half_depth_tex`、`half_view_normal_tex`、`ssgi_tex`、`restir_radiance_tex`、`restir_ray_dir_tex`、
//!    `restir_reservoir_tex`（读 0x19）→ `blue_noise_texture`（磁盘）→ `output_tex`（写 0x32）→ `dispatch_2d(half)`。
//! 3. 两个 temporal 对：`"Filtered indirect radiance"`（名长 0x1a）`new_2d((3w, h), 0x61).sampled().storage()`；
//!    `"Filtered indirect stats"`（名长 0x17）`new_2d(half, 0x25).sampled().storage()`（`Err` → `Result::unwrap` panic）。
//!    `history_valid = (radiance_pair.flag & 1) ? radiance_pair.flag : 0`。
//! 4. world 资源 `ShaderRefactoringState`（TypeId `0xc95ed87f413070ab / 0xe67d7e3df983f8ce`，缺失 → `Option::unwrap`），读 `u32` 判别式。
//! 5. **"Reproject GI"** `/gi/reproject_screen_radiance.cs.hlsl`（名长 0xc、URI 长 0x25）：`bool_render_shadows = true` → `shadow_map` →
//!    `history_valid`（i32）→ `allow_non_determinism = (state == 0)`（i32）→ `half_depth_tex`、`half_view_normal_tex`、`reprojection_tex`、
//!    `history_tex`（辐射度历史）、`temporal_stats_tex`（统计历史）（读）→ `temporal_stats_out_tex`（写 0x32，统计本帧）→
//!    `output_tex`（**写 0x37**，= "Screen indirect radiance"）→ `dispatch_2d(half)`。
//! 6. **"Filter GI"** `/gi/blur_screen_radiance.cs.hlsl`（名长 9、URI 长 0x20）：`bool_render_shadows = true` → `shadow_map` →
//!    `half_depth_tex`、`half_view_normal_tex`、`ssgi_tex`、`reprojection_tex`、`input_tex`（= Screen indirect radiance）（读）→
//!    `output_tex`（写 0x32，= 辐射度本帧）→ `temporal_stats_out_tex`（**写 0x37**）→ `dispatch_2d(half)`。
//! 7. 返回辐射度本帧句柄。
//!
//! 偏差/U：`ShaderRefactoringState` 判别式 0 的名字（I：Disabled）；`from(desc)` 的用途位。

use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resource_access::{RgTexture, WriteTextureUnchecked};
use crate::render::pass_builder::pass_resources::TypeId128;
use crate::resources::texture_presets::TexturePreset;
use crate::rhapsody_types::TextureDesc;
use crate::systems::render_loop::compute_passes::pass_kit::{
    BLUE_NOISE, PassEnv, binding, binding_hash, extent_of,
};
use crate::systems::render_loop::compute_passes::ray_trace_gi::RestirTextures;

pub const RADIANCE_NAME: &str = "Screen indirect radiance";
pub const FILTERED_RADIANCE_NAME: &str = "Filtered indirect radiance";
pub const FILTERED_STATS_NAME: &str = "Filtered indirect stats";
pub const EVAL: (&str, &str) = ("Eval screen radiance", "/gi/eval_screen_radiance.cs.hlsl");
pub const REPROJECT: (&str, &str) = ("Reproject GI", "/gi/reproject_screen_radiance.cs.hlsl");
pub const FILTER: (&str, &str) = ("Filter GI", "/gi/blur_screen_radiance.cs.hlsl");
pub const RADIANCE_FORMAT: u32 = 0x61;
pub const STATS_FORMAT: u32 = 0x25;
pub const TEMPORAL_STATS_TEX: u64 = binding_hash("temporal_stats_tex");
pub const TEMPORAL_STATS_OUT_TEX: u64 = binding_hash("temporal_stats_out_tex");
pub const ALLOW_NON_DETERMINISM: u64 = binding_hash("allow_non_determinism");
/// `ShaderRefactoringState` 的 TypeId（O）。
pub const SHADER_REFACTORING_STATE_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0xc95e_d87f_4130_70ab,
    hi: 0xe67d_7e3d_f983_f8ce,
};

pub struct GiScreenRadiance {
    pub output: RgTexture,
    pub eval: RecordedComputePass,
    pub reproject: RecordedComputePass,
    pub filter: RecordedComputePass,
}

/// W39：屏幕辐射度三连（求值 → 重投影 → 滤波）。`refactoring_state` = `ShaderRefactoringState` 的 u32 判别式。
pub fn eval_gi_screen_radiance(
    env: &mut PassEnv<'_>,
    half_depth: &RgTexture,
    half_normal: &RgTexture,
    ssgi: &RgTexture,
    restir: &RestirTextures,
    reprojection: &RgTexture,
    refactoring_state: u32,
) -> GiScreenRadiance {
    let half = extent_of(half_depth);
    let wide = [half.width * 3, half.height];
    let radiance = env.create_texture(RADIANCE_NAME, TextureDesc::new_2d(wide, RADIANCE_FORMAT));

    let eval = env
        .pass(EVAL.0, EVAL.1)
        .uniform(binding::BOOL_RENDER_SHADOWS, true)
        .bind_texture(binding::SHADOW_MAP, &TexturePreset(0x11), 0x19)
        .bind_texture(binding::HALF_DEPTH_TEX, &half_depth, 0x19)
        .bind_texture(binding::HALF_VIEW_NORMAL_TEX, &half_normal, 0x19)
        .bind_texture(binding::SSGI_TEX, &ssgi, 0x19)
        .bind_texture(binding::RESTIR_RADIANCE_TEX, &&restir.radiance, 0x19)
        .bind_texture(binding::RESTIR_RAY_DIR_TEX, &&restir.ray_dir, 0x19)
        .bind_texture(binding::RESTIR_RESERVOIR_TEX, &&restir.reservoir, 0x19)
        .bind_texture(binding::BLUE_NOISE_TEXTURE, &BLUE_NOISE, 0x19)
        .bind_texture(binding::OUTPUT_TEX, &WriteTextureUnchecked(radiance), 0x32)
        .dispatch_2d(half.width, half.height);

    let temporal = |extent, format| TextureDesc::new_2d(extent, format).storage().sampled();
    let rad_pair = env.temporal_pair(FILTERED_RADIANCE_NAME, temporal(wide, RADIANCE_FORMAT));
    let stats_pair = env.temporal_pair(
        FILTERED_STATS_NAME,
        temporal([half.width, half.height], STATS_FORMAT),
    );
    let history_valid = rad_pair.fresh as i32;

    let reproject = env
        .pass(REPROJECT.0, REPROJECT.1)
        .uniform(binding::BOOL_RENDER_SHADOWS, true)
        .bind_texture(binding::SHADOW_MAP, &TexturePreset(0x11), 0x19)
        .uniform(binding::HISTORY_VALID, history_valid)
        .uniform(ALLOW_NON_DETERMINISM, (refactoring_state == 0) as i32)
        .bind_texture(binding::HALF_DEPTH_TEX, &half_depth, 0x19)
        .bind_texture(binding::HALF_VIEW_NORMAL_TEX, &half_normal, 0x19)
        .bind_texture(binding::REPROJECTION_TEX, &reprojection, 0x19)
        .bind_texture(binding::HISTORY_TEX, &&rad_pair.previous, 0x19)
        .bind_texture(TEMPORAL_STATS_TEX, &&stats_pair.previous, 0x19)
        .bind_texture(
            TEMPORAL_STATS_OUT_TEX,
            &WriteTextureUnchecked(stats_pair.current),
            0x32,
        )
        .bind_texture(binding::OUTPUT_TEX, &WriteTextureUnchecked(radiance), 0x37);
    // 原版 reproject_screen_radiance SPV：`history_valid != 0 && frame_index != 0`
    // 不成立时只写统计并提前返回，history_tex / temporal_stats_tex 的全部采样
    // 都在该分支之后；保留原绑定与同步，仅声明无效历史下这两个读不要求内容。
    let reproject = if history_valid == 0 {
        reproject
            .inactive_texture_contents(binding::HISTORY_TEX)
            .inactive_texture_contents(TEMPORAL_STATS_TEX)
    } else {
        reproject
    };
    let reproject = reproject.dispatch_2d(half.width, half.height);

    let filter = env
        .pass(FILTER.0, FILTER.1)
        .uniform(binding::BOOL_RENDER_SHADOWS, true)
        .bind_texture(binding::SHADOW_MAP, &TexturePreset(0x11), 0x19)
        .bind_texture(binding::HALF_DEPTH_TEX, &half_depth, 0x19)
        .bind_texture(binding::HALF_VIEW_NORMAL_TEX, &half_normal, 0x19)
        .bind_texture(binding::SSGI_TEX, &ssgi, 0x19)
        .bind_texture(binding::REPROJECTION_TEX, &reprojection, 0x19)
        .bind_texture(binding::INPUT_TEX, &&radiance, 0x19)
        .bind_texture(
            binding::OUTPUT_TEX,
            &WriteTextureUnchecked(rad_pair.current),
            0x32,
        )
        .bind_texture(
            TEMPORAL_STATS_OUT_TEX,
            &WriteTextureUnchecked(stats_pair.current),
            0x37,
        )
        .dispatch_2d(half.width, half.height);

    GiScreenRadiance {
        output: rad_pair.current,
        eval,
        reproject,
        filter,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        Env, accesses, env, rgtex, temporal_key, tex_key, threads,
    };

    fn names() -> Vec<u64> {
        vec![
            binding::SHADOW_MAP,
            binding::HALF_DEPTH_TEX,
            binding::HALF_VIEW_NORMAL_TEX,
            binding::SSGI_TEX,
            binding::RESTIR_RADIANCE_TEX,
            binding::RESTIR_RAY_DIR_TEX,
            binding::RESTIR_RESERVOIR_TEX,
            binding::BLUE_NOISE_TEXTURE,
            binding::OUTPUT_TEX,
            binding::REPROJECTION_TEX,
            binding::HISTORY_TEX,
            TEMPORAL_STATS_TEX,
            TEMPORAL_STATS_OUT_TEX,
            binding::INPUT_TEX,
        ]
    }

    fn run(state: u32, valid: bool) -> (Env, GiScreenRadiance) {
        let mut e = env(
            &names(),
            &[
                binding::BOOL_RENDER_SHADOWS,
                binding::HISTORY_VALID,
                ALLOW_NON_DETERMINISM,
            ],
        );
        e.frame.pair_fresh = valid;
        let r = RestirTextures {
            radiance: rgtex(20, 10, 10),
            ray_dir: rgtex(21, 10, 10),
            reservoir: rgtex(22, 10, 10),
        };
        let out = e.run(|env| {
            eval_gi_screen_radiance(
                env,
                &rgtex(1, 480, 270),
                &rgtex(2, 480, 270),
                &rgtex(3, 480, 270),
                &r,
                &rgtex(4, 960, 540),
                state,
            )
        });
        (e, out)
    }

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(TEMPORAL_STATS_TEX, 0x2987_7799_16aa_cac9);
        assert_eq!(TEMPORAL_STATS_OUT_TEX, 0xf420_e28a_2fe7_d842);
        assert_eq!(ALLOW_NON_DETERMINISM, 0xb0e7_432e_3c23_2f38);
        assert_eq!(binding::RESTIR_RADIANCE_TEX, 0xfe8d_a59f_9774_9daa);
        assert_eq!(binding::RESTIR_RAY_DIR_TEX, 0x0d36_dd99_2ba2_9df1);
        assert_eq!(binding::RESTIR_RESERVOIR_TEX, 0x4b1b_8bd2_36f1_4df0);
        assert_eq!(binding::HISTORY_TEX, 0x185a_603c_077d_ad19);
    }

    #[test]
    fn radiance_is_three_times_wider_than_half_res() {
        let (e, _) = run(0, true);
        let t = &e.frame.textures[0];
        assert_eq!(tex_key(t), (RADIANCE_NAME, [1440, 270], 0x61));
        let tmp: Vec<_> = e.frame.temporal_pairs.iter().map(temporal_key).collect();
        assert_eq!(
            tmp,
            [
                (FILTERED_RADIANCE_NAME, [1440, 270], 0x61),
                (FILTERED_STATS_NAME, [480, 270], 0x25)
            ]
        );
    }

    #[test]
    fn all_three_passes_dispatch_half_extent_in_order() {
        let (e, out) = run(0, true);
        assert_eq!(
            (out.eval.name, out.reproject.name, out.filter.name),
            ("Eval screen radiance", "Reproject GI", "Filter GI")
        );
        for p in [&out.eval, &out.reproject, &out.filter] {
            assert_eq!(threads(p), (480, 270, 1));
        }
        assert_eq!(e.world.backend.paths.borrow().len(), 3);
        // blue_noise 是磁盘纹理（外部描述符，不登记 RG 访问）。
        assert_eq!(out.eval.external_bindings.len(), 1);
    }

    #[test]
    fn allow_non_determinism_is_state_equals_zero() {
        let (e, out) = run(0, true);
        assert_eq!(e.uniform_u32(&out.reproject, ALLOW_NON_DETERMINISM), 1);
        for s in [1u32, 2, 9] {
            let (e, out) = run(s, true);
            assert_eq!(
                e.uniform_u32(&out.reproject, ALLOW_NON_DETERMINISM),
                0,
                "state {s}"
            );
        }
    }

    #[test]
    fn history_valid_follows_radiance_pair_flag() {
        assert_eq!(
            {
                let (e, o) = run(0, true);
                e.uniform_u32(&o.reproject, binding::HISTORY_VALID)
            },
            1
        );
        assert_eq!(
            {
                let (e, o) = run(0, false);
                e.uniform_u32(&o.reproject, binding::HISTORY_VALID)
            },
            0
        );
    }

    #[test]
    fn write_access_codes_match_the_exe() {
        let (_e, out) = run(0, true);
        // reproject：…读 ×5，stats_out 写 0x32，output 写 0x37
        let a = accesses(&out.reproject);
        assert_eq!(a[a.len() - 2].1, 0x32);
        assert_eq!(a[a.len() - 1].1, 0x37);
        // filter：output 写 0x32，stats_out 写 0x37
        let b = accesses(&out.filter);
        assert_eq!((b[b.len() - 2].1, b[b.len() - 1].1), (0x32, 0x37));
        // 本帧辐射度句柄 = filter 的 output
        assert_eq!(b[b.len() - 2].0, out.output.resource);
    }
}
