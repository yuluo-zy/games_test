# rhapsody 架构

本文描述 rhapsody 当前代码的模块划分、关键类型与协议，并列出维护者必须知道的不变量、限制与未验证区域。所有类型名与路径以源码为准。实现与验收状态的唯一登记入口是 [REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md)；交换链的策略、证明方式与注意点见 [src/present/swapchain/README.md](src/present/swapchain/README.md)。

## 定位与边界

rhapsody 是基于 `ash` 的 Vulkan 渲染库，提供以下能力：

- 渲染图：声明、规划（纯计算）、帧实例绑定、录制与提交。
- 设备与队列：逻辑队列规划、每队列时间线信号量、提交锁。
- GPU 完成追踪与延迟回收：`SyncPoint`/`SyncSet`、`Retirement<T>`、`Doomed`。
- 资源身份与范围状态：类型化句柄、`SyncCell`、`RangeStateTable`。
- bindless 描述符堆、异步上传（`TransferManager`）、交换链与帧循环（`FrameLoop`）。
- 管线后台编译与磁盘缓存。

边界：

- 不包含游戏或建筑业务知识。资源内容版本与脏标记由宿主提供，`rg::result_cache` 只负责协议、同步与生命周期。
- 窗口表面目前只有 Win32 构造路径（`native::Surface::from_win32`、`from_win32_raw`、`from_win32_retained`）。实例在 portability 枚举可用时启用 `VK_KHR_portability_enumeration`，设备在可用时启用 `VK_KHR_portability_subset`（`native/instance/mod.rs`、`device/init/mod.rs`）。移动端/Android 路径不存在，见"已知限制与注意点"。
- 渲染图只使用动态渲染（`cmd_begin_rendering` 的 core 或 KHR 入口，`rg/pass_api/mod.rs`）。源码中未发现 `VkRenderPass`/`VkFramebuffer` 的创建路径。

### Cargo 特性

`crates/rhapsody/Cargo.toml` 中的特性（以下为代码中实际的 `cfg` 门控）：

| 特性 | 默认 | 含义 |
| --- | --- | --- |
| `suballocation` | 开 | 启用 `gpu-allocator` 0.27 子分配（`device::suballoc`）。 |
| `validation` | 关 | release 构建也运行独立计划校验器；debug 构建始终运行。 |
| `cpu-probe` | 关 | CPU 侧计数探针（纹理上传探针、命令 workload 计数）。 |
| `test-support` | 关 | 把 `internal_mod!` 声明的模块以 `#[doc(hidden)] pub` 暴露给本 crate 的集成测试。不是 API。 |

运行时开关：`rg::set_plan_validation(bool)`。`rg::validation_enabled()` 为 debug 构建、`validation` 特性或该运行时开关任一成立时返回 true。

诊断开关：环境变量 `RHAPSODY_SUBMIT_TRACE=1` 开启提交分段观测（`src/trace.rs`）。注意：不存在名为 `trace` 的 Cargo 特性。

### 公开面

`lib.rs` 以 `pub mod` 公开：`accounting`、`bindless`、`collections`、`command`、`device`、`format`、`native`、`pipeline`、`profile`、`present`、`resource`、`rg`、`sync`、`tracking`、`transfer`。`error` 重导出 `Error`、`ErrorKind`、`Result`、`SharedError`；`log_ring` 的 `init_logging`、`record_log`、`dump_log_vec` 也重导出。`retire`、`log_ring`、`trace` 经 `internal_mod!` 声明，默认 `pub(crate)`。

公开面比"精选 API"更宽：`accounting`、`collections`、`format`、`native`、`profile`、`tracking` 都是 `pub mod`。

## 模块布局

