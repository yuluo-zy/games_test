// ==RECONSTRUCTED== shaders/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl | evidence:
// raw disasm of _nani_prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl_cs_main_0.bin: per
// 64-draw-cmd group, exclusive scan of instance_count, then rewrite first_instance with the
// draw-list offset; atlas-entry prefix buffer written | status: O

#line 1 "shaders/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "draw_lists.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "draw_list_offsets.inc.hlsl"
#include "inc/prefix_scan.hlsl"

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_list_buffer : register(u14, space0);
RWStructuredBuffer<uint> __alias_unsorted_instance_atlas_entry_prefix_buffer : register(u15, space0);

struct PushConstants
{
    uint draw_list_index;
    uint list_draw_cmd_offset;
};

[[vk::push_constant]]
PushConstants push_constants;

groupshared uint shared_data1x[64];

[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    uint cmd_index = thread_id.x + push_constants.list_draw_cmd_offset;
    DrawIndexedIndirectCommand cmd = __alias_draw_list_buffer[cmd_index];
    uint lane = thread_id.x % 64u;
    shared_data1x[lane] = cmd.instance_count;
    GroupMemoryBarrierWithGroupSync();

    uint d = (lane >> 0u) << 1u;
    if ((d + 1u) < 64u)
    {
        shared_data1x[d + 1u] += shared_data1x[d];
    }
    GroupMemoryBarrierWithGroupSync();
    d = (lane >> 1u) << 2u;
    if ((d + 2u + (lane & 1u)) < 64u)
    {
        shared_data1x[d + 2u + (lane & 1u)] += shared_data1x[d + 1u];
    }
    GroupMemoryBarrierWithGroupSync();
    d = (lane >> 2u) << 3u;
    if ((d + 4u + (lane & 3u)) < 64u)
    {
        shared_data1x[d + 4u + (lane & 3u)] += shared_data1x[d + 3u];
    }
    GroupMemoryBarrierWithGroupSync();
    d = (lane >> 3u) << 4u;
    if ((d + 8u + (lane & 7u)) < 64u)
    {
        shared_data1x[d + 8u + (lane & 7u)] += shared_data1x[d + 7u];
    }
    GroupMemoryBarrierWithGroupSync();
    d = (lane >> 4u) << 5u;
    if ((d + 16u + (lane & 15u)) < 64u)
    {
        shared_data1x[d + 16u + (lane & 15u)] += shared_data1x[d + 15u];
    }
    GroupMemoryBarrierWithGroupSync();
    d = (lane >> 5u) << 6u;
    if ((d + 32u + (lane & 31u)) < 64u)
    {
        shared_data1x[d + 32u + (lane & 31u)] += shared_data1x[d + 31u];
    }
    GroupMemoryBarrierWithGroupSync();

    uint prefix = (lane == 0u) ? 0u : shared_data1x[lane - 1u];
    uint first_instance = prefix + instance_list_offset_at(push_constants.draw_list_index);
    cmd.first_instance = first_instance;
    __alias_draw_list_buffer[cmd_index] = cmd;
    __alias_unsorted_instance_atlas_entry_prefix_buffer[thread_id.x + (push_constants.draw_list_index * 64u)] = first_instance;
}
