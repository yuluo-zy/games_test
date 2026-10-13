// ==RECONSTRUCTED== shaders/nani/draw_list_offsets.inc.hlsl | nani-cull-a
// Observation: embedded as include in 8 nani compute bins. Provides the shared
// draw-list base-offset UBO `instance_list_offset` (b13/space0), a uint4[27]
// array (108 draw-list slots; 27 world subsets x 4). Member name recovered
// verbatim from OpMemberName (instance_list_offset_array).
#ifndef NANI_DRAW_LIST_OFFSETS_INC_HLSL
#define NANI_DRAW_LIST_OFFSETS_INC_HLSL

cbuffer instance_list_offset : register(b13, space0)
{
    uint4 instance_list_offset_array[27];
};

#endif // NANI_DRAW_LIST_OFFSETS_INC_HLSL
