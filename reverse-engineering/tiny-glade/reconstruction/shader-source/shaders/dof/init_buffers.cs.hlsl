// ==RECONSTRUCTED== shaders/dof/init_buffers.cs.hlsl | B (dof-aa review, 2026-10-13: tie, slight B; whole-struct copy matches target OpStore count).
//   Seeds the per-tile dispatch-args and the initial DofTileOffsetsPacked values.

#include "inc/common.hlsl"
#include "dof/tiles.inc.hlsl"

struct PushConstants
{
    uint args_type_count;
};
[[vk::push_constant]]
PushConstants push_constants;

ConstantBuffer<DofTileOffsetsPacked> tile_offsets_initial_values : register(b13, space0);
RWStructuredBuffer<uint4> __alias_dispatch_args_buffer : register(u14, space0);
RWStructuredBuffer<DofTileOffsetsPacked> tile_offsets_buffer : register(u15, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x < push_constants.args_type_count)
    {
        __alias_dispatch_args_buffer[gl_GlobalInvocationID.x] = uint4(0u, 1u, 1u, 0u);
    }
    if (gl_GlobalInvocationID.x == 0u)
    {
        tile_offsets_buffer[0] = tile_offsets_initial_values;
    }
}
