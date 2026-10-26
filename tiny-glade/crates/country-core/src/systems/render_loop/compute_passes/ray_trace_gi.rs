//! `ray_trace_gi.rs` — 软件 BVH 追踪 GI + ReSTIR 更新（W38，`ray_trace_gi` 0x140a01cf0，7850 B）。
//!
//! 机器码（O）：入参 `(recorder, world, gbuffer, half_depth, half_normal, taa_history, history_valid: u8, reprojection, ssgi)`；
//! `gbuffer` 同 `half_res::GBuffer`（`+0` gbuffer、`+0x24` 深度）。无 world 资源读取；结果 = 三个 ReSTIR 输出句柄（0x24 B ×3，
//! 分别在返回结构的 +0、+0x24、+0x48）。
//! 1. `half = div_up_extent(gbuffer.depth.desc, [2,2,1])`，`ext = extent_2d(half)`。
//!    `create_raw_resource("GI candidate radiance")`（名长 0x15）：`new_2d(ext, 0x61)`；
//!    `create_raw_resource("GI candidate hit")`（名长 0x10）：`new_2d(ext, 0x6d)`。
//! 2. **"Trace GI candidates"** `/gi/trace_gi_candidates.cs.hlsl`（名长 0x13、URI 长 0x1f）：uniform bool `bool_render_shadows = true` →
//!    `shadow_map`（预设 0x11）→ uniform i32 `history_valid = history_valid`（`movzx`）→ `depth_tex ← gbuffer.depth`、`taa_history_tex`、
//!    `reprojection_tex`、`ssgi_tex`、`half_depth_tex`、`half_view_normal_tex`（读 0x19）→ 预设 9 `terrain_heightmap`、10 `visual_terrain_heightmap`、
//!    8 `terrain_lighting_texture` → `blue_noise_texture`（磁盘）→ `bind_ray_tracing::<ComputePass>` →
//!    `candidate_radiance_tex`、`candidate_hit_tex`（写 0x32）→ `dispatch_2d(ext)`。
//! 3. 三对 temporal 纹理（`get_or_create_temporal_pair`，desc 均 `new_2d(ext, 格式).sampled().storage()`，`Err` → unwrap panic）：
//!    `"restir_radiance"`（名长 0xf，0x61）、`"restir_ray_dir"`（名长 0xe，0x61）、`"restir_reservoir"`（名长 0x10，0x65）。
//! 4. **"Update GI radiance"** `/gi/update_gi_radiance.cs.hlsl`（名长 0x12、URI 长 0x1e）：`bool_render_shadows = true` → `shadow_map` →
//!    `history_valid = (flag_ray_dir & flag_radiance & flag_reservoir & 1) != 0 ? history_valid : 0` → `gbuffer_tex ← gbuffer.gbuffer`、
//!    `depth_tex`、`taa_history_tex` → `restir_radiance_tex / restir_ray_dir_tex / restir_reservoir_tex ← 三个历史` → `reprojection_tex`、
//!    `ssgi_tex`、`candidate_radiance_tex`、`candidate_hit_tex`（读）→ 9、10、8、`blue_noise_texture` → `bind_ray_tracing` →
//!    `restir_radiance_out_tex / restir_ray_dir_out_tex / restir_reservoir_out_tex`（写 0x32，三个本帧句柄）→ `dispatch_2d(ext)`。
//!
//! 注意（O）：`bind_ray_tracing` 在两个 pass 里都在 `blue_noise_texture` 之后、输出绑定之前。
//! U：三个 `*_out_tex`/`candidate_*` 的 `new_2d` 之外的描述字段（`from(desc)` 的用途位）。

use crate::render::pass_builder::compute_pass_builder::{ComputePass, RecordedComputePass};
use crate::render::pass_builder::pass_resource_access::{RgTexture, WriteTextureUnchecked};
use crate::resources::texture_presets::TexturePreset;
use crate::rhapsody_types::TextureDesc;
use crate::systems::render_loop::compute_passes::half_res::{GBuffer, div_up_extent};
use crate::systems::render_loop::compute_passes::pass_kit::{
    BLUE_NOISE, PassEnv, binding, binding_hash, extent_of,
};

