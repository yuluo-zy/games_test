# Country-core render domain — INTEGRATION REPORT (wp1–wp5 → `country-core`)

Target crate: `reconstruction/country-core/` — package `country-core`, edition 2024,
rust-version 1.86, standalone (not in the reconstruction workspace).

## Status

- `cargo check --offline` — **PASSES** (67 warnings, all skeleton-markers).
- `cargo clippy --all-targets --offline` — **PASSES** (133 warnings, 0 errors).
- No remaining compile errors. All five work packages assembled into one module tree.

## Module tree assembled

```
src/lib.rs                        crate root: Gap / typed_gap / rhapsody_contract /
                                  shader_pipeline_contract / shader_variants_contract /
                                  utils_contract / puffin_contract / ustr_contract /
                                  consume (wp5 re-export layer) / resources (typed gaps) /
                                  utils (index_condenser) / asset_libraries / render / systems
src/asset_libraries/              wp4: mod, color_library, mesh_library, shader_library,
                                  ssbo_library, texture_library
src/render/pass_builder/          wp1 (11 files: bind_resources … uniform_binder)
src/render/nani/                  wp3 (mod, buffers, cull_shaders, draw_ext, gpu_world,
                                  instance_data, mesh_atlas, trimesh)  [systems.rs excluded]
src/render/{ssbo,shadowed_buffer,sparse_instance_buffer,vao,raster_single,
            dirty_index_range,uniform,indirect}.rs   wp2 flat render files
src/render/{mesh,shader,load_texture,render_config,view_constants,frame_constants,
            directional_light,atmosphere,fsr_plugin,debug_info_buffers,
            adaptive_render_state}.rs + mod.rs   wp4 render files
src/render/mesh_atlas.rs          NEW gap: render::mesh_atlas::AtlassedMeshName (wp3 ref)
src/render/utils.rs               NEW gap: render::utils::calculate_hash (wp1 ref)
src/systems/render_loop/          wp5 (mod.rs + compute_passes/ 41 + draw_passes/ 16)
src/systems/ui.rs                 NEW gap: systems::ui::GlobalId (wp1 draw_builder ref)
```

111 `.rs` files. `systems/` is wp5's render_loop; `systems::ui` is a typed-gap marker.

## Rhapsody boundary decision (single canonical surface)

The crate's one render-graph boundary is **`crate::rhapsody_contract`** — wp4's typed
contract, extended at assembly with the game-facing surface wp1–wp3 call
(`RenderPassBinding` + `bind()`, `PassBuilder<'_, UD>` + `add_resource_access` +
`resolve_resource`, managed `Handle<T>` / `ReadOnlyHandle<T>`, `RenderPass(Desc)`,
`TextureViewDesc`, `Extent2D`, `PassResourceSlot`, `RenderGraph`, and
`GraphicsDevice::{full_barrier_required, issue_full_pipeline_barrier, dispatch}`).

`rhapsody = { path = "../rhapsody-rebuild" }` is declared in Cargo.toml (the reused
render-graph boundary per the assembly spec) and resolves in the lock, but the wp source
never actually calls rhapsody-rebuild's *recovered* surface: rhapsody-rebuild has the raw
Vulkan/queue machinery + graph core (PassDeclaration/PassBuilder/ResourceHandle) but NOT the
game-facing API the packages use (`RenderPassBinding`, `bind()`, `PassBuilder<UD>`,
managed `Handle<RasterShaderProgram>`, `ReadOnlyHandle`, `RenderPassDesc`,
`RenderPassAttachmentDesc`, `Extent2D`, `resolve_resource`, …). Wiring to it would require
inventing bridging that duplicates the contract, which the rules forbid. Decision recorded;
the contract documents the mapped reused paths. No rhapsody internals were implemented.

## Cross-package wiring performed

- **wp5 `consume::` → real types**: `consume::{pass_builder,buffers,nani,core}` now
  re-export the assembled wp1–wp4 types (ComputePass/DrawBuilder/RasterPass/DrawPacket/
  PassResources/ShaderUri/UniformBinder/AsComputeShaderSource, TypedStorageBuffer/
  InstanceBuffer/ShadowedBuffer/SparseInstanceBuffer/indirect wrappers, GpuWorld/
  subsets/kinds/instance-data/MeshAtlas/cull-shader structs, Mesh/FrameConstants/
  ViewConstants/DirectionalLight/SunSkyColors/Asset*Library) plus typed-gap SSBO carriers
  (LanternSsbo, RtBvhSsbo, DofAutofocusSsbo, …) in `resources::render_ssbo`.
- **wp5 `AsComputeShaderSource` impls** (bin_lights, dof, taa, rasterize_terrain_stroke)
  re-wired to the real trait signature `as_compute_pipeline(&self) -> ComputeShaderSource<'_>`
  using the documented O URIs (e.g. `/gauss_blur.cs.hlsl:cs_main`).
