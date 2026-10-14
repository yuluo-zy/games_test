//! `ssgi.rs` — 屏幕空间 GI（W37，`calculate_ssgi` 0x1408743b0，4903 B）：三个 compute pass + 时间滤波历史。
//!
//! 机器码（O）：入参 `(recorder, world, half_depth, half_normal, gbuffer, reprojection)`；`gbuffer` 同 `half_res::GBuffer`
//! （`+0` gbuffer、`+0x24` 深度）。
//! 1. `half = extent_2d(half_depth.desc)`；`create_raw_resource("ssgi")`（名长 4）：`new_2d(half, 0x4c)`。
//! 2. **"SSGI"** `/ssgi/ssgi.cs.hlsl`（名长 4、URI 长 0x12）：`depth_tex ← half_depth`、`half_view_normal_tex ← half_normal`（读 0x19）、
//!    `output_tex`（写 0x32）= ssgi 图 → `dispatch_2d(half)`。
//!    注意：这里 `depth_tex` 绑的是 **半分辨率** 深度（参数 `param_4`，不是 gbuffer 深度）。
//! 3. `create_raw_resource("ssgi_upsampled")`（名长 0xe）：`new_2d(extent_2d(gbuffer.depth.desc), 0x4c)`。
//!    **"SSGI upsample"** `/ssgi/upsample.cs.hlsl`（名长 0xd、URI 长 0x16）：`ssgi_tex ← ssgi 图`、`depth_tex ← gbuffer.depth`、
//!    `half_depth_tex`、`half_view_normal_tex`、`gbuffer_tex ← gbuffer.gbuffer`（读）、`output_tex`（写）= upsampled → `dispatch_2d(全分辨率)`。
//! 4. `create_raw_resource("ssgi_temporal")`（名长 0xd）：`new_2d(full, 9)`；`get_or_create_temporal_pair("ssgi_history")`（名长 0xc）：
//!    `new_2d(full, 9).sampled().storage()`；`Err` → `Result::unwrap` panic。
//!    **"SSGI temporal"** `/ssgi/temporal_filter.cs.hlsl`（名长 0xd、URI 长 0x1d）：uniform i32 `history_valid = 对标志字节`（`movzx`）→
//!    `input_tex ← upsampled`、`history_tex ← 历史`、`reprojection_tex`（读）、`final_output_tex`（写）= ssgi_temporal、
//!    `history_output_tex`（写）= 本帧历史 → `dispatch_2d(temporal extent)`。
//! 5. 返回 ssgi_temporal 句柄。

use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resource_access::{RgTexture, WriteTextureUnchecked};
use crate::rhapsody_types::TextureDesc;
use crate::systems::render_loop::compute_passes::half_res::GBuffer;
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, binding, binding_hash, dispatch_extent, extent_of,
};

pub const SSGI_NAME: &str = "ssgi";
pub const UPSAMPLED_NAME: &str = "ssgi_upsampled";
pub const TEMPORAL_NAME: &str = "ssgi_temporal";
pub const HISTORY_NAME: &str = "ssgi_history";
pub const PASS1: (&str, &str) = ("SSGI", "/ssgi/ssgi.cs.hlsl");
pub const PASS2: (&str, &str) = ("SSGI upsample", "/ssgi/upsample.cs.hlsl");
pub const PASS3: (&str, &str) = ("SSGI temporal", "/ssgi/temporal_filter.cs.hlsl");
pub const SSGI_FORMAT: u32 = 0x4c;
pub const TEMPORAL_FORMAT: u32 = 9;
pub const FINAL_OUTPUT_TEX: u64 = binding_hash("final_output_tex");
pub const HISTORY_OUTPUT_TEX: u64 = binding_hash("history_output_tex");

pub struct Ssgi {
    pub output: RgTexture,
    pub passes: [RecordedComputePass; 3],
}

fn spec(extent: [u32; 2], format: u32, storage_sampled: bool) -> TextureDesc {
    let d = TextureDesc::new_2d(extent, format);
    if storage_sampled {
        d.storage().sampled()
    } else {
        d
    }
}

