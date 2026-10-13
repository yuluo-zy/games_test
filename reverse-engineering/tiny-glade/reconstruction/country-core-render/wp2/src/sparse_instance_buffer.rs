//! sparse_instance_buffer.rs — `SparseInstanceBuffer<T>` + `SparseInstanceBufferWriter<T>`.
//! GPU-driven sparse instancing: 32-instance chunks, byte map of active counts,
//! two atomic bitmaps (active/dirty), a free-chunk stack, per-allocation chunk lists.
//! O/I/U tags per `contracts/sparse_instance_buffer.md`.

use core::any::Any;
use core::sync::atomic::{AtomicUsize, Ordering};

use parking_lot::RwLock;

use crate::ssbo::rhapsody_boundary::{self, GraphicsDevice, ResourceHandle};
use crate::ssbo::TypedStorageBuffer;

// ---------------------------------------------------------------------------
// Per-type sparse layout (O constants)
// ---------------------------------------------------------------------------

/// Per-element-type constants for a `SparseInstanceBuffer<T>`.
/// Concrete SSBO types (RoofTileSsbo=0x2000/96…, BrickTransformSsbo=0x10000/96,
/// WoodenPlankSsbo=0x10000/64, SparkleSsbo=0x6000/…) implement this in their own modules.
/// I — trait shape; the chunk counts are O.
pub trait SparseSsbo: Sized {
    const CHUNK_COUNT: usize;
    const ELEMENT_SIZE: usize;
}

/// Per-chunk instance capacity: 32 (0x20) slots (O).
pub const SLOTS_PER_CHUNK: usize = 32;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// O — `Allocation`: one allocation owns a set of chunk ids.
#[derive(Debug)]
pub struct Allocation {
    /// U — pointer field freed on dealloc (role not recovered).
    pub _field0: *mut u8,
    /// O — chunk id array (u32).
    pub chunk_ids_ptr: *mut u32,
    /// O — chunk id count.
    pub chunk_ids_len: u64,
}

/// O/I — `AllocationHandle`: opaque word + inner allocation pointer.
#[derive(Debug, Clone, Copy)]
pub struct AllocationHandle {
    /// O — first qword; `is_null()` == this == 0.
    pub word: u64,
    /// I — inner allocation (unwrapped before use; `unwrap_failed` when null).
    pub allocation: Option<core::ptr::NonNull<Allocation>>,
}

impl AllocationHandle {
    /// O — `AllocationHandle::is_null` 0x140985f30: first qword zero ⇒ null.
    pub fn is_null(&self) -> bool {
        self.word == 0
    }
}

/// O — `ChunkId` is stored as a u32.
pub type ChunkId = u32;

/// `SparseInstanceBuffer<T>` — 0x90 bytes (18 qwords). O.
#[derive(Debug)]
pub struct SparseInstanceBuffer<T: SparseSsbo> {
    /// O — RwLock<Vec<ResourceHandle>> (pending GPU handles released on dealloc).
    pub pending_resource_handles: RwLock<alloc::vec::Vec<ResourceHandle>>,
    /// O — chunk_count (per-type constant).
    pub chunk_count: usize,
    /// O — free chunk id stack (Vec<u32>: ptr@0x10, len@0x18), init descending.
    pub free_chunks: alloc::vec::Vec<u32>,
    /// O — per-chunk active-instance-count byte map; len = chunk_count+1; byte[chunk_count]=0x20 sentinel.
    pub byte_map: alloc::vec::Vec<u8>,
    /// O — active bitmap (1 bit per chunk), len = chunk_count/64.
    pub active_bitset: alloc::vec::Vec<AtomicUsize>,
    /// U — per-instance-slot u32 table (allocated/zeroed O; consumer not decompiled).
    pub slot_meta: alloc::vec::Vec<u32>,
    /// O — dirty bitmap (1 bit per chunk), len = chunk_count/64.
    pub dirty_bitset: alloc::vec::Vec<AtomicUsize>,
    /// O — allocations registry.
    pub allocations: alloc::sync::Arc<RwLock<alloc::vec::Vec<Allocation>>>,
    /// O — typed instance-data GPU buffer.
    pub buffer: TypedStorageBuffer<T>,
}

