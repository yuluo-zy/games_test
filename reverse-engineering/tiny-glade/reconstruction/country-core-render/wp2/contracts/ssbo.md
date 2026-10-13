# ssbo.rs — GPU storage-buffer primitives (TypedStorageBuffer / InstanceBuffer / MappedStorageBuffer)

Target: B patched EXE (`tiny-glade.exe` sha256 cf048e27…, PDB 2d9686b8…).
Original source: `crates/country-core/src/render/ssbo.rs` (checksum e0517a92…, version B).
Bundle: `skeleton/B/crates/country-core/src/render/ssbo.rs.{pdb.json,structures.json.gz,enriched.json.gz,dependencies.json}`.

Legend: **O**=Observation (machine code / decompile / PDB), **I**=Inference (names/call-graph/combination), **U**=Unknown (typed gap, not guessed).

## Producer/consumer context

- Producer: game systems allocate SSBOs through `AssetSsboLibrary::get_rg_buffer<T>()` and through `new_storage_buffer`-family constructors; the four buffer wrapper types are the CPU-facing gate to one rhapsody GPU buffer.
- Consumers: `pass_builder::pass_resource_access::{read_access_buffer,write_access_buffer}` (WP1) binds these via `rhapsody::rg::pass_api::bind`; `gpu_write_ranges_nonoverlapping` is called by `SparseInstanceBuffer::gpu_flush` and by per-system uploaders; `cpu_write` by critters/path systems.
- Reuse boundary (O, dependencies.json): `rhapsody::raw::{bytes::slice_as_bytes, buffer::{Buffer,BufferDesc}, device::GraphicsDevice}` (REUSED rhapsody — reference `reconstruction/rhapsody-rebuild`, do not reimplement), `bumpalo` 3.18.1 (temp arena), `puffin` 0.14.1 (profiling scopes), `std`/`alloc`/`core`. Game-owned: the four wrapper types + `BufferWriteError`.

## 1. `TypedStorageBuffer<T>` — 40 bytes (5 qwords)

Layout (O, from `TypedStorageBuffer::new` 0x141618e00 stores 5 qwords; `ShadowedBuffer::gpu_flush` uses offset 0x18 of the embedded copy as byte capacity):

| off | size | field | status |
|---|---|---|---|
| 0x00 | 8 | `buffer_lo` — rhapsody raw `Buffer` handle low | O |
| 0x08 | 8 | `buffer_hi` — handle high (rhapsody `Buffer` is 16 bytes) | O |
| 0x10 | 8 | `length` — element count (usize) | O (write path scales by `size_of::<T>()`) |
| 0x18 | 8 | `byte_size` — GPU buffer byte capacity; `gpu_flush` guards `byte_size >= end*elem_size` | O |
| 0x20 | 8 | `field5` | U (typed gap; not referenced by any decompiled function in WP2) |

### `TypedStorageBuffer::new<T>` — 0x141618e00 (O)
- Creates a GPU-only storage buffer via `BufferDesc::new()` + `BufferDesc::alignment()` + `GraphicsDevice::create_buffer<Option<&[u8]>>()`.
- Panics (`panic_fmt`) when the size argument is 0 (O). Unwraps the create result (`unwrap_failed` unless Ok; Ok tag observed as `0xb`).
- The exact argument convention (element count vs byte size; which register is `&GraphicsDevice`) is partially I: impl$0/impl$4/impl$7 pass different (count, byte_size, device) arrangements.

### `TypedStorageBuffer::new_arbitrary<T>` (U/I)
- Inlined, no standalone machine code. Name implies a constructor with an arbitrary byte layout/alignment (not element-typed). Semantics not recovered.

### `TypedStorageBuffer::gpu_write` — inline (I)
- PDB: line 156, calls `gpu_write_with_offset` (line 208). Contract: `fn gpu_write(&mut self, data: &[T]) -> Result<(), BufferWriteError>` delegates to `gpu_write_with_offset(data, 0)`.

### `TypedStorageBuffer::gpu_write_with_offset` — inline (I)
- PDB deps (O): `slice_as_bytes` (rhapsody), `size_of_val`, `Result::map_err`. Contract: bounds-check `(offset+data.len())*size_of::<T>() <= byte_size`, else return `BufferWriteError`; build byte range and call `device.update_buffer_bytes(&self.buffer, ranges, slice_as_bytes(data))`.

### `TypedStorageBuffer::gpu_write_ranges_nonoverlapping<T>` — 0x14161ab00 (O; second instance 0x141fd3700)
- `fn gpu_write_ranges_nonoverlapping(&mut self, ranges: &[Range<usize>], data: &[T]) -> Result<(), BufferWriteError>`.
- No-op when `ranges.is_empty()` (O: early return).
- Allocates a `bumpalo::Bump` arena; asserts `ranges.len()*0x10 <= bump cap` (`oom`); converts each 16-byte `Range<usize>` to a byte range by multiplying both endpoints by `size_of::<T>()` (O: BrickTransform ×0x60=96, WoodenPlank ×0x40=64).
- Calls `GraphicsDevice::update_buffer_bytes(&self.buffer, &byte_ranges, data_bytes)` (O: `slice_as_bytes(data)`). Result tag `0xb`=Ok else `unwrap_failed`.
- Bump dropped at exit (walks chunk chain, frees each chunk via `__rust_dealloc`) (O).
- Side effect: one GPU write of non-overlapping ranges from contiguous CPU data; ranges are element-indexed and must not overlap (no validation of overlap — I).