```
src/
├─ lib.rs                  公开模块与 internal_mod! 声明
├─ error/                  Error / ErrorKind / SharedError；vk::Result 分类
├─ sync/
│  ├─ mod.rs               QueueId、SyncPoint、SyncSet、CompletedValues、MAX_QUEUES = 8
│  ├─ timeline/            Timelines<A>：每队列时间线信号量与提交锁；TimelineApi
│  └─ generation/          DeviceGeneration、Generationed（拒绝跨代际访问）
├─ retire/                 Retirement<T>：按 SyncSet 延迟回收的每队列有序结构
├─ device/
│  ├─ mod.rs               公开重导出
│  ├─ logical.rs           Device（五个组件字段）
│  ├─ init/                DeviceConfig、设备选择与能力启用、Device::new / new_headless
│  ├─ native/              DeviceCore：ash 句柄、实例、启用能力
│  ├─ queue_config/        QueueConfig、FamilyInfo、plan_queues、QueuePlan
│  ├─ queues.rs            主队列、Timelines、挂起 acquire
│  ├─ submission/          submit_prepared（crate 内生产提交入口）、SubmitBatch、Wait、Signal
│  ├─ lifetime.rs          Device::retire / collect_garbage、Lifetime
│  ├─ graveyard.rs         Doomed 枚举、DeferredMemory、RetirementStats
│  ├─ memory/              分配计量、驱动预算、驱逐登记
│  ├─ suballoc/, allocator/  gpu-allocator 子分配与设备内存块
│  ├─ host_flush/          非 coherent 映射内存的延迟 flush
│  ├─ caps/                DeviceCaps、Feature（硬件支持与实际启用分开）
│  ├─ policy/              ExecutionPolicy（描述符与分配策略）
│  ├─ extensions/          设备级惰性创建的上层状态
│  ├─ reclamation/         ReclamationConfig / ReclamationBudget（有界维护 tick）
│  └─ semaphore.rs, debug.rs, stats.rs, selection.rs, pools.rs, transient_backing.rs
├─ tracking/
│  ├─ handle/              Handle<T>（slotmap 键）、ResourceStore<T>
│  ├─ tracked/             SyncCell：最后 GPU 使用、所有权范围、提交门
│  ├─ range_state/         RangeStateTable、StatePatch、RangeOwner、NativeState
│  └─ sharing/             QueueFamilies、ResourceSharing
├─ resource/               Buffer、Texture、TextureView、Sampler、ManagedBuffer/ManagedTexture、
│                          DynamicConstants、ResourceName（name/）、CpuBytes
├─ bindless/               BindlessHeap、BindlessLease、BindlessIndex、BindlessPagePool、参数与 CPU credit
├─ command/
│  ├─ barrier/             同步范围与 sync2 / legacy 屏障下发
│  ├─ command_buffer/      命令池与命令缓冲、保留与退役
│  ├─ pending_acquire/     合并的跨族所有权 acquire
│  ├─ resource_access/     显式 raw 访问记录（与图使用同一范围状态）
│  ├─ descriptor_set/      描述符布局、池、池环
│  ├─ cpu_arena/           CPU 录制内存（不可用于 GPU 可见内存）
│  └─ crash/, recording/, device_ext.rs, texture_copy.rs
├─ transfer/               TransferManager、上传队列、tickets、flush、纹理上传、setup
├─ pipeline/               AsyncPipelineCompiler、PipelineCache、disk（磁盘缓存）、shader/、shader_interface/
├─ present/swapchain/      Swapchain、策略、acquire、retire、recreate、fullscreen/（见其 README）
├─ rg/
│  ├─ graph/               RenderGraph 声明、validate、build、FrameInstance（snapshot.rs）
│  ├─ plan/                GraphDefinition → CompiledPlan（纯规划，无物理对象）
│  ├─ plan_check/          独立校验器（range_program、bound、native_state、range_oracle）
│  ├─ sync_plan/           静态 RangeProgram 与每帧 bind（BoundTransition、StatePatch）
│  ├─ access_ledger/       PassBuilder 访问账本
│  ├─ queue_schedule/      按 QueueClass 分批，保持依赖 DAG
│  ├─ transient_memory/    瞬态内存放置规划（PhysicalMemoryPlan）
│  ├─ transient_memory_check/  放置结果的独立校验
│  ├─ transient_resources/ 别名集缓存（每帧借出）
│  ├─ temporal/            命名乒乓历史资源（TemporalTexturePair）
│  ├─ subgraph.rs          SubgraphInterface、SubgraphBuilder、SubgraphPlanCache
│  ├─ result_cache/        内容版本化的结果缓存（属于 graph session）
│  ├─ executor/            ExecutionChain：录制、提交、GraphExecution
│  ├─ parallel/            分块并行录制（bevy_tasks）
│  ├─ frame_imports/, import_export/  帧局部物理导入、导入导出
│  ├─ frame_loop/          FrameLoop：begin → acquire → execute → present → collect
│  ├─ frame_throttle/      在途提交节流（FrameThrottle<R>）
│  ├─ transaction.rs       FrameTransaction（批次计数与回执）
│  ├─ vulkan/              VulkanGraphBackend（backend/）、VulkanGraphSession（session/）、presentation/
│  ├─ attachments/         动态渲染附件与有界图像视图缓存
│  ├─ profiling/           可选 GPU 命令时间戳
│  └─ owner_storage.rs, frame_storage/, receipt_storage/, pass_bindings/, resource/, ...
├─ accounting/             MemoryBudgetController、EvictionRegistry / Evictable、CPU 存储记账、统计
├─ collections/            有界缓存（cache/）、CPU scratch、temp_list、token_map
├─ native/                 原生 Vulkan 包装：instance/、physical_device/、surface、submit、access_table、dxgi
├─ profile/                CPU 录制 profile（线程本地，默认关闭）
├─ log_ring/               进程级诊断日志环
├─ trace.rs                RHAPSODY_SUBMIT_TRACE 提交分段观测
└─ format/                 格式注册表

crates/shader-pipeline/    shader 资产协议（独立 crate，workspace 成员）
crates/shader-compiler/    shader 构建器（独立 crate，workspace 成员）
tests/                     集成测试（约 99 个条目，含文件与子目录；需要 Vulkan 设备的测试依赖运行环境）
examples/window_lab/       窗口实验程序
```

模块内的单元测试多为 `*_tests.rs` 或 `tests.rs`，与实现同目录。

## 关键设计（按子系统）

### 错误模型（`src/error/`）

`Error` 变体：`DeviceLost`、`OutOfDeviceMemory`、`OutOfHostMemory`、`SurfaceOutOfDate`、`SurfaceLost`、`Timeout`、`Exhausted(Cow)`、`Unsupported(Cow)`、`InvalidUsage { what, source }`、`Vulkan(vk::Result)`。`Error::kind()` 返回粗分类 `ErrorKind`。

`From<vk::Result>` 的分类：

