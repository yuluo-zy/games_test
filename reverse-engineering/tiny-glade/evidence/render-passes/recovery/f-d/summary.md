# F-D summary — depth/shadow/blur/prefix_scan/lut/rdi/ssgi HLSL reconstruction

Date: 2026-10-13. Owner: F-D. 31 shader sources (35 modules) reconstructed from compiled
SPIR-V under `reconstruction/shader-source/shaders/` and validated with
`tools/render-passes/recompile_check.py` (structural fingerprint + normalized sha256).

## Validation
- **Bindings + entry + LocalSize: match 35/35 modules** (all names/sets/registers verbatim).
- **Full structural PASS: 1/35** — `fill_buffer.cs.hlsl`.
- Residual diffs on the other 34 are dominated by the dxc toolchain-version gap
  (installed c4d8f4f9, generator 0x000e vs game 16ceb560, 0x000f): target-only
  `RuntimeDescriptorArray`+`SPV_EXT_descriptor_indexing`, `OpExtension`,
  debug `OpLine/OpSource/OpString/OpNoLine/OpSourceContinued`, and
  `OpBitcast/OpExtInstImport/OpConstantNull/OpSwitch` codegen patterns; plus
  `RWTexture2D<float3>` storage-image caps (game R11fG11fB10f/ExtendedFormats vs
  installed Unknown/Read-Without-Format). A few files keep small structural diffs
  (branch/loop shape: gauss_blur family `IEqual` vs `INotEqual+LogicalNot`,
  prefix_scan ByteAddressBuffer address math, rdi/ssgi control flow) — recorded per
  module in `per-shader-results.json`.

## Top unknowns
- SPD kernels (`generate_hzb`, `generate_mips`): GroupNonUniform global-atomic
  counter layout and 4-chain address math are O (disassembly); large
  OpBitcast/CompositeExtract deltas remain (bilateral of version codegen).
- `rdi/temporal`, `primary_rt_debug`, `apply_screen_shadows/volumetrics`, `ssgi/*`:
  inlined helper function names are Inference (dxc drops OpName); bodies match the
  SPIR-V arithmetic.

## Inc-layer issues (flagged; inc/* unmodified)
- `inc/samplers.hlsl` pins `sampler_lnc` at s0; `rev_blur` binds it at s3 ->
  shader declares its own sampler.
- `inc/history_valid.hlsl` function `history_valid()` collides with the verbatim
  `$Globals` member `history_valid` (worked around in consumers; also
  `pack_unpack->bilinear->history_valid` edge is spurious).
- `inc/current_view.hlsl` global creates a second `$Globals` cbuffer; `inc/math.hlsl`
  and `tracked_buffer.hlsl` pull headers not embedded in the original include set
  (inflates OpString); `inc/ray_cone.hlsl` is still a skeleton.
