// ==RECONSTRUCTED== shaders/nani_solid_albedo_map.raster.hlsl | B nani raster | bins: _nani_solid_albedo_map.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani solid raster with a per-instance albedo-map texture layer: VS forwards
// the albedo texture index + vertex flags; PS samples the albedo map unless a
// vertex color overrides it, applies the snow tint, and packs the gbuffer.

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

int bool_render_shadows;

struct InstanceData
{
    Affine3Packed transform;
    uint albedo_texture;
    uint pad0;
    uint pad1;
    uint pad2;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2DArray<float4> albedo_maps : register(t16);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
    float2 Vertex_UV : TEXCOORD3;
    int flags : TEXCOORD4;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    float3 C5 : TEXCOORD5;
    nointerpolation int C6 : TEXCOORD6;
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
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = world;
    o.C3 = vin.Vertex_UV;
    o.C4 = int(nani_instance_data[idx].albedo_texture);
    o.C5 = vin.Vertex_Position;
    o.C6 = vin.flags;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float2 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation int C6 : TEXCOORD6;
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
        albedo = albedo_maps.SampleBias(sampler_llr, float3(pin.C3, float(pin.C4)), global_mip_bias).xyz;
    }
    else
    {
        albedo = pin.C0;
    }
    float3 normal = normalize(pin.C1);
    if (glade_theme_id == 4u)
    {
        albedo = lerp(albedo, 0.7f.xxx, ((pin.C6 & 1) != 0 ? 0.5f : 1.0f)
            * smoothstep(0.5f, 1.0f, clamp(normal.y, 0.0f, 1.0f) * 0.8f));
    }
    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 117440512u;
    return o;
}
