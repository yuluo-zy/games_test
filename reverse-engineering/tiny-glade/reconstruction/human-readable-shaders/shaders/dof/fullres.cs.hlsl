// PATCHED: modules ain_6f1dd5dd8a66e104.bin:cs_main, ain_72a2a95d73e37edc.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
// === 中文说明 ===
// 全分辨率散焦 (dof/fullres):先做半分辨率 bokeh 的空间上采样(8 tap),再按抖动
//   双环 x 17 tap 的 CoC 聚散环采集,叠加后按深度权重重归一化,写 img_output。
//   入口: cs_main (compute, numthreads 8x8x1)。变体: 6f1dd5dd8a66e104 与
//   72a2a95d73e37edc 的 cs_main 均 lifted(6f1d 双环,72a2 四环 12 槽)。
// 绑定: blue_noise t14, color_tex t15, coc_tiles_dilated t16, coc_tex t18,
//   img_output u19 (Rgba16f), halfres_tex t20, spatial_resolve_offsets b21,
//   tile_offsets t22, tile_coords t23, $Globals b13, frame_constants set1/b0。
#if !((defined(TGV_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/dof/fullres.cs.hlsl | MERGE (dof-aa review, 2026-10-13): B loop/pack structure (OpLoopMerge 6/7, f16x2 masking unpack) + A DECREASING near-branch ramp (smoothstep(11,8), abs>=11) + A RWTexture2D<half4> img_output (Rgba16f). 6f1d noise scheme only (72a2 residual).
//   Full-res bokeh: half-res spatial resolve (upsample_halfres.inc) + the
//   dithered 2-ring x 17-tap CoC gather (multires.inc / shared with halfres),
//   summed and alpha-normalized into img_output.  Status: O (structure from
//   disassembly).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/math.hlsl"
#include "inc/coc.hlsl"
#include "inc/hash.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/morton.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/color/srgb.hlsl"
#include "dof/tiles.inc.hlsl"
#include "dof/upsample_halfres.inc.hlsl"
#include "dof/multires.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float bokeh_anisotropy;
    uint2 blue_noise_texture_dims;
    uint2 color_tex_dims;
    uint2 coc_tiles_dilated_tex_dims;
    uint2 depth_tex_dims;
    uint2 coc_tex_dims;
    uint2 img_output_dims;
    uint2 halfres_tex_dims;
};

Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<half4> img_output : register(u19, space0);
Texture2D<float4> halfres_tex : register(t20, space0);
cbuffer spatial_resolve_offsets : register(b21, space0)
{
    int4 spatial_resolve_offsets_array[512];
};
StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t22, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t23, space0);

