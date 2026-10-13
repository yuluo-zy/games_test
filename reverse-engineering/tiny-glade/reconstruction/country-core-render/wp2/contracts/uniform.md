# uniform.rs — lazy uniform binding + uniform buffer write helpers

Target: B (EXE cf048e27…, PDB 2d9686b8…). Original source: `crates/country-core/src/render/uniform.rs` (checksum 41234a9b…).
Bundle: `skeleton/B/crates/country-core/src/render/uniform.rs.{pdb.json,structures.json.gz,enriched.json.gz,dependencies.json}`.
Procedures: 0 standalone (all 3 functions inline, inst 0). PDB functions: `bind_lazy_uniform`, `impl$23::write_uniform_to_buffer`, `impl$24::write_uniform_to_buffer`.

Legend: **O**=Observation, **I**=Inference, **U**=Unknown.

## Producer/consumer
- Producer: pass builders (WP1) use these to write uniform values into dynamic-uniform buffers without binding them eagerly.
- Reuse (O): `hashbrown`/`std::collections::HashMap`, `rhapsody` (buffer/device), `shader_pipeline` (reflection types), `country_core::render::pass_builder::uniform_binder::UniformBinder` (WP1-owned).

## `bind_lazy_uniform` — inline (I, deps O)
- Deps (O): `UniformBinder::write_optional_uniform` (+ its closure), `HashMap::get`, `Option::{as_ref, as_mut, and_then}`.
- Contract (I): looks up a binding by hash in the UniformBinder's binding map and writes the given value lazily (only when present), via `write_optional_uniform`. Generic over the uniform type (`Vec3`, etc. per WP1 `write_uniform<Vec3>`/`write_optional_uniform`).

## `impl$23::write_uniform_to_buffer` — inline (I, deps O)
- Deps (O): `copy_from_nonoverlapping` / `copy_nonoverlapping`.
- Contract (I): `fn write_uniform_to_buffer<T>(buffer: &mut [u8], offset, value: &T)` — `copy_nonoverlapping` the value bytes into the mapped uniform buffer at the binding offset.

## `impl$24::write_uniform_to_buffer` — inline (I, deps O)
- Deps (O): `impl$23::write_uniform_to_buffer` + the same copy intrinsics.
- Contract (I): a wrapper variant of the above (likely a sized/unsized or aligned-trait specialization that forwards to impl$23). Exact specialization U.

## Failure branches
- None observed (U; writes assume caller-validated offsets).

## Integration notes
- `UniformBinder`/`write_uniform`/`write_optional_uniform` layout belongs to WP1 (`pass_builder::uniform_binder`); WP2 restores the write-to-buffer leaf and the lazy-bind orchestration.
