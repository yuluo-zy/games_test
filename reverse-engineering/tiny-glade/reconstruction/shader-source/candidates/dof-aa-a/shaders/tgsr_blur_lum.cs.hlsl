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
//     names        : sampler_lnc, ViewConstants, frame_constants, $Globals, color_tex, output_tex, cs_main
// ==RECONSTRUCTED== shaders/tgsr_blur_lum.cs.hlsl | dof-aa-a | status: O (structure from SPIR-V)
//   TGSR blurred-luminance pass: 3x3 Gaussian (exp(-(i^2+j^2)*0.75)) over luminance, unjittered
//   by half the current clip offset, written as sqrt(weighted / weight).

#include "inc/common.hlsl"
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
[[vk::ext_capability(49)]] RWTexture2D<unorm float> output_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID)
{
    float2 color_dims = float2(color_tex_dims);
    float2 uv = (float2(int2(dispatch_id.xy)) + 0.5f) / color_dims;

    float weight_sum = 0.0f;
    float lum_sum = 0.0f;
    [loop]
    for (int dy = -1; dy <= 1; ++dy)
    {
        [loop]
        for (int dx = -1; dx <= 1; ++dx)
        {
            float w = exp(float((dx * dx) + (dy * dy)) * (-0.75f));
            float2 sample_uv = (uv + ((float2(float(dx), float(dy)) * 2.0f) / color_dims))
                - ((view_constants[0].sample_offset_clip * 0.5f) * float2(1.0f, -1.0f));
            lum_sum += dot(color_tex.SampleLevel(sampler_lnc, sample_uv, 0.0f).xyz,
                float3(0.2126f, 0.7152f, 0.0722f)) * w;
            weight_sum += w;
        }
    }

    output_tex[dispatch_id.xy] = sqrt(lum_sum / weight_sum).x;
}
