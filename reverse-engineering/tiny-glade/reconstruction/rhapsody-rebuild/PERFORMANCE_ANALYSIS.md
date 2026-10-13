# 性能分析、已落地优化与后续提案

真实 Intel UHD 630 的测量和与通用引擎的比较已单独记录在 [真实 GPU 性能分析](perf/HARDWARE_ANALYSIS.md)。下文的优化前后绝对耗时仍来自 lavapipe，不能与真 GPU 混比。

- 日期：2026-10-13。对象：`rhapsody-rebuild` 候选实现（**不是**原作机制的镜像；见下文"与原作的关系"）。
- 测量环境：Linux + **lavapipe（CPU 软件 Vulkan）**，`cargo bench`（release 优化）。
  **lavapipe 没有真实 GPU 的并行与带宽特性**，GPU 场景的耗时只反映"CPU 侧驱动调用与同步开销"，
  不代表真实显卡吞吐。真实显卡上 idle 停顿的代价远大于此处数字，需在原 Windows/真机上复测。
- 追踪方式见 [perf/README.md](perf/README.md)。

## 一、方法

1. 先静态审查热点（`device_wait_idle` 调用面、`vkAllocateMemory` 粒度、统计与编译路径的复杂度）。
2. 加**确定性计数器**（`idle_waits`、`queue_submits`、`memory_allocations`），再建基准，**先存优化前基线**。
3. 用临时回溯定位计数器异常的来源（用后即删），再逐项修复并对比。
4. 把关键不变量固化为 `cargo test` 里的护栏（不依赖 bench 才能发现回退）。

## 二、测得的问题与已落地的修复

| # | 问题（证据） | 修复 | 效果 |
|---|---|---|---|
| 1 | **帧循环每帧 2 次整设备 idle**：回溯显示全部来自 `command_handle → VulkanCommandBuffer::reset → wait_idle`。帧节流本应允许多帧在飞，这等于把 CPU/GPU 串行化 | 新增 `unsafe reset_after_completion`；`recycled_commands` 只在对应时间线 `wait` 之后入池（既有的 `release_completed_owners` 已依赖同一约定），帧循环改用它 | `idle_waits` 60→**0**（32 帧）；每帧 96.6→27.4 µs（**3.5×**） |
| 2 | **资源 Drop 逐个 `device_wait_idle`**：Buffer/Texture/ImageView/Sampler/RenderPass/Framebuffer/Semaphore/Pipeline/PipelineLayout/DescriptorSetLayout/DescriptorPool/CommandPool/QueryPool 共 13 类 | 新增**延迟销毁场** [`raw/graveyard.rs`](src/raw/graveyard.rs)：Drop 入场 O(1)；真实提交在 `queue_lock` 内递增 epoch，入场项带 epoch，仅当 `completed_epoch ≥ epoch` 才销毁；无在飞提交时就地销毁；显存以不含设备 Arc 的 `DeferredMemory` 随句柄延后释放（避免引用环）；达到 64 项自动摊销一次 idle；设备 Drop 先排空 | N 次停顿 → 0 次（无在飞）或 ⌈N/64⌉ 次 |
| 3 | **分配统计 O(n²)**：`register()` 每次遍历全部存活记录求堆占用；每次刷新峰值都克隆整份报告 | 堆占用增量维护；峰值快照惰性物化（峰值之后只可能是登记或释放，故在首次释放前/查询时物化，结果与"刷新当刻"逐项一致） | 4096 存活：13866→**74 ns/op（187×）**，且与存活数基本无关 |
| 4 | **宿主读写缓冲每次整设备 idle**（`Buffer::transfer`/首次映射） | 新增 `idle_if_in_flight_locked`：仅当存在未确认完成的提交才 idle，并推进完成线；刚创建的暂存缓冲用 `write_bytes_unsynchronized` 免等 | 纹理上传 idle 16→**0**，117→65 µs/次 |

**图编译**（`cpu/graph_run`）每 pass 耗时平坦（6.9→5.1 µs），**没有**二次复杂度，不需要优化。

## 三、优化前后对照

数据取自 [perf/baseline_before_optimization.json](perf/baseline_before_optimization.json) 与 [perf/baseline.json](perf/baseline.json)。

