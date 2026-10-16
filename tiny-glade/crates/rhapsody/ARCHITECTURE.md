# rhapsody 架构与详细设计

本文是 rhapsody 重构的目标架构与各模块详细设计，进度与阶段见 [REFACTOR_PLAN.md](REFACTOR_PLAN.md)。

**唯一标准：正确性与高性能。** 原版 Tiny Glade 的地址、内存布局、panic 行为与错误文案不再约束设计；原版 oracle 测试已删除；对外 API 允许破坏性变更（下游只有 `tiny-glade`、`tg-text`，随改随迁）。

---

## 1. 现状问题（重构动机）

| # | 问题 | 证据 |
| --- | --- | --- |
| 0 | 主渲染路径不闭合：渲染图结果到不了屏幕 | `VulkanGraphSession` 不接触交换链；`SwapchainSyncSlots`（`rg/frame.rs`）、`Swapchain::submit_and_present`、`compile_presentation_graph` 均无生产调用者；外部同步只收时间线信号量，acquire/present 必需的二值信号量接不进来 |
| 1 | 调试日志兼作控制数据且无限增长 | `SequenceRuntime.events: Vec<String>` 从不清空（1000 帧 4324 → 27504 条）；`rg/vulkan.rs::execute_built` 用 `split(':')` 解析它得到命令序号并决定是否 poison |
| 2 | GPU 完成/回收有 5 套真相源 | graveyard epoch + `retire_timeline`、`SequenceRuntime`、`SubmissionEngine`（经 `mirror_submission_state` 整份拷贝）、`CommandBufferSeqTracker` + 销毁桶、`ManagedResources` 退役记录 |
| 3 | 时间线值在提交前分配 | `submit::DeviceState.next_timeline` 跨队列共享、提交前占用；提交失败后等待该值会永久挂起，旧修复只能 CPU 补发 signal |
| 4 | 两套设备对象 | `VulkanDevice` 与 `GraphicsDevice`（包 `Arc<VulkanDevice>`），未 `attach_graphics_device` 时回收行为不同且无报错 |
| 5 | 全局 `queue_lock` | 一把锁同时保护提交、回收、宿主映射、交换链重建（31 处），曾导致 acquire 持锁阻塞 3 秒 |
| 6 | 错误模型不一致 | `Result<_, String>` 23 处、20 个互不相通的错误枚举、"照原版 panic" 与 Err 混用、poison 全有或全无 |
| 7 | 资源身份类型擦除 | 图后端用不透明 `u64` token + `RwLock` 映射表；过期句柄只能运行时查表发现 |
| 8 | 多队列不可配置 | 跨队列族独占资源直接拒绝；lavapipe 单族单队列，辅助队列路径从未执行 |
| 9 | 测试测不到整体 | GPU 测试只读回像素、从不 present；无长时间运行测试 |

---

## 2. 目标架构

```
rhapsody
├─ error.rs            统一错误 Error/ErrorKind/Result                       ✅ 已实现
├─ sync/               GPU 完成的唯一真相源                                  ✅ 已实现
│  ├─ mod.rs           QueueId、SyncPoint、SyncSet、CompletedValues
│  ├─ timeline.rs      Timelines：每队列提交锁 + 时间线
│  └─ retire.rs        Retirement<T>：按 SyncSet 延迟回收
├─ device/             设备与队列
│  ├─ queue_config.rs  QueueConfig、plan_queues                             ✅ 已实现
│  ├─ mod.rs           Device（唯一设备对象）                                P2
│  ├─ init.rs          实例/物理设备选择/逻辑设备创建（DeviceConfig）         P2
│  └─ memory.rs        分配器封装（gpu-allocator，持久映射）                  P2
├─ resource/           语义化资源身份                                        P4
│  ├─ handle.rs        Handle<T>（slotmap 代数键）、ResourceName(Ustr)
│  ├─ buffer.rs / texture.rs / view.rs / sampler.rs
│  └─ tracked.rs       last_use: SyncSet 登记与 drop→Retirement
├─ pipeline/           shader、descriptor、render pass、管线缓存              P4/P7（逻辑沿用 raw/）
├─ command/            帧环命令池、录制器                                    P3/P7
├─ graph/              渲染图：声明 → 编译 → 执行                            P4–P6（编译逻辑沿用 rg/graph.rs）
├─ swapchain.rs        交换链（自带锁、按图像槽的二值信号量）                 P5
└─ frame.rs            FrameLoop：begin → acquire → main → present → submit → present   P5
```

