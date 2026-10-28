# rhapsody 性能优化说明

本文列出当前代码中已实现的优化：机制、位置、开关，以及已有的测量记录。尚未实现的设计见 [ARCHITECTURE.md](ARCHITECTURE.md) §4 与 §8；验收状态见 [REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md)。

## 先读这几条

- 测量只有两组，且都不是目标硬件：设备与内存在 lavapipe（软件 Vulkan，单队列，只有一致性内存）上测得；渲染图缓存在 Apple M3 + MoltenVK 上测得。二者都是 release 构建。绝对耗时不能外推到真显卡。
- 其余条目只给出机制与位置，没有量化数据。
- 多队列族传输、非一致性内存、独占全屏没有在真硬件上运行。标注"待真机验证"的条目，正确性依赖单元测试与模拟状态。
- 本机记录中没有 Vulkan 验证层的运行结果。屏障正确性依赖单元测试与 lavapipe 的执行结果。

## 一、测量记录

### 1. 设备与内存（lavapipe，release）

来源：[REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md) 中的性能条目（lavapipe 量级参考）。

| 场景 | 优化前 | 优化后 |
| --- | --- | --- |
| 分配一个 4 KiB 缓冲区 | 74–96 µs | 3.5–4.2 µs |
| 存活 4000 个缓冲区时调用一次 `Device::poll` | 8.6–10 µs | 15–18 ns |
| 每个缓冲区的释放加回收 | 18–27 µs | 1.1–1.5 µs |

### 2. 渲染图与缓存（Apple M3 + MoltenVK 1.4.2，release）

来源：[REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md) 中的 `render_perf_throughput` 记录。该入口为离屏路径（`present_mode` 为 `offscreen`），不是窗口帧率。

- 缓存开启：headless 3 次 × 3000 帧为 3135 / 2982 / 1823 FPS；帧 p50 231 / 225 / 348 µs；约 66 次 CPU 分配 / 帧。
- 缓存关闭（plan / subgraph / result 缓存一起关）：2745 / 3213 / 2979 FPS；约 734 次 CPU 分配 / 帧；compile p50 约 39–49 µs，开启时约 1.9–4.6 µs。
- FPS 三次结果离散较大，不能当作稳定的加速比。较可靠的对照是每帧分配次数与 compile p50。
- 同一记录中 GPU 读回抽样全部通过。本次编写没有重新运行这些测量。

## 二、内存与分配

| 优化 | 做法 | 位置 |
| --- | --- | --- |
| 子分配器增量计数 | 已保留字节、块数、分配数与峰值在分配、释放时增量维护；源码中已无 `generate_report()` 调用，不再每次遍历全部存活分配 | `src/accounting/stats/`、`src/device/suballoc/mod.rs` |
| 主机与显存分开记账 | 主机元数据超限返回 `OutOfHostMemory`，不计入显存；主机预算由 `DeviceConfig::host_memory_budget_bytes` 控制 | `src/accounting/budget/`、`src/error/mod.rs`、`src/device/init/mod.rs` |
| 跟随驱动预算 | 启用 `VK_EXT_memory_budget` 时按堆应用预算，并对 `maxMemoryAllocationCount` 做记账 | `src/device/memory/mod.rs`、`src/accounting/stats/budget.rs` |
| 预算紧张时驱逐空闲缓存 | `Evictable` 注册表：预算紧张时（`evict_idle_caches_reclaim`）各缓存释放已完成的空闲项。已注册：传输暂存槽（`transfer/enqueue.rs`、`transfer/staging.rs`）、描述符池环（`command/descriptor_set/pool_ring.rs`）、异步管线缓存（`pipeline/compiler.rs`）、渲染图资源池与别名集（`rg/vulkan/backend/mod.rs`、`rg/graph_caches.rs`） | `src/accounting/budget/mod.rs`（`Evictable`）、`src/device/memory/mod.rs`（`register_evictable`） |
| 瞬态内存别名 | 生命周期不重叠的瞬态资源共用一块内存；别名集按计划缓存，整组借给单帧，帧退役后复用。默认开启 | `src/rg/transient_resources/`、`src/rg/vulkan/backend/mod.rs` |
| 线性与最优资源隔离 | 分配请求携带 `linear` 标记；`bufferImageGranularity` 的处理由 `gpu-allocator` 完成（源码注释所述，本次未逐行复核） | `src/device/suballoc/mod.rs` |

