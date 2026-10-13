// ==RECONSTRUCTED== shaders/nani_solid_vertex_color_window_glass.raster.hlsl | B nani raster | bins: _nani_solid_vertex_color_window_glass.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani window glass: VS forwards instance glass params; PS dithers
// semitransparent glass, samples curtains/indoors, applies a bindless normal
// map and packs the gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/perlin.hlsl"
#include "inc/wall_distortion.hlsl"
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

uint2 curtains_texture_dims;
uint2 indoors_texture_dims;
int bool_render_shadows;

struct InstanceData
{
    Affine3Packed xform;
    uint is_semitransparent_packed;
    uint color_id;
    uint has_albedo_texture;
    uint albedo_texture;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> curtains_texture : register(t16);
Texture2D<float4> indoors_texture : register(t17);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
    float2 Vertex_UV : TEXCOORD3;
    int is_glass : TEXCOORD4;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    nointerpolation uint C4 : TEXCOORD4;
    nointerpolation uint C5 : TEXCOORD5;
    nointerpolation float3 C6 : TEXCOORD6;
    nointerpolation uint C7 : TEXCOORD7;
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
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = world;
    o.C3 = vin.Vertex_UV;
    o.C4 = idx;
    o.C5 = uint(vin.is_glass);
    o.C6 = mul(xform, float4(vin.Vertex_Position, 1.0f)).xyz;
    o.C7 = uint(vin.is_glass);
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    nointerpolation uint C4 : TEXCOORD4;
    nointerpolation uint C5 : TEXCOORD5;
    nointerpolation float3 C6 : TEXCOORD6;
    nointerpolation uint C7 : TEXCOORD7;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    InstanceData inst = nani_instance_data[pin.C4];
    bool semitransparent = ((inst.is_semitransparent_packed >> 31u) & 1u) != 0u;
    bool dither = semitransparent && bool_render_shadows != 0;
    if (dither)
    {
        static const uint2 hi_px_subpixels[4] =
        {
            uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u)
        };
        uint2 sub = hi_px_subpixels[frame_index & 3u];
        uint2 cell = uint2(pin.position.xy) & uint2(1u, 1u);
        if (all(cell != sub))
        {
            discard;
        }
    }

    float3 albedo;
    if (inst.has_albedo_texture != 0u)
    {
        float3 bindless_sample = bindless_textures_dyn[NonUniformResourceIndex(inst.albedo_texture)].SampleBias(sampler_llr, pin.C3, global_mip_bias).xyz;
        albedo = length(bindless_sample) > 0.12f ? bindless_sample : pin.C0;
    }
    else
    {
        albedo = pin.C0;
    }
    if (pin.C5 != 0u)
    {
        float3 curtains = curtains_texture.SampleBias(sampler_llr, pin.C3, global_mip_bias).xyz;
        float3 indoors = indoors_texture.SampleBias(sampler_llr, pin.C3, global_mip_bias).xyz;
        albedo = lerp(indoors, curtains, 0.5f.xxx);
    }

    float3 normal = normalize(pin.C1);
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
