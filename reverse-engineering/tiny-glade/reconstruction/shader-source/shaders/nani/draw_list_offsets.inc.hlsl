// ==RECONSTRUCTED== shaders/nani/draw_list_offsets.inc.hlsl | evidence: raw disasm of
// _nani_clear_draw_lists / _nani_allocate_draw_commands / _nani_fill_* / _nani_generate_*:
// OpMemberName instance_list_offset_array (uint4[27], b13 UBO) and a helper that reads
// instance_list_offset_array[index/4][index%4]; OpLine attribution (clear_draw_lists) pins
// the div/mod/load inside this file | status: O (cbuffer layout, helper operation); I (name)

#ifndef TINYGLADE_NANI_DRAW_LIST_OFFSETS_HLSL
#define TINYGLADE_NANI_DRAW_LIST_OFFSETS_HLSL

// Per-draw-list base offset into the global sorted/allocator instance buffers.
// 108 draw lists = 27 uint4 lanes; one draw list per lane, lane = index % 4.
cbuffer instance_list_offset : register(b13, space0)
{
    uint4 instance_list_offset_array[27];
};

// Base offset (into the filtered/allocator instance buffers) for a draw list.
uint instance_list_offset_at(uint draw_list_index)
{
    uint packed_index = draw_list_index / 4u;
    uint lane = draw_list_index % 4u;
    return instance_list_offset_array[packed_index][lane];
}

#endif // TINYGLADE_NANI_DRAW_LIST_OFFSETS_HLSL
