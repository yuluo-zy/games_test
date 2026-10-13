// ==RECONSTRUCTED== shaders/taa.cs.hlsl | A (dof-aa review, 2026-10-13): A winner (target Catmull-Rom w1 = (1 + f²(-2.5+1.5f)) + w2, center_uv = (base_c + w2/w1); B uses standard w1).
//   Temporal AA: reprojects + bicubic-cross filters the history, neighborhood-clamps in YCoCg,
//   blends, curve-quantizes the new history, and applies a tile-luminance sharpen.

#include "inc/common.hlsl"
#include "inc/uv.hlsl"
#include "inc/color/ycbcr.hlsl"
#include "inc/image.hlsl"
#include "inc/curve.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/unjitter_taa.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/history_valid.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 color_tex_dims;
    uint2 history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 history_output_tex_dims;
    uint2 output_tex_dims;
    uint2 temp_debug_tex_dims;
    uint bypass_neighborhood_clamping;
    float sharpen_amount;
    uint history_valid;
};

Texture2D<float4> color_tex : register(t14, space0);
Texture2D<float4> history_tex : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
[[vk::ext_capability(49)]] RWTexture2D<float3> history_output_tex : register(u17, space0);
[[vk::ext_capability(49)]] RWTexture2D<float3> output_tex : register(u18, space0);

