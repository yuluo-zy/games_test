// ==RECONSTRUCTED== shaders/read_terrain_stroke.cs.hlsl | evidence: spirv-cross of _read_terrain_stroke.cs.hlsl_cs_main_0.bin (query push constant -> stroke_indices sample -> terrain_stroke_index at byte 16 of out_buffer) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"

// World xz -> heightmap uv (1/130 world scale).
static const float TERRAIN_SAMPLE_SCALE = 0.0076923076994717121124267578125f;

// Query world position, pushed by the CPU.
struct PushConstants
{
    float2 query_position_xz;
};
[[vk::push_constant]]
PushConstants push_constants;

cbuffer $Globals : register(b13, space0)
{
    uint2 stroke_indices_dims;
};

struct CsOut
{
    float4 mouse_raycast_position;
    int terrain_stroke_index;
};

RWStructuredBuffer<CsOut> out_buffer : register(u15, space0);
Texture2D<int4> stroke_indices : register(t14, space0);

[numthreads(1, 1, 1)]
void cs_main()
{
    float3 query = (float3(push_constants.query_position_xz.x, 0.0f, push_constants.query_position_xz.y) * TERRAIN_SAMPLE_SCALE) + 0.5f;
    int2 pixel = int2(int(query.x * float(stroke_indices_dims.x)), int(query.z * float(stroke_indices_dims.y)));
    out_buffer[0].terrain_stroke_index = stroke_indices.Load(int3(pixel, 0)).x;
}
