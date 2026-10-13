// ==RECONSTRUCTED== shaders/inc/wall_color_ids.hlsl | evidence: spirv-cross (_nani_gothic_window_bricks VS/_gi_trace) ByteAddressBuffer wall_color_ids indexed by brick/triangle id (Load<uint>(id*4)); include-graph edge wall_color_ids.hlsl -> reservoir.hlsl | status: O (buffer access); O/I (helper names Inference)

#ifndef TINYGLADE_INC_WALL_COLOR_IDS_HLSL
#define TINYGLADE_INC_WALL_COLOR_IDS_HLSL

#include "reservoir.hlsl"

// Wall/brick palette color id for a brick (or triangle) index.
uint get_wall_color_id(ByteAddressBuffer wall_color_ids, uint index)
{
    return wall_color_ids.Load<uint>(index * 4u + 0u);
}

#endif // TINYGLADE_INC_WALL_COLOR_IDS_HLSL