groupshared float tile_luminance[8][8];

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint3 local_id : SV_GroupThreadID)
{
    float2 out_dims = float2(output_tex_dims);
    float2 center = float2(int2(dispatch_id.xy)) + 0.5f;
    float2 reproj_px = ((center / out_dims)
        + reprojection_tex.Load(int3(uint2((float2(dispatch_id.xy) + 0.5f)
            * float2(color_tex_dims / output_tex_dims)), 0)).xy) * out_dims;

    // 5-tap bicubic-cross history filter (Catmull-Rom-style weights).
    float2 base = floor(reproj_px - 0.5f);
    float2 base_c = base + 0.5f;
    float2 frac = reproj_px - base_c;
    float2 h2 = frac * 0.5f;
    float2 w0 = frac * ((-0.5f) + (frac * (1.0f - h2)));
    float2 frac2 = frac * frac;
    float2 frac15 = frac * 1.5f;
    float2 w2 = frac * (0.5f + (frac * (2.0f - frac15)));
    float2 w3 = frac2 * ((-0.5f) + h2);
    float2 w1 = (1.0f + (frac2 * ((-2.5f) + frac15))) + w2;
    float2 left_uv = (base + (-0.5f)) / out_dims;
    float2 right_uv = (base + 2.5f) / out_dims;
    float2 center_uv = (base_c + (w2 / w1)) / out_dims;

    float3 hist = (
        (history_tex.SampleLevel(sampler_default, float2(center_uv.x, left_uv.y), 0.0f).xyz * w1.x * w0.y)
        + (history_tex.SampleLevel(sampler_default, float2(left_uv.x, center_uv.y), 0.0f).xyz * w0.x * w1.y)
        + (history_tex.SampleLevel(sampler_default, center_uv, 0.0f).xyz * w1.x * w1.y)
        + (history_tex.SampleLevel(sampler_default, float2(right_uv.x, center_uv.y), 0.0f).xyz * w3.x * w1.y)
        + (history_tex.SampleLevel(sampler_default, float2(center_uv.x, right_uv.y), 0.0f).xyz * w1.x * w3.y)
    ) / ((((w1.x * w0.y) + (w0.x * w1.y)) + (w1.x * w1.y)) + ((w3.x * w1.y) + (w1.x * w3.y)));

    float3 hist_ycbcr = rgb_to_ycbcr(hist);

    float2 scale = float2(int2(color_tex_dims)) / out_dims;
    int2 current_px = int2(center * scale);
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
            float2 d = (current_pos + (float2(float(dx), float(dy)) / scale)) - center;
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

    float3 mean = sum_c / max(9.9999997473787516e-06f, sum_w1);
    float3 clamp_center = sum_c2.xyz / max(9.9999997473787516e-06f, sum_w2);
    float3 std = sqrt(max(float3(0.0f, 0.0f, 0.0f), (sum_sq / sum_w1) - (mean * mean)));
    float3 clamp_range = (std * lerp(0.5f, 1.0f,
        smoothstep(-0.1f, 0.3f, std.x / (mean.x + 9.9999997473787516e-06f)))) * 1.0f;
    float3 clamp_min = mean - clamp_range;
    float3 clamp_max = mean + clamp_range;

    float blend;
    float3 clamped_ycbcr;
    if (bypass_neighborhood_clamping != 0u)
    {
        blend = 0.125f;
        clamped_ycbcr = hist_ycbcr;
    }
    else
    {
        blend = 0.125f * lerp(0.2f, 1.0f, smoothstep(0.0f, 2.0f,
            min(abs(hist_ycbcr.x - clamp_min.x), abs(hist_ycbcr.x - clamp_max.x))
            / max(max(hist_ycbcr.x, mean.x), 9.9999997473787516e-06f)));
        clamped_ycbcr = clamp(hist_ycbcr, clamp_min, clamp_max);
    }

    bool history_available = (history_valid != 0u) && (frame_index != 0u);
    float3 final_color;
    if (!history_available)
    {
        final_color = max(float3(0.0f, 0.0f, 0.0f), ycbcr_to_rgb(clamp_center));
    }
    else
    {
        final_color = max(float3(0.0f, 0.0f, 0.0f), ycbcr_to_rgb(lerp(clamped_ycbcr, clamp_center, blend)));
    }

    history_output_tex[dispatch_id.xy] = curve(final_color);

    float3 out_color;
    if (sharpen_amount > 0.0f)
    {
        float lum = dot(final_color, float3(0.2126f, 0.7152f, 0.0722f));
        tile_luminance[local_id.x][local_id.y] = lum;
        GroupMemoryBarrierWithGroupSync();

        uint2 right = local_id.xy + uint2(1u, 0u);
        uint2 left = local_id.xy - uint2(1u, 0u);
        float x_accum = 0.0f;
        float x_weight = 0.0f;
        if (all(bool4(right.x < 8u, right.y < 8u, left.x < 8u, left.y < 8u)))
        {
            float w = max(0.0f, 1.0f - (6.0f
                * (abs(lum - tile_luminance[right.x][right.y]) + abs(lum - tile_luminance[left.x][left.y]))));
            float w2 = min(w, (sharpen_amount * w) * 1.25f);
            x_accum = w2 * (tile_luminance[right.x][right.y] + tile_luminance[left.x][left.y]);
            x_weight = w2 * 2.0f;
        }

        uint2 up = local_id.xy + uint2(0u, 1u);
        uint2 down = local_id.xy - uint2(0u, 1u);
        float y_accum = 0.0f;
        float y_weight = 0.0f;
        if (all(bool4(up.x < 8u, up.y < 8u, down.x < 8u, down.y < 8u)))
        {
            float w = max(0.0f, 1.0f - (6.0f
                * (abs(lum - tile_luminance[up.x][up.y]) + abs(lum - tile_luminance[down.x][down.y]))));
            float w2 = min(w, (sharpen_amount * w) * 1.25f);
            y_accum = (x_accum + (tile_luminance[up.x][up.y] * w2)) + (tile_luminance[down.x][down.y] * w2);
            y_weight = x_weight + (w2 * 2.0f);
        }
        else
        {
            y_accum = x_accum;
            y_weight = x_weight;
        }

        out_color = final_color * max(0.0f, max(0.0f,
            (lum * (y_weight + 1.0f)) - y_accum) / max(9.9999997473787516e-06f, lum));
    }
    else
    {
        out_color = final_color;
    }

    output_tex[dispatch_id.xy] = out_color;
}
