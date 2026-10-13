// ==RECONSTRUCTED== shaders/nani_wooden_plank.raster.hlsl | B nani raster | bins: _nani_wooden_plank.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani wooden plank: per-plank halftimber color with wood-roughness variation,
// brush eraser highlight, dirth-path snow tint; packs the gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/perlin.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/wall_distortion.hlsl"
#include "wooden_plank.inc.hlsl"
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

float4x4 model_to_world;
int highlight_bricks_to_delete;
float3 brush_position;
float eraser_mult;
uint2 dirthpath_heightmap_dims;
uint2 wood_roughness_dims;

struct InstanceData
{
    Affine3Packed xform;
    int wallspace_x_range_packed;
    int seed;
    uint wall_id_is_wooden_packed;
    uint mesh_size_x;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
ByteAddressBuffer halftimber_color_ids : register(t17);
Texture2DArray<float4> halftimber_colors : register(t18);
Texture2D<float4> dirthpath_heightmap : register(t19);
Texture2D<float4> wood_roughness : register(t20);

float hash2_rand(uint h)
{
    return asfloat(((h + (h << 15u)) & 8388607u) | 1065353216u) - 1.0f;
}

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Normal : TEXCOORD2;
    float2 Vertex_UV : TEXCOORD3;
    int is_bevel : TEXCOORD4;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    nointerpolation int C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation uint C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
    nointerpolation int C7 : TEXCOORD7;
    nointerpolation int C8 : TEXCOORD8;
    nointerpolation int C9 : TEXCOORD9;
    nointerpolation int C10 : TEXCOORD10;
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
    float3 normal = mul(xform, float4(vin.Vertex_Normal, 0.0f));

    int seed_sum = nani_instance_data[idx].wallspace_x_range_packed + nani_instance_data[idx].seed;
    int plank_id = seed_sum + 45;

    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C0 = world;
    o.C1 = normalize(normal);
    o.C3 = plank_id;
    o.C4 = vin.is_bevel;
    o.C5 = uint((nani_instance_data[idx].mesh_size_x & 2u) != 0u);
    o.C6 = ((vin.Vertex_UV * 0.4f) + float2(hash2_rand(hash(uint(plank_id ^ 2340774u))), hash2_rand(hash(uint(plank_id ^ 934565u)))))
        + (float(vin.is_bevel) * 0.12f).xx;
    o.C6 *= float2(1.0f, length(float3(xform[0].y, xform[1].y, xform[2].y)) * 0.4f);
    o.C7 = int(nani_instance_data[idx].seed);
    o.C8 = int(nani_instance_data[idx].mesh_size_x & 4u);
    o.C9 = int(nani_instance_data[idx].mesh_size_x & 8u);
    o.C10 = int(nani_instance_data[idx].mesh_size_x & 16u);
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    nointerpolation int C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation uint C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
    nointerpolation int C7 : TEXCOORD7;
    nointerpolation int C8 : TEXCOORD8;
    nointerpolation int C9 : TEXCOORD9;
    nointerpolation int C10 : TEXCOORD10;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 normal = normalize(pin.C1);
    int color_id = (pin.C10 > 0) ? pin.C7 : ((pin.C5 != 0u) ? 0 : int(halftimber_color_ids.Load<uint>(uint(pin.C7) * 4 + 0)));
    float3 albedo = halftimber_colors.SampleBias(sampler_llr, float3(pin.C6, float(color_id)), global_mip_bias).xyz;
    albedo = lerp(albedo, albedo * 1.1f, float(pin.C4).xxx);
    float roughness = lerp(0.7f, 0.9f, smoothstep(0.9f, 1.0f, wood_roughness.SampleBias(sampler_llr, pin.C6, global_mip_bias).x));

    bool highlight = pin.C9 == 0 && highlight_bricks_to_delete == 1;
    bool in_brush;
    if (highlight)
    {
        in_brush = distance(brush_position.xz, pin.C0.xz) < (1.889f * eraser_mult) - 0.2f;
        if (in_brush)
        {
            albedo = lerp(albedo, float3(0.7f, 0.21f, 0.0f), 0.8f.xxx);
        }
    }

    if (glade_theme_id == 4u)
    {
        float snow = smoothstep(-0.1f, 0.0f, pin.C0.y);
        if (pin.C8 > 0)
        {
            float path = dirthpath_heightmap.SampleBias(sampler_llr, pin.C0.xz * 0.05f, global_mip_bias).x;
            float mix = ((smoothstep(0.0f, 0.2f, smoothstep(0.5f, 1.0f, roughness) * lerp(0.0f, 0.5f, path)) * 0.6f) + (path * 0.2f)) * clamp(normal.y, 0.0f, 1.0f);
            albedo = lerp(albedo, 0.7f.xxx, (mix * snow).xxx);
        }
        else
        {
            albedo = lerp(albedo, 0.7f.xxx, (smoothstep(0.0f, 0.2f, clamp(normal.y, 0.0f, 1.0f)) * snow).xxx);
        }
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
