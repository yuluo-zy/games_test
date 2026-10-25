// ==RECONSTRUCTED== shaders/nani/fill_unsorted_instance_list.cs.hlsl | nani-cull-a
// Observation: _nani_fill_unsorted_instance_list.cs.hlsl_cs_main_0.bin (LocalSize
// 64,1,1; push_constants {draw_list_index,list_instance_offset,list_draw_cmd_offset};
// t14 thread_count_buffer, t15 filtered_instance_buffer, t16
// filtered_instance_meta_buffer, u17 __alias_sorted_instances_buffer, u18
// __alias_histogram, u19 RWStructuredBuffer<DrawIndexedIndirectCommand>
// __alias_draw_list_buffer). Sorted-list fill that additionally increments the
// per-draw-command instanceCount for the instance's atlas entry.
#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math.hlsl"
#include "draw_lists.inc.hlsl"
#include "draw_list_offsets.inc.hlsl"

struct PushConstants
{
    uint draw_list_index;
    uint list_instance_offset;
    uint list_draw_cmd_offset;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

StructuredBuffer<uint> thread_count_buffer : register(t14, space0);
StructuredBuffer<GpuFilteredInstance> filtered_instance_buffer : register(t15, space0);
StructuredBuffer<GpuFilteredInstanceMeta> filtered_instance_meta_buffer : register(t16, space0);
RWStructuredBuffer<uint> __alias_sorted_instances_buffer : register(u17, space0);
RWStructuredBuffer<uint> __alias_histogram : register(u18, space0);
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_list_buffer : register(u19, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    uint count = thread_count_buffer[push_constants.draw_list_index];
    if (dispatchThreadID.x >= count)
    {
        return;
    }
    uint slot = dispatchThreadID.x + push_constants.list_instance_offset;
    GpuFilteredInstance inst = filtered_instance_buffer[slot];
    GpuFilteredInstanceMeta meta_rec = filtered_instance_meta_buffer[slot];
    uint prev;
    InterlockedAdd(__alias_histogram[push_constants.draw_list_index * 1024u + inst.raw % 1024u], 1u, prev);
    __alias_sorted_instances_buffer[prev + push_constants.list_instance_offset] = (inst.raw / 1024u) * 64u + meta_rec.raw;

    uint unused;
    InterlockedAdd(__alias_draw_list_buffer[meta_rec.raw + push_constants.list_draw_cmd_offset].instance_count, 1u, unused);
}
