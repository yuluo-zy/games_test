// ==RECONSTRUCTED== shaders/nani/draw_indexed_indirect_to_dispatch.cs.hlsl | evidence: raw
// disasm of _nani_draw_indexed_indirect_to_dispatch.cs.hlsl_cs_main_0.bin: single thread turns
// a DrawIndexedIndirectCommand bucket into a uint4 dispatch arg (ceil(instanceCount/group_size),
// plus instanceCount as 4th word) | status: O

#line 1 "shaders/nani/draw_indexed_indirect_to_dispatch.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/draw_indirect_types.hlsl"

StructuredBuffer<DrawIndexedIndirectCommand> draw_buffer : register(t13, space0);
RWStructuredBuffer<uint4> args_buffer : register(u14, space0);

struct PushConstants
{
    uint bucket;
    uint group_size;
};

[[vk::push_constant]]
PushConstants push_constants;

[numthreads(1, 1, 1)]
void cs_main()
{
    uint instance_count = draw_buffer[push_constants.bucket].instanceCount;
    args_buffer[0] = uint4((instance_count + push_constants.group_size - 1u) / push_constants.group_size, 1u, 1u, instance_count);
}
