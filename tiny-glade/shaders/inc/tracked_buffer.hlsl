// ======================================================================
// inc/tracked_buffer.hlsl —— 计数追踪的字节地址缓冲追加辅助
// ======================================================================
// 用途:消费者(楼梯、网格生成器)先用 GPU 原子保留一个槽位,再写记录元素。
// 来源:OpLine 归属(_rocky_terrain_stairs_stairs、_garden_marching_squares、
//       _dof_autofocus、_dof_analyze、_nani_group_draw_cmd_prefix_sum):
//       AccessChain + CompositeInsert + Store + Load + ShiftRightLogical 混合,
//       即带追踪计数器的结构化记录写入(如楼梯:InterlockedAdd(draw_commands,1)
//       后按记录元素 Store<float>)。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/tracked_buffer.hlsl | evidence: OpLine attribution (consumers _rocky_terrain_stairs_stairs, _garden_marching_squares, _dof_autofocus, _dof_analyze, _nani_group_draw_cmd_prefix_sum): AccessChain + CompositeInsert + Store + Load + ShiftRightLogical mix — structured-record writes with a tracked atomic counter (e.g. stairs: InterlockedAdd(draw_commands,1) then Store<float> per record element); include-graph edge tracked_buffer.hlsl -> smooth_max.hlsl | status: I/U (helper names/body Inference — inlined; exact record layout is per-consumer, only the reserve+store pattern is observable)
//
// Helpers for appending packed records into a byte-address buffer while
// tracking the written count with a GPU atomic. Consumers (stairs, mesh
// generators) reserve a slot, then store record elements.

#ifndef TINYGLADE_INC_TRACKED_BUFFER_HLSL
#define TINYGLADE_INC_TRACKED_BUFFER_HLSL

// 原子保留追踪缓冲中的下一个槽位。`counter` 在 `counter_offset` 字节处保存
// 当前计数;返回旧值(要写入的槽索引),并把递增后的计数留给绘制派发使用。
uint tracked_buffer_reserve(RWByteAddressBuffer counter, uint counter_offset)
{
    uint previous;
    counter.InterlockedAdd(counter_offset, 1u, previous);
    return previous;
}

// 在 `base_byte_offset + element_index*4` 处存储一条记录的 float 元素。
void tracked_buffer_store_float(RWByteAddressBuffer buffer, uint base_byte_offset, uint element_index, float value)
{
    buffer.Store<float>(base_byte_offset + element_index * 4u, value);
}

#endif // TINYGLADE_INC_TRACKED_BUFFER_HLSL