/// W37：SSGI（半分辨率追踪 → 上采样 → 时间滤波）。
pub fn calculate_ssgi(
    env: &mut PassEnv<'_>,
    half_depth: &RgTexture,
    half_normal: &RgTexture,
    gbuffer: &GBuffer,
    reprojection: &RgTexture,
) -> Ssgi {
    let half = extent_of(half_depth);
    let ssgi = env.create_texture(
        SSGI_NAME,
        spec([half.width, half.height], SSGI_FORMAT, false),
    );
    let p1 = env
        .pass(PASS1.0, PASS1.1)
        .bind_texture(binding::DEPTH_TEX, &half_depth, 0x19)
        .bind_texture(binding::HALF_VIEW_NORMAL_TEX, &half_normal, 0x19)
        .bind_texture(binding::OUTPUT_TEX, &WriteTextureUnchecked(ssgi), 0x32);
    let p1 = dispatch_extent(p1, &ssgi);

    let full = extent_of(&gbuffer.depth);
    let up = env.create_texture(
        UPSAMPLED_NAME,
        spec([full.width, full.height], SSGI_FORMAT, false),
    );
    let p2 = env
        .pass(PASS2.0, PASS2.1)
        .bind_texture(binding::SSGI_TEX, &&ssgi, 0x19)
        .bind_texture(binding::DEPTH_TEX, &&gbuffer.depth, 0x19)
        .bind_texture(binding::HALF_DEPTH_TEX, &half_depth, 0x19)
        .bind_texture(binding::HALF_VIEW_NORMAL_TEX, &half_normal, 0x19)
        .bind_texture(binding::GBUFFER_TEX, &&gbuffer.gbuffer, 0x19)
        .bind_texture(binding::OUTPUT_TEX, &WriteTextureUnchecked(up), 0x32);
    let p2 = dispatch_extent(p2, &up);

    let temporal_out = env.create_texture(
        TEMPORAL_NAME,
        spec([full.width, full.height], TEMPORAL_FORMAT, false),
    );
    let pair = env.temporal_pair(
        HISTORY_NAME,
        spec([full.width, full.height], TEMPORAL_FORMAT, true),
    );
    let p3 = env
        .pass(PASS3.0, PASS3.1)
        .uniform(binding::HISTORY_VALID, pair.fresh as i32)
        .bind_texture(binding::INPUT_TEX, &&up, 0x19)
        .bind_texture(binding::HISTORY_TEX, &&pair.previous, 0x19)
        .bind_texture(binding::REPROJECTION_TEX, &reprojection, 0x19)
        .bind_texture(FINAL_OUTPUT_TEX, &WriteTextureUnchecked(temporal_out), 0x32)
        .bind_texture(
            HISTORY_OUTPUT_TEX,
            &WriteTextureUnchecked(pair.current),
            0x32,
        );
    let p3 = dispatch_extent(p3, &temporal_out);

    Ssgi {
        output: temporal_out,
        passes: [p1, p2, p3],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        accesses, env, rgtex, tex_key, threads,
    };

    fn run(
        valid: bool,
    ) -> (
        crate::systems::render_loop::compute_passes::pass_kit::test_env::Env,
        Ssgi,
    ) {
        let mut e = env(
            &[
                binding::DEPTH_TEX,
                binding::HALF_VIEW_NORMAL_TEX,
                binding::OUTPUT_TEX,
                binding::SSGI_TEX,
                binding::HALF_DEPTH_TEX,
                binding::GBUFFER_TEX,
                binding::INPUT_TEX,
                binding::HISTORY_TEX,
                binding::REPROJECTION_TEX,
                FINAL_OUTPUT_TEX,
                HISTORY_OUTPUT_TEX,
            ],
            &[binding::HISTORY_VALID],
        );
        e.frame.pair_fresh = valid;
        let g = GBuffer {
            gbuffer: rgtex(1, 1000, 600),
            depth: rgtex(2, 1000, 600),
        };
        let out = e.run(|env| {
            calculate_ssgi(
                env,
                &rgtex(3, 500, 300),
                &rgtex(4, 500, 300),
                &g,
                &rgtex(5, 1000, 600),
            )
        });
        (e, out)
    }

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(FINAL_OUTPUT_TEX, 0x805e_b02d_82ac_59bb);
        assert_eq!(HISTORY_OUTPUT_TEX, 0xaa75_015a_6ad6_1913);
        assert_eq!(binding::SSGI_TEX, 0xe003_e2bb_6439_9af9);
        assert_eq!(binding::GBUFFER_TEX, 0x41aa_4a0d_2dea_0b36);
    }

    #[test]
    fn resource_sizes_and_formats() {
        let (e, out) = run(false);
        let t = &e.frame.textures;
        assert_eq!(tex_key(&t[0]), ("ssgi", [500, 300], 0x4c));
        assert_eq!(tex_key(&t[1]), ("ssgi_upsampled", [1000, 600], 0x4c));
        assert_eq!(tex_key(&t[2]), ("ssgi_temporal", [1000, 600], 9));
        let (name, spec) = &e.frame.temporal_pairs[0];
        assert_eq!(
            (name.as_str(), spec.format, spec.storage, spec.sampled),
            (HISTORY_NAME, 9, true, true)
        );
        assert_eq!(out.output, t[2].2);
    }

    #[test]
    fn pass_chain_dispatch_extents_and_directions() {
        let (_e, out) = run(true);
        let [p1, p2, p3] = &out.passes;
        assert_eq!(
            (p1.name, p2.name, p3.name),
            ("SSGI", "SSGI upsample", "SSGI temporal")
        );
        assert_eq!(threads(p1), (500, 300, 1));
        assert_eq!(threads(p2), (1000, 600, 1));
        assert_eq!(threads(p3), (1000, 600, 1));
        // 第一遍 depth_tex 绑的是半分辨率深度（资源 3）
        assert_eq!(
            accesses(p1).iter().map(|a| a.0).take(2).collect::<Vec<_>>(),
            [3, 4]
        );
        assert_eq!(accesses(p2).len(), 6);
        assert_eq!(
            accesses(p3).iter().map(|a| a.1).collect::<Vec<_>>(),
            [0x19, 0x19, 0x19, 0x32, 0x32]
        );
    }

    #[test]
    fn history_valid_comes_from_the_temporal_pair_flag() {
        let (e, out) = run(true);
        assert_eq!(e.uniform_u32(&out.passes[2], binding::HISTORY_VALID), 1);
        let (e, out) = run(false);
        assert_eq!(e.uniform_u32(&out.passes[2], binding::HISTORY_VALID), 0);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn temporal_pair_failure_unwraps() {
        let mut e = env(&[], &[]);
        e.frame.fail_temporal = true;
        let g = GBuffer {
            gbuffer: rgtex(1, 8, 8),
            depth: rgtex(2, 8, 8),
        };
        e.run(|env| calculate_ssgi(env, &rgtex(3, 4, 4), &rgtex(4, 4, 4), &g, &rgtex(5, 8, 8)));
    }
}