- `DEVICE_LOST` → `DeviceLost`。
- `OUT_OF_DEVICE_MEMORY` → `OutOfDeviceMemory`；`OUT_OF_HOST_MEMORY` → `OutOfHostMemory`。
- `OUT_OF_DATE_KHR`、`FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT` → `SurfaceOutOfDate`；`SURFACE_LOST_KHR` → `SurfaceLost`；`TIMEOUT` → `Timeout`。
- `OUT_OF_POOL_MEMORY`、`FRAGMENTED_POOL` → `Exhausted`。
- `FEATURE_NOT_PRESENT`、`EXTENSION_NOT_PRESENT`、`LAYER_NOT_PRESENT`、`FORMAT_NOT_SUPPORTED`、`INCOMPATIBLE_DRIVER` → `Unsupported`。
- 其余 → `Vulkan(code)`。

规则：

- 调用方误用用 `Error::invalid_usage(..)` 或 `Error::invalid_usage_with(.., source)` 构造，并在 `log::debug!` 记录。
- `Exhausted` 表示有界容量耗尽，调用方应先回收再重试，与 `Timeout` 区分。
- `DeviceLost` 是粘滞的（见同步一节）。
- `SharedError` 是 `Arc<Error>` 的包装，用于同一错误需要被多方持有的场景。
- 模块内部的错误类型用 `thiserror` 派生 `Error`，不手写 `Display`。需要保持 `source()` 行为的类型例外：`SharedError`（`#[source]` 会返回 `Arc` 包装而不是内部 `Error`）与 `UploadFailure`（持有 `OnceLock<Error>`，没有可标注的字段）。许多类型的文案是 `{self:?}` 形式，这是沿用原实现。

### 设备（`src/device/`）

`DeviceConfig`（`device/init/mod.rs`）字段：

- `memory_budget_bytes`：设备内存预算，缓存与退役中的分配同样计入。
- `host_memory_budget_bytes`：宿主侧元数据预算，`0` 表示不限。
- `queues: QueueConfig`、`validation`、`required_features: FeatureSet`、`preferred_device: DevicePreference`。
- `frames_in_flight`：默认 2，`validate()` 要求在 `1..=3`。
- `allocation: AllocationStrategy`：默认 `Suballocated`，另有 `Dedicated`。
- `reclamation: ReclamationConfig`：每次维护 tick 的回收上限。

创建入口：`Device::new(&Arc<VulkanInstance>, Option<&Surface>, &DeviceConfig) -> Result<Arc<Device>>` 与 `Device::new_headless(&DeviceConfig)`。`device/selection.rs` 另有 `create_headless_device()` 与 `create_headless_device_configured(..)`。

`Device` 持有五个组件字段（`device/logical.rs`），另有自引用 `self_ref`，以及 `cpu-probe` 特性下的上传探针。组件字段按声明顺序析构：

- `extensions: Extensions`：设备级惰性创建的上层状态。设备本身不能命名资源或传输类型，这些类型依赖设备。
- `lifetime: Lifetime`：退役队列（`Retirement<Doomed>`）与回收预算。
- `memory: Memory`：分配计量、可选子分配器、驱动预算、驱逐登记。
- `queues: Queues`：`QueuePlan`、`Timelines`、主队列与附加队列、挂起的 acquire。
- `core: DeviceCore`：ash 句柄、实例、启用的能力。必须最后声明，因为它拥有 `VkDevice`。

`Drop for Device`：先 `drain_retirement_for_drop()`（`vkDeviceWaitIdle`，失败只记 warn 并继续；随后循环排空退役队列），再 `extensions.release_for_device_drop()`，之后字段按声明顺序析构。

主要方法：`queue(QueueClass) -> Result<QueueId>`、`poll()`、`completed()`、`wait(&SyncSet, timeout_ns)`、`is_device_lost()`、`wait_idle()`、`retire(SyncSet, Doomed)`、`collect_garbage()`、`try_collect_garbage()`、`wait_idle_and_collect()`、`retirement_stats()`、`caps()`、`limits()`、`raw_device()`、`primary_queue()`、`queue_plan()`、`timelines()`。

提交入口：生产路径经 `device/submission/mod.rs` 的 `submit_prepared`（`pub(crate)`）完成。公开的批次类型是 `SubmitBatch`，其 `Wait` 为 `Binary { semaphore, stage }` 或 `Timeline { point, stage }`，`Signal` 仅有 `Binary(semaphore)`。时间线 signal 由 `Timelines::submit` 分配，不由调用方传入。

#### 队列规划（`device/queue_config/mod.rs`）

- `QueueConfig { async_compute, transfer: QueuePreference, require_present }`，`QueuePreference` 为 `Off` / `Prefer` / `Require`。
- `plan_queues(&[FamilyInfo], &QueueConfig) -> Result<QueuePlan>`；`QueuePlan` 提供 `queue(class)`、`is_aliased(class)`、`physical(class)`、`create_counts()`、`families()`。
- Graphics：第一个同时支持 GRAPHICS 与 COMPUTE 的族；`require_present` 时该族必须支持目标 surface 的呈现。
- Compute：`Off` 直接别名到 graphics 队列。否则依次尝试：专用 COMPUTE 族（无 GRAPHICS）、graphics 族的空闲第二队列、任意 COMPUTE 族。全部失败时 `Require` 报 `Unsupported`，`Prefer` 别名到 graphics。
- Transfer：`Off` 直接别名。否则依次尝试：专用 TRANSFER 族（无 GRAPHICS/COMPUTE）、专用 compute 族的空闲队列、graphics 族的空闲队列、任意具备 fine-grained transfer 的族（图像传输粒度为 1×1×1）。全部失败时 `Require` 报 `Unsupported`，否则别名到 graphics。
- 别名队列共用同一个 `QueueId`、`VkQueue` 与时间线，语义与单队列一致。

