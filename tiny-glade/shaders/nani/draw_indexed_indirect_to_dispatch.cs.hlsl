// ==RECONSTRUCTED== shaders/nani/draw_indexed_indirect_to_dispatch.cs.hlsl
// 依据:_nani_draw_indexed_indirect_to_dispatch.cs.hlsl_cs_main_0.bin 的原始反汇编:
// 单线程把一个 DrawIndexedIndirectCommand 桶转换为 uint4 分发参数
// (ceil(instanceCount/group_size),外加 instanceCount 作为第 4 个词)
// 状态:O

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
    // 取指定桶的实例数,向上取整为工作组倍数后作为第 1 个分发词,
    // 原始实例数作为第 4 个词供后续 indirect 使用。
    uint instance_count = draw_buffer[push_constants.bucket].instanceCount;
    args_buffer[0] = uint4((instance_count + push_constants.group_size - 1u) / push_constants.group_size, 1u, 1u, instance_count);
}
