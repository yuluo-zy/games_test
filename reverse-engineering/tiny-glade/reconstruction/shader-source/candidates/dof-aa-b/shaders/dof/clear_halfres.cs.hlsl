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
//     index_pass   : W49 (Inference)
//     names        : img_output, DofTileOffsets, tile_offsets_buffer, tile_coords_buffer, cs_main
//
// ==RECONSTRUCTED== shaders/dof/clear_halfres.cs.hlsl
// Evidence: _dof_clear_halfres.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23, bound 58).
//   bindings: img_output u14 (StorageImage 2D), tile_offsets_buffer t15
//   (StructuredBuffer<DofTileOffsets>), tile_coords_buffer t16
//   (StructuredBuffer<uint2>); LocalSize [8,8,1], entry cs_main.
//   spirv-cross: img_output[(id.xy & 7) + (8,8)*tile_coords_buffer.Load<uint2>(
//   (tile_offsets_buffer.Load<uint>(24) + groupID.x)*8)] = 0.  (Per-group tile
//   clear: 8x8 thread tile at the coords table entry indexed by offsets[6].)
// Status: O (structure from disassembly).

#include "inc/common.hlsl"
#include "tiles.inc.hlsl"

RWTexture2D<float4> img_output : register(u14, space0);
StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t15, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t16, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
             uint3 gl_WorkGroupID : SV_GroupID)
{
    uint2 tile_coords = tile_coords_buffer[tile_offsets_buffer[0].offsets[6] + gl_WorkGroupID.x];
    img_output[(gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords)] = 0.0f;
}