依赖方向（只允许向下）：

```
frame ─► graph ─► command ─► resource ─► device ─► sync ─► error
   └────► swapchain ──────────────────────┘
```

---

## 3. 详细设计

### 3.1 错误模型（`src/error.rs`，已实现）

```rust
pub enum Error {
    DeviceLost,
    OutOfDeviceMemory,
    OutOfHostMemory,
    SurfaceOutOfDate,
    SurfaceLost,
    Timeout,
    Unsupported(Cow<'static, str>),
    InvalidUsage { what: Cow<'static, str>, source: Option<Box<dyn Error + Send + Sync>> },
    Vulkan(vk::Result),
}
pub enum ErrorKind { DeviceLost, OutOfMemory, SurfaceOutOfDate, SurfaceLost, Timeout, Unsupported, InvalidUsage, Vulkan }
```

规则：

1. 公共 API 一律 `crate::Result<T>`；不得出现 `Result<_, String>`、`format!("{e:?}")` 字符串化。
2. `From<vk::Result>` 负责分类：`DEVICE_LOST`→`DeviceLost`；`OUT_OF_DEVICE_MEMORY`/`OUT_OF_POOL_MEMORY`→`OutOfDeviceMemory`；`OUT_OF_HOST_MEMORY`→`OutOfHostMemory`；`OUT_OF_DATE_KHR`/`FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`→`SurfaceOutOfDate`；`SURFACE_LOST_KHR`→`SurfaceLost`；`TIMEOUT`→`Timeout`；缺特性/扩展/格式/驱动→`Unsupported`；其余→`Vulkan`。
3. 调用方可触发的误用（越界、过期句柄、阶段顺序错误、不合法描述符）→ `Error::invalid_usage(..)`（同时记 error 日志），**不 panic**。
4. `debug_assert!` 只用于 rhapsody 内部不变量（违反即 rhapsody 自身 bug）。`unwrap`/`expect` 只允许出现在不变量已由类型或前置校验保证之处，并就地注释理由。
5. 模块内部错误枚举（图编译、绑定、temporal 等）用 `thiserror` 派生，并实现 `From<X> for crate::Error`：携带的 Vulkan 失败保留其类别，其余映射为 `InvalidUsage { source: Some(x) }`。
6. 帧失败只回滚本帧；仅 `DeviceLost` 使设备/会话永久不可用（粘滞）。
7. 窗口最小化、交换链过期不是错误路径上的"误用"：由 `FrameOutcome::Skipped/Recreated` 表达，不记 error 日志。

### 3.2 同步与完成（`src/sync/`，已实现）

**数据类型**

- `QueueId(u8)`，`0..MAX_QUEUES`（8）。`QueueId(0)` 恒为 graphics。
- `SyncPoint { queue, value }`：某队列时间线上的一个值。
- `SyncSet { values: [u64; MAX_QUEUES] }`：每队列至多一个依赖值（0 = 不依赖），`insert`/`merge` 取逐队列最大值；`Copy`、64 字节、无堆分配，适合热路径。
- `CompletedValues`：各队列已完成值快照，只增不减；`CompletedValues::all()` 表示"全部完成"（device idle 或设备丢失后）。

**`Timelines<A: TimelineApi>`**

