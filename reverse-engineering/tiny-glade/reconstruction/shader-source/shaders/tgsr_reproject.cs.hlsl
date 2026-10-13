// ==RECONSTRUCTED== shaders/tgsr_reproject.cs.hlsl | A (dof-aa review, 2026-10-13): A winner (w1-includes-w2 bicubic form matching taa/tgsr family; leaner include closure; rgb_to_ycbcr from inc/color/ycbcr.hlsl).
//   TGSR reprojection: fetches the 2x2 motion quad, on disocclusion re-searches the max-depth
//   position, filters the history with a 5-tap bicubic cross, and writes YCoCg luma/chroma.

#include "inc/samplers.hlsl"
#include "inc/color/ycbcr.hlsl"
#include "inc/uv.hlsl"
#include "inc/image.hlsl"
#include "inc/curve.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/history_valid.hlsl"
#include "tgsr_common.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint history_valid;
    uint2 history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 depth_tex_dims;
    uint2 output_lum_tex_dims;
    uint2 output_chrom_tex_dims;
};

Texture2D<float4> history_tex : register(t14, space0);
Texture2D<float4> reprojection_tex : register(t15, space0);
Texture2D<float4> depth_tex : register(t16, space0);
RWTexture2D<half> output_lum_tex : register(u17, space0);
RWTexture2D<half2> output_chrom_tex : register(u18, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID)
{
    float2 depth_dims = float2(depth_tex_dims);
    float2 out_dims = float2(output_lum_tex_dims);
    uint2 quad_px = uint2((float2(dispatch_id.xy) + 0.5f) * (depth_dims / out_dims));

    float2 r00 = reprojection_tex.Load(int3(quad_px + uint2(0xFFFFFFFFu, 0xFFFFFFFFu), 0)).xy;
    float2 r10 = reprojection_tex.Load(int3(quad_px + uint2(1u, 0xFFFFFFFFu), 0)).xy;
    float2 r01 = reprojection_tex.Load(int3(quad_px + uint2(0xFFFFFFFFu, 1u), 0)).xy;
    float2 r11 = reprojection_tex.Load(int3(quad_px + uint2(1u, 1u), 0)).xy;
    float2 r_min = min(min(r00, r10), min(r01, r11));
    float2 r_max = max(max(r00, r10), max(r01, r11));
    float2 r_delta = r_max - r_min;
    float2 r_threshold = max(1.0f / depth_dims, abs(r_max + r_min)) * 0.1f;
    bool inconsistent = any(r_delta > r_threshold);
    bool quad_inconsistent = inconsistent
        | WaveReadLaneAt(inconsistent, WaveGetLaneIndex() ^ 2u);

    uint2 search_px;
    if ((quad_inconsistent | WaveReadLaneAt(quad_inconsistent, WaveGetLaneIndex() ^ 16u)) != 0)
    {
        float max_depth = depth_tex.Load(int3(quad_px, 0)).x;
        uint2 max_depth_px = quad_px;
        [loop]
        for (int dy = -1; dy <= 1; ++dy)
        {
            [loop]
            for (int dx = -1; dx <= 1; ++dx)
            {
                uint2 sample_px = quad_px + uint2(dx, dy);
                float d = depth_tex.Load(int3(sample_px, 0)).x;
                if (d > max_depth)
                {
                    max_depth = d;
                    max_depth_px = sample_px;
                }
            }
        }
        search_px = max_depth_px;
    }
    else
    {
        search_px = quad_px;
    }

    float2 reproj_px = (((float2(int2(dispatch_id.xy)) + 0.5f) / out_dims)
        + reprojection_tex.Load(int3(search_px, 0)).xy) * out_dims;

    // 5-tap bicubic-cross history filter.
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
        (history_tex.SampleLevel(sampler_llc, float2(center_uv.x, left_uv.y), 0.0f).xyz * w1.x * w0.y)
        + (history_tex.SampleLevel(sampler_llc, float2(left_uv.x, center_uv.y), 0.0f).xyz * w0.x * w1.y)
        + (history_tex.SampleLevel(sampler_llc, center_uv, 0.0f).xyz * w1.x * w1.y)
        + (history_tex.SampleLevel(sampler_llc, float2(right_uv.x, center_uv.y), 0.0f).xyz * w3.x * w1.y)
        + (history_tex.SampleLevel(sampler_llc, float2(center_uv.x, right_uv.y), 0.0f).xyz * w1.x * w3.y)
    ) / ((((w1.x * w0.y) + (w0.x * w1.y)) + (w1.x * w1.y)) + ((w3.x * w1.y) + (w1.x * w3.y)));

    bool history_available = (history_valid != 0u) && (frame_index != 0u);
    float3 ycbcr = rgb_to_ycbcr(hist * float(history_available));

    output_lum_tex[dispatch_id.xy] = ycbcr.x;
    output_chrom_tex[dispatch_id.xy] = ycbcr.yz;
}
