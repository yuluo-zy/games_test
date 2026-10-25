# nani-cull-a decisions

A-side of the dual reconstruction of the nani GPU cull/draw-list chain (B build).

## Structure choices
- **Shared incs** (candidate tree, mirroring `shaders/`): `nani/draw_list_offsets.inc.hlsl`
  (`cbuffer instance_list_offset : register(b13) { uint4 instance_list_offset_array[27]; }`),
  `nani/draw_lists.inc.hlsl` (GpuFilteredInstance/GpuFilteredInstanceMeta `{uint raw}` +
  nani-family `DrawIndexedIndirectCommand` camelCase→lowercase fields),
  `nani/atlas_part_info.inc.hlsl` (MeshAtlasPartInfo, stride 16),
  `nani/cull_and_bucket.inc.hlsl` (NaniDrawListInfo + meta decode: `size=unpackHalf2x16`,
  `mesh_index=meta>>16`), `nani/inc/trimesh.hlsl`, `nani/inc/cull_and_bucket_owned_custom.hlsl`.
  Wall: `wall/inc/brick_instance.hlsl` (InstancedWallData stride 96), `brick_instance_flags.hlsl`
  (#defines — NOT static consts, so they never pollute the implicit `$Globals`),
  `wall/inc/draw_lists.hlsl` (brick draw-list bucket helper). Candidate tree carries a symlink
  `inc -> reconstruction/shader-source/shaders/inc` so recompile_check's single `-I` resolves both.
- **Bindings/semantics per shader** follow the raw-per-bin tables verbatim (u/t registers,
  push-constant members from OpMemberName).
- **Push constants** use `[[vk::push_constant]] ConstantBuffer<PushConstants>` (project convention);
  `$Globals` uses explicit `cbuffer $Globals : register(b14,space0)` (inc-fix #6 convention).

## Validation result (nani-cull-a)
- 15/15 nani compute shaders pass the hard gate (bindings/entry/LocalSize/capabilities).
  Histogram residuals are the documented dxc-version artifacts (OpBitcast in RWByteAddressBuffer
  address math, OpAccessChain-vs-CompositeExtract for single-member structs, OpSource/OpString/
  OpLine counts, OpExtInstImport, ternary→OpSwitch).
- wall reset_pass2: hard-gate PASS (residual: OpConstant 6v5, OpLine).
- wall generate_draw_lists cs_pass1/cs_pass2: hard-gate PASS (larger opcode residual; byte-offset
  `frame_constants_storage_buffer` and per-struct field access reconstructed).

## Top Unknowns / limitations
1. **PhysicalStorageBufferAddresses** (cull_and_bucket base + curved_halftimber/plaster/smoke/
   stairs_floor): installed dxc (1.10.2605.37 Homebrew) has NO `[[vk::buffer_reference]]`
   (attribute rejected by the parser; no symbol in libdxcompiler). The PhysicalStorageBufferAddresses
   capability therefore cannot be reproduced → those 10 bins FAIL the capabilities hard gate.
   Recorded as a toolchain limitation, not a semantics change. `cull_and_bucket.cs.hlsl` is a
   best-effort logical-address reconstruction of the disassembled algorithm (meta decode,
   frustum + HZB, bucket/histogram) for the reviewer to compare; instance data layout
   (`position.xyz` + packed meta) recovered from OpLoad/OpBitcast evidence.
2. **NaniDrawListInfo layout**: only member 0 (a uint base) is pinned; the rest is inferred.
3. **Cull variants bush/flowers/roof_tile/wooden_plank** (logical addressing) and wall
   `wall_brick_lod0/1.raster` were not completed in this pass.
4. OpSource/OpString residuals: original include closures carry more files (prefix_scan→
   grid.inc.hlsl, tracked_buffer→smooth_max) than the reconstructed inc chain; not chased per
   the documented residual policy.

## inc issues (flag for review)
- `inc/prefix_scan.hlsl` macro PREFIX_SCAN_UINT emits barrier-before-add; the nani scan targets
  are barrier-after-add (11 barriers) — scan bodies written inline, not via the macro.
- `inc/frame_constants.hlsl` (main tree) declares the `frame_constants` cbuffer at set1/b0; wall
  compute consumers use a StructuredBuffer<FrameConstants> at t1/set1 and must NOT include it
  (would add the UBO binding); FrameConstants/ViewConstants are redeclared locally there.
- `inc/draw_indirect.hlsl` static-const globals collide with explicit `cbuffer $Globals`; wall
  shader includes draw_indirect_types.hlsl directly instead.
