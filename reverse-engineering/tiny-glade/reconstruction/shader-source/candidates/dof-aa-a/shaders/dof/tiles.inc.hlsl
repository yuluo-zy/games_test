// ==SKELETON==
// SOURCE PATH : shaders/dof/tiles.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN (as include, 14 bins):
//   _dof_analyze.cs.hlsl_cs_main_0.bin, _dof_clear_halfres.cs.hlsl_cs_main_0.bin, _dof_fullres.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin, _dof_fullres.cs.hlsl_cs_main_72a2a95d73e37edc.bin, _dof_fullres_all_lod0.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin, _dof_fullres_all_lod0.cs.hlsl_cs_main_72a2a95d73e37edc.bin, _dof_fullres_all_lod1.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin, _dof_fullres_all_lod1.cs.hlsl_cs_main_72a2a95d73e37edc.bin, _dof_fullres_upsample.cs.hlsl_cs_main_0.bin, _dof_halfres.cs.hlsl_cs_halfres_6f1dd5dd8a66e104.bin, _dof_halfres.cs.hlsl_cs_halfres_72a2a95d73e37edc.bin, _dof_halfres.cs.hlsl_cs_halfres_all_6f1dd5dd8a66e104.bin, _dof_halfres.cs.hlsl_cs_halfres_all_72a2a95d73e37edc.bin ...
// ==RECONSTRUCTED== shaders/dof/tiles.inc.hlsl | dof-aa-a | status: O (struct layouts from OpTypeStruct/OpMemberName across the 14 consumer bins)
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
