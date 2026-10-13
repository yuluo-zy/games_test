// ==RECONSTRUCTED== shaders/nani_instanced_roof.raster.hlsl | B nani raster | bins: _nani_instanced_roof.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani instanced roof: per-tile transform from instanced_data; PS shades the
// roof tile palette with moss, damage and normal map and packs the gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/perlin.hlsl"
#include "inc/parallax_mapping.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/texture_blend.hlsl"
#include "inc/wall_distortion.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/roof_colors_include.hlsl"
#include "roof/roof_tile.inc.hlsl"
#include "roof/dormer_hole.inc.hlsl"
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

uint2 moss_texture_dims;
uint2 roof_tile_normal_dims;
uint2 roof_tile_damage_dims;
uint highlighted_wall_id;
uint highlight_mode;

struct RoofTileData
{
    Affine3Packed xform;
    uint flags;
    uint wall_id;
    float3 apex;
    uint roof_type;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t15);
StructuredBuffer<RoofTileData> instanced_data : register(t21);
ByteAddressBuffer frame_constants_storage_buffer : register(t1, space1);
Texture2DArray<float4> roof_colors : register(t16);
ByteAddressBuffer roof_color_ids : register(t17);
Texture2D<float4> roof_tile_normal : register(t19);
Texture2D<float4> roof_tile_damage : register(t20);
StructuredBuffer<uint> dormer_holes : register(t22);
StructuredBuffer<uint> dormer_hole_count : register(t23);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C2 : TEXCOORD2;
    float4 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    nointerpolation uint C8 : TEXCOORD8;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = instanced_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));
    float4x4 view_matrix = float4x4(
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 624),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 640),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 656),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 672),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 628),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 644),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 660),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 676),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 632),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 648),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 664),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 680),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 636),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 652),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 668),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 684));
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(view_matrix, float4(world, 1.0f));
    o.C2 = world;
    o.C3 = float4(mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f)), 1.0f);
    o.C4 = vin.Vertex_Position.xz;
    o.C8 = idx;
    return o;
}

struct PsIn
{
    float3 C2 : TEXCOORD2;
    float4 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    nointerpolation uint C8 : TEXCOORD8;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 normal = normalize(cross(ddx(pin.C2), -ddy(pin.C2)));
    if (dot(float4(pin.C2, 1.0f), float4(0.0f, 1.0f, 0.0f, 0.0f)) < 0.0f)
    {
        discard;
    }
    uint dormer_count = dormer_hole_count[0];
    float4 dormer_check = frame_constants_storage_buffer.Load<float4>(current_view_index * 800 + 784);
    if (dormer_count > 0u)
    {
        for (uint i = 0u; i < min(dormer_count, 64u); i++)
        {
            if (distance(dormer_holes[i].xxx + dormer_check.xyz, pin.C2) < 1.0f)
            {
                discard;
            }
        }
    }
    RoofTileData inst = instanced_data[pin.C8];
    uint color_id = roof_color_ids.Load<uint>(inst.wall_id * 4 + 0);
    float3 albedo = roof_colors.SampleBias(sampler_llr, float3(pin.C4, float(color_id)), 0.0f).xyz;
    float tile_damage = roof_tile_damage.SampleBias(sampler_llr, pin.C4 * 0.5f, 0.0f).x;
    albedo *= lerp(0.7f, 1.0f, tile_damage);
    float3 tile_normal_map = roof_tile_normal.SampleBias(sampler_llr, pin.C4 * 0.5f, 0.0f).xyz * 2.0f - 1.0f.xxx;
    normal = normalize(normal + (tile_normal_map * 0.3f));
    if (highlight_mode == 1u && inst.wall_id == highlighted_wall_id)
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
