// PATCHED: modules ain_3ee6a953408e4c4c.bin:cs_main, ain_7a0af18450cdbcd1.bin:cs_main, ain_b303f36f0dbdd5f6.bin:cs_main, ain_f7efabb81dfe256b.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_3ee6a953408e4c4c) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_7a0af18450cdbcd1) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_b303f36f0dbdd5f6) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_f7efabb81dfe256b) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/tgsr.cs.hlsl | A (dof-aa review, 2026-10-13): A 胜出
//   (混合按 WIDE 权重 wc_sum 除;历史采样中间分量常为 0.5;B 用窄权重与 0.0)。
//   8x4 LocalSize 变体是有记录的编译期变体。
//   TGSR(时域高斯超分)累积:重投影历史 + 当前帧统计驱动 YCoCg 中的自适应混合/钳制,
//   带模糊亮度运动否决、曲线量化历史输出与瓦片亮度锐化。aux_output 存逐像素
//   (透明度, 混合) 状态。

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
    // Blend divides by the WIDE weight (wc_sum) -- A (target); B divided by the narrow weight.
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

#endif

#if defined(TGV_3ee6a953408e4c4c) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _tgsr.cs.hlsl_cs_main_3ee6a953408e4c4c.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
// dxc 1.9/1.10 lowers the native frexp() to invalid SPIR-V (OpConvertSToF on a non-int); same
// workaround as inc/curve.hlsl.
float3 incb_frexp(float3 x, out int3 exponent)
{
    // denormals are normalised first (x * 2^24) so the exponent/mantissa match GLSL Frexp exactly
    uint3 bits0 = asuint(x);
    bool3 denorm = ((bits0 & 0x7F800000u) == 0u) & ((bits0 & 0x7FFFFFFFu) != 0u);
    float3 xs = select(denorm, x * 16777216.0f, x);
    uint3 bits = asuint(xs);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu), int((bits.y >> 23u) & 0xFFu), int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127) - select(denorm, int3(24, 24, 24), int3(0, 0, 0));
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, xs, m);
}
#define frexp incb_frexp
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

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

static float ycbcr_alpha;
static float chroma_mid;

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
    uint2 reprojected_history_tex_lum_dims : packoffset(c0);
    uint2 reprojected_history_tex_chroma_dims : packoffset(c0.z);
    uint2 color_tex_dims : packoffset(c1);
    uint2 blurred_luminance_tex_dims : packoffset(c1.z);
    uint2 history_tex_dims : packoffset(c2);
    uint2 reprojection_tex_dims : packoffset(c2.z);
    uint2 aux_tex_dims : packoffset(c3);
    uint2 history_output_tex_dims : packoffset(c3.z);
    uint2 output_tex_dims : packoffset(c4);
    uint2 aux_output_tex_dims : packoffset(c4.z);
    uint bypass_neighborhood_clamping : packoffset(c5);
    float sharpen_amount : packoffset(c5.y);
    uint history_valid : packoffset(c5.z);
};

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> reprojected_history_tex_lum : register(t14, space0);
Texture2D<float4> reprojected_history_tex_chroma : register(t15, space0);
Texture2D<float4> color_tex : register(t16, space0);
Texture2D<float4> blurred_luminance_tex : register(t17, space0);
Texture2D<float4> reprojection_tex : register(t19, space0);
Texture2D<float4> aux_tex : register(t20, space0);
RWTexture2D<float3> history_output_tex : register(u21, space0);
RWTexture2D<float3> output_tex : register(u22, space0);
RWTexture2D<unorm float2> aux_output_tex : register(u23, space0);

static uint3 gl_LocalInvocationID;
static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_LocalInvocationID : SV_GroupThreadID;
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

groupshared float tile_luminance[8][8];

