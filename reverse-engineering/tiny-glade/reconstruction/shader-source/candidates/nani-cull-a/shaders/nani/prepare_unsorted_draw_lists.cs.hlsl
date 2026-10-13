// ==RECONSTRUCTED== shaders/nani/prepare_unsorted_draw_lists.cs.hlsl | nani-cull-a
// Observation: _nani_prepare_unsorted_draw_lists.cs.hlsl_cs_main_0.bin (LocalSize
// 64,1,1; t13 atlas_part_info (MeshAtlasPartInfo), u14 __alias_draw_list_buffer
// (DrawIndexedIndirectCommand); push_constants {world_subset,list_draw_cmd_offset}).
// Pre-seeds the unsorted draw list with one DrawIndexedIndirectCommand per atlas
// entry of the world subset: {index_count, 0, first_index, 0, 0}.
#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math.hlsl"
#include "atlas_part_info.inc.hlsl"
#include "draw_lists.inc.hlsl"

struct PushConstants
{
    uint world_subset;
    uint list_draw_cmd_offset;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

StructuredBuffer<MeshAtlasPartInfo> atlas_part_info : register(t13, space0);
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_list_buffer : register(u14, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    uint cmd_idx = dispatchThreadID.x + push_constants.list_draw_cmd_offset;
    MeshAtlasPartInfo part = atlas_part_info[push_constants.world_subset * 1024u + dispatchThreadID.x];
    DrawIndexedIndirectCommand cmd;
    cmd.index_count = part.index_count;
    cmd.instance_count = 0u;
    cmd.first_index = part.first_index;
    cmd.vertex_offset = 0u;
    cmd.first_instance = 0u;
    __alias_draw_list_buffer[cmd_idx] = cmd;
}
