// ==RECONSTRUCTED== shaders/nani/generate_draw_lists.cs.hlsl | evidence: raw disasm of
// _nani_generate_draw_lists.cs.hlsl_cs_main_0.bin: builds draw commands from per-bucket instance
// counts (mesh_histogram), emitting DrawIndexedIndirectCommand per non-empty mesh bucket | status: O

#line 1 "shaders/nani/generate_draw_lists.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "draw_lists.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "draw_list_offsets.inc.hlsl"
#include "atlas_part_info.inc.hlsl"

StructuredBuffer<MeshAtlasPartInfo[64]> atlas_part_info : register(t14, space0);
StructuredBuffer<uint> sorted_instances_buffer : register(t15, space0);
StructuredBuffer<uint> thread_count_buffer : register(t16, space0);
StructuredBuffer<uint> thread_group_draw_cmd_histogram : register(t17, space0);
StructuredBuffer<uint> thread_group_draw_cmd_subtotals : register(t18, space0);
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_list_buffer : register(u19, space0);
RWStructuredBuffer<uint> __alias_instance_indirection_buffer : register(u20, space0);
RWStructuredBuffer<uint> __alias_instance_indirection_allocator_buffer : register(u21, space0);

struct PushConstants
{
    uint world_subset;
    uint draw_list_index;
    uint list_draw_cmd_offset;
    uint list_merge_window_offset;
    uint list_merge_window_subtotal_offset;
};

[[vk::push_constant]]
PushConstants push_constants;

groupshared uint mesh_histogram[64];
groupshared uint group_draw_offset;

[numthreads(32, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 thread_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    for (uint i = local_index; i < 64u; i += 32u)
    {
        mesh_histogram[i] = 0u;
    }

    if (local_index == 0u)
    {
        group_draw_offset = thread_group_draw_cmd_histogram[push_constants.list_merge_window_offset + group_id.x] +
                            thread_group_draw_cmd_subtotals[push_constants.list_merge_window_subtotal_offset + (group_id.x / 1024u)];
    }

    uint count = thread_count_buffer[push_constants.draw_list_index];
    bool in_range = thread_id.x < count;
    uint instance_index = in_range ? sorted_instances_buffer[thread_id.x + instance_list_offset_at(push_constants.draw_list_index)] : 0u;
    uint mesh_id = instance_index / 64u;
    uint bucket = instance_index % 64u;

    GroupMemoryBarrierWithGroupSync();
    if (in_range)
    {
        InterlockedAdd(mesh_histogram[bucket], 1u);
    }
    GroupMemoryBarrierWithGroupSync();

    for (uint j = local_index; j < 64u; j += 32u)
    {
        uint bucket_count = mesh_histogram[j];
        if (bucket_count > 0u)
        {
            uint draw_cmd;
            InterlockedAdd(group_draw_offset, 1u, draw_cmd);
            uint cmd_index = draw_cmd + push_constants.list_draw_cmd_offset;
            uint base_instance;
            InterlockedAdd(__alias_instance_indirection_allocator_buffer[push_constants.draw_list_index], bucket_count, base_instance);
            DrawIndexedIndirectCommand cmd;
            cmd.index_count = atlas_part_info[push_constants.world_subset][j].index_count;
            cmd.instance_count = bucket_count;
            cmd.first_index = atlas_part_info[push_constants.world_subset][j].first_index;
            cmd.vertex_offset = 0u;
            cmd.first_instance = base_instance;
            __alias_draw_list_buffer[cmd_index] = cmd;
            mesh_histogram[j] = base_instance;
        }
    }
    GroupMemoryBarrierWithGroupSync();

    if (in_range)
    {
        uint slot;
        InterlockedAdd(mesh_histogram[bucket], 1u, slot);
        __alias_instance_indirection_buffer[slot] = mesh_id;
    }
}
