// ==RECONSTRUCTED== shaders/nani/allocate_draw_commands.cs.hlsl | evidence: raw disasm of
// _nani_allocate_draw_commands.cs.hlsl_cs_main_0.bin: groupshared mesh_histogram[64]+group_draw_count,
// per instance bucket by (instance%64), compact histogram to non-zero bucket count | status: O
//
// nani 分配绘制命令:统计本线程组内各实例桶(mesh_histogram)的非零桶数,
// 作为该线程组的绘制命令数量写进直方图。

#line 1 "shaders/nani/allocate_draw_commands.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "draw_lists.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "draw_list_offsets.inc.hlsl"

StructuredBuffer<uint> sorted_instances_buffer : register(t14, space0);
StructuredBuffer<uint> thread_count_buffer : register(t15, space0);
RWStructuredBuffer<uint> __alias_thread_group_draw_cmd_histogram : register(u16, space0);

struct PushConstants
{
    uint draw_list_index;
    uint histogram_offset;
};

[[vk::push_constant]]
PushConstants push_constants;

groupshared uint mesh_histogram[64];
groupshared uint group_draw_count;

[numthreads(32, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 thread_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    // 清零组共享直方图与计数。
    for (uint i = local_index; i < 64u; i += 32u)
    {
        mesh_histogram[i] = 0u;
    }
    if (local_index == 0u)
    {
        group_draw_count = 0u;
    }

    uint count = thread_count_buffer[push_constants.draw_list_index];
    bool in_range = thread_id.x < count;
    uint instance_index = in_range ? sorted_instances_buffer[thread_id.x + instance_list_offset_at(push_constants.draw_list_index)] : 0u;
    uint bucket = instance_index % 64u;

    GroupMemoryBarrierWithGroupSync();
    if (in_range)
    {
        // 按实例索引 % 64 分桶计数。
        InterlockedAdd(mesh_histogram[bucket], 1u);
    }
    GroupMemoryBarrierWithGroupSync();

    // 统计非零桶的数量 = 该线程组需要的绘制命令数。
    for (uint j = local_index; j < 64u; j += 32u)
    {
        if (mesh_histogram[j] > 0u)
        {
            InterlockedAdd(group_draw_count, 1u);
        }
    }
    GroupMemoryBarrierWithGroupSync();

    if (local_index == 0u)
    {
        __alias_thread_group_draw_cmd_histogram[push_constants.histogram_offset + group_id.x] = group_draw_count;
    }
}
