//! Host writes into buffers, immediate and deferred.
use crate::Error;
use crate::Result;
use super::Buffer;
use super::host::HOST_ACCESS_TIMEOUT_NS;

impl Buffer {
    fn write_impl(&self, offset: u64, bytes: &[u8], wait_ns: Option<u64>) -> Result<()> {
        self.host_access(offset, bytes.len() as u64, true, wait_ns, |base| unsafe {
            // SAFETY: the range was validated against the mapped buffer.
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), base.add(offset as usize), bytes.len())
        })
    }

    /// 以单次主机范围事务填充字节；在飞的重叠 GPU 使用返回 Timeout。
    /// 复用写入的所有权、版本登记与非 coherent flush，不分配临时零缓冲。
    pub fn fill_mapped_bytes_unsynchronized(&self, offset: u64, len: u64, value: u8) -> Result<()> {
        let start = usize::try_from(offset)
            .map_err(|_| Error::invalid_usage("映射偏移超过主机地址范围"))?;
        let count =
            usize::try_from(len).map_err(|_| Error::invalid_usage("映射长度超过主机地址范围"))?;
        self.host_access(offset, len, true, None, |base| unsafe {
            // SAFETY: host_access 已检查范围、映射、同步和所有权。
            std::ptr::write_bytes(base.add(start), value, count)
        })
    }

    /// Write after overlapping GPU uses complete, waiting at most
    /// `HOST_ACCESS_TIMEOUT_NS` (then `Timeout`).
    /// Mutate an existing persistent mapping under the same range reservation,
    /// GPU-completion wait and non-coherent flush used by `write_bytes`.
    /// The callback borrows the caller's existing mapping; no second mutable
    /// slice or full-buffer shadow is manufactured here. It must only change
    /// the specified range, and must not recursively submit/access this buffer.
    pub fn with_host_write(&self, offset: u64, bytes: u64, edit: impl FnOnce()) -> Result<()> {
        let mut panic = None;
        let result = self.host_access(offset, bytes, true, Some(HOST_ACCESS_TIMEOUT_NS), |_| {
            // Finish the flush and publish the host revision even if an edit
            // panics after changing a prefix. Resume the original panic only
            // after the host-access transaction released all native locks.
            panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(edit)).err();
        });
        if let Some(payload) = panic {
            std::panic::resume_unwind(payload);
        }
        result
    }

    pub fn write_bytes(&self, offset: u64, bytes: &[u8]) -> Result<()> {
        self.write_impl(offset, bytes, Some(HOST_ACCESS_TIMEOUT_NS))
    }

    /// [`Self::write_bytes`] with an explicit wait bound in nanoseconds.
    pub fn write_bytes_timeout(&self, offset: u64, bytes: &[u8], timeout_ns: u64) -> Result<()> {
        self.write_impl(offset, bytes, Some(timeout_ns))
    }

    /// Write several `(offset, bytes)` segments with one completion check and
    /// a single flush covering their union (none for coherent memory).
    /// Waits like [`Self::write_bytes`]. Segments must lie inside the buffer;
    /// overlapping segments are written in order.
    pub fn write_segments(&self, segments: &[(u64, &[u8])]) -> Result<()> {
        let mut start = u64::MAX;
        let mut end = 0u64;
        for &(offset, bytes) in segments {
            let segment_end = offset
                .checked_add(bytes.len() as u64)
                .filter(|end| *end <= self.size)
                .ok_or_else(|| Error::invalid_usage("映射访问越过缓冲区边界"))?;
            if !bytes.is_empty() {
                start = start.min(offset);
                end = end.max(segment_end);
            }
        }
        if start >= end {
            return Ok(());
        }
        self.host_access(
            start,
            end - start,
            true,
            Some(HOST_ACCESS_TIMEOUT_NS),
            |base| {
                for &(offset, bytes) in segments {
                    // SAFETY: every segment was validated against the buffer size.
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            bytes.as_ptr(),
                            base.add(offset as usize),
                            bytes.len(),
                        )
                    }
                }
            },
        )
    }

    /// 不等待在飞 GPU 工作的写入。
    ///
    /// 未完成的重叠 GPU 使用（未知范围按整缓冲处理）返回 Timeout；非 coherent 内存按原子粒度检查。
    pub fn write_bytes_unsynchronized(&self, offset: u64, bytes: &[u8]) -> Result<()> {
        self.write_impl(offset, bytes, None)
    }

    /// Fill `[offset, offset + len)` of a mapped buffer in place, with the same
    /// synchronization and flush as [`Self::write_bytes_unsynchronized`].
    /// Callers that assemble data from several slices write each slice straight
    /// into mapped memory instead of packing them into a temporary first.
    pub(crate) fn write_with_unsynchronized(
        &self,
        offset: u64,
        len: u64,
        fill: impl FnOnce(&mut [u8]),
    ) -> Result<()> {
        let count =
            usize::try_from(len).map_err(|_| Error::invalid_usage("映射长度超过主机地址范围"))?;
        let start = usize::try_from(offset)
            .map_err(|_| Error::invalid_usage("映射偏移超过主机地址范围"))?;
        self.host_access(offset, len, true, None, |base| {
            // SAFETY: host_access validated the range, the mapping, and that no
            // unfinished GPU access overlaps it; the slice is exclusively ours.
            fill(unsafe { std::slice::from_raw_parts_mut(base.add(start), count) })
        })
    }

    /// [`Self::write_bytes_unsynchronized`] whose non-coherent flush is
    /// deferred to the device's next queue submit (one call for all queued
    /// ranges). Only for persistently mapped dynamic-constants pages.
    pub(crate) fn write_bytes_deferred_flush(&self, offset: u64, bytes: &[u8]) -> Result<()> {
        if self.write_idle_constants_page(offset, bytes)? {
            return Ok(());
        }
        self.host_access_with(
            offset,
            bytes.len() as u64,
            true,
            true,
            None,
            |base| unsafe {
                // SAFETY: the range was validated against the mapped buffer.
                std::ptr::copy_nonoverlapping(
                    bytes.as_ptr(),
                    base.add(offset as usize),
                    bytes.len(),
                )
            },
        )
    }
}
