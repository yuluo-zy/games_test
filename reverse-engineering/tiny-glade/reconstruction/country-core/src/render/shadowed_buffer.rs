//! shadowed_buffer.rs — `ShadowedBuffer<T>` (CPU shadow + dirty-range GPU flush).
//! O/I/U tags per `contracts/shadowed_buffer.md`.

use core::sync::atomic::Ordering;

use crate::render::dirty_index_range::DirtyIndexRange;
use crate::render::ssbo::TypedStorageBuffer;
use crate::render::ssbo::rhapsody_boundary::{self, GraphicsDevice};

/// `ShadowedBuffer<T>` — 72 bytes (9 qwords). O.
/// Layout: dirty pair, data (Box<[T]> = ptr+len), then `TypedStorageBuffer<T>` (5 qwords).
/// The CPU `data` box is never grown after `new` (no capacity field retained — O).
#[repr(C)]
#[derive(Debug)]
pub struct ShadowedBuffer<T> {
    /// O — inclusive dirty window; `usize::MAX/0` = empty.
    pub dirty: DirtyIndexRange,
    /// O — CPU shadow (fat Box<[T]>: ptr @0x10, len @0x18). I: Box<[T]> (into_boxed_slice-like).
    pub data: alloc::boxed::Box<[T]>,
    /// O — GPU typed buffer (layout per contracts/ssbo.md).
    pub buffer: TypedStorageBuffer<T>,
}

impl<T> ShadowedBuffer<T> {
    /// O — `ShadowedBuffer::new<T>` 0x14168f870.
    /// Copies the initial data into an owned CPU box sized to `initial` (realloc/shrink to fit,
    /// `handle_error` on OOM), creates the GPU `TypedStorageBuffer<T>` of the same length, and
    /// resets dirty to empty.
    pub fn new(initial: &[T], device: &dyn GraphicsDevice) -> Self {
        // CPU shadow: fresh owned allocation sized to `initial` (cap==len; not retained).
        let es = core::mem::size_of::<T>();
        let mut shadow: alloc::vec::Vec<T> = alloc::vec::Vec::with_capacity(initial.len());
        unsafe {
            core::ptr::copy_nonoverlapping(
                initial.as_ptr() as *const u8,
                shadow.as_mut_ptr() as *mut u8,
                initial.len() * es,
            );
        }
        shadow.shrink_to_fit();
        let data = shadow.into_boxed_slice();

        let buffer = TypedStorageBuffer::new(initial.len(), initial.len() * es, device);

        Self {
            dirty: DirtyIndexRange::empty(),
            data,
            buffer,
        }
    }

    /// I — accessor.
    pub fn buffer(&self) -> &TypedStorageBuffer<T> {
        &self.buffer
    }

    /// I — inline; returns a mutable view over the CPU shadow and marks the whole buffer dirty.
    pub fn get_mut(&mut self) -> &mut [T] {
        self.dirty.add(0..self.data.len());
        &mut self.data
    }

    /// I — inline; copy within the CPU shadow + mark dirty (raw copy, no `T: Copy` bound).
    pub fn copy(&mut self, dst: usize, src: usize, count: usize) {
        self.dirty.add(dst..(dst + count).min(self.data.len()));
        let es = core::mem::size_of::<T>();
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.data.as_ptr().add(src) as *const u8,
                self.data.as_mut_ptr().add(dst) as *mut u8,
                count * es,
            );
        }
    }

    /// I — inline; write into the CPU shadow at `index` + mark dirty.
    pub fn write(&mut self, index: usize, data: &[T]) {
        self.dirty.add(index..index + data.len());
        let es = core::mem::size_of::<T>();
        unsafe {
            core::ptr::copy_nonoverlapping(
                data.as_ptr() as *const u8,
                self.data.as_mut_ptr().add(index) as *mut u8,
                data.len() * es,
            );
        }
    }

    /// O — `ShadowedBuffer::gpu_flush<T>` 0x141691b30 (elem 0x30; variants differ by element size).
    /// No-op when dirty is empty; uploads `[dirty.start ..= dirty.end]` (exclusive = end+1)
    /// via one `update_buffer_bytes`; clears dirty.
    pub fn gpu_flush(&mut self, device: &dyn GraphicsDevice) {
        if self.dirty.is_empty() {
            return;
        }
        let start = self.dirty.start.load(Ordering::Relaxed);
        let end = self.dirty.end.load(Ordering::Relaxed);
        let exclusive_end = end + 1;
        assert!(exclusive_end >= start, "gpu_flush: slice index order");
        assert!(exclusive_end <= self.data.len(), "gpu_flush: slice end index len");

        let es = core::mem::size_of::<T>();
        assert!(self.buffer.byte_size >= exclusive_end * es, "gpu_flush: buffer too small");

        let data_bytes = rhapsody_boundary::slice_as_bytes(&self.data[start..exclusive_end]);
        let byte_ranges = [(start * es, exclusive_end * es)];
        let _ = device.update_buffer_bytes(&self.buffer.buffer, &byte_ranges, data_bytes);

        self.dirty.clear();
    }
}