void comp_main()
{
    float2 out_dims = float2(output_tex_dims);
    float2 center = float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx;
    float2 uv = center / out_dims;
    bool bypass = bypass_neighborhood_clamping != 0u;
    float tier = float(bypass ? 16 : 4);
    float4 reprojection4 = reprojection_tex.SampleLevel(sampler_nnc, uv, 0.0f);
    float2 motion = reprojection4.xy;
    bool history_valid_b = history_valid != 0u;
    bool history_available;
    if (history_valid_b)
    {
        history_available = !(0u == frame_constants_frame_index);
    }
    else
    {
        history_available = false;
    }
    float2 aux;
    if (history_available)
    {
        aux = aux_tex.SampleLevel(sampler_lnc, uv + motion, 0.0f).xy;
    }
    else
    {
        aux = 0.0f.xx;
    }
    float aux_tier = aux.y * tier;
    float3 reproj_hist = float3(reprojected_history_tex_lum.Load(int3(gl_GlobalInvocationID.xy, 0u)).x, reprojected_history_tex_chroma.Load(int3(gl_GlobalInvocationID.xy, 0u)).xy);
    float2 scale = float2(int2(color_tex_dims)) / out_dims;
    int2 current_px = int2(center * scale);
    float2 current_pos = ((float2(current_px) + 0.5f.xx) + (frame_constants_view_constants[0u].sample_offset_pixels * float2(1.0f, -1.0f))) / scale;
    float scale_x = scale.x;
    float sigma_wide = ((-10.0f) * scale_x) * min(4.0f * scale_x, 1.0f);
    float sigma_narrow = (-1.0f) * (scale_x * scale_x);
    float3 sum_sq;
    float sum_w1;
    float3 sum_c;
    float sum_w2;
    float4 sum_c2;
    int dy;
    sum_sq = 0.0f.xxx;
    sum_w1 = 0.0f;
    sum_c = 0.0f.xxx;
    sum_w2 = 0.0f;
    sum_c2 = 0.0f.xxxx;
    dy = -1;
    float3 next_sum_sq;
    float next_sum_w1;
    float3 next_sum_c;
    float next_sum_w2;
    float4 next_sum_c2;
    for (; dy <= 1; sum_sq = next_sum_sq, sum_w1 = next_sum_w1, sum_c = next_sum_c, sum_w2 = next_sum_w2, sum_c2 = next_sum_c2, dy++)
    {
        next_sum_w1 = sum_w1;
        next_sum_sq = sum_sq;
        next_sum_c = sum_c;
        next_sum_w2 = sum_w2;
        next_sum_c2 = sum_c2;
        for (int dx = -1; dx <= 1; )
        {
            float4 c = float4(mul(float3x3(float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f), float3(-0.114600002765655517578125f, -0.385399997234344482421875f, 0.5f), float3(0.5f, -0.4541999995708465576171875f, -0.0458000004291534423828125f)), color_tex.Load(int3(uint2(current_px + int2(dx, dy)), 0u)).xyz), ycbcr_alpha);
            float2 d = (current_pos + (float2(float(dx), float(dy)) / scale)) - center;
            float dist = dot(d, d);
            float w_narrow = exp2(dist * sigma_narrow);
            float w_wide = exp2(dist * sigma_wide);
            float3 c_xyz = c.xyz;
            next_sum_w1 += w_narrow;
            next_sum_sq += ((c_xyz * c_xyz) * w_narrow);
            next_sum_c += (c_xyz * w_narrow);
            next_sum_w2 += w_wide;
            next_sum_c2 += (c * w_wide);
            dx++;
            continue;
        }
    }
    float3 w1_vec = sum_w1.xxx;
    float3 mean = sum_c / w1_vec;
    float3 clamp_center = sum_c2.xyz;
    float lum_sum;
    float lum_w;
    int lum_dy;
    lum_sum = 0.0f;
    lum_w = 0.0f;
    lum_dy = -1;
    float next_lum_sum;
    float next_lum_w;
    for (; lum_dy <= 1; lum_sum = next_lum_sum, lum_w = next_lum_w, lum_dy++)
    {
        next_lum_sum = lum_sum;
        next_lum_w = lum_w;
        for (int lum_dx = -1; lum_dx <= 1; )
        {
            float lum_wgt = exp(float((lum_dx * lum_dx) + (lum_dy * lum_dy)) * (-0.75f));
            next_lum_sum += (reprojected_history_tex_lum.SampleLevel(sampler_lnc, uv + ((float2(float(lum_dx), float(lum_dy)) * 2.0f) / float2(color_tex_dims)), 0.0f).x * lum_wgt);
            next_lum_w += lum_wgt;
            lum_dx++;
            continue;
        }
    }
    float lum_std = clamp(sqrt(lum_sum / lum_w), 0.0f, 1.0f);
    float lum_std_sq = lum_std * lum_std;
    float4 blur_sample = blurred_luminance_tex.SampleLevel(sampler_lnc, uv, 0.0f);
    float blur_lum = blur_sample.x;
    float blur_lum_sq = blur_lum * blur_lum;
    float blur_lerp_a;
    float std_lerp_a;
    if (WaveActiveMax(WaveGetLaneIndex()) >= 31u)
    {
        uint lane_xor16 = WaveGetLaneIndex() ^ 16u;
        blur_lerp_a = lerp(blur_lum_sq, WaveReadLaneAt(blur_lum_sq, lane_xor16), 0.5f);
        std_lerp_a = lerp(lum_std_sq, WaveReadLaneAt(lum_std_sq, lane_xor16), 0.5f);
    }
    else
    {
        blur_lerp_a = blur_lum_sq;
        std_lerp_a = lum_std_sq;
    }
    uint lane_xor2 = WaveGetLaneIndex() ^ 2u;
    float std_other = WaveReadLaneAt(std_lerp_a, lane_xor2);
    float std_half = lerp(std_lerp_a, std_other, 0.5f);
    float blur_other = WaveReadLaneAt(blur_lerp_a, lane_xor2);
    float blur_half = lerp(blur_lerp_a, blur_other, 0.5f);
    float3 std = sqrt(max(0.0f.xxx, (sum_sq / w1_vec) - (mean * mean)));
    float2 reproj_tap = 0.5f.xx / float2(reprojection_tex_dims);
    float3 std2 = std * 2.0f;
    float3 clamp_max = (mean + std2) + 0.001000000047497451305389404296875f.xxx;
    float3 clamp_min = (mean - std2) - 0.001000000047497451305389404296875f.xxx;
    float hist_sum_sq;
    float hist_sum;
    float3 hist_less;
    float3 hist_greater;
    float hist_count;
    int hist_dy;
    hist_sum_sq = 0.0f;
    hist_sum = 0.0f;
    hist_less = 0.0f.xxx;
    hist_greater = 0.0f.xxx;
    hist_count = 0.0f;
    hist_dy = -1;
    float next_hist_sum_sq;
    float next_hist_sum;
    float3 next_hist_less;
    float3 next_hist_greater;
    float next_hist_count;
    for (; hist_dy <= 1; hist_sum_sq = next_hist_sum_sq, hist_sum = next_hist_sum, hist_less = next_hist_less, hist_greater = next_hist_greater, hist_count = next_hist_count, hist_dy++)
    {
        next_hist_sum_sq = hist_sum_sq;
        next_hist_sum = hist_sum;
        next_hist_less = hist_less;
        next_hist_greater = hist_greater;
        next_hist_count = hist_count;
        for (int hist_dx = -1; hist_dx <= 1; )
        {
            uint2 hist_px = gl_GlobalInvocationID.xy + uint2(int2(hist_dx, hist_dy));
            float4 hist_lum4 = reprojected_history_tex_lum.Load(int3(hist_px, 0u));
            float hist_lum = hist_lum4.x;
            float3 hist_sample = float3(hist_lum, chroma_mid, reprojected_history_tex_chroma.Load(int3(hist_px, 0u)).y);
            next_hist_sum_sq += (hist_lum * hist_lum);
            next_hist_sum += hist_lum;
            next_hist_less += (select(bool3(hist_sample.x < clamp_min.x, hist_sample.y < clamp_min.y, hist_sample.z < clamp_min.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            next_hist_greater += (select(bool3(hist_sample.x > clamp_max.x, hist_sample.y > clamp_max.y, hist_sample.z > clamp_max.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            next_hist_count += 1.0f;
            hist_dx++;
            continue;
        }
    }
    float3 count_vec = hist_count.xxx;
    float hist_mean = hist_sum / hist_count;
    float hist_std = sqrt(max(0.0f, (hist_sum_sq / hist_count) - (hist_mean * hist_mean)));
    float4 reproj_gather = reprojection_tex.GatherAlpha(sampler_lnc, uv);
    float clamp_w = clamp((20.0f * max(0.0f, hist_std)) / max(9.9999999392252902907785028219223e-09f, hist_mean), 0.0f, 1.0f);
    float2 clip_counts = (hist_greater / count_vec).xz + (hist_less / count_vec).xz;
    float lum_diff = abs(std_half - blur_half);
    float lum_ratio = (2.0f * lum_diff) / max(9.9999997473787516355514526367188e-06f, blur_half + std_half);
    float mean_x = mean.x;
    float nz_sum;
    float nz_count;
    int nz_dy;
    nz_sum = 0.0f;
    nz_count = 0.0f;
    nz_dy = -1;
    float next_nz_sum;
    float next_nz_count;
    for (; nz_dy <= 1; nz_sum = next_nz_sum, nz_count = next_nz_count, nz_dy++)
    {
        next_nz_sum = nz_sum;
        next_nz_count = nz_count;
        for (int nz_dx = -1; nz_dx <= 1; )
        {
            next_nz_sum += ((reprojection_tex.SampleLevel(sampler_nnc, uv - ((reproj_tap * 1.0f) * float2(float(nz_dx), float(nz_dy))), 0.0f).z == 0.0f) ? 1.0f : 0.0f);
            next_nz_count += 1.0f;
            nz_dx++;
            continue;
        }
    }
    float2 motion_px = motion * out_dims;
    float2 abs_motion = abs(motion_px);
    float4 reproj_gather_b = reprojection_tex.GatherBlue(sampler_lnc, uv);
    float edge_factor = (((3.0f * clamp(1.0f - ((2.5f * hist_std) / hist_mean), 0.0f, 1.0f)) * lum_diff) / (0.5f * max(9.9999997473787516355514526367188e-05f, std_half + blur_half))) * ((min(min(reproj_gather_b.x, reproj_gather_b.y), min(reproj_gather_b.z, reproj_gather_b.w)) == 1.0f) ? 1.0f : 0.0f);
    float motion_penalty = ((((((aux.x * 2.0f) - 0.5f) + clamp(1.0f - clamp_w, 0.0f, 1.0f)) + max(clip_counts.x, clip_counts.y)) + ((lum_ratio * lum_ratio) * max(0.0f, 1.0f - ((((-3.0f) * abs(mean_x - blur_half)) + (2.0f * abs((sum_c2.x / sum_w2) - mean_x))) / blur_half)))) + (sqrt(nz_sum / nz_count) * clamp(max(abs_motion.x, abs_motion.y), 0.0f, 1.0f))) + ((edge_factor * edge_factor) * edge_factor);
    bool use_clamp = !bypass;
    float blend_w;
    float blend_t;
    if (use_clamp)
    {
        float blend_t_adj;
        float aux_tier_adj;
        if (clamp(1.0f - min(min(reproj_gather.x, reproj_gather.y), min(reproj_gather.z, reproj_gather.w)), 0.0f, 1.0f) < 0.75f)
        {
            blend_t_adj = motion_penalty + 1.0f;
            aux_tier_adj = min(aux_tier, 0.5f);
        }
        else
        {
            blend_t_adj = motion_penalty;
            aux_tier_adj = aux_tier;
        }
        float blend_w2 = aux_tier_adj * clamp_w;
        float2 motion_scaled = abs(motion_px * 0.03125f.xx);
        blend_w = min(blend_w2, lerp(blend_w2, 1.0f, clamp(max(motion_scaled.x, motion_scaled.y), 0.0f, 1.0f)));
        blend_t = blend_t_adj;
    }
    else
    {
        blend_w = aux_tier;
        blend_t = motion_penalty;
    }
    float blend_t_clamped = min(blend_t, 1.0f);
    float3 clamp_half = std * exp2((7.0f * clamp(1.0f - blend_t, 0.0f, 1.0f)) - 0.5f);
    float3 clamped_hist;
    if (use_clamp)
    {
        clamped_hist = clamp(reproj_hist, mean - clamp_half, mean + clamp_half);
    }
    else
    {
        clamped_hist = reproj_hist;
    }
    float blend_amt = min(4.0f, blend_w);
    float w1_total = sum_w2 + blend_amt;
    float3 w1_vec_b = sum_w2.xxx;
    float3 blended = lerp(clamp_center / w1_vec_b, clamped_hist, (blend_amt / max(9.9999997473787516355514526367188e-06f, w1_total)).xxx);
    float blended_x = blended.x;
    bool hist_ok;
    if (history_valid_b)
    {
        hist_ok = !(0u == frame_constants_frame_index);
    }
    else
    {
        hist_ok = false;
    }
    float3 final_color;
    if (!hist_ok)
    {
        final_color = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), clamp_center / max(9.9999997473787516355514526367188e-06f, sum_w2).xxx));
    }
    else
    {
        final_color = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), blended));
    }
    FrexpStructType frexp_res;
    { int3 _fe; frexp_res.mantissa = frexp(final_color, _fe); frexp_res.exponent = _fe; }
    history_output_tex[gl_GlobalInvocationID.xy] = final_color + exp2(float3(frexp_res.exponent) - float3(8.0f, 8.0f, 7.0f));
    tile_luminance[gl_LocalInvocationID.x][gl_LocalInvocationID.y] = blended_x;
    GroupMemoryBarrierWithGroupSync();
    uint2 right = gl_LocalInvocationID.xy + uint2(1u, 0u);
    uint2 left = gl_LocalInvocationID.xy - uint2(1u, 0u);
    uint right_x = right.x;
    uint right_y = right.y;
    uint left_x = left.x;
    uint left_y = left.y;
    uint4 bounds_x = uint4(right_x, right_y, left_x, left_y);
    float x_accum;
    float x_weight;
    if (all(bool4(bounds_x.x < uint4(8u, 8u, 8u, 8u).x, bounds_x.y < uint4(8u, 8u, 8u, 8u).y, bounds_x.z < uint4(8u, 8u, 8u, 8u).z, bounds_x.w < uint4(8u, 8u, 8u, 8u).w)))
    {
        float w = max(0.0f, 1.0f - (6.0f * (abs(blended_x - tile_luminance[right_x][right_y]) + abs(blended_x - tile_luminance[left_x][left_y]))));
        float w2_sharpen = min(w, (sharpen_amount * w) * 1.25f);
        x_accum = w2_sharpen * (tile_luminance[right_x][right_y] + tile_luminance[left_x][left_y]);
        x_weight = w2_sharpen * 2.0f;
    }
    else
    {
        x_accum = 0.0f;
        x_weight = 0.0f;
    }
    uint2 up = gl_LocalInvocationID.xy + uint2(0u, 1u);
    uint2 down = gl_LocalInvocationID.xy - uint2(0u, 1u);
    uint up_x = up.x;
    uint up_y = up.y;
    uint down_x = down.x;
    uint down_y = down.y;
    uint4 bounds_y = uint4(up_x, up_y, down_x, down_y);
    float y_accum;
    float y_weight;
    if (all(bool4(bounds_y.x < uint4(8u, 8u, 8u, 8u).x, bounds_y.y < uint4(8u, 8u, 8u, 8u).y, bounds_y.z < uint4(8u, 8u, 8u, 8u).z, bounds_y.w < uint4(8u, 8u, 8u, 8u).w)))
    {
        float w_y = max(0.0f, 1.0f - (6.0f * (abs(blended_x - tile_luminance[up_x][up_y]) + abs(blended_x - tile_luminance[down_x][down_y]))));
        float w2_y = min(w_y, (sharpen_amount * w_y) * 1.25f);
        y_accum = (x_accum + (tile_luminance[up_x][up_y] * w2_y)) + (tile_luminance[down_x][down_y] * w2_y);
        y_weight = x_weight + (w2_y * 2.0f);
    }
    else
    {
        y_accum = x_accum;
        y_weight = x_weight;
    }
    float3 final_sharpened = final_color * max(0.0f, max(0.0f, (blended_x * (y_weight + 1.0f)) - y_accum) / max(9.9999997473787516355514526367188e-06f, blended_x));
    FrexpStructType frexp_res2;
    { int3 _fe; frexp_res2.mantissa = frexp(final_sharpened, _fe); frexp_res2.exponent = _fe; }
    output_tex[gl_GlobalInvocationID.xy] = final_sharpened + exp2(float3(frexp_res2.exponent) - float3(8.0f, 8.0f, 7.0f));
    aux_output_tex[gl_GlobalInvocationID.xy] = float2(clamp(blend_t_clamped + (2.0f * clamp(reprojection4.w * 1000.0f, 0.0f, 1.0f)), 0.0f, 2.0f) * 0.5f, w1_total / tier);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_LocalInvocationID = stage_input.gl_LocalInvocationID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_7a0af18450cdbcd1) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _tgsr.cs.hlsl_cs_main_7a0af18450cdbcd1.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