groupshared uint dof_ring_samples[64][9];

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_WorkGroupID : SV_GroupID,
             uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
             uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    // 按 tile 坐标取当前 8x8 像素块。
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer[tile_offsets_buffer[0].offsets[0] + gl_WorkGroupID.x]);
    float2 pixel_uv = (float2(int2(pixel)) + 0.5f) / float2(img_output_dims);

    // Half-res 空间上采样(半分辨率 bokeh 的 8 tap)。
    uint2 in_tile = pixel & uint2(1u, 1u);
    uint quadrant = in_tile.x + (in_tile.y * 2u);
    float4 upsample = upsample_halfres(halfres_tex, spatial_resolve_offsets_array, pixel, quadrant);

    // CoC 上限:取纹理高度比例与膨胀 tile 的 CoC 中较小者。
    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, pixel_uv, 0.0f).x);

    // 蓝色噪声抖动(含帧索引平移)。
    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u))
        + uint2(int2(int(frame_index * 59u), int(frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f)
        * 0.00390625f) + 0.001953125f;

    // 每像素哈希抖动的子步进与符号选择。
    uint2 hash_pos = uint2(7u, 7u) + pixel;
    float sub = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(hash_pos.x))
        + (0.005837149918079376220703125f * float(hash_pos.y)))) - 0.5f) * 0.5f;
    float sub2 = 0.5f + sub;
    float dither_y = dither.y;
    float coc_step = coc_limit * 0.125f;
    float frame_rand = 5.588237762451171875f * float(frame_index % 64u);
    float tap_seed = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(pixel.x) + frame_rand))
        + (0.005837149918079376220703125f * (float(pixel.y) + frame_rand))));
    bool use_low = sub2 < 0.5f;
    float tap_sub = use_low ? (0.5f - sub) : sub2;
    float tap_sign = use_low ? (-1.0f) : 1.0f;

    // 各累加器初始化。
    float4 color_accum = 0.0f.xxxx;
    float weight_accum = 0.0f;
    float depth_weight = 0.0f;
    float4 ring_color_accum = 0.0f.xxxx;
    uint ring_bits[8];
    ring_bits[0] = 0u; ring_bits[1] = 0u; ring_bits[2] = 0u; ring_bits[3] = 0u;
    ring_bits[4] = 0u; ring_bits[5] = 0u; ring_bits[6] = 0u; ring_bits[7] = 0u;
    float coc_min = 1e10f;
    float coc_max = -1e10f;

    // 双环 x 17 tap 的 CoC 聚散环采集。
    for (uint ring = 0u; ring < 2u; ++ring)
    {
        float angle = (float(ring) + tap_seed) * 1.57079637050628662109375f;
        float2 dir = float2(cos(angle), sin(angle));
        float2 bokeh_dir;
        if (bokeh_anisotropy > 0.0f)
        {
            // 各向异性:按视线方向拉伸/压缩采样方向。
            float4 ctv = mul(view_constants[0].clip_to_view, float4((pixel_uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 view_dir = ctv.xy / ctv.z;
            view_dir.y = view_dir.y * (-1.0f);
            float2 norm_dir = normalize(view_dir);
            float2x2 rot = float2x2(norm_dir, float2(-norm_dir.y, norm_dir.x));
            float aniso = lerp(0.7f, 1.0f, exp((-4.0f * bokeh_anisotropy) * dot(view_dir, view_dir)));
            bokeh_dir = mul(transpose(rot), mul(rot, dir) * float2(aniso, 1.0f / aniso));
        }
        else
        {
            bokeh_dir = dir;
        }
        float2 step_px = bokeh_dir * (tap_sign / float2(color_tex_dims));
        float neg_coc = -coc_limit;
        float lerp_gain = dither.x * 0.2f;

        // 17 个 tap 依次采样 CoC,并把成对的 half 打包进共享内存。
        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float m = abs(coc0);
        float w = lerp(weight_accum, max(weight_accum, m), lerp_gain);
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][0] = dof_pack_half2(float2(coc0, 0.0f)) | (dof_pack_half2(float2(coc1, 0.0f)) << 16u);

        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc2)), lerp_gain);
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][1] = dof_pack_half2(float2(coc2, 0.0f)) | (dof_pack_half2(float2(coc3, 0.0f)) << 16u);

        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc4)), lerp_gain);
        float coc5 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc5)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][2] = dof_pack_half2(float2(coc4, 0.0f)) | (dof_pack_half2(float2(coc5, 0.0f)) << 16u);

        float coc6 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc6)), lerp_gain);
        float coc7 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc7)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][3] = dof_pack_half2(float2(coc6, 0.0f)) | (dof_pack_half2(float2(coc7, 0.0f)) << 16u);

        float coc8 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (dither_y * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc8)), lerp_gain);
        float coc9 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc9)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][4] = dof_pack_half2(float2(coc8, 0.0f)) | (dof_pack_half2(float2(coc9, 0.0f)) << 16u);

        float coc10 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc10)), lerp_gain);
        float coc11 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc11)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][5] = dof_pack_half2(float2(coc10, 0.0f)) | (dof_pack_half2(float2(coc11, 0.0f)) << 16u);

        float coc12 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc12)), lerp_gain);
        float coc13 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc13)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][6] = dof_pack_half2(float2(coc12, 0.0f)) | (dof_pack_half2(float2(coc13, 0.0f)) << 16u);

        float coc14 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc14)), lerp_gain);
        float coc15 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc15)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][7] = dof_pack_half2(float2(coc14, 0.0f)) | (dof_pack_half2(float2(coc15, 0.0f)) << 16u);

        float coc16 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        weight_accum = lerp(w, max(w, abs(coc16)), lerp_gain);
        coc_min = min(min(min(min(min(min(min(min(min(coc_min, abs(coc0)), abs(coc2)), abs(coc4)), abs(coc6)), abs(coc8)), abs(coc10)), abs(coc12)), abs(coc14)), abs(coc16));
        coc_max = max(max(max(max(max(max(max(max(max(coc_max, abs(coc0)), abs(coc2)), abs(coc4)), abs(coc6)), abs(coc8)), abs(coc10)), abs(coc12)), abs(coc14)), abs(coc16));
        dof_ring_samples[gl_LocalInvocationIndex][8] = dof_pack_half2(float2(coc16, 0.0f));

        // 记录每个 tap 落在哪个环槽位(按 slot_coc 比较)。
        for (uint i = 0u; i < 8u; ++i)
        {
            ring_bits[i] = 0u;
        }
        for (uint tap = 0u; tap < 17u; ++tap)
        {
            uint par = tap & 1u;
            uint half = tap >> 1u;
            float2 slot_coc = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][half] & 65535u).x,
                                     dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][half] >> 16u) & 65535u).x);
            float tap_coc = (float(int(tap - 8u)) + dither_y) * coc_step;
            for (int slot = 0; slot < 8; ++slot)
            {
                if ((tap_coc / (((float(slot) + tap_sub) * 0.25f) - 1.0f)) >= slot_coc[par])
                {
                    ring_bits[slot] |= (1u << (tap & 31u));
                }
            }
        }

        // 逐槽解析:取环上最近的两个 tap 做亚像素插值。
        float4 slot_color = 0.0f.xxxx;
        float slot_weight = 0.0f;
        float slot_depth = 0.0f;
        for (int slot = 0; slot < 8; slot_weight = depth_weight, slot_depth = slot_depth, slot_color = slot_color, ++slot)
        {
            float slot_frac = ((float(slot) + tap_sub) * 0.25f) - 1.0f;
            float abs_frac = abs(slot_frac);
            depth_weight = slot_depth + abs_frac;
            if (ring_bits[slot] == 0u)
            {
                continue;
            }
            bool positive = slot_frac >= 0.0f;
            float lo_coc, hi_coc;
            uint tap_idx;
            if (positive)
            {
                // 正向:用最前位与前一 bit 的 CoC 对。
                uint low_bit = firstbitlow(ring_bits[slot]);
                uint prev_bit = uint(max(0, (int(low_bit) - 1)));
                uint low_half = low_bit >> 1u;
                float2 a = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][low_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][low_half] >> 16u) & 65535u).x);
                uint prev_half = prev_bit >> 1u;
                float2 b = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][prev_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][prev_half] >> 16u) & 65535u).x);
                lo_coc = a[low_bit & 1u];
                hi_coc = b[prev_bit & 1u];
                tap_idx = low_bit;
            }
            else
            {
                // 负向:用最高位与下一 bit 的 CoC 对。
                uint high_bit = firstbithigh(ring_bits[slot]);
                uint next_bit = min(16u, (high_bit + 1u));
                uint high_half = high_bit >> 1u;
                float2 a = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][high_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][high_half] >> 16u) & 65535u).x);
                uint next_half = next_bit >> 1u;
                float2 b = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][next_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][next_half] >> 16u) & 65535u).x);
                lo_coc = a[high_bit & 1u];
                hi_coc = b[next_bit & 1u];
                tap_idx = high_bit;
            }
            float tap_coc = (float(int(tap_idx - 8u)) + dither_y) * coc_step;
            float coc_at_slot = tap_coc / slot_frac;
            float delta = coc_step * (positive ? 1.0f : (-1.0f));
            float lo_edge = tap_coc - delta;
            float hi_edge = hi_coc - (coc_at_slot - (delta / slot_frac));
            float slot_span = abs_frac * float(ring_bits[slot] != 0u);
            float resolved = lerp(lo_edge, tap_coc, clamp(hi_edge / (hi_edge + (-(lo_coc - coc_at_slot))), 0.0f, 1.0f));
            float coc_at_resolved = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * resolved), 0.0f).x, neg_coc, coc_limit);
            // 遮挡:按 CoC 跨度降权,并在相邻 lane 取最小。
            float occlusion = log2(max(0.0f, abs(lo_coc) * 0.125f)) * smoothstep(4.0f, 0.0f, coc_max - coc_min);
            float occ_lane = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 1u);
            occlusion = min(occlusion, occ_lane);
            float occ_wide = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 8u);
            if (lo_coc > coc_limit)
            {
                continue;
            }
            // 靠近环边缘时再做一次亚像素采样。
            float edge_eps = hi_edge * 0.1f;
            bool close_to_edge;
            if (((resolved / slot_frac) - coc_at_resolved) < edge_eps)
            {
                close_to_edge = (coc_at_resolved - lo_coc) < edge_eps;
            }
            else
            {
                close_to_edge = false;
            }
            float4 ring_color;
            float ring_weight;
            // DECREASING near-branch ramp for fullres (target): keep taps with
            // |coc| >= 11, ramp weight 11 -> 8 (opposite of halfres's 8 -> 11).
            if (close_to_edge)
            {
                float abs_resolved = abs(coc_at_resolved);
                if (abs_resolved >= 11.0f)
                {
                    continue;
                }
                ring_color = slot_color;
                ring_weight = slot_span * smoothstep(11.0f, 8.0f, abs_resolved);
            }
            else
            {
                float abs_hi = abs(hi_coc);
                if (abs_hi >= 11.0f)
                {
                    continue;
                }
                float w2 = slot_span * smoothstep(11.0f, 8.0f, abs_hi);
                float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.2f * coc_limit, 0.0f, lo_coc);
                ring_color = slot_color + (w2 * depth_gate);
                ring_weight = w2 * (1.0f - depth_gate);
            }
            depth_weight = ring_color.w;
            slot_color = slot_color + (color_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (close_to_edge ? resolved : lo_edge)), min(occlusion, occ_wide)) * ring_weight);
        }
        ring_color_accum = ring_color_accum + slot_color;
    }

    // 散射项:超出景深权重阈值时补充 16 点背景散射。
    float scatter = depth_weight * smoothstep(0.01f * ring_color_accum.w, max(1e-5f, 0.2f * ring_color_accum.w), depth_weight);
    float4 final_color;
    if (scatter > 0.0f)
    {
        float neg_coc = -coc_limit;
        float4 bg_accum = 0.0f.xxxx;
        float max_bg_coc = neg_coc;
        for (uint s = 0u; s < 16u; ++s)
        {
            float2 rnd = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(s)) + 0.5f) + dither.xy);
            float r = sqrt(rnd.x);
            float phi = rnd.y * 6.283185482025146484375f;
            float2 bg_uv = pixel_uv + (((float2(cos(phi) * r, sin(phi) * r) * 1.0f) / float2(color_tex_dims)) * weight_accum);
            float bg_coc = coc_tex.SampleLevel(sampler_nnc, bg_uv, 0.0f).x;
            if (bg_coc > (max_bg_coc - 10.0f))
            {
                bg_accum += color_tex.SampleLevel(sampler_llc, bg_uv, 0.0f);
                max_bg_coc = max(max_bg_coc, bg_coc);
            }
        }
        final_color = ring_color_accum + ((bg_accum / max(1.0f, bg_accum.w)) * scatter);
    }
    else
    {
        final_color = ring_color_accum;
    }

    // 合并半分辨率上采样与全分辨率聚散,并按 alpha 归一化。
    float4 combined = upsample + (final_color / depth_weight);
    img_output[pixel] = float4((combined / max(1e-10f, combined.w)).xyz, 1.0f);
}

