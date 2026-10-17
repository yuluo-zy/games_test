//! GPU timestamp readback buffer helpers. Query ownership lives in the production graph backend.
use crate::raw::{buffer::BufferDesc, error::Result as RawResult, managed::buffer::ManagedBuffer};
use ash::vk;

#[derive(Clone, Debug)]
pub struct TimedFrame {
    pub name: String,
    pub milliseconds: f64,
}
impl TimedFrame {
    pub fn new(name: impl Into<String>, milliseconds: f64) -> Self {
        Self {
            name: name.into(),
            milliseconds,
        }
    }
    /// 原 `send_to_puffin`：把本帧统计投递到 puffin 分析器。
    /// 本工程未启用 puffin 依赖，保留空操作并记录数值供外部采样。
    pub fn send_to_puffin(&self) {
        let _ = self.milliseconds;
    }
}
pub const GPU_PROFILER_OUTPUT_BYTES: u64 = 0x2000;

pub fn gpu_profiler_output_buffer_desc() -> BufferDesc {
    BufferDesc::new_gpu_to_cpu(
        GPU_PROFILER_OUTPUT_BYTES,
        vk::BufferUsageFlags::TRANSFER_DST,
    )
}

/// 原 `ProfilerBuffer::mapped_slice`（0x140bd3c80）的读回形态：把输出缓冲解释为 u64 时间戳数组。
///
/// # Safety
/// 调用者必须保证覆盖该缓冲的 GPU 写入已完成且对宿主可见。
pub unsafe fn read_output_timestamps(buffer: &ManagedBuffer) -> RawResult<Vec<u64>> {
    let mut bytes = vec![0u8; GPU_PROFILER_OUTPUT_BYTES as usize];
    // SAFETY: 由调用者保证。
    unsafe { buffer.read_bytes_after_completion(0, &mut bytes) }?;
    Ok(bytes
        .as_chunks::<8>()
        .0
        .iter()
        .map(|c| u64::from_le_bytes(*c))
        .collect())
}