impl<T: SparseSsbo> SparseInstanceBuffer<T> {
    /// O — `impl$10::new_storage_buffer<T>` 0x1412e4a90 / 0x1412e5800 / 0x1412e5170 / 0x1412e5f50.
    pub fn new_storage_buffer(device: &dyn GraphicsDevice) -> Self {
        let chunks = T::CHUNK_COUNT;
        let es = T::ELEMENT_SIZE;
        // byte_map: chunk_count+1 zeros, sentinel byte[chunk_count]=0x20.
        let mut byte_map = alloc::vec![0u8; chunks + 1];
        *byte_map.last_mut().unwrap() = SLOTS_PER_CHUNK as u8;
        // slot_meta: chunk_count*32 u32 zeros (role U).
        let slot_meta = alloc::vec![0u32; chunks * SLOTS_PER_CHUNK];
        // free stack: descending [chunk_count-1 .. 0].
        let free_chunks: alloc::vec::Vec<u32> = (0..chunks as u32).rev().collect();
        // bitmaps: (0..chunks/64).map(|_| AtomicUsize::new(0)).collect() (O from_iter).
        let bits = chunks / usize::BITS as usize;
        let active_bitset = (0..bits).map(|_| AtomicUsize::new(0)).collect();
        let dirty_bitset = (0..bits).map(|_| AtomicUsize::new(0)).collect();
        // Arc<RwLock<Vec<Allocation>>> with empty Vec (O header init).
        let allocations = alloc::sync::Arc::new(RwLock::new(alloc::vec::Vec::new()));
        let buffer = TypedStorageBuffer::new(chunks * SLOTS_PER_CHUNK, chunks * SLOTS_PER_CHUNK * es, device);

        Self {
            pending_resource_handles: RwLock::new(alloc::vec::Vec::new()),
            chunk_count: chunks,
            free_chunks,
            byte_map,
            active_bitset,
            slot_meta,
            dirty_bitset,
            allocations,
            buffer,
        }
    }

    /// O — `buffer` accessor (5-byte stub 0x140005c50).
    pub fn buffer(&self) -> &TypedStorageBuffer<T> {
        &self.buffer
    }

    /// I — inline; pops `count` chunk ids from `free_chunks`, builds an `Allocation`.
    pub fn allocate(&self, _count: usize) -> AllocationHandle {
        unreachable!("allocate: inlined, not recovered (contract-only)")
    }

    /// I — inline; writes data at the allocation's slots + sets dirty bits.
    pub fn write_allocation(&self, _handle: &AllocationHandle, _data: &[T]) {
        unreachable!("write_allocation: inlined, not recovered (contract-only)")
    }

    /// O — `hide` 0x1412e3c00: clear active bits for the allocation's chunks.
    pub fn hide(&self, handle: &AllocationHandle) {
        let inner = handle.allocation.expect("hide: null allocation");
        let alloc = unsafe { inner.as_ref() };
        let ids = unsafe { core::slice::from_raw_parts(alloc.chunk_ids_ptr, alloc.chunk_ids_len as usize) };
        for &id in ids {
            let (word, bit) = (id as usize >> 6, id as usize & 63);
            assert!(word < self.active_bitset.len(), "hide: bounds");
            self.active_bitset[word].fetch_and(!(1usize << bit), Ordering::Relaxed);
        }
    }

    /// O — `show` 0x1412e3c80: set active bits for the allocation's chunks.
    pub fn show(&self, handle: &AllocationHandle) {
        let inner = handle.allocation.expect("show: null allocation");
        let alloc = unsafe { inner.as_ref() };
        let ids = unsafe { core::slice::from_raw_parts(alloc.chunk_ids_ptr, alloc.chunk_ids_len as usize) };
        for &id in ids {
            let (word, bit) = (id as usize >> 6, id as usize & 63);
            assert!(word < self.active_bitset.len(), "show: bounds");
            self.active_bitset[word].fetch_or(1usize << bit, Ordering::Relaxed);
        }
    }

