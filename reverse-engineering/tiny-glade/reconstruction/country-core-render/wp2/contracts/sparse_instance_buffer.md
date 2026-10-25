# sparse_instance_buffer.rs — `SparseInstanceBuffer<T>` + `SparseInstanceBufferWriter<T>` (GPU-driven sparse instancing)

Target: B (EXE cf048e27…, PDB 2d9686b8…). Original source: `crates/country-core/src/render/sparse_instance_buffer.rs` (checksum 762b90cd…).
Bundle: `skeleton/B/crates/country-core/src/render/sparse_instance_buffer.rs.{pdb.json,structures.json.gz,enriched.json.gz,dependencies.json}`.
Procedure inventory (O): `impl$10::new_storage_buffer` ×4, `gpu_flush` ×5, `push`/`clear` ×4/3, `hide`/`show`, `take_out_waste`, `allocation_to_draw_multi_instanced`, `impl$3::to_draw_multi_instanced`, `AllocationHandle::is_null`, `impl$12::{as_any,as_any_mut,handle}`, plus inlined `allocate/write_allocation/free_impl/iter_*`.

Legend: **O**=Observation, **I**=Inference, **U**=Unknown.

## Producer/consumer

- Producer: per-element type constructors `impl$10::new_storage_buffer<T>`; `SparseInstanceBufferWriter` batches instance writes; `allocate`/`write_allocation`/`free_impl`/`hide`/`show` manage allocations.
- Consumers: nani/roof/wall draw paths call `to_draw_multi_instanced`/`allocation_to_draw_multi_instanced` to emit indirect draw ranges, and `gpu_flush` before the pass reads the SSBO.
- Reuse (O): `rhapsody::raw::bytes`, `rhapsody::rg::resource::ResourceHandle`, `parking_lot`/`parking_lot_core`/`lock_api` (RwLock/Mutex), `bit_iter`, `puffin`, `core::sync::atomic::AtomicUsize`. Game-owned: `SparseInstanceBuffer`, `SparseInstanceBufferWriter`, `Allocation`, `AllocationHandle`, `ChunkId`, `SparseInstanceBufferAllocation`, `bitmap_set`/`bitmap_unset`.

## Layout model — 32-instance chunks

Each element type partitions instance slots into chunks of 32 (0x20) instances. Per-chunk state is tracked by a byte map (active count in 0..=32) and two bitmaps (active, dirty). Type-specific constants (O, from `impl$10::new_storage_buffer` instances):

| T | chunk_count | byte_map len | slots | elem size (O) |
|---|---|---|---|---|
| RoofTileSsbo | 0x2000 (8192) | 0x2001 | 0x40000 | — |
| BrickTransformSsbo | 0x10000 (65536) | 0x10001 | 0x200000 | 0x60 (96) |
| WoodenPlankSsbo | 0x10000 (65536) | 0x10001 | 0x200000 | 0x40 (64) |
| SparkleSsbo | 0x6000 (24576) | 0x6001 | 0x300000 | — |

## `SparseInstanceBuffer<T>` — 0x90 bytes (18 qwords) (O)

| off | size | field | status |
|---|---|---|---|
| 0x00 | 8 | `pending_resource_handles: parking_lot::RwLock<Vec<rhapsody ResourceHandle>>` (fast-path writer flag 0/8) | O |
| 0x08 | 8 | `chunk_count: usize` | O |
| 0x10 | 16 | `free_chunks: Vec<u32>` (ptr @0x10, len @0x18), initialized descending `[chunk_count-1 … 0]` | O |
| 0x20 | 16 | `byte_map: Vec<u8>` (ptr @0x20, len @0x28 = chunk_count+1) — per-chunk active count; byte[chunk_count]=0x20 sentinel | O |
| 0x30 | 16 | `active_bitset: Vec<AtomicUsize>` (ptr @0x30, len @0x38 = chunk_count/64) | O |
| 0x40 | 16 | `slot_meta: Vec<u32>` (ptr @0x40, len @0x48 = chunk_count*32) — per-instance-slot u32 | U role (allocated/zeroed O; consumer not decompiled) |
| 0x50 | 16 | `dirty_bitset: Vec<AtomicUsize>` (ptr @0x50, len @0x58 = chunk_count/64) | O |
| 0x60 | 8 | `allocations: Arc<parking_lot::RwLock<Vec<Allocation>>>` | O |
| 0x68 | 40 | `buffer: TypedStorageBuffer<T>` (5 qwords, layout per `contracts/ssbo.md`) | O |

### `impl$10::new_storage_buffer<T>` — 0x1412e4a90 (RoofTile), 0x1412e5800 (Brick), 0x1412e5170 (WoodenPlank), 0x1412e5f50 (Sparkle) (O)
- `fn new_storage_buffer(device) -> SparseInstanceBuffer<T>` (I; the observed `if (in_R8 != 0) panic` is a device-arg guard, exact meaning U).
- Allocates and initializes: byte_map (zeros + sentinel 0x20), slot_meta (0x40000 u32 zero), free_chunks (descending), two bitsets (`(0..chunk_count/64).map(|_| AtomicUsize::new(0)).collect()` — O `from_iter<AtomicUsize, Map<Range<usize>, closure>>`), `Arc<RwLock<Vec<Allocation>>>` (empty, header fields observed), `TypedStorageBuffer::new<T>`, and the pending-handles RwLock.

