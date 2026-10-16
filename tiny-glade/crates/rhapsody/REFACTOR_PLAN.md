# rhapsody 架构重构计划

2026-10-16 起，rhapsody 不再以"还原原版"为目标，**正确性与高性能是唯一标准**。原版地址、布局、panic 行为不再约束设计；原版 oracle 测试直接删除；对外 API 允许破坏性变更（下游 `tiny-glade`、`tg-text` 同步修改）。本文件是各阶段的进度入口；目标架构与各模块详细设计见 [ARCHITECTURE.md](ARCHITECTURE.md)。

## 目标架构

```
rhapsody
├─ error.rs        Error { DeviceLost, OutOfMemory{device|host}, SurfaceOutOfDate, SurfaceLost,
│                  Timeout, Unsupported(..), InvalidUsage(..) } + Result<T>
├─ device/         Device（唯一设备对象）、DeviceConfig/QueueConfig、Queue（各自锁 + 时间线）、SyncPoint
├─ resource/       Handle<T>（slotmap 代数键）、ResourceName(Ustr)、buffer/texture/sampler/view、Retirement
├─ pipeline/       shader、descriptor、render pass、管线缓存
├─ graph/          类型化 TextureId/BufferId → 编译（屏障、别名、跨队列同步）→ 执行
├─ frame.rs        FrameLoop：begin → acquire → main → present_graph → submit → present
└─ swapchain.rs    自带锁；二值信号量按图像槽；最小化/过期 → Skip/Recreate
```

关键约定：

- **同步与回收**：每个 `Queue` 一条时间线；提交在本队列锁内分配 `next_value`，`vkQueueSubmit` 成功后才提交该值（失败不占用值，不做 CPU 补发 signal）。资源与命令池记录 `SyncPoint{queue, value}` 集合，`Retirement` 是唯一回收真相源。设备丢失进入 `DeviceLost`，等待立即返回错误。
- **锁**：无全局锁。锁顺序 Swapchain → Queue → Retirement；分配器自带锁；内存持久映射。
- **错误**：公共 API 一律 `Result<T, Error>`；调用方误用返回 `InvalidUsage`，`debug_assert!` 仅用于内部不变量。帧失败取消未提交部分，已提交 GPU 副作用不可回滚，按真实完成点退役；只有 `DeviceLost` 使对象不可用。
- **资源身份**：raw 层 `Handle<T>`，过期句柄可检测；图内 `TextureId`/`BufferId` 区分类型；跨帧资源以 `ResourceName` 为主键，temporal 为按名字的乒乓对，框架负责奇偶与首帧 reset；调试名自动设置。
- **多队列**：`QueueConfig { graphics: 1, async_compute: Off|Prefer|Require, transfer: Off|Prefer|Require }`；`Prefer` 不支持时别名到 graphics；pass 声明 `QueueClass`，编译器插入跨队列时间线同步与 ownership transfer（独占/CONCURRENT 可配置）。
- **性能**：命令池按帧环 × 线程 × 队列族隔离，全部使用完成后整池复位；描述符每帧池 reset；屏障按 pass 批量 `vkCmdPipelineBarrier2`。不可变图计划与帧资源实例分开，缓存与分配均受字节预算约束。
- **完成域**：设备时间线管理渲染提交完成；呈现资源用独立 `PresentTicket`/交换链代际证明可回收，不能以渲染完成代替呈现完成。

## 阶段

| 阶段 | 内容 | 状态 |
| --- | --- | --- |
| P0 | 删除原版 oracle 测试；1000 帧稳定性测试（P3 前 ignore）；性能基线 | 完成 |
| P1 | 统一错误模型（去 `Result<_, String>` 与原版 panic） | 进行中：图编译/会话改用统一错误，资源创建保留设备错误类别；声明与 mip 越界返回错误。raw 兼容 API 尚待迁移 |
| P2 | 设备合一 + 可配置队列 + 去全局 `queue_lock`（下游同步迁移） | 部分完成：全部生产队列调用已接每队列锁；配置规划接入设备创建。两个设备包装尚未合并 |
| P3 | 统一同步/回收：`SyncPoint` + `Retirement`；删 graveyard/SequenceRuntime/SubmissionEngine/销毁桶/events 日志 | 部分完成：设备时间线与 Retirement 已接线，删除 epoch/补发 signal/图状态镜像/热路径字符串日志；SequenceRuntime 与旧托管销毁桶、帧环池仍待迁移 |
| P4 | 语义化资源身份：`Handle<T>`、`ResourceName`、命名 temporal | 未开始 |
| P5 | 帧控制流闭合：acquire→主图→展示图→present；headless 整帧测试；帧失败回滚 | 图会话取消未提交命令、等待已提交批次后清理并允许下一帧；WSI FrameLoop 与呈现完成协议尚未闭合 |
| P6 | 图内多队列调度（lavapipe 只能用别名队列验证；真实多族需真机） | 未开始 |
| P7 | 删重复实现与原版注释；性能三项并对比基线；更新文档与 provenance 规则 | 未开始 |