## 三、同步、完成与回收

| 优化 | 做法 | 位置 |
| --- | --- | --- |
| 每队列时间线 | 完成状态由时间线值表示；每个队列有自己的提交锁，没有全局队列锁 | `src/sync/timeline/`、`src/device/submission/mod.rs` |
| 缓存的完成快照 | `Device::completed()` 返回缓存值，不访问驱动（过期值只会偏保守）；需要新鲜值时用 `Device::poll()` | `src/device/submission/mod.rs`、`src/sync/timeline/mod.rs` |
| 退役节点预留 | 退役节点在原生对象创建前预留，入队与跨队列移动只转移已有所有者；原生销毁在所有队列锁之外进行 | `src/retire/mod.rs`、`src/device/lifetime.rs` |
| 队列锁统计按需开启 | `Timelines::set_lock_statistics`，默认关闭 | `src/sync/timeline/mod.rs` |
| sync2 加载器 | 创建设备时加载一次，之后提交时直接使用 | `src/device/native/mod.rs` |
| 精确的跨队列等待阶段 | `StagedDependencies` 记录规划器证明过的依赖，按使用阶段等待；无法证明消费阶段的依赖才保守地使用 `ALL_COMMANDS` | `src/sync/mod.rs`、`src/device/submission/mod.rs` |
| 屏障统一发射 | 屏障由 `Device::issue_barriers` 统一发射，基于 `RangeBarrier`（合并细节本次未逐行复核） | `src/command/device_ext.rs`、`src/command/barrier/mod.rs` |

## 四、渲染图

| 优化 | 做法 | 位置 |
| --- | --- | --- |
| 计划缓存 | 图结构不变时复用已编译的计划；缓存有界（128 项、16 MiB），命中时不重新分析与校验。缓存键的具体字段见执行器代码 | `src/rg/executor/mod.rs`、`src/rg/executor/entry.rs`、`src/rg/subgraph.rs` |
| 生产者前沿分析 | `analyze_passes` 为每个资源保留当前生产者前沿，而不是完整访问历史；访问语义用直接表查找。复杂度本次未复核 | `src/rg/plan/analysis.rs` |
| 计划校验器按需运行 | 独立校验器（`plan_check/`，含测试约 4100 行）在 debug 构建、`validation` 特性或 `rg::set_plan_validation(true)` 时运行；release 默认不运行 | `src/rg/plan_check/`、`src/rg/mod.rs` |
| 自适应并行录制 | 进入线程池有固定成本（常量取自 `tests/parallel_dispatch_perf.rs`：空任务约 27 µs，之后每块约 4 µs）。`ChunkStrategy::Adaptive` 只在预计收益超过成本时切块，最多 8 块；块成本取上一帧各 pass 的耗时。多块时经 `bevy_tasks::ComputeTaskPool` 录制，单块在调用线程内联录制 | `src/rg/parallel/mod.rs`、`src/rg/executor/` |
| PGO 统计 | 记录各 pass 的上一帧耗时，作为分块权重的输入 | `src/rg/executor/pgo.rs`、`src/rg/parallel/mod.rs` |
| 获取只挡必要的批次 | `RenderGraph::gate_first_use` 在第一个使用交换链图像的 pass 前切出批次边界，之前的批次不被获取信号量阻塞 | `src/rg/graph/declare.rs` |
| 先提交主图再获取图像 | 主图不依赖交换链图像，先提交；CPU 等待 vsync 时 GPU 不空闲。跳帧时主图照常执行 | `src/rg/frame_loop/mod.rs` |
| 丢弃内容不做所有权转移 | 内容被丢弃的资源视为无主，不做队列族所有权转移（`discard_idle_contents`） | `src/tracking/range_state/table.rs` |
| 分离的深度与模板布局 | 设备支持 `separateDepthStencilLayouts` 时，规划器按方面输出布局，避免整体布局转换 | `src/rg/sync_plan/mod.rs`、`src/device/caps/mod.rs` |

