// ==RECONSTRUCTED== shaders/nani_preplaced_flower.raster.hlsl | B nani raster | bins: _nani_preplaced_flower.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani pre-placed flower: VS culls flowers inside building masks (NaN position),
// bends with a 2D wind lattice + grass_object_push; PS samples an albedo map
// unless vertex color overrides it, applies snow tint, packs gbuffer.

// Registerless terrain heightmap global for the inc/terrain_heightmap.inc.hlsl
// terrain_height() helper (INC fix #3); unused here, dxc DCEs it.
Texture2D terrain_heightmap : register(t63);

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "garden/defs.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/wind.hlsl"
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/material/plant_material.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 grass_object_push_dims;
uint2 albedo_map_dims;
uint2 buildings_mask_non_blurred_dims;
int bool_render_shadows;

struct InstanceData
{
    Affine3Packed transform;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> grass_object_push : register(t18);
Texture2D<float4> albedo_map : register(t19);
Texture2D<float4> buildings_mask_non_blurred : register(t20);

uint hash2(uint2 p)
{
    uint y = p.y;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    p.x = (p.x * 1664525u) + (y + (y << 15u)) + 1013904223u;
    p.x *= 1664525u;
    uint x = p.x ^ (p.x >> 11u);
    x = x ^ ((x << 7u) & 2636928640u);
    x = x ^ ((x << 15u) & 4022730752u);
    return x;
}

float hash2_rand(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 8388607u) | 1065353216u) - 1.0f;
}

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
    float wind_bend : TEXCOORD3;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].transform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));
    float2 origin_xz = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;

    float2 heightmap_uv = (origin_xz * 0.0076923077f) + 0.5f.xx;
    float2 wind_uv = (origin_xz * 0.1f) + float2(game_time_sec + game_time_frac, 0.0f) * 1.2f;
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    float2 wind_smooth = wind_frac * wind_frac * (3.0f.xx - (wind_frac * 2.0f));
    float n00 = hash2_rand(hash2(uint2(wind_cell))) * (1.0f - wind_smooth.x) * (1.0f - wind_smooth.y);
    float n10 = hash2_rand(hash2(uint2(wind_cell + int2(1, 0)))) * wind_smooth.x * (1.0f - wind_smooth.y);
    float n01 = hash2_rand(hash2(uint2(wind_cell + int2(0, 1)))) * (1.0f - wind_smooth.x) * wind_smooth.y;
    float n11 = hash2_rand(hash2(uint2(wind_cell + int2(1, 1)))) * wind_smooth.x * wind_smooth.y;
    float wind_noise = (n00 + n10) + (n01 + n11);
    world.x -= vin.wind_bend * (sin((wind_noise * 3.14f) - 1.57f + 0.3f) * 0.175f);

    float4 grass_push = grass_object_push.SampleLevel(sampler_default, heightmap_uv, 0.0f);
    float push_amount = clamp(grass_push.z, 0.0f, 1.0f);
    float3 displaced = world;
    if (push_amount > 0.0f)
    {
        float2 displaced_xz = world.xz + ((grass_push.xy * push_amount) * vin.wind_bend);
        displaced = float3(displaced_xz.x, world.y, displaced_xz.y);
    }

    float4 clip_pos;
    float3 out_color;
    float3 out_normal;
    float3 out_world;
    float2 out_uv;
    if (buildings_mask_non_blurred.SampleLevel(sampler_default, heightmap_uv, 0.0f).x > 0.5f)
    {
        // Inside a building: NaN position culls the vertex (never rasterizes).
        clip_pos = asfloat(0x7fbfffffu).xxxx;
        out_color = 0.0f.xxx;
        out_normal = 0.0f.xxx;
        out_world = 0.0f.xxx;
        out_uv = 0.0f.xx;
    }
    else
    {
        ViewConstants vc = view_constants[current_view_index];
        clip_pos = mul(vc.world_to_sample, float4(displaced, 1.0f));
        out_color = vin.Vertex_Color;
        out_normal = mul(xform, float4(vin.Vertex_Normal, 0.0f));
        out_world = displaced;
        out_uv = origin_xz * 0.1f;
    }
    o.position = clip_pos;
    o.C0 = out_color;
    o.C1 = out_normal;
    o.C2 = out_world;
    o.C3 = out_uv;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float2 C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 albedo;
    if (length(pin.C0) == 0.0f)
    {
        albedo = albedo_map.SampleBias(sampler_llr, pin.C3, global_mip_bias).xyz;
    }
    else
    {
        albedo = pin.C0;
    }
    float3 normal = normalize(pin.C1);
    if (glade_theme_id == 4u)
    {
        albedo = lerp(albedo, 0.7f.xxx, smoothstep(0.5f, 1.0f, clamp(normal.y, 0.0f, 1.0f) * 0.8f));
    }
    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 855638016u;
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 1711276032u;
    o.gbuffer = uint2(normal_word, albedo_word);
    return o;
}