### 同步与完成（`src/sync/`）

数据类型：

- `QueueId`：`0..MAX_QUEUES`（8），`QueueId::new(index) -> Option<Self>`。
- `SyncPoint { queue, value }`：某队列时间线上的一个值。
- `SyncSet`：每队列一个 `u64`（0 表示不依赖）。`insert` 与 `merge` 取逐队列最大值，`points()` 迭代非零项，`is_reached(&CompletedValues)` 判断是否全部完成。
- `CompletedValues`：各队列完成值的快照，`all()` 表示全部完成（idle 或设备丢失后使用）。
- `DeviceGeneration` 与 `Generationed`：标识设备的一次生命周期，拒绝跨代际的访问。

`Timelines<A: TimelineApi>`（`sync/timeline/mod.rs`）：

- `TimelineApi` 为 trait，生产实现是 `ash::Device`，测试使用 `FakeApi`。
- 每个队列持有一条时间线信号量、一把提交锁、已提交值（`submitted`）与已完成值缓存（`completed`）。
- `submit(queue, |SubmitSignal { semaphore, value }| -> Result<R>) -> Result<(SyncPoint, R)>`：在提交锁内计算 `value = last + 1`，闭包成功后才发布 `last` 与 `submitted`。
- `with_queue(queue, f)`：在提交锁内执行不 signal 的队列操作（如 `vkQueuePresentKHR`）。
- `with_all_queues(f)`：按 `QueueId` 升序取全部提交锁，用于需要整设备同步的调用。
- `submitted(queue)`、`submitted_set()`：只包含成功提交的工作。
- `completed()`：无设备查询，返回缓存快照。`poll()`：只对仍有未完成提交的队列查询 `vkGetSemaphoreCounterValue`，结果以 `fetch_max` 写回。`refresh()` 等同于 `poll()`。
- `is_reached(&SyncSet)`、`wait(&SyncSet, timeout_ns)`：先校验目标不超过 `submitted`。
- `is_device_lost()`、`mark_device_lost()`：粘滞标志。
- `set_lock_statistics(bool)` 与 `queue_lock_stats()`：可选的锁统计，默认关闭。

`Retirement<T>`（`src/retire/mod.rs`）：

- 内部为每队列的预分配有序节点结构。原生对象的 admission 在创建之前完成（`RetirementTicket` 的 `reserve`，随后 `enqueue_reserved`），入队与跨队列移动只转移已有所有者。
- `retire(after, item, &CompletedValues) -> Result<Option<T>, T>`：若已完成，把对象交还调用方立即销毁；否则入队。`try_enqueue(after, item)` 是无原生对象调用方的入口。
- `collect(&self, completed, &mut Vec<T>)`、`collect_limited(..)`：移出到期对象。`drain_all(&self, ready)`：用于 idle 或设备丢失后。
- 销毁在所有队列锁之外执行（模块说明）。

`Device::retire(after, Doomed)`（`device/lifetime.rs`）：若 `after` 已完成则立即销毁，否则进入退役队列。`Doomed` 枚举（`device/graveyard.rs`）包括 `Buffer(vk::Buffer, DeferredMemory)`、`Image(vk::Image, DeferredMemory)`、`ImageView`、`Sampler`、`Semaphore`、`QueryPool`、`PipelineLayout`、`Pipeline`、`DescriptorSetLayout`、`DescriptorPool`、`DescriptorSet`、`CommandBuffer`、`Owners(TempList<ResourceOwner>)` 等。`DeferredMemory` 为 `Shared`、`Dedicated`、`Sub` 之一。

### 资源身份与使用追踪（`src/tracking/`、`src/resource/`）

- `Handle<T>`（`tracking/handle`）：内部为 `slotmap::DefaultKey` 加 `PhantomData` 标记，`Copy`。`ResourceStore<T>` 是 `SlotMap` 的类型化封装，只负责存储，不做线程同步。删除后代际递增，旧句柄不会访问到复用槽位的新资源。
- `Buffer` 与 `Texture`（`resource/buffer`、`resource/texture`）持有：`last_use: Arc<SyncCell>`、可选 `allocation_use`、`sharing: ResourceSharing`，以及 `RetireTicket` 形式的退役凭据。析构时（`resource/texture/create.rs` 的 `Drop` 为例）经 `Device::retire_reserved` 把原生对象与 `last_use` 快照交给退役队列。
- `SyncCell`（`tracking/tracked`）：记录资源的最后 GPU 使用。实现为每队列一个 `AtomicU64`，登记用 `fetch_max`，读只发生在回收边界。另持有 `submission_gate` 互斥锁：预留（reservation）覆盖队列调用与最后使用发布的全过程，宿主访问需要先取同一把门，以关闭快照竞争。`SyncCell` 还可携带 `OwnershipRange`（`Buffer { offset, size }` 或 `Image { aspects, base_mip, mip_count, base_layer, layer_count }`）与 CPU 记账预留。
- 每次提交成功得到 `SyncPoint` 后，执行器把它登记到本次提交引用的每个资源的 `SyncCell`。
- `RangeStateTable`（`tracking/range_state/table.rs`）：逐范围的写者/读者前沿、布局与所有权。流程为 `prepare(&[StatePatch], queue, family)`（在提交门内），成功提交后 `publish(point)`，失败则 `cancel_prepared()`。宿主访问走 `prepare_host` / `publish_host` / `host_range_snapshot`。
- `RangeOwner`（`tracking/range_state/owner.rs`）：`Unclaimed`、`Owned(u32)`、`Released { transfer, point }`。准入判断定义在 `RangeOwner::admits`。
- `ResourceSharing` / `QueueFamilies`（`tracking/sharing`）：独占或并发的队列族共享描述，属于资源身份与缓存键的一部分。
- `ResourceName`（`resource/name`）：基于 `ustr::Ustr` 的进程级驻留字符串。驻留表只增不减，名字必须来自有限集合，不能按每帧唯一 ID 生成。
- 调试名经 `device/debug.rs` 的 `set_object_name` 设置（`VK_EXT_debug_utils`，无扩展时跳过）。是否对每个创建点都调用，本轮未逐一核实。
- 托管资源（`resource/managed`）：`ManagedBuffer`、`ManagedTexture`、`ManagedTextureView` 与 `ManagedResources`，内部使用 typed generational store 的弱索引。

