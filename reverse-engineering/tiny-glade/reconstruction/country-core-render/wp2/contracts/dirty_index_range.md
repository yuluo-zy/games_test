# dirty_index_range.rs — `DirtyIndexRange`

Target: B (EXE cf048e27…, PDB 2d9686b8…). Original source: `crates/country-core/src/render/dirty_index_range.rs` (checksum d6f67568…).
Bundle: `skeleton/B/crates/country-core/src/render/dirty_index_range.rs.{pdb.json,structures.json.gz,enriched.json.gz,dependencies.json}`.
Procedures (O): `impl$2::fmt` (Debug) 0x141693b30 (119 B); `is_empty`, `add`, `to_range`, `clear` are inline (inst 0).

Legend: **O**=Observation, **I**=Inference, **U**=Unknown.

## Producer/consumer
- Producer: `ShadowedBuffer<T>` embeds a `DirtyIndexRange` as its first two qwords (see `contracts/shadowed_buffer.md`); `get_mut`/`write`/`copy` call `add`; `gpu_flush` reads it.
- Reuse (O): `core::sync::atomic::{AtomicUsize, fetch_min, fetch_max, load}` only.

## Layout (O via ShadowedBuffer embedding + Debug derive)
`DirtyIndexRange { start: AtomicUsize @0x00, end: AtomicUsize @0x08 }` — **inclusive** on both ends.
- Empty sentinel (O): `start = usize::MAX`, `end = 0` (gpu_flush no-ops when `start > end`; new/clear write (MAX,0)).
- `impl$2::fmt` 0x141693b30 (O): `debug_struct_field2_finish` → derived `Debug` for a 2-field struct.

## Methods (I, deps O)
- `add(&self, range: Range<usize>)` (O deps fetch_min/fetch_max): `start.fetch_min(range.start, …)`, `end.fetch_max(range.end, …)` — grows the dirty window to cover `range`.
- `is_empty(&self) -> bool` (O dep `AtomicUsize::load`): `self.start.load() > self.end.load()`.
- `to_range(&self) -> Option<RangeInclusive<usize>>` (I; deps O: calls `is_empty` + two loads): `None` if empty else `Some(start..=end)`.
- `clear(&self)` (O no deps): reset to `(usize::MAX, 0)`.

## Failure branches
- None (no panics observed; the caller (gpu_flush) validates order/len against the buffer).

## Integration notes
- Inclusive semantics is load-bearing: the consumer converts `end` to exclusive via `end+1` (see `ShadowedBuffer::gpu_flush`).
- `AtomicUsize` usage is O (deps); the `Relaxed`/`SeqCst` ordering is U.