```rust
pub trait TimelineApi: Send + Sync {
    fn counter_value(&self, semaphore: vk::Semaphore) -> Result<u64>;
    fn wait_all(&self, semaphores: &[vk::Semaphore], values: &[u64], timeout_ns: u64) -> Result<bool>;
}
impl TimelineApi for ash::Device { .. }

impl<A: TimelineApi> Timelines<A> {
    pub fn new(api: A, semaphores: &[vk::Semaphore]) -> Result<Self>;
    pub fn submit<R>(&self, queue: QueueId, f: impl FnOnce(SubmitSignal) -> Result<R>) -> Result<(SyncPoint, R)>;
    pub fn with_queue<R>(&self, queue: QueueId, f: impl FnOnce() -> Result<R>) -> Result<R>;
    pub fn submitted(&self, queue: QueueId) -> Result<u64>;
    pub fn poll(&self) -> Result<CompletedValues>;
    pub fn is_reached(&self, set: &SyncSet) -> Result<bool>;
    pub fn wait(&self, set: &SyncSet, timeout_ns: u64) -> Result<()>;
    pub fn is_device_lost(&self) -> bool;
    pub fn mark_device_lost(&self);
}
```

不变量与协议：

1. 每个逻辑队列一把提交锁（`Mutex<u64>`，值为最后一次成功提交的时间线值）。该锁就是 Vulkan 要求的单 `VkQueue` 外部同步：`vkQueueSubmit`、`vkQueuePresentKHR`、`vkQueueWaitIdle` 都必须经 `submit`/`with_queue` 执行。
2. `submit` 在锁内计算 `value = last + 1`，把 `SubmitSignal { semaphore, value }` 交给闭包；闭包成功返回后才写回 `last` 与无锁镜像 `submitted`。**失败的提交不占用值**，因此任何已知 `SyncPoint` 最终都会完成。
3. `wait`/`is_reached` 先校验每个目标值 ≤ `submitted`，否则返回 `InvalidUsage`（等待未提交的值会永久挂起）。
4. `poll` 只对 `completed_cache < submitted` 的队列调用 `vkGetSemaphoreCounterValue`，结果写回缓存；空闲队列零开销。
5. 任何调用观测到 `DeviceLost` 都置粘滞标志；此后 `submit`/`with_queue`/`poll`/`wait` 立即返回 `DeviceLost`。
6. 不同队列的提交互不阻塞（已有并发单测）。

**`Retirement<T>`**

```rust
impl<T> Retirement<T> {
    pub fn retire(&mut self, after: SyncSet, item: T, completed: &CompletedValues) -> Option<T>; // 已完成则交还调用方立即销毁
    pub fn collect(&mut self, completed: &CompletedValues, ready: &mut Vec<T>) -> usize;        // 到期对象移出，锁外 drop
    pub fn drain_all(&mut self, ready: &mut Vec<T>) -> usize;                                   // idle 后或设备丢失后
}
```

`collect` 为 swap_remove 线性扫描（条目数为每帧退役对象量级，数百以内）。若基准显示瓶颈，按"主依赖队列"分桶并按值排序做前缀弹出。

**锁顺序（全局唯一）**：交换链锁 → 队列提交锁 → 回收锁 → 分配器内部锁。提交锁内只做队列调用，不做阻塞等待、不做 `vkDestroy*`。

### 3.3 队列规划（`src/device/queue_config.rs`，已实现）

```rust
pub enum QueuePreference { Off, Prefer, Require }
pub struct QueueConfig { pub async_compute: QueuePreference, pub transfer: QueuePreference, pub require_present: bool }
pub enum QueueClass { Graphics, Compute, Transfer }
pub struct FamilyInfo { flags, count, present, min_image_transfer_granularity }
pub fn plan_queues(families: &[FamilyInfo], config: &QueueConfig) -> Result<QueuePlan>;
impl QueuePlan { fn queue(class) -> QueueId; fn is_aliased(class) -> bool; fn physical(class) -> PhysicalQueue;
                 fn create_counts() -> Vec<(family, count)>; fn families() -> Vec<u32> }
```

规则：

- **Graphics**（必需）：优先 GRAPHICS|COMPUTE 族；`require_present` 时须支持目标 surface 呈现。
- **Compute**：专用族（COMPUTE 且无 GRAPHICS）→ graphics 族第二队列 → `Prefer`/`Off` 别名 graphics，`Require` 报 `Unsupported`。
- **Transfer**：专用 DMA 族（仅 TRANSFER，图像传输粒度 1×1×1）→ 专用 compute 族空闲队列 → 别名 graphics / `Require` 报错。
- 别名队列共用 `VkQueue` 与时间线（同一个 `QueueId`），语义等同单队列，结果与 `Off` 一致。

