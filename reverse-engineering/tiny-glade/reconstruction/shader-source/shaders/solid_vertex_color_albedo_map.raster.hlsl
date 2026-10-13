// ==RECONSTRUCTED== shaders/solid_vertex_color_albedo_map.raster.hlsl | B simple-solid | bins: _solid_vertex_color_albedo_map.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Solid raster sampling an albedo map (terrain/building ground) with snow tint,
// roughness gbuffer top byte, and contact-shadow flag.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

uint receive_contact_shadows;
uint is_semitransparent;
uint2 albedo_map_dims;
float4x4 model_to_world;
int bool_render_shadows;
int is_terrain;
float roughness_param;

Texture2D<float4> albedo_map : register(t14);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Normal : TEXCOORD1;
    float2 Vertex_UV : TEXCOORD2;
};

struct VsOut
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, world);
    o.C0 = vin.Vertex_UV;
    o.C1 = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    o.C2 = world.xyz;
    return o;
}

struct PsIn
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    bool dither = is_semitransparent != 0u && bool_render_shadows != 0;
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

    float3 albedo = albedo_map.SampleBias(sampler_llr, pin.C0, global_mip_bias).xyz;
    float3 normal = normalize(pin.C1);
    bool snow = glade_theme_id == 4u && !(is_terrain != 0);
    if (snow)
    {
        albedo = lerp(albedo, 0.7f.xxx, smoothstep(0.5f, 1.0f, clamp(normal.y, 0.0f, 1.0f)).xxx);
    }

    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + (uint(clamp(roughness_param, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    uint flags = ((((((0u | (uint(receive_contact_shadows != 0u) << 0u)) | 2u) | 4u) | 0u) | 0u) | 0u);
    o.gbuffer = uint2(normal_word, albedo_word | (flags << 24u));
    return o;
}
