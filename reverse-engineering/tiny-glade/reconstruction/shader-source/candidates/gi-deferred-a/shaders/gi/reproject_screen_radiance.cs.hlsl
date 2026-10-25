// ==RECONSTRUCTED== shaders/gi/reproject_screen_radiance.cs.hlsl | status: O (structure from disassembly)
// Evidence: _gi_reproject_screen_radiance.cs.hlsl_cs_main_0.bin (LocalSize 8x8, entry cs_main).
//   bindings: sampler_lnc s0, sampler_nnc s2, $Globals b13 (history_valid, 7 dims,
//   allow_non_determinism), half_depth_tex t14, half_view_normal_tex t15,
//   reprojection_tex t16, history_tex t17, temporal_stats_tex t18, output_tex u19,
//   temporal_stats_out_tex u20, frame_constants set1/b0.
//   Temporal reprojection of the 3 SH radiance texels: octahedral-encoded view
//   direction is tracked in temporal_stats (uv confidence); when history is valid the
//   reprojection map moves the sample, history is sampled per SH column and blended by
//   the temporal count; the spatial 5x5 fallback (gated on allow_non_determinism)
//   re-accumulates neighbor texels when the temporal stats disagree.
//   Include residual: original closure ~14 files vs candidate ~10 (see decisions.md).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/pack_unpack.hlsl"
#include "gi/gi_settings.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint history_valid;
    uint2 half_depth_tex_dims;
    uint2 half_view_normal_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 history_tex_dims;
    uint2 temporal_stats_tex_dims;
    uint2 output_tex_dims;
    uint2 temporal_stats_out_tex_dims;
    uint allow_non_determinism;
};

// samplers (sampler_lnc/sampler_nnc) come from inc/common.hlsl
Texture2D<float4> half_depth_tex : register(t14, space0);
Texture2D<float4> half_view_normal_tex : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
Texture2D<float4> history_tex : register(t17, space0);
Texture2D<float4> temporal_stats_tex : register(t18, space0);
RWTexture2D<float4> output_tex : register(u19, space0);
RWTexture2D<unorm float4> temporal_stats_out_tex : register(u20, space0);