### Bindless（`src/bindless/`）

- `BindlessHeap::new(device, BindlessConfig)`：每种 `DescriptorKind`（`SampledImage`、`StorageImage`、`StorageBuffer`、`Sampler`）一个 update-after-bind 表。
- 资源以 `Arc<BindlessLease>` 进入堆，`lease.index() -> BindlessIndex`。API 包括 `sampled_image`、`storage_image`、`storage_buffer`、`sampler`。
- lease 不可变：更换资源分配新槽位。槽位只在所有已登记的 GPU 使用完成后归还空闲表（模块说明）。
- `ResidentLeases`（`residency.rs`）与 `collect()` 负责驻留与回收。
- `BindlessPagePool`、`BindlessPageCursor`、`GroupReservation` 提供按页的参数组预留（细节见 `pages.rs`）。
- 参数与 shader ABI：`BindlessParameters`、`BindlessResource`、`BindlessShaderPackage`；参数 payload 的 CPU 记账见 `bindless/parameters/cpu`。
- 准入：`device::plan_policy` 要求 `bindless_features()` 全部启用，否则返回 `Unsupported`。

### 命令、屏障与原生访问（`src/command/`）

- `command::barrier`：同步范围（`SynchronizationScope`）与按设备能力选择 sync2 或 legacy 的 Vulkan 屏障下发（模块说明）。
- `command::command_buffer`：命令池与命令缓冲，负责资源保留与退役。`VulkanCommandPool::reset_completed(&[&mut VulkanCommandBuffer])` 只有在池的所有提交都已到达完成点、且所有存活命令都被独占借出时才整池复位；失败时保留命令状态、所有者与最后使用回执。
- `command::pending_acquire`：跨队列族上传的所有权 acquire 合并。上传在传输队列上 release；挂起的 acquire 由后续录制路径（包括 `TransferManager::flush_acquires`）消费。具体合并规则见模块说明。
- `command::resource_access`：显式 raw 访问记录，与图使用同一套范围状态和“成功提交后发布”规则。
- `command::cpu_arena`：CPU 录制内存，检出时独占一块固定的 bumpalo chunk，最后一个 CPU 借用结束后归还。该内存不能用于 GPU 可见的上传或回读。
- `command::descriptor_set`：布局、池与集合，`pool_ring.rs` 负责池环的 `reset_if_idle`。

### 图后端的命令池（`src/rg/vulkan/backend/`）

- `VulkanGraphBackend` 绑定一个实际选中的队列族（`new`、`new_for_class`、`new_for_queue`，`selected_family()`）。
- 帧命令池按 `FramePoolKey { frame_slot, worker, family }` 索引，每个池记录 `epoch`。epoch 变化时，若该帧槽仍有未回收的命令则返回 `NOT_READY`，否则对其已完成命令调用 `reset_completed`。
- 帧槽由 `active_frame_slot` 选择；worker 与 family 维度保证并行录制不共享池。

### 传输（`src/transfer/`）

- `TransferManager::new(device, TransferConfig)`；`TransferConfig` 默认 `slots: 4`、`chunk_bytes: 4 MiB`，`max_bytes` 同时计入上传与回读的真实内存需求。
- 入队：`enqueue_upload`、`enqueue_upload_owned` 返回 `QueuedUploadTicket`；`cancel_upload`、`cancel_queued_before(epoch)` 取消排队中的上传。
- 提交与回读：`flush_uploads(max_submits) -> UploadFlush`、`upload_textures(&[TextureUpload]) -> SyncPoint`、`flush_acquires() -> Option<SyncPoint>`、`read_buffer(..)`。
- tickets：`UploadTicket::completion() -> SyncPoint`、`wait(timeout_ns)`、`is_complete()`、`try_read(&mut [u8]) -> Result<Option<usize>>`。ticket 能检查所属 buffer 是否仍存活以及设备代际。
- 上传在设备规划的传输队列上执行；该队列与图形队列别名时语义同单队列。

### 渲染图：声明与验证（`src/rg/graph/`、`src/rg/pass_bindings/`、`src/rg/resource_contracts/`）

- `RenderGraph::new()`（或 `with_shape(DeclarationShape)`）；`add_pass(name) -> PassBuilder`，`PassBuilder` 提供 `on_queue(QueueClass)`、`render(..)`、`render_with_context(..)`、`render_with_result_context(..)`。
- `gate_first_use(resource)`（`rg/graph/declare.rs`）：把图像获取的等待挂在第一次使用该资源的批次上。呈现会话在 `rg/vulkan/presentation/mod.rs` 中调用它。
- 回调绑定校验：`pass_bindings::validate_declaration`，在触碰设备对象或描述符分配器之前检查绑定与图作用域、声明范围是否一致。
- 描述符级资源校验：`resource_contracts::validate_contracts`。

