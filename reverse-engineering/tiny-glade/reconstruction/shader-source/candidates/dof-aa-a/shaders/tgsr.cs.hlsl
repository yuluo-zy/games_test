// ==SKELETON==
// SOURCE PATH : shaders/tgsr.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (4 bins):
//   _tgsr.cs.hlsl_cs_main_3ee6a953408e4c4c.bin / _tgsr.cs.hlsl_cs_main_7a0af18450cdbcd1.bin (LocalSize 8,8,1)
//   _tgsr.cs.hlsl_cs_main_b303f36f0dbdd5f6.bin / _tgsr.cs.hlsl_cs_main_f7efabb81dfe256b.bin (LocalSize 8,4,1)
//     pass/stage   : _tgsr (taa_tgsr) / cs | entry cs:cs_main
//     names        : sampler_lnc, sampler_nnc, ViewConstants, frame_constants, $Globals, reprojected_history_tex_lum, reprojected_history_tex_chroma, color_tex, blurred_luminance_tex, reprojection_tex, aux_tex, history_output_tex
// ==RECONSTRUCTED== shaders/tgsr.cs.hlsl | dof-aa-a | status: O (structure from SPIR-V; primary variant 3ee6 = with tile-luminance sharpen, LocalSize 8x8)
//   TGSR (temporal Gaussian super-resolution) accumulation: reprojected-history + current-frame
//   statistics drive an adaptive blend/clamp in YCoCg, with a blurred-luminance motion veto,
//   curve-quantized history output and tile-luminance sharpen. aux_output holds per-pixel
//   (transparency, blend) state.

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
[[vk::ext_capability(56)]] RWTexture2D<float3> history_output_tex : register(u21, space0);
[[vk::ext_capability(56)]] RWTexture2D<float3> output_tex : register(u22, space0);
[[vk::ext_capability(56)]] RWTexture2D<unorm float2> aux_output_tex : register(u23, space0);

