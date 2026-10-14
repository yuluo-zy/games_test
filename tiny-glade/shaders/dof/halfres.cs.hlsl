// PATCHED: modules all_6f1dd5dd8a66e104.bin:cs_halfres_all, all_72a2a95d73e37edc.bin:cs_halfres_all, res_6f1dd5dd8a66e104.bin:cs_halfres, res_72a2a95d73e37edc.bin:cs_halfres are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
// === 中文说明 ===
// 半分辨率散焦 (dof/halfres):按 tile 调度 8x8 线程,用膨胀后的 CoC tile 作为
//   coc_limit,蓝噪声抖动的 2 环 x 17 tap 各向异性 CoC 采集 pack 进 groupshared
//   环表;按位域把 17 个 tap 分类到 8 个深度桶,用 firstbitlow/firstbithigh 解析
//   遮挡后聚集前景,再对低前景贡献像素做 16 样本背景散射并归一化写出。
// 入口: cs_halfres (compute, numthreads 8x8x1, offsets[4], occlude_gate=true)、
//   cs_halfres_all (compute, numthreads 8x8x1, offsets[5], occlude_gate=false)。
// 变体: 四个模块 (cs_halfres/cs_halfres_all x 6f1d/72a2) 均 lifted。
// 绑定: blue_noise t14, color_tex t15, coc_tiles_dilated t16, coc_tex t18,
//   img_output u19 (Rgba16f), tile_offsets t20 (StructuredBuffer), tile_coords
//   t21 (StructuredBuffer), $Globals b13, frame_constants set1/b0。
#if !((defined(TGV_cs_halfres_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_cs_halfres_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_cs_halfres_all_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_cs_halfres_all_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/dof/halfres.cs.hlsl | MERGE (dof-aa review, 2026-10-13): B loop/pack structure (OpLoopMerge 6, f16x2 masking unpack, halfres_core shared by cs_halfres/cs_halfres_all via offsets_slot+occlude_gate) + A RWTexture2D<half4> img_output (Rgba16f). INCREASING near-branch ramp (smoothstep(8,11), abs<8) kept (target halfres). 6f1d noise scheme only (72a2 residual).
//   Half-res bokeh: tile-dispatch 8x8, CoC limit from dilated tiles, blue-noise
//   dithered 2-ring x 17-tap anisotropic CoC gather into a packed groupshared
//   ring table, bitfield occlusion classification, a 16-sample background
//   scatter, and the final foreground gather.  Status: O (structure from
//   disassembly).
//   本手写段为重构参考;对应 (变体, 阶段) 组合的真实行为以 lifted 段为准。

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
#include "dof/multires.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float bokeh_anisotropy;      // 各向异性强度(>0 时把采样方向旋转/缩放)
    uint2 blue_noise_texture_dims;
    uint2 color_tex_dims;
    uint2 coc_tiles_dilated_tex_dims;
    uint2 depth_tex_dims;
    uint2 coc_tex_dims;
    uint2 img_output_dims;
};

Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<half4> img_output : register(u19, space0);
StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t20, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t21, space0);

// 每个线程(8x8=64 lane)一张 17-tap 环表:9 个 uint,每个 pack 一对 f16 CoC。
groupshared uint dof_ring_samples[64][9];

