# rhapsody 性能优化说明

本文列出当前代码里已经实现的优化：做了什么、为什么有效、在哪里、怎么开关、实测了多少。文档只写已落地的内容；设计阶段的路线图（bindless 全面启用、GPU 驱动渲染、移动端分块渲染等）见 [ARCHITECTURE.md](ARCHITECTURE.md) §4 与 §8。验收状态与已知缺口见 [REFACTOR_PROGRESS.md](REFACTOR_PROGRESS.md)。

## 先读这几条

- **实测数字只有"设备与内存"一组**，在 lavapipe（软件光栅，单队列，只有一致性内存）上用 release 构建测得。它只能说明 CPU 侧的复杂度和开销，**绝对耗时不能外推到真显卡**。其余优化给出的是机制和复杂度，没有量化。
- **多队列族传输、非一致性内存、独占全屏**没有在真硬件上跑过。标"待真机验证"的条目，机制正确性靠单元测试和模拟状态证明。
- 本机没有 Vulkan 验证层，所有屏障只靠单元测试和 lavapipe 的执行结果证明。

## 一、实测数据（lavapipe，release，各跑 3 次）

| 场景 | 优化前 | 优化后 |
| --- | --- | --- |
| 分配一个 4 KiB 缓冲区 | 74–96 µs | 3.5–4.2 µs |
| 存活 4000 个缓冲区时调用一次 `Device::poll` | 8.6–10 µs | 15–18 ns |
| 每个缓冲区的释放加回收 | 18–27 µs | 1.1–1.5 µs |
| 写 64 B（`write_bytes`） | 530–690 ns | 470–510 ns |

来源：对应修复的提交说明与当时的测试探针（`tests/core_perf.rs`，手动运行，默认忽略）。

## 二、内存与分配

| 优化 | 做法 | 位置 |
| --- | --- | --- |
| 子分配器增量计数 | 每种内存类型一把锁；已保留字节、块数、分配数、峰值都在分配/释放时增量更新，不再调用 `gpu_allocator::generate_report()`（它每次都遍历全部存活分配） | `src/raw/suballoc.rs` |
| 完成状态 O(已完成) | 在飞的分配按队列、按时间线值排序；完成值没前进时直接返回，不加锁。预算动作读原子变量 | `src/raw/allocator.rs` |
| 主机与显存分开记账 | 主机元数据超限返回 `OutOfHostMemory`，不再误报显存不足 | `src/device/memory.rs` |
| 跟随驱动预算 | 启用 `VK_EXT_memory_budget` 时按堆更新预算（创建设备时与每个维护周期各一次），并强制 `maxMemoryAllocationCount` 上限 | `src/device/budget.rs` |
| 预算紧张时驱逐空闲缓存 | `Evictable` 注册机制：分配前先释放 GPU 已用完的空闲项，再重试一次，再退回专用分配。已接入：传输暂存槽、描述符池环、异步管线编译缓存、渲染图资源池与别名集 | `src/device/` 的驱逐注册表，`src/rg/graph_caches.rs` |
| 瞬态内存别名 | 生命周期不重叠的瞬态资源共用一块内存；别名集按计划缓存，跨帧复用，命中缓存时不调用 `vkAllocateMemory`。借给 GPU 的别名集归还前确认已用完。默认开启 | `src/rg/transient_resources.rs` |
| 线性/最优资源隔离 | 线性 tiling 图像按 `bufferImageGranularity` 与最优资源隔开 | `src/raw/allocator.rs` |

## 三、同步、完成与回收