### 3.4 设备（`src/device/mod.rs`，P2）

```rust
pub struct DeviceConfig {
    pub queues: QueueConfig,
    pub validation: bool,
    pub required_features: FeatureSet,      // 动态渲染/同步2/时间线/描述符索引等
    pub preferred_device: DevicePreference, // Discrete/Integrated/ByName/ByUuid
    pub frames_in_flight: u32,              // 默认 2，1..=3
    pub recording: RecordingPolicy,         // 见 3.7
}

pub struct Device {
    instance: Arc<Instance>,
    raw: ash::Device,
    physical: PhysicalDeviceInfo,           // properties/limits/features/memory/format 能力缓存
    plan: QueuePlan,
    queues: Vec<vk::Queue>,                 // 下标 = QueueId
    timelines: Timelines<ash::Device>,
    retirement: Mutex<Retirement<Doomed>>,
    allocator: Allocator,                   // gpu-allocator，内部自带锁
    caches: Caches,                         // sampler / descriptor layout / pipeline layout / pipeline / render pass
}

impl Device {
    pub fn new(instance: Arc<Instance>, surface: Option<&Surface>, config: &DeviceConfig) -> Result<Arc<Self>>;
    pub fn queue(&self, class: QueueClass) -> QueueId;
    pub fn submit(&self, queue: QueueId, batches: &[SubmitBatch]) -> Result<SyncPoint>;
    pub fn poll(&self) -> Result<CompletedValues>;
    pub fn wait(&self, set: &SyncSet, timeout_ns: u64) -> Result<()>;
    pub fn wait_idle(&self) -> Result<()>;                // vkDeviceWaitIdle（不经任何队列锁之外的全局锁）
    pub fn retire(&self, after: SyncSet, doomed: Doomed); // 已完成则立即销毁
    pub fn collect_garbage(&self) -> Result<usize>;       // poll + collect，锁外销毁
    pub fn is_device_lost(&self) -> bool;
}
```

- `SubmitBatch { waits: &[Wait], commands: &[vk::CommandBuffer], signals: &[Signal] }`，`Wait = Binary{sem, stage} | Timeline{point, stage}`，`Signal = Binary(sem)`；`Device::submit` 自动追加本队列时间线 signal（`Timelines::submit` 给的值），一次 `vkQueueSubmit2` 提交全部批次。跨队列 `Timeline` 等待在调用前经 `Timelines::submitted` 校验。
- 设备创建：枚举族 → `FamilyInfo::from_properties`（呈现支持经 `vkGetPhysicalDeviceSurfaceSupportKHR`）→ `plan_queues` → `create_counts` 生成 `VkDeviceQueueCreateInfo` → 每个 `QueueId` 创建一条初值 0 的时间线信号量。
- `Doomed` 为枚举（Buffer+Allocation、Image+Allocation、ImageView、Sampler、CommandPool、DescriptorPool、Pipeline、Framebuffer、Semaphore …），`Drop` 中执行对应 `vkDestroy*`/`allocator.free`。
- **Drop 顺序**：`wait_idle`（失败或设备丢失也继续）→ `retirement.drain_all` 销毁 → 清缓存 → 销毁时间线信号量 → 关闭分配器 → `vkDestroyDevice`。
- **删除的旧物**：`raw::device::VulkanDevice`、`GraphicsDevice`、`queue_lock`、`retire_timeline`、`graveyard`、`attach_graphics_device`、`SubmissionEngine`、`DeviceState.next_timeline`。
- 内存：一律持久映射（gpu-allocator `MemoryLocation::CpuToGpu/GpuToCpu` 自带映射指针）；宿主读写不加设备锁，正确性由 `SyncSet` 等待保证（写前等待 GPU 读完、读前等待 GPU 写完）。

### 3.5 资源身份（`src/resource/`，P4）

**raw 层句柄**

