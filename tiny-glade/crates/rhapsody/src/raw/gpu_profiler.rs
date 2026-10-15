//! 原 `gpu-profiler`（本地 crate，Cargo.lock 记 0.1.0、无 source；仅依赖 ash；`shared.rs` + `backend/ash.rs`）。
//!
//! 链路（机器码读出，B patched EXE）：
//! `Profiler::new`（0x140c10f20）→ `begin_frame`（0x140c11090）→ `ActiveProfiler::create_marker`（0x140c110f0）/
//! `write_marker`（0x140c11150）→ `end_frame`（0x1418c3910）→ 游戏侧 `ProfilerSubmittedFrame::retrieve_results`
//! （0x140294cf0）。
//!
//! 与前序分析文档的**纠正**：
//! * 查询池创建信息是 `sType=QUERY_POOL_CREATE_INFO(0xb)`、`queryType=TIMESTAMP(2)`、`queryCount=0x400`
//!   （栈上 `0x40000000002` = `{queryType=2, queryCount=0x400}`；旧文档把 sType 的 0xb 误读成 queryType）。
//! * `write_marker` 第 4 个实参（r9）是**本次写入的查询下标**（`lock xadd` 返回的旧计数），不是 marker：marker 写进
//!   索引缓冲 `buf[idx]`，`vkCmdWriteTimestamp(cmd, BOTTOM_OF_PIPE(0x2000), pool, idx)`。
//! * 索引缓冲 0x1ff8B = 0x3ff 个 u64，初值全是 `0xffffffff`（`0x142a16e60` 的 16B 模式 `{0xffffffff,0xffffffff}`
//!   重复铺满，尾部 3 个 u64 也是 0xffffffff）。
//! * `retrieve_results` 的排序是 `sort_unstable_by_key(tick)`（小样本 `<0x15` 走插入排序 0x1405c3580，否则 ipnsort
//!   0x1405c75b0；比较键是条目的 u64 字段，经 0x14067dd80 恒等访问器）。
//!
//! 与 `raw::profiler::GpuProfiler`（候选，每区段两条时间戳）无等价关系：原版是"每个 marker 一条 BOTTOM_OF_PIPE
//! 时间戳 + 一次 `CopyQueryPoolResults` 到 0x2000B 的 GPU→CPU 缓冲"。
use ash::vk;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// 查询池大小（`queryCount = 0x400`，begin_frame 重置 `[0,0x400)`）。
pub const QUERY_POOL_SIZE: u32 = 0x400;
/// 每帧最多 marker 数（索引缓冲容量 0x3ff；越界 panic）。
pub const MAX_MARKERS: usize = 0x3ff;
/// 索引缓冲的初值（`0xffffffff`）。
pub const INDEX_BUFFER_FILL: u64 = 0xffff_ffff;
/// `end_frame` 的 `vkCmdCopyQueryPoolResults` 标志：`64_BIT | WAIT`（原版立即数 3）、stride 8。
pub const COPY_FLAGS: vk::QueryResultFlags =
    vk::QueryResultFlags::from_raw(vk::QueryResultFlags::TYPE_64.as_raw() | vk::QueryResultFlags::WAIT.as_raw());

/// `shared::MarkerId`（u32 包装）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MarkerId(pub u32);
impl MarkerId {
    pub fn as_u64(self) -> u64 {
        u64::from(self.0)
    }
    /// 0x140c10e20：`u64::MAX` 是非法哨兵 → panic；其余截断低 32 位。
    pub fn from_u64(value: u64) -> Self {
        assert!(value != u64::MAX, "MarkerId::from_u64: u64::MAX is not a valid marker");
        Self(value as u32)
    }
}

/// `shared::NanoSecond`（0x14067dd80 / `from_raw_ns` 都是 `mov rax,rcx; ret`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NanoSecond(pub u64);

/// `shared::GpuTick`。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GpuTick(pub u64);
impl GpuTick {
    /// 0x140c10e50：`xor eax,eax; sub rcx,rdx; cmovae rax,rcx` = 饱和差。
    pub fn duration_since(self, earlier: GpuTick) -> GpuTickDuration {
        GpuTickDuration(self.0.saturating_sub(earlier.0))
    }
}