### `SparseInstanceBuffer::buffer` — 0x140005c50 5-byte stub (O)
- Accessor `&self -> &TypedStorageBuffer<T>` (or `&GpuBuffer`); role confirmed by symbol seed `SparseInstanceBuffer$LT$T$GT$::buffer`.

### `Allocation` (O from `take_out_waste` / `allocation_to_draw_multi_instanced` / `hide`)
`{ field0 @0x00 (pointer, freed if non-null — U role), chunk_ids_ptr: *mut u32 @0x08, chunk_ids_len: u64 @0x10 }`. Holds the chunk ids owned by one allocation.

### `AllocationHandle` (O/I)
- 16 bytes: `{ word: u64 @0x00, allocation: Option<NonNull<Allocation>> @0x08 }` (I; `is_null` checks word==0).
- `AllocationHandle::is_null` — 0x140985f30 (O): returns `self.word == 0`. (Candidate in `skeleton/task-index/work/TG-f4e8ea3c0995e850c0/…/sparse_instance_buffer.rs` confirmed this — see evidence.json verdict.)

### `SparseInstanceBuffer::hide` — 0x1412e3c00 (O)
- `fn hide(&self, handle: &AllocationHandle)`: unwraps `handle.allocation`; for each chunk id: bounds `chunk_id/64 < bitset_len`, clears bit `chunk_id & 63` in `active_bitset` (atomic `fetch_and`; O).

### `SparseInstanceBuffer::show` — 0x1412e3c80 (O)
- Mirror of `hide`: sets the bits in `active_bitset`.

### `SparseInstanceBuffer::take_out_waste` — 0x1412e3d00 (O; BrickTransform)
- Scans `allocations` (Arc<RwLock<Vec<Allocation>>>): for each allocation, for each chunk id: bounds `id < byte_map.len`, sets `byte_map[id] = 0`, clears the `active_bitset` bit; pushes all chunk ids back onto `free_chunks` (Vec reserve + reverse copy); frees the allocation box and any `field0` pointer.
- Uses the `pending_resource_handles` RwLock fast-path (0/8 writer flag; O). Runs at the start of `gpu_flush` (O).

### `SparseInstanceBuffer::gpu_flush<T>` — 0x1412e2f10 (O; BrickTransform)
- `fn gpu_flush(&mut self, device: &GraphicsDevice)`.
- Calls `take_out_waste` first (O).
- Scans `dirty_bitset`: for each word, counts trailing zeros to find the lowest set chunk bit, accumulates non-overlapping chunk runs `[start_chunk, end_chunk)` into a temp `Vec<(u32,u32)>`, clearing each consumed word to 0 (O).
- Converts each run to element ranges `[chunk*32, chunk*32+32)` (O `<< 5`) and calls `TypedStorageBuffer::gpu_write_ranges_nonoverlapping<T>(ranges, data)` (O: direct call; data = the full CPU instance array).
- puffin scope (O).
- Side effect: consumes (clears) the dirty bitset; stale tail slots inside a dirty chunk are uploaded but hidden via byte_map.

### `SparseInstanceBuffer::allocate` / `write_allocation` / `free_impl` — inline (I)
- `allocate(&self, count) -> AllocationHandle` (I): pops chunk ids from `free_chunks`, sets byte_map/active bits, builds `Allocation`, returns handle; impl$2::write_allocation::closure$0 uses `checked_sub` (O dep).
- `write_allocation(&self, handle, data)` (I): writes data at the allocation's slots + sets dirty bits.
- `free_impl(&self, handle)` (I): core of `take_out_waste` for one handle.

### `SparseInstanceBuffer::allocation_to_draw_multi_instanced<T>` — 0x1412e3a60 (O; RoofTile)
- `fn allocation_to_draw_multi_instanced(&self, handle) -> (u32, u32, u32)`.
- Unwraps `handle.allocation`; iterates its chunk ids; for each chunk: `start_slot = chunk*32`, `count = byte_map[chunk]`; accumulates contiguous slot runs into an output buffer (growable `Vec<(u32,u32)>` via `grow_one<Range<ResourceAccessIndex>>`); writes `(start_slot, count)` pairs; returns `(0, 4, range_count)`.
- The `(0, 4, count)` triple is I: `(first=0, stride=4, count)` for a multi-draw command builder; the pair buffer is consumed by the draw path. The output destination pointer is not visible in the decompile (U).

### `impl$3::to_draw_multi_instanced<T>` — 0x1412e4250 (O; RoofTile)
- Same shape as `allocation_to_draw_multi_instanced` but iterates the whole `active_bitset` (all active chunks), not one allocation. Returns `(0, 4, count)`.

