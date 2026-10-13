// ==SKELETON==
// SOURCE PATH : shaders/tgsr_reproject.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins):
//   _tgsr_reproject.cs.hlsl_cs_main_0.bin
//     file_sha256  : a21bf39c0fe7b44520ce95d8552d79a1a53775ddc1cbf3e118ebcc2e8950ba57
//     pass/stage   : _tgsr_reproject (taa_tgsr) / cs
//     entry        : cs:cs_main
//     variant      : 0
//     local_size   : [8, 8, 1]
//     index_pass   : W52 (Inference)
//     names        : sampler_llc, ViewConstants, frame_constants, $Globals, history_tex, reprojection_tex, depth_tex, output_lum_tex, output_chrom_tex, cs_main
//
// ==RECONSTRUCTED== shaders/tgsr_reproject.cs.hlsl
// Evidence: _tgsr_reproject.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23, bound 322).
//   bindings: sampler_llc s3, $Globals b13 {history_valid, history_tex_dims,
//   reprojection_tex_dims, depth_tex_dims, output_lum_tex_dims,
//   output_chrom_tex_dims}, history_tex t14, reprojection_tex t15, depth_tex t16,
//   output_lum_tex u17 (R16f), output_chrom_tex u18 (Rg16f), frame_constants
//   set1/b0.  caps Shader + GroupNonUniform + GroupNonUniformShuffle +
//   RuntimeDescriptorArray + StorageImageExtendedFormats.
//   TGSR reprojection: 2x2 reprojection-motion range check with a
//   wave-shuffled OR (lanes ^2 and ^16) to flag wide motion, a 3x3 deepest-depth
//   search when flagged, bicubic (Catmull-Rom) history resample, then YCoCg
//   luminance/chroma split (gated on history_valid).  Status: O (structure from
//   disassembly).

#include "inc/samplers.hlsl"
#include "inc/color/ycbcr.hlsl"
#include "inc/uv.hlsl"
#include "inc/image.hlsl"
#include "inc/curve.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
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
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    float2 depth_dims = float2(depth_tex_dims);
    float2 lum_dims = float2(output_lum_tex_dims);
    uint2 sample_texel = uint2((float2(gl_GlobalInvocationID.xy) + 0.5f) * (depth_dims / lum_dims));

    // 2x2 reprojection motion range at the half-res texel.
    float2 m00 = reprojection_tex.Load(int3(sample_texel + uint2(0xFFFFFFFFu, 0xFFFFFFFFu), 0u)).xy;
    float2 m10 = reprojection_tex.Load(int3(sample_texel + uint2(1u, 0xFFFFFFFFu), 0u)).xy;
    float2 m01 = reprojection_tex.Load(int3(sample_texel + uint2(0xFFFFFFFFu, 1u), 0u)).xy;
    float2 m11 = reprojection_tex.Load(int3(sample_texel + uint2(1u, 1u), 0u)).xy;
    float2 mmin = min(min(min(m00, m10), m01), m11);
    float2 mmax = max(max(max(m00, m10), m01), m11);
    float2 mrange = mmax - mmin;
    float2 threshold = max(1.0f / depth_dims, abs(mmax + mmin)) * 0.1f;
    bool any_big = any(mrange > threshold);

    // Flag wide motion across the wave (lanes xor 2 and xor 16).
    bool wide = (int(any_big) | int(WaveReadLaneAt(any_big, WaveGetLaneIndex() ^ 2u))) != 0;
    if ((int(wide) | int(WaveReadLaneAt(wide, WaveGetLaneIndex() ^ 16u))) != 0)
    {
        // Wide motion: search for the deepest depth in a 3x3 neighborhood.
        float max_depth = depth_tex.Load(int3(sample_texel, 0u)).x;
        uint2 deepest_texel = sample_texel;
        for (int dy = -1; dy <= 1; ++dy)
        {
            for (int dx = -1; dx <= 1; ++dx)
            {
                uint2 t = sample_texel + uint2(dx, dy);
                float d = depth_tex.Load(int3(t, 0u)).x;
                bool deeper = d > max_depth;
                bool2 sel = bool2(deeper, deeper);
                max_depth = deeper ? d : max_depth;
                deepest_texel = uint2(sel.x ? t.x : deepest_texel.x, sel.y ? t.y : deepest_texel.y);
            }
        }
        sample_texel = deepest_texel;
    }

    // Reproject and bicubic-resample history at the (possibly shifted) texel.
    float2 reproj_uv = (((float2(int2(gl_GlobalInvocationID.xy)) + 0.5f) / lum_dims)
        + reprojection_tex.Load(int3(sample_texel, 0u)).xy) * lum_dims;
    float2 base = floor(reproj_uv - 0.5f);
    float2 f = reproj_uv - (base + 0.5f);

    float2 t = f * 0.5f;
    float2 w0 = f * (-0.5f + (f * (1.0f - t)));
    float2 w1 = 1.0f + (f * f * (-2.5f + (f * 1.5f)));
    float2 w2 = f * (0.5f + (f * (2.0f - (f * 1.5f))));
    float2 w3 = f * f * (-0.5f + t);
    float2 u0 = (base - 0.5f) / lum_dims;
    float2 u3 = (base + 2.5f) / lum_dims;
    float2 u1 = (base + 0.5f + (w2 / (w1 + w2))) / lum_dims;

    float4 hist_numerator =
          (float4(history_tex.SampleLevel(sampler_llc, float2(u1.x, u0.y), 0.0f).xyz, 1.0f) * (w1.x * w0.y))
        + (float4(history_tex.SampleLevel(sampler_llc, float2(u0.x, u1.y), 0.0f).xyz, 1.0f) * (w0.x * w1.y))
        + (float4(history_tex.SampleLevel(sampler_llc, u1, 0.0f).xyz, 1.0f) * (w1.x * w1.y))
        + (float4(history_tex.SampleLevel(sampler_llc, float2(u3.x, u1.y), 0.0f).xyz, 1.0f) * (w3.x * w1.y))
        + (float4(history_tex.SampleLevel(sampler_llc, float2(u1.x, u3.y), 0.0f).xyz, 1.0f) * (w1.x * w3.y));
    float hist_denom = (w1.x * w0.y) + (w0.x * w1.y) + (w1.x * w1.y) + (w3.x * w1.y) + (w1.x * w3.y);
    float3 history = (hist_numerator / hist_denom.xxxx).xyz;

    bool valid = (history_valid != 0u) && (frame_index != 0u);
    float3 ycocg = tgsr_rgb_to_ycocg(history * float(valid));
    output_lum_tex[gl_GlobalInvocationID.xy] = ycocg.x;
    output_chrom_tex[gl_GlobalInvocationID.xy] = ycocg.yz;
}
