// ==RECONSTRUCTED== shaders/clearing/meadow_plant.raster.hlsl
// Evidence: _clearing_meadow_plant.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings (vs): sampler_llb s4, sampler_default s9, $Globals b13, path_mask t14,
//   terrain_heightmap t16, rocky_terrain t18, water_mask t19, buildings_mask t20,
//   foliage_exclusion_mask t21, placement_buffer t22 (ByteAddressBuffer),
//   grass_object_push t23, frame_constants. (ps): sampler_mipmap s10, $Globals b13,
//   flower_texture t15, color_library b24, frame_constants.
//   vs_main: per-instance placement rejection (path/rocky/water/buildings/foliage), placement
//   transform + terrain snap + wind value-noise. ps_main: flower texture + theme color.
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

Texture2D<float4> terrain_heightmap : register(t16, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "buildings/building_mask.inc.hlsl"
#include "inc/foliage_exclusion.hlsl"
#include "inc/perlin.hlsl"
#include "inc/simplex_noise.hlsl"
#include "clearing/inc/instanced_placement.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/wind.hlsl"

// color_library.hlsl guard-skipped; flat-member cbuffer at set0/b24.
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
cbuffer color_library : register(b24, space0)
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
uint2 flower_texture_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 rocky_terrain_dims;
uint2 water_mask_dims;
uint2 buildings_mask_dims;
uint2 foliage_exclusion_mask_dims;
uint2 grass_object_push_dims;
int bool_render_shadows;

ByteAddressBuffer placement_buffer : register(t22, space0);
Texture2D<float4> path_mask : register(t14, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> water_mask : register(t19, space0);
Texture2D<float4> buildings_mask : register(t20, space0);
Texture2D<float4> foliage_exclusion_mask : register(t21, space0);
Texture2D<float4> grass_object_push : register(t23, space0);
Texture2D<float4> flower_texture : register(t15, space0);

struct VS_IN
{
    float3 vertex_position : TEXCOORD0;
    float3 vertex_color : TEXCOORD1;
    float3 vertex_normal : TEXCOORD2;
    float2 vertex_uv : TEXCOORD3;
    int plant_part_id : TEXCOORD4;
};

struct VS_OUT
{
    float3 C0 : TEXCOORD0;
    float2 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    float4 position : SV_Position;
};

VS_OUT vs_main(VS_IN i, uint instance_id : SV_InstanceID)
{
    VS_OUT o;
    float4 wp;
    float3 color;
    float2 uv;
    float3 world_pos;
    float3 normal;
    int plant_id;
    int part_id;
    do
    {
        float3 placement_pos = placement_buffer.Load<float3>(instance_id * 48u + 0);
        float2 mask_uv = (placement_pos.xz * (1.0f / 130.0f)) + 0.5f;
        bool in_bounds = !((mask_uv.x < 0.0f) || (mask_uv.x > 1.0f) || (mask_uv.y < 0.0f) || (mask_uv.y > 1.0f));
        float rocky = in_bounds ? rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).x : 0.0f;
        if ((smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x) > 0.3f) || (rocky > 0.4f))
        {
            wp = asfloat(0x7fbfffffu);
            color = 0.0f;
            uv = 0.0f;
            world_pos = 0.0f;
            normal = 0.0f;
            plant_id = 0;
            part_id = 0;
            break;
        }
        bool water_reject;
        if (!((smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x) > 0.3f) || (rocky > 0.4f)))
        {
            float water = in_bounds ? water_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y : 0.0f;
            water_reject = water > 0.25f;
        }
        else
        {
            water_reject = true;
        }
        if (!water_reject)
        {
            water_reject = buildings_mask.SampleLevel(sampler_default, mask_uv, 0.0f).x > 0.5f;
        }
        if (!water_reject)
        {
            water_reject = foliage_exclusion_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x > 0.0f;
        }
        if (water_reject)
        {
            wp = asfloat(0x7fbfffffu);
            color = 0.0f;
            uv = 0.0f;
            world_pos = 0.0f;
            normal = 0.0f;
            plant_id = 0;
            part_id = 0;
            break;
        }

        uint idx = uint(int(instance_id));
        float rot = radians(placement_buffer.Load<float>(idx * 48u + 16));
        float3 fwd = normalize(float3(cos(rot), 0.0f, sin(rot)));
        float4x4 basis = transpose(float4x4(float4(fwd, 0.0f),
                                            float4(0.0f, 1.0f, 0.0f, 0.0f),
                                            float4(normalize(cross(fwd, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                            float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float pos_x = placement_buffer.Load<float3>(idx * 48u + 0).x;
        float pos_y = placement_buffer.Load<float3>(idx * 48u + 0).y;
        float pos_z = placement_buffer.Load<float3>(idx * 48u + 0).z;
        float scale = placement_buffer.Load<float>(idx * 48u + 12);
        float4 placed = mul(float4x4(float4(1.0f, 0.0f, 0.0f, pos_x),
                                     float4(0.0f, 1.0f, 0.0f, pos_y),
                                     float4(0.0f, 0.0f, 1.0f, pos_z),
                                     float4(0.0f, 0.0f, 0.0f, 1.0f)),
                            mul(basis, mul(float4x4(float4(scale, 0.0f, 0.0f, 0.0f),
                                                   float4(0.0f, scale, 0.0f, 0.0f),
                                                   float4(0.0f, 0.0f, scale, 0.0f),
                                                   float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(i.vertex_position * 0.7f, 1.0f))));
        float4 snapped = placed;
        snapped.y = placed.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, mask_uv, 0.0f).x);
        bool is_petal = i.plant_part_id == 3;
        float2 wind_xy = (placement_pos.xz * 0.1f) + float2(float(game_time_sec) + game_time_frac, 0.0f) * 1.2f;
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
        int pid = placement_buffer.Load<int>(instance_id * 48u + 20);
        uint pid_hash = hash(uint(pid));
        float vert_y2 = i.vertex_position.y * i.vertex_position.y;
        snapped.x = placed.x - (((vert_y2 * sin((wind_noise * 3.14f) - 1.57f + 0.3f)) * 0.7f) * (is_petal ? 0.25f : 0.15f)) * lerp(0.7f, 1.0f, rand01(pid_hash));
        float4 push = grass_object_push.SampleLevel(sampler_default, mask_uv, 0.0f);
        float push_strength;
        if (is_petal)
        {
            push_strength = clamp(vert_y2, 0.0f, 1.0f) * 0.35f;
        }
        else
        {
            push_strength = pow(clamp(vert_y2, 0.0f, 1.0f), 0.5f) * 0.25f;
        }
        float4 pushed;
        if (clamp(push.z, 0.0f, 1.0f) > 0.0f)
        {
            float2 pushed_xz = snapped.xz + (push.xy * push_strength);
            pushed = float4(pushed_xz.x, snapped.y, pushed_xz.y, snapped.w);
        }
        else
        {
            pushed = snapped;
        }
        wp = mul(view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(pushed.xyz, 1.0f)));
        color = i.vertex_color;
        uv = i.vertex_uv;
        world_pos = pushed.xyz;
        normal = mul(basis, float4(i.vertex_normal, 0.0f)).xyz;
        plant_id = placement_buffer.Load<int>(instance_id * 48u + 20);
        part_id = i.plant_part_id;
        break;
    } while (false);
    o.position = wp;
    o.C0 = color;
    o.C1 = uv;
    o.C2 = world_pos;
    o.C3 = normal;
    o.C4 = plant_id;
    o.C5 = part_id;
    return o;
}

struct PS_IN
{
    float3 C0 : TEXCOORD0;
    float2 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i, float4 frag_coord : SV_Position)
{
    PS_OUT o;
    float4 cam4 = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam = cam4.xyz / cam4.w.xxx;
    bool shadow = (bool_render_shadows == 1);
    if (!shadow)
    {
        uint2 pix = uint2(frag_coord.xy);
        if (((pix.x ^ pix.y) & 1u) == 0u)
        {
            discard;
        }
    }

    float gloss;
    float alpha;
    float3 normal;
    float3 albedo;
    if (i.C5 == 2)
    {
        uint pid = uint(i.C4);
        float3 flower_col;
        if (rand01(hash(pid)) > 0.5f)
        {
            flower_col = flower_color_1.xyz;
        }
        else
        {
            flower_col = flower_color_2.xyz;
        }
        if (shadow)
        {
            float3 dxp = ddx(i.C2);
            float3 dyp = ddy(i.C2);
            float3 dx2 = ddx(i.C2);
            float3 dy2 = ddy(i.C2);
            float3 dy_neg = -dy2;
            float2 dx_uv = ddx(i.C1);
            float2 dy_uv = ddy(i.C1);
            float2 dy_neg_uv = -dy_uv;
            float3 n_col = cross(dx2, dy_neg);
            float2x3 cols = float2x3(cross(dy_neg, n_col), cross(n_col, dx2));
            float3 sphere = i.C2 + mul(transpose(float3x3(normalize(mul(float2(dx_uv.x, dy_neg_uv.x), cols)),
                                                          normalize(mul(float2(dx_uv.y, dy_neg_uv.y), cols)),
                                                          normalize(cross(dxp, -dyp)))), float3(0.5f - i.C1, 0.0f));
            float3 view_dir = -normalize(cam - i.C2);
            float3 oc = cam - sphere;
            float b = dot(oc, view_dir);
            float c = dot(oc, oc) - 0.20249998569488525390625f;
            float t = ((b * (-2.0f)) - sqrt((2.0f * b) * (2.0f * b) - ((4.0f * dot(view_dir, view_dir)) * c))) / (2.0f * dot(view_dir, view_dir));
            if (!(t == t) || !(t >= 0.0f))
            {
                discard;
            }
            normal = normalize((cam + (view_dir * t)) - sphere);
        }
        else
        {
            normal = i.C3;
        }
        if (flower_texture.Sample(sampler_mipmap, i.C1).w < 0.5f)
        {
            discard;
        }
        gloss = 0.0f;
        alpha = 1.0f;
        albedo = flower_col;
    }
    else
    {
        float g;
        float a;
        float3 ga;
        if (i.C5 == 1)
        {
            g = 1.0f;
            a = 0.0f;
            ga = float3(0.08439205586910247802734375f, 0.22000001370906829833984375f, 0.072599999606609344482421875f);
        }
        else
        {
            bool part3 = (i.C5 == 3);
            float3 stem_col;
            if (part3)
            {
                stem_col = (float3(0.093599997460842132568359375f, 0.23999999463558197021484375f, 0.10824002325534820556640625f) * clamp(i.C0, 0.0f, 1.0f)) * 0.7f;
            }
            else
            {
                stem_col = float3(0.08439205586910247802734375f, 0.22000001370906829833984375f, 0.072599999606609344482421875f);
            }
            g = float(part3);
            a = part3 ? 0.0f : 1.0f;
            ga = stem_col;
        }
        gloss = g;
        alpha = a;
        normal = i.C3;
        albedo = ga;
    }

    float3 packed_n = (normal * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(albedo * 0.7f, 0.0f, 1.0f));
    uint normal_packed = (uint(clamp(packed_n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u))
        + (uint(clamp(lerp(gloss, 0.2f, smoothstep(150.0f, 200.0f, length(i.C2))), 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    uint albedo_packed = (uint(clamp(color.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(color.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(color.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u))
        + (uint(clamp(alpha, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    o.target = uint2(normal_packed, albedo_packed);
    return o;
}