/// `shared::GpuTickDuration`。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GpuTickDuration(pub u64);
impl GpuTickDuration {
    /// 0x140c10e60：`(ticks as f64 * period as f64) as u64`（`period` 是 f32 第二参数，即
    /// `VkPhysicalDeviceLimits::timestampPeriod`；`as u64` 为饱和转换）。
    pub fn nanoseconds(self, timestamp_period: f32) -> NanoSecond {
        NanoSecond((self.0 as f64 * f64::from(timestamp_period)) as u64)
    }
}

/// `shared::FrameMarkers`：每帧 marker 名字表（原元素 16B = `&str`；这里用 `String`）。
/// `create_marker` **不去重**（0x140c110f0 直接 push），返回的 u32 就是下标。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FrameMarkers {
    names: Vec<String>,
}
impl FrameMarkers {
    pub fn create_marker(&mut self, name: &str) -> MarkerId {
        let id = self.names.len() as u32;
        self.names.push(name.to_owned());
        MarkerId(id)
    }
    /// 0x140c10ee0：返回完整 `&str`；越界 panic。
    pub fn marker_name(&self, id: MarkerId) -> &str {
        &self.names[id.0 as usize]
    }
    pub fn len(&self) -> usize {
        self.names.len()
    }
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

/// `backend::ash::Profiler`：`{index_buf(0x3ff×u64), query_pool, used_count}`。
pub struct Profiler {
    index_buf: Vec<AtomicU64>,
    pool: vk::QueryPool,
    used_count: AtomicU32,
}
impl Profiler {
    /// 0x140c10f20。原版创建失败直接 panic（`unwrap` 样式，0x142b1aaf8，17 字节文案）；这里返回错误。
    pub fn new(device: &ash::Device) -> Result<Self, vk::Result> {
        let info = vk::QueryPoolCreateInfo::default()
            .query_type(vk::QueryType::TIMESTAMP)
            .query_count(QUERY_POOL_SIZE);
        // SAFETY: `info` 有效；池由本结构保存，调用者负责 `destroy`。
        let pool = unsafe { device.create_query_pool(&info, None) }?;
        Ok(Self {
            index_buf: (0..MAX_MARKERS).map(|_| AtomicU64::new(INDEX_BUFFER_FILL)).collect(),
            pool,
            used_count: AtomicU32::new(0),
        })
    }
    /// 原版没有观察到销毁路径（池泄漏）；这里提供显式销毁以便测试不泄漏。
    ///
    /// # Safety
    /// 池不得仍被未完成的命令使用。
    pub unsafe fn destroy(&self, device: &ash::Device) {
        // SAFETY: 由调用者保证。
        unsafe { device.destroy_query_pool(self.pool, None) }
    }
    pub fn query_pool(&self) -> vk::QueryPool {
        self.pool
    }
    pub fn used_count(&self) -> u32 {
        self.used_count.load(Ordering::Acquire)
    }
    pub fn index_slot(&self, index: usize) -> u64 {
        self.index_buf[index].load(Ordering::Relaxed)
    }
    /// 0x140c11090：`vkCmdResetQueryPool(cmd, pool, 0, 0x400)`，`used_count = 0`。
    ///
    /// # Safety
    /// `cmd` 必须处于录制状态，且上一帧对池的使用已完成。
    pub unsafe fn begin_frame<'a>(&'a self, device: &ash::Device, cmd: vk::CommandBuffer) -> ActiveProfiler<'a> {
        // SAFETY: 由调用者保证。
        unsafe { device.cmd_reset_query_pool(cmd, self.pool, 0, QUERY_POOL_SIZE) };
        self.used_count.store(0, Ordering::Release);
        ActiveProfiler { markers: FrameMarkers::default(), profiler: self }
    }
}