// dxc 1.9/1.10 lowers the native frexp() to invalid SPIR-V (OpConvertSToF on a non-int); same
// workaround as inc/curve.hlsl.
float3 incb_frexp(float3 x, out int3 exponent)
{
    // denormals are normalised first (x * 2^24) so the exponent/mantissa match GLSL Frexp exactly
    uint3 bits0 = asuint(x);
    bool3 denorm = ((bits0 & 0x7F800000u) == 0u) & ((bits0 & 0x7FFFFFFFu) != 0u);
    float3 xs = select(denorm, x * 16777216.0f, x);
    uint3 bits = asuint(xs);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu), int((bits.y >> 23u) & 0xFFu), int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127) - select(denorm, int3(24, 24, 24), int3(0, 0, 0));
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, xs, m);
}
#define frexp incb_frexp
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

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

static float ycbcr_alpha;
static float chroma_mid;

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
    uint2 reprojected_history_tex_lum_dims : packoffset(c0);
    uint2 reprojected_history_tex_chroma_dims : packoffset(c0.z);
    uint2 color_tex_dims : packoffset(c1);
    uint2 blurred_luminance_tex_dims : packoffset(c1.z);
    uint2 history_tex_dims : packoffset(c2);
    uint2 reprojection_tex_dims : packoffset(c2.z);
    uint2 aux_tex_dims : packoffset(c3);
    uint2 history_output_tex_dims : packoffset(c3.z);
    uint2 output_tex_dims : packoffset(c4);
    uint2 aux_output_tex_dims : packoffset(c4.z);
    uint bypass_neighborhood_clamping : packoffset(c5);
    float sharpen_amount : packoffset(c5.y);
    uint history_valid : packoffset(c5.z);
};

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> reprojected_history_tex_lum : register(t14, space0);
Texture2D<float4> reprojected_history_tex_chroma : register(t15, space0);
Texture2D<float4> color_tex : register(t16, space0);
Texture2D<float4> blurred_luminance_tex : register(t17, space0);
Texture2D<float4> reprojection_tex : register(t19, space0);
Texture2D<float4> aux_tex : register(t20, space0);
RWTexture2D<float3> history_output_tex : register(u21, space0);
RWTexture2D<float3> output_tex : register(u22, space0);
RWTexture2D<unorm float2> aux_output_tex : register(u23, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    float2 out_dims = float2(output_tex_dims);
    float2 center = float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx;
    float2 uv = center / out_dims;
    bool bypass = bypass_neighborhood_clamping != 0u;
    float tier = float(bypass ? 16 : 4);
    float4 reprojection4 = reprojection_tex.SampleLevel(sampler_nnc, uv, 0.0f);
    float2 motion = reprojection4.xy;
    bool history_valid_b = history_valid != 0u;
    bool history_available;
    if (history_valid_b)
    {
        history_available = !(0u == frame_constants_frame_index);
    }
    else
    {
        history_available = false;
    }
    float2 aux;
    if (history_available)
    {
        aux = aux_tex.SampleLevel(sampler_lnc, uv + motion, 0.0f).xy;
    }
    else
    {
        aux = 0.0f.xx;
    }
    float aux_tier = aux.y * tier;
    float3 reproj_hist = float3(reprojected_history_tex_lum.Load(int3(gl_GlobalInvocationID.xy, 0u)).x, reprojected_history_tex_chroma.Load(int3(gl_GlobalInvocationID.xy, 0u)).xy);
    float2 scale = float2(int2(color_tex_dims)) / out_dims;
    int2 current_px = int2(center * scale);
    float2 current_pos = ((float2(current_px) + 0.5f.xx) + (frame_constants_view_constants[0u].sample_offset_pixels * float2(1.0f, -1.0f))) / scale;
    float scale_x = scale.x;
    float sigma_wide = ((-10.0f) * scale_x) * min(4.0f * scale_x, 1.0f);
    float sigma_narrow = (-1.0f) * (scale_x * scale_x);
    float3 sum_sq;
    float sum_w1;
    float3 sum_c;
    float sum_w2;
    float4 sum_c2;
    int dy;
    sum_sq = 0.0f.xxx;
    sum_w1 = 0.0f;
    sum_c = 0.0f.xxx;
    sum_w2 = 0.0f;
    sum_c2 = 0.0f.xxxx;
    dy = -1;
    float3 next_sum_sq;
    float next_sum_w1;
    float3 next_sum_c;
    float next_sum_w2;
    float4 next_sum_c2;
    for (; dy <= 1; sum_sq = next_sum_sq, sum_w1 = next_sum_w1, sum_c = next_sum_c, sum_w2 = next_sum_w2, sum_c2 = next_sum_c2, dy++)
    {
        next_sum_w1 = sum_w1;
        next_sum_sq = sum_sq;
        next_sum_c = sum_c;
        next_sum_w2 = sum_w2;
        next_sum_c2 = sum_c2;
        for (int dx = -1; dx <= 1; )
        {
            float4 c = float4(mul(float3x3(float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f), float3(-0.114600002765655517578125f, -0.385399997234344482421875f, 0.5f), float3(0.5f, -0.4541999995708465576171875f, -0.0458000004291534423828125f)), color_tex.Load(int3(uint2(current_px + int2(dx, dy)), 0u)).xyz), ycbcr_alpha);
            float2 d = (current_pos + (float2(float(dx), float(dy)) / scale)) - center;
            float dist = dot(d, d);
            float w_narrow = exp2(dist * sigma_narrow);
            float w_wide = exp2(dist * sigma_wide);
            float3 c_xyz = c.xyz;
            next_sum_w1 += w_narrow;
            next_sum_sq += ((c_xyz * c_xyz) * w_narrow);
            next_sum_c += (c_xyz * w_narrow);
            next_sum_w2 += w_wide;
            next_sum_c2 += (c * w_wide);
            dx++;
            continue;
        }
    }
    float3 w1_vec = sum_w1.xxx;
    float3 mean = sum_c / w1_vec;
    float lum_sum;
    float lum_w;
    int lum_dy;
    lum_sum = 0.0f;
    lum_w = 0.0f;
    lum_dy = -1;
    float next_lum_sum;
    float next_lum_w;
    for (; lum_dy <= 1; lum_sum = next_lum_sum, lum_w = next_lum_w, lum_dy++)
    {
        next_lum_sum = lum_sum;
        next_lum_w = lum_w;
        for (int lum_dx = -1; lum_dx <= 1; )
        {
            float lum_wgt = exp(float((lum_dx * lum_dx) + (lum_dy * lum_dy)) * (-0.75f));
            next_lum_sum += (reprojected_history_tex_lum.SampleLevel(sampler_lnc, uv + ((float2(float(lum_dx), float(lum_dy)) * 2.0f) / float2(color_tex_dims)), 0.0f).x * lum_wgt);
            next_lum_w += lum_wgt;
            lum_dx++;
            continue;
        }
    }
    float lum_std = clamp(sqrt(lum_sum / lum_w), 0.0f, 1.0f);
    float lum_std_sq = lum_std * lum_std;
    float4 blur_sample = blurred_luminance_tex.SampleLevel(sampler_lnc, uv, 0.0f);
    float blur_lum = blur_sample.x;
    float blur_lum_sq = blur_lum * blur_lum;
    float blur_lerp_a;
    float std_lerp_a;
    if (WaveActiveMax(WaveGetLaneIndex()) >= 31u)
    {
        uint lane_xor16 = WaveGetLaneIndex() ^ 16u;
        blur_lerp_a = lerp(blur_lum_sq, WaveReadLaneAt(blur_lum_sq, lane_xor16), 0.5f);
        std_lerp_a = lerp(lum_std_sq, WaveReadLaneAt(lum_std_sq, lane_xor16), 0.5f);
    }
    else
    {
        blur_lerp_a = blur_lum_sq;
        std_lerp_a = lum_std_sq;
    }
    uint lane_xor2 = WaveGetLaneIndex() ^ 2u;
    float std_other = WaveReadLaneAt(std_lerp_a, lane_xor2);
    float std_half = lerp(std_lerp_a, std_other, 0.5f);
    float blur_other = WaveReadLaneAt(blur_lerp_a, lane_xor2);
    float blur_half = lerp(blur_lerp_a, blur_other, 0.5f);
    float3 std = sqrt(max(0.0f.xxx, (sum_sq / w1_vec) - (mean * mean)));
    float2 reproj_tap = 0.5f.xx / float2(reprojection_tex_dims);
    float3 std2 = std * 2.0f;
    float3 clamp_max = (mean + std2) + 0.001000000047497451305389404296875f.xxx;
    float3 clamp_min = (mean - std2) - 0.001000000047497451305389404296875f.xxx;
    float hist_sum_sq;
    float hist_sum;
    float3 hist_less;
    float3 hist_greater;
    float hist_count;
    int hist_dy;
    hist_sum_sq = 0.0f;
    hist_sum = 0.0f;
    hist_less = 0.0f.xxx;
    hist_greater = 0.0f.xxx;
    hist_count = 0.0f;
    hist_dy = -1;
    float next_hist_sum_sq;
    float next_hist_sum;
    float3 next_hist_less;
    float3 next_hist_greater;
    float next_hist_count;
    for (; hist_dy <= 1; hist_sum_sq = next_hist_sum_sq, hist_sum = next_hist_sum, hist_less = next_hist_less, hist_greater = next_hist_greater, hist_count = next_hist_count, hist_dy++)
    {
        next_hist_sum_sq = hist_sum_sq;
        next_hist_sum = hist_sum;
        next_hist_less = hist_less;
        next_hist_greater = hist_greater;
        next_hist_count = hist_count;
        for (int hist_dx = -1; hist_dx <= 1; )
        {
            uint2 hist_px = gl_GlobalInvocationID.xy + uint2(int2(hist_dx, hist_dy));
            float4 hist_lum4 = reprojected_history_tex_lum.Load(int3(hist_px, 0u));
            float hist_lum = hist_lum4.x;
            float3 hist_sample = float3(hist_lum, chroma_mid, reprojected_history_tex_chroma.Load(int3(hist_px, 0u)).y);
            next_hist_sum_sq += (hist_lum * hist_lum);
            next_hist_sum += hist_lum;
            next_hist_less += (select(bool3(hist_sample.x < clamp_min.x, hist_sample.y < clamp_min.y, hist_sample.z < clamp_min.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            next_hist_greater += (select(bool3(hist_sample.x > clamp_max.x, hist_sample.y > clamp_max.y, hist_sample.z > clamp_max.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            next_hist_count += 1.0f;
            hist_dx++;
            continue;
        }
    }
    float3 count_vec = hist_count.xxx;
    float hist_mean = hist_sum / hist_count;
    float hist_std = sqrt(max(0.0f, (hist_sum_sq / hist_count) - (hist_mean * hist_mean)));
    float4 reproj_gather = reprojection_tex.GatherAlpha(sampler_lnc, uv);
    float clamp_w = clamp((20.0f * max(0.0f, hist_std)) / max(9.9999999392252902907785028219223e-09f, hist_mean), 0.0f, 1.0f);
    float2 clip_counts = (hist_greater / count_vec).xz + (hist_less / count_vec).xz;
    float lum_diff = abs(std_half - blur_half);
    float lum_ratio = (2.0f * lum_diff) / max(9.9999997473787516355514526367188e-06f, blur_half + std_half);
    float mean_x = mean.x;
    float nz_sum;
    float nz_count;
    int nz_dy;
    nz_sum = 0.0f;
    nz_count = 0.0f;
    nz_dy = -1;
    float next_nz_sum;
    float next_nz_count;
    for (; nz_dy <= 1; nz_sum = next_nz_sum, nz_count = next_nz_count, nz_dy++)
    {
        next_nz_sum = nz_sum;
        next_nz_count = nz_count;
        for (int nz_dx = -1; nz_dx <= 1; )
        {
            next_nz_sum += ((reprojection_tex.SampleLevel(sampler_nnc, uv - ((reproj_tap * 1.0f) * float2(float(nz_dx), float(nz_dy))), 0.0f).z == 0.0f) ? 1.0f : 0.0f);
            next_nz_count += 1.0f;
            nz_dx++;
            continue;
        }
    }
    float2 motion_px = motion * out_dims;
    float2 abs_motion = abs(motion_px);
    float4 reproj_gather_b = reprojection_tex.GatherBlue(sampler_lnc, uv);
    float edge_factor = (((3.0f * clamp(1.0f - ((2.5f * hist_std) / hist_mean), 0.0f, 1.0f)) * lum_diff) / (0.5f * max(9.9999997473787516355514526367188e-05f, std_half + blur_half))) * ((min(min(reproj_gather_b.x, reproj_gather_b.y), min(reproj_gather_b.z, reproj_gather_b.w)) == 1.0f) ? 1.0f : 0.0f);
    float motion_penalty = ((((((aux.x * 2.0f) - 0.5f) + clamp(1.0f - clamp_w, 0.0f, 1.0f)) + max(clip_counts.x, clip_counts.y)) + ((lum_ratio * lum_ratio) * max(0.0f, 1.0f - ((((-3.0f) * abs(mean_x - blur_half)) + (2.0f * abs((sum_c2.x / sum_w2) - mean_x))) / blur_half)))) + (sqrt(nz_sum / nz_count) * clamp(max(abs_motion.x, abs_motion.y), 0.0f, 1.0f))) + ((edge_factor * edge_factor) * edge_factor);
    bool use_clamp = !bypass;
    float blend_w;
    float blend_t;
    if (use_clamp)
    {
        float blend_t_adj;
        float aux_tier_adj;
        if (clamp(1.0f - min(min(reproj_gather.x, reproj_gather.y), min(reproj_gather.z, reproj_gather.w)), 0.0f, 1.0f) < 0.75f)
        {
            blend_t_adj = motion_penalty + 1.0f;
            aux_tier_adj = min(aux_tier, 0.5f);
        }
        else
        {
            blend_t_adj = motion_penalty;
            aux_tier_adj = aux_tier;
        }
        float blend_w2 = aux_tier_adj * clamp_w;
        float2 motion_scaled = abs(motion_px * 0.03125f.xx);
        blend_w = min(blend_w2, lerp(blend_w2, 1.0f, clamp(max(motion_scaled.x, motion_scaled.y), 0.0f, 1.0f)));
        blend_t = blend_t_adj;
    }
    else
    {
        blend_w = aux_tier;
        blend_t = motion_penalty;
    }
    float3 clamp_half = std * exp2((7.0f * clamp(1.0f - blend_t, 0.0f, 1.0f)) - 0.5f);
    float3 clamped_hist;
    if (use_clamp)
    {
        clamped_hist = clamp(reproj_hist, mean - clamp_half, mean + clamp_half);
    }
    else
    {
        clamped_hist = reproj_hist;
    }
    float blend_amt = min(4.0f, blend_w);
    float w1_total = sum_w2 + blend_amt;
    bool hist_ok;
    if (history_valid_b)
    {
        hist_ok = !(0u == frame_constants_frame_index);
    }
    else
    {
        hist_ok = false;
    }
    float3 blended;
    if (!hist_ok)
    {
        blended = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), sum_c2.xyz / max(9.9999997473787516355514526367188e-06f, sum_w2).xxx));
    }
    else
    {
        blended = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), lerp(sum_c2.xyz / sum_w2.xxx, clamped_hist, (blend_amt / max(9.9999997473787516355514526367188e-06f, w1_total)).xxx)));
    }
    FrexpStructType frexp_res;
    { int3 frexp_exp; frexp_res.mantissa = frexp(blended, frexp_exp); frexp_res.exponent = frexp_exp; }
    float3 history_color = blended + exp2(float3(frexp_res.exponent) - float3(8.0f, 8.0f, 7.0f));
    history_output_tex[gl_GlobalInvocationID.xy] = history_color;
    output_tex[gl_GlobalInvocationID.xy] = history_color;
    aux_output_tex[gl_GlobalInvocationID.xy] = float2(clamp(min(blend_t, 1.0f) + (2.0f * clamp(reprojection4.w * 1000.0f, 0.0f, 1.0f)), 0.0f, 2.0f) * 0.5f, w1_total / tier);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b303f36f0dbdd5f6) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _tgsr.cs.hlsl_cs_main_b303f36f0dbdd5f6.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
