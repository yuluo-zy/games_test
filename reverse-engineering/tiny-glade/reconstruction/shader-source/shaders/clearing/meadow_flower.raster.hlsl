// ==RECONSTRUCTED== shaders/clearing/meadow_flower.raster.hlsl
// Evidence: _clearing_meadow_flower.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings (vs): sampler_llb s4, sampler_default s9, $Globals b13, path_mask t14,
//   grass_object_push t19, terrain_heightmap t20, rocky_terrain t22, water_mask t23,
//   frame_constants. (ps): sampler_llr s1, $Globals b13, albedo_map t15, normal_map t16,
//   alpha_map t17, leafy_greens t18, frame_constants.
//   vs_main: rocky/path/water rejection, terrain snap, wind value-noise, TBN. ps_main:
//   alpha discard + frame-stagger discard, albedo/normal from maps, packed uint2.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"

Texture2D<float4> terrain_heightmap : register(t20, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/simplex_noise.hlsl"
#include "clearing/inc/instanced_placement.hlsl"
#include "inc/pack_unpack.hlsl"

// color_library.hlsl guard-skipped (not bound by this shader).
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
#include "inc/math.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/material/plant_material.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
// NOTE: inc/lighting.hlsl omitted (nested color_library reference; see billboard_forest note).

// $Globals members (implicit cbuffer at set0/b13).
float4x4 model_to_world;
float time;
uint2 path_mask_dims;
uint2 albedo_map_dims;
uint2 normal_map_dims;
int has_normal_map;
uint2 alpha_map_dims;
uint2 leafy_greens_dims;
uint2 grass_object_push_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 rocky_terrain_dims;
uint2 water_mask_dims;
int bool_render_shadows;

Texture2D<float4> path_mask : register(t14, space0);
Texture2D<float4> grass_object_push : register(t19, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);
Texture2D<float4> albedo_map : register(t15, space0);
Texture2D<float4> normal_map : register(t16, space0);
Texture2D<float4> alpha_map : register(t17, space0);
Texture2D<float4> leafy_greens : register(t18, space0);

struct VS_IN
{
    float3 vertex_position : TEXCOORD0;
    float3 vertex_normal : TEXCOORD1;
    float2 vertex_uv : TEXCOORD2;
    float3 flower_pivot : TEXCOORD3;
    int stem : TEXCOORD4;
    float wind_bend : TEXCOORD5;
};

struct VS_OUT
{
    float2 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    nointerpolation int C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
    float3x3 C7 : TEXCOORD7;
    float4 position : SV_Position;
};

VS_OUT vs_main(VS_IN i)
{
    VS_OUT o;
    float4 wp;
    float2 wind_uv;
    float3 world_pos;
    float3 normal;
    int stem_out;
    float2 uv_out;
    float3x3 tbn;
    do
    {
        float2 mask_uv = (i.flower_pivot.xz * (1.0f / 130.0f)) + 0.5f;
        bool in_bounds = !((mask_uv.x < 0.0f) || (mask_uv.x > 1.0f) || (mask_uv.y < 0.0f) || (mask_uv.y > 1.0f));
        float rocky = in_bounds ? rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).x : 0.0f;
        if ((smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x) > 0.1f) || (rocky > 0.1f))
        {
            wp = asfloat(0x7fbfffffu);
            wind_uv = 0.0f;
            world_pos = 0.0f;
            normal = 0.0f;
            stem_out = 0;
            uv_out = 0.0f;
            tbn = float3x3(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f);
            break;
        }
        bool water_reject;
        if (!((smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x) > 0.1f) || (rocky > 0.1f)))
        {
            float water = in_bounds ? water_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y : 0.0f;
            water_reject = water > 0.25f;
        }
        else
        {
            water_reject = true;
        }
        if (water_reject)
        {
            wp = asfloat(0x7fbfffffu);
            wind_uv = 0.0f;
            world_pos = 0.0f;
            normal = 0.0f;
            stem_out = 0;
            uv_out = 0.0f;
            tbn = float3x3(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f);
            break;
        }
        float4 v = float4(i.vertex_position, 1.0f);
        v.y = i.vertex_position.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, mask_uv, 0.0f).x);
        float2 wind_xy = (i.flower_pivot.xz * 0.1f) + float2(time, 0.0f) * 1.2f;
        int2 wind_cell = int2(floor(wind_xy));
        float2 wind_frac = frac(wind_xy);
        float2 wind_smooth = wind_frac * wind_frac * (3.0f - (2.0f * wind_frac));
        float wind_n00 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 0))));
        float wind_n10 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 0))));
        float wind_n01 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 1))));
        float wind_n11 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 1))));
        float wind_noise = ((1.0f - wind_smooth.y) * ((wind_n00 * (1.0f - wind_smooth.x)) + (wind_n10 * wind_smooth.x)))
            + ((wind_n01 * (1.0f - wind_smooth.x)) * wind_smooth.y)
            + ((wind_n11 * wind_smooth.x) * wind_smooth.y);
        v.x = i.vertex_position.x - (i.wind_bend * sin((wind_noise * 3.14f) - 1.57f + 0.3f)) * 0.175f;
        float4 push = grass_object_push.SampleLevel(sampler_default, mask_uv, 0.0f);
        float4 v2;
        if (clamp(push.z, 0.0f, 1.0f) > 0.0f)
        {
            float2 pushed = v.xz + ((push.xy * pow(i.wind_bend, 0.5f)) * 0.5f);
            v2 = float4(pushed.x, v.y, pushed.y, v.w);
        }
        else
        {
            v2 = v;
        }
        wp = mul(view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(v2.xyz, 1.0f)));
        wind_uv = wind_xy;
        world_pos = v2.xyz;
        normal = i.vertex_normal;
        stem_out = i.stem;
        uv_out = i.vertex_uv;
        tbn = transpose(float3x3(normalize(mul(model_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz),
                                 normalize(mul(model_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz),
                                 normalize(mul(model_to_world, float4(0.0f, 1.0f, 0.0f, 0.0f)).xyz)));
        break;
    } while (false);
    o.position = wp;
    o.C1 = wind_uv;
    o.C2 = world_pos;
    o.C3 = normal;
    o.C5 = stem_out;
    o.C6 = uv_out;
    o.C7 = tbn;
    return o;
}