## 五、上传与传输

| 优化 | 做法 | 位置 |
| --- | --- | --- |
| 批量异步纹理上传 | `TransferManager::upload_textures` 把多张纹理合并为一次提交，同步点登记在资源跟踪状态中，屏障由状态推导 | `src/transfer/flush.rs`、`src/transfer/mod.rs` |
| 上传合并与脏区 | `enqueue_upload` 对同一目标的脏区做合并；按 epoch 取消排队中的上传 | `src/transfer/enqueue.rs`、`src/transfer/flush.rs` |
| 专用传输队列（待真机验证） | 配置了独立传输队列时上传在该队列进行：同族只靠时间线等待，跨族做所有权释放与获取 | `src/transfer/route.rs`、`src/transfer/record/` |
| 跨族获取合并 | 传输队列登记的获取不逐个提交，而是由下一个图批次合并成一次屏障批次；没有图提交时 `flush_acquires()` 最多提交一次 | `src/command/pending_acquire/mod.rs`、`src/transfer/flush.rs` |
| 可复用的 setup 命令池 | 一次性上传命令使用的池放在设备里复用，池内命令缓冲在其时间线点完成后回收；每个上下文保留的空闲池有上限 | `src/transfer/setup.rs` |
| 动态常量延迟 flush | 非一致性内存上的脏区间在下一次提交前统一 `flush` 一次；一致性内存从不 flush（非一致性路径待真机验证） | `src/resource/dynamic_constants/mod.rs` |
| 多段写入 | `Buffer::write_segments` 一次完成检查，只 flush 一次 | `src/resource/buffer/write.rs` |

## 六、描述符与管线

| 优化 | 做法 | 位置 |
| --- | --- | --- |
| bindless 不可变租约 | 修改资源会分配新槽位而不是原地改写；槽位在所有已登记的 GPU 使用完成后才回到空闲列表 | `src/bindless/mod.rs`、`src/bindless/slots.rs` |
| 可增长的描述符池环 | 池满时增加新池；瞬态池整池重置，持久池逐个释放 set | `src/command/descriptor_set/pool_ring.rs` |
| 异步管线编译 | 管线在后台编译；帧内只做非阻塞查询（`PipelineTicket::try_get`），预热时才显式等待 | `src/pipeline/compiler.rs`、`src/pipeline/request.rs` |
| 磁盘管线缓存 | 缓存文件校验头部与设备标识，写入采用原子替换 | `src/pipeline/disk.rs` |
| 管线缓存的完整键 | 键覆盖着色器、布局与渲染状态；字段清单见 `src/pipeline/shader/pipeline_key.rs`（本次未逐项复核） | `src/pipeline/shader/` |
| 热替换的帧边界发布 | `HotPipeline` 在帧边界核验 bindless ABI 与 shader 接口后才发布；不兼容时保留旧管线 | `src/pipeline/hot.rs` |

## 七、呈现

| 优化 | 做法 | 位置 |
| --- | --- | --- |
| WSI 调用不持状态锁 | `inner` 只保存记账信息，不在阻塞等待或原生 WSI 调用期间持有；查询尺寸、焦点不会等待 acquire | `src/present/swapchain/mod.rs`、`src/present/swapchain/README.md` |
| SUBOPTIMAL 不无谓重建 | 只有 surface 的尺寸或变换确实变化时才标记交换链过期 | `src/present/swapchain/` |
| SurfaceLost 粘滞 | 之后的 acquire 直接返回 `SurfaceLost`，不再逐帧调用原生接口；`replace_surface` 换上新 surface 后恢复 | `src/present/swapchain/acquire.rs`、`src/present/swapchain/recreate.rs` |

