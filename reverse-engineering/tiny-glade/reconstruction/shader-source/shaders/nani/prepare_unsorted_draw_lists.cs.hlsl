// ==RECONSTRUCTED== shaders/nani/prepare_unsorted_draw_lists.cs.hlsl | evidence: raw disasm of
// _nani_prepare_unsorted_draw_lists.cs.hlsl_cs_main_0.bin: writes one DrawIndexedIndirectCommand
// per atlas part (index_count/first_index from atlas, instance_count/vertex_offset/first_instance=0) | status: O

#line 1 "shaders/nani/prepare_unsorted_draw_lists.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "draw_lists.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "atlas_part_info.inc.hlsl"

StructuredBuffer<MeshAtlasPartInfo[64]> atlas_part_info : register(t13, space0);
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_list_buffer : register(u14, space0);

struct PushConstants
{
    uint world_subset;
    uint list_draw_cmd_offset;
};

[[vk::push_constant]]
PushConstants push_constants;

[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    uint cmd_index = thread_id.x + push_constants.list_draw_cmd_offset;
    DrawIndexedIndirectCommand cmd;
    cmd.index_count = atlas_part_info[push_constants.world_subset][thread_id.x].index_count;
    cmd.instance_count = 0u;
    cmd.first_index = atlas_part_info[push_constants.world_subset][thread_id.x].first_index;
    cmd.vertex_offset = 0u;
    cmd.first_instance = 0u;
    __alias_draw_list_buffer[cmd_index] = cmd;
}