// Pack a [0,1] normal rgb into one uint channel of the uint2 target.
uint pack_normal8(float3 n)
{
    return (uint(clamp(n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
}

struct PS_IN
{
    float2 C1 : TEXCOORD1;
    float3 C3 : TEXCOORD3;
    nointerpolation int C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
    float3x3 C7 : TEXCOORD7;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i, float4 frag_coord : SV_Position)
{
    PS_OUT o;
    if (alpha_map.Sample(sampler_llr, i.C6).x < 0.5f)
    {
        discard;
    }
    if (bool_render_shadows != 1)
    {
        uint2 pix = uint2(frag_coord.xy) & uint2(1u, 1u);
        if ((frame_index & 3u) == (pix.x + (pix.y * 2u)))
        {
            discard;
        }
    }
    float4 albedo_tex = albedo_map.Sample(sampler_llr, i.C1);
    float3 albedo;
    if (i.C5 == 1)
    {
        albedo = leafy_greens.Sample(sampler_llr, i.C1).xyz;
    }
    else
    {
        albedo = albedo_tex.xyz;
    }
    float3 n;
    if (has_normal_map == 1)
    {
        n = normalize(mul(i.C7, (normal_map.Sample(sampler_llr, i.C6).xyz * 2.0f) - 1.0f));
    }
    else
    {
        n = i.C3;
    }
    float3 packed_n = (n * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(albedo, 0.0f, 1.0f));
    o.target = uint2((pack_normal8(packed_n) + 0x80000000u), (pack_normal8(color) + 0x4D000000u));
    return o;
}
