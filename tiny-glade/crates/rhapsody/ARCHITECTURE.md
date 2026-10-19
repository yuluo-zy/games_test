# rhapsody 架构与详细设计

本文只定义 rhapsody 重构的目标架构、接口职责与正确性不变量。开发任务见 [REFACTOR_PLAN.md](REFACTOR_PLAN.md)，实际实现与验证状态只登记在 [REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md)。

逐版交付范围、M0–M9 里程碑、窗口 demo 与验收门槛见 [DELIVERY_PLAN.md](DELIVERY_PLAN.md)。

**唯一标准：正确性与高性能。** 原版 Tiny Glade 的地址、内存布局、panic 行为与错误文案不再约束设计；原版 oracle 测试清理记录见进度文档；对外 API 允许破坏性变更（下游 `tiny-glade`、`tg-text` 及实际调用 rhapsody 的 `country-core` 宿主接口，随改随迁）。

2026-10-16 设计升级：补入帧提交事务、呈现完成域、图计划与帧实例分离、子图缓存、瞬态内存着色、分层分配、预算与有界后台任务。本文的 P0–P7 是技术领域索引，不是开发顺序或验收状态；实施与交付统一按 `DELIVERY_PLAN.md` 的 V0–V9。V2 提前闭合帧事务与呈现，V4 完成帧局部池，V5–V8 依次交付缓存、着色、管线与多队列。

本文代码块是目标接口草图，不保证与迁移中的公开签名一致。类型存在、能力已启用或局部单测通过，均不代表对应交付版本已验收。完整范围以 `DELIVERY_PLAN.md` §1 为准；GPU 驱动渲染、mesh shader、程序化建模迁到 compute、descriptor buffer、GPL、shader object 作为后续可选实验。

---

## 1. 重构前问题（历史动机，不是当前缺陷清单）

下表描述重构起点。已修复与仍开放的部分见 `REFACTOR_PROGRESS.md`，不据此重复恢复已删除的旧机制。

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
├─ error.rs            统一错误 Error/ErrorKind/Result                       P1
├─ sync/               GPU 完成的唯一真相源                                  P2/P3
│  ├─ mod.rs           QueueId、SyncPoint、SyncSet、CompletedValues
│  ├─ timeline.rs      Timelines：每队列提交锁 + 时间线
│  └─ retire.rs        Retirement<T>：按 SyncSet 延迟回收
├─ device/             设备与队列
│  ├─ queue_config.rs  QueueConfig、plan_queues                             P2
│  ├─ logical.rs       Device（唯一设备对象，持有 caches/lifetime/memory/queues/core） P2
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

### 2.1 公开 API 面

`lib.rs` 只公开以下模块，宿主所需类型都在模块根重导出，子模块一律是实现细节：

| 模块 | 内容 |
|------|------|
| `device` | `Device`、`DeviceConfig`、能力/特性、队列规划、提交、预算与统计 |
| `resource` | `Buffer`/`Texture`/`TextureView`/`Sampler`、描述结构、托管句柄、setup 上传、格式查询、`GpuUse`/`AccessScope` |
| `command` | 命令池/命令缓冲、`SynchronizationScope` 屏障、资源访问描述、CPU arena |
| `pipeline` | 着色器模块、管线布局/缓存、异步编译与持久缓存 |
| `bindless` / `transfer` / `sync` | bindless 堆、异步上传、`QueueId`/`SyncPoint`/`SyncSet`/`Timelines` |
| `rg` | 渲染图声明、规划、执行、会话与呈现目标 |
| `present` | `Surface`、`Swapchain` 与呈现租约 |
| `interop` | 显式逃生口：`VulkanInstance`、实例构建、调试工具、物理设备包装、`DeviceQueue` |

- `raw`、`log_ring` 以及上述模块的子模块经 `internal_mod!` 声明：默认 `pub(crate)`；`test-support` 特性下以 `#[doc(hidden)] pub` 暴露，只供本 crate 的集成测试（经自身 dev-dependency 开启），不属于 API。
- 拥有原生对象的类型不公开句柄字段，经 `raw_handle()`（以及 `Device::raw_device()`、`VulkanInstance::raw_instance()`、`Swapchain::loader()` 等）只读取得；这些访问器即互操作逃生口，取得的句柄仍归原对象所有。描述/配置结构（`TextureDesc`、`RasterPipelineDesc` 等）里的 Vulkan 枚举与标志字段属于输入数据，保持公开。
- `shader_pipeline`/`shader_compiler` 不再整包重导出；下游直接依赖 `shader-pipeline`。

---

## 3. 详细设计

### 3.1 错误模型（`src/error.rs`，P1）

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
6. 帧失败取消未提交部分；已提交的 GPU 副作用不可回滚，按实际完成点收尾（§3.10）。仅 `DeviceLost` 使设备/会话永久不可用（粘滞）。
7. 窗口最小化、交换链过期不是错误路径上的"误用"：由 `FrameOutcome::Skipped/Recreated` 表达，不记 error 日志。

### 3.2 同步与完成（`src/sync/`，P2/P3）

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
2. `submit` 在锁内计算 `value = last + 1`，把 `SubmitSignal { semaphore, value }` 交给闭包；闭包成功返回后才写回 `last` 与无锁镜像 `submitted`。**失败的提交不占用值**，不发布等待目标；成功提交也不保证在设备丢失或外部依赖停滞时最终完成，等待仍须处理超时与设备丢失。
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

### 3.2.1 范围同步与状态跟踪（模块分工）

一条链路，各层只做一件事：

| 层 | 模块 | 职责 |
|---|---|---|
| 访问词汇 | `resource::range_state::AccessScope` + `vk::ImageLayout`；`raw::access_table::AccessSpec` | 唯一的访问描述。图的 u8 访问码只是 `ACCESS_TABLE` 的下标，规划时立即降为 `AccessScope`/布局（`rg::plan::AccessSemantics.scope`）；`declared_access` 只用于推导资源 usage（`raw::resource_usage`）。 |
| 范围状态机 | `resource::range_state`（`RangeStateTable`、`StatePatch`、`RangeOwner::admits`） | 唯一的运行期逐范围状态：写者/读者前沿、布局、所有权（Unclaimed/Owned/Released）、修订号。补丁在提交闸门内 `prepare`，仅在成功提交后以真实 `SyncPoint` `publish`；宿主映射走 `prepare_host`。所有权准入只有 `RangeOwner::admits` 一处规则。 |
| 资源挂载 | `resource::tracked::SyncCell` | 持有资源的状态表与整体最后使用点；`validate_queue_range` 为传输服务的预检。 |
| 屏障构建/下发 | `raw::barrier`（`RangeBarrier`、`SynchronizationScope`、`BarrierDispatch`、`Device::issue_barriers`） | 唯一把范围转换变成 Vulkan 屏障并按 sync2/legacy 下发的地方。rg、传输、挂起 acquire、raw 资源访问、交换链、诊断都经此。 |
| 规划期（图） | `rg::access_ledger`（声明收集、同步组）→ `rg::sync_plan`（静态 `RangeProgram`）→ `rg::sync_plan::bind`（每帧把程序叠加到物理 `SyncCell` 表上，产出 `BoundTransition` 与 `StatePatch`） | 规划器用符号依赖（`DependencyRef`）模拟所有权与前沿，提交时产出的补丁仍由运行期状态机校验并发布；Released 范围由规划器自己 acquire。 |
| 运行期（非图） | `raw::resource_access`（显式 raw 访问）、`transfer`、`device::pending_acquire` | 从同一状态表推导屏障并记录同一种 `StatePatch`。跨族上传的 acquire 挂在设备队列上，由下一图批次、传输管理器的图形族命令或 `TransferManager::flush_acquires` 合并录制；未 acquire 的范围被任何使用拒绝（明确错误，不会竞态）。 |
| 校验器 | `rg::plan_check`（`range_program`、`bound`、`native_state`、`range_oracle`） | 独立验证器：刻意不调用生产的拆分/状态转换实现，用自己的稠密格/切片重新推导，再与规划结果比较；共享 `AccessScope`/`RangeTransfer`/`NativeRange` 等词汇类型。仅在开启计划校验时运行（debug 构建、`validation` feature 或 `rg::set_plan_validation`）。 |

