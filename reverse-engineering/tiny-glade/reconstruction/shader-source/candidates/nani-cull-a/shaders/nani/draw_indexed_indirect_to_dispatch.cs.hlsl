// ==RECONSTRUCTED== shaders/nani/draw_indexed_indirect_to_dispatch.cs.hlsl | nani-cull-a
// Observation: _nani_draw_indexed_indirect_to_dispatch.cs.hlsl_cs_main_0.bin
// (LocalSize 1,1,1; t13 StructuredBuffer<DrawIndexedIndirectCommand> draw_buffer,
// u14 RWStructuredBuffer<uint4> args_buffer, push_constants {bucket, group_size}).
// Converts one indexed-indirect draw into a vkCmdDispatchIndirect args vector:
// thread group count = ceil(instanceCount / group_size), instanceCount passes
// through as the 4th component.
#include "inc/common.hlsl"
#include "inc/draw_indirect_types.hlsl"

struct PushConstants
{
    uint bucket;
    uint group_size;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

StructuredBuffer<DrawIndexedIndirectCommand> draw_buffer : register(t13, space0);
RWStructuredBuffer<uint4> args_buffer : register(u14, space0);

[numthreads(1, 1, 1)]
void cs_main()
{
    uint instance_count = draw_buffer[push_constants.bucket].instanceCount;
    args_buffer[0] = uint4((instance_count + push_constants.group_size - 1u) / push_constants.group_size,
                           1u, 1u, instance_count);
}
