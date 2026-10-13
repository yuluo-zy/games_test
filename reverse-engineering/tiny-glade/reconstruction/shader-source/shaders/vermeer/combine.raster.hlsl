// ==RECONSTRUCTED== shaders/vermeer/combine.raster.hlsl | evidence: _vermeer_combine.raster.hlsl_{b903,f0ad}.bin — fullscreen vs + ps: reveal sweep over progress (lerp(1,alpha,smoothstep(.28,.49,progress)), color*min(2,reveal/alpha)) + vignette (exp(remap*dist^4)) in the non-photomode branch; photomode passes output_tex through; caps Shader+RuntimeDescriptorArray | status: O (reveal + vignette from inlined SPIR-V); I (ps input signature)
//
// Vermeer thumbnail chain: final combine. In non-photomode the accumulated
// stroke alpha drives a progressive reveal and a soft vignette fades in.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/tracked_buffer.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/perlin.hlsl"
#include "inc/fbm_noise.hlsl"
#include "inc/uv.hlsl"
#include "inc/color/srgb.hlsl"
#include "vermeer/stroke.inc.hlsl"
#include "fullscreen_triangle_vert.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint max_strokes;
    uint2 target_tex_dims;
    uint2 output_tex_dims;
    uint2 framebuffer_dims;
    float progress_t;
    uint photomode;
};

Texture2D<float4> output_tex : register(t15, space0);

float4 ps_main(float4 position : SV_Position) : SV_Target0
{
    float2 uv = (float2(int2(position.xy)) + 0.5f.xx) / float2(framebuffer_dims);
    float4 c = output_tex.SampleLevel(sampler_lnc, uv, 0.0f);

    float4 out_color;
    if (photomode == 0u)
    {
        float reveal = lerp(1.0f, c.w, smoothstep(0.2800000011920928955078125f, 0.4899999797344207763671875f, progress_t));
        float3 color = c.xyz * min(2.0f, reveal / c.w);
        out_color = float4(color, reveal);
    }
    else
    {
        out_color = c;
    }

    if (photomode == 0u)
    {
        float dist = length(uv - 0.5f.xx);
        float dist2 = dist * dist;
        out_color.xyz *= exp((clamp(1.0f - progress_t, 0.0f, 1.0f) * (-1.39999997615814208984375f)) * (dist2 * dist2));
    }

    return out_color;
}