```rust
pub struct Handle<T> { key: slotmap::KeyData, _marker: PhantomData<T> }   // 代数键：删除后旧句柄失效
pub struct ResourceName(ustr::Ustr);                                     // 语义名，驱动调试名与跨帧查找

pub struct Buffer  { raw: vk::Buffer, allocation: Allocation, desc: BufferDesc, name: ResourceName, last_use: SyncCell }
pub struct Texture { raw: vk::Image, allocation: Allocation, desc: TextureDesc, name: ResourceName, last_use: SyncCell,
                     views: Mutex<SmallMap<ViewDesc, vk::ImageView>> }
```

- 资源以 `Arc<Buffer>`/`Arc<Texture>` 共享；`Handle<T>` 是存储在 `ResourceStore<T>`（slotmap）里的轻量 `Copy` 键，供图与外部系统长期引用。过期句柄访问返回 `InvalidUsage("stale handle")`。
- `SyncCell` 记录资源的 `SyncSet`：一次提交成功得到 `SyncPoint` 后，对本次提交引用的所有资源 `insert(point)`。实现先用 `Mutex<SyncSet>`；若基准显示争用，换成 `[AtomicU64; MAX_QUEUES]` 的 `fetch_max`。
- 最后一个 `Arc` drop 时，把 `Doomed::{Buffer,Image}`（含视图）连同 `last_use` 交给 `Device::retire`。
- 每个 Vulkan 对象创建后自动以 `ResourceName` 设置 `VK_EXT_debug_utils` 名称（无扩展时跳过）。
- 视图按 `ViewDesc` 缓存在纹理内部，随纹理一起退役（取代原 TextureView 无销毁路径的问题）。

**图层句柄**

```rust
pub struct TextureId(u32);  pub struct BufferId(u32);   // 图内资源，类型区分
pub enum Origin { Create(desc), Import(Arc<Texture|Buffer>, Access), Persistent(ResourceName), Temporal(ResourceName) }
```

- `Persistent(name)`：跨帧存活的命名资源，首帧创建，此后复用；描述符变化时自动重建。
- `Temporal(name)`：按名字的乒乓对。框架维护 `parity`、首帧 / 尺寸变化 / 显式 `reset(name)` 时标记 `history_valid = false` 并以 `UNDEFINED` 起始布局，pass 通过 `ctx.history_valid(id)` 获知。取代 `get_or_create_temporal` + u64 token。
- 资源池按完整描述符（含 usage）精确匹配复用，帧完成后归还；失败路径归还由 RAII 守卫保证。

### 3.6 渲染图（`src/graph/`，P4–P6）

**声明**（沿用 `rg/graph.rs` 的声明与访问账本、生命周期计算，改造接口）

```rust
let mut graph = RenderGraph::new();
let color = graph.create_texture("main.color", TextureDesc { .. });
let history = graph.temporal_texture("taa.history", desc);
graph.add_pass("taa", QueueClass::Graphics)?            // add_pass 返回 Result
     .read(history.previous(), Access::FragmentSampled)
     .write(history.current(), Access::ColorAttachment)
     .render(|ctx| { .. });
```

**编译**

1. 校验（`validate_declaration`）：资源/pass/访问数上限、句柄越界、导出类型一致；失败返回 `InvalidUsage`，不 panic。
2. 生命周期与池分配；同帧内别名（非重叠生命周期共享物理资源）作为可选优化。
3. 屏障规划：每个 pass 前一次 `vkCmdPipelineBarrier2`（`VkDependencyInfo` 批量），由"前一访问 → 当前访问"导出 stage/access/layout；整个 crate 只保留**一份**屏障计算。
4. 队列调度（P6）：pass 的 `QueueClass` 经 `QueuePlan` 映射到 `QueueId`；按队列切分批次；跨队列依赖插入 `Wait::Timeline{point}`（由上游批次提交返回的 `SyncPoint`）；资源跨队列族时按 `SharingPolicy`：
   - `Exclusive`：上游批次末尾 release 屏障（`srcQueueFamily`→`dstQueueFamily`），下游批次开头 acquire 屏障；
   - `Concurrent`：创建时用 `VK_SHARING_MODE_CONCURRENT` 与 `QueuePlan::families()`，不插 ownership 屏障。
   别名队列（同一 `QueueId`）退化为单队列，无额外同步。