// dxc 1.9/1.10 lowers the native frexp() to invalid SPIR-V (OpConvertSToF on a non-int); same
// workaround as inc/curve.hlsl.
float3 incb_frexp(float3 x, out int3 exponent)
{
    // denormals are normalised first (x * 2^24) so the exponent/mantissa match GLSL Frexp exactly
    uint3 bits0 = asuint(x);
    bool3 denorm = ((bits0 & 0x7F800000u) == 0u) & ((bits0 & 0x7FFFFFFFu) != 0u);
    float3 xs = select(denorm, x * 16777216.0f, x);
    uint3 bits = asuint(xs);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu), int((bits.y >> 23u) & 0xFFu), int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127) - select(denorm, int3(24, 24, 24), int3(0, 0, 0));
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, xs, m);
}
#define frexp incb_frexp
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

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

static float ycbcr_alpha;
static float chroma_mid;

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
    uint2 reprojected_history_tex_lum_dims : packoffset(c0);
    uint2 reprojected_history_tex_chroma_dims : packoffset(c0.z);
    uint2 color_tex_dims : packoffset(c1);
    uint2 blurred_luminance_tex_dims : packoffset(c1.z);
    uint2 history_tex_dims : packoffset(c2);
    uint2 reprojection_tex_dims : packoffset(c2.z);
    uint2 aux_tex_dims : packoffset(c3);
    uint2 history_output_tex_dims : packoffset(c3.z);
    uint2 output_tex_dims : packoffset(c4);
    uint2 aux_output_tex_dims : packoffset(c4.z);
    uint bypass_neighborhood_clamping : packoffset(c5);
    float sharpen_amount : packoffset(c5.y);
    uint history_valid : packoffset(c5.z);
};

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> reprojected_history_tex_lum : register(t14, space0);
Texture2D<float4> reprojected_history_tex_chroma : register(t15, space0);
Texture2D<float4> color_tex : register(t16, space0);
Texture2D<float4> blurred_luminance_tex : register(t17, space0);
Texture2D<float4> reprojection_tex : register(t19, space0);
Texture2D<float4> aux_tex : register(t20, space0);
RWTexture2D<float3> history_output_tex : register(u21, space0);
RWTexture2D<float3> output_tex : register(u22, space0);
RWTexture2D<unorm float2> aux_output_tex : register(u23, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    float2 out_dims = float2(output_tex_dims);
    float2 center = float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx;
    float2 uv = center / out_dims;
    bool bypass = bypass_neighborhood_clamping != 0u;
    float tier = float(bypass ? 16 : 4);
    float4 reprojection4 = reprojection_tex.SampleLevel(sampler_nnc, uv, 0.0f);
    float2 motion = reprojection4.xy;
    bool history_valid_b = history_valid != 0u;
    bool history_available;
    if (history_valid_b)
    {
        history_available = !(0u == frame_constants_frame_index);
    }
    else
    {
        history_available = false;
    }
    float2 aux;
    if (history_available)
    {
        aux = aux_tex.SampleLevel(sampler_lnc, uv + motion, 0.0f).xy;
    }
    else
    {
        aux = 0.0f.xx;
    }
    float aux_tier = aux.y * tier;
    float3 reproj_hist = float3(reprojected_history_tex_lum.Load(int3(gl_GlobalInvocationID.xy, 0u)).x, reprojected_history_tex_chroma.Load(int3(gl_GlobalInvocationID.xy, 0u)).xy);
    float2 scale = float2(int2(color_tex_dims)) / out_dims;
    int2 current_px = int2(center * scale);
    float2 current_pos = ((float2(current_px) + 0.5f.xx) + (frame_constants_view_constants[0u].sample_offset_pixels * float2(1.0f, -1.0f))) / scale;
    float scale_x = scale.x;
    float sigma_wide = ((-10.0f) * scale_x) * min(4.0f * scale_x, 1.0f);
    float sigma_narrow = (-1.0f) * (scale_x * scale_x);
    float3 sum_sq;
    float sum_w1;
    float3 sum_c;
    float sum_w2;
    float4 sum_c2;
    int dy;
    sum_sq = 0.0f.xxx;
    sum_w1 = 0.0f;
    sum_c = 0.0f.xxx;
    sum_w2 = 0.0f;
    sum_c2 = 0.0f.xxxx;
    dy = -1;
    float3 next_sum_sq;
    float next_sum_w1;
    float3 next_sum_c;
    float next_sum_w2;
    float4 next_sum_c2;
    for (; dy <= 1; sum_sq = next_sum_sq, sum_w1 = next_sum_w1, sum_c = next_sum_c, sum_w2 = next_sum_w2, sum_c2 = next_sum_c2, dy++)
    {
        next_sum_w1 = sum_w1;
        next_sum_sq = sum_sq;
        next_sum_c = sum_c;
        next_sum_w2 = sum_w2;
        next_sum_c2 = sum_c2;
        for (int dx = -1; dx <= 1; )
        {
            float4 c = float4(mul(float3x3(float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f), float3(-0.114600002765655517578125f, -0.385399997234344482421875f, 0.5f), float3(0.5f, -0.4541999995708465576171875f, -0.0458000004291534423828125f)), color_tex.Load(int3(uint2(current_px + int2(dx, dy)), 0u)).xyz), ycbcr_alpha);
            float2 d = (current_pos + (float2(float(dx), float(dy)) / scale)) - center;
            float dist = dot(d, d);
            float w_narrow = exp2(dist * sigma_narrow);
            float w_wide = exp2(dist * sigma_wide);
            float3 c_xyz = c.xyz;
            next_sum_w1 += w_narrow;
            next_sum_sq += ((c_xyz * c_xyz) * w_narrow);
            next_sum_c += (c_xyz * w_narrow);
            next_sum_w2 += w_wide;
            next_sum_c2 += (c * w_wide);
            dx++;
            continue;
        }
    }
    float3 w1_vec = sum_w1.xxx;
    float3 mean = sum_c / w1_vec;
    float lum_sum;
    float lum_w;
    int lum_dy;
    lum_sum = 0.0f;
    lum_w = 0.0f;
    lum_dy = -1;
    float next_lum_sum;
    float next_lum_w;
    for (; lum_dy <= 1; lum_sum = next_lum_sum, lum_w = next_lum_w, lum_dy++)
    {
        next_lum_sum = lum_sum;
        next_lum_w = lum_w;
        for (int lum_dx = -1; lum_dx <= 1; )
        {
            float lum_wgt = exp(float((lum_dx * lum_dx) + (lum_dy * lum_dy)) * (-0.75f));
            next_lum_sum += (reprojected_history_tex_lum.SampleLevel(sampler_lnc, uv + ((float2(float(lum_dx), float(lum_dy)) * 2.0f) / float2(color_tex_dims)), 0.0f).x * lum_wgt);
            next_lum_w += lum_wgt;
            lum_dx++;
            continue;
        }
    }
    float lum_std = clamp(sqrt(lum_sum / lum_w), 0.0f, 1.0f);
    float lum_std_sq = lum_std * lum_std;
    float4 blur_sample = blurred_luminance_tex.SampleLevel(sampler_lnc, uv, 0.0f);
    float blur_lum = blur_sample.x;
    float blur_lum_sq = blur_lum * blur_lum;
    float blur_lerp_a;
    float std_lerp_a;
    if (WaveActiveMax(WaveGetLaneIndex()) >= 31u)
    {
        uint lane_xor16 = WaveGetLaneIndex() ^ 16u;
        blur_lerp_a = lerp(blur_lum_sq, WaveReadLaneAt(blur_lum_sq, lane_xor16), 0.5f);
        std_lerp_a = lerp(lum_std_sq, WaveReadLaneAt(lum_std_sq, lane_xor16), 0.5f);
    }
    else
    {
        blur_lerp_a = blur_lum_sq;
        std_lerp_a = lum_std_sq;
    }
    uint lane_xor2 = WaveGetLaneIndex() ^ 2u;
    float std_other = WaveReadLaneAt(std_lerp_a, lane_xor2);
    float std_half = lerp(std_lerp_a, std_other, 0.5f);
    float blur_other = WaveReadLaneAt(blur_lerp_a, lane_xor2);
    float blur_half = lerp(blur_lerp_a, blur_other, 0.5f);
    float3 std = sqrt(max(0.0f.xxx, (sum_sq / w1_vec) - (mean * mean)));
    float2 reproj_tap = 0.5f.xx / float2(reprojection_tex_dims);
    float3 std2 = std * 2.0f;
    float3 clamp_max = (mean + std2) + 0.001000000047497451305389404296875f.xxx;
    float3 clamp_min = (mean - std2) - 0.001000000047497451305389404296875f.xxx;
    float hist_sum_sq;
    float hist_sum;
    float3 hist_less;
    float3 hist_greater;
    float hist_count;
    int hist_dy;
    hist_sum_sq = 0.0f;
    hist_sum = 0.0f;
    hist_less = 0.0f.xxx;
    hist_greater = 0.0f.xxx;
    hist_count = 0.0f;
    hist_dy = -1;
    float next_hist_sum_sq;
    float next_hist_sum;
    float3 next_hist_less;
    float3 next_hist_greater;
    float next_hist_count;
    for (; hist_dy <= 1; hist_sum_sq = next_hist_sum_sq, hist_sum = next_hist_sum, hist_less = next_hist_less, hist_greater = next_hist_greater, hist_count = next_hist_count, hist_dy++)
    {
        next_hist_sum_sq = hist_sum_sq;
        next_hist_sum = hist_sum;
        next_hist_less = hist_less;
        next_hist_greater = hist_greater;
        next_hist_count = hist_count;
        for (int hist_dx = -1; hist_dx <= 1; )
        {
            uint2 hist_px = gl_GlobalInvocationID.xy + uint2(int2(hist_dx, hist_dy));
            float4 hist_lum4 = reprojected_history_tex_lum.Load(int3(hist_px, 0u));
            float hist_lum = hist_lum4.x;
            float3 hist_sample = float3(hist_lum, chroma_mid, reprojected_history_tex_chroma.Load(int3(hist_px, 0u)).y);
            next_hist_sum_sq += (hist_lum * hist_lum);
            next_hist_sum += hist_lum;
            next_hist_less += (select(bool3(hist_sample.x < clamp_min.x, hist_sample.y < clamp_min.y, hist_sample.z < clamp_min.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            next_hist_greater += (select(bool3(hist_sample.x > clamp_max.x, hist_sample.y > clamp_max.y, hist_sample.z > clamp_max.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            next_hist_count += 1.0f;
            hist_dx++;
            continue;
        }
    }
    float3 count_vec = hist_count.xxx;
    float hist_mean = hist_sum / hist_count;
    float hist_std = sqrt(max(0.0f, (hist_sum_sq / hist_count) - (hist_mean * hist_mean)));
    float4 reproj_gather = reprojection_tex.GatherAlpha(sampler_lnc, uv);
    float clamp_w = clamp((20.0f * max(0.0f, hist_std)) / max(9.9999999392252902907785028219223e-09f, hist_mean), 0.0f, 1.0f);
    float2 clip_counts = (hist_greater / count_vec).xz + (hist_less / count_vec).xz;
    float lum_diff = abs(std_half - blur_half);
    float lum_ratio = (2.0f * lum_diff) / max(9.9999997473787516355514526367188e-06f, blur_half + std_half);
    float mean_x = mean.x;
    float nz_sum;
    float nz_count;
    int nz_dy;
    nz_sum = 0.0f;
    nz_count = 0.0f;
    nz_dy = -1;
    float next_nz_sum;
    float next_nz_count;
    for (; nz_dy <= 1; nz_sum = next_nz_sum, nz_count = next_nz_count, nz_dy++)
    {
        next_nz_sum = nz_sum;
        next_nz_count = nz_count;
        for (int nz_dx = -1; nz_dx <= 1; )
        {
            next_nz_sum += ((reprojection_tex.SampleLevel(sampler_nnc, uv - ((reproj_tap * 1.0f) * float2(float(nz_dx), float(nz_dy))), 0.0f).z == 0.0f) ? 1.0f : 0.0f);
            next_nz_count += 1.0f;
            nz_dx++;
            continue;
        }
    }
    float2 motion_px = motion * out_dims;
    float2 abs_motion = abs(motion_px);
    float4 reproj_gather_b = reprojection_tex.GatherBlue(sampler_lnc, uv);
    float edge_factor = (((3.0f * clamp(1.0f - ((2.5f * hist_std) / hist_mean), 0.0f, 1.0f)) * lum_diff) / (0.5f * max(9.9999997473787516355514526367188e-05f, std_half + blur_half))) * ((min(min(reproj_gather_b.x, reproj_gather_b.y), min(reproj_gather_b.z, reproj_gather_b.w)) == 1.0f) ? 1.0f : 0.0f);
    float motion_penalty = ((((((aux.x * 2.0f) - 0.5f) + clamp(1.0f - clamp_w, 0.0f, 1.0f)) + max(clip_counts.x, clip_counts.y)) + ((lum_ratio * lum_ratio) * max(0.0f, 1.0f - ((((-3.0f) * abs(mean_x - blur_half)) + (2.0f * abs((sum_c2.x / sum_w2) - mean_x))) / blur_half)))) + (sqrt(nz_sum / nz_count) * clamp(max(abs_motion.x, abs_motion.y), 0.0f, 1.0f))) + ((edge_factor * edge_factor) * edge_factor);
    bool use_clamp = !bypass;
    float blend_w;
    float blend_t;
    if (use_clamp)
    {
        float blend_t_adj;
        float aux_tier_adj;
        if (clamp(1.0f - min(min(reproj_gather.x, reproj_gather.y), min(reproj_gather.z, reproj_gather.w)), 0.0f, 1.0f) < 0.75f)
        {
            blend_t_adj = motion_penalty + 1.0f;
            aux_tier_adj = min(aux_tier, 0.5f);
        }
        else
        {
            blend_t_adj = motion_penalty;
            aux_tier_adj = aux_tier;
        }
        float blend_w2 = aux_tier_adj * clamp_w;
        float2 motion_scaled = abs(motion_px * 0.03125f.xx);
        blend_w = min(blend_w2, lerp(blend_w2, 1.0f, clamp(max(motion_scaled.x, motion_scaled.y), 0.0f, 1.0f)));
        blend_t = blend_t_adj;
    }
    else
    {
        blend_w = aux_tier;
        blend_t = motion_penalty;
    }
    float3 clamp_half = std * exp2((7.0f * clamp(1.0f - blend_t, 0.0f, 1.0f)) - 0.5f);
    float3 clamped_hist;
    if (use_clamp)
    {
        clamped_hist = clamp(reproj_hist, mean - clamp_half, mean + clamp_half);
    }
    else
    {
        clamped_hist = reproj_hist;
    }
    float blend_amt = min(4.0f, blend_w);
    float w1_total = sum_w2 + blend_amt;
    bool hist_ok;
    if (history_valid_b)
    {
        hist_ok = !(0u == frame_constants_frame_index);
    }
    else
    {
        hist_ok = false;
    }
    float3 blended;
    if (!hist_ok)
    {
        blended = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), sum_c2.xyz / max(9.9999997473787516355514526367188e-06f, sum_w2).xxx));
    }
    else
    {
        blended = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), lerp(sum_c2.xyz / sum_w2.xxx, clamped_hist, (blend_amt / max(9.9999997473787516355514526367188e-06f, w1_total)).xxx)));
    }
    FrexpStructType frexp_res;
    { int3 frexp_exp; frexp_res.mantissa = frexp(blended, frexp_exp); frexp_res.exponent = frexp_exp; }
    float3 history_color = blended + exp2(float3(frexp_res.exponent) - float3(8.0f, 8.0f, 7.0f));
    history_output_tex[gl_GlobalInvocationID.xy] = history_color;
    output_tex[gl_GlobalInvocationID.xy] = history_color;
    aux_output_tex[gl_GlobalInvocationID.xy] = float2(clamp(min(blend_t, 1.0f) + (2.0f * clamp(reprojection4.w * 1000.0f, 0.0f, 1.0f)), 0.0f, 2.0f) * 0.5f, w1_total / tier);
}