groupshared float tile_luminance[8][8];

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint3 local_id : SV_GroupThreadID)
{
    float2 out_dims = float2(output_tex_dims);
    float2 uv = (float2(int2(dispatch_id.xy)) + 0.5f) / out_dims;
    bool bypass = bypass_neighborhood_clamping != 0u;
    float tier = float(bypass ? 16 : 4);

    float4 reprojection = reprojection_tex.SampleLevel(sampler_nnc, uv, 0.0f);
    float2 motion = reprojection.xy;
    bool history_available = (history_valid != 0u) && (frame_index != 0u);
    float2 aux = history_available
        ? aux_tex.SampleLevel(sampler_lnc, uv + motion, 0.0f).xy : float2(0.0f, 0.0f);
    float aux_tier = aux.y * tier;

    float3 reproj_hist = float3(reprojected_history_tex_lum.Load(int3(dispatch_id.xy, 0)).x,
        reprojected_history_tex_chroma.Load(int3(dispatch_id.xy, 0)).xy);

    float2 scale = float2(int2(color_tex_dims)) / out_dims;
    int2 current_px = int2((float2(int2(dispatch_id.xy)) + 0.5f) * scale);
    float2 current_pos = (float2(current_px) + 0.5f
        + (view_constants[0].sample_offset_pixels * float2(1.0f, -1.0f))) / scale;
    float sigma_wide = ((-10.0f) * scale.x) * min(4.0f * scale.x, 1.0f);
    float sigma_narrow = (-1.0f) * (scale.x * scale.x);

    float3 sum_sq = float3(0.0f, 0.0f, 0.0f);
    float sum_w1 = 0.0f;
    float3 sum_c = float3(0.0f, 0.0f, 0.0f);
    float sum_w2 = 0.0f;
    float4 sum_c2 = float4(0.0f, 0.0f, 0.0f, 0.0f);
    [loop]
    for (int dy = -1; dy <= 1; ++dy)
    {
        [loop]
        for (int dx = -1; dx <= 1; ++dx)
        {
            float4 c = float4(rgb_to_ycbcr(
                color_tex.Load(int3(uint2(current_px + int2(dx, dy)), 0)).xyz), 1.0f);
            float2 d = (current_pos + (float2(float(dx), float(dy)) / scale))
                - (float2(int2(dispatch_id.xy)) + 0.5f);
            float dist = dot(d, d);
            float w_narrow = exp2(dist * sigma_narrow);
            float w_wide = exp2(dist * sigma_wide);
            sum_w1 += w_narrow;
            sum_sq += (c.xyz * c.xyz) * w_narrow;
            sum_c += c.xyz * w_narrow;
            sum_w2 += w_wide;
            sum_c2 += c * w_wide;
        }
    }

    float3 mean = sum_c / sum_w1;
    float3 wide_result = sum_c2.xyz;

    float blur_sum = 0.0f;
    float blur_weight = 0.0f;
    [loop]
    for (int by = -1; by <= 1; ++by)
    {
        [loop]
        for (int bx = -1; bx <= 1; ++bx)
        {
            float w = exp(float((bx * bx) + (by * by)) * (-0.75f));
            blur_sum += reprojected_history_tex_lum.SampleLevel(sampler_lnc,
                uv + ((float2(float(bx), float(by)) * 2.0f) / float2(color_tex_dims)), 0.0f).x * w;
            blur_weight += w;
        }
    }
    float hist_lum_blur = clamp(sqrt(blur_sum / blur_weight), 0.0f, 1.0f);
    float hist_lum_blur_sq = hist_lum_blur * hist_lum_blur;
    float current_lum = blurred_luminance_tex.SampleLevel(sampler_lnc, uv, 0.0f).x;
    float current_lum_sq = current_lum * current_lum;

    float wide_lum_blur;
    float wide_hist_blur;
    if (WaveActiveMax(WaveGetLaneIndex()) >= 31u)
    {
        wide_lum_blur = lerp(current_lum_sq, WaveReadLaneAt(current_lum_sq, WaveGetLaneIndex() ^ 16u), 0.5f);
        wide_hist_blur = lerp(hist_lum_blur_sq, WaveReadLaneAt(hist_lum_blur_sq, WaveGetLaneIndex() ^ 16u), 0.5f);
    }
    else
    {
        wide_lum_blur = current_lum_sq;
        wide_hist_blur = hist_lum_blur_sq;
    }
    float lum_delta = lerp(wide_hist_blur, WaveReadLaneAt(wide_hist_blur, WaveGetLaneIndex() ^ 2u), 0.5f);
    float lum_delta2 = lerp(wide_lum_blur, WaveReadLaneAt(wide_lum_blur, WaveGetLaneIndex() ^ 2u), 0.5f);

    float3 std = sqrt(max(float3(0.0f, 0.0f, 0.0f), (sum_sq / sum_w1) - (mean * mean))) * 2.0f;
    float3 clamp_min = (mean - std) - 0.001f;
    float3 clamp_max = (mean + std) + 0.001f;
    float2 reproj_dims = float2(reprojection_tex_dims);

    float hist_sum_sq = 0.0f;
    float hist_sum = 0.0f;
    float3 below_count = float3(0.0f, 0.0f, 0.0f);
    float3 above_count = float3(0.0f, 0.0f, 0.0f);
    float hist_count = 0.0f;
    [loop]
    for (int hy = -1; hy <= 1; ++hy)
    {
        [loop]
        for (int hx = -1; hx <= 1; ++hx)
        {
            uint2 sample_px = dispatch_id.xy + uint2(hx, hy);
            float lum = reprojected_history_tex_lum.Load(int3(sample_px, 0)).x;
            float3 sample = float3(lum, 0.5f, reprojected_history_tex_chroma.Load(int3(sample_px, 0)).y);
            hist_sum_sq += lum * lum;
            hist_sum += lum;
            below_count += float3(sample.x < clamp_min.x, sample.y < clamp_min.y, sample.z < clamp_min.z) * 1.0f;
            above_count += float3(sample.x > clamp_max.x, sample.y > clamp_max.y, sample.z > clamp_max.z) * 1.0f;
            hist_count += 1.0f;
        }
    }
    float hist_mean = hist_sum / hist_count;
    float hist_std = sqrt(max(0.0f, (hist_sum_sq / hist_count) - (hist_mean * hist_mean)));
    float4 gather_alpha = reprojection_tex.GatherAlpha(sampler_lnc, uv);
    float history_inconsistency = clamp((20.0f * max(0.0f, hist_std)) / max(9.99999993922529e-09f, hist_mean), 0.0f, 1.0f);
    float2 outside_frac = (above_count / hist_count).xz + (below_count / hist_count).xz;

    float lum_delta_abs = abs(lum_delta - lum_delta2);
    float lum_delta_norm = (2.0f * lum_delta_abs) / max(9.9999997473787516e-06f, lum_delta2 + lum_delta);
    float mean_lum = mean.x;

    float reproj_zero_count = 0.0f;
    float reproj_count = 0.0f;
    [loop]
    for (int ry = -1; ry <= 1; ++ry)
    {
        [loop]
        for (int rx = -1; rx <= 1; ++rx)
        {
            reproj_zero_count += float(reprojection_tex.SampleLevel(sampler_nnc,
                uv - ((reproj_dims * 0.5f) * float2(float(rx), float(ry))), 0.0f).z == 0.0f);
            reproj_count += 1.0f;
        }
    }

    float2 motion_px = motion * out_dims;
    float2 motion_abs = abs(motion_px);
    float4 gather_blue = reprojection_tex.GatherBlue(sampler_lnc, uv);
    float reproj_blue_term = (((3.0f * clamp(1.0f - ((2.5f * hist_std) / hist_mean), 0.0f, 1.0f)) * lum_delta_abs)
        / (0.5f * max(9.9999997473787516e-05f, lum_delta2 + lum_delta)))
        * float(min(min(gather_blue.x, gather_blue.y), min(gather_blue.z, gather_blue.w)) == 1.0f);

    float reproj_occlusion = sqrt(reproj_zero_count / reproj_count)
        * clamp(max(motion_abs.x, motion_abs.y), 0.0f, 1.0f);
    float blend_factor =
        ((((aux.x * 2.0f) - 0.5f) + clamp(1.0f - history_inconsistency, 0.0f, 1.0f))
            + max(outside_frac.x, outside_frac.y)
            + ((lum_delta_norm * lum_delta_norm) * max(0.0f, 1.0f
                - ((((-3.0f) * abs(mean_lum - lum_delta2)) + (2.0f * abs((sum_c2.x / sum_w2) - mean_lum))) / lum_delta2)))
            + reproj_occlusion)
        + ((reproj_blue_term * reproj_blue_term) * reproj_blue_term);

    float color_blend;
    float motion_blend;
    if (!bypass)
    {
        float alpha_mask = clamp(1.0f - min(min(gather_alpha.x, gather_alpha.y), min(gather_alpha.z, gather_alpha.w)), 0.0f, 1.0f);
        if (alpha_mask < 0.75f)
        {
            color_blend = blend_factor + 1.0f;
            motion_blend = min(aux_tier, 0.5f);
        }
        else
        {
            color_blend = blend_factor;
            motion_blend = aux_tier;
        }
        float motion_factor = motion_blend * history_inconsistency;
        float2 motion_norm = abs(motion_px * 0.03125f);
        color_blend = min(motion_factor, lerp(motion_factor, 1.0f, clamp(max(motion_norm.x, motion_norm.y), 0.0f, 1.0f)));
    }
    else
    {
        color_blend = aux_tier;
        motion_blend = blend_factor;
    }

    float final_blend = min(color_blend, 1.0f);
    float3 clamp_range = std * exp2((7.0f * clamp(1.0f - color_blend, 0.0f, 1.0f)) - 0.5f);
    float3 clamped_hist = bypass ? reproj_hist : clamp(reproj_hist, mean - clamp_range, mean + clamp_range);

    float blend_t = min(4.0f, color_blend);
    float total_weight = sum_w2 + blend_t;
    float3 blended = lerp(wide_result / max(9.9999997473787516e-06f, sum_w2).xxx, clamped_hist,
        (blend_t / max(9.9999997473787516e-06f, total_weight)).xxx);
    float blended_lum = blended.x;

    bool valid = (history_valid != 0u) && (frame_index != 0u);
    float3 color;
    if (!valid)
    {
        color = max(float3(0.0f, 0.0f, 0.0f),
            ycbcr_to_rgb(wide_result / max(9.9999997473787516e-06f, sum_w2).xxx));
    }
    else
    {
        color = max(float3(0.0f, 0.0f, 0.0f), ycbcr_to_rgb(blended));
    }

    history_output_tex[dispatch_id.xy] = curve(color);

    tile_luminance[local_id.x][local_id.y] = blended_lum;
    GroupMemoryBarrierWithGroupSync();

    uint2 right = local_id.xy + uint2(1u, 0u);
    uint2 left = local_id.xy - uint2(1u, 0u);
    float x_accum = 0.0f;
    float x_weight = 0.0f;
    if (all(bool4(right.x < 8u, right.y < 8u, left.x < 8u, left.y < 8u)))
    {
        float w = max(0.0f, 1.0f - (6.0f
            * (abs(blended_lum - tile_luminance[right.x][right.y]) + abs(blended_lum - tile_luminance[left.x][left.y]))));
        float w2 = min(w, (sharpen_amount * w) * 1.25f);
        x_accum = w2 * (tile_luminance[right.x][right.y] + tile_luminance[left.x][left.y]);
        x_weight = w2 * 2.0f;
    }

    uint2 up = local_id.xy + uint2(0u, 1u);
    uint2 down = local_id.xy - uint2(0u, 1u);
    float y_accum;
    float y_weight;
    if (all(bool4(up.x < 8u, up.y < 8u, down.x < 8u, down.y < 8u)))
    {
        float w = max(0.0f, 1.0f - (6.0f
            * (abs(blended_lum - tile_luminance[up.x][up.y]) + abs(blended_lum - tile_luminance[down.x][down.y]))));
        float w2 = min(w, (sharpen_amount * w) * 1.25f);
        y_accum = (x_accum + (tile_luminance[up.x][up.y] * w2)) + (tile_luminance[down.x][down.y] * w2);
        y_weight = x_weight + (w2 * 2.0f);
    }
    else
    {
        y_accum = x_accum;
        y_weight = x_weight;
    }

    float3 sharpened = color * max(0.0f, max(0.0f,
        (blended_lum * (y_weight + 1.0f)) - y_accum) / max(9.9999997473787516e-06f, blended_lum));
    output_tex[dispatch_id.xy] = curve(sharpened);

    aux_output_tex[dispatch_id.xy] = float2(
        clamp(final_blend + (2.0f * clamp(reprojection.w * 1000.0f, 0.0f, 1.0f)), 0.0f, 2.0f) * 0.5f,
        total_weight / tier);
}
