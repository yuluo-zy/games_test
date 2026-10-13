# dof-aa-b reconstruction decisions (2026-10-13)

Independent B-side reconstruction of the DoF + temporal AA family under
`candidates/dof-aa-b/shaders/` (14 dof/ + 7 flat files, 21 sources).

**Hard gate (bindings/entry/LocalSize/caps): PASS 15/18 validated bins.** All 3
remaining cap diffs are documented dxc-version residuals, not reconstruction
errors: analyze (QuadReadAcrossX/Y pulls `ComputeDerivativeGroupQuadsKHR` +
SPV_KHR_compute_shader_derivatives in dxc 1.10.2605.37; the original 16ceb560
emits only GroupNonUniformQuad), and fused/tgsr_blur_lum (single R11g11b10f / R8
storage image: our dxc emits StorageImageExtendedFormats but not
StorageImageWriteWithoutFormat).

**Opcode histogram**: converges within ≤3 rounds for the small/medium files
(code opcodes match for fsr_copy_history/fsr_init_reactive/clear_halfres/
fullres_upsample; taa/tgsr caps match). The bokeh gather files (halfres/fullres/
fused/all_lod0/all_lod1) carry a transcription residual (OpLoad ~+45 from the
packed ring-table unpack; OpBitwiseOr/ShiftLeft differences).

**Residuals (recorded, not chased):**
- OpSource/OpString +1: original build compiles from a temp unity wrapper with a
  `#line`-style real-path reference; our dxc embeds one fewer (wave-1-consistent).
- Frexp: curve.hlsl `incb_frexp` workaround emits bit-ops where the original used
  OpExtInst frexp (documented tool limitation, RECOVERY_TRACKER).
- init_buffers: dxc copies the ConstantBuffer struct member-wise vs the target's
  whole-struct load.

**Inc issues flagged**: reconstructed `inc/coc.hlsl` defines `ring_samples` which
collides with the halfres/fullres/fused shaders' own `groupshared uint
ring_samples[64][N]` — the original coc.hlsl evidently did not export that symbol;
B-side renamed its groupshared to `dof_ring_samples` to compile.