pub const CANDIDATE_RADIANCE_NAME: &str = "GI candidate radiance";
pub const CANDIDATE_HIT_NAME: &str = "GI candidate hit";
pub const TRACE: (&str, &str) = ("Trace GI candidates", "/gi/trace_gi_candidates.cs.hlsl");
pub const UPDATE: (&str, &str) = ("Update GI radiance", "/gi/update_gi_radiance.cs.hlsl");
pub const TEMPORAL_RADIANCE: &str = "restir_radiance";
pub const TEMPORAL_RAY_DIR: &str = "restir_ray_dir";
pub const TEMPORAL_RESERVOIR: &str = "restir_reservoir";
pub const CANDIDATE_RADIANCE_FORMAT: u32 = 0x61;
pub const CANDIDATE_HIT_FORMAT: u32 = 0x6d;
pub const RESERVOIR_FORMAT: u32 = 0x65;
pub const CANDIDATE_RADIANCE_TEX: u64 = binding_hash("candidate_radiance_tex");
pub const CANDIDATE_HIT_TEX: u64 = binding_hash("candidate_hit_tex");
pub const RESTIR_RADIANCE_OUT_TEX: u64 = binding_hash("restir_radiance_out_tex");
pub const RESTIR_RAY_DIR_OUT_TEX: u64 = binding_hash("restir_ray_dir_out_tex");
pub const RESTIR_RESERVOIR_OUT_TEX: u64 = binding_hash("restir_reservoir_out_tex");

/// ReSTIR 三张纹理（O：结构里依次 `+0` radiance、`+0x24` ray dir、`+0x48` reservoir，各 0x24 B）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RestirTextures {
    pub radiance: RgTexture,
    pub ray_dir: RgTexture,
    pub reservoir: RgTexture,
}

pub struct RayTraceGi {
    pub output: RestirTextures,
    pub trace: RecordedComputePass,
    pub update: RecordedComputePass,
}

/// 两个 pass 共用的“地形 + 光照 + 蓝噪声 + 光追资源”绑定尾巴（O：9 → 10 → 8 → blue_noise → bind_ray_tracing）。
fn bind_world_inputs(pass: ComputePass) -> ComputePass {
    let mut pass = pass
        .bind_texture(binding::TERRAIN_HEIGHTMAP, &TexturePreset(9), 0x19)
        .bind_texture(binding::VISUAL_TERRAIN_HEIGHTMAP, &TexturePreset(10), 0x19)
        .bind_texture(binding::TERRAIN_LIGHTING_TEXTURE, &TexturePreset(8), 0x19)
        .bind_texture(binding::BLUE_NOISE_TEXTURE, &BLUE_NOISE, 0x19);
    pass.bind_ray_tracing();
    pass
}

fn temporal_spec(extent: [u32; 2], format: u32) -> TextureDesc {
    TextureDesc::new_2d(extent, format).storage().sampled()
}

