# F-R (rocky/paths/stairs) reconstruction summary

Date: 2026-10-13. Family F-R owns `paths/`, `rocky_terrain/` (incl. `stairs/`), and the flat
`path_pebbles.cs.hlsl`, `path_pebble.raster.hlsl`, `stairs_foliage_exclusion.raster.hlsl`.
All 15 bins reconstructed. Method per file: extract SPIR-V modules (length-prefixed container),
spirv-cross --hlsl scaffolding + clean disasm, then behavior-equivalent HLSL 2021 / SM 6.4 using
the read-only `inc/` layer; validated with `tools/render-passes/recompile_check.py`
(structural fingerprint: entry/stage, LocalSize, capabilities, descriptor bindings, opcode histogram).

## Anchors (binding + entry + LocalSize + capabilities)

Every module matches the target on bindings, entry, LocalSize and capabilities
(`diff[bindings]/[entry]/[local_size]/[capabilities]` empty) — verified per module in
`per-shader-results.json`. Capability parity required forcing `SPV_EXT_descriptor_indexing` +
`RuntimeDescriptorArray` (`[[vk::ext_capability(5302)]]`) on the StructuredBuffer SSBOs (path_pebbles,
rocky_terrain.raster, stairs_indirect.raster, stairs.cs) and `RWTexture2D<half>` for R16f storage
images (combine, directional, path_mask_visual) — the original dxc build emitted these.

## Full fingerprint (opcode histogram)

The opcode histogram does NOT match exactly for any consumer. Residual diffs are two known, bounded
artifact classes (recorded per module in `per-shader-results.json`):

1. Inc-layer include-graph drift (not consumer-fixable, `inc/*` is read-only): reconstructed
   `inc/tracked_buffer.hlsl -> smooth_max.hlsl` and `inc/math.hlsl -> bicubic/quad_mesh/unjitter_taa`
   (+ image/curve) are not present in the original closures, adding OpSource/OpString/OpLine to every
   consumer. FLAGGED for the inc owners.
2. dxc-version codegen (original commit 16ceb560 vs installed 1.10.2605.37/c4d8f4f9):
   `Load(int3(p,0))` coordinate construction adds OpCompositeConstruct/Extract/VectorShuffle/OpBitcast;
   the original's do-while(false)/switch/phi control flow compiles to a flatter form under the current
   dxc; FMul-vs-VectorTimesScalar and constant-splat choices differ.

Normalized hashes therefore never equal (expected toolchain-version signal).

## Files

- `paths/combine_path_exclusions.cs.hlsl`, `paths/directional_contract.cs.hlsl`,
  `paths/path_mask_sampler.inc.hlsl`, `paths/path_mask_visual.cs.hlsl`
- `rocky_terrain/displace_rocky_terrain.cs.hlsl`, `rocky_terrain/rocky_terrain.raster.hlsl`,
  `rocky_terrain/inc_mesh.inc.hlsl`, `rocky_terrain/triangle_adjacency.inc.hlsl`
- `rocky_terrain/stairs/stairs.cs.hlsl`, `stairs_counter.cs.hlsl`, `stairs_indirect.raster.hlsl`,
  `isolines.inc.hlsl`, `marching_squares.inc.hlsl`, `place_stairs.inc.hlsl`
- `path_pebbles.cs.hlsl`, `path_pebble.raster.hlsl`, `stairs_foliage_exclusion.raster.hlsl`

## Inc issues flagged

- `inc/draw_indirect_readonly.hlsl` `StairsCounter{uint,pad*3}` is wrong for `stairs_counter` (the bin
  embeds `StairsCounter{int count}`, stride 4); the include was skipped there to avoid the name clash.
- Extra include edges (tracked_buffer->smooth_max, math.hlsl->bicubic/quad_mesh/unjitter_taa) inflate
  consumer debug-source opcodes.

## Unknowns

Function names for inlined helpers (path/mask samplers, scatter grid, marching-squares case table)
are Inference; `stairs.cs` contouring exact iso-height/interpolation constants and the `terrain_height`
dependency of `find_root.hlsl` are best-effort from the inlined SPIR-V.