### `TypedStorageBuffer::gpu_write_element<T>` — inline (U/I)
- No standalone code; name implies single-element write; exact signature not recovered.

## 2. `InstanceBuffer<T>` — 48 bytes (6 qwords) (O)

`{ instance_count: usize @0x00, buffer: TypedStorageBuffer<T> @0x08 }` (O from `impl$7::new_storage_buffer` 0x1416207e0 stores `[0]=count`, `[1..5]`=TSB).

### `impl$7::new_storage_buffer<T>` — 0x1416207e0 (O; T=NaniDrawListOffsetDefaults, elem 0x1b0=432)
- `fn new_storage_buffer(count: usize, byte_size: usize) -> InstanceBuffer<T>`.
- If `count == 0`: reads `T::default_value()` (Option, tag `1`=Some); Some → allocates a 0x1b0-byte CPU copy of the default, calls `TypedStorageBuffer::new`, `instance_count = 1`; None → `instance_count = 0`. (I: default instance = 1 slot carrying the constant default struct.)
- Else: asserts `byte_size % size_of::<T>() == 0` and `count % 4 == 0` (`assert_failed<usize,usize>`, O); `instance_count = byte_size / size_of::<T>()`.

### `InstanceBuffer::new / set_instance_count / instance_count / gpu_write / gpu_write_subset` — inline (U/I)
- No standalone machine code. From names: `new` (constructor), `set_instance_count`/`instance_count` (O field at 0x00), `gpu_write`/`gpu_write_subset` (write to the embedded TSB; `_subset` implies index/count form). Exact signatures not recovered.

## 3. `MappedStorageBuffer<T>` — 56 bytes (7 qwords) (O)

`{ mapped_ptr: *mut T @0x00, length: usize @0x08, buffer: Buffer @0x10 (2 qwords), f4/f5/f6 @0x20..0x38 }` (O from `impl$4::new_storage_buffer` 0x14161ad70 stores 7 qwords; `impl$6::cpu_write` reads `[0]`=ptr, `[1]`=length). f4–f6: U (typed gap; not referenced by decompiled WP2 functions).

### `impl$4::new_storage_buffer<T>` — 0x14161ad70 (O; T=Cobblestone2RasterStagingSsbo, others RasterStaging/TerrainHeight/…)
- `fn new_storage_buffer(initial: Option<&[T]>, device) -> MappedStorageBuffer<T>` (I for arg role; O for the Option branch).
- None → allocates a 0x40000 (256 KiB) zeroed temporary CPU block (`__rust_alloc` + `memset`, O). Some → asserts byte length ≤ 0x40000 (`panic_fmt` else, O).
- Creates + maps the GPU buffer: `BufferDesc::new`/`alignment` + `create_buffer<Option<&[u8]>>` + `Buffer::mapped_slice_mut()` (O). Unwraps result (tag 0xb Ok).
- Writes `[0]=mapped_ptr`, `[1]=length`, `[2..3]=buffer`. The temporary CPU block is freed when the None-path flag is set (O: `if (cVar1) __rust_dealloc`). Extra `panic()` when mapped length < requested length (O).
- Purpose: persistent CPU-mapped SSBO for per-frame CPU writes (RasterStaging, TerrainHeight, WaterMask, GardenMask, critters, MouseDepth, …).

### `impl$6::cpu_write<T>` — 0x14230edb0 (O; T=ButterflySsbo elem 0x20=32)
- `fn cpu_write(&self, data: &[T])`.
- puffin scope (O).
- `assert!(data.len() <= self.length)` (O: `len_mismatch_fail`); copies `min(data.len(), length)` elements from `data` to `self.mapped_ptr` via `copy_from_nonoverlapping`-style memcpy (O: `memcpy(dst, src, n*0x20)`).

### `impl$6::mapped_data / mapped_data_mut` — inline (U/I)
- Accessors returning `&[T]`/`&mut [T]` over `mapped_ptr..+length`. Signatures inferred from names.

## 4. `BufferWriteError` (U/I)

- Enum type present (O: `drop_in_place<BufferWriteError>`, ssbo.rs structures entity). Exact variants not recovered.
- `impl$22::fmt` (Debug for `Result<(), BufferWriteError>`, 15 instances; 0x1408c21e0): tag `0xb` → unit variant via `Formatter::write_str` (i.e. `Ok`); else 1-field tuple variant via `debug_tuple_field1_finish` (i.e. `Err(e)`) (O).
- `impl$15::fmt` (Display, 0 standalone instances) (I).

## Failure branches (O unless noted)

- `TypedStorageBuffer::new`: size==0 → panic; create failure → unwrap_failed.
- `gpu_write_ranges_nonoverlapping`: bump OOM → `oom`; bump range end mismatch → `expect_failed`; update_buffer_bytes non-Ok → unwrap_failed.
- `impl$4::new_storage_buffer`: byte_len>0x40000 → panic; alloc OOM → `handle_error`; map failure → unwrap_failed; mapped len check → panic.
- `cpu_write`: data longer than capacity → `len_mismatch_fail` panic.

## Notes for cross-package integration (WP1/WP3/WP4)

- The `0xb` Ok tag is an artifact of Rust enum layout, not a semantic constant; do not hard-code it into Rust logic.
- `slice_as_bytes` + `update_buffer_bytes` live in REUSED rhapsody; reference `reconstruction/rhapsody-rebuild`, do not reimplement.
- Consumer contracts that reference these types must use the field layout above (`TypedStorageBuffer[0x18]=byte_size` is load-bearing for `ShadowedBuffer::gpu_flush` and `SparseInstanceBuffer::gpu_flush` guards).