[numthreads(8, 4, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f7efabb81dfe256b) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _tgsr.cs.hlsl_cs_main_f7efabb81dfe256b.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
// dxc 1.9/1.10 lowers the native frexp() to invalid SPIR-V (OpConvertSToF on a non-int); same
// workaround as inc/curve.hlsl.
float3 incb_frexp(float3 x, out int3 exponent)
{
    // denormals are normalised first (x * 2^24) so the exponent/mantissa match GLSL Frexp exactly
    uint3 bits0 = asuint(x);
    bool3 denorm = ((bits0 & 0x7F800000u) == 0u) & ((bits0 & 0x7FFFFFFFu) != 0u);
    float3 xs = select(denorm, x * 16777216.0f, x);
    uint3 bits = asuint(xs);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu), int((bits.y >> 23u) & 0xFFu), int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127) - select(denorm, int3(24, 24, 24), int3(0, 0, 0));
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, xs, m);
}
#define frexp incb_frexp
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

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

static float ycbcr_alpha;
static float chroma_mid;

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
    uint2 reprojected_history_tex_lum_dims : packoffset(c0);
    uint2 reprojected_history_tex_chroma_dims : packoffset(c0.z);
    uint2 color_tex_dims : packoffset(c1);
    uint2 blurred_luminance_tex_dims : packoffset(c1.z);
    uint2 history_tex_dims : packoffset(c2);
    uint2 reprojection_tex_dims : packoffset(c2.z);
    uint2 aux_tex_dims : packoffset(c3);
    uint2 history_output_tex_dims : packoffset(c3.z);
    uint2 output_tex_dims : packoffset(c4);
    uint2 aux_output_tex_dims : packoffset(c4.z);
    uint bypass_neighborhood_clamping : packoffset(c5);
    float sharpen_amount : packoffset(c5.y);
    uint history_valid : packoffset(c5.z);
};

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> reprojected_history_tex_lum : register(t14, space0);
Texture2D<float4> reprojected_history_tex_chroma : register(t15, space0);
Texture2D<float4> color_tex : register(t16, space0);
Texture2D<float4> blurred_luminance_tex : register(t17, space0);
Texture2D<float4> reprojection_tex : register(t19, space0);
Texture2D<float4> aux_tex : register(t20, space0);
RWTexture2D<float3> history_output_tex : register(u21, space0);
RWTexture2D<float3> output_tex : register(u22, space0);
RWTexture2D<unorm float2> aux_output_tex : register(u23, space0);

