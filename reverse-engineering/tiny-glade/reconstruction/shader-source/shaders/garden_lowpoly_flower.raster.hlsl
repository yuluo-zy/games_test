// ==RECONSTRUCTED== shaders/garden_lowpoly_flower.raster.hlsl | evidence: spirv-cross of _garden_lowpoly_flower.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin (identical VS both variants: Flower03Xform.position float4 = pos.xyz*scale.w+scale, terrain height lift, perlin wind bend (wind_bend^2 * sin(noise*PI-HALF_PI+0.3)*0.0735), grass_object_push gust offset, world_to_sample; PS variant b903: albedo SampleBias sqrt-clamp, RG8 pack uint2(0x80808080, RGB8 + 0x4D000000); variant f0ad: empty PS) | status: O (algorithm from SPIR-V); I (function names; 0.4/0.2/0.8/0.0735 literals) | note: the two 16-hex variants are compile-time feature variants — b903 = albedo PS (default), f0ad = empty PS via GARDEN_FLOWER_ALBEDO=0

#ifndef GARDEN_FLOWER_ALBEDO
#define GARDEN_FLOWER_ALBEDO 1
#endif

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/wind.hlsl"
#include "inc/perlin.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"
Texture2D<float4> terrain_heightmap : register(t17, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/math.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/material/plant_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sh.hlsl"

struct Flower03Xform
{
    float4 position;
};

cbuffer $Globals : register(b13, space0)
{
    // NOTE: inc/current_view.hlsl (reconstructed) declares current_view_index
    // as a file-scope global; the original embeds it as $Globals member 0 here.
    // Declared locally so the $Globals binding stays b13 (flag: inc issue).
    uint current_view_index;
    float4x4 model_to_world;
    uint2 path_mask_dims;
    uint2 albedo_map_dims;
    uint2 grass_object_push_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    int bool_render_shadows;
};

// frame_constants.hlsl (reconstructed inc/) omits the dead unbounded-array
// decl the original used to emit RuntimeDescriptorArray; reproduce it here.
RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

StructuredBuffer<Flower03Xform> instance_buffer : register(t21, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

struct VSInput
{
    float3 position : TEXCOORD0;
    float3 normal : TEXCOORD2;
    float wind_bend : TEXCOORD3;
    uint instance_id : SV_InstanceID;
};

struct VSOutput
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VSOutput vs_main(VSInput i)
{
    float4 xform = instance_buffer[i.instance_id].position;

    float3 pos = i.position * xform.w + xform.xyz;

    float4 world_pos = float4(pos.x, pos.yz, 1.0f);
    float2 uv = (xform.xz * 0.0076923076994717121124267578125f) + 0.5f;
    world_pos.y = (pos.y + get_terrain_height(terrain_heightmap, sampler_default, xform.xz)) - 0.2f;

    float2 wind_uv = world_pos.xz * 0.1f + float2(float(game_time_sec) + game_time_frac, 0.0f) * 1.2f;
    float gust = sin(perlin(wind_uv) * PI - 1.57f + 0.3f);
    world_pos.x = pos.x - (i.wind_bend * i.wind_bend) * gust * 0.0735f;

    float3 grass = grass_object_push.SampleLevel(sampler_default, uv, 0.0f).xyz * 0.4f;
    float g = clamp(grass.z, 0.0f, 1.0f);

    float4 final_pos;
    if (g > 0.0f)
        final_pos = float4((world_pos.xz + (grass.xy * g) * i.wind_bend), world_pos.y, world_pos.w);
    else
        final_pos = world_pos;

    VSOutput o;
    o.position = mul(view_constants[current_view_index].world_to_sample, final_pos);
    o.C1 = final_pos.xyz;
    o.C2 = i.normal;
    o.C3 = (final_pos.xz * 0.1f + xform.w.xx) + 0.8f;
    return o;
}

Texture2D<float4> albedo_map : register(t15, space0);

#if GARDEN_FLOWER_ALBEDO
struct PSInput
{
    float2 C3 : TEXCOORD3;
};

uint2 ps_main(PSInput i) : SV_Target0
{
    float3 c = sqrt(clamp(albedo_map.SampleBias(sampler_llr, i.C3, global_mip_bias).xyz, 0.0f, 1.0f));
    uint2 out_c = uint2(2155905152u, 0u);
    out_c.y = (uint(clamp(c.x, 0.0f, 1.0f) * 255.0f + 0.5f)
               | (uint(clamp(c.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
               | (uint(clamp(c.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u))
              + 1291845632u;
    return out_c;
}
#else
// Empty fragment (feature-off variant f0ad): no output interface.
void ps_main()
{
}
#endif