| 场景 | 优化前 ns/op | 优化后 ns/op | 倍数 | 计数器 |
|---|---:|---:|---:|---|
| cpu/alloc_accounting live=256 | 952 | 62 | 15× | |
| cpu/alloc_accounting live=1024 | 3585 | 70 | 51× | |
| cpu/alloc_accounting live=4096 | 13866 | 74 | 187× | |
| cpu/graph_run passes=512 | 5162 | ≈5100 | 1.0× | |
| gpu/frame_loop passes=1 | 96550 | ≈27400 | 3.5× | idle_waits 60→0 |
| gpu/frame_loop passes=8 | 81509 | ≈28700 | 2.8× | idle_waits 60→0 |
| gpu/texture_upload | 117296 | ≈65200 | 1.8× | idle_waits 16→0 |
| gpu/buffer_create_drop n=256 | 4416 | ≈4200 | 1.0× | memory_allocations 256（见提案 A） |

## 四、与原作的关系（务必区分）

- 延迟销毁场是**候选优化**，不是原作机制。原作 `dispose` 不调任何 `vkDestroy`，而是把句柄推入
  按**序列号**排出的销毁桶（`MECHANISM_DIFF` A4）。本实现按 epoch 判断，粒度更粗（整设备），
  但避免了每次 Drop 一次全设备停顿，且保持"无在飞工作时立即释放"的可观察行为。
- 原作 setup 命令缓冲并入下一次图提交且不等待；候选 `with_setup_cb` 是**同步**版本
  （见 [p4 证据](../../evidence/render-recovery/p4-sync-setup-cb/FINDINGS.md)）。
- 帧循环去掉 idle 不改变原作的帧节流语义（`begin_frame` 排空到 ≤1、`wait_gpu_queue_trim` 只等不弹）。

## 四之二、并行与去锁（第二轮，2026-10-13）

| 项 | 实测 | 说明 |
|---|---|---|
| 并行录制，64 个 pass | 回调 50µs：3328→682 µs/帧（4.9×）；10µs：2.2×；2µs：持平；0µs：**慢 1.5×** | 人造回调耗时；临界点约每 pass 2µs；轻帧由 `ADAPTIVE` 退回串行 |
| managed `track`，同一句柄 4 线程 | 168→40 ns/次（4.2×） | 控制块改原子 |
| 管线创建，64 个键 | 4 线程 24.1→11.1 µs/管线；旧实现线程越多越慢 | 按键槽位；同键去重 |

详见 [ARCHITECTURE.md](ARCHITECTURE.md)。基准新增：`cpu/managed_track`、`gpu/pipeline_create`、`gpu/record_*`；并行块数与场景内计时放在 `info` 字段，**不进入门禁**（它们依赖实测耗时，会抖动）。

## 四之三、子分配与屏障计数（第三轮，2026-10-13）

| 项 | 实测（lavapipe） | 说明 |
|---|---|---|
| 512 个缓冲的真实内存块数 | 512 → **1** | 即提案 A 设想的"降到 ⌈总字节/块大小⌉"，`memory_blocks` 为硬门禁 |
| 每缓冲分配耗时 | 4433 → 751 ns（5.9×） | 软件 Vulkan，绝对值不可外推 |
| `frame_loop` 屏障 | 每帧 1 命令/1 转换（1 个或 8 个 pass 相同） | 8 个 pass 访问同一缓冲同一类型，没有可合并的屏障——印证"没有真实 pass 就无法评估提案 D" |

**重新评估后的结论**：提案 A 实质是**恢复原作行为**（原作本来就子分配），不是新设计；规则要求复用原依赖，所以接 `gpu-allocator` 而不自写。稳态每帧不受益（资源池已复用），收益在加载期耗时、碎片与 `maxMemoryAllocationCount` 上限风险。**默认仍为专用模式**：尚无真显卡验证，特别是非一致性内存的刷新路径从未执行。

## 五、后续提案（A、D 已落地，其余按评估保持未做）

> 状态：**A 已做**（`raw/suballoc.rs`，默认关闭）；**D** 只做了计数器 `barrier_commands`/`barrier_transitions`，合并未做；B/C/E/G 经评估不做，F 取决于目标平台。以下原文保留各提案的设想与风险。

### A. 显存子分配器（已做：接公开 gpu-allocator，默认关闭）

