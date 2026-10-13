// ==SKELETON==
// SOURCE PATH : shaders/tgsr_blur_lum.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins):
//   _tgsr_blur_lum.cs.hlsl_cs_main_0.bin
//     file_sha256  : 63190855c057e3b2f3aca909c688c48a521027450ae14f2f1818a3cbfa53c327
//     pass/stage   : _tgsr_blur_lum (taa_tgsr) / cs
//     entry        : cs:cs_main
//     variant      : 0
//     local_size   : [8, 8, 1]
//     index_pass   : W52 (Inference)
//     names        : sampler_lnc, ViewConstants, frame_constants, $Globals, color_tex, output_tex, cs_main
//
// ==RECONSTRUCTED== shaders/tgsr_blur_lum.cs.hlsl
// Evidence: _tgsr_blur_lum.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23, bound 128).
//   bindings: sampler_lnc s0, $Globals b13 {color_tex_dims, output_tex_dims},
//   color_tex t14, output_tex u15 (R16f luminance), frame_constants set1/b0.
//   3x3 gaussian blur of Rec.709 luminance (exp(-0.75 r^2)), jitter-unjittered
//   by sample_offset_clip, written as sqrt(lum/weight).  Status: O (structure
//   from disassembly).

#include "inc/common.hlsl"
#include "inc/uv.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/math.hlsl"
#include "tgsr_common.inc.hlsl"
#include "inc/color/ycbcr.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 color_tex_dims;
    uint2 output_tex_dims;
};

Texture2D<float4> color_tex : register(t14, space0);
[[vk::image_format("r8")]]
RWTexture2D<unorm float> output_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    float2 dims = float2(color_tex_dims);
    float2 uv = (float2(int2(gl_GlobalInvocationID.xy)) + 0.5f) / dims;

    float lum = 0.0f;
    float weight = 0.0f;
    for (int dy = -1; dy <= 1; ++dy)
    {
        for (int dx = -1; dx <= 1; ++dx)
        {
            float w = exp(float((dx * dx) + (dy * dy)) * (-0.75f));
            lum += (dot(color_tex.SampleLevel(sampler_lnc,
                (uv + ((float2(float(dx), float(dy)) * 2.0f) / dims))
                    - ((view_constants[0].sample_offset_clip * 0.5f) * float2(1.0f, -1.0f)),
                0.0f).xyz, float3(0.2126f, 0.7152f, 0.0722f)) * w);
            weight += w;
        }
    }
    output_tex[gl_GlobalInvocationID.xy] = sqrt(lum / weight).x;
}