## 八、开关与调参

| 项 | 方式 | 默认 |
| --- | --- | --- |
| 瞬态别名 | `VulkanGraphBackend::set_transient_aliasing(false)` | 开 |
| 并行录制策略 | `ParallelRecording::PARALLEL`（自适应）、`ALWAYS_PARALLEL`（每批次都进线程池）、`EVERY_PASS`（每个 pass 一块，测试与基准用）；执行器字段 `parallel` 接受其中之一 | `PARALLEL` |
| 计划校验器 | `rg::set_plan_validation(bool)`；`validation` 特性使 release 也可用 | debug 开，release 关 |
| 子分配 | `suballocation` 特性；运行时用 `DeviceConfig::allocation`（`AllocationStrategy::{Suballocated, Dedicated}`）选择。Window Lab 的 `--allocation suballocated\|dedicated` 即此开关 | `suballocated` |
| 设备内存预算 | `DeviceConfig::memory_budget_bytes` | 1 GiB |
| 主机元数据预算 | `DeviceConfig::host_memory_budget_bytes`（0 表示不限） | 512 MiB |
| 队列锁统计 | `Timelines::set_lock_statistics(bool)` | 关 |
| 命名缓冲零初始化 | `named_buffer` 的内容未定义；需要零值时用 `named_buffer_zeroed` | 未定义 |
| GPU 计时 | `VulkanGraphBackend::configure_gpu_timing`；Window Lab 为 `--gpu-timing`、`--gpu-pass-timing` | 关 |

以下名称在当前源码中不存在，不要使用：环境变量 `RHAPSODY_ALLOCATION`、`set_default_allocation_mode`、`DeviceConfig::submission_api`、`ParallelRecording::ADAPTIVE`。

## 九、复现与验证

```bash
cd tiny-glade
cargo test -p rhapsody --all-features --no-fail-fast
cargo test -p rhapsody --no-default-features --lib
cargo clippy -p rhapsody --no-deps --all-targets --all-features -- -D warnings
```

性能探针默认忽略，需要手动在 release 下运行：

```bash
# 设备与内存探针
cargo test -p rhapsody --release --test core_perf -- --ignored --nocapture
# 并行调度常量探针
cargo test -p rhapsody --release --test parallel_dispatch_perf -- --ignored --nocapture
# 渲染框架：预生成参考 -> 严格验收 -> 吞吐（吞吐要求先有通过的 acceptance.json）
cargo test -p rhapsody --release --test p0_perf_baseline -- --ignored --nocapture render_perf_prepare
cargo test -p rhapsody --release --test p0_perf_baseline -- --ignored --nocapture render_perf_strict_acceptance
cargo test -p rhapsody --release --test p0_perf_baseline -- --ignored --nocapture render_perf_throughput
```

`render_perf_*` 的参数可用 `RHAPSODY_PERF_WARMUP`（默认 300）、`RHAPSODY_PERF_SAMPLES`（默认 3000）、`RHAPSODY_PERF_REPEATS`（默认 3）、`RHAPSODY_PERF_FRAMES_IN_FLIGHT`（1–3）、`RHAPSODY_PERF_CACHE=on|off` 覆盖。低于正式门槛的参数会被拒绝，冒烟运行须设 `RHAPSODY_PERF_ALLOW_SHORT=1`。输出写入 `target/render-perf/`。

## 十、尚未测量或需真机验证

- 在真显卡上测量上面各项的实际收益，尤其是渲染图、上传与呈现三组。目前只有第 1、2 节的两组数字。
- 带独立传输或计算队列族的 GPU 上的跨族传输与异步计算。
- 非一致性内存的 flush 路径、独占全屏、驱动的 `vkReleaseSwapchainImagesEXT` 路径。
- 移动端分块渲染的 pass 合并、load/store 自动推导、惰性分配附件。这些目前只是设计，尚未实现，也没有对应的代码。
