// ==SKELETON==
// SOURCE PATH : shaders/dof/upsample_halfres.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN (as include, 3 bins):
//   _dof_fullres.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin, _dof_fullres.cs.hlsl_cs_main_72a2a95d73e37edc.bin, _dof_fullres_upsample.cs.hlsl_cs_main_0.bin
// ==RECONSTRUCTED== shaders/dof/upsample_halfres.inc.hlsl | dof-aa-a | status: U (helper division Inference — the spatial-resolve upsample loop is
//   inlined identically in _dof_fullres and _dof_fullres_upsample; kept here as a shared comment
//   because each consumer declares its own cbuffer/registers, so no live declarations can be shared)
//
// Half-res spatial-resolve upsample. Both consumers embed the same 8-tap loop:
//   subpixel = pixel & 1;  index = subpixel.x + subpixel.y * 2;
//   acc = sum(halfres_tex.Load((pixel / 2) + spatial_resolve_offsets[index * 16 + i].xy)) for i in 0..7
//   out = (acc * 0.125) / max(1e-10, acc.w)

#ifndef TINYGLADE_DOF_UPSAMPLE_HALFRES_INC_HLSL
#define TINYGLADE_DOF_UPSAMPLE_HALFRES_INC_HLSL

#endif // TINYGLADE_DOF_UPSAMPLE_HALFRES_INC_HLSL
