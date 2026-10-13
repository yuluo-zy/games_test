// ==SKELETON==
// SOURCE PATH : shaders/dof/tiles.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN (as include, 14 bins):
//   _dof_analyze/_dof_clear_halfres/_dof_fullres(_all_lod0/_all_lod1)/_dof_fullres_upsample/_dof_halfres(_all)/_dof_init_buffers
//
// ==RECONSTRUCTED== shaders/dof/tiles.inc.hlsl
// Evidence: _dof_clear_halfres and _dof_init_buffers bins embed this include.
//   DofTileOffsets { uint offsets[8] } (32B, member_names "offsets",
//   OpTypeArray uint[8]) is the per-tile offset table read by analyze/clear_halfres/
//   fullres_upsample (StructuredBuffer<DofTileOffsets>).  DofTileOffsetsPacked
//   { uint4 offsets[2] } (32B) is the packed initial-value form written by
//   init_buffers (ConstantBuffer<DofTileOffsetsPacked> + RWStructuredBuffer).
//   Status: O (struct layouts from member_names/debug_names across 14 bins).

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