    /// O — `take_out_waste` 0x1412e3d00: reclaim freed allocations.
    /// For each allocation: zero byte_map + active bits for its chunk ids, push those ids back
    /// onto the free stack, release the allocation box. Runs at the start of `gpu_flush`.
    /// (&mut self chosen for compileability; the observed ABI is a single pointer, so &self/&mut
    /// self are indistinguishable — the original interior-mutability scheme for byte_map is U.)
    pub fn take_out_waste(&mut self) {
        let mut allocs = self.allocations.write();
        for alloc in allocs.drain(..) {
            let ids = unsafe { core::slice::from_raw_parts(alloc.chunk_ids_ptr, alloc.chunk_ids_len as usize) };
            for &id in ids {
                let id = id as usize;
                self.byte_map[id] = 0;
                bitmap_unset(&self.active_bitset, id);
            }
            // push chunk ids back onto the free stack (reverse append; O).
            for &id in ids.iter().rev() {
                self.free_chunks.push(id);
            }
            if !alloc._field0.is_null() {
                unsafe { drop(alloc::boxed::Box::from_raw(alloc._field0)) };
            }
        }
    }

    /// O — `gpu_flush` 0x1412e2f10: reclaim waste, then upload dirty chunks.
    pub fn gpu_flush(&mut self, device: &dyn GraphicsDevice) {
        self.take_out_waste();
        // Scan dirty_bitset, clear each consumed word, accumulate non-overlapping chunk runs,
        // then one `gpu_write_ranges_nonoverlapping<T>` for element ranges [chunk*32, chunk*32+32).
        let mut runs: alloc::vec::Vec<core::ops::Range<usize>> = alloc::vec::Vec::new();
        let mut run_start: Option<usize> = None;
        let mut prev_end: usize = 0;
        for (w, word) in self.dirty_bitset.iter().enumerate() {
            let mut bits = word.swap(0, Ordering::Relaxed); // clear the word (O)
            while bits != 0 {
                let tz = bits.trailing_zeros() as usize;
                let chunk = w * 64 + tz;
                match run_start {
                    None => {
                        run_start = Some(chunk);
                        prev_end = chunk + 1;
                    }
                    Some(_) if chunk == prev_end => prev_end += 1,
                    Some(s) => {
                        runs.push(s * SLOTS_PER_CHUNK..prev_end * SLOTS_PER_CHUNK);
                        run_start = Some(chunk);
                        prev_end = chunk + 1;
                    }
                }
                bits &= bits - 1;
            }
        }
        if let Some(s) = run_start {
            runs.push(s * SLOTS_PER_CHUNK..prev_end * SLOTS_PER_CHUNK);
        }
        if runs.is_empty() {
            return;
        }
        // data source: the buffer's persistent-mapped CPU view (I).
        let data = unsafe { core::slice::from_raw_parts(
            rhapsody_boundary::buffer_cpu_ptr(&self.buffer.buffer) as *const T,
            self.buffer.length,
        ) };
        let _ = self.buffer.gpu_write_ranges_nonoverlapping(&runs, data, device);
    }

    /// O — `allocation_to_draw_multi_instanced` 0x1412e3a60: draw ranges for one allocation.
    /// Returns (first=0, stride=4, range_count). The (start_slot, count) pairs are written to an
    /// output buffer consumed by the draw path (destination pointer U).
    pub fn allocation_to_draw_multi_instanced(&self, handle: &AllocationHandle) -> (u32, u32, u32) {
        let inner = handle.allocation.expect("allocation_to_draw_multi_instanced: null");
        let alloc = unsafe { inner.as_ref() };
        let ids = unsafe { core::slice::from_raw_parts(alloc.chunk_ids_ptr, alloc.chunk_ids_len as usize) };
        let mut runs: alloc::vec::Vec<(u32, u32)> = alloc::vec::Vec::new();
        let mut run_start: Option<u32> = None;
        let mut prev_end: u32 = 0;
        for &id in ids {
            let id = id as usize;
            assert!(id < self.byte_map.len(), "allocation_to_draw: bounds");
            let start = (id * SLOTS_PER_CHUNK) as u32;
            let count = self.byte_map[id] as u32;
            if run_start.is_none() || start != prev_end {
                if let Some(s) = run_start.take() {
                    runs.push((s, prev_end - s));
                }
                run_start = Some(start);
            }
            prev_end = start + count;
        }
        if let Some(s) = run_start {
            runs.push((s, prev_end - s));
        }
        let _ = runs; // destination write is U
        (0, 4, runs.len() as u32)
    }