**现状**：每个 Buffer/Texture 一次 `vkAllocateMemory`（`memory_allocations` 与资源数 1:1）。真实驱动的
`maxMemoryAllocationCount` 通常只有 4096，且每次分配代价高、易碎片化。
**方案**：按 `(memory_type, BDA 标志)` 分池，大块（如 64 MiB）+ 空闲链表/TLSF 子分配；`bufferImageGranularity`
对线性/最优资源分区；映射改为"块级持久映射 + 偏移"。**需要改的面**：`MemoryBlock` 要携带 `offset`；
`bind_*_memory(handle, block, offset)`；`map_memory` 改用块指针 + 偏移；`DeferredMemory` 释放变为"归还子区间"。
**度量**：`memory_allocations` 应降到 `⌈总字节/块大小⌉`，由 bench 硬门禁锁住。
**风险**：中高。涉及全部资源创建与映射路径；原作用的是 gpu-allocator 分叉，**来源未确认**，
对外行为（统计口径）要保持，注意 `AllocatorStats` 的"每分配一条记录"口径需另议。

### B. 逐队列锁 + 提交批合并

**现状**：主队列与辅助队列共用一把 `queue_lock`，多队列提交被串行化；整设备 idle 也靠它保护。
**方案**：`Mutex<()>` 数组，每队列一把；`idle` 路径按固定顺序获取全部锁（避免死锁）。同一帧内多个批次已合并为
一次 `vkQueueSubmit`（`queue_submits=1/帧` 已由护栏锁定），保持。
**风险**：中。`queue_lock` 使用点多，需逐一确认保护的是 `VkQueue` 还是更大范围的状态。

### C. 帧内命令缓冲复位粒度

**方案**：按帧 `vkResetCommandPool` 代替逐个 `vkResetCommandBuffer`（驱动侧更省，且可用
`RELEASE_RESOURCES` 控制回收）。依赖第二节修复 1 已建立的"完成后才复用"约定。
**度量**：新增 bench 场景 `frame_loop` 的 `ns/op`，参考项。

### D. 屏障合并与分割屏障

**现状**：按 pass 产生屏障作用域。**方案**：对相邻、阶段/访问可并的作用域合并；写后读跨度大时用
`vkCmdSetEvent2/WaitEvents2` 分割屏障，让无关工作填充间隙；统一迁移到 `synchronization2`。
**度量**：需先加计数器 `barriers_per_frame`（目前没有，**先测再改**）。
**风险**：中。屏障正确性是画面正确性的底线，必须配合 validation layer（本机没有）。

### E. 多队列调度（异步计算）

**现状**：整图走单一队列。**算法提案**：由访问账本得到 pass 依赖 DAG；用**关键路径优先的列表调度**
（HLFET：以到汇点的最长路径作优先级）把无依赖的计算 pass 分到辅助计算队列；跨队列族边界插入
所有权转移与时间线信号量等待；以"跨队列边数 × 代价"作为分配惩罚，避免为省几微秒引入同步开销。
复杂度 O(V+E) 构建 + O(V log V) 调度。**前提**：本机无真实多队列硬件（lavapipe 只有 1 个队列族），
**无法验证收益**。**原作也没有这么做**：渲染图录制写死主队列，屏障无所有权转移、图像 EXCLUSIVE
（[证据](../../evidence/render-recovery/multi-queue/FINDINGS.md)），所以这是超出原作的新设计。建议在真机上
先用 bench 量化"单队列 vs 双队列"再决定是否实现。

### F. 瞬时资源别名（降低显存峰值）

由生命周期区间（首次/末次访问已在编译产物里）做**区间图着色**：按大小降序、起点升序贪心复用同一块显存。
**依赖提案 A**（需要偏移绑定）。收益取决于图的瞬时资源重叠度，需用真实游戏帧数据评估。

### G. 帧在飞数可配置

原作行为是保留最近一帧（`begin_frame` 排空到 ≤1）。若目标平台吞吐受限于 CPU/GPU 抖动，可把上限做成可配置（2–3），
并以 `wait_gpu_queue_trim` 的有界等待做背压。这是**偏离原作**的调优项，默认保持原行为。

## 六、已知限制（诚实边界）

- 所有数值来自**软件 Vulkan**，趋势与计数器可信，绝对耗时不可外推到真显卡。
- 延迟销毁的完成线只由**整设备 idle** 推进；fence/时间线 wait 确认的是"单个提交"，目前不推进完成线，
  所以有在飞提交时每 64 项或 `wait_idle` 才回收一次。若要更及时，需要**逐队列 epoch**（与提案 B 一并做）。
- `DescriptorSet` 的 `free_descriptor_sets` 仍保留 `wait_idle`（需要对池做外部同步，未并入销毁场）。
- swapchain 的 `queue_wait_idle`（重建/Drop）未改动：低频路径，且受 `retire` 语义约束。
- 未覆盖场景：真正的多队列、真实窗口呈现、validation layer 下的同步正确性。
