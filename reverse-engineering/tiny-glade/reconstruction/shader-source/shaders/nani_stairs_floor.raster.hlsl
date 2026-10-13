// ==RECONSTRUCTED== shaders/nani_stairs_floor.raster.hlsl | B nani raster | bins: _nani_stairs_floor.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani stairs floor: VS forwards the floor tile transform; PS shades the stone
// floor with a height-parallax + normal map, snow tint and packs the gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "nani/inc/trimesh.hlsl"
#include "roof_material_id.inc.hlsl"
#include "roof/stone_floor.inc.hlsl"
#include "roof/wooden_floor.inc.hlsl"
#include "roof/dormer_hole.inc.hlsl"
#include "stairs_floor.inc.hlsl"
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

uint2 dirthpath_heightmap_dims;

struct InstanceData
{
    Affine3Packed xform;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> dirthpath_heightmap : register(t16);
Texture2DArray<float4> brick_colors : register(t35);
Texture2D<float4> stone_floor_normal : register(t37);
Texture2D<float4> stone_floor_seed : register(t38);
Texture2D<float4> stone_floor_height : register(t39);
Texture2D<float4> stone_floor_roughness : register(t40);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C3 : TEXCOORD3;
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
    o.C3 = idx + 100000u;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float2 floor_uv = pin.C2.xz * 0.2f;
    float3 normal = normalize(pin.C1);
    float height_lod = stone_floor_height.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
    float3 normal_map = stone_floor_normal.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
    float seed = stone_floor_seed.SampleBias(sampler_llr, floor_uv, height_lod).x;
    normal = normalize(normal + (normal_map * 0.4f));
    float3 albedo = lerp(0.8f.xxx, 0.9f.xxx, stone_floor_roughness.SampleBias(sampler_llr, floor_uv, height_lod).x) * (0.9f + seed * 0.2f);
    float2 brick_uv = floor_uv * 0.5f;
    albedo = lerp(albedo, brick_colors.SampleBias(sampler_llr, float3(brick_uv, 0.0f), global_mip_bias).xyz, 0.05f.xxx);
    if (glade_theme_id == 4u)
    {
        float path = dirthpath_heightmap.SampleBias(sampler_llr, pin.C2.xz / max(float2(dirthpath_heightmap_dims), 1.0f.xx), global_mip_bias).x;
        albedo = lerp(albedo, 0.7f.xxx, (path * smoothstep(-0.1f, 0.0f, pin.C2.y) * clamp(normal.y, 0.0f, 1.0f)).xxx);
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