| 优化 | 做法 | 位置 |
| --- | --- | --- |
| 每队列时间线信号量 | 完成状态由时间线值表示，没有全局队列锁；每个队列有自己的提交锁 | `src/sync/` |
| 缓存的完成快照 | `Device::completed()` 返回缓存值，不调用驱动；热路径用它，只有需要新鲜值的调用才 `poll`（`vkGetSemaphoreCounterValue`） | `src/sync/timeline.rs` |
| 有序退役队列 | 退役改成按队列、按时间线值排序的 FIFO，只处理已完成的前缀，已完成的对象不会被未完成的卡住。入队用 `try_reserve`，失败时同步销毁而不是 abort | `src/device/lifetime.rs` |
| 队列锁统计按需开启 | 默认不做每次加锁的计时与计数 | `src/sync/timeline.rs` |
| sync2 加载器缓存 | 创建设备时加载一次，不再每次提交重新查找 | `src/device/queues.rs` |
| 精确的跨队列等待阶段 | 规划器证明过的跨队列依赖，按"首次使用阶段"等待，不再一律 `ALL_COMMANDS`；不透明依赖才保守处理 | `src/sync/mod.rs`（`StagedDependencies`），`src/device/submission.rs` |
| 屏障合批 | 屏障统一由一个构建点生成；同一源阶段的 raw 访问屏障合并成批，调用次数减少 | `src/raw/barrier.rs`（`RangeBarrier`、`Device::issue_barriers`） |

## 四、渲染图

| 优化 | 做法 | 位置 |
| --- | --- | --- |
| 计划缓存 | 图结构不变时直接复用已编译的计划，命中时不重新分析、不重新校验。缓存键包含队列调度、设备代际、策略、共享方式、访问声明等 | `src/rg/` |
| 访问分析 O(n) | `analyze_passes` 只保留每个资源的访问前沿（最近的写者与其后的读者），不再保留完整历史；访问语义用直接表查找 | `src/rg/plan.rs` |
| 计划校验器只在需要时运行 | 约 3300 行的独立校验器只在 debug 构建、`validation` 特性或运行时开关 `set_plan_validation` 打开时运行，release 默认不跑 | `src/rg/plan_check/` |
| 自适应并行录制 | 没有上一帧统计或帧很轻时走串行；否则按上一帧各 pass 的耗时均衡分块，用 `bevy_tasks` 并行录制。每块使用独立命令池。设备默认策略为自适应 | `src/rg/parallel.rs` |
| 录制路径零额外分配 | 分块录制的暂存区由执行器持有并跨帧复用；导出资源检查改用位图；单块批次直接复用第一条命令，不再多开一条 | `src/rg/executor.rs` |
| 获取信号量只挡必要的批次 | 在第一个使用交换链图像的 pass 前强制切批次边界，前面不用图像的批次不被获取信号量阻塞 | `RenderGraph::gate_first_use` |
| 先提交主图再获取图像 | 主图不依赖交换链图像，先提交；CPU 等 vsync 时 GPU 不空闲。跳帧时主图照常执行 | `src/rg/frame_loop.rs` |
| 无多余的所有权转移 | 内容被丢弃的资源（首次使用声明丢弃）视为无主，不做队列族所有权转移 | `src/resource/range_state.rs` |
| 分离的深度/模板布局 | 设备支持 `separateDepthStencilLayouts` 时启用，规划器按方面输出布局，避免不必要的整体布局转换 | `src/rg/sync_plan.rs` |

## 五、上传与传输

| 优化 | 做法 | 位置 |
| --- | --- | --- |
| 批量异步纹理上传 | 多张纹理合并为一次提交，数据只写一次暂存区，不阻塞 CPU，同步点登记在资源的跟踪状态里，屏障由状态推导 | `src/transfer/` 的 `upload_textures` |
| 传输屏障合并 | 每个任务前后各一批屏障（而不是每个分段两次 `ALL_COMMANDS`）；所有分段一次性写入暂存区，只 flush 一次 | `src/transfer.rs` |
| 专用传输队列 | 配置了独立传输队列时，上传在该队列上进行：同族只靠时间线等待，异族做所有权的释放与获取，`CONCURRENT` 图像不转移（**待真机验证**） | `src/transfer/record.rs` |
| 跨队列族获取合并 | 上传的 acquire 不再每个任务单独提交，而是登记到设备，由下一个图批次合并成一次屏障批次；没有图提交时用 `flush_acquires()` 最多提交一次 | `src/device/pending_acquire.rs` |
| 可复用的 setup 上下文 | 一次性命令不再每次新建命令池，池放在设备里复用，用完按完成点回收 | `src/device/` 的 setup 上下文 |
| 动态常量 flush 合并 | 非一致性内存上，常量页持久映射，脏区间按原子对齐合并，每次提交前统一 flush 一次；一致性内存从不 flush（**非一致性路径待真机验证**） | `src/raw/dynamic_constants.rs` |
| 多段写入 | `Buffer::write_segments` 一次完成检查并只 flush 一次 | `src/raw/buffer.rs` |