// 半分辨率散焦核心:cs_halfres 与 cs_halfres_all 共享。
//   offsets_slot: tile_offsets_buffer 的槽位(4=近景, 5=全量);
//   occlude_gate: 是否启用 8->11 的递增近景斜坡(近景入口开启)。
void halfres_core(uint offsets_slot,
                  bool occlude_gate,
                  uint3 gl_WorkGroupID,
                  uint3 gl_GlobalInvocationID,
                  uint gl_LocalInvocationIndex)
{
    // 由 tile 坐标 + 组内 8x8 位置得到全局像素坐标。
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer[tile_offsets_buffer[0].offsets[offsets_slot] + gl_WorkGroupID.x]);
    float2 uv = (float2(int2(pixel)) + 0.5f) / float2(img_output_dims);

    // CoC 上限:颜色高度决定的最大散焦半径,与膨胀 tile 的 CoC 取小。
    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, uv, 0.0f).x);

    // 蓝噪声抖动的环 tap 偏移。
    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u))
        + uint2(int2(int(frame_index * 59u), int(frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f)
        * 0.00390625f) + 0.001953125f;

    // 抖动样本派生的 Halton 式子像素偏移。
    uint2 hash_pos = uint2(7u, 7u) + pixel;
    float sub = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(hash_pos.x))
        + (0.005837149918079376220703125f * float(hash_pos.y)))) - 0.5f) * 0.5f;
    float sub2 = 0.5f + sub;
    float dither_y = dither.y;
    float coc_step = coc_limit * 0.125f;   // 每 tap 的 CoC 步长 = 1/8 CoC 上限
    float frame_rand = 5.588237762451171875f * float(frame_index % 64u);
    float tap_seed = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(pixel.x) + frame_rand))
        + (0.005837149918079376220703125f * (float(pixel.y) + frame_rand))));
    bool use_low = sub2 < 0.5f;
    float tap_sub = use_low ? (0.5f - sub) : sub2;
    float tap_sign = use_low ? (-1.0f) : 1.0f;

    // 环循环的跨迭代累积量。
    float4 color_accum = 0.0f.xxxx;
    float weight_accum = 0.0f;
    float depth_weight = 0.0f;
    float4 ring_color_accum = 0.0f.xxxx;
    uint ring_bits[8];
    ring_bits[0] = 0u; ring_bits[1] = 0u; ring_bits[2] = 0u; ring_bits[3] = 0u;
    ring_bits[4] = 0u; ring_bits[5] = 0u; ring_bits[6] = 0u; ring_bits[7] = 0u;
    float coc_min = 1e10f;
    float coc_max = -1e10f;

    // 两环,每环 17 tap,沿(可选各向异性的)散焦方向。
    for (uint ring = 0u; ring < 2u; ++ring)
    {
        float angle = (float(ring) + tap_seed) * 1.57079637050628662109375f;
        float2 dir = float2(cos(angle), sin(angle));
        float2 bokeh_dir;
        if (bokeh_anisotropy > 0.0f)
        {
            // 各向异性:把圆环方向按视线方向旋转并缩放(aniso/1-aniso)。
            float4 ctv = mul(view_constants[0].clip_to_view, float4((uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f));
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

        // 17 个 CoC 采样,步长从 -8 到 +8(乘 coc_step),逐对 pack 进环表。
        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float m = abs(coc0);
        float w = lerp(weight_accum, max(weight_accum, m), lerp_gain);
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][0] = dof_pack_half2(float2(coc0, 0.0f)) | (dof_pack_half2(float2(coc1, 0.0f)) << 16u);

        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc2)), lerp_gain);
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][1] = dof_pack_half2(float2(coc2, 0.0f)) | (dof_pack_half2(float2(coc3, 0.0f)) << 16u);

        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc4)), lerp_gain);
        float coc5 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc5)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][2] = dof_pack_half2(float2(coc4, 0.0f)) | (dof_pack_half2(float2(coc5, 0.0f)) << 16u);

        float coc6 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc6)), lerp_gain);
        float coc7 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc7)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][3] = dof_pack_half2(float2(coc6, 0.0f)) | (dof_pack_half2(float2(coc7, 0.0f)) << 16u);

        float coc8 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (dither_y * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc8)), lerp_gain);
        float coc9 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc9)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][4] = dof_pack_half2(float2(coc8, 0.0f)) | (dof_pack_half2(float2(coc9, 0.0f)) << 16u);

        float coc10 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc10)), lerp_gain);
        float coc11 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc11)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][5] = dof_pack_half2(float2(coc10, 0.0f)) | (dof_pack_half2(float2(coc11, 0.0f)) << 16u);

        float coc12 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc12)), lerp_gain);
        float coc13 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc13)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][6] = dof_pack_half2(float2(coc12, 0.0f)) | (dof_pack_half2(float2(coc13, 0.0f)) << 16u);

        float coc14 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc14)), lerp_gain);
        float coc15 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc15)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][7] = dof_pack_half2(float2(coc14, 0.0f)) | (dof_pack_half2(float2(coc15, 0.0f)) << 16u);

        float coc16 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        weight_accum = lerp(w, max(w, abs(coc16)), lerp_gain);
        coc_min = min(min(min(min(min(min(min(min(min(coc_min, abs(coc0)), abs(coc2)), abs(coc4)), abs(coc6)), abs(coc8)), abs(coc10)), abs(coc12)), abs(coc14)), abs(coc16));
        coc_max = max(max(max(max(max(max(max(max(max(coc_max, abs(coc0)), abs(coc2)), abs(coc4)), abs(coc6)), abs(coc8)), abs(coc10)), abs(coc12)), abs(coc14)), abs(coc16));
        dof_ring_samples[gl_LocalInvocationIndex][8] = dof_pack_half2(float2(coc16, 0.0f));

        // 把 17 个 tap 按深度桶分类到 8 个槽位的位域。
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

        // 为每个桶聚集颜色,用相邻 tap 之间的 firstbitlow/firstbithigh 遮挡来
        // 解析环跨度。
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
                // 正向桶:取最低位 tap,再取前一位 tap 组成 [lo, hi]。
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
                // 负向桶:取最高位 tap,再取下一位 tap。
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
            float coc_at_resolved = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * resolved), 0.0f).x, neg_coc, coc_limit);
            float occlusion = log2(max(0.0f, abs(lo_coc) * 0.125f)) * smoothstep(4.0f, 0.0f, coc_max - coc_min);
            float occ_lane = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 1u);
            occlusion = min(occlusion, occ_lane);
            float occ_wide = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 8u);
            if (lo_coc > coc_limit)
            {
                continue;
            }
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
            // 半分辨率递增近景斜坡(target):拒绝 |coc| < 8,权重在 8 -> 11 间渐变。
            // cs_halfres_all (occlude_gate=false) 省略该斜坡。
            if (close_to_edge)
            {
                if (occlude_gate)
                {
                    float abs_resolved = abs(coc_at_resolved);
                    if (abs_resolved < 8.0f)
                    {
                        continue;
                    }
                    ring_color = slot_color;
                    ring_weight = slot_span * smoothstep(8.0f, 11.0f, abs_resolved);
                }
                else
                {
                    ring_color = slot_color;
                    ring_weight = slot_span;
                }
            }
            else
            {
                if (occlude_gate)
                {
                    float abs_hi = abs(hi_coc);
                    if (abs_hi < 8.0f)
                    {
                        continue;
                    }
                    float w2 = slot_span * smoothstep(8.0f, 11.0f, abs_hi);
                    float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.2f * coc_limit, 0.0f, lo_coc);
                    ring_color = slot_color + (w2 * depth_gate);
                    ring_weight = w2 * (1.0f - depth_gate);
                }
                else
                {
                    float w2 = slot_span;
                    float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.2f * coc_limit, 0.0f, lo_coc);
                    ring_color = slot_color + (w2 * depth_gate);
                    ring_weight = w2 * (1.0f - depth_gate);
                }
            }
            depth_weight = ring_color.w;
            slot_color = slot_color + (color_tex.SampleLevel(sampler_nnc, uv + (step_px * (close_to_edge ? resolved : lo_edge)), min(occlusion, occ_wide)) * ring_weight);
        }
        ring_color_accum = ring_color_accum + slot_color;
    }

    // 当前景贡献低时做 16 样本背景散射。
    float scatter = depth_weight * smoothstep(0.01f * ring_color_accum.w, max(1e-5f, 0.2f * ring_color_accum.w), depth_weight);
    float4 final_color;
    if (scatter > 0.0f)
    {
        float neg_coc = -coc_limit;
        float4 bg_accum = 0.0f.xxxx;
        float max_bg_coc = neg_coc;
        for (uint s = 0u; s < 16u; ++s)
        {
            // 半径 sqrt 采样 + 均匀角度,沿背景散焦盘累积颜色。
            float2 rnd = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(s)) + 0.5f) + dither.xy);
            float r = sqrt(rnd.x);
            float phi = rnd.y * 6.283185482025146484375f;
            float2 bg_uv = uv + (((float2(cos(phi) * r, sin(phi) * r) * 1.0f) / float2(color_tex_dims)) * weight_accum);
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
    img_output[pixel] = max(0.0f, final_color / depth_weight);
}

