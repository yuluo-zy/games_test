// ==SKELETON==
// SOURCE PATH : shaders/dof/clear_halfres.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins):
//   _dof_clear_halfres.cs.hlsl_cs_main_0.bin
//     file_sha256  : d47553444054f67f1f9b35bede3cef987034840f82da28cabb81a95b1a87462c
//     pass/stage   : _dof_clear_halfres (dof) / cs
//     entry        : cs:cs_main
//     variant      : 0
//     local_size   : [8, 8, 1]
//     names        : img_output, DofTileOffsets, tile_offsets_buffer, tile_coords_buffer, cs_main
// ==RECONSTRUCTED== shaders/dof/clear_halfres.cs.hlsl | dof-aa-a | status: O (verbatim from SPIR-V)
//   Clears the half-res color output for the tile list (group offsets[6] from DofTileOffsets).

#include "inc/common.hlsl"
#include "dof/tiles.inc.hlsl"

StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t15, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t16, space0);
RWTexture2D<float4> img_output : register(u14, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 dispatch_id : SV_DispatchThreadID)
{
    uint tile_base = tile_offsets_buffer[0].offsets[6] + group_id.x;
    uint2 tile_origin = uint2(8u, 8u) * tile_coords_buffer[tile_base];
    img_output[(dispatch_id.xy & uint2(7u, 7u)) + tile_origin] = float4(0.0f, 0.0f, 0.0f, 0.0f);
}