### 渲染图：规划（`src/rg/plan/`、`src/rg/plan_check/`）

- `GraphDefinition`（`rg/plan/definition.rs`）是声明的纯数据形式。`compile_definition(GraphDefinition) -> Result<CompiledPlan>`（`rg/plan/compile.rs`）只计算符号化结果：依赖、批次、队列调度与生命周期。计划只持有声明与符号索引，不持有物理对象、回调或完成点（模块说明）。
- 访问码经 `ACCESS_TABLE`（`native/access_table.rs`）与 `AccessTable`（`rg/vulkan/access_table.rs`）降为阶段、访问掩码与布局。
- `plan_check::check_plan`：独立校验器。它不调用生产的范围拆分与状态转换实现，而是用自己的稠密格重新推导，并与规划结果比较。`transient_memory_check::check_memory_plan` 对瞬态放置做同样的独立校验。校验在 `rg::validation_enabled()` 为真时于计划缓存未命中时运行。
- 计划缓存：哈希只用于定位候选，完整规范化键相等才确认命中（`collections/cache` 的契约）。

### 渲染图：同步程序与绑定（`src/rg/sync_plan/`）

- `RangeProgram`（`rg/sync_plan/mod.rs`）：静态的单一范围同步程序，编译一次。
- `bind`（`rg/sync_plan/bind/`）：每帧把静态程序叠加到物理 `SyncCell` 的范围状态表上，产出 `BoundTransition` 与 `StatePatch`。`bind/shadow.rs` 与 `bind/access.rs` 分别处理影子状态与访问。
- 串行、并行与所有权前置录制消费同一份绑定后的投影（`sync_plan` 模块说明）。

### 渲染图：队列调度（`src/rg/queue_schedule/`）

- 按 `QueueClass` 分批，并保持图的依赖 DAG。物理队列别名不产生新的批次边界，也不产生信号量依赖。
- 跨族所有权转移由 `command::pending_acquire` 与帧局部导入共同处理（见上）。

### 渲染图：实例、执行与事务（`src/rg/graph/snapshot.rs`、`src/rg/executor/`、`src/rg/transaction.rs`）

- `FrameInstance` 与 `FrameInstanceTables`（`rg/graph/snapshot.rs`）承载每帧的物理绑定：导入对象、池分配、回调参数与回执。它不进入编译缓存。
- `ExecutionChain`（`rg/executor/mod.rs`）执行录制与提交。结果为 `GraphExecution { compiled: FrameInstance, transaction: FrameTransaction, stats: ExecutionStats }`，`completion() -> SyncSet` 给出本次执行的完成集合。
- `FrameTransaction`（`rg/transaction.rs`）记录计划批次数、已发布批次数、`SmallVec<[SyncPoint; 8]>` 形式的回执，以及完成集合。`prepare(&FrameInstance)` 在提交前用 `try_reserve_exact` 预留回执容量，失败返回 `OutOfHostMemory`。它不是显式的状态机（见"已知限制"）。
- 并行录制（`rg/parallel`）：默认配置为 `ParallelRecording::PARALLEL`。`plan_chunks_into` 依据上一帧各 pass 的 CPU 录制耗时把一个批次切成连续块，每块一条命令缓冲；`map_chunks` 基于 `bevy_tasks` 执行。切块策略为 `ChunkStrategy::Balanced` 或 `Adaptive`，后者在预计收益不超过调度成本时不切块。
- `GpuTimingConfig` 与 `rg/profiling`：可选的 GPU 命令时间戳，每对查询只由一条提交上的一个命令写入，完成由该提交的时间线点确定。

### 渲染图：瞬态内存与别名（`src/rg/transient_memory/`、`src/rg/transient_resources/`）

- `plan_memory_requirements(..) -> PhysicalMemoryPlan`：使用驱动给出的内存需求与既有依赖证明，生成 `Placement`、`PhysicalBacking`、`PlanStats`。
- 后端的 `set_transient_aliasing(bool)` 控制是否启用别名，构造时默认开启。
- 别名集（`AliasCache`）按物理计划缓存，每次只借给一个在飞帧（`lend`），帧结束归还（`give_back`），退役中的别名由 `collect_retiring` 回收，空闲的由 `evict_idle` 驱逐。
- `transient_memory_check` 的独立校验会检查放置结果，不信任规划器的分组决策。

### 渲染图：时间性资源（`src/rg/temporal/`）

- `TemporalTexturePair`：`current()`、`previous()`、`history_valid()`、`current_history_valid()`、`parity()`。命名资源为 `NamedTexture`、`NamedBuffer`。
- 会话级 API：`reset_temporal(name)`、`reset_all_temporals()`（`rg/vulkan/session/named.rs`）。
- 历史版本只有在对应输出成功提交后才发布；失败的帧回滚（`temporal/recorder_tests.rs` 中有对应用例）。

### 渲染图：子图与结果缓存（`src/rg/subgraph.rs`、`src/rg/result_cache/`）

