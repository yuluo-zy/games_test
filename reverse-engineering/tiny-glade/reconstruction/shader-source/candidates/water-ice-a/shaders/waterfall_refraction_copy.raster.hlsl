// ==SKELETON== -> ==RECONSTRUCTED== (water-ice-a)
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
#include "water/waterfall_vert.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 refraction_source_tex_dims;
};

ByteAddressBuffer instance_buffer : register(t14, space0);

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID, uint instance_id : SV_InstanceID)
{
    VsOut o;
    WaterfallMesh m = load_waterfall_mesh(instance_buffer, instance_id);
    float2 grid = float2(float(vertex_id % 2u), float(vertex_id / 2u)) * float2(1.0f, 0.100000001490116119384765625f);
    float time = float(game_time_sec) + game_time_frac;
    float3 position;
    float3 normal;
    float3 world_pos;
    waterfall_vert(m, grid, time, position, normal, world_pos);
    o.position = mul(view_constants[current_view_index].world_to_sample, float4(position, 1.0f));
    o.C0 = normal;
    o.C1 = position;
    return o;
}

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