[numthreads(8, 8, 1)]
void cs_halfres(uint3 gl_WorkGroupID : SV_GroupID,
                uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
                uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    halfres_core(4u, true, gl_WorkGroupID, gl_GlobalInvocationID, gl_LocalInvocationIndex);
}

[numthreads(8, 8, 1)]
void cs_halfres_all(uint3 gl_WorkGroupID : SV_GroupID,
                    uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
                    uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    halfres_core(5u, false, gl_WorkGroupID, gl_GlobalInvocationID, gl_LocalInvocationIndex);
}

#endif
#if defined(TGV_cs_halfres_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _dof_halfres.cs.hlsl_cs_halfres_6f1dd5dd8a66e104.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// 6f1d 变体的 cs_halfres 段:与手写 halfres_core(offsets_slot=4, occlude_gate=true)
//   语义一致。2 环 x 17 tap、8 槽(除数 0.25f)、递增近景斜坡 8->11。变量名已语义化。
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
};

ByteAddressBuffer tile_offsets_buffer : register(t20, space0);
ByteAddressBuffer tile_coords_buffer : register(t21, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);

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
    // 像素坐标:tile 坐标(offsets[4]) + 组内 8x8。
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(16) + gl_WorkGroupID.x) * 8 + 0));
    float2 uv = (float2(int2(pixel)) + 0.5f.xx) / float2(img_output_dims);
    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, uv, 0.0f).x);
    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint2 hash_pos = uint2(7u, 7u) + pixel;
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
    float4 ring_color_accum;
    float weight_accum;
    float span_accum;
    float depth_weight;
    ring_color_accum = 0.0f.xxxx;
    weight_accum = 0.0f;
    span_accum = 0.0f;
    depth_weight = 0.0f;
    float4 ring_color_accum_next;
    float weight_accum_next;
    float coc_min_next;
    float coc_max_next;
    uint ring_bits[8];
    float slot_depth;
    float slot_weight;
    uint ring = 0u;
    float coc_min = 10000000000.0f;
    float coc_max = -10000000000.0f;
    [loop]
    for (; ring < 2u; ring_color_accum = ring_color_accum_next, weight_accum = weight_accum_next, span_accum = slot_depth, depth_weight = slot_weight, ring++, coc_min = coc_min_next, coc_max = coc_max_next)
    {
        float angle = (float(ring) + tap_seed) * 1.57079637050628662109375f;
        float2 dir = float2(cos(angle), sin(angle));
        float2 bokeh_dir;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 ctv = mul(frame_constants_view_constants[0u].clip_to_view, float4((uv - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
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
        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-8.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc0 = abs(coc0);
        float lerp_gain = dither.x * 0.20000000298023223876953125f;
        float w = lerp(weight_accum, max(weight_accum, abs_coc0), lerp_gain);
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-7.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(coc0, 0.0f)) | (spvPackHalf2x16(float2(coc1, 0.0f)) << 16u);
        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-6.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc2 = abs(coc2);
        w = lerp(w, max(w, abs_coc2), lerp_gain);
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-5.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(coc2, 0.0f)) | (spvPackHalf2x16(float2(coc3, 0.0f)) << 16u);
        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-4.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc4 = abs(coc4);
        w = lerp(w, max(w, abs_coc4), lerp_gain);
        float coc5 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-3.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc5)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(coc4, 0.0f)) | (spvPackHalf2x16(float2(coc5, 0.0f)) << 16u);
        float coc6 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-2.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc6 = abs(coc6);
        w = lerp(w, max(w, abs_coc6), lerp_gain);
        float coc7 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-1.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc7)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(coc6, 0.0f)) | (spvPackHalf2x16(float2(coc7, 0.0f)) << 16u);
        float coc8 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (dither_y * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc8 = abs(coc8);
        w = lerp(w, max(w, abs_coc8), lerp_gain);
        float coc9 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc9)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(coc8, 0.0f)) | (spvPackHalf2x16(float2(coc9, 0.0f)) << 16u);
        float coc10 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc10 = abs(coc10);
        w = lerp(w, max(w, abs_coc10), lerp_gain);
        float coc11 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc11)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][5u] = spvPackHalf2x16(float2(coc10, 0.0f)) | (spvPackHalf2x16(float2(coc11, 0.0f)) << 16u);
        float coc12 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc12 = abs(coc12);
        w = lerp(w, max(w, abs_coc12), lerp_gain);
        float coc13 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc13)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][6u] = spvPackHalf2x16(float2(coc12, 0.0f)) | (spvPackHalf2x16(float2(coc13, 0.0f)) << 16u);
        float coc14 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc14 = abs(coc14);
        w = lerp(w, max(w, abs_coc14), lerp_gain);
        float coc15 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc15)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][7u] = spvPackHalf2x16(float2(coc14, 0.0f)) | (spvPackHalf2x16(float2(coc15, 0.0f)) << 16u);
        float coc16 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc16 = abs(coc16);
        weight_accum_next = lerp(w, max(w, abs_coc16), lerp_gain);
        coc_min_next = min(min(min(min(min(min(min(min(min(coc_min, abs_coc0), abs_coc2), abs_coc4), abs_coc6), abs_coc8), abs_coc10), abs_coc12), abs_coc14), abs_coc16);
        coc_max_next = max(max(max(max(max(max(max(max(max(coc_max, abs_coc0), abs_coc2), abs_coc4), abs_coc6), abs_coc8), abs_coc10), abs_coc12), abs_coc14), abs_coc16);
        ring_samples[gl_LocalInvocationIndex][8u] = spvPackHalf2x16(float2(coc16, 0.0f));
        for (uint i = 0u; i < 8u; )
        {
            ring_bits[i] = 0u;
            i++;
            continue;
        }
        // 把 17 个 tap 按桶分类到 8 槽位域。
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
        float4 slot_color;
        slot_weight = depth_weight;
        slot_depth = span_accum;
        slot_color = 0.0f.xxxx;
        float slot_depth_next;
        float slot_weight_next;
        float4 slot_color_next;
        for (int slot = 0; slot < 8; slot_weight = slot_weight_next, slot_depth = slot_depth_next, slot_color = slot_color_next, slot++)
        {
            float slot_frac = ((float(slot) + tap_sub) * 0.25f) - 1.0f;
            float abs_frac = abs(slot_frac);
            slot_depth_next = slot_depth + abs_frac;
            if (ring_bits[slot] == 0u)
            {
                slot_weight_next = slot_weight;
                slot_color_next = slot_color;
                continue;
            }
            bool positive = slot_frac >= 0.0f;
            float lo_coc;
            float hi_coc;
            uint tap_idx;
            if (positive)
            {
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
            float coc_at_resolved = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * resolved), 0.0f).x, neg_coc, coc_limit);
            float occlusion = log2(max(0.0f, abs(lo_coc) * 0.125f)) * smoothstep(4.0f, 0.0f, coc_max - coc_min);
            float occ_lane = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 1u);
            float occ_min = min(occlusion, occ_lane);
            float occ_wide = WaveReadLaneAt(occ_min, WaveGetLaneIndex() ^ 8u);
            if (lo_coc > coc_limit)
            {
                slot_weight_next = slot_weight;
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
            float ring_color_w;
            float ring_weight;
            if (close_to_edge)
            {
                float abs_resolved = abs(coc_at_resolved);
                if (abs_resolved < 8.0f)
                {
                    slot_weight_next = slot_weight;
                    slot_color_next = slot_color;
                    continue;
                }
                ring_color_w = slot_weight;
                ring_weight = slot_span * smoothstep(8.0f, 11.0f, abs_resolved);
            }
            else
            {
                float abs_hi = abs(hi_coc);
                if (abs_hi < 8.0f)
                {
                    slot_weight_next = slot_weight;
                    slot_color_next = slot_color;
                    continue;
                }
                float w2 = slot_span * smoothstep(8.0f, 11.0f, abs_hi);
                float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.20000000298023223876953125f * coc_limit, 0.0f, lo_coc);
                ring_color_w = slot_weight + (w2 * depth_gate);
                ring_weight = w2 * (1.0f - depth_gate);
            }
            slot_weight_next = ring_color_w;
            slot_color_next = slot_color + (color_tex.SampleLevel(sampler_nnc, uv + (step_px * (close_to_edge ? resolved : lo_edge)), min(occ_min, occ_wide)) * ring_weight);
        }
        ring_color_accum_next = ring_color_accum + slot_color;
    }
    float scatter = depth_weight * smoothstep(0.00999999977648258209228515625f * ring_color_accum.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * ring_color_accum.w), depth_weight);
    float4 final_color;
    [branch]
    if (int(scatter > 0.0f) != 0)
    {
        float scatter_neg_coc = -coc_limit;
        float4 bg_accum;
        bg_accum = 0.0f.xxxx;
        float4 bg_accum_next;
        float max_bg_coc_next;
        float max_bg_coc = scatter_neg_coc;
        uint s = 0u;
        for (; s < 16u; bg_accum = bg_accum_next, max_bg_coc = max_bg_coc_next, s++)
        {
            float2 rnd = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(s)) + 0.5f.xx) + dither.xy);
            float r = sqrt(rnd.x);
            float phi = rnd.y * 6.283185482025146484375f;
            float2 bg_uv = uv + (((float2(cos(phi) * r, sin(phi) * r) * 1.0f) / float2(color_tex_dims)) * weight_accum);
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
        final_color = ring_color_accum + ((bg_accum / max(1.0f, bg_accum.w).xxxx) * scatter);
    }
    else
    {
        final_color = ring_color_accum;
    }
    img_output[pixel] = max(0.0f.xxxx, final_color / span_accum.xxxx);
}