#endif

#if defined(TGV_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _dof_fullres.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// 6f1d 变体的 cs 段:双环 x 17 tap 聚散采集(与手写 cs 语义一致,变量名已语义化)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct DofTileOffsets
{
    uint offsets[8];
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    float bokeh_anisotropy : packoffset(c0);
    uint2 blue_noise_texture_dims : packoffset(c0.y);
    uint2 color_tex_dims : packoffset(c1);
    uint2 coc_tiles_dilated_tex_dims : packoffset(c1.z);
    uint2 depth_tex_dims : packoffset(c2);
    uint2 coc_tex_dims : packoffset(c2.z);
    uint2 img_output_dims : packoffset(c3);
    uint2 halfres_tex_dims : packoffset(c3.z);
};

cbuffer spatial_resolve_offsets : register(b21, space0)
{
    int4 spatial_resolve_offsets_spatial_resolve_offsets_array[512] : packoffset(c0);
};

ByteAddressBuffer tile_offsets_buffer : register(t22, space0);
ByteAddressBuffer tile_coords_buffer : register(t23, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);
Texture2D<float4> halfres_tex : register(t20, space0);

static uint3 gl_WorkGroupID;
static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_WorkGroupID : SV_GroupID;
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint ring_samples[64][9];

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    // 当前 8x8 像素块与象限。
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(0) + gl_WorkGroupID.x) * 8 + 0));
    uint2 in_tile = pixel & uint2(1u, 1u);
    uint quadrant = in_tile.x + (in_tile.y * 2u);
    // 半分辨率上采样(4 象限 8 tap 求和)。
    float4 upsample;
    upsample = 0.0f.xxxx;
    for (uint i = 0u; i < 8u; )
    {
        upsample += halfres_tex.Load(int3((pixel / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_spatial_resolve_offsets_array[(quadrant * 16u) + i].xy), 0u));
        i++;
        continue;
    }
    float2 pixel_uv = (float2(int2(pixel)) + 0.5f.xx) / float2(img_output_dims);
    // CoC 上限。
    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, pixel_uv, 0.0f).x);
    // 蓝色噪声抖动。
    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint2 hash_pos = uint2(7u, 7u) + pixel;
    // 每像素哈希抖动的子步进与符号选择。
    float sub = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(hash_pos.x)) + (0.005837149918079376220703125f * float(hash_pos.y)))) - 0.5f) * 0.5f;
    float sub2 = 0.5f + sub;
    float dither_y = dither.y;
    float coc_step = coc_limit * 0.125f;
    float frame_rand = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
    float tap_seed = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(pixel.x) + frame_rand)) + (0.005837149918079376220703125f * (float(pixel.y) + frame_rand))));
    bool use_low = sub2 < 0.5f;
    float tap_sub;
    if (use_low)
    {
        tap_sub = 0.5f - sub;
    }
    else
    {
        tap_sub = sub2;
    }
    float tap_sign = use_low ? (-1.0f) : 1.0f;
    // 累加器初始化。
    float4 color_accum;
    float weight_accum;
    float depth_weight;
    float ring_weight_acc;
    color_accum = 0.0f.xxxx;
    weight_accum = 0.0f;
    depth_weight = 0.0f;
    ring_weight_acc = 0.0f;
    float4 color_accum_next;
    float weight_accum_next;
    float coc_min_next;
    float coc_max_next;
    uint ring_bits[8];
    float depth_weight_next;
    float ring_weight_acc_next;
    uint ring = 0u;
    float coc_min = 10000000000.0f;
    float coc_max = -10000000000.0f;
    // 双环 x 17 tap。
    [loop]
    for (; ring < 2u; color_accum = color_accum_next, weight_accum = weight_accum_next, depth_weight = depth_weight_next, ring_weight_acc = ring_weight_acc_next, ring++, coc_min = coc_min_next, coc_max = coc_max_next)
    {
        float angle = (float(ring) + tap_seed) * 1.57079637050628662109375f;
        float2 dir = float2(cos(angle), sin(angle));
        float2 bokeh_dir;
        if (bokeh_anisotropy > 0.0f)
        {
            // 各向异性:按视线方向拉伸采样方向。
            float4 ctv = mul(frame_constants_view_constants[0u].clip_to_view, float4((pixel_uv - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 view_dir = ctv.xy / ctv.z.xx;
            view_dir.y = view_dir.y * (-1.0f);
            float2 norm_dir = normalize(view_dir);
            float2x2 rot = float2x2(norm_dir, float2(-norm_dir.y, norm_dir.x));
            float aniso = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * bokeh_anisotropy) * dot(view_dir, view_dir)));
            bokeh_dir = mul(transpose(rot), mul(rot, dir) * float2(aniso, 1.0f / aniso));
        }
        else
        {
            bokeh_dir = dir;
        }
        float2 step_px = bokeh_dir * (tap_sign.xx / float2(color_tex_dims));
        float neg_coc = -coc_limit;
        // tap -8。
        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-8.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc0 = abs(coc0);
        float lerp_gain = dither.x * 0.20000000298023223876953125f;
        float w = lerp(weight_accum, max(weight_accum, abs_coc0), lerp_gain);
        // tap -7。
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-7.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(coc0, 0.0f)) | (spvPackHalf2x16(float2(coc1, 0.0f)) << 16u);
        // tap -6。
        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-6.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc2 = abs(coc2);
        w = lerp(w, max(w, abs_coc2), lerp_gain);
        // tap -5。
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-5.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(coc2, 0.0f)) | (spvPackHalf2x16(float2(coc3, 0.0f)) << 16u);
        // tap -4。
        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-4.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc4 = abs(coc4);
        w = lerp(w, max(w, abs_coc4), lerp_gain);
        // tap -3。
        float coc5 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-3.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc5)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(coc4, 0.0f)) | (spvPackHalf2x16(float2(coc5, 0.0f)) << 16u);
        // tap -2。
        float coc6 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-2.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc6 = abs(coc6);
        w = lerp(w, max(w, abs_coc6), lerp_gain);
        // tap -1。
        float coc7 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-1.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc7)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(coc6, 0.0f)) | (spvPackHalf2x16(float2(coc7, 0.0f)) << 16u);
        // tap 0。
        float coc8 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (dither_y * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc8 = abs(coc8);
        w = lerp(w, max(w, abs_coc8), lerp_gain);
        // tap +1。
        float coc9 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc9)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(coc8, 0.0f)) | (spvPackHalf2x16(float2(coc9, 0.0f)) << 16u);
        // tap +2。
        float coc10 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc10 = abs(coc10);
        w = lerp(w, max(w, abs_coc10), lerp_gain);
        // tap +3。
        float coc11 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc11)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][5u] = spvPackHalf2x16(float2(coc10, 0.0f)) | (spvPackHalf2x16(float2(coc11, 0.0f)) << 16u);
        // tap +4。
        float coc12 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc12 = abs(coc12);
        w = lerp(w, max(w, abs_coc12), lerp_gain);
        // tap +5。
        float coc13 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc13)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][6u] = spvPackHalf2x16(float2(coc12, 0.0f)) | (spvPackHalf2x16(float2(coc13, 0.0f)) << 16u);
        // tap +6。
        float coc14 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc14 = abs(coc14);
        w = lerp(w, max(w, abs_coc14), lerp_gain);
        // tap +7。
        float coc15 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc15)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][7u] = spvPackHalf2x16(float2(coc14, 0.0f)) | (spvPackHalf2x16(float2(coc15, 0.0f)) << 16u);
        // tap +8。
        float coc16 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc16 = abs(coc16);
        weight_accum_next = lerp(w, max(w, abs_coc16), lerp_gain);
        // 更新该环的 CoC 最小/最大跨度。
        coc_min_next = min(min(min(min(min(min(min(min(min(coc_min, abs_coc0), abs_coc2), abs_coc4), abs_coc6), abs_coc8), abs_coc10), abs_coc12), abs_coc14), abs_coc16);
        coc_max_next = max(max(max(max(max(max(max(max(max(coc_max, abs_coc0), abs_coc2), abs_coc4), abs_coc6), abs_coc8), abs_coc10), abs_coc12), abs_coc14), abs_coc16);
        ring_samples[gl_LocalInvocationIndex][8u] = spvPackHalf2x16(float2(coc16, 0.0f));
        // 清空环槽位位图。
        for (uint i = 0u; i < 8u; )
        {
            ring_bits[i] = 0u;
            i++;
            continue;
        }
        // 17 个 tap 标记到 8 个槽位。
        for (uint tap = 0u; tap < 17u; tap++)
        {
            uint par = tap & 1u;
            uint half = tap >> 1u;
            float2 slot_coc = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][half] >> 16u) & 65535u).x);
            float tap_coc = (float(int(tap - 8u)) + dither_y) * coc_step;
            for (int slot = 0; slot < 8; slot++)
            {
                if ((tap_coc / (((float(slot) + tap_sub) * 0.25f) - 1.0f)) >= slot_coc[par])
                {
                    ring_bits[slot] |= (1u << (tap & 31u));
                }
            }
        }
        // 逐槽解析。
        float4 slot_color;
        ring_weight_acc_next = ring_weight_acc;
        depth_weight_next = depth_weight;
        slot_color = 0.0f.xxxx;
        float slot_weight_next;
        float slot_depth_next;
        float4 slot_color_next;
        for (int slot = 0; slot < 8; ring_weight_acc_next = slot_depth_next, depth_weight_next = slot_weight_next, slot_color = slot_color_next, slot++)
        {
            float slot_frac = ((float(slot) + tap_sub) * 0.25f) - 1.0f;
            float abs_frac = abs(slot_frac);
            slot_weight_next = depth_weight_next + abs_frac;
            if (ring_bits[slot] == 0u)
            {
                slot_depth_next = ring_weight_acc_next;
                slot_color_next = slot_color;
                continue;
            }
            bool positive = slot_frac >= 0.0f;
            float lo_coc;
            float hi_coc;
            uint tap_idx;
            if (positive)
            {
                // 正向:最前位 CoC 对。
                uint low_bit = firstbitlow(ring_bits[slot]);
                uint prev_bit = uint(max(0, (int(low_bit) - 1)));
                uint low_half = low_bit >> 1u;
                float2 slot_coc_a = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][low_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][low_half] >> 16u) & 65535u).x);
                uint prev_half = prev_bit >> 1u;
                float2 slot_coc_b = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][prev_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][prev_half] >> 16u) & 65535u).x);
                lo_coc = slot_coc_a[low_bit & 1u];
                hi_coc = slot_coc_b[prev_bit & 1u];
                tap_idx = low_bit;
            }
            else
            {
                // 负向:最高位 CoC 对。
                uint high_bit = firstbithigh(ring_bits[slot]);
                uint next_bit = min(16u, (high_bit + 1u));
                uint high_half = high_bit >> 1u;
                float2 slot_coc_a = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][high_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][high_half] >> 16u) & 65535u).x);
                uint next_half = next_bit >> 1u;
                float2 slot_coc_b = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][next_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][next_half] >> 16u) & 65535u).x);
                lo_coc = slot_coc_a[high_bit & 1u];
                hi_coc = slot_coc_b[next_bit & 1u];
                tap_idx = high_bit;
            }
            float tap_coc = (float(int(tap_idx - 8u)) + dither_y) * coc_step;
            float coc_at_slot = tap_coc / slot_frac;
            float delta = coc_step * (positive ? 1.0f : (-1.0f));
            float lo_edge = tap_coc - delta;
            float hi_edge = hi_coc - (coc_at_slot - (delta / slot_frac));
            float slot_span = abs_frac * ((ring_bits[slot] != 0u) ? 1.0f : 0.0f);
            float resolved = lerp(lo_edge, tap_coc, clamp(hi_edge / (hi_edge + (-(lo_coc - coc_at_slot))), 0.0f, 1.0f));
            float coc_at_resolved = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * resolved), 0.0f).x, neg_coc, coc_limit);
            // 遮挡降权。
            float occlusion = log2(max(0.0f, abs(lo_coc) * 0.125f)) * smoothstep(4.0f, 0.0f, coc_max_next - coc_min_next);
            float occ_lane = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 1u);
            float occ_min = min(occlusion, occ_lane);
            float occ_wide = WaveReadLaneAt(occ_min, WaveGetLaneIndex() ^ 8u);
            if (lo_coc > coc_limit)
            {
                slot_depth_next = ring_weight_acc_next;
                slot_color_next = slot_color;
                continue;
            }
            float edge_eps = hi_edge * 0.100000001490116119384765625f;
            bool close_to_edge;
            if (((resolved / slot_frac) - coc_at_resolved) < edge_eps)
            {
                close_to_edge = (coc_at_resolved - lo_coc) < edge_eps;
            }
            else
            {
                close_to_edge = false;
            }
            float ring_color;
            float ring_weight;
            if (close_to_edge)
            {
                // 近边缘:直接用解析点的 CoC,渐近权重 11 -> 8。
                float abs_resolved = abs(coc_at_resolved);
                if (abs_resolved >= 11.0f)
                {
                    slot_depth_next = ring_weight_acc_next;
                    slot_color_next = slot_color;
                    continue;
                }
                ring_color = ring_weight_acc_next;
                ring_weight = slot_span * smoothstep(11.0f, 8.0f, abs_resolved);
            }
            else
            {
                float abs_hi = abs(hi_coc);
                if (abs_hi >= 11.0f)
                {
                    slot_depth_next = ring_weight_acc_next;
                    slot_color_next = slot_color;
                    continue;
                }
                float w2 = slot_span * smoothstep(11.0f, 8.0f, abs_hi);
                float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.20000000298023223876953125f * coc_limit, 0.0f, lo_coc);
                ring_color = ring_weight_acc_next + (w2 * depth_gate);
                ring_weight = w2 * (1.0f - depth_gate);
            }
            slot_depth_next = ring_color;
            slot_color_next = slot_color + (color_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (close_to_edge ? resolved : lo_edge)), min(occ_min, occ_wide)) * ring_weight);
        }
        // 累加该环的颜色贡献。
        color_accum_next = color_accum + slot_color;
    }
    // 散射项。
    float scatter = ring_weight_acc * smoothstep(0.00999999977648258209228515625f * color_accum.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * color_accum.w), ring_weight_acc);
    float4 final_color;
    [branch]
    if (int(scatter > 0.0f) != 0)
    {
        // 16 点背景散射。
        float neg_coc_bg = -coc_limit;
        float4 bg_accum;
        bg_accum = 0.0f.xxxx;
        float4 bg_accum_next;
        float max_bg_coc_next;
        float max_bg_coc = neg_coc_bg;
        uint s = 0u;
        for (; s < 16u; bg_accum = bg_accum_next, max_bg_coc = max_bg_coc_next, s++)
        {
            float2 rnd = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(s)) + 0.5f.xx) + dither.xy);
            float r = sqrt(rnd.x);
            float phi = rnd.y * 6.283185482025146484375f;
            float2 bg_uv = pixel_uv + (((float2(cos(phi) * r, sin(phi) * r) * 1.0f) / float2(color_tex_dims)) * weight_accum);
            float4 coc_sample = coc_tex.SampleLevel(sampler_nnc, bg_uv, 0.0f);
            float bg_coc = coc_sample.x;
            if (bg_coc > (max_bg_coc - 10.0f))
            {
                bg_accum_next = bg_accum + color_tex.SampleLevel(sampler_llc, bg_uv, 0.0f);
                max_bg_coc_next = max(max_bg_coc, bg_coc);
            }
            else
            {
                bg_accum_next = bg_accum;
                max_bg_coc_next = max_bg_coc;
            }
        }
        final_color = color_accum + ((bg_accum / max(1.0f, bg_accum.w).xxxx) * scatter);
    }
    else
    {
        final_color = color_accum;
    }
    // 合并上采样与聚散结果并归一化。
    float4 combined = (upsample * 0.125f.xxxx) + (final_color / depth_weight.xxxx);
    img_output[pixel] = float4((combined / max(1.0000000133514319600180897396058e-10f, combined.w).xxxx).xyz, 1.0f);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _dof_fullres.cs.hlsl_cs_main_72a2a95d73e37edc.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// 72a2 变体的 cs 段:四环 x 17 tap 聚散采集,每环 12 槽(除数 1/6),
