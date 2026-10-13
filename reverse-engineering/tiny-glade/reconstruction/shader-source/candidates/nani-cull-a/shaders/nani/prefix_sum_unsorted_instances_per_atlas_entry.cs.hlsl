// ==RECONSTRUCTED== shaders/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl | nani-cull-a
// Observation: _nani_prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl_cs_main_0.bin
// (LocalSize 64,1,1; b13 instance_list_offset; u14
// RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_list_buffer; u15
// __alias_unsorted_instance_atlas_entry_prefix_buffer; push_constants
// {draw_list_index,list_draw_cmd_offset}; groupshared shared_data1x[64]).
// Per draw list, scans the 64 atlas-entry instance-counts of the unsorted draw
// commands (inclusive, 64-element Hillis-Steele) and rewrites each command's
// firstInstance to its prefix + the draw list base offset, recording the prefix
// for the later unsorted-instance indirection pass.
#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math.hlsl"
#include "inc/prefix_scan.hlsl"
#include "draw_lists.inc.hlsl"
#include "draw_list_offsets.inc.hlsl"

struct PushConstants
{
    uint draw_list_index;
    uint list_draw_cmd_offset;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_list_buffer : register(u14, space0);
RWStructuredBuffer<uint> __alias_unsorted_instance_atlas_entry_prefix_buffer : register(u15, space0);
groupshared uint shared_data1x[64];

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    uint cmd_idx = dispatchThreadID.x + push_constants.list_draw_cmd_offset;
    uint index_count = __alias_draw_list_buffer[cmd_idx].index_count;
    uint instance_count = __alias_draw_list_buffer[cmd_idx].instance_count;
    uint first_index = __alias_draw_list_buffer[cmd_idx].first_index;
    uint vertex_offset = __alias_draw_list_buffer[cmd_idx].vertex_offset;

    uint lane = dispatchThreadID.x % 64u;
    shared_data1x[lane] = instance_count;
    GroupMemoryBarrierWithGroupSync();

    [unroll]
    for (uint s = 0u; s < 6u; ++s)
    {
        uint d = 1u << s;
        uint block = (lane >> s) << (s + 1u);
        uint i = block + d + (lane & (d - 1u));
        if (i < 64u)
        {
            shared_data1x[i] += shared_data1x[block + d - 1u];
        }
        GroupMemoryBarrierWithGroupSync();
    }

    uint ex = lane == 0u ? 0u : shared_data1x[lane - 1u];
    uint first_instance = ex + instance_list_offset_array[push_constants.draw_list_index / 4u][push_constants.draw_list_index % 4u];
    __alias_draw_list_buffer[cmd_idx].index_count = index_count;
    __alias_draw_list_buffer[cmd_idx].instance_count = instance_count;
    __alias_draw_list_buffer[cmd_idx].first_index = first_index;
    __alias_draw_list_buffer[cmd_idx].vertex_offset = vertex_offset;
    __alias_draw_list_buffer[cmd_idx].first_instance = first_instance;
    __alias_unsorted_instance_atlas_entry_prefix_buffer[dispatchThreadID.x + push_constants.draw_list_index * 64u] = first_instance;
}
