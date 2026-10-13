// ==RECONSTRUCTED== shaders/nani_curved_halftimber.raster.hlsl | B nani raster | bins: _nani_curved_halftimber.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani curved half-timber: VS builds the curved timber beam from instance
// control points; PS shades halftimber + wood roughness + plaster fill and
// packs the gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "buildings/curved_halftimber.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/parallax_mapping.hlsl"
#include "inc/raymarch.hlsl"
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

uint2 wood_normal_dims;
uint2 wood_roughness_dims;
uint2 wood_edge_damage_dims;
uint2 dirthpath_heightmap_dims;

struct InstanceData
{
    Affine3Packed xform;
    uint flags;
    float beam_count;
    uint curve_type;
    uint color_id;
    uint wall_id;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> wood_normal : register(t16);
Texture2D<float4> wood_roughness : register(t17);
Texture2D<float4> wood_edge_damage : register(t18);
Texture2DArray<float4> halftimber_colors : register(t20);
Texture2D<float4> dirthpath_heightmap : register(t21);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    uint vertex_id : SV_VertexID;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    float C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    nointerpolation uint C9 : TEXCOORD9;
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
    o.C5 = vin.Vertex_Position.z;
    o.C6 = 1.0f;
    o.C7 = vin.Vertex_Position.xy;
    o.C8 = vin.Vertex_Position.x;
    o.C9 = 0u;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    float C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    nointerpolation uint C9 : TEXCOORD9;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float2 uv = pin.C7 * float2(0.4f, 0.06f);
    float3 normal = pin.C1;
    float2 beam_uv = uv;
    float edge = 0.0f;
    if (pin.C5 <= 0.01f)
    {
        normal = normalize(cross(ddx(pin.C2), -ddy(pin.C2)));
        beam_uv = float2(uv.y, uv.x * 2.0f) + 0.87f.xx;
        edge = 1.0f - wood_edge_damage.SampleBias(sampler_llr, pin.C7 * float2(1.0f, 0.05f), global_mip_bias).x;
    }
    int color_id = int(nani_instance_data[pin.C4].color_id);
    float2 sample_uv = beam_uv + (edge * 0.3f);
    float3 albedo = halftimber_colors.SampleBias(sampler_llr, float3(sample_uv.x, sample_uv.y, float(color_id)), global_mip_bias).xyz;
    float2 rough_uv = sample_uv * float2(0.5f, (pin.C8 < 0.0f) ? 3.0f : 2.0f);
    float roughness = lerp(0.7f, 0.9f, 1.0f - wood_roughness.SampleBias(sampler_llr, rough_uv, global_mip_bias).x);
    if (pin.C5 <= 0.01f)
    {
        albedo = lerp(albedo, 0.75f.xxx, (pin.C8 < 0.0f ? 0.2f : 0.0f).xxx);
    }
    float3 normal_map = wood_normal.SampleBias(sampler_llr, sample_uv, global_mip_bias).xyz * 2.0f - 1.0f.xxx;
    float3 shaded_normal = normalize(normal + (normal_map * 0.4f));
    if (glade_theme_id == 4u)
    {
        float path = dirthpath_heightmap.SampleBias(sampler_llr, pin.C2.xz * 0.05f, global_mip_bias).x;
        albedo = lerp(albedo, 0.7f.xxx, (((smoothstep(0.0f, 0.2f, smoothstep(0.5f, 1.0f, roughness) * lerp(0.0f, 0.5f, path)) * 0.6f) + (path * 0.2f)) * clamp(shaded_normal.y, 0.0f, 1.0f)).xxx);
    }
    float3 packed_normal = (normalize(shaded_normal) * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + (uint(clamp(roughness, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 117440512u;
    return o;
}