## 当前实现记录

本轮已完成的接线、验证及剩余边界见 [REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md)。后续新增的设计升级要求仍以本文与 `ARCHITECTURE.md` 为准；基础接线通过测试不表示 P1–P7 全部完成。

## P0 结果

- 删除 9 个测试文件与 45 个断言原版机器码/布局/panic 文案的测试函数；保留行为测试。测试 342 → 298 通过 + 2 ignored。
- `tests/p0_stability.rs`：1000 帧会话簿记（在飞帧、资源/命令数、生命周期桶、托管待销毁、导入释放）全部平稳；`events` 日志 4324 → 27504（约 29 条/帧）无限增长，该用例 ignore 待 P3。
- `tests/p0_perf_baseline.rs`（ignore）：6 pass 图（清除、填充、光栅三角、两次拷贝、读回导出），300 帧去 10 帧预热。

| 构建 | mean | p50 | p95 | 每帧命令缓冲 | 每帧屏障命令/转换 | 每帧 vkQueueSubmit |
| --- | --- | --- | --- | --- | --- | --- |
| release | 0.30 ms | 0.29 ms | 0.41 ms | 3.03 | 5 / 11 | 1 |
| debug | 0.45 ms | 0.44 ms | 0.56 ms | 3.11 | 5 / 11 | 1 |

运行：`cargo test --release -p rhapsody --test p0_perf_baseline -- --ignored --nocapture`。

## 待修缺陷登记（按新标准，原版行为不再是保留理由）

| 缺陷 | 位置 | 归属阶段 |
| --- | --- | --- |
| `resolve_view_mip_range` 截断与等于剩余 mip 被拒 | `src/rg/binding.rs` | 已修复；同时拒绝空范围 |
| cube 初值上传只写 1 个面（按 `array_elements` 而非 6 层） | `src/raw/setup.rs`、`texture_upload.rs` | P4 |
| 合法 BC 格式（BC1_RGBA、BC2、BC4_SNORM、BC6H 等）初值上传被拒（原版块字节表不全） | `src/raw/format_table.rs` | P4 |
| 单批 6 pass 图默认分块过细 | `src/rg/parallel.rs`、`vulkan.rs` | 已改为默认串行单命令；并行录制可显式配置 |
| `src/` 内仍有原版布局单测（约 66 处 `size_of`/`offset_of`）与 `original_*` 辅助 API | 多处 | P1 起随模块重写删除 |

## 同步与队列内核（已接生产路径）

| 模块 | 内容 | 验证 |
| --- | --- | --- |
| `src/error.rs` | 统一 `Error`/`ErrorKind`，`vk::Result` 分类映射 | 2 个单测 |
| `src/sync/mod.rs` | `QueueId`（≤ `MAX_QUEUES`=8）、`SyncPoint`、`SyncSet`（逐队列最大值，`Copy`）、`CompletedValues` | 3 个单测 |
| `src/sync/timeline.rs` | `Timelines<A: TimelineApi>`：每队列提交锁 + 时间线；值仅在提交成功后确认；`poll` 只查有未完成工作的队列并缓存；`wait` 拒绝等待未提交的值；设备丢失粘滞 | 8 个单测 + `tests/sync_timeline_vulkan.rs`（lavapipe 真队列：51 次提交含 1 次失败，设备计数器 = 51） |
| `src/sync/retire.rs` | `Retirement<T>`：按 `SyncSet` 延迟回收；到期对象移交调用方在锁外销毁 | 3 个单测 |
| `src/device/queue_config.rs` | `QueueConfig`/`plan_queues`：专用 compute/DMA 族、同族第二队列、粗粒度 DMA 跳过、呈现约束、`Prefer` 别名/`Require` 报 Unsupported | 7 个单测（单队列、独显、AMD 式、同族多队列、无呈现等布局） |