//   抖动子步进改用 dither.z 决定(tap_sub = 1-dither.z 或 dither.z)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct DofTileOffsets
{
    uint offsets[8];
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    float bokeh_anisotropy : packoffset(c0);
    uint2 blue_noise_texture_dims : packoffset(c0.y);
    uint2 color_tex_dims : packoffset(c1);
    uint2 coc_tiles_dilated_tex_dims : packoffset(c1.z);
    uint2 depth_tex_dims : packoffset(c2);
    uint2 coc_tex_dims : packoffset(c2.z);
    uint2 img_output_dims : packoffset(c3);
    uint2 halfres_tex_dims : packoffset(c3.z);
};

cbuffer spatial_resolve_offsets : register(b21, space0)
{
    int4 spatial_resolve_offsets_spatial_resolve_offsets_array[512] : packoffset(c0);
};

ByteAddressBuffer tile_offsets_buffer : register(t22, space0);
ByteAddressBuffer tile_coords_buffer : register(t23, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);
Texture2D<float4> halfres_tex : register(t20, space0);

static uint3 gl_WorkGroupID;
static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_WorkGroupID : SV_GroupID;
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint ring_samples[64][9];

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    // 当前 8x8 像素块与象限。
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(0) + gl_WorkGroupID.x) * 8 + 0));
    uint2 in_tile = pixel & uint2(1u, 1u);
    uint quadrant = in_tile.x + (in_tile.y * 2u);
    // 半分辨率上采样。
    float4 upsample;
    upsample = 0.0f.xxxx;
    for (uint i = 0u; i < 8u; )
    {
        upsample += halfres_tex.Load(int3((pixel / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_spatial_resolve_offsets_array[(quadrant * 16u) + i].xy), 0u));
        i++;
        continue;
    }
    float2 pixel_uv = (float2(int2(pixel)) + 0.5f.xx) / float2(img_output_dims);
    // CoC 上限。
    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, pixel_uv, 0.0f).x);
    // 蓝色噪声抖动。
    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    float dither_z = dither.z;
    float dither_y = dither.y;
    float coc_step = coc_limit * 0.125f;
    float tap_seed = dither.x;
    // 用 dither.z 决定子步进:取 1-z 或 z。
    bool use_low = dither_z < 0.5f;
    float tap_sub;
    if (use_low)
    {
        tap_sub = 1.0f - dither_z;
    }
    else
    {
        tap_sub = dither_z;
    }
    float tap_sign = use_low ? (-1.0f) : 1.0f;
    // 累加器初始化。
    float4 color_accum;
    float weight_accum;
    float depth_weight;
    float ring_weight_acc;
    color_accum = 0.0f.xxxx;
    weight_accum = 0.0f;
    depth_weight = 0.0f;
    ring_weight_acc = 0.0f;
    float4 color_accum_next;
    float weight_accum_next;
    float coc_min_next;
    float coc_max_next;
    uint ring_bits[12];
    float depth_weight_next;
    float ring_weight_acc_next;
    uint ring = 0u;
    float coc_min = 10000000000.0f;
    float coc_max = -10000000000.0f;
    // 四环 x 17 tap。
    [loop]
    for (; ring < 4u; color_accum = color_accum_next, weight_accum = weight_accum_next, depth_weight = depth_weight_next, ring_weight_acc = ring_weight_acc_next, ring++, coc_min = coc_min_next, coc_max = coc_max_next)
    {
        float angle = (float(ring) + tap_seed) * 0.785398185253143310546875f;
        float2 dir = float2(cos(angle), sin(angle));
        float2 bokeh_dir;
        if (bokeh_anisotropy > 0.0f)
        {
            // 各向异性拉伸。
            float4 ctv = mul(frame_constants_view_constants[0u].clip_to_view, float4((pixel_uv - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 view_dir = ctv.xy / ctv.z.xx;
            view_dir.y = view_dir.y * (-1.0f);
            float2 norm_dir = normalize(view_dir);
            float2x2 rot = float2x2(norm_dir, float2(-norm_dir.y, norm_dir.x));
            float aniso = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * bokeh_anisotropy) * dot(view_dir, view_dir)));
            bokeh_dir = mul(transpose(rot), mul(rot, dir) * float2(aniso, 1.0f / aniso));
        }
        else
        {
            bokeh_dir = dir;
        }
        float2 step_px = bokeh_dir * (tap_sign.xx / float2(color_tex_dims));
        float neg_coc = -coc_limit;
        // tap -8。
        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-8.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc0 = abs(coc0);
        float lerp_gain = tap_seed * 0.20000000298023223876953125f;
        float w = lerp(weight_accum, max(weight_accum, abs_coc0), lerp_gain);
        // tap -7。
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-7.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(coc0, 0.0f)) | (spvPackHalf2x16(float2(coc1, 0.0f)) << 16u);
        // tap -6。
        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-6.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc2 = abs(coc2);
        w = lerp(w, max(w, abs_coc2), lerp_gain);
        // tap -5。
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-5.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(coc2, 0.0f)) | (spvPackHalf2x16(float2(coc3, 0.0f)) << 16u);
        // tap -4。
        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-4.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc4 = abs(coc4);
        w = lerp(w, max(w, abs_coc4), lerp_gain);
        // tap -3。
        float coc5 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-3.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc5)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(coc4, 0.0f)) | (spvPackHalf2x16(float2(coc5, 0.0f)) << 16u);
        // tap -2。
        float coc6 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-2.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc6 = abs(coc6);
        w = lerp(w, max(w, abs_coc6), lerp_gain);
        // tap -1。
        float coc7 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-1.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc7)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(coc6, 0.0f)) | (spvPackHalf2x16(float2(coc7, 0.0f)) << 16u);
        // tap 0。
        float coc8 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (dither_y * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc8 = abs(coc8);
        w = lerp(w, max(w, abs_coc8), lerp_gain);
        // tap +1。
        float coc9 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc9)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(coc8, 0.0f)) | (spvPackHalf2x16(float2(coc9, 0.0f)) << 16u);
        // tap +2。
        float coc10 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc10 = abs(coc10);
        w = lerp(w, max(w, abs_coc10), lerp_gain);
        // tap +3。
        float coc11 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc11)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][5u] = spvPackHalf2x16(float2(coc10, 0.0f)) | (spvPackHalf2x16(float2(coc11, 0.0f)) << 16u);
        // tap +4。
        float coc12 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc12 = abs(coc12);
        w = lerp(w, max(w, abs_coc12), lerp_gain);
        // tap +5。
        float coc13 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc13)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][6u] = spvPackHalf2x16(float2(coc12, 0.0f)) | (spvPackHalf2x16(float2(coc13, 0.0f)) << 16u);
        // tap +6。
        float coc14 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc14 = abs(coc14);
        w = lerp(w, max(w, abs_coc14), lerp_gain);
        // tap +7。
        float coc15 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc15)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][7u] = spvPackHalf2x16(float2(coc14, 0.0f)) | (spvPackHalf2x16(float2(coc15, 0.0f)) << 16u);
        // tap +8。
        float coc16 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc16 = abs(coc16);
        weight_accum_next = lerp(w, max(w, abs_coc16), lerp_gain);
        // 更新该环 CoC 最小/最大跨度。
        coc_min_next = min(min(min(min(min(min(min(min(min(coc_min, abs_coc0), abs_coc2), abs_coc4), abs_coc6), abs_coc8), abs_coc10), abs_coc12), abs_coc14), abs_coc16);
        coc_max_next = max(max(max(max(max(max(max(max(max(coc_max, abs_coc0), abs_coc2), abs_coc4), abs_coc6), abs_coc8), abs_coc10), abs_coc12), abs_coc14), abs_coc16);
        ring_samples[gl_LocalInvocationIndex][8u] = spvPackHalf2x16(float2(coc16, 0.0f));
        // 清空 12 个环槽位位图。
        for (uint i = 0u; i < 12u; )
        {
            ring_bits[i] = 0u;
            i++;
            continue;
        }
        // 17 个 tap 标记到 12 个槽位。
        for (uint tap = 0u; tap < 17u; tap++)
        {
            uint par = tap & 1u;
            uint half = tap >> 1u;
            float2 slot_coc = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][half] >> 16u) & 65535u).x);
            float tap_coc = (float(int(tap - 8u)) + dither_y) * coc_step;
            for (int slot = 0; slot < 12; slot++)
            {
                if ((tap_coc / (((float(slot) + tap_sub) * 0.16666667163372039794921875f) - 1.0f)) >= slot_coc[par])
                {
                    ring_bits[slot] |= (1u << (tap & 31u));
                }
            }
        }
        // 逐槽解析。
        float4 slot_color;
        ring_weight_acc_next = ring_weight_acc;
        depth_weight_next = depth_weight;
        slot_color = 0.0f.xxxx;
        float slot_weight_next;
        float slot_depth_next;
        float4 slot_color_next;
        for (int slot = 0; slot < 12; ring_weight_acc_next = slot_depth_next, depth_weight_next = slot_weight_next, slot_color = slot_color_next, slot++)
        {
            float slot_frac = ((float(slot) + tap_sub) * 0.16666667163372039794921875f) - 1.0f;
            float abs_frac = abs(slot_frac);
            slot_weight_next = depth_weight_next + abs_frac;
            if (ring_bits[slot] == 0u)
            {
                slot_depth_next = ring_weight_acc_next;
                slot_color_next = slot_color;
                continue;
            }
            bool positive = slot_frac >= 0.0f;
            float lo_coc;
            float hi_coc;
            uint tap_idx;
            if (positive)
            {
                // 正向:最前位 CoC 对。
                uint low_bit = firstbitlow(ring_bits[slot]);
                uint prev_bit = uint(max(0, (int(low_bit) - 1)));
                uint low_half = low_bit >> 1u;
                float2 slot_coc_a = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][low_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][low_half] >> 16u) & 65535u).x);
                uint prev_half = prev_bit >> 1u;
                float2 slot_coc_b = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][prev_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][prev_half] >> 16u) & 65535u).x);
                lo_coc = slot_coc_a[low_bit & 1u];
                hi_coc = slot_coc_b[prev_bit & 1u];
                tap_idx = low_bit;
            }
            else
            {
                // 负向:最高位 CoC 对。
                uint high_bit = firstbithigh(ring_bits[slot]);
                uint next_bit = min(16u, (high_bit + 1u));
                uint high_half = high_bit >> 1u;
                float2 slot_coc_a = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][high_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][high_half] >> 16u) & 65535u).x);
                uint next_half = next_bit >> 1u;
                float2 slot_coc_b = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][next_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][next_half] >> 16u) & 65535u).x);
                lo_coc = slot_coc_a[high_bit & 1u];
                hi_coc = slot_coc_b[next_bit & 1u];
                tap_idx = high_bit;
            }
            float tap_coc = (float(int(tap_idx - 8u)) + dither_y) * coc_step;
            float coc_at_slot = tap_coc / slot_frac;
            float delta = coc_step * (positive ? 1.0f : (-1.0f));
            float lo_edge = tap_coc - delta;
            float hi_edge = hi_coc - (coc_at_slot - (delta / slot_frac));
            float slot_span = abs_frac * ((ring_bits[slot] != 0u) ? 1.0f : 0.0f);
            float resolved = lerp(lo_edge, tap_coc, clamp(hi_edge / (hi_edge + (-(lo_coc - coc_at_slot))), 0.0f, 1.0f));
            float coc_at_resolved = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * resolved), 0.0f).x, neg_coc, coc_limit);
            // 遮挡降权。
            float occlusion = log2(max(0.0f, abs(lo_coc) * 0.125f)) * smoothstep(4.0f, 0.0f, coc_max_next - coc_min_next);
            float occ_lane = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 1u);
            float occ_min = min(occlusion, occ_lane);
            float occ_wide = WaveReadLaneAt(occ_min, WaveGetLaneIndex() ^ 8u);
            if (lo_coc > coc_limit)
            {
                slot_depth_next = ring_weight_acc_next;
                slot_color_next = slot_color;
                continue;
            }
            float edge_eps = hi_edge * 0.100000001490116119384765625f;
            bool close_to_edge;
            if (((resolved / slot_frac) - coc_at_resolved) < edge_eps)
            {
                close_to_edge = (coc_at_resolved - lo_coc) < edge_eps;
            }
            else
            {
                close_to_edge = false;
            }
            float ring_color;
            float ring_weight;
            if (close_to_edge)
            {
                // 近边缘:直接用解析点的 CoC,渐近权重 11 -> 8。
                float abs_resolved = abs(coc_at_resolved);
                if (abs_resolved >= 11.0f)
                {
                    slot_depth_next = ring_weight_acc_next;
                    slot_color_next = slot_color;
                    continue;
                }
                ring_color = ring_weight_acc_next;
                ring_weight = slot_span * smoothstep(11.0f, 8.0f, abs_resolved);
            }
            else
            {
                float abs_hi = abs(hi_coc);
                if (abs_hi >= 11.0f)
                {
                    slot_depth_next = ring_weight_acc_next;
                    slot_color_next = slot_color;
                    continue;
                }
                float w2 = slot_span * smoothstep(11.0f, 8.0f, abs_hi);
                float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.20000000298023223876953125f * coc_limit, 0.0f, lo_coc);
                ring_color = ring_weight_acc_next + (w2 * depth_gate);
                ring_weight = w2 * (1.0f - depth_gate);
            }
            slot_depth_next = ring_color;
            slot_color_next = slot_color + (color_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (close_to_edge ? resolved : lo_edge)), min(occ_min, occ_wide)) * ring_weight);
        }
        // 累加该环的颜色贡献。
        color_accum_next = color_accum + slot_color;
    }
    // 散射项。
    float scatter = ring_weight_acc * smoothstep(0.00999999977648258209228515625f * color_accum.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * color_accum.w), ring_weight_acc);
    float4 final_color;
    [branch]
    if (int(scatter > 0.0f) != 0)
    {
        // 16 点背景散射。
        float neg_coc_bg = -coc_limit;
        float4 bg_accum;
        bg_accum = 0.0f.xxxx;
        float4 bg_accum_next;
        float max_bg_coc_next;
        float max_bg_coc = neg_coc_bg;
        uint s = 0u;
        for (; s < 16u; bg_accum = bg_accum_next, max_bg_coc = max_bg_coc_next, s++)
        {
            float2 rnd = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(s)) + 0.5f.xx) + dither.xy);
            float r = sqrt(rnd.x);
            float phi = rnd.y * 6.283185482025146484375f;
            float2 bg_uv = pixel_uv + (((float2(cos(phi) * r, sin(phi) * r) * 1.0f) / float2(color_tex_dims)) * weight_accum);
            float4 coc_sample = coc_tex.SampleLevel(sampler_nnc, bg_uv, 0.0f);
            float bg_coc = coc_sample.x;
            if (bg_coc > (max_bg_coc - 10.0f))
            {
                bg_accum_next = bg_accum + color_tex.SampleLevel(sampler_llc, bg_uv, 0.0f);
                max_bg_coc_next = max(max_bg_coc, bg_coc);
            }
            else
            {
                bg_accum_next = bg_accum;
                max_bg_coc_next = max_bg_coc;
            }
        }
        final_color = color_accum + ((bg_accum / max(1.0f, bg_accum.w).xxxx) * scatter);
    }
    else
    {
        final_color = color_accum;
    }
    // 合并上采样与聚散结果并归一化。
    float4 combined = (upsample * 0.125f.xxxx) + (final_color / depth_weight.xxxx);
    img_output[pixel] = float4((combined / max(1.0000000133514319600180897396058e-10f, combined.w).xxxx).xyz, 1.0f);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
