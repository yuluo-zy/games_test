// ============================================================
// nani/prepare_unsorted_draw_lists.cs.hlsl
// 功能:为每个图集部件(atlas part)写一条初始绘制命令:
//       index_count/first_index 取自图集,instance_count/vertex_offset/
//       first_instance 置 0。后续阶段再填充实例数。
// 入口:cs_main,64 线程/工作组,每线程处理一个图集部件。
// 绑定:atlas_part_info (t13)、draw_list_buffer (u14)。
// ============================================================

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

// 图集部件信息(索引数/首索引等)。
StructuredBuffer<MeshAtlasPartInfo[64]> atlas_part_info : register(t13, space0);
// 输出绘制命令缓冲。
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_list_buffer : register(u14, space0);

struct PushConstants
{
    uint world_subset;       // 图集子集索引
    uint list_draw_cmd_offset; // 本列表绘制命令在全局缓冲中的起始偏移
};

[[vk::push_constant]]
PushConstants push_constants;

[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    // 目标绘制命令索引。
    uint cmd_index = thread_id.x + push_constants.list_draw_cmd_offset;
    DrawIndexedIndirectCommand cmd;
    // 索引数与首索引取自图集;实例数、顶点偏移、首实例先置 0。
    cmd.index_count = atlas_part_info[push_constants.world_subset][thread_id.x].index_count;
    cmd.instance_count = 0u;
    cmd.first_index = atlas_part_info[push_constants.world_subset][thread_id.x].first_index;
    cmd.vertex_offset = 0u;
    cmd.first_instance = 0u;
    __alias_draw_list_buffer[cmd_index] = cmd;
}
