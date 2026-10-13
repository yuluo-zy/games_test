// ==RECONSTRUCTED== Renders ivy as a strip of quads (two vertices per IvySegment). Each vertex is
// SOURCE PATH : shaders/ivy/ivy.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _ivy_ivy.raster.hlsl_b903e43ffb3da915.bin
//     file_sha256  : 32ad6deb2f72678062e964e8dba685f0c4cdedcf73ec6874f1bf1e07c4b5d092
//     pass/stage   : _ivy_ivy (vegetation_grass) / raster
//     entry        : vs:vs_main; ps:ps_main
//     variant      : b903e43ffb3da915
//     local_size   : -
//     names        : sampler_default, ViewConstants, frame_constants, $Globals, IvySegment, ivy_buffer, terrain_heightmap, vs_main, ps_main
//   _ivy_ivy.raster.hlsl_f0adff76827e73ff.bin
//     file_sha256  : 3397837a783f6ca96bfb1139905858c02c32590c91e962e1c6b3c22e2bc12a32
//     pass/stage   : _ivy_ivy (vegetation_grass) / raster
//     entry        : vs:vs_main; ps:ps_main
//     variant      : f0adff76827e73ff
//     local_size   : -
//     names        : sampler_default, ViewConstants, frame_constants, $Globals, IvySegment, ivy_buffer, terrain_heightmap, vs_main, ps_main
// Renders ivy as a strip of quads (two vertices per IvySegment). Each vertex is
// snapped to the terrain heightmap, and the pass-through "thickness" (fractional
// part of the segment thickness) is rasterized into TEXCOORD0 so the fragment
// can shade the ivy as a solid gray scaled by thickness.

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
Texture2D<float4> terrain_heightmap : register(t15, space0);
#include "inc/terrain_heightmap.inc.hlsl"

// Reproduce the original shared 13-sampler $Globals layout (set0/b13).

// Per-shader $Globals members (implicit $Globals UBO, set0/b13).
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;

struct IvySegment
{
    float3 start_pos;
    float start_thickness;
    float3 end_pos;
    float end_thickness;
};

StructuredBuffer<IvySegment> ivy_buffer : register(t14, space0);

struct VSOutput
{
    float4 position : SV_Position;
    float thickness : TEXCOORD0;
};

VSOutput vs_main(uint vid : SV_VertexID)
{
    VSOutput output;
    uint segment = vid / 2u;
    float3 position;
    if ((vid % 2u) == 0u)
    {
        position = ivy_buffer[segment].start_pos;
        output.thickness = frac(ivy_buffer[segment].start_thickness);
    }
    else
    {
        position = ivy_buffer[segment].end_pos;
        output.thickness = frac(ivy_buffer[segment].end_thickness);
    }
    float2 uv = position.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx;
    float height = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv, 0.0f).x);
    ViewConstants view = GetCurrentViewConstants();
    output.position = mul(view.world_to_sample,
                          float4(position.x, position.y + height, position.z, 1.0f));
    return output;
}

struct PSInput
{
    float thickness : TEXCOORD0;
};

float4 ps_main(PSInput input) : SV_Target0
{
    return float4(input.thickness, input.thickness, input.thickness, 1.0f);
}