不再存在的平行实现：vk-sync 式按访问码推导屏障（`access_table::get_*_barrier`）、`rg::vulkan::AccessSpec` 的散字段/`original_mask`、传输本地的 acquire 待重试列表与逐作业 acquire 提交、各处手写的 `vk::*MemoryBarrier` 与所有权判断。

### 3.3 队列规划（`src/device/queue_config.rs`，P2）

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

### 3.4 设备（`src/device/`，P2）

```rust
pub struct DeviceConfig {
    pub queues: QueueConfig,
    pub validation: bool,
    pub required_features: FeatureSet,      // 动态渲染/同步2/时间线/描述符索引等
    pub preferred_device: DevicePreference, // Discrete/Integrated/ByName/ByUuid
    pub frames_in_flight: u32,              // 默认 2，1..=3
    pub recording: RecordingPolicy,         // 见 3.7
}

// src/device/logical.rs —— 只持有五个组件，方法按组件分布在各自文件中
pub struct Device {
    self_ref: Weak<Device>,
    caches: Caches,     // device/caches.rs：托管资源表、内置采样器、停放的 setup 命令池（只持原生池，不持 Arc<Device>）
    lifetime: Lifetime, // device/lifetime.rs：退役队列 Retirement<Doomed>、回收预算、idle 计数
    memory: Memory,     // device/memory.rs：AllocatorStats、可选子分配器、驱动预算、驱逐登记
    queues: Queues,     // device/queues.rs + submission.rs + pending_acquire.rs：
                        //   主队列/附加队列、QueuePlan、Timelines、挂起 acquire、延迟宿主 flush
    core: DeviceCore,   // device/native.rs：ash::Device、实例、物理设备属性、实际启用能力、策略
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
- **组件与锁**：组件都是 `Device` 的直接字段（无额外 `Arc`/`dyn`），热路径访问即字段访问。各组件只持有自己的锁；跨组件嵌套仅限以下几种（外 → 内）：`Caches` 的采样器表 / setup 池表 → `Memory` 计量与托管资源表锁；`Queues` 挂起 acquire 表 → 资源同步单元；`Queues` 每队列外部同步锁（`with_all_queues` 按队列下标升序）→ 仅原生队列调用。退役队列与驱逐登记先出队/取快照再在无锁状态下执行回调，因此销毁路径可自由取 `Memory` 的锁。
- **Drop 顺序**：`Device::drop` 只做需要多个组件协作的一步：`vkDeviceWaitIdle`（失败或设备丢失也继续）→ 排空退役队列销毁全部 `Doomed` → 释放内置采样器票据。随后字段按声明顺序析构：`caches` → `lifetime` → `memory`（`Drop` 关闭子分配器、释放原生内存块）→ `queues`（`Drop` 销毁时间线信号量）→ `core`（`Drop` 执行 `vkDestroyDevice`，最后释放实例）。创建失败时同样依赖这些组件 `Drop` 回滚已建对象。
- **对外接口**：组件字段为 `pub(crate)`；外部经 `Device` 的委托方法访问（`caps`/`limits`/`memory_properties`/`allocator_stats`/`timelines`/`queue_plan`/`resources` 等）。原生句柄只经显式互操作出口 `raw_device()`、`instance()`、`primary_queue()`、`physical_device()` 取得。
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

**Bindless 槽位（P4 一并纳入，见 §4.2）**

- 设备持有全局描述符堆：一个 update-after-bind + partially-bound 的大描述符集（采样图像、存储图像、存储缓冲、采样器各一个数组），或在支持时用 `VK_EXT_descriptor_buffer` 直接写入 GPU 缓冲。
- 每个可被 shader 访问的资源（及其常用视图）在创建时分配一个 `BindlessIndex(u32)`，由 `slotmap` 式空闲表管理；`Handle<T>` 可直接取得该索引，shader 按索引访问。
- 缓冲优先经 `buffer_device_address` 以 64 位地址访问，绝大多数缓冲不需要描述符。
- 资源退役时槽位随 `Doomed` 一起进 `Retirement`，GPU 完成后才归还空闲表，避免在飞的命令读到被复用的槽位。
- 回退：设备不支持 descriptor indexing 时整套 bindless 不可用，直接报 `Unsupported`（目标硬件与 lavapipe 均支持；descriptor buffer 为可选加速路径，需按设备探测）。

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
2. 纯编译只计算符号化生命周期；实际 requirements 查询、池分配与物理绑定由后续放置/帧实例阶段负责。同帧内别名是受依赖与内存兼容性约束的可选优化，不把 Vulkan 分配塞回可缓存的编译计划。
3. 屏障规划：每个 pass 前一次 `vkCmdPipelineBarrier2`（`VkDependencyInfo` 批量），由"前一访问 → 当前访问"导出 stage/access/layout；整个 crate 只保留**一份**屏障计算。
4. 队列调度（P6）：pass 的 `QueueClass` 经 `QueuePlan` 映射到 `QueueId`；按队列切分批次；跨队列依赖插入 `Wait::Timeline{point}`（由上游批次提交返回的 `SyncPoint`）；资源跨队列族时按 `SharingPolicy`：
   - `Exclusive`：上游批次末尾 release 屏障（`srcQueueFamily`→`dstQueueFamily`），下游批次开头 acquire 屏障；
   - `Concurrent`：创建时用 `VK_SHARING_MODE_CONCURRENT` 与 `QueuePlan::families()`，不插 ownership 屏障。
   别名队列（同一 `QueueId`）退化为单队列，无额外同步。
5. 编译结果缓存（见 §4.4）：拆成 `GraphDefinition → CompiledPlan → FrameInstance`。只缓存不持有具体 Vulkan 资源的不可变计划；导入对象、真实池命中、回调与提交回执属于帧实例。哈希只负责查找，完整规范化键比较确认命中。规划以声明、显式能力/队列配置为输入，不查询可变设备状态；实际内存需求查询与物理绑定另行执行。
6. 光栅 pass 使用动态渲染（`VK_KHR_dynamic_rendering`，Vulkan 1.3 核心）：不创建 `VkRenderPass`/`VkFramebuffer`，`vkCmdBeginRendering` 直接给出附件；`raw::render_pass`、`raw::framebuffer` 及其缓存随之删除，管线以 `VkPipelineRenderingCreateInfo` 声明附件格式。移动端（Tiler 画像）需要子 pass 合并，见 §8.2：届时用 `VK_KHR_dynamic_rendering_local_read`，否则由编译器生成 `VkRenderPass`。
7. 并行录制分块：`RecordingPolicy { Serial, PerBatch, Chunked { min_pass_ns, max_chunks } }`，默认 `PerBatch`（一批一个命令缓冲）；PGO 分块保留为可选策略。P0 基线显示 6 pass 单批每帧 3 个命令缓冲，默认改为 `PerBatch` 后应为 1。

**执行**

- `Executor::execute(compiled, frame: &mut FrameContext) -> Result<ExecutionReceipt>`；`ExecutionReceipt { submissions: SmallVec<SyncPoint>, stats }` 直接返回，**不再有事件日志**；诊断用 `#[cfg(feature = "trace")]` 的定长环形缓冲。
- 回调失败或录制失败：未提交资源直接释放；已提交命令及其资源按 §3.10 退役。同一池内只要仍有在飞命令就不能整池复位，会话保留可继续执行的状态；只有 `DeviceLost` 使会话不可用。
- 提交成功后对本帧引用的所有资源登记 `SyncPoint`。

