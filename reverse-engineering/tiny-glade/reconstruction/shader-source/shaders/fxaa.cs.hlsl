// ==RECONSTRUCTED== shaders/fxaa.cs.hlsl | F-E | bins: _fxaa.cs.hlsl_cs_main_0.bin | entry cs:cs_main | local_size [8,8,1]
// FXAA entry: computes per-pixel uv, samples the center, calls the FXAA core
// (inc/fxaa.inc.hlsl), writes to output_tex.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/uv.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "fxaa.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 input_tex_dims;
    uint2 output_tex_dims;
};

Texture2D<float4> input_tex : register(t14, space0);
RWTexture2D<float3> output_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID)
{
    float2 dims = float2(input_tex_dims);
    float2 uv = (float2(int2(dispatch_id.xy)) + 0.5f) / dims;
    float2 rcp = 1.0f / dims;

    float4 center = input_tex.SampleLevel(sampler_lnc, uv, 0.0f);
    float4 result = fxaa(input_tex, sampler_lnc, uv, rcp, center);

    output_tex[dispatch_id.xy] = result.xyz;
}
