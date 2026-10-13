// ==SKELETON==
// SOURCE PATH : shaders/taa.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins):
//   _taa.cs.hlsl_cs_main_0.bin
//     file_sha256  : 16435ae25686fde954f267127dc50c62a2506d17b99561e8bd6601bc210c0927
//     pass/stage   : _taa (taa_tgsr) / cs
//     entry        : cs:cs_main
//     variant      : 0
//     local_size   : [8, 8, 1]
//     index_pass   : W52 (Inference)
//     names        : sampler_default, ViewConstants, frame_constants, $Globals, color_tex, history_tex, reprojection_tex, history_output_tex, output_tex, tile_luminance, cs_main, FrexpStructType
//
// ==RECONSTRUCTED== shaders/taa.cs.hlsl
// Evidence: _taa.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23, bound 497).
//   bindings: sampler_default s9, $Globals b13 {color_tex_dims, history_tex_dims,
//   reprojection_tex_dims, history_output_tex_dims, output_tex_dims,
//   temp_debug_tex_dims, bypass_neighborhood_clamping, sharpen_amount, history_valid},
//   color_tex t14, history_tex t15, reprojection_tex t16, history_output_tex u17,
//   output_tex u18, frame_constants set1/b0.  caps Shader + RuntimeDescriptorArray +
//   StorageImageExtendedFormats + StorageImageWriteWithoutFormat; LocalSize [8,8,1].
//   Standard TAA: reproject + bicubic (Catmull-Rom) history resample, YCoCg,
//   3x3 neighborhood variance clamp, optional sharpening via a tile-luminance
//   laplacian, frexp-style history quantization (curve.hlsl).  Status: O
//   (structure from disassembly; helper boundaries Inference).

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
[[vk::image_format("r11g11b10f")]]
RWTexture2D<float3> history_output_tex : register(u17, space0);
RWTexture2D<float3> output_tex : register(u18, space0);

groupshared float tile_luminance[8][8];