### 3.7 命令录制（`src/command/`，P3/P7）

- 图后端按 `FramePoolKey { frame_slot, worker, family }` 管理真实 `FrameCommandPool`，动态支持配置的 1–3 个帧槽；命令池保留相关提交的完成集合，不再另设未接线的 CPU FrameRing 模型。
- `begin_frame` 选下一个槽：等待 `slot.done`（`Device::wait`）→ `vkResetCommandPool` 整池复位 → 复用其中已分配的命令缓冲。取代逐个命令缓冲回收、`pool_sequences`、销毁桶。
- 描述符：bindless 落地后（§3.5），pass 只通过 push constant 传参数结构索引或设备地址，不再分配描述符集；过渡期每槽一个 `VkDescriptorPool`、帧开始整池 reset，取代每次 dispatch 分配瞬时集。
- 动态常量/上传：每槽一块持久映射的线性分配环（bump allocator），帧完成后整块复用。
- 每个录制 worker 独占其池与临时 arena；任务窃取时按实际执行 worker 获取池，不能由并发任务共享同一池。并行录制期间不访问共享空闲链，低频批量借还内存块。局部缓存设字节上限、空闲回流与设备代际，避免线程数放大常驻占用。框架 CPU 临时 arena 在 CPU 使用结束后可重置；GPU 可见上传区必须等其完成点。

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
5. 提交：`Device::submit(graphics, ..)`，首个触碰交换链图像的批次等待 `acquire[slot]`，等待阶段由首次访问推导（光栅颜色附件通常为 `COLOR_ATTACHMENT_OUTPUT`，拷贝/compute 使用对应阶段），最后批次 signal `present[image]` 与本队列时间线。
6. present：`Timelines::with_queue(graphics, vkQueuePresentKHR)`；`OUT_OF_DATE`/`SUBOPTIMAL` → 标记 `stale`，下一帧重建。
7. 记录 `slot.done = point`，返回 `Presented(point)`。

失败处理由 `AcquiredImageLease` 记录 acquire 信号是否已消费、图像布局、提交完成点与呈现状态。未提交部分取消；已提交部分依 §3.10 收尾。图像按 `PresentPolicy` 返回（`FrameLoop::set_on_failure` / `VulkanGraphSession::set_present_policy`）：默认 `SkipPresent` 不呈现未定义内容——release-images 可用（`Swapchain::can_release_images`：启用 maintenance1 且不是 Mesa 软件 WSI，其 `vkReleaseSwapchainImagesEXT` 会崩溃）时等待已提交使用完成后直接释放；不可用时以真实提交消费 acquire 并呈现不透明黑帧。`ClearTo(rgba)` 呈现指定颜色（需要 `TRANSFER_DST` 用法，否则退化为 `SkipPresent`）。`PreviousFrame` 需要保留上一帧副本，交换链不保存，设置时返回 `Unsupported`。补救提交必须等待尚未消费的 acquire（或链接到已提交使用之后），不能无条件用空提交，也不能重复等待已消费的二值信号量。释放图像不等于消费 acquire 信号，二值信号量的复用/销毁单独跟踪。设备丢失时停止补救提交，转入设备销毁路径。

**获取等待与批次**：获取等待附在批次的提交上，所以呈现图通过 `RenderGraph::gate_first_use` 在交换链图像首个使用者前强制批次边界（并入计划键，`import_swapchain_image` 自动登记）：之前的独立 pass 在前一批次提交，不被获取信号量阻塞；不需要 `batch_per_pass`。

**所有权**：内容已丢弃（UNDEFINED 布局）的图像范围无主——规划器、运行期状态机（`RangeOwner::admits_discarding`）、raw 访问与两套独立校验器都允许任一队列族直接认领并在其队列上做 UNDEFINED -> layout 转换，不发出 release/acquire 对；写入过的内容仍按原规则转移，回到呈现族的最终转移保持不变。

**呈现完成域**：`SyncPoint` 证明 GPU 提交完成，`PresentTicket` 证明相应呈现资源可以回收，二者不混用。支持并启用 swapchain maintenance1 时用呈现 fence；否则按再次 acquire 同一图像并等待其信号、呈现历史及交换链代际实现延迟销毁协议。旧交换链停止 acquire 后，不能依赖它未来再次 acquire，应采用官方交换链重建示例的代际关联方案。重建同时满足在飞提交与呈现资源的回收条件；只等待设备时间线不足以证明呈现资源已释放。

