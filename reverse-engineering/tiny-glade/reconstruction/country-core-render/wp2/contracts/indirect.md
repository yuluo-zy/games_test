# indirect.rs — indirect draw/dispatch command wrappers

Target: B (EXE cf048e27…, PDB 2d9686b8…). Original source: `crates/country-core/src/render/indirect.rs` (checksum 9b377942…).
Bundle: `skeleton/B/crates/country-core/src/render/indirect.rs.{pdb.json,structures.json.gz,enriched.json.gz,dependencies.json}`.
Procedures: 0 standalone (all inline, inst 0). PDB functions (5): `DrawIndexedIndirectCommand::{w_vertex_count, empty}`, `DrawIndirectCommand::{w_instance_count, empty}`, `DispatchIndirectCommand::empty`. Types: the three command structs.

Legend: **O**=Observation, **I**=Inference, **U**=Unknown.

## Producer/consumer
- Producer: `country_core::render` CPU-side command builders; consumers: the indirect GPU buffers (`DrawIndexedIndirectBuffer`, `DrawIndirectBuffer`, `DispatchIndirectBuffer` — WP1 `indirect_params`/`AssetSsboLibrary`).
- These wrap the Vulkan indirect command layouts (REUSED ash/Vulkan ABI; `DrawBuilder::draw_indexed_indirect_raw` uses stride 0x14 = 20 bytes = `VkDrawIndexedIndirectCommand`).

## Structs (I, sizes from Vulkan ABI + DrawBuilder stride evidence)
- `DrawIndexedIndirectCommand` (20 bytes = `VkDrawIndexedIndirectCommand`): `{ index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32 }`.
- `DrawIndirectCommand` (16 bytes = `VkDrawIndirectCommand`): `{ vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32 }`.
- `DispatchIndirectCommand` (12 bytes = `VkDispatchIndirectCommand`): `{ x: u32, y: u32, z: u32 }`.

## Methods (I, names O; no machine code)
- `empty() -> Self` for all three: zero-initialized command (O name).
- `DrawIndexedIndirectCommand::w_vertex_count` / `DrawIndirectCommand::w_instance_count`: write accessors (I: set the named field). Exact semantics U (inlined).

## Failure branches
- None observed.

## Integration notes
- Field order follows the Vulkan ABI (I); if WP1 passes these as bytes to `ash::vk` structs, byte order must match `VkDrawIndexedIndirectCommand`/`VkDrawIndirectCommand`/`VkDispatchIndirectCommand` (20/16/12 bytes).
- WP2 owns the wrapper names only; the byte-level layout should be verified against the shader-side `StructuredBuffer` consumption (WP1 nani cull) before fixing field order as O.
