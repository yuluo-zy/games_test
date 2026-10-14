// ============================================================
// nani/fill_unsorted_instance_list.cs.hlsl
// 功能:把经 GPU 筛选后的实例,按直方图桶散列写入"未排序实例列表"缓冲,
//       并同步累加对应绘制命令的 instanceCount。
// 入口:cs_main(计算着色器,64 线程/工作组)。
// 数据流:
//   filtered_instance_buffer(筛选结果) -> 按 raw%1024 直方图桶
//   -> sorted_instances_buffer(打包实例) + draw_list_buffer.instance_count++。
// ============================================================

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

// 每张绘制列表已筛选出的实例总数(按绘制列表索引读取)。
StructuredBuffer<uint> thread_count_buffer : register(t14, space0);
// 筛选后的实例数据(打包的 GpuFilteredInstance)。
StructuredBuffer<GpuFilteredInstance> filtered_instance_buffer : register(t15, space0);
// 筛选后的实例元数据(打包的 GpuFilteredInstanceMeta)。
StructuredBuffer<GpuFilteredInstanceMeta> filtered_instance_meta_buffer : register(t16, space0);
// 未排序实例列表缓冲(按直方图桶写入)。
RWStructuredBuffer<uint> __alias_sorted_instances_buffer : register(u17, space0);
// 直方图:每张绘制列表 1024 个桶,统计各桶已写入数量。
RWStructuredBuffer<uint> __alias_histogram : register(u18, space0);
// 绘制命令缓冲,累加 instanceCount。
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_list_buffer : register(u19, space0);

// 推入常量:当前绘制列表及该列表在全局缓冲中的实例/绘制命令偏移。
struct PushConstants
{
    uint draw_list_index;      // 绘制列表索引
    uint list_instance_offset; // 本列表实例在全局实例缓冲中的起始偏移
    uint list_draw_cmd_offset; // 本列表绘制命令在全局命令缓冲中的起始偏移
};

[[vk::push_constant]]
PushConstants push_constants;

[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    // 越界线程直接退出(本列表的实例可能不满一个线程束)。
    if (thread_id.x >= thread_count_buffer[push_constants.draw_list_index])
    {
        return;
    }

    // 取出该线程负责的实例及其元数据。
    uint instance = thread_id.x + push_constants.list_instance_offset;
    GpuFilteredInstance filtered = filtered_instance_buffer[instance];
    GpuFilteredInstanceMeta meta = filtered_instance_meta_buffer[instance];

    // 以 filtered.raw 的低 10 位作为直方图桶号,原子递增取得桶内写入槽位。
    uint slot;
    InterlockedAdd(__alias_histogram[push_constants.draw_list_index * 1024u + (filtered.raw % 1024u)], 1u, slot);
    // 打包实例写入桶内槽位(高 22 位 + 元数据),保证同一桶内顺序稳定。
    __alias_sorted_instances_buffer[slot + push_constants.list_instance_offset] = (filtered.raw / 1024u) * 64u + meta.raw;
    // 累加对应绘制命令的实例数。
    InterlockedAdd(__alias_draw_list_buffer[meta.raw + push_constants.list_draw_cmd_offset].instance_count, 1u);
}