/// `backend::ash::ActiveProfiler`：`{markers Vec, &Profiler @0x18}`。
pub struct ActiveProfiler<'a> {
    markers: FrameMarkers,
    profiler: &'a Profiler,
}
impl ActiveProfiler<'_> {
    /// 0x140c110f0。
    pub fn create_marker(&mut self, name: &str) -> MarkerId {
        self.markers.create_marker(name)
    }
    /// 0x140c11150。原子取下标 `idx`；`idx < 0x3ff` 才写索引缓冲 + 时间戳，否则 panic（index out of bounds）。
    ///
    /// # Safety
    /// `cmd` 必须处于录制状态，且本帧已 `begin_frame`。
    pub unsafe fn write_marker(&self, device: &ash::Device, cmd: vk::CommandBuffer, marker: MarkerId) {
        let idx = self.profiler.used_count.fetch_add(1, Ordering::AcqRel) as usize;
        assert!(idx < MAX_MARKERS, "index out of bounds: the len is {MAX_MARKERS} but the index is {idx}");
        self.profiler.index_buf[idx].store(marker.as_u64(), Ordering::Relaxed);
        // SAFETY: 由调用者保证。
        unsafe {
            device.cmd_write_timestamp(cmd, vk::PipelineStageFlags::BOTTOM_OF_PIPE, self.profiler.pool, idx as u32)
        }
    }
    /// 0x1418c3910：`CopyQueryPoolResults(pool, 0, used_count, dst, 0, stride 8, 64_BIT|WAIT)`；把索引缓冲前
    /// `used_count` 项（`as u32`，I）与 marker 表打包成 [`ProfilerSubmittedFrame`]。
    ///
    /// # Safety
    /// `cmd` 必须处于录制状态；`dst` 至少 `used_count * 8` 字节且带 `TRANSFER_DST`。
    pub unsafe fn end_frame(
        self,
        device: &ash::Device,
        cmd: vk::CommandBuffer,
        dst: vk::Buffer,
    ) -> ProfilerSubmittedFrame {
        let count = self.profiler.used_count.load(Ordering::Acquire);
        // SAFETY: 由调用者保证。
        unsafe {
            device.cmd_copy_query_pool_results(cmd, self.profiler.pool, 0, count, dst, 0, 8, COPY_FLAGS)
        };
        let take = (count as usize).min(MAX_MARKERS);
        let query_indices = self.profiler.index_buf[..take]
            .iter()
            .map(|slot| slot.load(Ordering::Relaxed) as u32)
            .collect();
        ProfilerSubmittedFrame { markers: self.markers, query_indices }
    }
}

/// `retrieve_results` 的一项（原 16B：`{u32 marker, u64 tick}`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MarkerTimestamp {
    pub marker: MarkerId,
    pub tick: GpuTick,
}

/// `backend::ash::ProfilerSubmittedFrame`。
pub struct ProfilerSubmittedFrame {
    pub markers: FrameMarkers,
    query_indices: Vec<u32>,
}
impl ProfilerSubmittedFrame {
    /// 由已知的 marker 表与查询下标构造（`end_frame` 之外的重建/测试入口）。
    pub fn from_parts(markers: FrameMarkers, query_indices: Vec<u32>) -> Self {
        Self { markers, query_indices }
    }
    pub fn query_indices(&self) -> &[u32] {
        &self.query_indices
    }
    /// 0x140294cf0：`timestamps` 是输出缓冲的映射内容（`ProfilerBuffer::mapped_slice`，u64 数组）。
    /// 把第 i 个查询下标（marker）与第 i 个时间戳配对，按 tick `sort_unstable_by_key` 排序。
    /// 原版有两处 panic（对齐检查 0x5a 字节文案、`count > len` 越界）；这里 `count > timestamps.len()` 同样 panic。
    pub fn retrieve_results(self, timestamps: &[u64]) -> (Vec<MarkerTimestamp>, FrameMarkers) {
        let count = self.query_indices.len();
        assert!(count <= timestamps.len(), "range end index {count} out of range for slice of length {}", timestamps.len());
        let mut entries: Vec<MarkerTimestamp> = self
            .query_indices
            .iter()
            .zip(timestamps)
            .map(|(&marker, &tick)| MarkerTimestamp { marker: MarkerId(marker), tick: GpuTick(tick) })
            .collect();
        entries.sort_unstable_by_key(|e| e.tick);
        (entries, self.markers)
    }
}
