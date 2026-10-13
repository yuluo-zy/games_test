# vao.rs — `Vao` / `VaoBinding` / `build_vao`

Target: B (EXE cf048e27…, PDB 2d9686b8…). Original source: `crates/country-core/src/render/vao.rs` (checksum f8bdaf0d…).
Bundle: `skeleton/B/crates/country-core/src/render/vao.rs.{pdb.json,structures.json.gz,enriched.json.gz,dependencies.json}`.
Procedures (O): `build_vao` 0x140909690 (463 B), plus inlined `build_vao::closure$0`, `Vao::new`, `Vao::rebuild`. Types: `Vao`, `VaoBinding`.

Legend: **O**=Observation, **I**=Inference, **U**=Unknown.

## Producer/consumer
- Producer: mesh/assets build a `Vao` (vertex-attribute buffer + bindings) for raster draws; `DrawBuilder`/`RasterPass` (WP1) consume it.
- Reuse (O): `rhapsody::raw::{buffer::BufferDesc, device::GraphicsDevice}` (REUSED), `hashbrown`, `std::collections::HashMap`, `country_core::render::mesh::{VaoBinding, VertexAttributeValues}` (WP4-owned).

## `Vao` — 64 bytes (8 qwords) (O from `build_vao` store)
`{ buffer_lo @0x00, buffer_hi @0x08, bindings_ptr @0x10, bindings_len @0x18, bindings_cap @0x20, f6 @0x28, f7 @0x30, f8 @0x38 }` — the first 2 qwords are the GPU vertex buffer; the next 3 a `Vec<VaoBinding>`; f6–f8 role U (typed gap; plausibly vertex count / stride / VAO attributes, not observed).

## `build_vao` — 0x140909690 (O)
- `fn build_vao(attributes: HashMap<String, VertexAttributeValues>, device) -> Vao`.
- Creates a GPU-only buffer: `BufferDesc::new_gpu_only()` + `GraphicsDevice::create_buffer<Option<&[u8]>>()`; unwraps result (Ok tag 0xb) (O).
- Builds `Vec<VaoBinding>` from `attributes.iter().map(build_vao::closure$0).collect()` (`from_iter<Map<Iter<String,VertexAttributeValues>, closure>>`, O). The closure converts each named attribute into a binding using `VertexAttributeValues::size_in_bytes` (O dep) and indexing into the raw attribute bytes.
- Returns the `Vao` (8 qwords) (O).

## `build_vao::closure$0` — inline (I, deps O)
- Maps `(&String, &VertexAttributeValues)` → `VaoBinding`; uses `size_in_bytes` + byte-slice indexing + `unwrap`. Exact `VaoBinding` field layout U (WP4 mesh contract territory).

## `Vao::new` / `Vao::rebuild` — inline (U/I)
- `new` (constructor, no deps), `rebuild` (drops the previous buffer and rebuilds — `drop_in_place` deps O). Exact signatures not recovered.

## Failure branches (O)
- `create_buffer` failure → `unwrap_failed`.

## Integration notes
- `VaoBinding` layout is owned jointly with WP4 (mesh); WP2 only restores the `build_vao` orchestration + the 8-qword `Vao` shell.