### `SparseInstanceBuffer::to_draw_multi_instanced_impl` / `iter_*` — inline (I)
- Helpers over chunk bitmaps (`iter_chunk_bitmap`, `iter_all_chunks`, `iter_allocation_chunks`, `drain_chunk_bitmap`); `drain_chunk_bitmap` consumes a bitset while iterating (I). Used by the draw-range builders and gpu_flush.

### `impl$12::{as_any, as_any_mut, handle}` — 0x1412e6640..0x1412e6670 (O, 11-byte stubs)
- Trait-object interface to the buffer: `as_any(&self) -> &dyn Any`, `as_any_mut(&mut self) -> &mut dyn Any`, `handle(&self) -> &Handle<…>` (the instance-data handle). I: a game `Ssbo`-style trait shared with the ssbo.rs wrapper types (used by `AssetSsboLibrary::get_rg_buffer<T>`); trait name U.

## `SparseInstanceBufferWriter<T>` — 0x58 bytes (11 qwords) (O)

| off | size | field | status |
|---|---|---|---|
| 0x00 | 8 | `byte_map_ptr` | O |
| 0x08 | 8 | `byte_map_len` (chunk_count+1) | O |
| 0x10 | 8 | `active_bitset_ptr` | O |
| 0x18 | 8 | `active_bitset_len` (chunk_count/64) | O |
| 0x20 | 24 | `allocated_chunks: Vec<ChunkId>` (cap@0x20, ptr@0x28, len@0x30) | O |
| 0x38 | 8 | `instance_data_ptr` — writable instance buffer (mapped), element stride `size_of::<T>()` | O |
| 0x40 | 8 | `total_slots` (chunk_count*32) | O |
| 0x48 | 8 | `free_chunks: parking_lot::RwLock<Vec<u32>>` (refcount + Vec header, writer flag) | O |
| 0x50 | 8 | `dirty_bitset_ptr` | O |
| 0x58 | 8 | `dirty_bitset_len` | O |
| 0x60 | 4 | `current_chunk: u32` | O |

### `SparseInstanceBufferWriter::push<T>` — 0x1412e4450 (O; BrickTransform elem 0x60, chunk stride 0xc00)
- `fn push(&mut self, data: &T)`.
- `chunk = self.current_chunk`; bounds `chunk < byte_map_len` (O).
- If `byte_map[chunk] < 32` (not full): `slot = chunk*32 + byte_map[chunk]`; `byte_map[chunk] += 1`; bounds `slot < total_slots`; copy `*data` to `instance_data_ptr + slot*size_of::<T>()`; set the dirty bit for `chunk` (O).
- Else (full): lock `free_chunks`; if empty → do nothing (O); else pop chunk id, append to `allocated_chunks` (grow_one<ChunkId>); `byte_map[chunk]=1`; bounds `chunk*32 < total_slots`; copy `*data` to `instance_data_ptr + chunk*32*size_of::<T>()`; set active + dirty bits; `current_chunk = chunk` (O).
- Note: the instance buffer written by push is the mapped buffer directly; `gpu_flush` later re-uploads the dirty chunk range from the CPU array (redundant re-write, kept for the waste-reclaim path) (I).

### `SparseInstanceBufferWriter::clear<T>` — 0x1412e4760 (O; BrickTransform)
- `fn clear(&mut self)`.
- `self.current_chunk = byte_map_len - 1` (the full sentinel, forcing a fresh chunk allocation on the next push) (O/I).
- For each chunk in `allocated_chunks`: `byte_map[chunk]=0`; clear the active bit (O).
- Drains `allocated_chunks` back into `free_chunks` (reverse append; O) and empties the Vec (drop_in_place<Drain<ResourceHandle>> shown is the generic drain drop) (O).
- Dirty bits are NOT cleared (O): the dirty bitset still marks those chunks for a subsequent `gpu_flush` upload.

## Failure branches (O)
- All bitmap/byte-map accesses: bounds-check → `panic_bounds_check`.
- Handle unwraps: `unwrap_failed`.
- `gpu_flush`: order/len guards + `gpu_write_ranges_nonoverlapping` non-Ok → `unwrap_failed`.

## Integration notes
- `ChunkId` is a u32-sized id (stored as u32 in free stack / allocation arrays) (O).
- The `slot_meta` u32 table (0x40) and `field0` of `Allocation` have no decompiled consumer in WP2 — do not fabricate their semantics.
- `bitmap_set`/`bitmap_unset` (free functions, O deps `AtomicUsize::fetch_or`/`fetch_and`) implement the bitset ops used above; share them instead of re-inlining.
- The `(0, 4, count)` return of the draw-range builders is consumed by WP1 `DrawBuilder::draw_multi_instanced` / `multi_draw_indirect` (0xb8-byte command) — the stride `4` and pair encoding must stay consistent across packages.
