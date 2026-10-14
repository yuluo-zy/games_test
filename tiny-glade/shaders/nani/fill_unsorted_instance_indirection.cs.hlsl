// ==RECONSTRUCTED== shaders/nani/fill_unsorted_instance_indirection.cs.hlsl
// 依据:_nani_fill_unsorted_instance_indirection.cs.hlsl_cs_main_0.bin 的原始反汇编:
// 每个实例把 mesh id(instance/64)写入 atlas 条目前缀槽下的间接缓冲
// 状态:O

#line 1 "shaders/nani/fill_unsorted_instance_indirection.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "draw_lists.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "draw_list_offsets.inc.hlsl"

StructuredBuffer<uint> sorted_instances_buffer : register(t14, space0);
StructuredBuffer<uint> thread_count_buffer : register(t15, space0);
RWStructuredBuffer<uint> __alias_unsorted_instance_atlas_entry_prefix_buffer : register(u16, space0);
RWStructuredBuffer<uint> __alias_instance_indirection_buffer : register(u17, space0);

struct PushConstants
{
    uint draw_list_index;
};

[[vk::push_constant]]
PushConstants push_constants;

[numthreads(32, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    // 取出当前绘制列表内、本线程负责的排序后实例索引。
    uint instance = sorted_instances_buffer[thread_id.x + instance_list_offset_at(push_constants.draw_list_index)];
    if (thread_id.x < thread_count_buffer[push_constants.draw_list_index])
    {
        // 在该 atlas 条目(instance%64)的前缀槽原子占位,得到间接缓冲槽位,
        // 写入 mesh id = instance/64。
        uint slot;
        InterlockedAdd(__alias_unsorted_instance_atlas_entry_prefix_buffer[(instance % 64u) + (push_constants.draw_list_index * 64u)], 1u, slot);
        __alias_instance_indirection_buffer[slot] = instance / 64u;
    }
}