5. 并行录制分块：`RecordingPolicy { Serial, PerBatch, Chunked { min_pass_ns, max_chunks } }`，默认 `PerBatch`（一批一个命令缓冲）；PGO 分块保留为可选策略。P0 基线显示 6 pass 单批每帧 3 个命令缓冲，默认改为 `PerBatch` 后应为 1。

**执行**

- `Executor::execute(compiled, frame: &mut FrameContext) -> Result<ExecutionReceipt>`；`ExecutionReceipt { submissions: SmallVec<SyncPoint>, stats }` 直接返回，**不再有事件日志**；诊断用 `#[cfg(feature = "trace")]` 的定长环形缓冲。
- 回调失败或录制失败：本帧已开始的命令池整池复位、池资源归还、借用的导入释放，会话保持可用；只有 `DeviceLost` 使会话不可用。
- 提交成功后对本帧引用的所有资源登记 `SyncPoint`。

### 3.7 命令录制（`src/command/`，P3/P7）

- `FrameRing { slots: [FrameSlot; frames_in_flight] }`，`FrameSlot { pools: per (thread × queue family) VkCommandPool, done: SyncSet }`。
- `begin_frame` 选下一个槽：等待 `slot.done`（`Device::wait`）→ `vkResetCommandPool` 整池复位 → 复用其中已分配的命令缓冲。取代逐个命令缓冲回收、`pool_sequences`、销毁桶。
- 描述符：每槽一个 `VkDescriptorPool`，帧开始整池 reset；持久描述符集（纹理数组等）单独长期池。取代每次 dispatch 分配瞬时集。
- 动态常量/上传：每槽一块持久映射的线性分配环（bump allocator），帧完成后整块复用。

### 3.8 交换链与帧循环（`src/swapchain.rs`、`src/frame.rs`，P5）

```rust
pub struct Swapchain {
    device: Arc<Device>, surface: Arc<Surface>,
    state: Mutex<SwapchainState>,   // 自带锁；acquire 只持这一把
}
struct SwapchainState {
    raw: vk::SwapchainKHR, images: Vec<Arc<Texture>>, extent, format, present_mode,
    acquire: Vec<vk::Semaphore>,    // 按帧环槽
    present: Vec<vk::Semaphore>,    // 按图像槽（规避复用仍待 present 消费的信号量）
    stale: bool,
}

pub enum FrameOutcome { Presented(SyncPoint), Skipped, Recreated }

impl FrameLoop {
    pub fn run_frame(&mut self, build: impl FnOnce(&mut RenderGraph, SwapchainTarget) -> Result<()>) -> Result<FrameOutcome>;
}
```

一帧的固定顺序：

1. `begin_frame`：取帧环槽（等待其 `done`）、`Device::collect_garbage`、temporal 翻转。
2. `acquire`（只持交换链锁）：零尺寸 → `Skipped`；`OUT_OF_DATE` → 重建后 `Recreated`（本帧不渲染）；`SUBOPTIMAL` → 正常渲染并标记 `stale`。
3. 构图：主图与展示图合并为一张图，交换链图像作为 `Import`（起始 `UNDEFINED`，结束 `PRESENT_SRC_KHR`）。
4. 编译、录制。
5. 提交：`Device::submit(graphics, ..)`，首个触碰交换链图像的批次等待 `acquire[slot]` @ `COLOR_ATTACHMENT_OUTPUT`，最后批次 signal `present[image]` 与本队列时间线。
6. present：`Timelines::with_queue(graphics, vkQueuePresentKHR)`；`OUT_OF_DATE`/`SUBOPTIMAL` → 标记 `stale`，下一帧重建。
7. 记录 `slot.done = point`，返回 `Presented(point)`。

