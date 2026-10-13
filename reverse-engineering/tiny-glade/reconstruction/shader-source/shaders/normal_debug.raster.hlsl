// ==RECONSTRUCTED== shaders/normal_debug.raster.hlsl | F-D
// Evidence: _normal_debug.raster.hlsl_b903e43ffb3da915.bin (vs_main + ps_main).
//   bindings: $Globals b13 (current_view_index), debug_normals_buffer b14,
//   frame_constants set1/b0. NormalDebug = { position, normal } (32-byte records).
//   spirv-cross: vs draws an instanced "normal debug" line (vertex 1 adds the
//   record's normal to its position), transformed by world_to_sample of the current
//   view; ps outputs a constant red.
// Status: O (structure from disassembly).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
};

struct NormalDebug
{
    float4 position;
    float4 normal;
};

StructuredBuffer<NormalDebug> debug_normals_buffer : register(t14, space0);

struct VSOut
{
    float4 position : SV_Position;
};

VSOut vs_main(uint vertexIndex : SV_VertexID, uint instanceIndex : SV_InstanceID)
{
    VSOut output;
    float3 p = debug_normals_buffer[instanceIndex].position.xyz;
    if (vertexIndex == 1u)
    {
        p += debug_normals_buffer[instanceIndex].normal.xyz;
    }
    output.position = mul(view_constants[current_view_index].world_to_sample, float4(p, 1.0f));
    return output;
}

float4 ps_main() : SV_Target0
{
    return float4(1.0f, 0.0f, 0.0f, 1.0f);
}
