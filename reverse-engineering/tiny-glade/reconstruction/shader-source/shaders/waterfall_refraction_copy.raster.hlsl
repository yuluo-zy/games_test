// ==RECONSTRUCTED== shaders/waterfall_refraction_copy.raster.hlsl
// SOURCE PATH : shaders/waterfall_refraction_copy.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _waterfall_refraction_copy.raster.hlsl_b903e43ffb3da915.bin (ps copies refraction_source_tex)
//   _waterfall_refraction_copy.raster.hlsl_f0adff76827e73ff.bin (empty ps)
// entries: vs:vs_main  ps:ps_main | index_pass W44 (refraction_copy_pass)
// Status: O (bindings/entry; vs + ps from spirv-cross)
// Evidence: spirv-cross vs (waterfall_vert.inc per-instance displacement + 3D
//   value-noise; C0=vertex normal, C1=world) and ps (b903e: copy
//   refraction_source_tex.Load(fragCoord).xyz; f0adff: empty).
// MERGE (water-ice review waterfall_refraction_copy.raster.hlsl): A adopted wholesale.
//   A has the procedural VS the bin embeds; vs_main + instance_buffer (t14) now live in
//   the shared water/waterfall_vert.inc.hlsl (bin layout).
// Note: two 16-hex variants gated on WATERFALL_REFRACTION_COPY (b903e=copy, f0adff=empty).

#ifndef WATERFALL_REFRACTION_COPY
#define WATERFALL_REFRACTION_COPY 1
#endif

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/perlin.hlsl"
#include "inc/hash.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 refraction_source_tex_dims;
};

// vs_main (uses $Globals.current_view_index + instance_buffer) lives here.
#include "water/waterfall_vert.inc.hlsl"

#if WATERFALL_REFRACTION_COPY
struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
};

Texture2D<float4> refraction_source_tex : register(t15, space0);

float4 ps_main(PsIn vin, float4 position : SV_Position) : SV_Target0
{
    return float4(refraction_source_tex.Load(int3(uint2(position.xy), 0u)).xyz, 1.0f);
}
#else
struct PsInEmpty
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
};

void ps_main(PsInEmpty vin)
{
}
#endif