## P2/P3 接线设计（目标；当前完成范围见阶段表）

### 旧机制 → 新内核

| 旧机制 | 位置 | 由谁替代 | 处置 |
| --- | --- | --- | --- |
| 全局 `VulkanDevice.queue_lock` | `raw/device.rs` 等 7 文件 | `Timelines` 每队列提交锁（`submit`/`with_queue`）；交换链自带锁；分配器自带锁 | 删除 |
| `VulkanDevice.retire_timeline` + `Graveyard` epoch + 末尾空提交 | `raw/graveyard.rs`、`rg/vulkan.rs::submit` | 每队列时间线；`Retirement<Doomed>` | 删除，省掉每次提交的额外空 `SubmitInfo` |
| `submit::DeviceState.next_timeline`（提交前分配、跨队列共享计数） | `raw/submit.rs` | `Timelines::submit` 内分配，成功才确认 | 删除 |
| `SequenceRuntime`（序号、桶、`events: Vec<String>`） | `raw/managed/lifecycle.rs` | 命令记录 `SyncSet`；`Retirement<CommandAllocation>` | 删除（P0 稳定性用例转绿） |
| `SubmissionEngine` + `mirror_submission_state` 双份状态 | `raw/submit.rs`、`rg/vulkan.rs` | 同上 | 删除 |
| `CommandBufferSeqTracker` + `CommandBufferDisposalBuckets` | `raw/command_buffer.rs`、`raw/disposal.rs` | 资源 `last_use: SyncSet`，drop 时进 `Retirement` | 删除 |
| `ManagedResources` dropped 记录 + `take_out_dropped_waste` | `raw/managed/resources.rs` | 同上 | 删除 |
| `VulkanDevice` / `GraphicsDevice` 两层 | `raw/device.rs` | 单一 `Device { ash, caps, allocator, timelines: Timelines<ash::Device>, plan: QueuePlan, retirement: Mutex<Retirement<Doomed>>, caches }` | 合并；`attach_graphics_device` 消失 |

### 资源使用登记（热路径）

- 资源对象内含 `last_use: Mutex<SyncSet>`（或按队列的 `AtomicU64` 数组，P3 按基准选型）。
- 图执行在一次提交成功后得到 `SyncPoint`，对本次提交引用的每个资源 `last_use.insert(point)`；`SyncSet` 是 `Copy` 的 64 字节，无堆分配。
- 资源最后一个句柄 drop 时，把 Vulkan 对象连同 `last_use` 交给 `Device::retire`：已完成则当场销毁，否则进 `Retirement`。
- 每帧 `begin_frame` 调 `Timelines::poll` 一次，`Retirement::collect` 到期对象，锁外销毁。
- `DeviceLost` 时 `drain_all` 直接销毁（不再等待）；`Device` drop 时先 `vkDeviceWaitIdle`、再 `drain_all`、再释放分配器、最后销毁设备。

### 帧提交（P5）

一帧主图 + 展示图合并为一次 `Timelines::submit(graphics, ..)`：

```
wait:   acquire 二值信号量 @ 首次访问阶段（颜色附件为 COLOR_ATTACHMENT_OUTPUT；拷贝/compute 使用对应阶段）
        跨队列依赖的时间线值（P6）
signal: 本队列时间线 = SubmitSignal.value
        present 二值信号量（按交换链图像槽持有，避免复用仍在等待的信号量）
```

随后通过同一队列的 `with_queue(graphics, vkQueuePresentKHR)` 单独加锁呈现。acquire 只持交换链锁，不持任何队列锁。`SUBOPTIMAL` 可继续呈现并标记 stale；`OUT_OF_DATE` 触发重建，实际重建后返回 `Recreated`，零尺寸返回 `Skipped`。提交完成与呈现资源回收分别跟踪；失败补救须核对 acquire 是否已消费及图像布局，不能无条件空提交后 present，详见 `ARCHITECTURE.md` §3.8–3.10。

### 命令池（P3 起步、P7 优化）

每 (帧环槽 × 录制线程 × 队列族) 一个 `VkCommandPool`；帧环槽的 `SyncSet` 完成后整池 `vkResetCommandPool`，取代逐个命令缓冲的回收与 `pool_sequences` 账本。

不在本轮：country-core `RenderHost` 的宿主实现（依赖 `ustr_contract::intern` 恢复）。