- **wp5 `gpu_generate_draw_lists`** re-wired to the real by-value `ComputePass` builder API
  (`push_constants(&[u8])`, `dispatch_1d`), `GpuWorldSubsetKind::all()` (added to wp3,
  replacing the enum-iterator placeholder), and real `GpuWorld` methods.
- **wp2 boundary stubs removed**: `uniform.rs` `UniformBinder` → real
  `render::pass_builder::uniform_binder::UniformBinder`; `raster_single.rs`
  `pass_builder_boundary` module → real `RasterPass`/`DrawBuilder`/`ColorAttachment`.
- **wp2 flat paths normalized** to the B-skeleton `crate::render/*` location
  (`crate::ssbo` → `crate::render::ssbo`, etc.), matching wp3/wp1 references.
- **wp1 `ColorLibrarySsbo`** routed to wp4's real type; ray-tracing/wall SSBO carriers kept
  as typed gaps with the observed `SsboType::rg_buffer_hash` constants. Local wp1
  `GraphicsDeviceResource` deleted; `ComputePass::new` / `RasterPass::new` take
  `&render::GraphicsDeviceResource`. `GameContext::uniform_offset` impl added (call-site
  shape). `BindingIndex` routed to `shader_pipeline_contract::BindingIndex`.
- **wp4 `RasterShaderProgram::default_handle()`** added (typed-gap static for
  `DrawPacket::new`).
- **wp3 `rhapsody_rg::render_graph::RenderGraph`** → `rhapsody_contract::RenderGraph`;
  `TypedStorageBuffer::new()` zero-arg call → documented todo (wp2 real arity differs);
  `nohash_hasher` → local `AtlasNameHasher`.

## Resolved integration gaps (categorized)

Cross-ref gaps fixed during assembly (all documented in-file):
- Borrowck (`E0499`): `ComputePass`/`DrawBuilder` access methods that passed both
  `&mut self.as_pass_builder()` (a `todo!()` rhapsody boundary) and `&mut self.bindings` in
  one call → sequential typed-gap form. `bind_resources` slot-cache readdressed via
  `as_ref().map` (removed `Option::copied` on `&mut`).
- `ComputePass`/`DrawBuilder`/`RasterPass` builder arity vs wp5 placeholder call shapes →
  wp5 bodies adapted to the real by-value API.
- InstanceKey f32 `Eq`/`Hash` → manual Eq/Hash on `chunk_id`+`instance_index` (map-key O).
- `TrimeshBuilder` `Attrib<'static>` lifetime bug → `TrimeshBuilder<'a>`.
- `DrawPacket`/`RasterPass` derives needing `T: Clone/Debug` → manual `Handle<T>`/
  `ReadOnlyHandle<T>` Clone/Debug (bound-free); `DrawPassRenderCheck`, `ColorAttachment`
  gained `Clone/Debug/Default` derives.
- `impl_uniform_value_copy` `to_le_bytes` on `bool/[u32;2]/[u32;4]` → raw byte copy.
- `shader_uri` `panic_internals`/unsized-hash → `panic!` / `calculate_hash(&_source)`.

## Recorded gaps / environment notes

- **Cross-ref gap (deferred)**: `render/nani/systems.rs` (wp3's 20 `nani_update_*` +
  `nani_maintain_instances` ECS systems) is EXCLUDED from the module tree. Its inputs are
  the whole game component/resource graph (`crate::components::*`, `crate::resources::*`,
  `utils::components::transform::Transform`), which the five render packages do not deliver
  (wp5 itself marks `resources::` OUT OF SCOPE). File preserved on disk; not declared.
- **Rhapsody-boundary gap**: wp1's `pass_resource_access`/`draw_stream` texture/Handle
  ReadTexture impls and `as_pass_builder` are `todo!()` (real rhapsody managed-handle/temporal
  surface not wired — see boundary decision).
- **Environment (offline-cache) gaps**: `puffin` (→ `puffin_contract`), `enum-iterator` 1.2
  (→ manual `GpuWorldSubsetKind::all()`), `nohash-hasher` (→ local no-op hasher),
  `bytemuck = 1.16.0` is **yanked** (dropped from deps — unused by the assembled code;
  would otherwise force a newer version).
- `itertools`/`bumpalo`/`anyhow`/`ustr`/`serde_json` are referenced only in docs/comments
  but declared (cached, resolve offline).

## Exact `cargo check --offline` tail

```
warning: function `with_stencil_write` is never used
  --> src/systems/render_loop/draw_passes/main_pass.rs:16:4
   |
16 | fn with_stencil_write(draw: &mut DrawBuilder) {
   |    ^^^^^^^^^^^^^^^^^^

warning: `country-core` (lib) generated 67 warnings (run `cargo fix --lib -p country-core` to apply 46 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
```

`cargo clippy --all-targets --offline` tail: `warning: country-core (lib) generated 133 warnings … Finished dev profile … in 0.80s`.