#[allow(clippy::too_many_arguments)]
/// W38：追踪候选 + 更新 ReSTIR 辐射度。
pub fn ray_trace_gi(
    env: &mut PassEnv<'_>,
    gbuffer: &GBuffer,
    half_depth: &RgTexture,
    half_normal: &RgTexture,
    taa_history: &RgTexture,
    history_valid: u8,
    reprojection: &RgTexture,
    ssgi: &RgTexture,
) -> RayTraceGi {
    let full = extent_of(&gbuffer.depth);
    let ext = div_up_extent([full.width, full.height], [2, 2]);
    let plain = |format| TextureDesc::new_2d(ext, format);
    let cand_radiance =
        env.create_texture(CANDIDATE_RADIANCE_NAME, plain(CANDIDATE_RADIANCE_FORMAT));
    let cand_hit = env.create_texture(CANDIDATE_HIT_NAME, plain(CANDIDATE_HIT_FORMAT));

    let trace = env
        .pass(TRACE.0, TRACE.1)
        .uniform(binding::BOOL_RENDER_SHADOWS, true)
        .bind_texture(binding::SHADOW_MAP, &TexturePreset(0x11), 0x19)
        .uniform(binding::HISTORY_VALID, history_valid as i32)
        .bind_texture(binding::DEPTH_TEX, &&gbuffer.depth, 0x19)
        .bind_texture(binding::TAA_HISTORY_TEX, &taa_history, 0x19)
        .bind_texture(binding::REPROJECTION_TEX, &reprojection, 0x19)
        .bind_texture(binding::SSGI_TEX, &ssgi, 0x19)
        .bind_texture(binding::HALF_DEPTH_TEX, &half_depth, 0x19)
        .bind_texture(binding::HALF_VIEW_NORMAL_TEX, &half_normal, 0x19);
    // 原版 trace_gi_candidates SPV：taa_history_tex 唯一的 SampleLevel 位于
    // `history_valid != 0 && frame_index != 0 && result_hit` 分支内；history_valid==0
    // 时不可达。保留原绑定与同步，仅在历史无效时声明该读不要求内容。
    let trace = if history_valid == 0 {
        trace.inactive_texture_contents(binding::TAA_HISTORY_TEX)
    } else {
        trace
    };
    let trace = bind_world_inputs(trace)
        .bind_texture(
            CANDIDATE_RADIANCE_TEX,
            &WriteTextureUnchecked(cand_radiance),
            0x32,
        )
        .bind_texture(CANDIDATE_HIT_TEX, &WriteTextureUnchecked(cand_hit), 0x32)
        .dispatch_2d(ext[0], ext[1]);

    let radiance = env.temporal_pair(
        TEMPORAL_RADIANCE,
        temporal_spec(ext, CANDIDATE_RADIANCE_FORMAT),
    );
    let ray_dir = env.temporal_pair(
        TEMPORAL_RAY_DIR,
        temporal_spec(ext, CANDIDATE_RADIANCE_FORMAT),
    );
    let reservoir = env.temporal_pair(TEMPORAL_RESERVOIR, temporal_spec(ext, RESERVOIR_FORMAT));
    let all_valid = radiance.fresh && ray_dir.fresh && reservoir.fresh;

    let update = env
        .pass(UPDATE.0, UPDATE.1)
        .uniform(binding::BOOL_RENDER_SHADOWS, true)
        .bind_texture(binding::SHADOW_MAP, &TexturePreset(0x11), 0x19)
        .uniform(
            binding::HISTORY_VALID,
            if all_valid { history_valid as i32 } else { 0 },
        )
        .bind_texture(binding::GBUFFER_TEX, &&gbuffer.gbuffer, 0x19)
        .bind_texture(binding::DEPTH_TEX, &&gbuffer.depth, 0x19)
        .bind_texture(binding::TAA_HISTORY_TEX, &taa_history, 0x19)
        .bind_texture(binding::RESTIR_RADIANCE_TEX, &&radiance.previous, 0x19)
        .bind_texture(binding::RESTIR_RAY_DIR_TEX, &&ray_dir.previous, 0x19)
        .bind_texture(binding::RESTIR_RESERVOIR_TEX, &&reservoir.previous, 0x19)
        .bind_texture(binding::REPROJECTION_TEX, &reprojection, 0x19)
        .bind_texture(binding::SSGI_TEX, &ssgi, 0x19)
        .bind_texture(CANDIDATE_RADIANCE_TEX, &&cand_radiance, 0x19)
        .bind_texture(CANDIDATE_HIT_TEX, &&cand_hit, 0x19);
    let update = bind_world_inputs(update)
        .bind_texture(
            RESTIR_RADIANCE_OUT_TEX,
            &WriteTextureUnchecked(radiance.current),
            0x32,
        )
        .bind_texture(
            RESTIR_RAY_DIR_OUT_TEX,
            &WriteTextureUnchecked(ray_dir.current),
            0x32,
        )
        .bind_texture(
            RESTIR_RESERVOIR_OUT_TEX,
            &WriteTextureUnchecked(reservoir.current),
            0x32,
        )
        .dispatch_2d(ext[0], ext[1]);

    RayTraceGi {
        output: RestirTextures {
            radiance: radiance.current,
            ray_dir: ray_dir.current,
            reservoir: reservoir.current,
        },
        trace,
        update,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        Env, accesses, env, rgtex, tex_key, threads,
    };

    fn names() -> Vec<u64> {
        vec![
            binding::SHADOW_MAP,
            binding::DEPTH_TEX,
            binding::TAA_HISTORY_TEX,
            binding::REPROJECTION_TEX,
            binding::SSGI_TEX,
            binding::HALF_DEPTH_TEX,
            binding::HALF_VIEW_NORMAL_TEX,
            binding::TERRAIN_HEIGHTMAP,
            binding::VISUAL_TERRAIN_HEIGHTMAP,
            binding::TERRAIN_LIGHTING_TEXTURE,
            binding::BLUE_NOISE_TEXTURE,
            CANDIDATE_RADIANCE_TEX,
            CANDIDATE_HIT_TEX,
            binding::GBUFFER_TEX,
            binding::RESTIR_RADIANCE_TEX,
            binding::RESTIR_RAY_DIR_TEX,
            binding::RESTIR_RESERVOIR_TEX,
            RESTIR_RADIANCE_OUT_TEX,
            RESTIR_RAY_DIR_OUT_TEX,
            RESTIR_RESERVOIR_OUT_TEX,
        ]
    }

    fn run(hist: u8, pair_valid: bool) -> (Env, RayTraceGi) {
        let mut e = env(
            &names(),
            &[binding::BOOL_RENDER_SHADOWS, binding::HISTORY_VALID],
        );
        e.frame.pair_fresh = pair_valid;
        let g = GBuffer {
            gbuffer: rgtex(1, 1001, 601),
            depth: rgtex(2, 1001, 601),
        };
        let out = e.run(|env| {
            ray_trace_gi(
                env,
                &g,
                &rgtex(3, 501, 301),
                &rgtex(4, 501, 301),
                &rgtex(5, 1001, 601),
                hist,
                &rgtex(6, 1001, 601),
                &rgtex(7, 501, 301),
            )
        });
        (e, out)
    }

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(CANDIDATE_RADIANCE_TEX, 0xb5cc_c14f_cb0c_f31c);
        assert_eq!(CANDIDATE_HIT_TEX, 0x5014_5c2c_a7eb_95e6);
        assert_eq!(RESTIR_RADIANCE_OUT_TEX, 0xc43e_3a1f_deb8_1ff5);
        assert_eq!(RESTIR_RAY_DIR_OUT_TEX, 0x7f8f_7289_42ff_f84a);
        assert_eq!(RESTIR_RESERVOIR_OUT_TEX, 0xf03e_bb08_739d_6f53);
        assert_eq!(binding::TERRAIN_LIGHTING_TEXTURE, 0x7716_0113_ac8d_ca3f);
        assert_eq!(binding::TAA_HISTORY_TEX, 0x0417_5185_826c_09d0);
    }

    #[test]
    fn resources_are_created_in_exe_order_at_half_res_rounded_up() {
        let (e, _) = run(1, true);
        let t = &e.frame.textures;
        assert_eq!(tex_key(&t[0]), (CANDIDATE_RADIANCE_NAME, [501, 301], 0x61));
        assert_eq!(tex_key(&t[1]), (CANDIDATE_HIT_NAME, [501, 301], 0x6d));
        let names: Vec<_> = e
            .frame
            .temporal_pairs
            .iter()
            .map(|x| (x.0.as_str(), x.1.format, x.1.storage && x.1.sampled))
            .collect();
        assert_eq!(
            names,
            [
                (TEMPORAL_RADIANCE, 0x61, true),
                (TEMPORAL_RAY_DIR, 0x61, true),
                (TEMPORAL_RESERVOIR, 0x65, true)
            ]
        );
    }

    #[test]
    fn two_passes_dispatch_half_extent() {
        let (e, out) = run(1, true);
        assert_eq!(
            (out.trace.name, out.update.name),
            ("Trace GI candidates", "Update GI radiance")
        );
        assert_eq!(threads(&out.trace), (501, 301, 1));
        assert_eq!(threads(&out.update), (501, 301, 1));
        assert_eq!(
            *e.world.backend.paths.borrow(),
            vec![
                format!("{}:cs_main", TRACE.1),
                format!("{}:cs_main", UPDATE.1)
            ]
        );
        // blue_noise 是磁盘纹理（外部描述符，不登记 RG 访问）。
        assert_eq!(out.trace.external_bindings.len(), 1);
        assert_eq!(
            out.update.external_bindings.len(),
            1,
            "update 也经 bind_world_inputs 绑 blue_noise"
        );
    }

    #[test]
    fn update_history_valid_requires_all_three_flags() {
        let (e, out) = run(1, true);
        assert_eq!(e.uniform_u32(&out.trace, binding::HISTORY_VALID), 1);
        assert_eq!(e.uniform_u32(&out.update, binding::HISTORY_VALID), 1);
        let (e, out) = run(1, false);
        assert_eq!(
            e.uniform_u32(&out.trace, binding::HISTORY_VALID),
            1,
            "trace pass uses the raw argument"
        );
        assert_eq!(e.uniform_u32(&out.update, binding::HISTORY_VALID), 0);
        let (e, out) = run(0, true);
        assert_eq!(e.uniform_u32(&out.update, binding::HISTORY_VALID), 0);
    }

    #[test]
    fn update_writes_the_three_returned_current_handles() {
        let (_e, out) = run(1, true);
        let a = accesses(&out.update);
        let writes: Vec<u32> = a.iter().filter(|x| x.1 == 0x32).map(|x| x.0).collect();
        assert_eq!(
            writes,
            [
                out.output.radiance.resource,
                out.output.ray_dir.resource,
                out.output.reservoir.resource
            ]
        );
        // 候选纹理在 trace 里是写、在 update 里是读
        let tw: Vec<u32> = accesses(&out.trace)
            .iter()
            .filter(|x| x.1 == 0x32)
            .map(|x| x.0)
            .collect();
        assert_eq!(tw.len(), 2);
        assert!(
            a.iter().any(|x| x.0 == tw[0] && x.1 == 0x19)
                && a.iter().any(|x| x.0 == tw[1] && x.1 == 0x19)
        );
    }
}
