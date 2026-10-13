# dof-aa-a reconstruction decisions

Date: 2026-10-13. A-side of dual DoF + TAA/TGSR/FSR reconstruction (B build). 21 files under
`reconstruction/shader-source/candidates/dof-aa-a/shaders/` (dof/ 11 cs + 3 inc, flat 7).

## Hard gate (bindings + entry + LocalSize + capabilities)

22/26 bins PASS. Residuals (dxc-version / compile-time variant):
- `analyze`: extra `ComputeDerivativeGroupQuadsKHR` (dxc 1.10 quad-read; original only
  GroupNonUniformQuad).
- `tgsr_blur_lum`: missing `StorageImageWriteWithoutFormat` (R8 image; cap 56 not forceable).
- `tgsr` b303/f7ef (LocalSize 8x4): file is the 8x8 primary; thread-Y is a compile-time variant.

## Labels

O: clear_halfres, fsr_copy_history, fsr_init_reactive, fullres_upsample, init_buffers (verbatim).
O/I: analyze/autofocus/fused/halfres/fullres/lod0/lod1/taa/tgsr/tgsr_reproject/tgsr_blur_lum
(structure from SPIR-V; helper division + names Inference). U: multires.inc, upsample_halfres.inc,
tgsr_common.inc (inlined into consumers; kept empty).

## Histogram

All bins converge to documented residual classes: +1 OpSource/OpString (main file compiled from
temp path), OpLine layout, Load-coordinate Bitcast/Extract/Shuffle drift. Normalized hashes
differ (dxc 16ceb560 vs c4d8f4f9).

## Inc issues

1. `inc/coc.hlsl` `ring_samples[16]` collides with DoF `groupshared ring_samples`; consumers
   renamed to `dof_ring_samples`.
2. `inc/blue_noise.hlsl` pattern (golden-ratio) != DoF inlined `(px+13+uint2(int(frame*59),
   int(frame*37)))&255`; consumers inline it.
3. `inc/coc.hlsl` `DofFocusReadback.focus_lock_request`; bin embeds `lock_reponse`.
