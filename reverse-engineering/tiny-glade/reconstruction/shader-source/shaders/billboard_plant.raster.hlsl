// ==RECONSTRUCTED== shaders/billboard_plant.raster.hlsl
// Evidence: _billboard_plant.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings (vs): $Globals b13 (current_view_index/model_to_world/flower1..3/grass_patch/
//   terrain_texture dims/bool_render_shadows), frame_constants set1/b0. (ps): sampler_lnc s0,
//   sampler_llr s1, flower1..3 t14..16, grass_patch t17, terrain_texture t18, frame_constants.
//   vs_main passes the model transform through; ps_main blends the terrain texture with a
//   plant-type atlas (flower1/2/3 or grass_patch) by plant_id, alpha-discards, packs the
//   normal + albedo into a uint2 target.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/sun.inc.hlsl"

// color_library.hlsl is guard-skipped (billboard_plant binds no color_library cbuffer).
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
// NOTE: inc/lighting.hlsl omitted (references nested color_library.indirect_light_tint;
// unused here) — see billboard_forest note.

// $Globals members (implicit cbuffer at set0/b13).
float4x4 model_to_world;
uint2 flower1_dims;
uint2 flower2_dims;
uint2 flower3_dims;
uint2 grass_patch_dims;
uint2 terrain_texture_dims;
int bool_render_shadows;

Texture2D<float4> flower1 : register(t14, space0);
Texture2D<float4> flower2 : register(t15, space0);
Texture2D<float4> flower3 : register(t16, space0);
Texture2D<float4> grass_patch : register(t17, space0);
Texture2D<float4> terrain_texture : register(t18, space0);

struct VS_IN
{
    float3 vertex_position : TEXCOORD0;
    float3 vertex_normal : TEXCOORD1;
    float2 vertex_uv : TEXCOORD2;
    float3 uv2 : TEXCOORD3;
    float bby : TEXCOORD4;
    int plant_id : TEXCOORD5;
    int prim_id : TEXCOORD6;
};

struct VS_OUT
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation int C6 : TEXCOORD6;
    float4 position : SV_Position;
};

VS_OUT vs_main(VS_IN i)
{
    VS_OUT o;
    float4 world = mul(model_to_world, float4(i.vertex_position, 1.0f));
    o.position = mul(view_constants[current_view_index].world_to_sample, world);
    o.C0 = i.vertex_uv;
    o.C1 = mul(model_to_world, float4(i.vertex_normal, 0.0f)).xyz;
    o.C2 = world.xyz;
    o.C3 = float2(i.uv2.x, 1.0f - i.uv2.y);
    o.C4 = i.bby;
    o.C5 = i.plant_id;
    o.C6 = i.prim_id;
    return o;
}

struct PS_IN
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float2 C3 : TEXCOORD3;
    float C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation int C6 : TEXCOORD6;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    float4 terrain = terrain_texture.Sample(sampler_lnc, i.C3);
    float3 albedo;
    float alpha;
    if (i.C5 == 0)
    {
        float4 tex = flower1.SampleBias(sampler_lnc, i.C0, (-1.0f) + global_mip_bias);
        albedo = lerp(terrain.xyz, tex.xyz, (i.C4 * i.C4).xxx);
        alpha = tex.w * tex.w;
    }
    else if (i.C5 == 2)
    {
        float4 tex = flower2.SampleBias(sampler_lnc, i.C0, (-1.0f) + global_mip_bias);
        albedo = lerp(terrain.xyz, tex.xyz, i.C4.xxx);
        alpha = tex.w * tex.w;
    }
    else if (i.C5 == 3)
    {
        float4 tex = flower3.SampleBias(sampler_lnc, i.C0, (-1.0f) + global_mip_bias);
        albedo = lerp(terrain.xyz, tex.xyz, i.C4.xxx);
        alpha = tex.w * tex.w;
    }
    else
    {
        int sub = i.C6 - 4 * (i.C6 / 4);
        float4 tex = grass_patch.SampleBias(sampler_llr, float2(0.5f * (i.C0.x + float(sub - 2 * (sub / 2))), 0.5f * (i.C0.y + float(int(floor(float(sub / 2)))))), (-1.0f) + global_mip_bias);
        float3 ga;
        if (glade_theme_id == 4u)
        {
            ga = lerp(terrain.xyz, tex.xyz, pow(i.C4, 2.5f));
        }
        else
        {
            ga = lerp(terrain.xyz, tex.xyz, pow(i.C4, 1.5f));
        }
        albedo = ga;
        alpha = tex.w * tex.w;
    }
    if (alpha < 0.1f)
    {
        discard;
    }
    float3 n = (normalize(i.C1) * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(albedo, 0.0f, 1.0f));
    uint albedo_packed = (uint(clamp(color.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(color.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(color.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_packed = (uint(clamp(n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 0xE6000000u;
    o.target = uint2(normal_packed, albedo_packed | 0x06000000u);
    return o;
}
