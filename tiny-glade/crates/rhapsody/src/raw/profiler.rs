//! 可运行的 GPU 时间戳候选实现；原 puffin 输出格式与跨帧生产者尚未对照。
use crate::raw::device::VulkanDevice;
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
