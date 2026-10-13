// ==SKELETON==
// SOURCE PATH : shaders/tgsr.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (4 bins):
//   _tgsr.cs.hlsl_cs_main_{3ee6a953408e4c4c,7a0af18450cdbcd1,b303f36f0dbdd5f6,f7efabb81dfe256b}.bin
//     pass/stage   : _tgsr (taa_tgsr) / cs
//     entry        : cs:cs_main
//     variant      : 3ee6a953408e4c4c / 7a0af18450cdbcd1 (LocalSize 8,8,1) and
//                    b303f36f0dbdd5f6 / f7efabb81dfe256b (LocalSize 8,4,1)
//     index_pass   : W52 (Inference)
//
// ==RECONSTRUCTED== shaders/tgsr.cs.hlsl
// Evidence: _tgsr.cs.hlsl_cs_main_3ee6a953408e4c4c.bin (SPIR-V, offset 23, bound 706).
//   bindings: sampler_lnc s0, sampler_nnc s2, $Globals b13 (13 members: the
//   reprojected_history/color/blurred_luminance/reprojection/aux/output dims +
//   bypass_neighborhood_clamping, sharpen_amount, history_valid),
//   reprojected_history_tex_lum t14, reprojected_history_tex_chroma t15,
//   color_tex t16, blurred_luminance_tex t17, reprojection_tex t19, aux_tex t20,
//   history_output_tex u21, output_tex u22, aux_output_tex u23, frame_constants
//   set1/b0.  TGSR (FidelityFX Temporal Super Resolution): YCoCg color
//   neighborhood gather with jitter-aware Gaussian weights, reprojected-history
//   luminance blur, wave-lane variance estimation (WaveReadLaneAt ^2 / ^16),
//   3x3 clamp + static-reprojection stats (GatherAlpha/GatherBlue), a fused
//   history-weight heuristic, frexp-quantized history/color output (curve.hlsl),
//   tile-luminance sharpening, and an aux (history-weight / blend-scale) output.
//   Status: O (structure from disassembly; constants _156=1.0 / _162=0.0
//   Inference from the inlined gather/clamp).

#include "inc/common.hlsl"
#include "inc/uv.hlsl"
#include "inc/image.hlsl"
#include "inc/curve.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/unjitter_taa.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/morton.hlsl"
#include "tgsr_common.inc.hlsl"
#include "inc/color/ycbcr.hlsl"
#include "inc/history_valid.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 reprojected_history_tex_lum_dims;
    uint2 reprojected_history_tex_chroma_dims;
    uint2 color_tex_dims;
    uint2 blurred_luminance_tex_dims;
    uint2 history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 aux_tex_dims;
    uint2 history_output_tex_dims;
    uint2 output_tex_dims;
    uint2 aux_output_tex_dims;
    uint bypass_neighborhood_clamping;
    float sharpen_amount;
    uint history_valid;
};

Texture2D<float4> reprojected_history_tex_lum : register(t14, space0);
Texture2D<float4> reprojected_history_tex_chroma : register(t15, space0);
Texture2D<float4> color_tex : register(t16, space0);
Texture2D<float4> blurred_luminance_tex : register(t17, space0);
Texture2D<float4> reprojection_tex : register(t19, space0);
Texture2D<float4> aux_tex : register(t20, space0);
[[vk::image_format("r11g11b10f")]]
RWTexture2D<float3> history_output_tex : register(u21, space0);
RWTexture2D<float3> output_tex : register(u22, space0);
RWTexture2D<float2> aux_output_tex : register(u23, space0);

