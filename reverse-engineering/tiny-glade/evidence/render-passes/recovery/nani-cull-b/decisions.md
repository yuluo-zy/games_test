# nani-cull-b decisions (Nani GPU-driven cull/draw-list, B-side)

Scope: `nani/` cull/draw-list chain + `wall/` draw-list/rasters, candidates only
under `reconstruction/shader-source/candidates/nani-cull-b/shaders/`.

## Key structure choices
- **draw_list_offsets.inc**: `cbuffer instance_list_offset` (b13, uint4[27]) + helper
  `instance_list_offset_at(index)` = `array[index/4][index%4]` (108 draw lists = 27×4 lanes).
- **draw_lists.inc**: `GpuFilteredInstance{raw}`, `GpuFilteredInstanceMeta{raw}`,
  `NaniDrawListInfo{global_instance_offset}`, and the nani `DrawIndexedIndirectCommand`
  (member names `index_count/instance_count/first_index/vertex_offset/first_instance` —
  distinct from `draw_indirect_types.hlsl` which the wall chain uses).
- **atlas_part_info.inc**: `MeshAtlasPartInfo{first_index,index_count,bounding_sphere_radius,pad0}`,
  consumed as `StructuredBuffer<MeshAtlasPartInfo[64]>` (ArrayStride 16/1024, world_subset×64+mesh).
- **cull_and_bucket.inc**: shared cull (frustum side-plane loop + next-visibility OR +
  filtered-append under a histogram bucket); buffers declared here, identical across the family.
- **Affine3Packed** declared in `cull_and_bucket.inc`/`wall/inc/draw_lists.hlsl` under
  `#ifndef TINYGLADE_INC_UTILS_HLSL` because the cull/wall closures omit `inc/utils.hlsl`.
- **Variant switch**: hzb/no-hzb cull pair via `#if !defined(TINYGLADE_NANI_NO_HZB)`
  (default = hzb 8a7d superset); wall raster b903/f0ad via `TINYGLADE_RASTER_NO_COLOR`.
  recompile_check runs without -D, so the reduced variants report extra bindings there;
  both variants verified hard-gate PASS with the define.
- **Push constants** use the `[[vk::push_constant]]` struct pattern (b0-free, matches target
  PushConstant storage class).
- **Dead bindings** (unused StructuredBuffers the old dxc kept, e.g. `dispatch_args_buffer`,
  `wall_color_ids`) are kept alive with never-firing guards (old dxc preserved unused bindings;
  new dxc DCEs them).

## Names (I where not verbatim)
Struct/member names are verbatim from OpMemberName. Function/helper names are Inference
(inlined by dxc; no OpName survives). `cull_frustum_side_planes` / `cull_append_visible`
etc. are reconstruction names.

## Open Unknowns
- **PhysicalStorageBufferAddresses instance read** (base cull, plaster, smoke, stairs, curved):
  dxc emits the `Int64` capability for uint64 address math; the original kept the address as
  uint2 and had no Int64. The physical read is therefore approximated (per-index mesh/radius
  or instance_buffer where bound) and the caps are declared via `[[vk::ext_capability(5347)]]`.
  This is the largest histogram residual class.
- HZB test constants (screen-extent → mip → 2x2) reconstructed at histogram level; exact
  projection/AABB math is approximated.
- Wall raster bodies are binding-correct but the 39-file closure pulls read-only incs with
  `color_library` register/`current_view_index`/sampler drift; trimmed to the bindings.
