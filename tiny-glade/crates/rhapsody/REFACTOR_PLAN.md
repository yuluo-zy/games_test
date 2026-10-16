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
- **错误**：公共 API 一律 `Result<T, Error>`；误用在 debug 下 `debug_assert!`、release 下返回 `InvalidUsage`；帧失败回滚本帧，只有 `DeviceLost` 使对象不可用。
- **资源身份**：raw 层 `Handle<T>`，过期句柄可检测；图内 `TextureId`/`BufferId` 区分类型；跨帧资源以 `ResourceName` 为主键，temporal 为按名字的乒乓对，框架负责奇偶与首帧 reset；调试名自动设置。
- **多队列**：`QueueConfig { graphics: 1, async_compute: Off|Prefer|Require, transfer: Off|Prefer|Require }`；`Prefer` 不支持时别名到 graphics；pass 声明 `QueueClass`，编译器插入跨队列时间线同步与 ownership transfer（独占/CONCURRENT 可配置）。
- **性能**：命令池按帧环 × 线程整池复位；描述符每帧池 reset；屏障按 pass 批量 `vkCmdPipelineBarrier2`。

## 阶段

| 阶段 | 内容 | 状态 |
| --- | --- | --- |
| P0 | 删除原版 oracle 测试；1000 帧稳定性测试（P3 前 ignore）；性能基线 | 完成 |
| P1 | 统一错误模型（去 `Result<_, String>` 与原版 panic） | 部分：`src/error.rs` 已落地并全仓迁移；rg/raw 深改半成品存于分支 `wip/p1-partial`（未编译），按用户要求暂停 |
| P2 | 设备合一 + 可配置队列 + 去全局 `queue_lock`（下游同步迁移） | 未开始 |
| P3 | 统一同步/回收：`SyncPoint` + `Retirement`；删 graveyard/SequenceRuntime/SubmissionEngine/销毁桶/events 日志 | 未开始 |
| P4 | 语义化资源身份：`Handle<T>`、`ResourceName`、命名 temporal | 未开始 |
| P5 | 帧控制流闭合：acquire→主图→展示图→present；headless 整帧测试；帧失败回滚 | 未开始 |
| P6 | 图内多队列调度（lavapipe 只能用别名队列验证；真实多族需真机） | 未开始 |
| P7 | 删重复实现与原版注释；性能三项并对比基线；更新文档与 provenance 规则 | 未开始 |

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
| `resolve_view_mip_range` 用 `base as u8` 截断（base=256 通过检查）；`n >= avail` 拒绝"恰好剩余全部 mip" | `src/rg/binding.rs:352` | P1 |
| cube 初值上传只写 1 个面（按 `array_elements` 而非 6 层） | `src/raw/setup.rs`、`texture_upload.rs` | P4 |
| 合法 BC 格式（BC1_RGBA、BC2、BC4_SNORM、BC6H 等）初值上传被拒（原版块字节表不全） | `src/raw/format_table.rs` | P4 |
| 单批 6 pass 图每帧录 3 个命令缓冲：session 强制原版 PGO 分块 | `src/rg/parallel.rs`、`vulkan.rs` | P7（P2 先改成可配置） |
| `src/` 内仍有原版布局单测（约 66 处 `size_of`/`offset_of`）与 `original_*` 辅助 API | 多处 | P1 起随模块重写删除 |

## 已就绪的新内核（P1 期间先行实现，尚未接线）

| 模块 | 内容 | 验证 |
| --- | --- | --- |
| `src/error.rs` | 统一 `Error`/`ErrorKind`，`vk::Result` 分类映射 | 2 个单测 |
| `src/sync/mod.rs` | `QueueId`（≤ `MAX_QUEUES`=8）、`SyncPoint`、`SyncSet`（逐队列最大值，`Copy`）、`CompletedValues` | 3 个单测 |
| `src/sync/timeline.rs` | `Timelines<A: TimelineApi>`：每队列提交锁 + 时间线；值仅在提交成功后确认；`poll` 只查有未完成工作的队列并缓存；`wait` 拒绝等待未提交的值；设备丢失粘滞 | 8 个单测 + `tests/sync_timeline_vulkan.rs`（lavapipe 真队列：51 次提交含 1 次失败，设备计数器 = 51） |
| `src/sync/retire.rs` | `Retirement<T>`：按 `SyncSet` 延迟回收；到期对象移交调用方在锁外销毁 | 3 个单测 |
| `src/device/queue_config.rs` | `QueueConfig`/`plan_queues`：专用 compute/DMA 族、同族第二队列、粗粒度 DMA 跳过、呈现约束、`Prefer` 别名/`Require` 报 Unsupported | 7 个单测（单队列、独显、AMD 式、同族多队列、无呈现等布局） |

## P2/P3 接线设计

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
wait:   acquire 二值信号量 @ COLOR_ATTACHMENT_OUTPUT（仅首个触碰交换链图像的批次）
        跨队列依赖的时间线值（P6）
signal: 本队列时间线 = SubmitSignal.value
        present 二值信号量（按交换链图像槽持有，避免复用仍在等待的信号量）
```

随后在同一队列锁内 `with_queue(graphics, vkQueuePresentKHR)`。acquire 只持交换链锁，不持任何队列锁。`SUBOPTIMAL`/`OUT_OF_DATE` → `FrameOutcome::Recreated`，零尺寸 → `FrameOutcome::Skipped`。

### 命令池（P3 起步、P7 优化）

每 (帧环槽 × 录制线程 × 队列族) 一个 `VkCommandPool`；帧环槽的 `SyncSet` 完成后整池 `vkResetCommandPool`，取代逐个命令缓冲的回收与 `pool_sequences` 账本。

不在本轮：country-core `RenderHost` 的宿主实现（依赖 `ustr_contract::intern` 恢复）。

## 验证口径

在 `tiny-glade/` 下，`CARGO_TARGET_DIR=/tmp/tg-target`、`VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`：

```bash
cargo test -p rhapsody -p shader-compiler -p shader-pipeline [--all-features] --no-fail-fast
cargo clippy -p rhapsody -p shader-compiler -p shader-pipeline --no-deps --all-targets [--all-features] -- -D warnings
cargo build -p tiny-glade -p tg-text --features tg-text/gpu --all-targets
```

每阶段全绿后单独提交。
