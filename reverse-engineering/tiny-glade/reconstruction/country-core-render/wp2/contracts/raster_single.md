# raster_single.rs — `raster_single_object`

Target: B (EXE cf048e27…, PDB 2d9686b8…). Original source: `crates/country-core/src/render/raster_single.rs` (checksum 04897a09…).
Bundle: `skeleton/B/crates/country-core/src/render/raster_single.rs.{pdb.json,structures.json.gz,enriched.json.gz,dependencies.json}`.
Procedures (O): 3 monomorphizations of `raster_single_object<P, COLOR, CLOSURE>`:
- `<1, [ColorAttachment;1], ui_2d_pass::mei_pass::closure_env$0>` — 0x1409cdec0 (2225 B)
- `<1, [ColorAttachment;1], final_post::final_post::closure_env$1>` — 0x1409ce780 (6417 B)
- `<1, [ColorAttachment;1], final_post::resolve_to_swapchain::closure_env$0>` — 0x1409d00a0 (2081 B)

Legend: **O**=Observation, **I**=Inference, **U**=Unknown.

## Producer/consumer
- Producer: a reusable helper that turns "one offscreen color target + a draw closure" into a full raster pass. Consumed by `ui_2d_pass::mei_pass`, `final_post`, `final_post::resolve_to_swapchain`.
- Reuse (O): `rhapsody::raw::{texture::TextureDesc, shader}` (REUSED), `modular_bitfield`, `utils`, `puffin`, `country_core::render::pass_builder::{draw_stream::RasterPass, draw_builder::{DrawBuilder, bind_texture_to_draw_builder}}` (WP1), `country_core::resources::texture_presets::TextureFromDisk` (asset).

## `raster_single_object<COLOR, CLOSURE>` — 0x1409cdec0 (O; shape identical across the 3 instances, sizes differ by closure)
- `fn raster_single_object(extent: UVec2, color_attachments: [ColorAttachment; 1], closure: CLOSURE)` (I; O: reads a 4-qword extent + a color-attachment struct).
- puffin scope (O).
- Computes the framebuffer extent via `TextureDesc::extent_2d()` (O).
- Builds `RasterPass::new_impl()` with the single `ColorAttachment` (`local_1d0 = 2` attachment discriminator, O).
- Constructs a `DrawBuilder` (`shader::default()` + RandomState hash-map keys + `DrawBuilder::new()`, O), binds the pass texture twice (`bind_texture_to_draw_builder<TextureFromDisk>` then `<&Handle<Texture>>`, O), issues `draw_no_vao()` (O), then `RasterPass::render(…)` (O).
- The closure (not decompiled as a procedure; only as the monomorphization driver) supplies the pass-specific draw parameters/shader URIs (I).

## Failure branches (O)
- `draw_no_vao`/`render` paths may panic internally (WP1-owned); `raster_single_object` itself only unwraps profiler TLS (`panic_access_error`/`panic_already_borrowed`, O).

## Integration notes
- The exact `ColorAttachment` enum discriminator (observed `2`) and the 0x44-byte RenderPassBinding layout belong to WP1 (`pass_builder`); do not redefine here.
- The three monomorphizations confirm the helper is parameterized by `<pass_depth, ColorAttachment-array, closure>`; WP2 restores the orchestration only.