[numthreads(8, 8, 1)]
void cs_halfres(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
#if defined(TGV_cs_halfres_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _dof_halfres.cs.hlsl_cs_halfres_72a2a95d73e37edc.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// 72a2 变体的 cs_halfres 段:4 环 x 17 tap、12 槽(除数 1/6)、角度步长 pi/4;
//   抖动方案用 dither.z 直接决定 use_low/tap_sub,tap_seed = dither.x。
//   仍开启递增近景斜坡 8->11(occlude_gate=true)。
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
};

ByteAddressBuffer tile_offsets_buffer : register(t20, space0);
ByteAddressBuffer tile_coords_buffer : register(t21, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);

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
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(16) + gl_WorkGroupID.x) * 8 + 0));
    float2 uv = (float2(int2(pixel)) + 0.5f.xx) / float2(img_output_dims);
    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, uv, 0.0f).x);
    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    float dither_z = dither.z;
    float dither_y = dither.y;
    float coc_step = coc_limit * 0.125f;
    float tap_seed = dither.x;
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
    float4 ring_color_accum;
    float weight_accum;
    float span_accum;
    float depth_weight;
    ring_color_accum = 0.0f.xxxx;
    weight_accum = 0.0f;
    span_accum = 0.0f;
    depth_weight = 0.0f;
    float4 ring_color_accum_next;
    float weight_accum_next;
    float coc_min_next;
    float coc_max_next;
    uint ring_bits[12];
    float slot_depth;
    float slot_weight;
    uint ring = 0u;
    float coc_min = 10000000000.0f;
    float coc_max = -10000000000.0f;
    [loop]
    for (; ring < 4u; ring_color_accum = ring_color_accum_next, weight_accum = weight_accum_next, span_accum = slot_depth, depth_weight = slot_weight, ring++, coc_min = coc_min_next, coc_max = coc_max_next)
    {
        float angle = (float(ring) + tap_seed) * 0.785398185253143310546875f;
        float2 dir = float2(cos(angle), sin(angle));
        float2 bokeh_dir;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 ctv = mul(frame_constants_view_constants[0u].clip_to_view, float4((uv - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
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
        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-8.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc0 = abs(coc0);
        float lerp_gain = tap_seed * 0.20000000298023223876953125f;
        float w = lerp(weight_accum, max(weight_accum, abs_coc0), lerp_gain);
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-7.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(coc0, 0.0f)) | (spvPackHalf2x16(float2(coc1, 0.0f)) << 16u);
        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-6.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc2 = abs(coc2);
        w = lerp(w, max(w, abs_coc2), lerp_gain);
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-5.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(coc2, 0.0f)) | (spvPackHalf2x16(float2(coc3, 0.0f)) << 16u);
        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-4.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc4 = abs(coc4);
        w = lerp(w, max(w, abs_coc4), lerp_gain);
        float coc5 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-3.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc5)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(coc4, 0.0f)) | (spvPackHalf2x16(float2(coc5, 0.0f)) << 16u);
        float coc6 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-2.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc6 = abs(coc6);
        w = lerp(w, max(w, abs_coc6), lerp_gain);
        float coc7 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-1.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc7)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(coc6, 0.0f)) | (spvPackHalf2x16(float2(coc7, 0.0f)) << 16u);
        float coc8 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (dither_y * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc8 = abs(coc8);
        w = lerp(w, max(w, abs_coc8), lerp_gain);
        float coc9 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc9)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(coc8, 0.0f)) | (spvPackHalf2x16(float2(coc9, 0.0f)) << 16u);
        float coc10 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc10 = abs(coc10);
        w = lerp(w, max(w, abs_coc10), lerp_gain);
        float coc11 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc11)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][5u] = spvPackHalf2x16(float2(coc10, 0.0f)) | (spvPackHalf2x16(float2(coc11, 0.0f)) << 16u);
        float coc12 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc12 = abs(coc12);
        w = lerp(w, max(w, abs_coc12), lerp_gain);
        float coc13 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc13)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][6u] = spvPackHalf2x16(float2(coc12, 0.0f)) | (spvPackHalf2x16(float2(coc13, 0.0f)) << 16u);
        float coc14 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc14 = abs(coc14);
        w = lerp(w, max(w, abs_coc14), lerp_gain);
        float coc15 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc15)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][7u] = spvPackHalf2x16(float2(coc14, 0.0f)) | (spvPackHalf2x16(float2(coc15, 0.0f)) << 16u);
        float coc16 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc16 = abs(coc16);
        weight_accum_next = lerp(w, max(w, abs_coc16), lerp_gain);
        coc_min_next = min(min(min(min(min(min(min(min(min(coc_min, abs_coc0), abs_coc2), abs_coc4), abs_coc6), abs_coc8), abs_coc10), abs_coc12), abs_coc14), abs_coc16);
        coc_max_next = max(max(max(max(max(max(max(max(max(coc_max, abs_coc0), abs_coc2), abs_coc4), abs_coc6), abs_coc8), abs_coc10), abs_coc12), abs_coc14), abs_coc16);
        ring_samples[gl_LocalInvocationIndex][8u] = spvPackHalf2x16(float2(coc16, 0.0f));
        for (uint i = 0u; i < 12u; )
        {
            ring_bits[i] = 0u;
            i++;
            continue;
        }
        // 17 个 tap 分类到 12 槽位域。
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
        float4 slot_color;
        slot_weight = depth_weight;
        slot_depth = span_accum;
        slot_color = 0.0f.xxxx;
        float slot_depth_next;
        float slot_weight_next;
        float4 slot_color_next;
        for (int slot = 0; slot < 12; slot_weight = slot_weight_next, slot_depth = slot_depth_next, slot_color = slot_color_next, slot++)
        {
            float slot_frac = ((float(slot) + tap_sub) * 0.16666667163372039794921875f) - 1.0f;
            float abs_frac = abs(slot_frac);
            slot_depth_next = slot_depth + abs_frac;
            if (ring_bits[slot] == 0u)
            {
                slot_weight_next = slot_weight;
                slot_color_next = slot_color;
                continue;
            }
            bool positive = slot_frac >= 0.0f;
            float lo_coc;
            float hi_coc;
            uint tap_idx;
            if (positive)
            {
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
            float coc_at_resolved = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * resolved), 0.0f).x, neg_coc, coc_limit);
            float occlusion = log2(max(0.0f, abs(lo_coc) * 0.125f)) * smoothstep(4.0f, 0.0f, coc_max - coc_min);
            float occ_lane = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 1u);
            float occ_min = min(occlusion, occ_lane);
            float occ_wide = WaveReadLaneAt(occ_min, WaveGetLaneIndex() ^ 8u);
            if (lo_coc > coc_limit)
            {
                slot_weight_next = slot_weight;
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
            float ring_color_w;
            float ring_weight;
            if (close_to_edge)
            {
                float abs_resolved = abs(coc_at_resolved);
                if (abs_resolved < 8.0f)
                {
                    slot_weight_next = slot_weight;
                    slot_color_next = slot_color;
                    continue;
                }
                ring_color_w = slot_weight;
                ring_weight = slot_span * smoothstep(8.0f, 11.0f, abs_resolved);
            }
            else
            {
                float abs_hi = abs(hi_coc);
                if (abs_hi < 8.0f)
                {
                    slot_weight_next = slot_weight;
                    slot_color_next = slot_color;
                    continue;
                }
                float w2 = slot_span * smoothstep(8.0f, 11.0f, abs_hi);
                float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.20000000298023223876953125f * coc_limit, 0.0f, lo_coc);
                ring_color_w = slot_weight + (w2 * depth_gate);
                ring_weight = w2 * (1.0f - depth_gate);
            }
            slot_weight_next = ring_color_w;
            slot_color_next = slot_color + (color_tex.SampleLevel(sampler_nnc, uv + (step_px * (close_to_edge ? resolved : lo_edge)), min(occ_min, occ_wide)) * ring_weight);
        }
        ring_color_accum_next = ring_color_accum + slot_color;
    }
    float scatter = depth_weight * smoothstep(0.00999999977648258209228515625f * ring_color_accum.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * ring_color_accum.w), depth_weight);
    float4 final_color;
    [branch]
    if (int(scatter > 0.0f) != 0)
    {
        float scatter_neg_coc = -coc_limit;
        float4 bg_accum;
        bg_accum = 0.0f.xxxx;
        float4 bg_accum_next;
        float max_bg_coc_next;
        float max_bg_coc = scatter_neg_coc;
        uint s = 0u;
        for (; s < 16u; bg_accum = bg_accum_next, max_bg_coc = max_bg_coc_next, s++)
        {
            float2 rnd = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(s)) + 0.5f.xx) + dither.xy);
            float r = sqrt(rnd.x);
            float phi = rnd.y * 6.283185482025146484375f;
            float2 bg_uv = uv + (((float2(cos(phi) * r, sin(phi) * r) * 1.0f) / float2(color_tex_dims)) * weight_accum);
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
        final_color = ring_color_accum + ((bg_accum / max(1.0f, bg_accum.w).xxxx) * scatter);
    }
    else
    {
        final_color = ring_color_accum;
    }
    img_output[pixel] = max(0.0f.xxxx, final_color / span_accum.xxxx);
}

