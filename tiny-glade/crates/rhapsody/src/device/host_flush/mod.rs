//! Deferred host-write flush bookkeeping: which written ranges of non-coherent mapped memory
//! still need `vkFlushMappedMemoryRanges`, coalesced and issued once before each submit.
use crate::{Error, Result};
use ash::vk;

/// When a host write to mapped memory must be flushed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HostFlush {
    /// Coherent memory: never flushed.
    None,
    /// Flush the written atoms right after the copy.
    Now,
    /// Queue the written atoms on the device; flushed once before the next submit.
    Deferred,
}
pub(crate) fn host_write_flush(properties: vk::MemoryPropertyFlags, deferred: bool) -> HostFlush {
    if properties.contains(vk::MemoryPropertyFlags::HOST_COHERENT) {
        HostFlush::None
    } else if deferred {
        HostFlush::Deferred
    } else {
        HostFlush::Now
    }
}

/// Per-page merged ranges beyond which a page flushes its hull instead.
const MAX_DIRTY_RANGES_PER_PAGE: usize = 16;

struct DirtyPage {
    id: u64,
    memory: vk::DeviceMemory,
    /// Sorted, disjoint, non-adjacent `[start, end)` memory ranges; atom
    /// aligned by construction. `end == u64::MAX` means `VK_WHOLE_SIZE`.
    ranges: smallvec::SmallVec<[(u64, u64); 4]>,
}
impl DirtyPage {
    fn insert(&mut self, start: u64, end: u64) {
        let (mut start, mut end) = (start, end);
        self.ranges.retain(|&mut (s, e)| {
            if s <= end && start <= e {
                start = start.min(s);
                end = end.max(e);
                false
            } else {
                true
            }
        });
        let at = self.ranges.partition_point(|&(s, _)| s < start);
        self.ranges.insert(at, (start, end));
        if self.ranges.len() > MAX_DIRTY_RANGES_PER_PAGE {
            let hull = (self.ranges[0].0, self.ranges[self.ranges.len() - 1].1);
            self.ranges.clear();
            self.ranges.push(hull);
        }
    }
}

/// Device-wide queue of non-coherent dynamic-constants writes whose flush is
/// deferred to the next queue submit. Each page keeps its dirty atoms merged;
/// all pages are flushed with one `vkFlushMappedMemoryRanges` call.
#[derive(Default)]
pub(crate) struct HostFlushQueue {
    dirty: std::sync::atomic::AtomicBool,
    pages: parking_lot::Mutex<Vec<DirtyPage>>,
    flush_calls: std::sync::atomic::AtomicU64,
}
impl HostFlushQueue {
    /// Queue `[offset, offset + size)` of `memory` (atom-aligned, or
    /// `VK_WHOLE_SIZE`) written for allocation `id`.
    pub(crate) fn record(
        &self,
        id: u64,
        memory: vk::DeviceMemory,
        offset: u64,
        size: u64,
    ) -> Result<()> {
        if size == 0 {
            return Ok(());
        }
        let end = if size == vk::WHOLE_SIZE {
            u64::MAX
        } else {
            offset
                .checked_add(size)
                .ok_or_else(|| Error::invalid_usage("flush range overflows"))?
        };
        let mut pages = self.pages.lock();
        match pages.iter_mut().find(|page| page.id == id) {
            Some(page) => page.insert(offset, end),
            None => {
                pages.try_reserve(1).map_err(|_| Error::OutOfHostMemory)?;
                let mut ranges = smallvec::SmallVec::new();
                ranges.push((offset, end));
                pages.push(DirtyPage { id, memory, ranges });
            }
        }
        self.dirty.store(true, std::sync::atomic::Ordering::Release);
        Ok(())
    }
    /// Drop the pending ranges of a freed allocation.
    pub(crate) fn forget(&self, id: u64) {
        if !self.dirty.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        self.pages.lock().retain(|page| page.id != id);
    }
    /// Flush every pending range with one call of `flush`. Ranges stay
    /// pending when it fails.
    pub(crate) fn flush_with(
        &self,
        flush: impl FnOnce(&[vk::MappedMemoryRange<'_>]) -> Result<()>,
    ) -> Result<()> {
        if !self.dirty.load(std::sync::atomic::Ordering::Acquire) {
            return Ok(());
        }
        let mut pages = self.pages.lock();
        let count = pages.iter().map(|page| page.ranges.len()).sum();
        let mut ranges: smallvec::SmallVec<[vk::MappedMemoryRange<'_>; 8]> =
            smallvec::SmallVec::new();
        ranges.try_reserve(count).map_err(|_| Error::OutOfHostMemory)?;
        for page in pages.iter() {
            for &(start, end) in &page.ranges {
                ranges.push(
                    vk::MappedMemoryRange::default()
                        .memory(page.memory)
                        .offset(start)
                        .size(if end == u64::MAX {
                            vk::WHOLE_SIZE
                        } else {
                            end - start
                        }),
                );
            }
        }
        if ranges.is_empty() {
            self.dirty.store(false, std::sync::atomic::Ordering::Release);
            return Ok(());
        }
        self.flush_calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match flush(&ranges) {
            Ok(()) => {
                pages.clear();
                // Pending and in-progress ranges both remain dirty until the
                // native flush finishes while holding the same page lock.
                self.dirty.store(false, std::sync::atomic::Ordering::Release);
                Ok(())
            }
            Err(error) => Err(error),
        }
    }
    /// Number of `vkFlushMappedMemoryRanges` calls issued (statistics).
    pub(crate) fn flush_calls(&self) -> u64 {
        self.flush_calls.load(std::sync::atomic::Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests;
