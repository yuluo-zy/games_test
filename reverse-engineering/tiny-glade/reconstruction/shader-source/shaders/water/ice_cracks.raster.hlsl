// ==RECONSTRUCTED== shaders/water/ice_cracks.raster.hlsl
// SOURCE PATH : shaders/water/ice_cracks.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _water_ice_cracks.raster.hlsl_b903e43ffb3da915.bin   (ps packs crack tint -> uint2)
//   _water_ice_cracks.raster.hlsl_f0adff76827e73ff.bin   (empty ps, no output interface)
// entries: vs:vs_main  ps:ps_main | index_pass W2 (Inference)
// Status: O (bindings/entry; vs + ps from spirv-cross)
// Evidence: spirv-cross vs (model_to_world transform, world_to_sample; C0=vertex color,
//   C1=model*normal, C2=world) and ps (b903e: view_constants[2] view_to_world up-vector
//   color, sqrt-clamp of C0*0.4, RGB8 pack into uint2 albedo+material-id; f0adff: empty).
// MERGE (water-ice review water_ice_cracks.raster.hlsl): A adopted wholesale. A's file
//   path water/ice_cracks.raster.hlsl matches the bin's embedded source path (NOT
//   water_ice_cracks.raster.hlsl); water_mask_sampler.inc added (the bin closure
//   includes shaders/water/water_mask_sampler.inc.hlsl + texture_utils).
// Note: two 16-hex variants are a compile-time feature variant — b903e = colored
//   cracks, f0adff = no-output. Gated on WATER_ICE_CRACKS_COLOR (I).

#ifndef WATER_ICE_CRACKS_COLOR
#define WATER_ICE_CRACKS_COLOR 1
#endif

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/math.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/sh.hlsl"
#include "inc/perlin.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 water_mask_dims;
    float4x4 model_to_world;
};

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    o.position = mul(view_constants[current_view_index].world_to_sample, world);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    o.C2 = world.xyz;
    return o;
}

#if WATER_ICE_CRACKS_COLOR
struct PsIn
{
    float3 C0 : TEXCOORD0;
};

uint2 ps_main(PsIn vin) : SV_Target0
{
    float3 up_color = (mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz * 0.5f) + 0.5f;
    float3 c = sqrt(clamp(vin.C0 * 0.4000000059604644775390625f, 0.0f, 1.0f));
    uint albedo = (uint((clamp(c.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                + (uint((clamp(c.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                + (uint((clamp(c.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 out_px = uint2(
        ((uint((clamp(up_color.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
        + (uint((clamp(up_color.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
        + (uint((clamp(up_color.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 1291845632u,
        albedo);
    out_px.y = albedo | 100663296u;
    return out_px;
}
#else
struct PsInEmpty
{
    float3 C0 : TEXCOORD0;
};

void ps_main(PsInEmpty vin)
{
}
#endif
