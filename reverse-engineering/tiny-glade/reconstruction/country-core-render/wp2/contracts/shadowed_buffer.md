# shadowed_buffer.rs — `ShadowedBuffer<T>` (CPU shadow + dirty-range GPU flush)

Target: B (EXE cf048e27…, PDB 2d9686b8…). Original source: `crates/country-core/src/render/shadowed_buffer.rs` (checksum 5e095de0…).
Bundle: `skeleton/B/crates/country-core/src/render/shadowed_buffer.rs.{pdb.json,structures.json.gz,enriched.json.gz,dependencies.json}`.
Procedure inventory (O): 22 `ShadowedBuffer::new<T>` + 17 `ShadowedBuffer::gpu_flush<T>` standalone instances (module 1653), plus inlined `buffer/get_mut/copy/write`.

Legend: **O**=Observation, **I**=Inference, **U**=Unknown.

## Producer/consumer

- Producer: nani instance-data owners (`UberInstanceData`, `SolidAlbedoMapInstanceData`, `PlacementDataSsbo`, `GpuInstance`, …) build a shadowed buffer from CPU data and mutate it per frame.
- Consumers: `get_mut`/`write`/`copy` mutate the CPU shadow (marking dirty); `gpu_flush` (called in the nani draw path before the pass reads the SSBO) uploads the dirty range.
- Reuse (O): `rhapsody::raw::{buffer::BufferDesc, device::GraphicsDevice, bytes::slice_as_bytes}` (REUSED), `core::sync::atomic::{AtomicUsize,fetch_min,fetch_max}`, `puffin`. Game-owned: `ShadowedBuffer<T>` + `dirty_index_range::DirtyIndexRange`.

## `ShadowedBuffer<T>` — 72 bytes (9 qwords) (O)

| off | size | field | status |
|---|---|---|---|
| 0x00 | 8 | `dirty.start` — `AtomicUsize`, inclusive; `usize::MAX` = empty sentinel | O |
| 0x08 | 8 | `dirty.end` — `AtomicUsize`, inclusive | O |
| 0x10 | 8 | `data.ptr` — CPU shadow element pointer (`Vec<T>`/`Box<[T]>`) | O |
| 0x18 | 8 | `data.len` — element count | O |
| 0x20 | 40 | `buffer: TypedStorageBuffer<T>` (5 qwords, layout per `contracts/ssbo.md`) | O |
| — | — | no capacity field retained for `data` | O (only ptr/len stored; buffer never grows after `new`) |

The dirty pair is exactly `DirtyIndexRange` (contract `dirty_index_range.md`): `add` = `fetch_min(start)` + `fetch_max(end)`, empty = `start > end`.

### `ShadowedBuffer::new<T>` — 0x14168f870 (O; variants 392/412 bytes differ only by element size/alignment)
- `fn new(initial: …, device) -> ShadowedBuffer<T>` (I: `initial` is a `Vec<T>` moved in, or copied `&[T]`; O: realloc/shrink of the CPU buffer to fit the initial length, `__rust_realloc`/`__rust_dealloc`, `handle_error` on OOM).
- Creates the GPU `TypedStorageBuffer<T>` for the same element count (O: calls `TypedStorageBuffer::new`).
- Stores `data` (ptr,len), `buffer` (5 qwords), `dirty = (usize::MAX, 0)` (O).

### `ShadowedBuffer::buffer` — inline (I)
- Accessor `&self -> &TypedStorageBuffer<T>` (no deps; O in dependencies.json).

### `ShadowedBuffer::get_mut` — inline (I, deps O)
- Deps (O): `DirtyIndexRange::add`, `AtomicUsize::fetch_min/fetch_max`. Contract: returns `&mut [T]` over the CPU shadow and marks the covered index range dirty via `dirty.add(range)`. Exact parameter shape (whole buffer vs sub-range) I.

### `ShadowedBuffer::copy` — inline (I, deps O)
- Deps (O): `DirtyIndexRange::add` + atomics. Contract: copies within the CPU shadow (`src` → `dst`, count or slice form) and marks the affected range dirty. Exact shape I (not recovered).

### `ShadowedBuffer::write` — inline (I, deps O)
- Deps (O): `DirtyIndexRange::add` + atomics. Contract: writes `&[T]` into the CPU shadow at an index and marks dirty. Exact shape I.

### `ShadowedBuffer::gpu_flush<T>` — 0x141691b30 (elem 0x30=48; variants 1623/1639 bytes differ by element size e.g. Uber 0x40) (O)
- `fn gpu_flush(&mut self, device: &GraphicsDevice)`.
- No-op when `dirty.start > dirty.end` (empty) (O).
- Computes `exclusive_end = dirty.end + 1`; asserts order (`slice_index_order_fail` if `exclusive_end < start`) and length (`slice_end_index_len_fail` if `length < exclusive_end`) (O).
- Guards GPU capacity: `self.buffer.byte_size >= exclusive_end * size_of::<T>()` else `unwrap_failed` (O).
- Calls `device.update_buffer_bytes(&self.buffer, &[(start*es .. exclusive_end*es)], slice_as_bytes(&self.data[start..exclusive_end]))` (O).
- Clears dirty to `(usize::MAX, 0)` (O). puffin scope (O).

## Failure branches (O)
- `new`: OOM → `handle_error`.
- `gpu_flush`: order/len violations → panic; capacity guard or non-Ok `update_buffer_bytes` → `unwrap_failed`.

## Integration notes
- Dirty range is inclusive on both ends; `exclusive_end = end+1` is the load-bearing conversion (gpu_flush) — do not model as half-open.
- `dirty_index_range::DirtyIndexRange::add` must be shared (WP2 owns it); WP1/WP3 consumers that mutate a ShadowedBuffer must call `get_mut`/`write`/`copy` (which mark dirty) — direct `data` access would skip the flush.
