// ==RECONSTRUCTED== shaders/nani_uber.raster.hlsl | B nani raster | bins: _nani_uber.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani uber material raster: VS forwards instance flags + bindless albedo
// index; PS samples the bindless albedo, applies snow tint and packs the
// gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
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

struct InstanceFlags
{
    uint has_albedo_texture;
};

struct InstanceData
{
    Affine3Packed xform;
    InstanceFlags flags;
    uint albedo_texture;
    uint pad0;
    uint pad1;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Normal : TEXCOORD1;
    float2 Vertex_UV : TEXCOORD2;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C3 : TEXCOORD3;
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
    o.C0 = vin.Vertex_UV;
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = world;
    o.C3 = nani_instance_data[idx].flags.has_albedo_texture;
    o.C4 = nani_instance_data[idx].albedo_texture;
    return o;
}

struct PsIn
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C3 : TEXCOORD3;
    nointerpolation uint C4 : TEXCOORD4;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    bool dither = ((pin.C3 >> 1u) & 1u) != 0u && bool_render_shadows != 0;
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
    if ((pin.C3 & 1u) != 0u)
    {
        albedo = bindless_textures_dyn[NonUniformResourceIndex(pin.C4)].SampleBias(sampler_llr, pin.C0, global_mip_bias).xyz;
    }
    else
    {
        albedo = 1.0f.xxx;
    }
    float3 normal = normalize(pin.C1);
    if (glade_theme_id == 4u)
    {
        albedo = lerp(albedo, 0.7f.xxx, smoothstep(0.5f, 1.0f, clamp(normal.y, 0.0f, 1.0f)));
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
    uint flags = ((0u | (uint(((pin.C3 >> 2u) & 1u) != 0u) << 0u)) | 2u) | 4u;
    o.gbuffer = uint2(normal_word, albedo_word | (flags << 24u));
    return o;
}
