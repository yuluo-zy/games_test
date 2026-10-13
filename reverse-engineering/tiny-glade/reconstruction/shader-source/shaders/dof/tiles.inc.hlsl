// ==RECONSTRUCTED== shaders/dof/tiles.inc.hlsl | A/B tie (dof-aa review, 2026-10-13): identical struct content.
//
// Tile-list bookkeeping shared by the tile-based DoF passes.
//   DofTileOffsets     - 8 uint offsets into tile_coords_buffer, one per dispatch group
//                        (analyze writes prefix-sums of 7 near/far groups; consumers read
//                        slots 0..6: fullres=0, fullres_all_lod0=1, fullres_all_lod1=2,
//                        fullres_upsample=3, halfres=4, halfres_all=5, clear_halfres=6).
//   DofTileOffsetsPacked - the packed (uint4[2]) form used by init_buffers.

#ifndef TINYGLADE_DOF_TILES_INC_HLSL
#define TINYGLADE_DOF_TILES_INC_HLSL

struct DofTileOffsets
{
    uint offsets[8];
};

struct DofTileOffsetsPacked
{
    uint4 offsets[2];
};

#endif // TINYGLADE_DOF_TILES_INC_HLSL
