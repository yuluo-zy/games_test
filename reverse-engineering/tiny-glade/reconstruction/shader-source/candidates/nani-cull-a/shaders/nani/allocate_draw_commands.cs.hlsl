// ==RECONSTRUCTED== shaders/nani/allocate_draw_commands.cs.hlsl | nani-cull-a
// Observation: _nani_allocate_draw_commands.cs.hlsl_cs_main_0.bin (LocalSize
// 32,1,1; b13 instance_list_offset; t14 sorted_instances_buffer, t15
// thread_count_buffer; u16 __alias_thread_group_draw_cmd_histogram; push_constants
// {draw_list_index,histogram_offset}; groupshared mesh_histogram[64] +
// group_draw_count). Per workgroup, buckets the workgroup's sorted instances
// into a 64-entry shared histogram and stores the count of non-empty buckets
// (the workgroup's draw-command count) into the per-group histogram slot.
#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "draw_lists.inc.hlsl"
#include "draw_list_offsets.inc.hlsl"

struct PushConstants
{
    uint draw_list_index;
    uint histogram_offset;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

StructuredBuffer<uint> sorted_instances_buffer : register(t14, space0);
StructuredBuffer<uint> thread_count_buffer : register(t15, space0);
RWStructuredBuffer<uint> __alias_thread_group_draw_cmd_histogram : register(u16, space0);

groupshared uint mesh_histogram[64];
groupshared uint group_draw_count;

[numthreads(32, 1, 1)]
void cs_main(uint3 groupID : SV_GroupID, uint3 dispatchThreadID : SV_DispatchThreadID, uint groupIndex : SV_GroupIndex)
{
    for (uint i = groupIndex; i < 64u; i += 32u)
    {
        mesh_histogram[i] = 0u;
    }
    if (groupIndex == 0u)
    {
        group_draw_count = 0u;
    }

    uint count = thread_count_buffer[push_constants.draw_list_index];
    bool valid = dispatchThreadID.x < count;
    uint sorted_val = valid ? sorted_instances_buffer[dispatchThreadID.x + instance_list_offset_array[push_constants.draw_list_index / 4u][push_constants.draw_list_index % 4u]] : 0u;
    uint bucket = sorted_val % 64u;

    GroupMemoryBarrierWithGroupSync();
    if (valid)
    {
        uint unused;
        InterlockedAdd(mesh_histogram[bucket], 1u, unused);
    }
    GroupMemoryBarrierWithGroupSync();

    for (uint j = groupIndex; j < 64u; j += 32u)
    {
        if (mesh_histogram[j] > 0u)
        {
            uint unused;
            InterlockedAdd(group_draw_count, 1u, unused);
        }
    }
    GroupMemoryBarrierWithGroupSync();

    if (groupIndex == 0u)
    {
        __alias_thread_group_draw_cmd_histogram[push_constants.histogram_offset + groupID.x] = group_draw_count;
    }
}