- 子图：`SubgraphInterface`（端口为 `SubgraphPort`，队列约束为 `SubgraphQueueConstraint`）、`SubgraphBuilder::subgraph(..)`、`validate_subgraphs`。
- `SubgraphPlanCache::new(max_entries, max_bytes)`，带 `set_enabled`、`stats()`、`clear()`。
- 结果缓存：`ResultCache::new(entries, bytes)`，属于 graph session 而非 plan。`ContentDependency::external(id, version)`、`ResultRequest`、`ResultDependencies`、`ResultLookup`。开关为 `set_enabled`。
  - `ResultNodeId` 由宿主选定，在同一个缓存内必须唯一且跨帧稳定；它不含每帧都变化的 `GraphId`，否则所有跨帧命中都会失效。
  - 每个 `ResultCache` 创建时领取一个进程内唯一的命名空间，写入缓存 key，并盖在它签发的 `Result` 依赖上。`External` 依赖由宿主版本化，命名空间为 `0`。版本号按缓存各自计数，所以另一个缓存签发的 `(node, version)` 不能当作本缓存的依赖：这样的依赖被视为不可缓存，该节点本帧照常执行并计入 `bypassed`。
  - 失效（`invalidate_result_dependency`）只作用于本缓存签发的 `Result` 依赖。

### 帧循环与交换链（`src/rg/frame_loop/`、`src/present/swapchain/`）

- `FrameLoop::new(VulkanGraphSession, Arc<Swapchain>)`：要求交换链策略为 `SwapchainPolicy::Strict`，否则返回错误（`frame_loop/mod.rs`）。
- 一帧的典型顺序：`begin_frame() -> RenderGraph` → 构图 → `execute_graph(BuiltGraph)` 或 `execute(..)` → `acquire() -> Option<AcquiredImageLease>` → `present(&PresentTarget) -> PresentationReceipt` → `wait_render(receipt, timeout)` / `collect()`。
- 其他方法：`resize`、`replace_surface`、`set_focused`、`set_swapchain_contents`、`set_on_failure(PresentPolicy)`、`last_cpu_timings()`、`wait_idle(timeout)`、`shutdown(timeout)`。
- 在途帧节流：`FrameThrottle<R>`（`rg/frame_throttle`）以 `VecDeque` 保存提交回执，`begin_frame` 等待直到只剩最近一个。
- `SwapchainPolicy`：`Strict`（默认，生产呈现）与 `AcquireOnly`（只获取，拒绝呈现）。
- `PresentPolicy`：默认 `SkipPresent`（不呈现未定义内容），另有 `ClearTo(color)`。
- `PresentProof`（`present/swapchain/policy.rs`）：`Fences`（有 `VK_EXT_swapchain_maintenance1` 时用 present fence）与 `QueueIdle`（无该扩展时的弱证明）。细节见交换链 README。
- 渲染完成与呈现完成分开证明：渲染完成只回收渲染资源，不单独释放呈现资源（见 `src/present/swapchain/README.md`）。

### 管线编译与磁盘缓存（`src/pipeline/`）

- `AsyncPipelineCompiler::new(Arc<PipelineCache>, CompilerConfig)`：请求接口为 `request_raster`、`request_compute`、`request_compute_spirv`、`defer_raster`、`defer_compute_spirv`（及 `_with_options` 变体）。`stats()`、`cancel_queued_before(epoch)`。
- 编译器由 renderer 持有，不由 `Device` 持有，避免设备与缓存之间的所有权循环（`pipeline/mod.rs` 说明）。编译结果在帧边界发布。
- `pipeline/disk.rs`：持久化磁盘缓存，负责格式、完整性校验与原子写入。
- `pipeline/shader_interface/`：所有管线创建路径共用的入口契约。`pipeline/shader/` 使用 `rspirv` 解析 SPIR-V，反射信息来自存储的 shader 容器目录（`shader-pipeline` crate）。

### 内存预算、记账与回收（`src/accounting/`、`src/device/memory/`、`src/device/reclamation/`）

- `MemoryBudgetController`（`accounting/budget`）：`new(limit)`、`new_with_watermarks(limit, high, low)`；`reserve`、`record`、`release`、`evict_cache`；`tick() -> BudgetAction`；`is_under_pressure()`。分类由 `BudgetCategory` 表示。
- `EvictionRegistry`：持有 `Weak<dyn Evictable>`，优先级为 `Transient` < `Normal` < `Expensive`，`evict(&CompletedValues, target_bytes)` 只释放已完成的条目。
- 驱动预算：`Feature::MemoryBudget` 开启时，`Memory::refresh_memory_budget` 读取 `VK_EXT_memory_budget` 快照；未开启时只使用配置预算。
- `ReclamationConfig` / `ReclamationBudget`：每个维护 tick 有上限。关闭和显式排空使用完整的回收操作，不受上限约束。
- `device/host_flush`：非 coherent 映射内存的写入先登记，在每次提交前合并为一次 `vkFlushMappedMemoryRanges`。
- 宿主内存：`accounting/cpu_storage` 的 `CpuMemoryReservation` 为渲染器持有的宿主存储做准入记账，`collections` 提供有界缓存（`cache`）与 CPU scratch。

### 诊断与观测

- `log_ring`：固定槽位的进程级诊断日志环，`record_log`、`dump_log_vec`、`init_logging`。
- `profile`：线程本地的 CPU 录制 profile（span 与计数器），未激活时关闭，不产生争用。
- `trace.rs`：`RHAPSODY_SUBMIT_TRACE=1` 时记录提交各阶段耗时，使用固定大小数组，不分配堆内存。
- `device::stats`：`RenderStats`、`StatsSnapshot`；`Timelines::poll_stats()` 给出 poll 次数与驱动查询次数。

## 不变量

