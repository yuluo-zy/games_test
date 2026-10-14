// ============================================================
// nani/generate_draw_lists.cs.hlsl
// 功能:根据每个网格桶的实例计数(mesh_histogram),为每个非空桶生成一条
//       DrawIndexedIndirectCommand(索引/实例来自图集),并维护实例间接缓冲。
// 入口:cs_main,32 线程/工作组。
// 绑定:atlas_part_info (t14)、sorted_instances_buffer (t15)、
//       thread_count_buffer (t16)、thread_group_draw_cmd_histogram (t17)、
//       thread_group_draw_cmd_subtotals (t18)、draw_list_buffer (u19)、
//       instance_indirection_buffer (u20)、instance_indirection_allocator_buffer (u21)。
// ============================================================

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

// 图集部件信息(索引数/首索引等)。
StructuredBuffer<MeshAtlasPartInfo[64]> atlas_part_info : register(t14, space0);
// 已排序实例缓冲。
StructuredBuffer<uint> sorted_instances_buffer : register(t15, space0);
// 每张绘制列表的实例总数。
StructuredBuffer<uint> thread_count_buffer : register(t16, space0);
// 线程组级绘制命令直方图与累加小计。
StructuredBuffer<uint> thread_group_draw_cmd_histogram : register(t17, space0);
StructuredBuffer<uint> thread_group_draw_cmd_subtotals : register(t18, space0);
// 输出绘制命令缓冲。
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_list_buffer : register(u19, space0);
// 实例间接索引缓冲与分配器。
RWStructuredBuffer<uint> __alias_instance_indirection_buffer : register(u20, space0);
RWStructuredBuffer<uint> __alias_instance_indirection_allocator_buffer : register(u21, space0);

struct PushConstants
{
    uint world_subset;             // 图集子集索引
    uint draw_list_index;          // 绘制列表索引
    uint list_draw_cmd_offset;     // 本列表绘制命令在全局缓冲中的起始偏移
    uint list_merge_window_offset; // 合并窗口的直方图偏移
    uint list_merge_window_subtotal_offset; // 合并窗口的小计偏移
};

[[vk::push_constant]]
PushConstants push_constants;

// 每个网格桶的实例计数(64 个桶)。
groupshared uint mesh_histogram[64];
// 本线程组已分配的绘制命令计数。
groupshared uint group_draw_offset;

[numthreads(32, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 thread_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    // 初始化共享直方图。
    for (uint i = local_index; i < 64u; i += 32u)
    {
        mesh_histogram[i] = 0u;
    }

    // 计算本线程组绘制命令的全局起始偏移(直方图前缀 + 合并窗口小计)。
    if (local_index == 0u)
    {
        group_draw_offset = thread_group_draw_cmd_histogram[push_constants.list_merge_window_offset + group_id.x] +
                            thread_group_draw_cmd_subtotals[push_constants.list_merge_window_subtotal_offset + (group_id.x / 1024u)];
    }

    // 取出本线程负责的实例;越界则用 0 占位。
    uint count = thread_count_buffer[push_constants.draw_list_index];
    bool in_range = thread_id.x < count;
    uint instance_index = in_range ? sorted_instances_buffer[thread_id.x + instance_list_offset_at(push_constants.draw_list_index)] : 0u;
    // 实例索引拆分:高 6 位为网格 id,低 6 位为网格内桶号。
    uint mesh_id = instance_index / 64u;
    uint bucket = instance_index % 64u;

    GroupMemoryBarrierWithGroupSync();
    // 统计每个桶的实例数。
    if (in_range)
    {
        InterlockedAdd(mesh_histogram[bucket], 1u);
    }
    GroupMemoryBarrierWithGroupSync();

    // 对每个非空桶生成一条绘制命令。
    for (uint j = local_index; j < 64u; j += 32u)
    {
        uint bucket_count = mesh_histogram[j];
        if (bucket_count > 0u)
        {
            // 原子分配一条绘制命令。
            uint draw_cmd;
            InterlockedAdd(group_draw_offset, 1u, draw_cmd);
            uint cmd_index = draw_cmd + push_constants.list_draw_cmd_offset;
            // 原子分配本桶实例的起始实例号。
            uint base_instance;
            InterlockedAdd(__alias_instance_indirection_allocator_buffer[push_constants.draw_list_index], bucket_count, base_instance);
            // 组装绘制命令:索引数/首索引取自图集,实例数为本桶计数。
            DrawIndexedIndirectCommand cmd;
            cmd.index_count = atlas_part_info[push_constants.world_subset][j].index_count;
            cmd.instance_count = bucket_count;
            cmd.first_index = atlas_part_info[push_constants.world_subset][j].first_index;
            cmd.vertex_offset = 0u;
            cmd.first_instance = base_instance;
            __alias_draw_list_buffer[cmd_index] = cmd;
            // 复用直方图槽记录本桶的起始实例号,供下阶段回写间接索引。
            mesh_histogram[j] = base_instance;
        }
    }
    GroupMemoryBarrierWithGroupSync();

    // 为每个实例回写其网格 id 到间接索引缓冲。
    if (in_range)
    {
        uint slot;
        InterlockedAdd(mesh_histogram[bucket], 1u, slot);
        __alias_instance_indirection_buffer[slot] = mesh_id;
    }
}
