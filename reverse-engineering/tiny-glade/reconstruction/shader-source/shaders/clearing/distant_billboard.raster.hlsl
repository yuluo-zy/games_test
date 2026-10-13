// ==RECONSTRUCTED== shaders/clearing/distant_billboard.raster.hlsl
// Evidence: _clearing_distant_billboard.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings: sampler_default s9, distant_hills_texture t14, $Globals b13
//   (current_view_index/distant_hills_texture_dims/model_to_world), frame_constants set1/b0.
//   vs_main scales the distant hill quad (two slopes around y = 12) into world space via
//   model_to_world; ps_main samples the hills texture with a mip bias, discards alpha > 0.99,
//   and tints by sky color / time of day.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/color_library.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/math.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"

// $Globals members (implicit cbuffer at set0/b13).
uint2 distant_hills_texture_dims;
float4x4 model_to_world;

Texture2D<float4> distant_hills_texture : register(t14, space0);

struct VS_IN
{
    [[vk::location(0)]] float3 vertex_position: Vertex_Position;
    [[vk::location(1)]] float2 vertex_uv: Vertex_UV;
};

struct VS_OUT
{
    float2 C0 : TEXCOORD0;
    float4 position : SV_Position;
};

VS_OUT vs_main(VS_IN i)
{
    VS_OUT o;
    float3 pos = i.vertex_position;
    float2 uv = i.vertex_uv * 0.71428573131561279296875f;
    float3 p;
    if (pos.x > 0.0f)
    {
        float3 pp = pos;
        pp.y = (pos.y * 0.675f) - 74.0f;
        float2 pp2 = pp.xz * 1.5f;
        p = float3(pp2.x, pp.y, pp2.y);
    }
    else
    {
        float3 pp = pos;
        pp.y = (pos.y * 0.675f) - 80.0f;
        float2 pp2 = pp.xz * 1.45f;
        p = float3(pp2.x, pp.y, pp2.y);
    }
    o.position = mul(view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(p.x, p.y + 12.0f, p.z, 1.0f)));
    o.C0 = float2(frac((uv.x * 19.0f) + 0.7f), 1.0f - uv.y);
    return o;
}

struct PS_IN
{
    float2 C0 : TEXCOORD0;
};

struct PS_OUT
{
    float4 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    float4 tex = distant_hills_texture.SampleBias(sampler_default, i.C0, global_mip_bias);
    if (tex.x > 0.99f)
    {
        discard;
    }
    float3 s = sqrt(tex.xyz + 0.1f);
    float3 tint = (lerp(sky_high_color.xyz, sky_low_color.xyz * 0.5f, s.x.xxx) * 0.7f)
        * clamp(1.1f - ((smoothstep(0.483f, 0.505f, time_of_day) - smoothstep(0.84f, 0.91f, time_of_day)) * 0.75f), 0.0f, 1.0f);
    o.target = float4(s * tint, 1.0f);
    return o;
}
