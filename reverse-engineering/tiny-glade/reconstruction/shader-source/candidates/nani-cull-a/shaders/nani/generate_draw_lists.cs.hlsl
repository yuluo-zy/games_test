// ==RECONSTRUCTED== shaders/nani/generate_draw_lists.cs.hlsl | nani-cull-a
// Observation: _nani_generate_draw_lists.cs.hlsl_cs_main_0.bin (LocalSize 32,1,1;
// b13 instance_list_offset; t14 atlas_part_info, t15 sorted_instances_buffer,
// t16 thread_count_buffer, t17 thread_group_draw_cmd_histogram, t18
// thread_group_draw_cmd_subtotals; u19 __alias_draw_list_buffer (DrawIndexedIndirectCommand),
// u20 __alias_instance_indirection_buffer, u21 __alias_instance_indirection_allocator_buffer;
// push_constants {world_subset,draw_list_index,list_draw_cmd_offset,
// list_merge_window_offset,list_merge_window_subtotal_offset}; groupshared
// mesh_histogram[64] + group_draw_offset). Merges the per-workgroup draw-command
// counts into a global draw list: each non-empty atlas-entry bucket allocates one
// command (indirection allocator), writes the command, and fills the instance
// indirection list for its surviving instances.
#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "atlas_part_info.inc.hlsl"
#include "draw_lists.inc.hlsl"
#include "draw_list_offsets.inc.hlsl"

struct PushConstants
{
    uint world_subset;
    uint draw_list_index;
    uint list_draw_cmd_offset;
    uint list_merge_window_offset;
    uint list_merge_window_subtotal_offset;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

StructuredBuffer<MeshAtlasPartInfo> atlas_part_info : register(t14, space0);
StructuredBuffer<uint> sorted_instances_buffer : register(t15, space0);
StructuredBuffer<uint> thread_count_buffer : register(t16, space0);
StructuredBuffer<uint> thread_group_draw_cmd_histogram : register(t17, space0);
StructuredBuffer<uint> thread_group_draw_cmd_subtotals : register(t18, space0);
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_list_buffer : register(u19, space0);
RWStructuredBuffer<uint> __alias_instance_indirection_buffer : register(u20, space0);
RWStructuredBuffer<uint> __alias_instance_indirection_allocator_buffer : register(u21, space0);

groupshared uint mesh_histogram[64];
groupshared uint group_draw_offset;

[numthreads(32, 1, 1)]
void cs_main(uint3 groupID : SV_GroupID, uint3 dispatchThreadID : SV_DispatchThreadID, uint groupIndex : SV_GroupIndex)
{
    for (uint i = groupIndex; i < 64u; i += 32u)
    {
        mesh_histogram[i] = 0u;
    }
    if (groupIndex == 0u)
    {
        group_draw_offset = thread_group_draw_cmd_histogram[push_constants.list_merge_window_offset + groupID.x]
                          + thread_group_draw_cmd_subtotals[push_constants.list_merge_window_subtotal_offset + groupID.x / 1024u];
    }

    uint count = thread_count_buffer[push_constants.draw_list_index];
    bool valid = dispatchThreadID.x < count;
    uint sorted_val = valid ? sorted_instances_buffer[dispatchThreadID.x + instance_list_offset_array[push_constants.draw_list_index / 4u][push_constants.draw_list_index % 4u]] : 0u;
    uint atlas_entry = sorted_val / 64u;
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
        uint hist = mesh_histogram[j];
        if (hist > 0u)
        {
            uint cmd_slot;
            InterlockedAdd(group_draw_offset, 1u, cmd_slot);
            uint cmd_idx = cmd_slot + push_constants.list_draw_cmd_offset;

            uint first_instance;
            InterlockedAdd(__alias_instance_indirection_allocator_buffer[push_constants.draw_list_index], hist, first_instance);

            MeshAtlasPartInfo part = atlas_part_info[push_constants.world_subset * 1024u + j];
            DrawIndexedIndirectCommand cmd;
            cmd.index_count = part.index_count;
            cmd.instance_count = hist;
            cmd.first_index = part.first_index;
            cmd.vertex_offset = 0u;
            cmd.first_instance = first_instance;
            __alias_draw_list_buffer[cmd_idx] = cmd;

            mesh_histogram[j] = first_instance;
        }
    }
    GroupMemoryBarrierWithGroupSync();

    if (valid)
    {
        uint prev;
        InterlockedAdd(mesh_histogram[bucket], 1u, prev);
        __alias_instance_indirection_buffer[prev] = atlas_entry;
    }
}
