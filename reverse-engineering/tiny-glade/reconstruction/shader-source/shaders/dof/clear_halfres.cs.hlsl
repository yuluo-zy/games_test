// ==RECONSTRUCTED== shaders/dof/clear_halfres.cs.hlsl | A (dof-aa review, 2026-10-13: tie, include form) + Rgba16f fix: img_output RWTexture2D<half4> (target format operand Rgba16f, same buffer halfres writes).
//   Clears the half-res color output for the tile list (group offsets[6] from DofTileOffsets).

#include "inc/common.hlsl"
#include "dof/tiles.inc.hlsl"

StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t15, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t16, space0);
RWTexture2D<half4> img_output : register(u14, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 dispatch_id : SV_DispatchThreadID)
{
    uint tile_base = tile_offsets_buffer[0].offsets[6] + group_id.x;
    uint2 tile_origin = uint2(8u, 8u) * tile_coords_buffer[tile_base];
    img_output[(dispatch_id.xy & uint2(7u, 7u)) + tile_origin] = float4(0.0f, 0.0f, 0.0f, 0.0f);
}