groupshared float tile_luminance[8][8];

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_LocalInvocationID : SV_GroupThreadID,
             uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    float2 output_dims = float2(output_tex_dims);
    float2 pixel = float2(int2(gl_GlobalInvocationID.xy)) + 0.5f;
    float2 pixel_uv = pixel / output_dims;

    bool bypass = (bypass_neighborhood_clamping != 0u);
    float clamp_gain = float(bypass ? 16 : 4);

    float4 reproj = reprojection_tex.SampleLevel(sampler_nnc, pixel_uv, 0.0f);
    float2 reproj_motion = reproj.xy;
    bool valid_history = (history_valid != 0u) && (frame_index != 0u);

    float2 aux = valid_history ? aux_tex.SampleLevel(sampler_lnc, pixel_uv + reproj_motion, 0.0f).xy : 0.0f.xx;
    float aux_blend = aux.y * clamp_gain;

    // Reprojected history (YCoCg) for the current pixel.
    float3 reprojected_ycocg = float3(reprojected_history_tex_lum.Load(int3(gl_GlobalInvocationID.xy, 0u)).x,
                                      reprojected_history_tex_chroma.Load(int3(gl_GlobalInvocationID.xy, 0u)).xy);

    // Current-frame YCoCg 3x3 gather with jitter-aware Gaussian weights.
    float2 color_scale = float2(int2(color_tex_dims)) / output_dims;
    int2 color_pixel = int2(pixel * color_scale);
    float2 sample_uv = ((float2(color_pixel) + 0.5f)
        + (view_constants[0].sample_offset_pixels * float2(1.0f, -1.0f))) / color_scale;
    float k1 = (-10.0f) * color_scale.x * min(4.0f * color_scale.x, 1.0f);
    float k2 = (-1.0f) * (color_scale.x * color_scale.x);

    float wg_sum = 0.0f;
    float3 color2_sum = 0.0f.xxx;
    float3 color_sum = 0.0f.xxx;
    float wc_sum = 0.0f;
    float4 color4_sum = 0.0f.xxxx;
    for (int gy = -1; gy <= 1; ++gy)
    {
        for (int gx = -1; gx <= 1; ++gx)
        {
            float4 sample = float4(mul(YCOCG_FWD, color_tex.Load(int3(uint2(color_pixel + int2(gx, gy)), 0u)).xyz), 1.0f);
            float2 off = (sample_uv + (float2(float(gx), float(gy)) / color_scale)) - pixel;
            float d2 = dot(off, off);
            float wg = exp2(d2 * k2);
            float wc = exp2(d2 * k1);
            float3 s = sample.xyz;
            wg_sum += wg;
            color2_sum += ((s * s) * wg);
            color_sum += (s * wg);
            wc_sum += wc;
            color4_sum += (sample * wc);
        }
    }
    float3 mean = color_sum / wg_sum.xxx;
    float3 color_mean = color4_sum.xyz;
    float weight_sum = wg_sum;

    // Reprojected-history luminance blur (3x3 gaussian).
    float lum_blur = 0.0f;
    float lum_weight = 0.0f;
    for (int by = -1; by <= 1; ++by)
    {
        for (int bx = -1; bx <= 1; ++bx)
        {
            float w = exp(float((bx * bx) + (by * by)) * (-0.75f));
            lum_blur += (reprojected_history_tex_lum.SampleLevel(sampler_lnc,
                pixel_uv + ((float2(float(bx), float(by)) * 2.0f) / float2(color_tex_dims)),
                0.0f).x * w);
            lum_weight += w;
        }
    }
    float lum_var = clamp(sqrt(lum_blur / lum_weight), 0.0f, 1.0f);
    float lum_var2 = lum_var * lum_var;

    // Current-frame blurred luminance and its wave-lane variance.
    float4 blur4 = blurred_luminance_tex.SampleLevel(sampler_lnc, pixel_uv, 0.0f);
    float blur_lum = blur4.x;
    float blur_lum2 = blur_lum * blur_lum;
    float lane_var2, lane_var;
    if (WaveActiveMax(WaveGetLaneIndex()) >= 31u)
    {
        lane_var2 = lerp(blur_lum2, WaveReadLaneAt(blur_lum2, WaveGetLaneIndex() ^ 16u), 0.5f);
        lane_var = lerp(lum_var2, WaveReadLaneAt(lum_var2, WaveGetLaneIndex() ^ 16u), 0.5f);
    }
    else
    {
        lane_var2 = blur_lum2;
        lane_var = lum_var2;
    }
    float lane_var_x = WaveReadLaneAt(lane_var, WaveGetLaneIndex() ^ 2u);
    lane_var = lerp(lane_var, lane_var_x, 0.5f);
    float lane_var2_x = WaveReadLaneAt(lane_var2, WaveGetLaneIndex() ^ 2u);
    lane_var2 = lerp(lane_var2, lane_var2_x, 0.5f);

    // Static clamp range (variance scaled).
    float3 stddev = sqrt(max(0.0f.xxx, (color2_sum / wg_sum.xxx) - (mean * mean)));
    float2 reproj_dims_f = 0.5f / float2(reprojection_tex_dims);
    float3 hi = (mean + (stddev * 2.0f)) + 0.001f;
    float3 lo = (mean - (stddev * 2.0f)) - 0.001f;

    // 3x3 static-clamp statistics over the reprojected history.
    float lum2_sum = 0.0f;
    float lum_sum = 0.0f;
    float3 below_sum = 0.0f.xxx;
    float3 above_sum = 0.0f.xxx;
    float count = 0.0f;
    for (int cy = -1; cy <= 1; ++cy)
    {
        for (int cx = -1; cx <= 1; ++cx)
        {
            uint2 t = gl_GlobalInvocationID.xy + uint2(cx, cy);
            float tl = reprojected_history_tex_lum.Load(int3(t, 0u)).x;
            float3 s = float3(tl, 0.0f, reprojected_history_tex_chroma.Load(int3(t, 0u)).y);
            lum2_sum += (tl * tl);
            lum_sum += tl;
            below_sum += (float3(s.x < lo.x, s.y < lo.y, s.z < lo.z) * 1.0f);
            above_sum += (float3(s.x > hi.x, s.y > hi.y, s.z > hi.z) * 1.0f);
            count += 1.0f;
        }
    }
    float3 counts = count.xxx;
    float mean_lum = lum_sum / count;
    float stddev_lum = sqrt(max(0.0f, (lum2_sum / count) - (mean_lum * mean_lum)));

    float4 gather_alpha = reprojection_tex.GatherAlpha(sampler_lnc, pixel_uv);
    float static_factor = clamp((20.0f * max(0.0f, stddev_lum)) / max(1e-9f, mean_lum), 0.0f, 1.0f);
    float2 out_of_range = (above_sum / counts).xz + (below_sum / counts).xz;

    float lane_diff = abs(lane_var - lane_var2);
    float lane_ratio = (2.0f * lane_diff) / max(1e-5f, lane_var2 + lane_var);

    // Fraction of the 3x3 reprojection neighborhood with zero motion.
    float static_count = 0.0f;
    float static_total = 0.0f;
    for (int sy = -1; sy <= 1; ++sy)
    {
        for (int sx = -1; sx <= 1; ++sx)
        {
            static_count += float(reprojection_tex.SampleLevel(sampler_nnc,
                pixel_uv - (reproj_dims_f * float2(float(sx), float(sy))), 0.0f).z == 0.0f);
            static_total += 1.0f;
        }
    }

    float2 motion_px = reproj_motion * output_dims;
    float2 motion_abs = abs(motion_px);
    float4 gather_blue = reprojection_tex.GatherBlue(sampler_lnc, pixel_uv);
    float depth_static = (((3.0f * clamp(1.0f - ((2.5f * stddev_lum) / mean_lum), 0.0f, 1.0f)) * lane_diff)
        / (0.5f * max(1e-5f, lane_var2 + lane_var))) * float(min(min(gather_blue.x, gather_blue.y), min(gather_blue.z, gather_blue.w)) == 1.0f);

    float history_weight =
        (((aux.x * 2.0f) - 0.5f) + clamp(1.0f - static_factor, 0.0f, 1.0f))
        + max(out_of_range.x, out_of_range.y)
        + ((lane_ratio * lane_ratio) * max(0.0f, 1.0f - ((((-3.0f) * abs(mean.x - lane_var2))
            + (2.0f * abs((color4_sum.x / wc_sum) - mean.x))) / lane_var2)))
        + (sqrt(static_count / static_total) * clamp(max(motion_abs.x, motion_abs.y), 0.0f, 1.0f))
        + ((depth_static * depth_static) * depth_static);

    float blend_scale;
    float clamp_blend;
    if (!bypass)
    {
        float hi2;
        float lo2;
        if (clamp(1.0f - min(min(gather_alpha.x, gather_alpha.y), min(gather_alpha.z, gather_alpha.w)), 0.0f, 1.0f) < 0.75f)
        {
            hi2 = history_weight + 1.0f;
            lo2 = min(aux_blend, 0.5f);
        }
        else
        {
            hi2 = history_weight;
            lo2 = aux_blend;
        }
        float lo2_scaled = lo2 * static_factor;
        float2 m2 = abs(motion_px * 0.03125f);
        blend_scale = min(lo2_scaled, lerp(lo2_scaled, 1.0f, clamp(max(m2.x, m2.y), 0.0f, 1.0f)));
        clamp_blend = hi2;
    }
    else
    {
        blend_scale = aux_blend;
        clamp_blend = history_weight;
    }

    float blend = min(clamp_blend, 1.0f);
    float3 clamp_range = stddev * exp2((7.0f * clamp(1.0f - clamp_blend, 0.0f, 1.0f)) - 0.5f);
    float3 clamped;
    if (!bypass)
    {
        clamped = clamp(reprojected_ycocg, mean - clamp_range, mean + clamp_range);
    }
    else
    {
        clamped = reprojected_ycocg;
    }

    float total_weight = wc_sum + min(4.0f, blend_scale);
    float3 blended = lerp(color_mean / wg_sum.xxx, clamped, (min(4.0f, blend_scale) / max(1e-5f, total_weight)).xxx);
    float blended_lum = blended.x;

    // Convert to RGB (bypass clamp uses the un-clamped color mean).
    float3 rgb;
    if (!valid_history)
    {
        rgb = max(0.0f.xxx, mul(YCOCG_INV, color_mean / max(1e-5f, wc_sum).xxx));
    }
    else
    {
        rgb = max(0.0f.xxx, mul(YCOCG_INV, blended));
    }
    history_output_tex[gl_GlobalInvocationID.xy] = curve(rgb);

    // Sharpen via tile-luminance laplacian.
    tile_luminance[gl_LocalInvocationID.x][gl_LocalInvocationID.y] = blended_lum;
    GroupMemoryBarrierWithGroupSync();
    uint2 rx = gl_LocalInvocationID.xy + uint2(1u, 0u);
    uint2 lx = gl_LocalInvocationID.xy - uint2(1u, 0u);
    uint4 xid = uint4(rx.x, rx.y, lx.x, lx.y);
    float edge_sum;
    float edge_weight;
    if (all(xid < uint4(8u, 8u, 8u, 8u)))
    {
        float w = max(0.0f, 1.0f - (6.0f * (abs(blended_lum - tile_luminance[rx.x][rx.y])
            + abs(blended_lum - tile_luminance[lx.x][lx.y]))));
        float sw = min(w, (sharpen_amount * w) * 1.25f);
        edge_sum = sw * (tile_luminance[rx.x][rx.y] + tile_luminance[lx.x][lx.y]);
        edge_weight = sw * 2.0f;
    }
    else
    {
        edge_sum = 0.0f;
        edge_weight = 0.0f;
    }
    uint2 dy = gl_LocalInvocationID.xy + uint2(0u, 1u);
    uint2 uy = gl_LocalInvocationID.xy - uint2(0u, 1u);
    uint4 yid = uint4(dy.x, dy.y, uy.x, uy.y);
    if (all(yid < uint4(8u, 8u, 8u, 8u)))
    {
        float w = max(0.0f, 1.0f - (6.0f * (abs(blended_lum - tile_luminance[dy.x][dy.y])
            + abs(blended_lum - tile_luminance[uy.x][uy.y]))));
        float sw = min(w, (sharpen_amount * w) * 1.25f);
        edge_sum += (tile_luminance[dy.x][dy.y] * sw) + (tile_luminance[uy.x][uy.y] * sw);
        edge_weight += (sw * 2.0f);
    }

    float3 sharpened = rgb * max(0.0f, max(0.0f, (blended_lum * (edge_weight + 1.0f)) - edge_sum)
        / max(1e-5f, blended_lum));
    output_tex[gl_GlobalInvocationID.xy] = curve(sharpened);

    aux_output_tex[gl_GlobalInvocationID.xy] = float2(clamp(blend + (2.0f * clamp(reproj.w * 1000.0f, 0.0f, 1.0f)), 0.0f, 2.0f) * 0.5f, total_weight / clamp_gain);
}