static uint3 gl_LocalInvocationID;
static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_LocalInvocationID : SV_GroupThreadID;
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

groupshared float tile_luminance[8][4];

void comp_main()
{
    float2 out_dims = float2(output_tex_dims);
    float2 center = float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx;
    float2 uv = center / out_dims;
    bool bypass = bypass_neighborhood_clamping != 0u;
    float tier = float(bypass ? 16 : 4);
    float4 reprojection4 = reprojection_tex.SampleLevel(sampler_nnc, uv, 0.0f);
    float2 motion = reprojection4.xy;
    bool history_valid_b = history_valid != 0u;
    bool history_available;
    if (history_valid_b)
    {
        history_available = !(0u == frame_constants_frame_index);
    }
    else
    {
        history_available = false;
    }
    float2 aux;
    if (history_available)
    {
        aux = aux_tex.SampleLevel(sampler_lnc, uv + motion, 0.0f).xy;
    }
    else
    {
        aux = 0.0f.xx;
    }
    float aux_tier = aux.y * tier;
    float3 reproj_hist = float3(reprojected_history_tex_lum.Load(int3(gl_GlobalInvocationID.xy, 0u)).x, reprojected_history_tex_chroma.Load(int3(gl_GlobalInvocationID.xy, 0u)).xy);
    float2 scale = float2(int2(color_tex_dims)) / out_dims;
    int2 current_px = int2(center * scale);
    float2 current_pos = ((float2(current_px) + 0.5f.xx) + (frame_constants_view_constants[0u].sample_offset_pixels * float2(1.0f, -1.0f))) / scale;
    float scale_x = scale.x;
    float sigma_wide = ((-10.0f) * scale_x) * min(4.0f * scale_x, 1.0f);
    float sigma_narrow = (-1.0f) * (scale_x * scale_x);
    float3 sum_sq;
    float sum_w1;
    float3 sum_c;
    float sum_w2;
    float4 sum_c2;
    int dy;
    sum_sq = 0.0f.xxx;
    sum_w1 = 0.0f;
    sum_c = 0.0f.xxx;
    sum_w2 = 0.0f;
    sum_c2 = 0.0f.xxxx;
    dy = -1;
    float3 next_sum_sq;
    float next_sum_w1;
    float3 next_sum_c;
    float next_sum_w2;
    float4 next_sum_c2;
    for (; dy <= 1; sum_sq = next_sum_sq, sum_w1 = next_sum_w1, sum_c = next_sum_c, sum_w2 = next_sum_w2, sum_c2 = next_sum_c2, dy++)
    {
        next_sum_w1 = sum_w1;
        next_sum_sq = sum_sq;
        next_sum_c = sum_c;
        next_sum_w2 = sum_w2;
        next_sum_c2 = sum_c2;
        for (int dx = -1; dx <= 1; )
        {
            float4 c = float4(mul(float3x3(float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f), float3(-0.114600002765655517578125f, -0.385399997234344482421875f, 0.5f), float3(0.5f, -0.4541999995708465576171875f, -0.0458000004291534423828125f)), color_tex.Load(int3(uint2(current_px + int2(dx, dy)), 0u)).xyz), ycbcr_alpha);
            float2 d = (current_pos + (float2(float(dx), float(dy)) / scale)) - center;
            float dist = dot(d, d);
            float w_narrow = exp2(dist * sigma_narrow);
            float w_wide = exp2(dist * sigma_wide);
            float3 c_xyz = c.xyz;
            next_sum_w1 += w_narrow;
            next_sum_sq += ((c_xyz * c_xyz) * w_narrow);
            next_sum_c += (c_xyz * w_narrow);
            next_sum_w2 += w_wide;
            next_sum_c2 += (c * w_wide);
            dx++;
            continue;
        }
    }
    float3 w1_vec = sum_w1.xxx;
    float3 mean = sum_c / w1_vec;
    float3 clamp_center = sum_c2.xyz;
    float lum_sum;
    float lum_w;
    int lum_dy;
    lum_sum = 0.0f;
    lum_w = 0.0f;
    lum_dy = -1;
    float next_lum_sum;
    float next_lum_w;
    for (; lum_dy <= 1; lum_sum = next_lum_sum, lum_w = next_lum_w, lum_dy++)
    {
        next_lum_sum = lum_sum;
        next_lum_w = lum_w;
        for (int lum_dx = -1; lum_dx <= 1; )
        {
            float lum_wgt = exp(float((lum_dx * lum_dx) + (lum_dy * lum_dy)) * (-0.75f));
            next_lum_sum += (reprojected_history_tex_lum.SampleLevel(sampler_lnc, uv + ((float2(float(lum_dx), float(lum_dy)) * 2.0f) / float2(color_tex_dims)), 0.0f).x * lum_wgt);
            next_lum_w += lum_wgt;
            lum_dx++;
            continue;
        }
    }
    float lum_std = clamp(sqrt(lum_sum / lum_w), 0.0f, 1.0f);
    float lum_std_sq = lum_std * lum_std;
    float4 blur_sample = blurred_luminance_tex.SampleLevel(sampler_lnc, uv, 0.0f);
    float blur_lum = blur_sample.x;
    float blur_lum_sq = blur_lum * blur_lum;
    float blur_lerp_a;
    float std_lerp_a;
    if (WaveActiveMax(WaveGetLaneIndex()) >= 31u)
    {
        uint lane_xor16 = WaveGetLaneIndex() ^ 16u;
        blur_lerp_a = lerp(blur_lum_sq, WaveReadLaneAt(blur_lum_sq, lane_xor16), 0.5f);
        std_lerp_a = lerp(lum_std_sq, WaveReadLaneAt(lum_std_sq, lane_xor16), 0.5f);
    }
    else
    {
        blur_lerp_a = blur_lum_sq;
        std_lerp_a = lum_std_sq;
    }
    uint lane_xor2 = WaveGetLaneIndex() ^ 2u;
    float std_other = WaveReadLaneAt(std_lerp_a, lane_xor2);
    float std_half = lerp(std_lerp_a, std_other, 0.5f);
    float blur_other = WaveReadLaneAt(blur_lerp_a, lane_xor2);
    float blur_half = lerp(blur_lerp_a, blur_other, 0.5f);
    float3 std = sqrt(max(0.0f.xxx, (sum_sq / w1_vec) - (mean * mean)));
    float2 reproj_tap = 0.5f.xx / float2(reprojection_tex_dims);
    float3 std2 = std * 2.0f;
    float3 clamp_max = (mean + std2) + 0.001000000047497451305389404296875f.xxx;
    float3 clamp_min = (mean - std2) - 0.001000000047497451305389404296875f.xxx;
    float hist_sum_sq;
    float hist_sum;
    float3 hist_less;
    float3 hist_greater;
    float hist_count;
    int hist_dy;
    hist_sum_sq = 0.0f;
    hist_sum = 0.0f;
    hist_less = 0.0f.xxx;
    hist_greater = 0.0f.xxx;
    hist_count = 0.0f;
    hist_dy = -1;
    float next_hist_sum_sq;
    float next_hist_sum;
    float3 next_hist_less;
    float3 next_hist_greater;
    float next_hist_count;
    for (; hist_dy <= 1; hist_sum_sq = next_hist_sum_sq, hist_sum = next_hist_sum, hist_less = next_hist_less, hist_greater = next_hist_greater, hist_count = next_hist_count, hist_dy++)
    {
        next_hist_sum_sq = hist_sum_sq;
        next_hist_sum = hist_sum;
        next_hist_less = hist_less;
        next_hist_greater = hist_greater;
        next_hist_count = hist_count;
        for (int hist_dx = -1; hist_dx <= 1; )
        {
            uint2 hist_px = gl_GlobalInvocationID.xy + uint2(int2(hist_dx, hist_dy));
            float4 hist_lum4 = reprojected_history_tex_lum.Load(int3(hist_px, 0u));
            float hist_lum = hist_lum4.x;
            float3 hist_sample = float3(hist_lum, chroma_mid, reprojected_history_tex_chroma.Load(int3(hist_px, 0u)).y);
            next_hist_sum_sq += (hist_lum * hist_lum);
            next_hist_sum += hist_lum;
            next_hist_less += (select(bool3(hist_sample.x < clamp_min.x, hist_sample.y < clamp_min.y, hist_sample.z < clamp_min.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            next_hist_greater += (select(bool3(hist_sample.x > clamp_max.x, hist_sample.y > clamp_max.y, hist_sample.z > clamp_max.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            next_hist_count += 1.0f;
            hist_dx++;
            continue;
        }
    }
    float3 count_vec = hist_count.xxx;
    float hist_mean = hist_sum / hist_count;
    float hist_std = sqrt(max(0.0f, (hist_sum_sq / hist_count) - (hist_mean * hist_mean)));
    float4 reproj_gather = reprojection_tex.GatherAlpha(sampler_lnc, uv);
    float clamp_w = clamp((20.0f * max(0.0f, hist_std)) / max(9.9999999392252902907785028219223e-09f, hist_mean), 0.0f, 1.0f);
    float2 clip_counts = (hist_greater / count_vec).xz + (hist_less / count_vec).xz;
    float lum_diff = abs(std_half - blur_half);
    float lum_ratio = (2.0f * lum_diff) / max(9.9999997473787516355514526367188e-06f, blur_half + std_half);
    float mean_x = mean.x;
    float nz_sum;
    float nz_count;
    int nz_dy;
    nz_sum = 0.0f;
    nz_count = 0.0f;
    nz_dy = -1;
    float next_nz_sum;
    float next_nz_count;
    for (; nz_dy <= 1; nz_sum = next_nz_sum, nz_count = next_nz_count, nz_dy++)
    {
        next_nz_sum = nz_sum;
        next_nz_count = nz_count;
        for (int nz_dx = -1; nz_dx <= 1; )
        {
            next_nz_sum += ((reprojection_tex.SampleLevel(sampler_nnc, uv - ((reproj_tap * 1.0f) * float2(float(nz_dx), float(nz_dy))), 0.0f).z == 0.0f) ? 1.0f : 0.0f);
            next_nz_count += 1.0f;
            nz_dx++;
            continue;
        }
    }
    float2 motion_px = motion * out_dims;
    float2 abs_motion = abs(motion_px);
    float4 reproj_gather_b = reprojection_tex.GatherBlue(sampler_lnc, uv);
    float edge_factor = (((3.0f * clamp(1.0f - ((2.5f * hist_std) / hist_mean), 0.0f, 1.0f)) * lum_diff) / (0.5f * max(9.9999997473787516355514526367188e-05f, std_half + blur_half))) * ((min(min(reproj_gather_b.x, reproj_gather_b.y), min(reproj_gather_b.z, reproj_gather_b.w)) == 1.0f) ? 1.0f : 0.0f);
    float motion_penalty = ((((((aux.x * 2.0f) - 0.5f) + clamp(1.0f - clamp_w, 0.0f, 1.0f)) + max(clip_counts.x, clip_counts.y)) + ((lum_ratio * lum_ratio) * max(0.0f, 1.0f - ((((-3.0f) * abs(mean_x - blur_half)) + (2.0f * abs((sum_c2.x / sum_w2) - mean_x))) / blur_half)))) + (sqrt(nz_sum / nz_count) * clamp(max(abs_motion.x, abs_motion.y), 0.0f, 1.0f))) + ((edge_factor * edge_factor) * edge_factor);
    bool use_clamp = !bypass;
    float blend_w;
    float blend_t;
    if (use_clamp)
    {
        float blend_t_adj;
        float aux_tier_adj;
        if (clamp(1.0f - min(min(reproj_gather.x, reproj_gather.y), min(reproj_gather.z, reproj_gather.w)), 0.0f, 1.0f) < 0.75f)
        {
            blend_t_adj = motion_penalty + 1.0f;
            aux_tier_adj = min(aux_tier, 0.5f);
        }
        else
        {
            blend_t_adj = motion_penalty;
            aux_tier_adj = aux_tier;
        }
        float blend_w2 = aux_tier_adj * clamp_w;
        float2 motion_scaled = abs(motion_px * 0.03125f.xx);
        blend_w = min(blend_w2, lerp(blend_w2, 1.0f, clamp(max(motion_scaled.x, motion_scaled.y), 0.0f, 1.0f)));
        blend_t = blend_t_adj;
    }
    else
    {
        blend_w = aux_tier;
        blend_t = motion_penalty;
    }
    float blend_t_clamped = min(blend_t, 1.0f);
    float3 clamp_half = std * exp2((7.0f * clamp(1.0f - blend_t, 0.0f, 1.0f)) - 0.5f);
    float3 clamped_hist;
    if (use_clamp)
    {
        clamped_hist = clamp(reproj_hist, mean - clamp_half, mean + clamp_half);
    }
    else
    {
        clamped_hist = reproj_hist;
    }
    float blend_amt = min(4.0f, blend_w);
    float w1_total = sum_w2 + blend_amt;
    float3 w1_vec_b = sum_w2.xxx;
    float3 blended = lerp(clamp_center / w1_vec_b, clamped_hist, (blend_amt / max(9.9999997473787516355514526367188e-06f, w1_total)).xxx);
    float blended_x = blended.x;
    bool hist_ok;
    if (history_valid_b)
    {
        hist_ok = !(0u == frame_constants_frame_index);
    }
    else
    {
        hist_ok = false;
    }
    float3 final_color;
    if (!hist_ok)
    {
        final_color = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), clamp_center / max(9.9999997473787516355514526367188e-06f, sum_w2).xxx));
    }
    else
    {
        final_color = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), blended));
    }
    FrexpStructType frexp_res;
    { int3 _fe; frexp_res.mantissa = frexp(final_color, _fe); frexp_res.exponent = _fe; }
    history_output_tex[gl_GlobalInvocationID.xy] = final_color + exp2(float3(frexp_res.exponent) - float3(8.0f, 8.0f, 7.0f));
    tile_luminance[gl_LocalInvocationID.x][gl_LocalInvocationID.y] = blended_x;
    GroupMemoryBarrierWithGroupSync();
    uint2 right = gl_LocalInvocationID.xy + uint2(1u, 0u);
    uint2 left = gl_LocalInvocationID.xy - uint2(1u, 0u);
    uint right_x = right.x;
    uint right_y = right.y;
    uint left_x = left.x;
    uint left_y = left.y;
    uint4 bounds_x = uint4(right_x, right_y, left_x, left_y);
    float x_accum;
    float x_weight;
    if (all(bool4(bounds_x.x < uint4(8u, 4u, 8u, 4u).x, bounds_x.y < uint4(8u, 4u, 8u, 4u).y, bounds_x.z < uint4(8u, 4u, 8u, 4u).z, bounds_x.w < uint4(8u, 4u, 8u, 4u).w)))
    {
        float w = max(0.0f, 1.0f - (6.0f * (abs(blended_x - tile_luminance[right_x][right_y]) + abs(blended_x - tile_luminance[left_x][left_y]))));
        float w2_sharpen = min(w, (sharpen_amount * w) * 1.25f);
        x_accum = w2_sharpen * (tile_luminance[right_x][right_y] + tile_luminance[left_x][left_y]);
        x_weight = w2_sharpen * 2.0f;
    }
    else
    {
        x_accum = 0.0f;
        x_weight = 0.0f;
    }
    uint2 up = gl_LocalInvocationID.xy + uint2(0u, 1u);
    uint2 down = gl_LocalInvocationID.xy - uint2(0u, 1u);
    uint up_x = up.x;
    uint up_y = up.y;
    uint down_x = down.x;
    uint down_y = down.y;
    uint4 bounds_y = uint4(up_x, up_y, down_x, down_y);
    float y_accum;
    float y_weight;
    if (all(bool4(bounds_y.x < uint4(8u, 4u, 8u, 4u).x, bounds_y.y < uint4(8u, 4u, 8u, 4u).y, bounds_y.z < uint4(8u, 4u, 8u, 4u).z, bounds_y.w < uint4(8u, 4u, 8u, 4u).w)))
    {
        float w_y = max(0.0f, 1.0f - (6.0f * (abs(blended_x - tile_luminance[up_x][up_y]) + abs(blended_x - tile_luminance[down_x][down_y]))));
        float w2_y = min(w_y, (sharpen_amount * w_y) * 1.25f);
        y_accum = (x_accum + (tile_luminance[up_x][up_y] * w2_y)) + (tile_luminance[down_x][down_y] * w2_y);
        y_weight = x_weight + (w2_y * 2.0f);
    }
    else
    {
        y_accum = x_accum;
        y_weight = x_weight;
    }
    float3 final_sharpened = final_color * max(0.0f, max(0.0f, (blended_x * (y_weight + 1.0f)) - y_accum) / max(9.9999997473787516355514526367188e-06f, blended_x));
    FrexpStructType frexp_res2;
    { int3 _fe; frexp_res2.mantissa = frexp(final_sharpened, _fe); frexp_res2.exponent = _fe; }
    output_tex[gl_GlobalInvocationID.xy] = final_sharpened + exp2(float3(frexp_res2.exponent) - float3(8.0f, 8.0f, 7.0f));
    aux_output_tex[gl_GlobalInvocationID.xy] = float2(clamp(blend_t_clamped + (2.0f * clamp(reprojection4.w * 1000.0f, 0.0f, 1.0f)), 0.0f, 2.0f) * 0.5f, w1_total / tier);
}

[numthreads(8, 4, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_LocalInvocationID = stage_input.gl_LocalInvocationID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