## 设计升级实施表（2026-10-16）

逐版细化：见 [DELIVERY_PLAN.md](DELIVERY_PLAN.md) 的 V0–V9、M0–M9 与 Window Lab 验收协议。预期验收与实测结果分别登记。

本表是新增的目标设计与验收约束，**不改变上方阶段的实际完成状态**。详细协议见 [ARCHITECTURE.md](ARCHITECTURE.md) §3.10–3.11、§4.4–4.9。新接口先按阶段落地；默认启用优化前必须验证正确性与实际收益。

| 顺序 / 阶段 | 交付 | 前置与边界 | 验收 |
| --- | --- | --- | --- |
| P2 | `DeviceCaps` / `ExecutionPolicy`、设备代际、分层分配入口 | 复用 `gpu-allocator`，不重写通用分配器；区分支持与已启用能力 | 不支持能力被明确拒绝/按策略选择；跨设备和旧代际句柄不能混用 |
| P3 | 帧 × worker × 队列族池、CPU arena、区间完成登记 | 池独占；全部 GPU 使用完成后 reset；局部缓存受限并低频回流 | 并行录制无共享池竞争；CPU 临时内存与 GPU 可见内存按各自完成条件复用 |
| P3/P4 | `UploadManager` / `ReadbackManager`、预算控制器、有界后台任务 | ticket 管理资源区间；无整设备等待；容量耗尽走背压 | 上传/读回正确，non-coherent 路径有效；饱和与长跑无无界队列/缓存增长 |
| P4 | `GraphDefinition` / `CompiledPlan` / `FrameInstance`、资源版本与子图接口 | 计划不持有具体资源、回调和提交值；实际绑定另行执行 | 缓存开关输出相同，强制哈希碰撞不误命中，键覆盖烘焙输入 |
| P4 | 整图编译缓存、单队列 `TransientMemoryPlan` | 真实内存需求参与放置；先只复用已排序且兼容的瞬态资源 | 无重叠覆盖/重复释放；报告峰值、碎片、分配次数和额外同步，不预设节省比例 |
| P4/P7，测量后 | 子图编译缓存、版本驱动结果缓存 | 跨子图同步与内存放置仍全局协调；业务脏标记由下游提供 | 子图变更仅使相关计划失效；结果依赖完整且保留完成点；统计收益和管理成本 |
| P3 起约束，P5 闭合 | `FrameTransaction` / `AcquiredImageLease` / `PresentTicket` | 每批提交成功即不可逆；呈现资源有独立完成协议 | 部分提交失败、录制失败、present 失败后无在飞池 reset、无资源丢失，下一帧可恢复 |
| P6 | 多队列偏序与内存别名协调 | 不能以 pass 序号判断不重叠；默认保留并行性 | CPU 模型覆盖多队列；真机覆盖跨族；省内存策略报告新增等待和 GPU 关键路径代价 |
| 各阶段 / P7 汇总 | 独立计划检查器、失败注入、有界诊断、性能 A/B | 检查规则独立于编译算法；串行/无别名路径作对照 | 检查访问依赖、所有权、别名与回收；记录 p50/p95/p99、显存、等待和缓存失效原因 |

落地次序：能力/分配基础 → 局部池与预算 → 图计划分层和整图缓存 → 单队列着色 → 子图/结果缓存按测量启用 → 多队列扩展。帧事务与独立检查器随基础改造同步引入，不能在部分提交或别名已经启用后再补。GPU 驱动渲染、mesh shader 与复杂异步调度继续由真机剖析决定。

结果缓存使输出成为持久资源，不再进入普通瞬态着色；多帧在飞的 backing allocation 也必须等待其全部使用完成。所有缓存按字节设置上限，局部池上限还受设备总预算约束。为省内存而添加串行依赖是可选策略，默认策略不牺牲已有并行性。

## 验证口径

在 `tiny-glade/` 下，`CARGO_TARGET_DIR=/tmp/tg-target`、`VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`：

```bash
cargo test -p rhapsody -p shader-compiler -p shader-pipeline [--all-features] --no-fail-fast
cargo clippy -p rhapsody -p shader-compiler -p shader-pipeline --no-deps --all-targets [--all-features] -- -D warnings
cargo build -p tiny-glade -p tg-text --features tg-text/gpu --all-targets
```

每阶段全绿后单独提交。