参考：[呈现信号量复用](https://docs.vulkan.org/guide/latest/swapchain_semaphore_reuse.html)、[交换链重建示例](https://docs.vulkan.org/samples/latest/samples/api/swapchain_recreation/README.html)、[呈现布局约束](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentInfoKHR.html)。

### 3.9 性能目标

| 指标 | P0 基线（release，6 pass） | 目标 |
| --- | --- | --- |
| 每帧 CPU（begin+构图+execute） | mean 0.30 ms / p95 0.41 ms | 不升高，目标 ≤ 0.2 ms |
| 每帧命令缓冲 | 3.03 | 1（`PerBatch`） |
| 每帧 vkQueueSubmit | 1 | 1（去掉 graveyard 空提交后 SubmitInfo 数 −1） |
| 屏障命令 / 转换 | 5 / 11 | 每 pass ≤ 1 次 `vkCmdPipelineBarrier2` |
| 每帧堆分配 | 未测 | 稳态 0（事件日志删除、SmallVec、帧环复用） |
| 长时间运行 | events 无限增长 | 所有计数平稳（`p0_stability` 两个用例全绿） |

### 3.10 帧事务与部分提交

`FrameTransaction` 状态为 `Preparing → Recorded → PartiallySubmitted → Submitted → Completed`；执行器允许批次交错录制/提交，但每次提交都以成功返回为不可逆边界。

- 提交前预备资源引用、回执存储与退役记录容量，成功后登记完成点的步骤不得再依赖可失败的分配或用户回调。
- 每个成功批次立即登记 `SyncPoint`、资源使用、实际访问/所有权终态；后续失败不能把状态恢复成图开始前的值。未提交工作释放，已提交工作移交持有资源的退役记录，正常失败路径不为清理而无限等待 GPU。
- 同一命令池、上传区、描述符池的完成集合覆盖其中全部已提交工作，只有满足集合才可整体 reset。取消未提交命令不授予整池 reset 权限。
- temporal 的内容版本只有在对应产出已成功提交、且后继使用有完成依赖时才发布；部分写入或缺失最终产出的历史失效。其他 CPU 业务副作用不承诺由框架自动回滚，录制回调应避免此类副作用。
- RAII 守卫保证引用和租约不会丢失；析构不执行无界等待或可失败的业务补救。需要 Vulkan 调用的补救显式执行，失败记录交由帧循环/设备销毁收尾。
- 清理和退役回调执行一次；逐项收集清理失败，保留首个业务错误作为主错误。设备丢失取消后续提交并排空可安全销毁的记录。

### 3.11 异步传输、内存预算与能力策略

**传输服务**：`UploadManager` / `ReadbackManager` 返回带资源区间、设备代际与 `SyncPoint` 的 ticket。上传合并脏区，readback 提供非阻塞 `try_read`；需要同步的便捷 API 只等待相关资源区间，不等待整设备所有提交。区间预留与 GPU 使用登记必须防止宿主写入和并发新提交竞争，不能只读一次完成快照就授予写权限。非 coherent 内存的范围对齐、flush/invalidate 由分配层统一处理。上传环容量有上限，满时由策略选择延后、受限扩容或显式等待；传输队列是可选策略，不是正确性的前提。

**可驱逐缓存**：渲染图空闲资源池（帧完成后“停放”到 `rg::graph_caches`，复用时认领）与空闲瞬态别名集、以及持有资源引用的 CPU 投影快照一起注册为 `Evictable`（`Transient` 优先级）。压力下只释放 GPU 已完成的停放资源（LRU），再释放空闲别名集；借给在飞帧的资源/别名集结构上不在存储内，不可达。登记只持 `Weak`，随后端析构注销。

**统一预算**：`MemoryBudgetController` 记录 live、in-flight、可淘汰 cache、pending-retirement 字节数。各缓存有局部上限，统一预算约束总量；先淘汰已完成的空闲资源，再延后低优先级上传，画质降级交上层决定。高低水位与滞回防止频繁淘汰/重建。常规回收限制每帧工作量，内存压力触发加速回收；不能淘汰在飞资源。`VK_EXT_memory_budget` 的动态估计仅作为策略输入，未支持时使用配置预算，任何情况下仍处理真实 OOM。

**有界后台任务**：管线编译、资源上传按规范化键合并重复请求，队列有容量、优先级及过期请求取消规则。管线热重载结果在帧边界发布；失败保留旧版本，旧对象按实际使用完成点退役。缓存落盘使用版本/设备兼容标识与原子替换，损坏或不兼容时丢弃并重建。

**能力与身份**：`DeviceCaps` 保存硬件支持和设备实际启用特性，`ExecutionPolicy` 在初始化时选择描述符实现、队列、录制和分配策略。句柄携带设备归属/代际，图内 ID 带图作用域校验，拒绝跨设备或跨图误用。设备恢复可后续实施，但旧代际资源不得进入新设备的帧实例。

参考：[非 coherent 映射内存规则](https://docs.vulkan.org/spec/latest/chapters/memory.html)、[动态内存预算](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_memory_budget.html)。

---

## 4. 性能路线图

§3 的重构解决正确性与结构问题，同时以减少命令缓冲、移除额外退役提交、稳态零临时分配为性能目标。是否已实现及实测结果见 `REFACTOR_PROGRESS.md`；稳态零分配仍须独立测量。下列方向的收益是待验证假设，优化默认启用须满足 §4.9 的验收条件。

| # | 方向 | 预期收益 | 代价 | 决策 |
| --- | --- | --- | --- | --- |
| 4.0 | 剖析 | 决定后续全部方向 | 低 | 必须先做，且在真显卡上 |
| 4.1 | 管线编译去卡顿 | 消除首次遇到材质/变体时的帧时间尖刺 | 中 | 最优先 |
| 4.2 | Bindless 描述符 | 描述符分配与绑定的 CPU 开销接近 0 | 中 | 并入 P4（影响句柄设计） |
| 4.3 | GPU 驱动渲染 | draw 提交 CPU 成本从 O(对象数) 降到 O(材质类别数) | 高 | 剖析确认 CPU 提交是瓶颈后 |
| 4.4 | 整图/子图编译缓存、版本驱动结果缓存 | 减少重复编译与未变化内容的重复执行 | 中 | P4 分层与整图缓存；子图/结果缓存按命中率启用 |
| 4.5 | 瞬态内存着色、分层分配 | 降低峰值显存及分配次数；不承诺帧率提升 | 中 | P4 单队列方案，P6 扩展到多队列偏序 |
| 4.6 | 异步 compute 按关键路径调度 | GPU 利用率 | 高 | 真机 GPU 时间戳证明有重叠空间后 |
| 4.7 | 数据导向热路径 | 框架自身 CPU 常数开销 | 中 | 剖析显示框架开销占比可观时 |

### 4.0 剖析

- GPU：每个 pass 前后写时间戳查询（扩展现有 `begin_top_level_timestamp_query`），按帧环槽回读，不阻塞。
- CPU：编译、录制、提交、回收各打 puffin（已在依赖中）或 `tracing` 区段。
- lavapipe 只能衡量 CPU 侧；GPU 数据必须来自真显卡。P0 基线的 0.30 ms/帧是"框架 + 回调 + 驱动"总和，未拆分。

### 4.1 管线编译去卡顿

- **离线枚举 + 磁盘缓存**：shader 包已离线编译，管线变体组合同样离线枚举；启动时加载 `VkPipelineCache` 并在加载阶段后台预热。
- **异步编译 + 回退**：编译只在后台线程进行；未就绪时本帧用简化变体或跳过该 draw，渲染线程绝不同步编译。
- **Graphics Pipeline Library**（`VK_EXT_graphics_pipeline_library`）：顶点输入、shader、片元输出分段预编译，运行时快速链接，化解变体组合爆炸。
- **`VK_EXT_shader_object`**：变体极多时可完全不用管线对象；需先确认目标驱动支持面。

### 4.2 Bindless 描述符

设计见 §3.5。要点：全局描述符堆 + `BindlessIndex`；缓冲走 `buffer_device_address`；pass 参数经 push constant 传索引；槽位随 `Retirement` 延迟归还。它同时是 4.3 的前提。

### 4.3 GPU 驱动渲染

- 场景数据常驻 GPU：实例变换、材质 ID、包围盒存于持久缓冲，编辑时只上传脏区。
- compute 剔除：视锥 + 上一帧深度生成的 HZB 遮挡剔除，结果直接写间接绘制参数。
- `vkCmdDrawIndexedIndirectCount`：一次调用绘制整类物体。
- Meshlet / mesh shader：几何按簇剔除；程序化几何可在生成阶段直接产出 meshlet（lavapipe 不支持 mesh shader，需真机）。
- 评估把建筑/地形网格的程序化生成移到 compute，省去 CPU 生成后的上传。
- 这是渲染方式的根本改变（shader、资产管线都要改）；场景对象不多时不值得做。

### 4.4 渲染图编译缓存

**计划与实例分离**

| 类型 | 内容 | 可缓存性 |
| --- | --- | --- |
| `GraphDefinition` | 资源规格、资源版本、子资源访问、pass 与显式副作用 | 结构不变时复用 |
| `CompiledPlan` | 校验、依赖图、保留的 pass、队列调度、屏障模板、虚拟资源生命周期 | 不可变、可跨帧复用，不持有具体资源或回调 |
| `FrameInstance` | 资源实际绑定、导入初始状态、回调参数、提交回执和资源引用 | 每帧独立，不进编译缓存 |

每次写入在图内形成新资源版本；读依赖明确版本，以最终导出和显式副作用为根反向裁剪无用 pass。访问范围表达 texture 的 aspect/mip/layer 和 buffer 的 offset/size。原地更新保留物理存储反依赖约束，不能因逻辑版本不同就错误地并发执行。

缓存键覆盖拓扑、资源规格与使用标志、访问范围、导出、副作用、队列映射、选择的能力策略和编译器版本。凡被烘焙进计划的导入初始状态、shader 接口等也必须入键；选择作为运行时参数的状态则由绑定阶段重新校验并生成边界屏障。哈希只定位候选，完整规范化键比较才确认命中，不能因 64 位哈希相同复用错误计划。缓存同时受条目数与字节预算约束，记录命中、未命中与失效原因。

**子图编译缓存**

- `SubgraphInterface` 声明输入/输出规格、读写范围、边界状态、队列约束和副作用。稳定的阴影、后处理、文字等子图可独立编译，父图修改时只重编受影响的结构。
- 子图缓存保存局部依赖和符号化计划；全图组合后仍统一完成跨子图同步、队列调度和瞬态内存放置。子图不能私自固化全局内存偏移或时间线值。
- 先实现整图缓存，统计重复子图比例、编译耗时与缓存管理成本后启用细粒度缓存。过细划分会增加键构造与边界协调成本，不要求所有 pass 都成为子图。

**结果缓存（与编译缓存独立）**

- `ResultCacheKey` 包含输入内容版本、参数、相关 shader 实现版本和设备代际。上层发布脏标记，依赖图传播失效；名字或资源地址不能代替内容版本。
- 适用候选：未变化建筑的生成网格、输入稳定的阴影结果、文字排版及顶点数据。时间、随机种子、相机、光源、遮挡物或 temporal 历史只要影响输出，都必须纳入依赖；无法完整声明依赖的 pass 不允许复用结果。
- 命中结果仍需等待其产出完成点，并登记本帧的实际使用。输出成为受预算管理的持久资源，不再属于可任意覆盖的瞬态别名区。
- 业务版本与脏标记由 `tiny-glade` / `tg-text` 提供，rhapsody 负责缓存协议、同步和生命周期，不引入建筑业务知识。

### 4.5 瞬态内存着色与分层分配

**内存着色 / 别名规划**

`TransientMemoryPlan` 为短生命周期资源生成兼容分组、内存块/偏移、别名切换依赖及预计峰值。概念上对可能并存的资源建立冲突图，但大小与对齐不同，实际采用带容量约束的贪心放置/区间分配；不要求求解最优图着色，也不必显式构造稠密冲突矩阵。

1. 拓扑规划输出符号化生命周期；资源实现层查询真实 `VkMemoryRequirements` 和 dedicated 要求。放置器接收这些不可变事实后生成物理方案；设备相关方案只能在相同兼容性签名下复用。
2. 检查 size、alignment、memoryTypeBits 交集、buffer-image granularity、资源创建标志、图像别名规则与独立分配要求。仅依据纹理宽高和估算字节数分组不够。需要独立分配或外部所有权的资源不进入普通别名组。
3. 单队列按包含同步边界的生命周期区间规划。多队列按依赖偏序证明“旧资源的全部访问先于新资源首次访问”；pass 编号不重叠不代表实际执行不重叠。不同在飞帧也不得无依赖地复用同一物理区间。
4. 别名切换生成适用的执行/内存依赖、布局转换及必要的队列同步。`oldLayout = UNDEFINED` 表示丢弃内容，不免除先前访问的同步；新资源首次使用必须建立所需内容，拒绝读取未定义数据。相关规则见 [Vulkan 资源别名](https://docs.vulkan.org/spec/latest/chapters/resources.html) 与 [同步规范](https://docs.vulkan.org/spec/latest/chapters/synchronization.html)。
5. 第一版只为本来已被依赖排序的资源复用内存。默认 `PreserveParallelism` 不新增串行依赖；可选 `MinimizeMemory` 才允许受控添加无环依赖，并报告新增等待和关键路径代价。任何策略都不能通过覆盖仍在用的资源达成预算。
6. temporal、导出持久结果、外部导入及结果缓存输出默认排除；跨帧瞬态堆随帧槽完成点复用。显式支持更复杂生命周期前，不尝试跨这些边界着色。

示例：A 需要 64 MiB，B 需要 32 MiB；只有当 A 全部访问先于 B、内存约束兼容且内容无需保留时，二者才可共享 64 MiB。收益按实际峰值对比，不预设节省比例；省显存不等于省带宽或提高帧率。

**分层分配（与着色不同）**

| 资源类别 | 策略 | 复用条件 |
| --- | --- | --- |
| 持久纹理、网格缓冲 | 复用 `gpu-allocator` 的通用子分配；同时存活对象使用不同区间 | 对象退役完成 |
| 动态常量、上传与读回 | 持久映射 ring/bump，按使用方向和对齐要求分区 | 对应区间的 GPU 使用完成 |
| 图内瞬态附件/缓冲 | `TransientMemoryPlan` 约束的别名堆 | 图内依赖及帧槽完成条件满足 |
| 要求独立分配的资源 | dedicated allocation；其他特大资源按测量与碎片策略选择 | 对象退役完成 |
| CPU 编译/录制临时数据 | 每 worker / 帧的 arena，预留高水位容量并设上限 | 所有相关 CPU 任务结束 |

不另写通用 GPU 分配器。别名区由单一 backing-allocation owner 拥有，多资源共享该分配并汇总完成点；不能让每个资源重复释放同一分配。若现有分配器接口不支持受控别名，用狭窄的瞬态堆适配层承载，保持预算与释放责任唯一。

记录 committed/live/峰值字节、内部与外部碎片、别名复用字节、分配次数、额外同步和 GPU 时长；预算协调遵循 §3.11。

### 4.6 异步 compute 按关键路径调度

- 编译器估计 GPU 关键路径（初期用上一帧每 pass 时间戳），把不在关键路径上、且与图形工作无资源冲突的 compute pass（SSAO、光照剔除、粒子等）移到异步队列，与阴影/GBuffer 等光栅受限 pass 重叠。
- 每帧跨队列汇合点尽量控制在一两个。
- 别名队列（lavapipe、多数集显）上收益为零，同步不当反而变慢；默认关闭，测量证明有效才开启。移动端固定默认关闭（§8.4）。

### 4.7 数据导向热路径

- 每帧线性分配器（arena）承载编译与录制的临时数据，帧末整体重置。
- 图执行按索引访问资源表，`Arc`/`Mutex` 只出现在资源创建与销毁处。
- pass 与访问记录用结构数组（SoA）存放，编译按列遍历。
- 并行录制基于 `bevy_tasks` 任务窃取，按上一帧 pass 耗时分块。
- 线程局部池按 §3.7 隔离，各池完成后整体 reset；共享缓存采用只读快照或短临界区发布，不在逐 draw 热路径做全局查找/分配。局部缓存命中率、锁等待与总驻留字节一起度量。参考 [Khronos 多线程命令录制示例](https://docs.vulkan.org/samples/latest/samples/performance/command_buffer_usage/README.html)。

### 4.8 其他较小改进

- **动态渲染**：已写入 §3.6 第 6 点，随 P4/P7 落地。
- **上传路径**：按 §3.11 实现异步传输与区间同步；专用 transfer 队列、可映射 DEVICE_LOCAL 内存直写均为可选策略，按实际内存属性与测量选择，不能仅凭 ReBAR 支持就默认直写更快。
- **帧延迟控制**：`VK_KHR_present_id` + `VK_KHR_present_wait` 按显示节奏决定何时开始下一帧，降低输入延迟。

### 4.9 硬件要求与顺序

- 依赖较新特性：descriptor indexing、buffer device address、dynamic rendering、synchronization2、timeline semaphore（Vulkan 1.2/1.3 核心，lavapipe 支持）；descriptor buffer、graphics pipeline library、shader object、mesh shader 为可选，需按设备探测（lavapipe 不支持 mesh shader，其余需实测）。目标硬件的最低要求尚未确定，须在 4.2/4.3 动工前确认。
- 交付顺序：V0 可重复窗口基线 → V1 设备/完成/精确回收 → V2 提交事务与呈现闭环 → V3 图分层与资源契约 → V4 帧局部池/传输/预算 → V5 整图/子图/结果缓存 → V6 单队列瞬态着色 → V7 管线/描述符 → V8 多队列 → V9 全链路验收。P 编号仅用于映射上述技术领域。
- V1 先按真实完成点回收现有命令分配；V2 引入满足事务安全的最小帧槽；V4 再完成 worker/family 池隔离与整池复位优化，不能把回收正确性拖到 V4。V3 建立独立检查器，V6/V8 扩展别名和多队列规则。
- 剖析从 V0 开始贯穿各版；异步管线编译在 V7 正式交付。子图和结果缓存是 V5 必做功能，是否默认启用依据重复率与管理成本决定。bindless、动态渲染在 V7 交付；GPU 驱动、自动关键路径迁移和为省内存新增串行依赖按真机 A/B 决定启用。
- 每项优化保留关闭路径；相同负载下比较 CPU/GPU 的 p50/p95/p99、峰值显存、稳态分配数、等待时长及长跑计数。未证明收益的策略保持可选，不把“实现了”当成“默认更快”。测试记录硬件、驱动、构建配置和负载；无对应硬件时标记未验证，不外推 lavapipe 结果。

---

## 5. 测试策略

- **单元**：纯逻辑模块（sync、queue_config、图编译、屏障规划、调度）用 CPU 假后端，覆盖多队列布局与失败注入。
- **lavapipe 集成**：`VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`。像素读回、temporal 乒乓、提交失败注入、1000 帧稳定性。
- **整帧**（P5）：`VK_EXT_headless_surface` 跑 acquire → 主图 → 展示图 → present 全链路。
- **多队列**：lavapipe 只有 1 族 1 队列，用别名队列验证逻辑；真实多族路径的测试保留但无独立队列时跳过，需在有独立 compute 队列的显卡上补测。
- **性能**：`cargo test --release -p rhapsody --test p0_perf_baseline -- --ignored --nocapture`，每阶段对比 3.9 表；GPU 侧数据按 §4.0 在真显卡上采集。
- **独立计划检查器**：以声明与编译结果为输入，用独立规则检查读取来源、冲突访问的依赖路径、跨族交接、别名区间不并发、回收覆盖全部使用；不照抄编译器算法。随机小图及随机完成顺序与保守无别名/串行执行对照。
- **缓存**：缓存开关输出一致；强制哈希碰撞仍正确；只改变输入内容、子图结构、导入初态、shader 接口/实现或设备代际时，分别验证正确层级失效。结果缓存命中不能跳过完成依赖。
- **内存与局部池**：覆盖跨队列/在飞帧生命周期重叠、对齐/类型不兼容、dedicated 排除、别名首次读取拒绝；使用 Vulkan 验证层检查支持的路径。循环切换分辨率和图形态、填满上传环与后台队列，验证预算上限、背压及退役积压最终消退。
- **失败矩阵**：在 acquire、分配、录制、首批/后续提交、present、清理步骤注入失败；确认已提交部分不复位/不重复释放、未提交部分释放、回调恰好一次、原始错误保留、可恢复错误后的下一帧可执行。交换链代际与二值信号量状态另行验证。
- **可观测性**：记录 pass GPU 时间、CPU 编译/录制/提交/回收时间、缓存失效原因、锁等待、内存高水位和退役年龄；诊断事件有界且不参与控制流。设备丢失或超时输出最近提交及资源版本的关联信息。

## 6. 迁移与删除清单

| 阶段 | 新增 | 删除 |
| --- | --- | --- |
| P2 | `device::{Device, DeviceConfig, init, memory}`、能力策略与设备代际 | `raw::device::{VulkanDevice, GraphicsDevice}`、`queue_lock`、`raw::init` 中的原版排序/白名单 |
| P3 | 图后端帧槽与 worker/family 命令池、CPU arena、`Device::retire/collect_garbage`、`Doomed`、预算计数与帧事务基础 | `raw::graveyard`、`raw::managed::lifecycle::SequenceRuntime`、`raw::submit::SubmissionEngine/DeviceState`、`raw::disposal`、`CommandBufferSeqTracker`、`ManagedResources` 退役记录、`events`、未接线的 CPU FrameRing |
| P4 | `resource::*`、类型化/命名资源、bindless、动态渲染、图计划/帧实例分层、资源版本、子图边界、整图缓存、单队列瞬态着色、传输 ticket | `raw::managed::*` 句柄与 checksum、`rg::temporal` token 方案、`VulkanGraphBackend` 的 u64 resolver、`raw::render_pass`/`raw::framebuffer` 及其缓存、每次 dispatch 的瞬时描述符集 |
| P5 | `swapchain`、`frame::FrameLoop`、帧事务闭合、图像租约与呈现完成域 | `raw::swapchain` 租约表、`rg::frame` 原版阶段检查器、`rg::state` |
| P6 | 图内 `QueueClass` 调度、`SharingPolicy`、偏序驱动别名规划 | 跨队列族"拒绝提交"分支 |
| P7 | 性能 A/B、预算与尾延迟验收、按测量启用子图/结果缓存 | `rg/compiled_graph.rs`、多余的屏障计算、`rg/resource_usage`、原版地址注释、`pass_entries.rs` |

下游 `tiny-glade`（`render_host.rs`、`raster_host.rs`、测试）与 `tg-text`（`gpu.rs`）在 P2、P4、P5 同步迁移。

## 7. 已知风险

| 风险 | 缓解 |
| --- | --- |
| 改动规模大（约 39k 行、50 个测试文件） | 新架构以新模块落地，接线完成再删旧模块；每阶段全绿后提交 |
| 真实多队列族本机无法验证 | 别名队列验证逻辑；真机测试路径默认跳过并在文档注明 |
| headless surface 与真实窗口系统差异 | 整帧测试覆盖协议；窗口相关在手动测试验证 |
| 去 panic 后误用变成被吞掉的 Err | `invalid_usage` 统一记 error 日志 |
| 仓库 AGENTS.md 的"还原原版"规则与 provenance 工具 | 用户已明确授权 rhapsody 脱离；P7 调整 `verify_inventory.py` 对 rhapsody 的规则 |

---

## 8. 移动端平台

移动 GPU（ARM Mali、Qualcomm Adreno、Imagination PowerVR、经 MoltenVK 的 Apple GPU）是**分块延迟渲染（TBDR）**：屏幕切成小块，每块在片上内存里完成着色后才写回系统内存。同时它们与 CPU 共享一条有限的内存带宽，并且受功耗与发热约束。因此移动端的首要矛盾是**带宽与发热**，不是 draw call 数量。§3、§4 中若干桌面优先的决定在移动端需要调整，本节给出差异化设计；与前文冲突处以本节为准（由 `DeviceProfile` 选择）。

### 8.1 设备画像 `DeviceProfile`

```rust
pub enum GpuArchitecture { Immediate, Tiler { vendor: TilerVendor } }
pub enum TilerVendor { Arm, Qualcomm, Imagination, Apple, Other }
pub struct DeviceProfile {
    pub architecture: GpuArchitecture,     // vendorID: ARM 0x13B5、Qualcomm 0x5143、ImgTec 0x1010、Apple 0x106B（MoltenVK）
    pub unified_memory: bool,              // 存在 DEVICE_LOCAL | HOST_VISIBLE 的大堆
    pub lazily_allocated: bool,            // 存在 LAZILY_ALLOCATED 内存类型
    pub tier: FeatureTier,                 // 见 8.6
    pub thermal: Option<ThermalSource>,    // Android 热状态回调 / iOS thermalState
}
```

- 在 `Device::new` 时探测一次，作为 `DeviceConfig` 的默认值来源；所有策略（渲染 pass 合并、队列、bindless、上传路径、分辨率）都从画像派生，调用方也可以显式覆盖。
- 画像只决定**策略**，不分叉 API：同一套渲染图代码在桌面和移动端都能运行，编译器按画像生成不同的执行计划。

### 8.2 渲染 pass 合并与片上数据（最大收益项）

TBDR 上每结束一个渲染 pass，片上的附件就要写回主存，下一个 pass 再读回来，这是移动端最大的带宽浪费。

- **图编译器合并光栅 pass**：相邻、尺寸相同、只做像素本地读写（当前像素读上一 pass 同一位置的输出）的光栅 pass 合并为一个渲染 pass 的多个子 pass；中间附件改为 input attachment，在片上传递，不落主存。典型可合并链：GBuffer → 延迟光照 → 半透明 → 色调映射。
- **与 §3.6 第 6 点的关系**：桌面画像使用动态渲染；Tiler 画像需要子 pass 语义，二选一：
  - 设备支持 `VK_KHR_dynamic_rendering_local_read` 时，仍用动态渲染 + 片上本地读；
  - 否则生成 `VkRenderPass` + 多子 pass。因此 `raw::render_pass` 不随 P4 删除，而是收敛为"由编译器生成、按图结构缓存"的内部实现，P7 再按画像裁剪。
- **load/store 操作由生命周期推导**：资源在本 pass 首次被写且无需旧内容 → `LOAD_OP_CLEAR` 或 `DONT_CARE`；本 pass 之后不再被读 → `STORE_OP_DONT_CARE`。深度、MSAA 颜色、GBuffer 中间层在合并后几乎都不需要写回。禁止在 pass 中途用 `vkCmdClearAttachments` 替代 load op。
- **瞬态附件用 LAZILY_ALLOCATED 内存**：只在片上存在的附件（合并后的中间附件、MSAA 颜色、深度）用 `TRANSIENT_ATTACHMENT` usage + 惰性分配内存，实际不占物理显存。这比 §4.5 的内存别名收益更直接，两者在 Tiler 画像上叠加使用。
- **MSAA 在片上解析**：用子 pass 的 resolve attachment 解析，多采样数据不写回主存。移动端上 4×MSAA 因此代价很低，可以替代 TAA 等全屏后处理抗锯齿。

### 8.3 带宽

- **附件格式**：HDR 颜色优先 `B10G11R11_UFLOAT_PACK32`，而不是 `R16G16B16A16_SFLOAT`；GBuffer 按需打包、尽量窄；深度在不需要采样时设为瞬态。
- **帧缓冲压缩（ARM AFBC、Adreno UBWC）**：驱动只对 `OPTIMAL` tiling 的图像自动压缩，而 `STORAGE` usage 或部分格式会关闭压缩。资源的 usage 由图编译器从访问记录推导（现有 `image_usage_flags` 机制），只给真正需要的 usage，避免误开 `STORAGE`。
- **后处理链合并**：色调映射、颜色分级、暗角、FXAA 等合并成一个全屏片元 pass（在合并后的子 pass 里读 input attachment）；泛光、景深这类必须降采样的效果用半分辨率或四分之一分辨率。
- **优先片元而非 compute 做后处理**：在 Tiler 上，compute 写图像再由片元读取会打断"本帧片元与下一帧顶点重叠执行"的流水，而且无法利用片上内存与帧缓冲压缩。画像为 Tiler 时，可二选一实现的效果（模糊、色调映射）走片元路径。
- **纹理压缩用 ASTC/ETC2**：移动 GPU 基本不支持 BC 格式。资产管线需额外产出 ASTC（首选）与 ETC2（回退）；上传路径按设备格式能力选择；`raw::format_table` 的块格式表需覆盖 ASTC 全部块尺寸。
- **shader 精度**：在 HLSL 中用 `min16float` 等，经 dxc 生成 `RelaxedPrecision`，或启用 `shaderFloat16`，让 Mali/Adreno 用半精度单元运行，寄存器压力与功耗都下降。由 shader 编译器按画像选择变体。

### 8.4 同步与流水

- Tiler 会让第 N+1 帧的顶点/分块阶段与第 N 帧的片元阶段重叠执行。屏障的 stage 掩码必须精确：禁止 `ALL_COMMANDS`/`ALL_GRAPHICS` 这类宽泛掩码，跨帧依赖只阻塞真正需要的阶段（例如用 `FRAGMENT_SHADER → VERTEX_SHADER` 时，要确认确有顶点阶段读取）。屏障规划器需要按画像做静态检查，在 debug 构建下给出警告。
- 渲染 pass 内部不插入外部屏障；需要的依赖用子 pass 依赖（`BY_REGION`）表达。
- **异步 compute**：移动端通常只有一个队列族，或者独立 compute 队列与图形共享同一组着色器核心，重叠收益为零甚至为负。Tiler 画像下 `QueueConfig::async_compute` 的默认值固定为 `Off`（§3.3 规则不变，只是默认值不同）。
- **独立传输队列**：仍可用于后台资源上传；在统一内存设备上，上传路径改为直接写入（见 8.5），不需要传输队列。

### 8.5 内存

- **统一内存**：画像检测到 `DEVICE_LOCAL | HOST_VISIBLE` 堆时，动态缓冲、uniform、实例数据直接持久映射写入，跳过暂存缓冲与拷贝命令；对非 `HOST_COHERENT` 的类型按需 `vkFlushMappedMemoryRanges`。
- **显存预算小**：所有分配走统一的预算跟踪（`VK_EXT_memory_budget`）。超预算时按以下顺序降级：缩小资源池、降低 temporal 历史精度、降低渲染分辨率；不能等到分配失败才处理（移动端系统可能直接终止进程）。
- 瞬态附件优先惰性分配（8.2），其余瞬态资源按 §4.5 做别名。

### 8.6 特性分级

移动端驱动的特性支持差异大，按分级决定启用哪些路径：

| 分级 | 典型设备 | 启用 |
| --- | --- | --- |
| `Baseline` | Vulkan 1.1，较老的 Mali/Adreno | 绑定式描述符（每帧池 reset）、CPU 剔除 + 实例化、子 pass 合并、ASTC/ETC2 |
| `Modern` | Vulkan 1.3，descriptor indexing、timeline semaphore、synchronization2 | 加：bindless（受 `maxPerStageDescriptorUpdateAfterBindSampledImages` 限制，资源数超限时分页）、间接绘制 |
| `Desktop` | 独显/集显 | §3、§4 全部 |

- timeline semaphore 在部分老 Android 驱动上不可用。`sync::Timelines` 需要一个回退实现：每次提交配一个 `VkFence` 和一个单调计数，`TimelineApi` 抽象正好容纳这种实现，上层的 `SyncPoint` 语义不变。
- GPU 驱动渲染（§4.3）在移动端只保留"间接绘制 + CPU 剔除"；mesh shader 基本不可用，compute 剔除的带宽代价常常高于收益，默认不开启。

### 8.7 功耗、发热与帧节奏

- **帧率上限与精确节奏**：默认锁 30 或 60 FPS，使用 `VK_GOOGLE_display_timing`（Android）或 present wait（§4.8）按显示节拍提交，不让 CPU/GPU 空转追帧。
- **动态分辨率**：根据上一帧 GPU 时间戳（§4.0）与热状态调整渲染分辨率，UI 保持原生分辨率并单独合成。热状态升高时依次降低：分辨率 → 阴影分辨率 → 后处理质量 → 帧率上限。
- **线程**：大小核架构下，并行录制线程数默认不超过大核数（通常 2–4）；`RecordingPolicy` 在 Tiler 画像下默认 `PerBatch` 单线程录制，只有剖析证明录制是瓶颈时才开并行。有些移动驱动的 secondary command buffer 开销偏高，不使用。
- **管线编译**：移动端驱动编译 shader 尤其慢，§4.1 的磁盘缓存与加载期预热是必需项，不是可选项。

### 8.8 平台集成

- **Android**：
  - surface 由 `ANativeWindow` 创建（`VK_KHR_android_surface`），现有代码只有 `Surface::from_win32`，需要补齐。
  - 应用切到后台或暂停时 surface 会被销毁：`SurfaceLost` 必须走"释放交换链 → 等待新窗口 → 重建"流程，而不是视为致命错误；后台期间释放可重建的资源（资源池、瞬态资源）以降低被系统回收的概率。
  - **预旋转**：交换链的 `preTransform` 使用 surface 的 `currentTransform`，按设备方向在渲染时旋转投影（同时交换视口宽高），避免合成器再做一次全屏旋转（这会额外消耗带宽和延迟）。
  - 交换链默认 3 张图像，`FIFO` 呈现模式。
- **iOS/macOS**：经 MoltenVK 运行于 Metal 之上，需启用 `VK_KHR_portability_subset` 并遵守其限制（部分格式、事件、三角扇等不可用）；Apple GPU 同样是 TBDR，8.2 的合并与惰性分配直接适用（对应 Metal 的 memoryless 存储）。若对 Apple 平台的性能要求高，长期可评估原生 Metal 后端，本轮不做。

### 8.9 测试与工具

- lavapipe 不模拟分块渲染，只能验证 8.2 合并逻辑的**功能正确性**（合并前后像素一致），带宽和功耗收益必须在真机上测。
- 编译器对合并与 load/store 推导输出可读的执行计划，并加单元测试：哪些 pass 被合并、每个附件的 load/store 操作、哪些附件是瞬态。
- 真机工具：Arm Mobile Studio（Streamline、Mali Offline Compiler）、Snapdragon Profiler、Android GPU Inspector（AGI）、Xcode GPU 帧捕获；开启验证层的 best-practices 检查（含 ARM 专项）作为 CI 的静态检查。
- 关键指标：每帧外部内存读写字节数、片上 tile 写回次数、GPU 活跃周期、温度上升曲线，而不只是帧时间。

### 8.10 对现有阶段的影响

| 阶段 | 移动端相关变更 |
| --- | --- |
| P2 | `DeviceProfile` 探测；统一内存检测；Tiler 下 `async_compute` 默认 `Off`；Android surface |
| P3 | `TimelineApi` 增加 fence 回退实现（无 timeline semaphore 的设备） |
| P4 | 渲染 pass 合并与 load/store 推导、惰性分配瞬态附件、usage 最小化保帧缓冲压缩；bindless 按特性分级启用；`raw::render_pass` 保留为编译器内部实现 |
| P5 | `SurfaceLost` 重建流程、预旋转、帧率上限与动态分辨率钩子 |
| P7 | ASTC/ETC2 上传与格式表；shader 半精度变体；后处理合并 |
| 资产管线 | 产出 ASTC/ETC2 纹理；shader 编译加 `RelaxedPrecision` 变体 |
