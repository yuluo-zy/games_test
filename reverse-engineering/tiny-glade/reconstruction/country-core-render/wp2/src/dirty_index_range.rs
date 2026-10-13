//! dirty_index_range.rs — `DirtyIndexRange` (atomic inclusive dirty window).
//! O/I/U tags per `contracts/dirty_index_range.md`.

use core::ops::Range;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Inclusive index window used by `ShadowedBuffer<T>`.
/// Empty sentinel (O): `start = usize::MAX`, `end = 0` → `is_empty()` = `start > end`.
#[repr(C)]
#[derive(Default)]
pub struct DirtyIndexRange {
    /// O — inclusive lower bound.
    pub start: AtomicUsize,
    /// O — inclusive upper bound.
    pub end: AtomicUsize,
}

impl DirtyIndexRange {
    /// O — empty sentinel (matches `ShadowedBuffer::new` / `gpu_flush` clear: (MAX,0)).
    pub fn empty() -> Self {
        Self {
            start: AtomicUsize::new(usize::MAX),
            end: AtomicUsize::new(0),
        }
    }

    /// I — inline; grows the window with fetch_min/fetch_max (O deps).
    pub fn add(&self, range: Range<usize>) {
        self.start.fetch_min(range.start, Ordering::Relaxed);
        self.end.fetch_max(range.end, Ordering::Relaxed);
    }

    /// I — inline (`AtomicUsize::load` deps O): empty when start > end.
    pub fn is_empty(&self) -> bool {
        self.start.load(Ordering::Relaxed) > self.end.load(Ordering::Relaxed)
    }

    /// I — inline; inclusive range when non-empty.
    pub fn to_range(&self) -> Option<Range<usize>> {
        if self.is_empty() {
            None
        } else {
            Some(self.start.load(Ordering::Relaxed)..self.end.load(Ordering::Relaxed) + 1)
        }
    }

    /// O — `clear`: reset to the empty sentinel.
    pub fn clear(&self) {
        self.start.store(usize::MAX, Ordering::Relaxed);
        self.end.store(0, Ordering::Relaxed);
    }
}

/// O — `impl$2::fmt` 0x141693b30: derived `Debug` (2 fields).
impl core::fmt::Debug for DirtyIndexRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DirtyIndexRange")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}