## 六、描述符与管线

| 优化 | 做法 | 位置 |
| --- | --- | --- |
| bindless 槽位退役队列 | 退役槽位进每个表自己的队列，只处理已完成的；创建描述符不再全表扫描；`stats()` 是 O(1)；limits 在创建堆时缓存 | `src/bindless.rs` |
| 可增长的描述符池环 | 池满时自动增长；瞬态池整池重置，持久池逐个 set 释放 | `src/raw/descriptor_pool_ring.rs` |
| 异步管线编译与磁盘缓存 | 管线编译在后台完成，按完整键去重；磁盘缓存校验头部与设备标识，原子写入 | `src/pipeline.rs` |
| 管线缓存完整键 | 键覆盖着色器、布局、拓扑、混合、深度模板、动态渲染格式等全部状态，按全字比较，不会因哈希碰撞用错管线 | `src/raw/shader/` |

## 七、呈现

| 优化 | 做法 | 位置 |
| --- | --- | --- |
| WSI 调用不持状态锁 | 等待 fence、获取图像、present 期间不持有状态写锁，窗口线程的查询不会被卡住 | `src/raw/swapchain.rs` |
| SUBOPTIMAL 不无谓重建 | 只有 surface 的尺寸或旋转确实变化才重建交换链 | 同上 |
| 回收维护每帧一次 | WSI 回收与全屏维护每帧只做一次，不再每帧三次 | 同上 |
| SurfaceLost 廉价失败 | 粘滞状态，后续帧直接返回，不再每帧重试原生调用；`replace_surface` 换上新 surface | 同上 |
| 被拒绝的 present 不等待 fence | 只轮询，不阻塞（避免某些驱动上不触发 fence 导致 3 秒卡顿） | 同上 |

## 八、怎么开关与调参

| 项 | 方式 |
| --- | --- |
| 瞬态别名 | 默认开启；`VulkanGraphSession` 上的 `set_transient_aliasing(false)` 关闭 |
| 并行录制策略 | `RecordingPolicy::Serial` / `Adaptive`（设备默认）；渲染图会话里可用 `EVERY_PASS` 强制并行（测试用） |
| 计划校验器 | debug 构建默认开；release 用 `validation` 特性或 `rg::set_plan_validation` |
| 子分配器 | `suballocation` 特性（默认开）；运行时用环境变量 `RHAPSODY_ALLOCATION=dedicated` 或 `set_default_allocation_mode` 切到每资源专用分配 |
| 主机内存预算 | `DeviceConfig::host_memory_budget_bytes`（默认 512 MiB） |
| 锁统计 | `Timelines::set_lock_statistics`，默认关 |
| 具名缓冲零初始化 | 默认内容未定义（省掉创建时的上传）；需要零值用 `named_buffer_zeroed` |

## 九、复现与验证

```bash
export PATH=$HOME/.cargo/bin:$PATH
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json   # lavapipe

cd tiny-glade
cargo test -p rhapsody --all-features --no-fail-fast
cargo test -p rhapsody --no-default-features --lib
cargo clippy -p rhapsody --no-deps --all-targets --all-features -- -D warnings
```

性能探针 `tests/core_perf.rs` 默认忽略，需要手动用 release 运行（`cargo test -p rhapsody --release --test core_perf -- --ignored --nocapture`）。

## 十、尚未做、需要真机才有意义的

- 在真显卡上测量上面各项的实际收益，特别是渲染图、上传、呈现三组（目前只有机制，没有量化）。
- 带独立传输/计算队列族的 GPU 上验证跨族传输、异步 compute。
- 非一致性内存的 flush 路径、独占全屏、真驱动的 `vkReleaseSwapchainImagesEXT` 释放路径。
- 移动端分块渲染的 pass 合并、load/store 自动推导、惰性分配附件（只有设计，见 ARCHITECTURE.md §8）。