    /// O — `impl$3::to_draw_multi_instanced` 0x1412e4250: same over the whole active bitmap.
    pub fn to_draw_multi_instanced(&self) -> (u32, u32, u32) {
        let mut runs: alloc::vec::Vec<(u32, u32)> = alloc::vec::Vec::new();
        let mut run_start: Option<u32> = None;
        let mut prev_end: u32 = 0;
        for (w, word) in self.active_bitset.iter().enumerate() {
            let mut bits = word.load(Ordering::Relaxed);
            while bits != 0 {
                let tz = bits.trailing_zeros() as usize;
                let chunk = w * 64 + tz;
                let start = (chunk * SLOTS_PER_CHUNK) as u32;
                let count = self.byte_map[chunk] as u32;
                if run_start.is_none() || start != prev_end {
                    if let Some(s) = run_start.take() {
                        runs.push((s, prev_end - s));
                    }
                    run_start = Some(start);
                }
                prev_end = start + count;
                bits &= bits - 1;
            }
        }
        if let Some(s) = run_start {
            runs.push((s, prev_end - s));
        }
        let _ = runs; // destination write is U
        (0, 4, runs.len() as u32)
    }
}

// ---------------------------------------------------------------------------
// Writer
// ---------------------------------------------------------------------------

/// `SparseInstanceBufferWriter<T>` — 0x58 bytes (11 qwords). O.
#[derive(Debug)]
pub struct SparseInstanceBufferWriter<T: SparseSsbo> {
    /// O — byte map pointer.
    pub byte_map_ptr: *mut u8,
    /// O — byte map length (chunk_count+1).
    pub byte_map_len: usize,
    /// O — active bitmap pointer.
    pub active_bitset_ptr: *mut AtomicUsize,
    /// O — active bitmap length (chunk_count/64).
    pub active_bitset_len: usize,
    /// O — chunks claimed by this writer session.
    pub allocated_chunks: alloc::vec::Vec<ChunkId>,
    /// O — writable instance buffer pointer (mapped; stride = size_of::<T>()).
    pub instance_data_ptr: *mut u8,
    /// O — total instance slots (chunk_count*32).
    pub total_slots: usize,
    /// O — free-chunk stack (RwLock<Vec<u32>>).
    pub free_chunks: RwLock<alloc::vec::Vec<u32>>,
    /// O — dirty bitmap pointer.
    pub dirty_bitset_ptr: *mut AtomicUsize,
    /// O — dirty bitmap length.
    pub dirty_bitset_len: usize,
    /// O — current chunk id (u32).
    pub current_chunk: u32,
    /// Marker so `T` participates in the writer type (element stride is `size_of::<T>()`).
    pub _marker: core::marker::PhantomData<T>,
}

