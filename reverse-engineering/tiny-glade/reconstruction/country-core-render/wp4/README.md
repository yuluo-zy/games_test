# WP4 — country-core render-core wiring (mesh / shader / render-config / asset libraries)

Delivery for the Tiny Glade original-game reverse-engineering project, version **B**
(patched `tiny-glade.exe` cf048e27…, PDB `tiny_glade.pdb` 2d9686b8…).

## What is here

```
contracts/
  mesh.md                       Mesh + VertexAttributeValues contracts
  shader.md                     Compute/Raster shader program + variant cache
  view_frame_light_atmos_texture.md
                                ViewConstants/FrameConstants/DirectionalLight/SunSkyColors/load_texture
  render_config_mod_fsr_debug_adaptive.md
                                VideoSettings/FullscreenConfig/GraphicsDeviceResource/FSR/debug/adaptive
  asset_libraries.md            shader/ssbo/mesh/texture/color library contracts
src/                            Rust aligned to the original skeleton module tree
  render/  mod, mesh, shader, load_texture, render_config, view_constants,
           frame_constants, directional_light, atmosphere, fsr_plugin,
           debug_info_buffers, adaptive_render_state
  asset_libraries/  mod (Handle/HandleId/Asset), shader_library, ssbo_library,
           mesh_library, texture_library, color_library
  lib.rs                        crate glue + typed contracts for reused deps
evidence.json                   decompile addresses, bundle SHAs, judgment notes
_analysis/                      working dir: Ghidra jobs/results + extracted .c decompiles
```

## Coverage summary

| area | implemented (from machine-code behavior) | typed gaps |
|---|---|---|
| `Mesh` / `VertexAttributeValues` | new, set_indices, set_attribute (in-place replace), positions, aabb_bounds, collision_bbx, collision_radius_xz, retain_vertices (full vertex-compaction algorithm), as_triangles, arrayed_indices, add_color, append | +0x18/+0x20/+0x58..+0x70 fields; exact attribute names beyond `Vertex_Position`; full variant list |
| `shader.rs` | ComputeShaderProgram::new, RasterShaderProgram::new_draw, compile_shader_variants, RasterShaderProgramVariant::get_or_generate (variant→pipeline cache) | RasterPipelineDesc +0x18 semantics; desc/recompile bodies |
| `view/frame constants` | ViewportConstantBuilder::build (matrix chain), set_pixel_offset (proj shear), FrameConstants reset flag | per-matrix column mapping; full UBO field list |
| `directional_light` / `atmosphere` | look_at, set_direction_towards_light (position += dir*dist, rebuild basis), SunSkyColors::lerp | translate/update/view_to_clip bodies |
| `render_config` / `render::mod` | FullscreenConfig::resolution, AvailableGraphicsDevices::from_preference_order (reverse), GraphicsDeviceResource Deref | VideoSettings full layout/upgrade body, tgsr_resolution_scale value |
| `load_texture` | load_compiled_texture_image (mmap + magic/header validation + create_texture) | compiled-image header field semantics |
| `AssetShaderLibrary` | new, add_raster (dedup), add_compute (hash-keyed), get_compute_by_name, get_raster_handle_by_name, debug_raster_get_name | FrozenMap container internals (reused dep) |
| `AssetSsboLibrary` | new/Default, add_new, add_new_with_data, export_from_rg, get_rg_buffer (re-import on export), get/get_mut | SsboId hash fn; BufferState other states; non-render methods |
| `AssetMeshLibrary` | new, add, get, get_by_name, try_get_handle_by_name, remove, reload_mesh_src (JSON reload) | transient/hashbrown-vs-FrozenMap dual store internals; get_mut return detail |
| `AssetTextureLibrary` | new, add, create_name_mapping, get_rg_texture, handle_to_bindless_index (limit 0x1fdf), bindless_textures_dset flush | NamedTexture layout; handle→texture map; DescriptorIndex::allocate |
| `ColorLibrarySsbo` | deserialize_from (field-name→slot table from byte compares), deserialize_from_path, update_color_library (default white + GPU upload + fallback) | remaining slot indices |
| `fsr_plugin` / `debug_info_buffers` / `adaptive_render_state` | retained as modules; 0 standalone procs recovered | full bodies |

## Reused dependencies (NOT reimplemented)

`bevy` (fork 0.16), `glam` 0.29.3, `ash`, `parking_lot`, `hashbrown`, `frozen`,
`rhapsody` (render graph: rg/raw/import_export/managed), `shader-pipeline` /
`shader-variants` / `shader-compiler` (shader toolchain), `puffin`, `ustr`, `ron`
(PounceLight fork in the original). The WP4 code calls these through the typed
contract modules in `src/lib.rs`.

## Consumer wiring claims

- `AssetShaderLibrary::get_compute_by_name` is consumed by `ComputePass::new`
  (0x1405876a0, pass-builder work): cache-hit reuses the program, miss triggers
  `as_compute_pipeline` + `ComputeShaderProgram::new` + `add_compute`.
- `AssetSsboLibrary::get_rg_buffer<T>` is the render-facing entry used by
  `pass_resource_access::{read,write}_access_buffer<T>` and DrawBuilder
  indirect draws (0x1409076b0/0x140907cc0/0x140907f90) — the observed
  export→re-import path keeps CPU/GPU buffer handles in sync.
- `AssetTextureLibrary::bindless_textures_dset` flushes the bindless texture
  table into the persistent `"bindless_textures"` descriptor set, matching the
  `bindless_textures_dyn@2.0` variant note in `evidence/render-passes/INDEX.md`.
- `RasterShaderProgramVariant::get_or_generate` implements the two-16-hex
  variant→runtime-pipeline cache described in
  `evidence/render-passes/shaders/catalog.md`.

## Remaining gaps

1. **asset_libraries non-render surface** (deferred, not implemented):
   `meshwatch`, `vao_library`, `placement_data_library`, `configwatch`,
   `texture_watch`, `audio_trigger_library` — render-facing entry points only
   were implemented per WP4 scope.
2. `Mesh` attribute key set beyond `Vertex_Position`; `VertexAttributeValues`
   exact variant list; field semantics of `Mesh` +0x18/+0x20/+0x58..+0x70.
3. `VideoSettings` serde layout + `upgrade` body; `tgsr_resolution_scale`
   numeric table; `AdaptiveRenderState` fields; `debug_info_buffers` content.
4. `ViewConstants` per-matrix column mapping (0x40-stride interleave) — behavior
   captured, exact field-by-field source layout remains inferred.
5. Bindless `DescriptorIndex` allocation/eviction policy details.
6. `ColorLibrarySsbo` slot indices beyond those byte-observed in
   `deserialize_from`.
7. No GPU/dynamic verification was performed (project scope: CPU business logic
   recovery first).

## Build check

```
cargo check   # in this directory (skeleton compiles against contract modules)
```

(If crates are unavailable offline, `cargo check` requires network to fetch the
glam/serde/ron/parking_lot/log deps; the code itself is dependency-light.)