[numthreads(8, 8, 1)]
void cs_halfres(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
#if defined(TGV_cs_halfres_all_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _dof_halfres.cs.hlsl_cs_halfres_all_6f1dd5dd8a66e104.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// 6f1d 变体的 cs_halfres_all 段:与 cs_halfres 相同的 2 环 x 17 tap、8 槽结构,
//   但 occlude_gate=false(tile offsets[5])——不启用 8->11 近景斜坡。
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
};

ByteAddressBuffer tile_offsets_buffer : register(t20, space0);
ByteAddressBuffer tile_coords_buffer : register(t21, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);

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
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(20) + gl_WorkGroupID.x) * 8 + 0));
    float2 uv = (float2(int2(pixel)) + 0.5f.xx) / float2(img_output_dims);
    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, uv, 0.0f).x);
    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint2 hash_pos = uint2(7u, 7u) + pixel;
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
    float4 ring_color_accum;
    float weight_accum;
    float span_accum;
    float depth_weight;
    ring_color_accum = 0.0f.xxxx;
    weight_accum = 0.0f;
    span_accum = 0.0f;
    depth_weight = 0.0f;
    float4 ring_color_accum_next;
    float weight_accum_next;
    float coc_min_next;
    float coc_max_next;
    uint ring_bits[8];
    float slot_depth;
    float slot_weight;
    uint ring = 0u;
    float coc_min = 10000000000.0f;
    float coc_max = -10000000000.0f;
    [loop]
    for (; ring < 2u; ring_color_accum = ring_color_accum_next, weight_accum = weight_accum_next, span_accum = slot_depth, depth_weight = slot_weight, ring++, coc_min = coc_min_next, coc_max = coc_max_next)
    {
        float angle = (float(ring) + tap_seed) * 1.57079637050628662109375f;
        float2 dir = float2(cos(angle), sin(angle));
        float2 bokeh_dir;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 ctv = mul(frame_constants_view_constants[0u].clip_to_view, float4((uv - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
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
        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-8.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc0 = abs(coc0);
        float lerp_gain = dither.x * 0.20000000298023223876953125f;
        float w = lerp(weight_accum, max(weight_accum, abs_coc0), lerp_gain);
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-7.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(coc0, 0.0f)) | (spvPackHalf2x16(float2(coc1, 0.0f)) << 16u);
        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-6.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc2 = abs(coc2);
        w = lerp(w, max(w, abs_coc2), lerp_gain);
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-5.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(coc2, 0.0f)) | (spvPackHalf2x16(float2(coc3, 0.0f)) << 16u);
        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-4.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc4 = abs(coc4);
        w = lerp(w, max(w, abs_coc4), lerp_gain);
        float coc5 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-3.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc5)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(coc4, 0.0f)) | (spvPackHalf2x16(float2(coc5, 0.0f)) << 16u);
        float coc6 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-2.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc6 = abs(coc6);
        w = lerp(w, max(w, abs_coc6), lerp_gain);
        float coc7 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-1.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc7)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(coc6, 0.0f)) | (spvPackHalf2x16(float2(coc7, 0.0f)) << 16u);
        float coc8 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (dither_y * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc8 = abs(coc8);
        w = lerp(w, max(w, abs_coc8), lerp_gain);
        float coc9 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc9)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(coc8, 0.0f)) | (spvPackHalf2x16(float2(coc9, 0.0f)) << 16u);
        float coc10 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc10 = abs(coc10);
        w = lerp(w, max(w, abs_coc10), lerp_gain);
        float coc11 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc11)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][5u] = spvPackHalf2x16(float2(coc10, 0.0f)) | (spvPackHalf2x16(float2(coc11, 0.0f)) << 16u);
        float coc12 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc12 = abs(coc12);
        w = lerp(w, max(w, abs_coc12), lerp_gain);
        float coc13 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc13)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][6u] = spvPackHalf2x16(float2(coc12, 0.0f)) | (spvPackHalf2x16(float2(coc13, 0.0f)) << 16u);
        float coc14 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc14 = abs(coc14);
        w = lerp(w, max(w, abs_coc14), lerp_gain);
        float coc15 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc15)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][7u] = spvPackHalf2x16(float2(coc14, 0.0f)) | (spvPackHalf2x16(float2(coc15, 0.0f)) << 16u);
        float coc16 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc16 = abs(coc16);
        weight_accum_next = lerp(w, max(w, abs_coc16), lerp_gain);
        coc_min_next = min(min(min(min(min(min(min(min(min(coc_min, abs_coc0), abs_coc2), abs_coc4), abs_coc6), abs_coc8), abs_coc10), abs_coc12), abs_coc14), abs_coc16);
        coc_max_next = max(max(max(max(max(max(max(max(max(coc_max, abs_coc0), abs_coc2), abs_coc4), abs_coc6), abs_coc8), abs_coc10), abs_coc12), abs_coc14), abs_coc16);
        ring_samples[gl_LocalInvocationIndex][8u] = spvPackHalf2x16(float2(coc16, 0.0f));
        for (uint i = 0u; i < 8u; )
        {
            ring_bits[i] = 0u;
            i++;
            continue;
        }
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
        float4 slot_color;
        slot_weight = depth_weight;
        slot_depth = span_accum;
        slot_color = 0.0f.xxxx;
        float slot_depth_next;
        float slot_weight_next;
        float4 slot_color_next;
        for (int slot = 0; slot < 8; slot_weight = slot_weight_next, slot_depth = slot_depth_next, slot_color = slot_color_next, slot++)
        {
            float slot_frac = ((float(slot) + tap_sub) * 0.25f) - 1.0f;
            float abs_frac = abs(slot_frac);
            slot_depth_next = slot_depth + abs_frac;
            if (ring_bits[slot] == 0u)
            {
                slot_weight_next = slot_weight;
                slot_color_next = slot_color;
                continue;
            }
            bool positive = slot_frac >= 0.0f;
            float lo_coc;
            float hi_coc;
            uint tap_idx;
            if (positive)
            {
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
            float coc_at_resolved = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * resolved), 0.0f).x, neg_coc, coc_limit);
            float occlusion = log2(max(0.0f, abs(lo_coc) * 0.125f)) * smoothstep(4.0f, 0.0f, coc_max - coc_min);
            float occ_lane = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 1u);
            float occ_min = min(occlusion, occ_lane);
            float occ_wide = WaveReadLaneAt(occ_min, WaveGetLaneIndex() ^ 8u);
            if (lo_coc > coc_limit)
            {
                slot_weight_next = slot_weight;
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
            float ring_color_w;
            float ring_weight;
            if (close_to_edge)
            {
                ring_color_w = slot_weight;
                ring_weight = slot_span;
            }
            else
            {
                float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.20000000298023223876953125f * coc_limit, 0.0f, lo_coc);
                ring_color_w = slot_weight + (slot_span * depth_gate);
                ring_weight = slot_span * (1.0f - depth_gate);
            }
            slot_weight_next = ring_color_w;
            slot_color_next = slot_color + (color_tex.SampleLevel(sampler_nnc, uv + (step_px * (close_to_edge ? resolved : lo_edge)), min(occ_min, occ_wide)) * ring_weight);
        }
        ring_color_accum_next = ring_color_accum + slot_color;
    }
    float scatter = depth_weight * smoothstep(0.00999999977648258209228515625f * ring_color_accum.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * ring_color_accum.w), depth_weight);
    float4 final_color;
    [branch]
    if (int(scatter > 0.0f) != 0)
    {
        float scatter_neg_coc = -coc_limit;
        float4 bg_accum;
        bg_accum = 0.0f.xxxx;
        float4 bg_accum_next;
        float max_bg_coc_next;
        float max_bg_coc = scatter_neg_coc;
        uint s = 0u;
        for (; s < 16u; bg_accum = bg_accum_next, max_bg_coc = max_bg_coc_next, s++)
        {
            float2 rnd = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(s)) + 0.5f.xx) + dither.xy);
            float r = sqrt(rnd.x);
            float phi = rnd.y * 6.283185482025146484375f;
            float2 bg_uv = uv + (((float2(cos(phi) * r, sin(phi) * r) * 1.0f) / float2(color_tex_dims)) * weight_accum);
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
        final_color = ring_color_accum + ((bg_accum / max(1.0f, bg_accum.w).xxxx) * scatter);
    }
    else
    {
        final_color = ring_color_accum;
    }
    img_output[pixel] = max(0.0f.xxxx, final_color / span_accum.xxxx);
}

