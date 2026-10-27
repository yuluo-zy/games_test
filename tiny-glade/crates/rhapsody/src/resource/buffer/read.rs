//! Host reads from buffers after completion.
use crate::Error;
use crate::Result;
use super::Buffer;
use super::host::HOST_ACCESS_TIMEOUT_NS;

impl Buffer {
    fn read_impl(&self, offset: u64, bytes: &mut [u8], wait_ns: Option<u64>) -> Result<()> {
        let len = bytes.len();
        self.host_access(offset, len as u64, false, wait_ns, |base| unsafe {
            // SAFETY: the range was validated against the mapped buffer.
            std::ptr::copy_nonoverlapping(base.add(offset as usize), bytes.as_mut_ptr(), len)
        })
    }

    /// 在已确认本缓冲 GPU 操作完成后读回，保留非 coherent 内存失效处理，不做整设备等待。
    ///
    /// # Safety
    /// 调用者必须等待覆盖本缓冲全部在飞访问的 fence/timeline 提交凭据，
    /// 并保证复制期间没有其他 GPU 或宿主并发访问。GPU 写入还需已建立 HOST_READ 可见性。
    /// 此调用只依赖本缓冲完成，不推进设备全局完成 epoch。
    pub unsafe fn read_bytes_after_completion(&self, offset: u64, bytes: &mut [u8]) -> Result<()> {
        self.read_impl(offset, bytes, None)
    }

    /// Read after overlapping GPU writes complete, waiting at most
    /// `HOST_ACCESS_TIMEOUT_NS` (then `Timeout`).
    /// Inspect an existing persistent mapping only when all prior overlapping
    /// GPU writers have completed. Busy work returns false without running the
    /// callback. Non-coherent memory is invalidated before inspecting it.
    pub fn try_with_host_read(&self, offset: u64, bytes: u64, read: impl FnOnce()) -> Result<bool> {
        match self.host_access(offset, bytes, false, None, |_| read()) {
            Ok(()) => Ok(true),
            Err(Error::Timeout) => Ok(false),
            Err(error) => Err(error),
        }
    }

    pub fn read_bytes(&self, offset: u64, bytes: &mut [u8]) -> Result<()> {
        self.read_impl(offset, bytes, Some(HOST_ACCESS_TIMEOUT_NS))
    }

    /// [`Self::read_bytes`] with an explicit wait bound in nanoseconds.
    pub fn read_bytes_timeout(&self, offset: u64, bytes: &mut [u8], timeout_ns: u64) -> Result<()> {
        self.read_impl(offset, bytes, Some(timeout_ns))
    }
}