失败处理：步骤 3–5 失败 → 回滚本帧（池复位、资源归还），acquire 已取得的图像通过一次空提交（等待 acquire、signal present）+ present 归还，避免图像泄漏；交换链重建前 `Device::wait(set of all in-flight)` 而非 `vkDeviceWaitIdle`。

### 3.9 性能目标

| 指标 | P0 基线（release，6 pass） | 目标 |
| --- | --- | --- |
| 每帧 CPU（begin+构图+execute） | mean 0.30 ms / p95 0.41 ms | 不升高，目标 ≤ 0.2 ms |
| 每帧命令缓冲 | 3.03 | 1（`PerBatch`） |
| 每帧 vkQueueSubmit | 1 | 1（去掉 graveyard 空提交后 SubmitInfo 数 −1） |
| 屏障命令 / 转换 | 5 / 11 | 每 pass ≤ 1 次 `vkCmdPipelineBarrier2` |
| 每帧堆分配 | 未测 | 稳态 0（事件日志删除、SmallVec、帧环复用） |
| 长时间运行 | events 无限增长 | 所有计数平稳（`p0_stability` 两个用例全绿） |

---

## 4. 测试策略

- **单元**：纯逻辑模块（sync、queue_config、图编译、屏障规划、调度）用 CPU 假后端，覆盖多队列布局与失败注入。
- **lavapipe 集成**：`VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`。像素读回、temporal 乒乓、提交失败注入、1000 帧稳定性。
- **整帧**（P5）：`VK_EXT_headless_surface` 跑 acquire → 主图 → 展示图 → present 全链路。
- **多队列**：lavapipe 只有 1 族 1 队列，用别名队列验证逻辑；真实多族路径的测试保留但无独立队列时跳过，需在有独立 compute 队列的显卡上补测。
- **性能**：`cargo test --release -p rhapsody --test p0_perf_baseline -- --ignored --nocapture`，每阶段对比 3.9 表。

## 5. 迁移与删除清单

| 阶段 | 新增 | 删除 |
| --- | --- | --- |
| P2 | `device::{Device, DeviceConfig, init, memory}` | `raw::device::{VulkanDevice, GraphicsDevice}`、`queue_lock`、`raw::init` 中的原版排序/白名单 |
| P3 | `command::FrameRing`、`Device::retire/collect_garbage`、`Doomed` | `raw::graveyard`、`raw::managed::lifecycle::SequenceRuntime`、`raw::submit::SubmissionEngine/DeviceState`、`raw::disposal`、`CommandBufferSeqTracker`、`ManagedResources` 退役记录、`events` |
| P4 | `resource::*`、图层 `TextureId/BufferId/Persistent/Temporal` | `raw::managed::*` 句柄与 checksum、`rg::temporal` token 方案、`VulkanGraphBackend` 的 u64 resolver |
| P5 | `swapchain`、`frame::FrameLoop` | `raw::swapchain` 租约表、`rg::frame` 原版阶段检查器、`rg::state` |
| P6 | 图内 `QueueClass` 调度、`SharingPolicy` | 跨队列族"拒绝提交"分支 |
| P7 | 性能三项 | `rg/compiled_graph.rs`、多余的屏障计算、`rg/resource_usage`、原版地址注释、`pass_entries.rs` |

下游 `tiny-glade`（`render_host.rs`、`raster_host.rs`、测试）与 `tg-text`（`gpu.rs`）在 P2、P4、P5 同步迁移。

## 6. 已知风险

| 风险 | 缓解 |
| --- | --- |
| 改动规模大（约 39k 行、50 个测试文件） | 新架构以新模块落地，接线完成再删旧模块；每阶段全绿后提交 |
| 真实多队列族本机无法验证 | 别名队列验证逻辑；真机测试路径默认跳过并在文档注明 |
| headless surface 与真实窗口系统差异 | 整帧测试覆盖协议；窗口相关在手动测试验证 |
| 去 panic 后误用变成被吞掉的 Err | `invalid_usage` 统一记 error 日志 |
| 仓库 AGENTS.md 的"还原原版"规则与 provenance 工具 | 用户已明确授权 rhapsody 脱离；P7 调整 `verify_inventory.py` 对 rhapsody 的规则 |