[numthreads(8, 8, 1)]
void cs_halfres_all(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
#if defined(TGV_cs_halfres_all_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _dof_halfres.cs.hlsl_cs_halfres_all_72a2a95d73e37edc.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// 72a2 变体的 cs_halfres_all 段:4 环 x 17 tap、12 槽(除数 1/6)、occlude_gate=false
//   (tile offsets[5])——不启用 8->11 近景斜坡;抖动方案用 dither.z 决定 tap_sub。
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
};

ByteAddressBuffer tile_offsets_buffer : register(t20, space0);
ByteAddressBuffer tile_coords_buffer : register(t21, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);

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
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(20) + gl_WorkGroupID.x) * 8 + 0));
    float2 uv = (float2(int2(pixel)) + 0.5f.xx) / float2(img_output_dims);
    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, uv, 0.0f).x);
    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    float dither_z = dither.z;
    float dither_y = dither.y;
    float coc_step = coc_limit * 0.125f;
    float tap_seed = dither.x;
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
    float4 ring_color_accum;
    float weight_accum;
    float span_accum;
    float depth_weight;
    ring_color_accum = 0.0f.xxxx;
    weight_accum = 0.0f;
    span_accum = 0.0f;
    depth_weight = 0.0f;
    float4 ring_color_accum_next;
    float weight_accum_next;
    float coc_min_next;
    float coc_max_next;
    uint ring_bits[12];
    float slot_depth;
    float slot_weight;
    uint ring = 0u;
    float coc_min = 10000000000.0f;
    float coc_max = -10000000000.0f;
    [loop]
    for (; ring < 4u; ring_color_accum = ring_color_accum_next, weight_accum = weight_accum_next, span_accum = slot_depth, depth_weight = slot_weight, ring++, coc_min = coc_min_next, coc_max = coc_max_next)
    {
        float angle = (float(ring) + tap_seed) * 0.785398185253143310546875f;
        float2 dir = float2(cos(angle), sin(angle));
        float2 bokeh_dir;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 ctv = mul(frame_constants_view_constants[0u].clip_to_view, float4((uv - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
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
        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-8.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc0 = abs(coc0);
        float lerp_gain = tap_seed * 0.20000000298023223876953125f;
        float w = lerp(weight_accum, max(weight_accum, abs_coc0), lerp_gain);
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-7.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(coc0, 0.0f)) | (spvPackHalf2x16(float2(coc1, 0.0f)) << 16u);
        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-6.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc2 = abs(coc2);
        w = lerp(w, max(w, abs_coc2), lerp_gain);
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-5.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(coc2, 0.0f)) | (spvPackHalf2x16(float2(coc3, 0.0f)) << 16u);
        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-4.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc4 = abs(coc4);
        w = lerp(w, max(w, abs_coc4), lerp_gain);
        float coc5 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-3.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc5)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(coc4, 0.0f)) | (spvPackHalf2x16(float2(coc5, 0.0f)) << 16u);
        float coc6 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-2.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc6 = abs(coc6);
        w = lerp(w, max(w, abs_coc6), lerp_gain);
        float coc7 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (((-1.0f) + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc7)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(coc6, 0.0f)) | (spvPackHalf2x16(float2(coc7, 0.0f)) << 16u);
        float coc8 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (dither_y * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc8 = abs(coc8);
        w = lerp(w, max(w, abs_coc8), lerp_gain);
        float coc9 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc9)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(coc8, 0.0f)) | (spvPackHalf2x16(float2(coc9, 0.0f)) << 16u);
        float coc10 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc10 = abs(coc10);
        w = lerp(w, max(w, abs_coc10), lerp_gain);
        float coc11 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc11)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][5u] = spvPackHalf2x16(float2(coc10, 0.0f)) | (spvPackHalf2x16(float2(coc11, 0.0f)) << 16u);
        float coc12 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc12 = abs(coc12);
        w = lerp(w, max(w, abs_coc12), lerp_gain);
        float coc13 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc13)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][6u] = spvPackHalf2x16(float2(coc12, 0.0f)) | (spvPackHalf2x16(float2(coc13, 0.0f)) << 16u);
        float coc14 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc14 = abs(coc14);
        w = lerp(w, max(w, abs_coc14), lerp_gain);
        float coc15 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc15)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][7u] = spvPackHalf2x16(float2(coc14, 0.0f)) | (spvPackHalf2x16(float2(coc15, 0.0f)) << 16u);
        float coc16 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc16 = abs(coc16);
        weight_accum_next = lerp(w, max(w, abs_coc16), lerp_gain);
        coc_min_next = min(min(min(min(min(min(min(min(min(coc_min, abs_coc0), abs_coc2), abs_coc4), abs_coc6), abs_coc8), abs_coc10), abs_coc12), abs_coc14), abs_coc16);
        coc_max_next = max(max(max(max(max(max(max(max(max(coc_max, abs_coc0), abs_coc2), abs_coc4), abs_coc6), abs_coc8), abs_coc10), abs_coc12), abs_coc14), abs_coc16);
        ring_samples[gl_LocalInvocationIndex][8u] = spvPackHalf2x16(float2(coc16, 0.0f));
        for (uint i = 0u; i < 12u; )
        {
            ring_bits[i] = 0u;
            i++;
            continue;
        }
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
        float4 slot_color;
        slot_weight = depth_weight;
        slot_depth = span_accum;
        slot_color = 0.0f.xxxx;
        float slot_depth_next;
        float slot_weight_next;
        float4 slot_color_next;
        for (int slot = 0; slot < 12; slot_weight = slot_weight_next, slot_depth = slot_depth_next, slot_color = slot_color_next, slot++)
        {
            float slot_frac = ((float(slot) + tap_sub) * 0.16666667163372039794921875f) - 1.0f;
            float abs_frac = abs(slot_frac);
            slot_depth_next = slot_depth + abs_frac;
            if (ring_bits[slot] == 0u)
            {
                slot_weight_next = slot_weight;
                slot_color_next = slot_color;
                continue;
            }
            bool positive = slot_frac >= 0.0f;
            float lo_coc;
            float hi_coc;
            uint tap_idx;
            if (positive)
            {
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
            float coc_at_resolved = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * resolved), 0.0f).x, neg_coc, coc_limit);
            float occlusion = log2(max(0.0f, abs(lo_coc) * 0.125f)) * smoothstep(4.0f, 0.0f, coc_max - coc_min);
            float occ_lane = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 1u);
            float occ_min = min(occlusion, occ_lane);
            float occ_wide = WaveReadLaneAt(occ_min, WaveGetLaneIndex() ^ 8u);
            if (lo_coc > coc_limit)
            {
                slot_weight_next = slot_weight;
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
            float ring_color_w;
            float ring_weight;
            if (close_to_edge)
            {
                ring_color_w = slot_weight;
                ring_weight = slot_span;
            }
            else
            {
                float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.20000000298023223876953125f * coc_limit, 0.0f, lo_coc);
                ring_color_w = slot_weight + (slot_span * depth_gate);
                ring_weight = slot_span * (1.0f - depth_gate);
            }
            slot_weight_next = ring_color_w;
            slot_color_next = slot_color + (color_tex.SampleLevel(sampler_nnc, uv + (step_px * (close_to_edge ? resolved : lo_edge)), min(occ_min, occ_wide)) * ring_weight);
        }
        ring_color_accum_next = ring_color_accum + slot_color;
    }
    float scatter = depth_weight * smoothstep(0.00999999977648258209228515625f * ring_color_accum.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * ring_color_accum.w), depth_weight);
    float4 final_color;
    [branch]
    if (int(scatter > 0.0f) != 0)
    {
        float scatter_neg_coc = -coc_limit;
        float4 bg_accum;
        bg_accum = 0.0f.xxxx;
        float4 bg_accum_next;
        float max_bg_coc_next;
        float max_bg_coc = scatter_neg_coc;
        uint s = 0u;
        for (; s < 16u; bg_accum = bg_accum_next, max_bg_coc = max_bg_coc_next, s++)
        {
            float2 rnd = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(s)) + 0.5f.xx) + dither.xy);
            float r = sqrt(rnd.x);
            float phi = rnd.y * 6.283185482025146484375f;
            float2 bg_uv = uv + (((float2(cos(phi) * r, sin(phi) * r) * 1.0f) / float2(color_tex_dims)) * weight_accum);
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
        final_color = ring_color_accum + ((bg_accum / max(1.0f, bg_accum.w).xxxx) * scatter);
    }
    else
    {
        final_color = ring_color_accum;
    }
    img_output[pixel] = max(0.0f.xxxx, final_color / span_accum.xxxx);
}

[numthreads(8, 8, 1)]
void cs_halfres_all(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
