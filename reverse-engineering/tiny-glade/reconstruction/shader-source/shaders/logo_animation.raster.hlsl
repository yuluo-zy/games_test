// ==RECONSTRUCTED== shaders/logo_animation.raster.hlsl | F-E | bins: _logo_animation.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Intro logo: a vertex-fed quad; the ps composites the logo texture with an
// animation-texture reveal sweep driven by anim_t, fading out with fade_out.

#include "inc/common.hlsl"            // samplers (sampler_llr s1)
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float4x4 model_to_world;
    float anim_t;
    float fade_out;
    uint2 logo_dims;
    uint2 animation_dims;
    uint2 framebuffer_dims;
};

Texture2D<float4> logo : register(t14, space0);
Texture2D<float4> animation : register(t15, space0);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float2 Vertex_UV : TEXCOORD1;
};

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float2 pos = vin.Vertex_Position.xy;
    pos.x = vin.Vertex_Position.x * (float(framebuffer_dims.y) / float(framebuffer_dims.x));
    o.position = float4(pos * 0.45f, 1.0f, 1.0f);
    o.uv = vin.Vertex_UV;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    float t = pow(anim_t, 0.75f);
    float4 l = logo.SampleBias(sampler_llr, uv, -0.5f);
    float reveal = animation.SampleBias(sampler_llr, uv, -0.5f).x;
    float alpha = l.a * (1.0f - smoothstep(t, t + 0.05f, reveal)) * smoothstep(1.0f, 0.0f, fade_out);
    return float4(l.rgb * alpha, alpha);
}
