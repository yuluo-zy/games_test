//! 可运行的 GPU 时间戳候选实现；原 puffin 输出格式与跨帧生产者尚未对照。
//!
//! 原版 `raw::profiler`（CALL-CHAIN：`TimedFrame::{new,send_to_puffin}`、`ProfilerBuffer`、
//! `create_gpu_profiler_output_buffer` 0x140bff4a0、`create_query_result_buffer` 0x140bff5a0、`mapped_slice`
//! 0x140bd3c80）是 `gpu-profiler` 与 `GraphicsDevice` 之间的胶水；`gpu-profiler` 本体见 [`super::gpu_profiler`]。
//! 本文件下方 `GpuProfiler`/`TimedFrame` 是候选（每区段两条时间戳），不是原版结构。
use crate::raw::{
    buffer::BufferDesc,
    device::VulkanDevice,
    error::Result as RawResult,
    managed::buffer::ManagedBuffer,
};
use ash::vk;
use parking_lot::Mutex;
use std::sync::Arc;

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
pub struct GpuProfiler {
    device: Arc<VulkanDevice>,
    pool: vk::QueryPool,
    max_spans: u32,
    period_ns: f64,
    valid_bits: u32,
    spans: Mutex<Vec<String>>,
}
impl GpuProfiler {
    pub fn new(
        device: Arc<VulkanDevice>,
        max_spans: u32,
        period_ns: f64,
        valid_bits: u32,
    ) -> Result<Self, vk::Result> {
        if valid_bits == 0 || valid_bits > 64 || !period_ns.is_finite() || period_ns <= 0.0 {
            return Err(vk::Result::ERROR_FEATURE_NOT_PRESENT);
        }
        let count = max_spans
            .checked_mul(2)
            .ok_or(vk::Result::ERROR_OUT_OF_HOST_MEMORY)?;
        let info = vk::QueryPoolCreateInfo::default()
            .query_type(vk::QueryType::TIMESTAMP)
            .query_count(count);
        let pool = unsafe { device.device.create_query_pool(&info, None) }?;
        Ok(Self {
            device,
            pool,
            max_spans,
            period_ns,
            valid_bits,
            spans: Mutex::new(Vec::new()),
        })
    }
    /// # Safety
    /// 上一帧的查询使用必须已完成，command 必须在合法录制状态。
    pub unsafe fn reset(&self, command: vk::CommandBuffer) {
        self.spans.lock().clear();
        unsafe {
            self.device
                .device
                .cmd_reset_query_pool(command, self.pool, 0, self.max_spans * 2)
        }
    }
    /// # Safety
    /// command 必须处于合法录制状态且本帧已经 reset；必须用同一个编号结束。
    pub unsafe fn begin(&self, command: vk::CommandBuffer, name: &str) -> Result<u32, vk::Result> {
        let mut spans = self.spans.lock();
        if spans.len() >= self.max_spans as usize {
            return Err(vk::Result::ERROR_OUT_OF_POOL_MEMORY);
        }
        let index = spans.len() as u32;
        spans.push(name.to_owned());
        unsafe {
            self.device.device.cmd_write_timestamp(
                command,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                self.pool,
                index * 2,
            )
        }
        Ok(index)
    }
    /// # Safety
    /// index 必须对应本帧已开始的区段且尚未结束。
    pub unsafe fn end(&self, command: vk::CommandBuffer, index: u32) {
        unsafe {
            self.device.device.cmd_write_timestamp(
                command,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                self.pool,
                index * 2 + 1,
            )
        }
    }
    /// 未完成时返回 NOT_READY，让上层选择下一帧再读，不强制等待。
    pub fn results(&self) -> Result<Vec<TimedFrame>, vk::Result> {
        let spans = self.spans.lock();
        if spans.is_empty() {
            return Ok(Vec::new());
        }
        let mut values = vec![0u64; spans.len() * 2];
        unsafe {
            self.device.device.get_query_pool_results(
                self.pool,
                0,
                &mut values,
                vk::QueryResultFlags::TYPE_64,
            )
        }?;
        let mask = if self.valid_bits == 64 {
            u64::MAX
        } else {
            (1u64 << self.valid_bits) - 1
        };
        Ok(spans
            .iter()
            .zip(values.chunks_exact(2))
            .map(|(name, v)| TimedFrame {
                name: name.clone(),
                milliseconds: ((v[1].wrapping_sub(v[0])) & mask) as f64 * self.period_ns
                    / 1_000_000.0,
            })
            .collect())
    }
}
impl Drop for GpuProfiler {
    fn drop(&mut self) {
        self.device
            .bury(super::graveyard::Doomed::QueryPool(self.pool));
    }
}

/// 原输出缓冲大小：`create_gpu_profiler_output_buffer`（0x140bff4a0）→ `create_query_result_buffer(size = 0x2000)`
/// （0x140bff5a0 的 `VkBufferCreateInfo`：`sType=12, size=r8=0x2000, usage=2(TRANSFER_DST), sharing=0`；分配名 `"buffer"`、
/// 位置 `GpuToCpu(4)`）。**纠正** raw-core ALGORITHMS 里的 "STORAGE|TRANSFER_DST"：usage 只有 `TRANSFER_DST`。
pub const GPU_PROFILER_OUTPUT_BYTES: u64 = 0x2000;

/// 输出缓冲描述（`GpuToCpu`、`TRANSFER_DST`、0x2000B）。
pub fn gpu_profiler_output_buffer_desc() -> BufferDesc {
    BufferDesc::new_gpu_to_cpu(GPU_PROFILER_OUTPUT_BYTES, vk::BufferUsageFlags::TRANSFER_DST)
}

/// 原 `ProfilerBuffer::mapped_slice`（0x140bd3c80）的读回形态：把输出缓冲解释为 u64 时间戳数组。
///
/// # Safety
/// 调用者必须保证覆盖该缓冲的 GPU 写入已完成且对宿主可见。
pub unsafe fn read_output_timestamps(buffer: &ManagedBuffer) -> RawResult<Vec<u64>> {
    let mut bytes = vec![0u8; GPU_PROFILER_OUTPUT_BYTES as usize];
    // SAFETY: 由调用者保证。
    unsafe { buffer.read_bytes_after_completion(0, &mut bytes) }?;
    Ok(bytes.chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect())
}
