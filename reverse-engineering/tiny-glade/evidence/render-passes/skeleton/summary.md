# shader-source skeleton extraction summary

Date: 2026-10-13. Input: 404 .bin under compiled-shaders/ (read-only oracle).

## Counts (Observation)
- bins: 404 (170 compute 1-module, 234 raster 2-module)
- unique embedded source paths (raw strings): 417 (prior agent: 419; delta = the 2 SSGI
  files, embedded as `shaders/ssgi/XeGTAO.hlsl.inc` / `shaders/ssgi/vaShared.hlsl.inc` but
  truncated to `.hlsl` in the prior list)
- unique resolved (../-collapsed) paths: 413
- files compiled as main in >=1 bin: 260; include-only files: 153
- include-only files that are `.inc.hlsl`: 64
- include-only files that are plain `.hlsl` (non-inc): 89

## Empty-path shaders (need Investigation)
- bins with empty embedded path set: 0 []
- bins with sources but no unique main match: 0 []

## Include graph (Inference)
- observed main->include closure edges: 4389
- direct-include hypothesis edges (majority-consensus minimal DAG): 411
- top shared inc files by bin count: shaders/inc/common.hlsl=402, shaders/inc/debug_utils.hlsl=402,
  shaders/inc/samplers.hlsl=402, shaders/inc/tracked_buffer.hlsl=402, shaders/inc/uv.hlsl=339,
  shaders/inc/math_const.hlsl=335, shaders/inc/frame_constants.hlsl=307, shaders/inc/ray_cone.hlsl=307,
  shaders/inc/math.hlsl=253, shaders/inc/consts.hlsl=250, shaders/inc/hash.hlsl=250,
  shaders/inc/pack_unpack.hlsl=235
- confident shared-layer nesting chain: `inc/common.hlsl -> inc/samplers.hlsl ->
  inc/debug_utils.hlsl -> inc/tracked_buffer.hlsl` (each is the immediate first include of the
  previous across ~all raster leading blocks). See include-graph.json `ambiguity_note` for the
  sibling-vs-descendant limitation.

## Skeleton tree (reconstruction/shader-source/shaders/)
- 413 placeholder files, one per resolved source path, each with a `// ==SKELETON==` header
  (mapped .bin + sha256, stage/entry/variant, pass + catalog family + INDEX pass ids, sample of
  embedded names). No HLSL body.
- Top-level layout (files per top dir): inc 89, nani 31, water 19, dof 14, garden 11, wall 11,
  clearing 10, rocky_terrain 10, terrain_editing 10, grass 8, gi 7, critters 6, deferred 6,
  painterly 6, vermeer 6, light_binning 5, ivy 4, paths 4, roof 4, session_selection 4,
  blit 3, prefix_scan 3, screenshot 3, ssgi 3, blur_pyramid 2, buildings 2, lut 2, lut_gallery 1,
  rdi 1, and 128 `.hlsl` files flat in the shaders/ root.
- `../`-style raw embedded paths are collapsed to their resolved directory (no literal `..`
  dirs); raw strings are kept in per-shader-mapping.json `raw_path_variants` and in each
  placeholder header.

## Open unknowns
- exact #include nesting of every shared inc file (the 411-edge graph is the minimal DAG
  consistent with all orderings; sibling-vs-descendant is ambiguous from preorder alone).
- full source text is not embedded (OpSource holds newline-padding only).
- which INDEX.json frame pass each non-frame shader family feeds is unresolved for
  painterly_vermeer / session_ui / fsr_fxaa / sky_cloud / screenshot_capture / critters /
  primitives families (see INDEX.json `non_frame_shader_families`).
