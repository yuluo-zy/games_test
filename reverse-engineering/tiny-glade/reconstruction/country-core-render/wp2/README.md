# WP2 — country-core render GPU buffer primitives (Rust reconstruction, B)

Analysis root: `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade`.
Delivery dir: `reconstruction/country-core-render/wp2/`.

## Scope

~178 native procedures across `crates/country-core/src/render/`:
`ssbo.rs`, `shadowed_buffer.rs`, `sparse_instance_buffer.rs`, `vao.rs`,
`dirty_index_range.rs`, `raster_single.rs`, `uniform.rs`, `indirect.rs`.

Method: business logic from decompilation (Ghidra `--worker shared`, read-only B) + call chains +
PDB skeleton bundles; Rust written from the source skeleton. Machine-code diff was NOT used as an
implementation method (no oracles, no case-count inflation).

## Deliverables

- `contracts/<module>.md` — per function-group business-logic contracts with O/I/U tags,
  field layouts + layout source, side effects, failure branches, typed gaps, integration notes.
- `src/<module>.rs` + `src/mod.rs` — contract-aligned Rust mirroring the original module tree.
  `src/ssbo.rs::rhapsody_boundary` is the REUSED-rhapsody surface (reference
  `reconstruction/rhapsody-rebuild`); WP1 (`pass_builder`), WP4 (`mesh`) types are boundary-only.
- `evidence.json` — decompile addresses, skeleton bundle SHAs, key facts, reuse boundary, gaps,
  and the sparse_instance_buffer candidate verdict.
- `evidence/` — raw Ghidra job/result files + per-function `.c` decompiles.

## Contract coverage

| module | status | coverage |
|---|---|---|
| ssbo.rs | Reconstructed | `TypedStorageBuffer<T>` (5-qword layout; `new`/`gpu_write*`/`gpu_write_ranges_nonoverlapping`), `InstanceBuffer<T>`, `MappedStorageBuffer<T>`, `BufferWriteError` (2 variants O, names U) |
| shadowed_buffer.rs | Reconstructed | `ShadowedBuffer<T>` 9-qword layout; `new`/`gpu_flush` O; `get_mut`/`copy`/`write` I (deps O); inclusive dirty range |
| sparse_instance_buffer.rs | Reconstructed | `SparseInstanceBuffer<T>` 18-qword layout, 32-instance chunks; `new_storage_buffer`/`hide`/`show`/`take_out_waste`/`gpu_flush`/`allocation_to_draw_multi_instanced`/`to_draw_multi_instanced` O; `Writer::push`/`clear` O; `allocate`/`write_allocation`/`free_impl` I |
| vao.rs | Reconstructed | `build_vao` O; `Vao` 8-qword shell O; `VaoBinding` layout WP4-owned |
| dirty_index_range.rs | Reconstructed | `DirtyIndexRange` (AtomicUsize start/end, inclusive); `add` deps O; Debug O |
| raster_single.rs | Reconstructed | `raster_single_object` orchestration O (3 monomorphizations) |
| uniform.rs | Reconstructed | `bind_lazy_uniform`/`write_uniform_to_buffer` I (all inlined) |
| indirect.rs | Reconstructed | `DrawIndexedIndirectCommand`/`DrawIndirectCommand`/`DispatchIndirectCommand` wrappers I (Vulkan ABI) |

## Sparse candidate verdict

`skeleton/task-index/work/TG-f4e8ea3c0995e850c0/abc-tools-52/…/sparse_instance_buffer.rs`
(status `implemented_unverified`) covers **only** `AllocationHandle::is_null` (B address correct).
It is a **minimal_reusable_part**: merged for the `word==0` null check, superseded for everything
else (32 of 53 PDB symbols uncovered by it).

## Shared types OWNED by WP2 (cross-package contract)

- `ssbo::TypedStorageBuffer<T>` — 40 bytes; `[0x18]=byte_size` is load-bearing for flush guards.
- `ssbo::InstanceBuffer<T>`, `ssbo::MappedStorageBuffer<T>`.
- `shadowed_buffer::ShadowedBuffer<T>` — dirty range is INCLUSIVE, empty=(MAX,0).
- `sparse_instance_buffer::{SparseInstanceBuffer, SparseInstanceBufferWriter, Allocation, AllocationHandle, ChunkId}` + `bitmap_set/bitmap_unset`.
- `dirty_index_range::DirtyIndexRange`.
- `vao::Vao` (8-qword shell; `VaoBinding` co-owned with WP4 mesh).
- `indirect::*Command` wrappers.

WP1 (`pass_builder`) / WP3 (nani) / WP4 (mesh) reference these via the contracts above.

## Remaining gaps (also in evidence.json)

- TypedStorageBuffer field[4], MappedStorageBuffer f4–f6: role unknown.
- Sparse `slot_meta` u32 table + `Allocation.field0`: no decompiled consumer.
- Draw-range output destination pointer: U.
- BufferWriteError variant names: U.
- Inlined methods (`InstanceBuffer::gpu_write_subset`, `allocate`, `write_allocation`, `free_impl`,
  `ShadowedBuffer::{get_mut,copy,write}` exact shapes, uniform/indirect leafs): I only.
- Compile check: `src/` passes an offline `cargo check` (0 errors / 0 warnings) against a temporary
  `parking_lot` stub; a full build requires the rhapsody-rebuild boundary + WP1/WP4 crates. The src
  is contract-aligned, not a standalone build.
