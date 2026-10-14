// ==RECONSTRUCTED== shaders/nani/fill_sorted_instance_list.cs.hlsl
// 依据:_nani_fill_sorted_instance_list.cs.hlsl_cs_main_0.bin 的原始反汇编:
// 每线程在直方图槽下把一个过滤后实例去重写入排序缓冲;推进 draw_list_index/list_instance_offset
// 状态:O

#line 1 "shaders/nani/fill_sorted_instance_list.cs.hlsl"

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

struct PushConstants
{
    uint draw_list_index;
    uint list_instance_offset;
};

[[vk::push_constant]]
PushConstants push_constants;

[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    uint count = thread_count_buffer[push_constants.draw_list_index];
    if (thread_id.x >= count)
    {
        return;
    }

    uint instance = thread_id.x + push_constants.list_instance_offset;
    GpuFilteredInstance filtered = filtered_instance_buffer[instance];
    GpuFilteredInstanceMeta meta = filtered_instance_meta_buffer[instance];

    // 解包过滤后索引:mesh_id = raw/1024,sub_id = raw%1024。
    uint mesh_id = filtered.raw / 1024u;
    uint sub_id = filtered.raw % 1024u;
    // 在子网格直方图槽上原子占位,得到排序缓冲槽位。
    uint slot;
    InterlockedAdd(__alias_histogram[push_constants.draw_list_index * 1024u + sub_id], 1u, slot);
    // 写入 mesh_id*64 + 实例在子网格内的序号。
    __alias_sorted_instances_buffer[slot + push_constants.list_instance_offset] = mesh_id * 64u + meta.raw;
}
