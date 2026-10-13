// ==RECONSTRUCTED== shaders/logo_branches_animation.raster.hlsl | F-E | bins: _logo_branches_animation.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Logo "branch" intro: each of 4 branches grows from its start position with a
// per-branch angle and timing; the ps picks the branch texture by branch index.

#include "inc/common.hlsl"            // samplers (sampler_llr s1)
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float4x4 model_to_world;
    float anim_t;
    float fade_out;
    uint2 branch_t_dims;
    uint2 branch_y_dims;
    uint2 branch_g_dims;
    uint2 branch_e_dims;
    uint2 framebuffer_dims;
};

Texture2D<float4> branch_t : register(t14, space0);
Texture2D<float4> branch_y : register(t15, space0);
Texture2D<float4> branch_g : register(t16, space0);
Texture2D<float4> branch_e : register(t17, space0);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 start_pos : TEXCOORD1;
    float2 Vertex_UV : TEXCOORD2;
    int branch_idx : TEXCOORD3;
};

struct VsOut
{
    float2 uv : TEXCOORD0;
    nointerpolation int branch_idx : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float angle;
    float len;
    if (vin.branch_idx == 0) { angle = radians(-10.0f); len = 0.2f; }
    else if (vin.branch_idx == 1) { angle = radians(-30.0f); len = 0.1f; }
    else if (vin.branch_idx == 2) { angle = radians(-30.0f); len = 0.18f; }
    else if (vin.branch_idx == 3) { angle = radians(20.0f); len = 0.22f; }
    else { angle = 0.0f; len = 0.0f; }

    float t = clamp(1.0f - (anim_t * 1.22f - len), 0.0f, 1.0f);
    float grow = 1.0f - ((t * t * t) - t * sin(t * 3.1415927f));

    float3 offset = lerp(float3(0.0f, 0.0f, 0.0f), vin.Vertex_Position - vin.start_pos, grow);
    float rot = lerp(angle, 0.0f, grow);
    float s = sin(rot);
    float c = cos(rot);
    float2x2 m = float2x2(c, s, -s, c);
    float3 world = float3(mul(m, offset.xy), offset.z) + vin.start_pos;

    float2 pos = world.xy;
    pos.x = world.x * (float(framebuffer_dims.y) / float(framebuffer_dims.x));
    o.position = float4(pos * 0.45f, 1.0f, 1.0f);
    o.uv = vin.Vertex_UV;
    o.branch_idx = vin.branch_idx;
    return o;
}

float4 ps_main(VsOut vin) : SV_Target0
{
    float4 s;
    if (vin.branch_idx == 0) s = branch_t.SampleBias(sampler_llr, vin.uv, -0.5f);
    else if (vin.branch_idx == 1) s = branch_y.SampleBias(sampler_llr, vin.uv, -0.5f);
    else if (vin.branch_idx == 2) s = branch_g.SampleBias(sampler_llr, vin.uv, -0.5f);
    else if (vin.branch_idx == 3) s = branch_e.SampleBias(sampler_llr, vin.uv, -0.5f);
    else s = float4(0.0f, 0.0f, 0.0f, 0.0f);

    float alpha = s.w * smoothstep(1.0f, 0.0f, fade_out);
    return float4(s.rgb * alpha, alpha);
}
