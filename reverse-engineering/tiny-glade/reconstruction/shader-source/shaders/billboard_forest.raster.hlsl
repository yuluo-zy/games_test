// ==RECONSTRUCTED== shaders/billboard_forest.raster.hlsl
// Evidence: _billboard_forest.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings (vs): $Globals b13 (current_view_index/framebuffer_dims/cloud1..4_dims),
//   frame_constants set1/b0. (ps): sampler_lnc s0, cloud1..4 t14..17, color_library b18,
//   frame_constants set1/b0. vs_main: procedural forest billboard quads (golden-ratio angle,
//   distance ring, 4 cloud-size variants, NaN cull past 500). ps_main: 5-tap cloud-cluster
//   normal from the per-type cloud texture, theme-colored albedo, packed uint2 target.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/simplex_noise.hlsl"

// color_library.hlsl is guard-skipped; flat-member cbuffer at set0/b18.
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
cbuffer color_library : register(b18, space0)
{
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/sh.hlsl"
// NOTE: inc/lighting.hlsl is omitted: it references color_library.indirect_light_tint
// (nested-struct form) which conflicts with this shader's flat-member `color_library`
// cbuffer (embedded target is flat, 37 members). Its functions are unused here; the
// embedded lighting.hlsl OpString is recorded only in the original build (residual).

// $Globals members (implicit cbuffer at set0/b13).
uint2 framebuffer_dims;
uint2 cloud1_dims;
uint2 cloud2_dims;
uint2 cloud3_dims;
uint2 cloud4_dims;

Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

struct VS_OUT
{
    float2 C0 : TEXCOORD0;
    nointerpolation uint C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float4 position : SV_Position;
};

VS_OUT vs_main(uint vertex_id : SV_VertexID, uint instance_id : SV_InstanceID)
{
    VS_OUT o;
    float vx = float(vertex_id & 1u);
    float vy = float((vertex_id >> 1u) & 1u);
    float2 quad = (float2(vx, vy) - 0.5f) * float2(1.0f, 2.0f);
    uint seed = (instance_id * 12u) + 25u;
    float inst = float(instance_id);

    float angle = hash_angle(seed);
    float2 facing = -float2(cos(angle), sin(angle));
    float2 facing_perp = float2(facing.y, -facing.x);
    float3 dir = float3(facing.x, 0.0f, facing.y);
    float dist_f = 1.0f - (inst * 0.002f);
    float dist2 = dist_f * dist_f;
    float3 base = (-dir) * lerp(450.0f, 550.0f, dist2);
    base.y -= lerp(35.0f, 45.0f, dist2);

    uint h1 = hash(seed);
    uint h2 = hash(h1);
    uint h3 = hash(h2);
    uint h4 = hash(h3);
    float2 size = float2(36.0f, 18.0f) * lerp(0.9f, 1.2f, rand01(h4));
    int type = int(frac(inst * 0.25f) * 4.0f);
    float2 size2;
    if (type == 1)
    {
        size2 = size * 1.2f;
        size2.y *= 0.7f;
    }
    else if (type == 2)
    {
        size2 = size * 0.6f;
    }
    else if (type == 3)
    {
        size2 = size * 1.0f;
        size2.y *= 0.9f;
    }
    else
    {
        size2 = size;
    }

    uint h5 = hash(h4);
    uint h6 = hash(h5);
    uint h7 = hash(h6);
    float3 pos = base
        + (float3(facing.y, 0.0f, -facing.x) * ((size2.x * quad.x) * lerp(1.5f, 2.0f, rand01(h6))))
        + (float3(0.0f, lerp(1.5f, 2.5f, rand01(h5)), 0.0f) * (size2.y * quad.y));

    float2 uv_base = float2(vx, 1.0f - vy);
    float2 uv_out;
    float2 facing_out;
    if (rand01(h7) > 0.5f)
    {
        uv_out = float2(1.0f - vx, uv_base.y);
        facing_out = -facing_perp;
    }
    else
    {
        uv_out = uv_base;
        facing_out = facing_perp;
    }

    float4 clip = mul(view_constants[current_view_index].world_to_sample, float4(pos, 1.0f));
    if (instance_id > 500u)
    {
        clip = asfloat(0x7fbfffffu);
    }
    o.position = clip;
    o.C0 = uv_out;
    o.C1 = instance_id;
    o.C2 = pos;
    o.C3 = float3(facing_out.x, 0.0f, facing_out.y);
    o.C4 = dir;
    return o;
}

struct PS_IN
{
    float2 C0 : TEXCOORD0;
    nointerpolation uint C1 : TEXCOORD1;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    int type = int(frac(float(i.C1) * 0.25f) * 4.0f);
    float s;
    float t;
    float u;
    float v;
    float w;
    if (type == 1)
    {
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud2.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud2.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud2.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud2.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud2.Sample(sampler_lnc, float2(sx, sy)).x;
    }
    else if (type == 2)
    {
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud3.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud3.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud3.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud3.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud3.Sample(sampler_lnc, float2(sx, sy)).x;
    }
    else if (type == 3)
    {
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud4.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud4.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud4.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud4.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud4.Sample(sampler_lnc, float2(sx, sy)).x;
    }
    else
    {
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud1.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud1.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud1.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud1.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud1.Sample(sampler_lnc, float2(sx, sy)).x;
    }

    float w2 = w * w;
    float v2 = v * v;
    float v3 = (v2 * v2) * v;
    float u2 = u * u;
    float t2 = t * t;
    float s2 = s * s;
    float3 n = normalize(float3(((w2 * w2) * w) - (v3 * v3), (t2 * t2) * t, 1.0f - ((s2 * s2) * s)));
    if (smoothstep(0.0f, 0.5f, (u2 * u2) * u) < 0.5f)
    {
        discard;
    }

    float3 albedo;
    if (glade_theme_id == 2u)
    {
        albedo = pow(canopy_color_1.xyz, 1.5f) * 0.9f;
    }
    else if (glade_theme_id == 4u)
    {
        albedo = lerp(pow(canopy_color_1.xyz, 1.5f), 1.0f, 0.5f);
    }
    else
    {
        albedo = float3(0.024f, 0.054f, 0.018f);
    }

    float3 world_n = (normalize((i.C3 * n.x) + (float3(0.0f, 1.0f, 0.0f) * n.y) + (i.C4 * n.z)) * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(lerp(albedo * 0.5f, sky_low_color.xyz * (1.0f - ((smoothstep(0.483f, 0.505f, time_of_day) - smoothstep(0.84f, 0.91f, time_of_day)) * 0.75f)), 0.02f), 0.0f, 1.0f));
    uint albedo_packed = (uint(clamp(color.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(color.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(color.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_packed = (uint(clamp(world_n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(world_n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(world_n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 0xFF000000u;
    o.target = uint2(normal_packed, albedo_packed | 0x46000000u);
    return o;
}