// Rec.709 <-> YCoCg matrices (inlined at the use sites, matching the compiled
// constant set).
#define YCOCG_FWD float3x3(float3(0.2126f, 0.7152f, 0.0722f), float3(-0.1146f, -0.3854f, 0.5f), float3(0.5f, -0.4542f, -0.0458f))
#define YCOCG_INV float3x3(float3(1.0f, 0.0f, 1.5748f), float3(1.0f, -0.1873f, -0.4681f), float3(1.0f, 1.8556f, 0.0f))

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_LocalInvocationID : SV_GroupThreadID,
             uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    float2 output_dims = float2(output_tex_dims);
    float2 pixel = float2(int2(gl_GlobalInvocationID.xy)) + 0.5f;

    // Reproject into history space.
    float2 reproj_uv = ((pixel / output_dims)
        + reprojection_tex.Load(int3(uint2((float2(gl_GlobalInvocationID.xy) + 0.5f)
            * float2(color_tex_dims / output_tex_dims)), 0u)).xy) * output_dims;
    float2 base = floor(reproj_uv - 0.5f);
    float2 f = reproj_uv - (base + 0.5f);

    // Catmull-Rom weights (5-tap separable).
    float2 t = f * 0.5f;
    float2 w0 = f * (-0.5f + (f * (1.0f - t)));
    float2 w1 = 1.0f + (f * f * (-2.5f + (f * 1.5f)));
    float2 w2 = f * (0.5f + (f * (2.0f - (f * 1.5f))));
    float2 w3 = f * f * (-0.5f + t);
    float2 u0 = (base - 0.5f) / output_dims;
    float2 u3 = (base + 2.5f) / output_dims;
    float2 u1 = (base + 0.5f + (w2 / (w1 + w2))) / output_dims;

    // 5-tap bicubic resample (history), renormalized by the summed weights.
    float4 hist_numerator =
          (float4(history_tex.SampleLevel(sampler_default, float2(u1.x, u0.y), 0.0f).xyz, 1.0f) * (w1.x * w0.y))
        + (float4(history_tex.SampleLevel(sampler_default, float2(u0.x, u1.y), 0.0f).xyz, 1.0f) * (w0.x * w1.y))
        + (float4(history_tex.SampleLevel(sampler_default, u1, 0.0f).xyz, 1.0f) * (w1.x * w1.y))
        + (float4(history_tex.SampleLevel(sampler_default, float2(u3.x, u1.y), 0.0f).xyz, 1.0f) * (w3.x * w1.y))
        + (float4(history_tex.SampleLevel(sampler_default, float2(u1.x, u3.y), 0.0f).xyz, 1.0f) * (w1.x * w3.y));
    float hist_denom = (w1.x * w0.y) + (w0.x * w1.y) + (w1.x * w1.y) + (w3.x * w1.y) + (w1.x * w3.y);
    float3 history = (hist_numerator / hist_denom.xxxx).xyz;
    float3 ycocg = mul(YCOCG_FWD, history);

    // Current-frame sample in YCoCg (for clamping + the weighted 3x3 gather).
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
    float3 color_mean = color4_sum.xyz / max(1e-5f, wc_sum).xxx;
    float3 stddev = sqrt(max(0.0f.xxx, (color2_sum / wg_sum.xxx) - (mean * mean)));
    float3 range = (stddev * lerp(0.5f, 1.0f, smoothstep(-0.1f, 0.3f, stddev.x / (mean.x + 1e-5f)))) * 1.0f;
    float3 lo = mean - range;
    float3 hi = mean + range;

    // Neighborhood clamping (bypassable), then the history blend factor.
    float blend;
    float3 clamped;
    if (bypass_neighborhood_clamping == 0u)
    {
        blend = 0.125f * lerp(0.2f, 1.0f, smoothstep(0.0f, 2.0f,
            min(abs(ycocg.x - lo.x), abs(ycocg.x - hi.x)) / max(max(ycocg.x, mean.x), 1e-5f)));
        clamped = clamp(ycocg, lo, hi);
    }
    else
    {
        blend = 0.125f;
        clamped = ycocg;
    }

    bool use_history = (history_valid != 0u) && (frame_index != 0u);
    float3 result;
    if (!use_history)
    {
        result = max(0.0f.xxx, mul(YCOCG_INV, color_mean));
    }
    else
    {
        result = max(0.0f.xxx, mul(YCOCG_INV, lerp(clamped, color_mean, blend.xxx)));
    }

    // History write: frexp-style quantization (curve.hlsl).
    history_output_tex[gl_GlobalInvocationID.xy] = curve(result);

    // Optional sharpening via a tile-luminance laplacian.
    float3 output;
    if (sharpen_amount > 0.0f)
    {
        float lum = dot(result, float3(0.2126f, 0.7152f, 0.0722f));
        tile_luminance[gl_LocalInvocationID.x][gl_LocalInvocationID.y] = lum;
        GroupMemoryBarrierWithGroupSync();

        uint2 rx = gl_LocalInvocationID.xy + uint2(1u, 0u);
        uint2 lx = gl_LocalInvocationID.xy - uint2(1u, 0u);
        uint4 xid = uint4(rx.x, rx.y, lx.x, lx.y);
        float edge_sum;
        float edge_weight;
        if (all(xid < uint4(8u, 8u, 8u, 8u)))
        {
            float w = max(0.0f, 1.0f - (6.0f * (abs(lum - tile_luminance[rx.x][rx.y])
                + abs(lum - tile_luminance[lx.x][lx.y]))));
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
            float w = max(0.0f, 1.0f - (6.0f * (abs(lum - tile_luminance[dy.x][dy.y])
                + abs(lum - tile_luminance[uy.x][uy.y]))));
            float sw = min(w, (sharpen_amount * w) * 1.25f);
            edge_sum += (tile_luminance[dy.x][dy.y] * sw) + (tile_luminance[uy.x][uy.y] * sw);
            edge_weight += (sw * 2.0f);
        }
        output = result * max(0.0f, max(0.0f, (lum * (edge_weight + 1.0f)) - edge_sum)
            / max(1e-5f, lum));
    }
    else
    {
        output = result;
    }
    output_tex[gl_GlobalInvocationID.xy] = output;
}
