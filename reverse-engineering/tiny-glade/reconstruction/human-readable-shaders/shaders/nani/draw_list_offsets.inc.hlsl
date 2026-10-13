// ==RECONSTRUCTED== shaders/nani/draw_list_offsets.inc.hlsl
// 依据:_nani_clear_draw_lists / _nani_allocate_draw_commands / _nani_fill_* / _nani_generate_*
// 的原始反汇编:OpMemberName instance_list_offset_array (uint4[27], b13 UBO) 与一个读取
// instance_list_offset_array[index/4][index%4] 的辅助函数;OpLine 归属(clear_draw_lists)
// 把 div/mod/load 固定在本文件内。
// 状态:O (cbuffer 布局,辅助操作);I (函数名)

#ifndef TINYGLADE_NANI_DRAW_LIST_OFFSETS_HLSL
#define TINYGLADE_NANI_DRAW_LIST_OFFSETS_HLSL

// 每个绘制列表在全局排序/分配实例缓冲中的基准偏移。
// 108 个绘制列表 = 27 个 uint4 通道;每条通道一个绘制列表,通道 = index % 4。
cbuffer instance_list_offset : register(b13, space0)
{
    uint4 instance_list_offset_array[27];
};

// 某绘制列表在(过滤/分配实例缓冲中的)基准偏移。
uint instance_list_offset_at(uint draw_list_index)
{
    uint packed_index = draw_list_index / 4u;
    uint lane = draw_list_index % 4u;
    return instance_list_offset_array[packed_index][lane];
}

#endif // TINYGLADE_NANI_DRAW_LIST_OFFSETS_HLSL