impl<T: SparseSsbo> SparseInstanceBufferWriter<T> {
    /// O — `push` 0x1412e4450.
    pub fn push(&mut self, data: &T) {
        let chunk = self.current_chunk as usize;
        assert!(chunk < self.byte_map_len, "push: chunk bounds");
        let es = T::ELEMENT_SIZE;
        let byte_map = unsafe { core::slice::from_raw_parts_mut(self.byte_map_ptr, self.byte_map_len) };
        if (byte_map[chunk] as usize) < SLOTS_PER_CHUNK {
            // chunk not full: append to current chunk's slot.
            let slot = chunk * SLOTS_PER_CHUNK + byte_map[chunk] as usize;
            byte_map[chunk] += 1;
            assert!(slot < self.total_slots, "push: slot bounds");
            unsafe {
                core::ptr::copy_nonoverlapping(
                    (data as *const T) as *const u8,
                    self.instance_data_ptr.add(slot * es),
                    es,
                );
            }
            // dirty bit for this chunk.
            let (w, b) = (chunk >> 6, chunk & 63);
            assert!(w < self.dirty_bitset_len, "push: dirty bounds");
            unsafe { &*self.dirty_bitset_ptr.add(w) }.fetch_or(1usize << b, Ordering::Relaxed);
        } else {
            // chunk full: pop a fresh chunk from the free stack.
            let mut free = self.free_chunks.write();
            if let Some(id) = free.pop() {
                let chunk = id as usize;
                self.allocated_chunks.push(id);
                byte_map[chunk] = 1;
                assert!(chunk * SLOTS_PER_CHUNK < self.total_slots, "push: slot bounds");
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        (data as *const T) as *const u8,
                        self.instance_data_ptr.add(chunk * SLOTS_PER_CHUNK * es),
                        es,
                    );
                }
                let (w, b) = (chunk >> 6, chunk & 63);
                assert!(w < self.active_bitset_len, "push: active bounds");
                unsafe { &*self.active_bitset_ptr.add(w) }.fetch_or(1usize << b, Ordering::Relaxed);
                assert!(w < self.dirty_bitset_len, "push: dirty bounds");
                unsafe { &*self.dirty_bitset_ptr.add(w) }.fetch_or(1usize << b, Ordering::Relaxed);
                self.current_chunk = id;
            }
            // free stack empty → no-op (O).
        }
    }

    /// O — `clear` 0x1412e4760: reset current chunk to the full sentinel, release claimed chunks.
    pub fn clear(&mut self) {
        // Reset current chunk to byte_map_len-1 (the sentinel byte = 0x20 = full), forcing a
        // fresh chunk allocation on the next push (O/I).
        self.current_chunk = (self.byte_map_len - 1) as u32;
        let byte_map = unsafe { core::slice::from_raw_parts_mut(self.byte_map_ptr, self.byte_map_len) };
        for &id in &self.allocated_chunks {
            let chunk = id as usize;
            byte_map[chunk] = 0;
            let (w, b) = (chunk >> 6, chunk & 63);
            assert!(w < self.active_bitset_len, "clear: active bounds");
            unsafe { &*self.active_bitset_ptr.add(w) }.fetch_and(!(1usize << b), Ordering::Relaxed);
        }
        // Drain claimed chunks back onto the free stack (reverse append; O).
        let mut free = self.free_chunks.write();
        while let Some(id) = self.allocated_chunks.pop() {
            free.push(id);
        }
        // Dirty bits are NOT cleared (O): a later gpu_flush still uploads those chunks.
    }
}

// ---------------------------------------------------------------------------
// Trait-object interface (O: impl$12)
// ---------------------------------------------------------------------------

/// Game `Ssbo`-style trait implemented by the sparse buffer (and, I, the ssbo.rs wrappers).
/// Used by `AssetSsboLibrary::get_rg_buffer<T>` (WP1). Trait name U.
pub trait Ssbo {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn handle(&self) -> &ResourceHandle;
}

impl<T: SparseSsbo + 'static> Ssbo for SparseInstanceBuffer<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn handle(&self) -> &ResourceHandle {
        // O — 5-byte stub 0x140005c50 returns the embedded buffer handle.
        unsafe { &*(&self.buffer.buffer as *const rhapsody_boundary::Buffer as *const ResourceHandle) }
    }
}

/// O — `bitmap_set`: atomic set bit in a `Vec<AtomicUsize>` bitmap.
pub fn bitmap_set(bits: &[AtomicUsize], index: usize) {
    let (w, b) = (index >> 6, index & 63);
    bits[w].fetch_or(1usize << b, Ordering::Relaxed);
}

/// O — `bitmap_unset`: atomic clear bit in a `Vec<AtomicUsize>` bitmap.
pub fn bitmap_unset(bits: &[AtomicUsize], index: usize) {
    let (w, b) = (index >> 6, index & 63);
    bits[w].fetch_and(!(1usize << b), Ordering::Relaxed);
}
