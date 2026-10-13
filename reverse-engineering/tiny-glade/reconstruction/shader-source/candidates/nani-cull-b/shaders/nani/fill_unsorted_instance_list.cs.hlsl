// ==RECONSTRUCTED== shaders/nani/fill_unsorted_instance_list.cs.hlsl | evidence: raw disasm of
// _nani_fill_unsorted_instance_list.cs.hlsl_cs_main_0.bin: per filtered instance, histogram slot
// then write packed instance into the sorted buffer and bump the draw cmd instance count | status: O

#line 1 "shaders/nani/fill_unsorted_instance_list.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "draw_lists.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "draw_list_offsets.inc.hlsl"

StructuredBuffer<uint> thread_count_buffer : register(t14, space0);
StructuredBuffer<GpuFilteredInstance> filtered_instance_buffer : register(t15, space0);
StructuredBuffer<GpuFilteredInstanceMeta> filtered_instance_meta_buffer : register(t16, space0);
RWStructuredBuffer<uint> __alias_sorted_instances_buffer : register(u17, space0);
RWStructuredBuffer<uint> __alias_histogram : register(u18, space0);
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_list_buffer : register(u19, space0);

struct PushConstants
{
    uint draw_list_index;
    uint list_instance_offset;
    uint list_draw_cmd_offset;
};

[[vk::push_constant]]
PushConstants push_constants;

[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    if (thread_id.x >= thread_count_buffer[push_constants.draw_list_index])
    {
        return;
    }

    uint instance = thread_id.x + push_constants.list_instance_offset;
    GpuFilteredInstance filtered = filtered_instance_buffer[instance];
    GpuFilteredInstanceMeta meta = filtered_instance_meta_buffer[instance];

    uint slot;
    InterlockedAdd(__alias_histogram[push_constants.draw_list_index * 1024u + (filtered.raw % 1024u)], 1u, slot);
    __alias_sorted_instances_buffer[slot + push_constants.list_instance_offset] = (filtered.raw / 1024u) * 64u + meta.raw;
    InterlockedAdd(__alias_draw_list_buffer[meta.raw + push_constants.list_draw_cmd_offset].instance_count, 1u);
}