// Octahedral projection of a view-space direction to a 2D uv.
float3 octahedral_wrap(float3 d)
{
    if (d.z < 0.0f) {
        float2 wrapped = (1.0f - abs(d.yx)) * ((step(0.0f, d.xy) * 2.0f) - 1.0f);
        return float3(wrapped.x, wrapped.y, d.z);
    }
    return d;
}

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint2 px = gl_GlobalInvocationID.xy;

    float4 vn = half_view_normal_tex.Load(int3(px, 0));
    float3 view_n = vn.xyz;
    float reproj_sel = vn.w;

    float3 oct = view_n / (abs(view_n.x) + abs(view_n.y) + abs(view_n.z));
    float3 oct_wrap = octahedral_wrap(oct);
    float2 oct_uv = (oct_wrap.xy * 0.5f) + 0.5f;
    float4 tstats_new = float4(0.0f, oct_uv.x, oct_uv.y, 0.0f);

    if ((history_valid == 0u) || (frame_index == 0u)) {
        temporal_stats_out_tex[px] = tstats_new;
        return;
    }

    float2 half_dims = float2(half_depth_tex_dims);
    float4 reproj = reprojection_tex.Load(int3((px * 2u) + uint2(uint(abs(reproj_sel) > 0.75f), uint(reproj_sel >= 0.0f)), 0));
    float2 uv = ((float2(int2(px)) + 0.5f) / half_dims) + reproj.xy;
    float reproj_valid = reproj.z;

    float4 tstats_val;
    bool reset_tstats;
    if (reproj_valid > 0.0f) {
        float2 uv_n;
        if (reproj_valid != 1.0f) {
            uint4 sel = uint((reproj_valid * 15.0f) + 0.5f).xxxx & uint4(1u, 2u, 4u, 8u);
            float4 selb = float4(sel.x != 0u, sel.y != 0u, sel.z != 0u, sel.w != 0u);
            uv_n = uv + (((((-0.5f).xx * selb.x) + (float2(0.5f, -0.5f) * selb.y)) +
                (float2(-0.5f, 0.5f) * selb.z) + (0.5f.xx * selb.w)) * 2.0f) / half_dims;
        } else {
            uv_n = uv;
        }

        float2 uv_c = clamp(uv_n, float2(0.5f / half_dims.x, 0.0f), 1.0f - float2(0.5f / half_dims.x, 0.0f));

        float4 hist[3];
        [unroll]
        for (uint c = 0u; c < 3u; ++c) {
            hist[c] = history_tex.SampleLevel(sampler_lnc, (uv_c * float2(0.3333333432674407958984375f, 1.0f)) + float2(float(c) * 0.3333333432674407958984375f, 0.0f), 0.0f);
        }

        float2 uv_check = ((octahedral_wrap(oct).xy * 0.5f) + 0.5f);
        float4 dg = abs(uv_check.xxxx - temporal_stats_tex.GatherGreen(sampler_nnc, uv_c, int2(0, 0)));
        float4 db = abs(uv_check.yyyy - temporal_stats_tex.GatherBlue(sampler_nnc, uv_c, int2(0, 0)));
        bool agree = (min(min(dg.x, dg.y), min(dg.z, dg.w)) + min(min(db.x, db.y), min(db.z, db.w))) < 0.2f;

        if (agree) {
            float count = clamp((temporal_stats_tex.SampleLevel(sampler_nnc, uv_c, 0.0f) * float4(255.0f, 1.0f, 1.0f, 1.0f)).x + 1.0f, 1.0f, 48.0f);
            float4 ts = tstats_new;
            ts.x = count;
            float inv = 1.0f / count;

            float4 outv[3];
            [unroll]
            for (uint c = 0u; c < 3u; ++c) {
                float4 v = lerp(hist[c], output_tex[px + uint2(c * half_depth_tex_dims.x, 0u)], inv);
                v.x = max(0.0f, v.x);
                float3 clampv = clamp(v.yzw, -1024.0f, 1024.0f);
                outv[c] = float4(v.x, clampv.x, clampv.y, clampv.z);
            }
            [unroll]
            for (uint c = 0u; c < 3u; ++c) {
                output_tex[px + uint2(c * half_depth_tex_dims.x, 0u)] = outv[c];
            }
            tstats_val = ts;
            reset_tstats = false;
        } else {
            tstats_val = tstats_new;
            reset_tstats = true;
        }
    } else {
        tstats_val = tstats_new;
        reset_tstats = true;
    }

    if (reset_tstats) {
        temporal_stats_out_tex[px] = tstats_val;
    }

    if (!(reset_tstats && (allow_non_determinism != 0u))) {
        return;
    }

    // Spatial accumulation over a 5x5 (stride-2) neighborhood.
    float depth = half_depth_tex.Load(int3(px, 0)).x;
    float normal_z = max(0.3f, view_n.z);
    float4 sh[3] = { float4(0.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 0.0f) };
    float4 ts_acc = float4(0.0f, 0.0f, 0.0f, 0.0f);
    float weight_sum = 0.0f;
    float weight_sum_cnt = 0.0f;

    for (int dy = -2; dy <= 2; dy++) {
        for (int dx = -2; dx <= 2; dx++) {
            uint2 np = px + uint2(int2(dx, dy) * int2(2, 2));
            if (any(np >= half_depth_tex_dims)) {
                continue;
            }
            float nd = dot(view_n, half_view_normal_tex.Load(int3(np, 0)).xyz);
            float nw = (nd * nd) * (-0.9f) + 1.0f;
            float w = exp2(-min(30.0f, 80.0f * (normal_z * abs(1.0f - (half_depth_tex.Load(int3(np, 0)).x / depth))))) *
                      (0.01f / (nw * nw));
            float4 ts = temporal_stats_out_tex[np] * float4(255.0f, 1.0f, 1.0f, 1.0f);
            float count = max(ts.x, 1.0f);
            [unroll]
            for (uint c = 0u; c < 3u; ++c) {
                sh[c] += output_tex[np + uint2(c * half_depth_tex_dims.x, 0u)] * w * count;
            }
            ts_acc += ts * w;
            weight_sum += w;
            weight_sum_cnt += w * count;
        }
    }

    float inv_cnt = 1.0f / max(1e-20f, weight_sum_cnt);
    [unroll]
    for (uint c = 0u; c < 3u; ++c) {
        float4 v = sh[c] * inv_cnt;
        v.x = max(0.0f, v.x);
        float3 clampv = clamp(v.yzw, -1024.0f, 1024.0f);
        output_tex[px + uint2(c * half_depth_tex_dims.x, 0u)] = float4(v.x, clampv.x, clampv.y, clampv.z);
    }

    float4 ts_out = ts_acc * (1.0f / max(1e-20f, weight_sum));
    ts_out.x = min(ts_out.x, 24.0f);
    temporal_stats_out_tex[px] = ts_out * float4(0.0039215688593685626983642578125f, 1.0f, 1.0f, 1.0f);
}