以下规则是代码与测试共同依赖的前提。修改相关代码时必须保持它们成立。

1. 提交值只在 `Timelines::submit` 的闭包返回 `Ok` 后发布（`last` 与 `submitted` 同时更新）。失败的提交不占用值，因此任何已发布的 `SyncPoint` 最终都会完成，除非设备丢失。
2. `wait` 与 `is_reached` 拒绝大于 `submitted` 的目标，返回 `InvalidUsage`，而不是永久挂起。
3. 对 `VkQueue` 的 `vkQueueSubmit`、`vkQueuePresentKHR` 等调用必须位于该队列的提交锁内（`submit`、`with_queue` 或 `with_all_queues`）。提交锁内只做队列调用，不做阻塞等待与 `vkDestroy*`。
4. 完成缓存只增不减：`poll` 对完成值使用 `fetch_max` 写回。
5. `DeviceLost` 是粘滞的。`Timelines` 观测到后，后续 `submit`、`with_queue`、`poll`、`wait` 立即返回 `DeviceLost`。
6. 退役对象只在其 `SyncSet` 中每个队列的值都已完成后才销毁。销毁在队列锁之外执行。
7. `Device` 析构先 `vkDeviceWaitIdle` 并排空退役队列，再析构组件；`core` 必须最后析构。
8. 句柄代际：`Handle<T>` 删除后，旧键不会访问到复用同一槽位的新资源。
9. bindless lease 不可变，槽位在所有已登记的 GPU 使用完成之前不归还空闲表。
10. 范围状态只在提交门内 `prepare`，并仅在提交成功后以真实 `SyncPoint` `publish`。
11. 图计划（`rg::plan`）不包含物理对象、回调或完成点；这些属于每帧的 `FrameInstance`。
12. 计划缓存命中需要完整规范化键相等，哈希碰撞不能导致错误复用。
13. 生产 `FrameLoop` 只接受 `SwapchainPolicy::Strict`。
14. 命令池只有在所有使用它的提交完成、且没有借出的存活命令时才整池复位。
15. 管线编译结果在帧边界发布；编译器归 renderer 所有，不归 `Device` 所有。
16. `cpu_arena` 的内存不能作为 GPU 可见的上传或回读内存。
17. 非 coherent 映射内存的写入在提交前统一 flush。

## 已知限制与注意点

- 移动端与 TBDR 方向不存在于代码中：没有设备画像（`DeviceProfile`）、没有渲染 pass 合并与子 pass、没有惰性分配的瞬态附件、没有 Android surface、没有 timeline semaphore 的 fence 回退实现。不要把这些视为已有能力。
- GPU 驱动渲染、mesh shader、异步 compute 的关键路径调度均未实现。`QueuePreference` 只做静态的队列规划。
- `Feature` 中的 `DescriptorBuffer`、`GraphicsPipelineLibrary`、`ShaderObject`、`MeshShader`、`PresentId`、`PresentWait` 只做能力探测并记录在 `DeviceCaps` 中。在 `device/caps` 与 `device/init` 之外，本轮未发现生产路径使用它们。生产描述符策略只有 `DescriptorStrategy::Bindless`。
- 窗口表面只有 Win32 构造路径。macOS 上通过 MoltenVK 的 portability 路径运行，具体验证范围见 REFACTOR_PROGRESS.md。
- 显式逃生口是 `native` 模块与各类型的 `raw_device()`、`raw_handle()` 等访问器；没有单独的 `interop` 模块。
- `Error::invalid_usage` 只写 `debug` 日志。调用方必须传播或显式处理返回的错误，否则误用会被静默吞掉。
- `Texture::image_view` 每次调用都会创建视图，纹理本体不缓存视图。有界的图像视图缓存位于 `rg/attachments`。
- `FrameTransaction` 只记录批次计数、回执与完成集合，不是显式的状态机。失败路径的清理语义由 `ExecutionChain` 决定，本轮未逐条核对。
- 多队列：本机与 lavapipe 只有单个族、单个队列，真实多族路径未在本轮运行。默认 `QueueConfig` 的 `async_compute` 与 `transfer` 均为 `Off`，即默认单队列别名。
- 录制分块由 `ChunkStrategy` 与上一帧的 CPU 耗时决定。不存在按批次的 `RecordingPolicy`。
- `ResourceName` 的驻留表只增不减，名字必须来自有限集合。
- `Device::poll` 对每个仍有未完成提交的队列发起驱动查询，高频调用的成本由 `poll_stats()` 观测。
- 交换链的 `QueueIdle` 证明弱于 fence，超时行为与未覆盖的测试见交换链 README。
- 宿主侧 `Timelines::completed()` 返回的是缓存快照，可能偏保守（报告为未完成），但不会过早报告完成。需要新鲜值时使用 `poll()`。

## 验证状态

2026-10-28，Linux 开发机：

- `cargo test -p rhapsody --lib`：494 通过，2 个显式忽略。其中 `sync`、`retire`、`device::queue_config`、`error` 子集共 50 项，全部通过。
- 本轮未运行：`tests/` 下的集成测试（含 lavapipe 路径与 `p0_perf_baseline`、`p0_stability` 等）、`examples/window_lab`、macOS/MoltenVK、Windows、Intel 等目标硬件、真实多队列硬件。本机存在 lavapipe ICD（`/usr/share/vulkan/icd.d/lvp_icd.json`），但本轮未以它运行 GPU 测试。
- 以上未运行项的状态以 [REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md) 为准；本文不对其作出通过声明。
