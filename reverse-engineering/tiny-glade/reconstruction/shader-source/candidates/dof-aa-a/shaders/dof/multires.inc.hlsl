// ==SKELETON==
// SOURCE PATH : shaders/dof/multires.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN (as include, 10 bins):
//   _dof_fullres.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin, _dof_fullres.cs.hlsl_cs_main_72a2a95d73e37edc.bin, _dof_fullres_all_lod0.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin, _dof_fullres_all_lod0.cs.hlsl_cs_main_72a2a95d73e37edc.bin, _dof_fullres_all_lod1.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin, _dof_fullres_all_lod1.cs.hlsl_cs_main_72a2a95d73e37edc.bin, _dof_halfres.cs.hlsl_cs_halfres_6f1dd5dd8a66e104.bin, _dof_halfres.cs.hlsl_cs_halfres_72a2a95d73e37edc.bin, _dof_halfres.cs.hlsl_cs_halfres_all_6f1dd5dd8a66e104.bin, _dof_halfres.cs.hlsl_cs_halfres_all_72a2a95d73e37edc.bin
// ==RECONSTRUCTED== shaders/dof/multires.inc.hlsl | dof-aa-a | status: U (helper division Inference — all bodies inlined into consumers; the tile-based passes
//   share their coc/ring/gather code which dxc fully inlines, so this header carries no live
//   declarations in the compiled modules)
//
// Multiresolution DoF helpers for the tile-indirect passes (halfres / fullres / fullres_all_lod*).
// Every helper body observed in consumers is inlined, so this header only contributes the
// OpSource/OpString debug-source entries; keep it empty to avoid adding code to the consumers.

#ifndef TINYGLADE_DOF_MULTIRES_INC_HLSL
#define TINYGLADE_DOF_MULTIRES_INC_HLSL

#endif // TINYGLADE_DOF_MULTIRES_INC_HLSL
