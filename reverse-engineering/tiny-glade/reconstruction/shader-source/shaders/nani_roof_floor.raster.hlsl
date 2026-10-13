// ==RECONSTRUCTED== shaders/nani_roof_floor.raster.hlsl | B nani raster | bins: _nani_roof_floor.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani roof/floor: per-instance roof-tile transform; PS selects a wooden or
// stone floor material by instance type, applies height-parallax + normal map,
// cuts dormer holes via a spatial hash grid and packs the gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "roof_material_id.inc.hlsl"
#include "roof/stone_floor.inc.hlsl"
#include "roof/wooden_floor.inc.hlsl"
#include "roof/dormer_hole.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/parallax_mapping.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/wall_color_ids.hlsl"
#include "inc/color_library.hlsl"
#include "inc/wall_color_customization.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
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

uint highlighted_wall_id;
uint highlight_mode;

struct InstanceData
{
    Affine3Packed xform;
    uint material_id;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
Texture2D<float4> dirthpath_heightmap : register(t15);
StructuredBuffer<InstanceData> nani_instance_data : register(t16);
ByteAddressBuffer dormer_holes : register(t18);
ByteAddressBuffer dormer_hole_count : register(t19);
ByteAddressBuffer halftimber_color_ids : register(t20);
Texture2DArray<float4> halftimber_colors : register(t21);
Texture2D<float4> wooden_floor_normal : register(t22);
Texture2D<float4> wooden_floor_height : register(t23);
Texture2D<float4> wooden_floor_seed : register(t24);
Texture2D<float4> wooden_floor_roughness : register(t25);
Texture2D<float4> wooden_floor_normal_2 : register(t26);
Texture2D<float4> wooden_floor_height_2 : register(t27);
Texture2D<float4> wooden_floor_seed_2 : register(t28);
Texture2D<float4> wooden_floor_roughness_2 : register(t29);
Texture2D<float4> wooden_floor_normal_3 : register(t30);
Texture2D<float4> wooden_floor_height_3 : register(t31);
Texture2D<float4> wooden_floor_seed_3 : register(t32);
Texture2D<float4> wooden_floor_roughness_3 : register(t33);
ByteAddressBuffer wall_color_ids : register(t34);
Texture2DArray<float4> brick_colors : register(t35);
Texture2D<float4> stone_floor_normal : register(t37);
Texture2D<float4> stone_floor_seed : register(t38);
Texture2D<float4> stone_floor_height : register(t39);
Texture2D<float4> stone_floor_roughness : register(t40);
Texture2D<float4> stone_floor_seed_2 : register(t41);
Texture2D<float4> stone_floor_normal_2 : register(t42);
Texture2D<float4> stone_floor_height_2 : register(t43);
Texture2D<float4> stone_floor_roughness_2 : register(t44);
Texture2D<float4> stone_floor_seed_3 : register(t45);
Texture2D<float4> stone_floor_normal_3 : register(t46);
Texture2D<float4> stone_floor_height_3 : register(t47);
Texture2D<float4> stone_floor_roughness_3 : register(t48);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C1 = mul(xform, float4(0.0f, 1.0f, 0.0f, 0.0f));
    o.C2 = world;
    o.C4 = idx;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    InstanceData inst = nani_instance_data[pin.C4];
    float2 floor_uv = pin.C2.xz * 0.2f;
    float3 normal = normalize(pin.C1);

    float3 albedo;
    float3 normal_map;
    float seed_val;
    uint variant = inst.material_id & 3u;
    if (inst.material_id < 3u)
    {
        float height_lod;
        float rough;
        float3 nm;
        float sd;
        if (variant == 0u)
        {
            height_lod = stone_floor_height.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = stone_floor_roughness.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = stone_floor_normal.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = stone_floor_seed.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        else if (variant == 1u)
        {
            height_lod = stone_floor_height_2.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = stone_floor_roughness_2.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = stone_floor_normal_2.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = stone_floor_seed_2.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        else
        {
            height_lod = stone_floor_height_3.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = stone_floor_roughness_3.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = stone_floor_normal_3.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = stone_floor_seed_3.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        albedo = lerp(0.8f.xxx, 0.9f.xxx, rough);
        normal_map = nm;
        seed_val = sd;
        albedo *= 0.9f + seed_val * 0.2f;
    }
    else
    {
        uint wv = variant;
        float height_lod;
        float rough;
        float3 nm;
        float sd;
        if (wv == 0u)
        {
            height_lod = wooden_floor_height.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = wooden_floor_roughness.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = wooden_floor_normal.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = wooden_floor_seed.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        else if (wv == 1u)
        {
            height_lod = wooden_floor_height_2.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = wooden_floor_roughness_2.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = wooden_floor_normal_2.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = wooden_floor_seed_2.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        else
        {
            height_lod = wooden_floor_height_3.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = wooden_floor_roughness_3.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = wooden_floor_normal_3.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = wooden_floor_seed_3.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        albedo = lerp(0.6f.xxx, 0.8f.xxx, rough);
        normal_map = nm;
        seed_val = sd;
        albedo *= 0.9f + seed_val * 0.2f;
    }
    normal = normalize(normal + (normal_map * 0.4f));
    uint wall_id = nani_instance_data[pin.C4].material_id & 0xFFFFu;
    int timber_color_id = int(halftimber_color_ids.Load<uint>(wall_id * 4 + 0));
    float3 timber = halftimber_colors.SampleBias(sampler_llr, float3(floor_uv, float(timber_color_id)), global_mip_bias).xyz;
    albedo = lerp(albedo, timber, 0.3f);
    float3 brick = brick_colors.SampleBias(sampler_llr, float3(floor_uv, float(wall_color_ids.Load<uint>(wall_id * 4 + 0))), global_mip_bias).xyz;
    albedo = lerp(albedo, brick, 0.1f);
    uint dormer_cell = uint(int(floor(floor_uv.x * 8.0f)) + 15 * int(floor(floor_uv.y * 8.0f)));
    uint dormer_count = min(128u, dormer_hole_count.Load<uint>(dormer_cell * 4 + 0));
    for (uint i = 0u; i < dormer_count; i++)
    {
        uint hole_idx = uint(int(uint(dormer_cell * 50) + i));
        if (dormer_holes.Load<uint>(hole_idx * 48 + 12) == wall_id)
        {
            discard;
        }
    }
    if (glade_theme_id == 4u)
    {
        float path = dirthpath_heightmap.SampleBias(sampler_llr, pin.C2.xz * 0.05f, global_mip_bias).x;
        albedo = lerp(albedo, 0.7f.xxx, (path * smoothstep(-0.1f, 0.0f, pin.C2.y) * clamp(normal.y, 0.0f, 1.0f)).xxx);
    }
    if (highlight_mode == 1u && wall_id == highlighted_wall_id)
    {
        albedo = lerp(albedo, float3(0.7f, 0.21f, 0.0f), 0.8f.xxx);
    }
    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 117440512u;
    return o;
}
