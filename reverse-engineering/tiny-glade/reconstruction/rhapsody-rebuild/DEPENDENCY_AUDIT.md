# 依赖审计：骨架 36 个依赖 vs 本工程

- 日期：2026-10-13。起因：`skeleton/B/crates/rhapsody/Cargo.toml` 列了 36 个依赖，本工程只有 8 个，且一部分是"手写等价物"，违反"直接复用原依赖"的规则。
- 骨架清单的性质：它把原锁里所有边都当普通依赖放进去（"inspection union"），**依赖种类、features、平台条件未恢复**，所以 36 不是"必需数"。
- 证据口径：下表"文件数"来自骨架源码注解里引用该 crate 的原模块数（PDB 内联/调用证据），不是源码级用法。

## 一、本轮采纳（原作有、此前我们手写了等价物）

| 依赖 | 版本（取自原锁） | 原作用法（证据） | 本工程做法 | 状态 |
|---|---|---|---|---|
| `parking_lot` | 0.12.1 | 16 个模块 | 全部 `Mutex`/`RwLock` 改用它；删掉约 90 处 `unwrap_or_else(\|e\| e.into_inner())` 与 10 处"锁失效"错误分支（parking_lot 不中毒，这些分支本来就是死代码） | 已做 |
| `append-only-vec` | 0.1.2，git rev `4031d67`（h3r2tic 分叉，与原锁同一 rev） | `ManagedResourceStore<T>::allocate`（RVA `0x1418ae7e0/8a0/9f0/ae0`）内联 `AppendOnlyVec::push`（含 `AtomicPtr::compare_exchange_weak`，即分块扩容），读取走 `Index`/`len`；元素类型 `ManagedResource<T>`，覆盖 Buffer/Texture/Sampler/RenderPass/DescriptorSet/Framebuffer | 槽位表改为 `AppendOnlyVec<Slot<T>>`：`get`/`generation_at` 完全无锁，不再经 `Arc<Mutex<ResourceState>>` | 已做 |
| `arc-swap` | 1.7.1（`weak` feature 为本工程所需，不引入新依赖） | ① `PhysicalDevice::refresh_memory_budget` 用 `ArcSwapAny::{load,store,swap}`，`device_local/host_memory_budget_bytes` 用 `load`；② `ManagedBuffer::dispose`、`ManagedTexture::new` 含 ArcSwap 的构造/析构（字段内容未恢复） | ① 已做：`PhysicalDevice` 持有 `ArcSwapOption<PerformanceInfo>` 快照，getter 只读快照，且设备扩展只枚举一次；② 槽位内的句柄弱引用用 `ArcSwapWeak`（候选，与原 slot 布局无关）。**未做**：managed buffer/texture 内的那个 ArcSwap 字段（不知道放什么） | 部分 |
| `log` | 0.4.27 | 17 个模块 | 接入三处：子分配初始化/失败（此前错误被 `\|_\|` 吞掉）、会话 poisoned、模式启用 | 最小接入 |

## 二、仍未采纳（逐项给理由）

| 依赖 | 文件数 | 为什么没做 | 什么时候该做 |
|---|---:|---|---|
| `puffin` | 10 | 每个 pass 闭包的性能作用域，对分析并行录制有用；但会引入 puffin 依赖树，且原作的作用域名/粒度未恢复 | 接真实 pass、要做 CPU 时间线分析时 |
| `leekspin` | 10 | **原作的本地 crate（诊断飞行记录器），不是性能结构**，细节见第七节。我们没有等价物，也没有在热路径上付它的开销 | 要做崩溃上下文/事件追踪时 |
| `arrayvec` | 6 | 原作在 framebuffer/render_pass/shader/executor/pass_api 用它放小数组；**容量常数未恢复**，随意取常数会引入新的失败模式 | 恢复各处容量上限后 |
| `bumpalo` | 5 | descriptor_set/submit/buffer/binding 的每次调用 arena；属于分配优化，没有测得的热点 | 有 `vkUpdateDescriptorSets` 热点数据时 |
| `anyhow` | 3 | init/physical_device/command_buffer 的错误类型。我们的 `Error` 是公开 API 且被测试断言，原错误 ABI 未恢复 | 恢复错误传播形状后 |
| `backtrace`、`spin_sleep` | 3、1 | 诊断与 submit 内的等待辅助，行为细节未恢复 | 同上 |
| `modular-bitfield` | 1 | `RasterPipelineOpts` 的 4 字节位域（第七节）。我们用 14 个 `Option` 字段的普通结构，**字段集合与原作不同**；位偏移未恢复 | 恢复位偏移后，作为管线键的一部分重写 |
| `ash-window`、`raw-window-handle`、`nvapi-hi`、`known-folders`、`uuid`、`image` | 各 1–2 | 窗口/WSI、厂商 API、路径、图片解码：不在 rhapsody 渲染核心范围 | 做窗口与资源加载时 |

## 三、骨架里在源码注解中找不到使用点的依赖

`byte-slice-cast`、`bytes`、`c_str_macro`、`nohash-hasher`、`num_enum`、`pretty_env_logger`、`thiserror`：没有观察到引用，不采纳。

## 四、并发协议变化（本轮）

```
旧：Arc<Mutex<ResourceState{slots: Vec, available: Vec}>>   get / allocate / retire / generation_at 全部串行
新：Arc<ResourceState{slots: AppendOnlyVec, available: Mutex<Vec<u32>>}>
    get / generation_at：无锁（Index + ArcSwapWeak::load + clone_if_live CAS）
    allocate：空闲链表短暂加锁；新槽位 push 无锁
    retire：dispose_control_kind（CAS）后只对空闲链表加锁
```

安全性依赖的不变量（沿用上一轮控制块协议）：槽位进入空闲链表前 `resource_checksum` 已递增，所以持旧键的 `get` 即使 upgrade 到复用后的新句柄，`clone_if_live` 的校验和比较也会失败。测试：`tests/managed.rs` 新增 2 个（16000 槽位跨线程增长；复用槽位拒绝过期键），变异（去掉校验和检查）会让 4 个测试失败。

## 五、实测（lavapipe 所在 CPU，`cargo bench --bench perf`，仅作参考）

`cpu/managed_get`（多线程 `get` 同一槽位）：1/4/8 线程 = 46 / 162 / 158 ns/op。把旧设计的锁用一把全局 `parking_lot::Mutex` 近似模拟后是 60 / 201 / 347 ns/op。**这是近似**：旧实现用的是 `std::Mutex`，竞争下更慢；也不含 `Vec` 与 `AppendOnlyVec` 的访问差异。剩余的 160 ns 主要是同一句柄的引用计数与 `Arc` 强引用在多核间的缓存行争用，无锁也消不掉。

## 六、没验证的

- 原 `ManagedResource<T>` 的槽位内部布局、空闲链表算法（原作 `0x160` 字节 + 44 桶的结构，疑似 AppendOnlyVec 的分块指针数组，未逐字段对照）。
- 原作 `refresh_memory_budget` 是否按时间节流（`Instant::now` 出现在该函数里，用途未证实）；本工程没有节流。
- 真显卡上的 `VK_EXT_memory_budget` 数值。lavapipe 支持该扩展（实测 `device_local_budget = Some(34221223936)`，即本机内存量级），所以快照路径被执行过，但数值不代表真实显存。

## 七、`leekspin::RingBuffer` 与 `RasterPipelineOpts` 位域（2026-10-13 补查，PE 反汇编）

### 7.1 RingBuffer：飞行记录器，不是性能数据结构

- 布局 `{ptr, capacity, cursor: AtomicUsize}`（`new` @`0x1418c1310`，`append` @`0x1418c1380`，`to_inner` @`0x1418c1430`）。`append(&self, bytes)`：`pos = cursor.fetch_add(len)`（`lock xadd`），`start = pos % capacity`（容量 0 会 panic），长度超过容量会 panic，跨尾部时拆成两段 `memcpy`。无锁、不检查读写竞争；`to_inner` 读光标后 `rotate_left` 得到按时间排序的字节副本。
- 全局唯一一个：`init_logging`（`0xbe6a90`）建 **64 KiB**（`0x10000`）并存入全局，重复初始化会 panic；`dump_log`（`0xbe6b50`）返回 `to_inner`。
- 写入的是**紧凑二进制事件**：无负载事件 1 字节标签（`wait_submit_done` 写 `0x1c`，`immediate_empty_bucket` 写 `0x0e`）；"销毁了 N 个某类资源" 5 字节（`u32` 个数 + 1 字节标签，标签 6–13 对应 `destroy_buffers/images/…`）。`rhapsody::leek::*` 共约 50 个事件函数（`queue_submit`、`begin_rg_pass`、`present_*` …）是这些标签的解码名，`leekspin_dump`/`print_deduplicated_with_count` 负责打印。
- 调用点：`queue_submit`、`wait_submit_done`、`dispose_cb_resource`（7 类资源）、`SequentialDisposal::dispose_in_order`、`immediate_empty_bucket`、`DynamicConstants::allocate_buffer`。
- **性能含义**：它是原作为崩溃转储付出的**开销**（每个事件一次 `lock xadd` 加一次除法加 1–5 字节拷贝），本身不加速任何东西。唯一的并行相关点是所有线程竞争同一个游标缓存行；`begin_rg_pass` 等事件若在并行录制线程里写入，会随线程数增加变慢。**我们没有写这些事件，所以没有付这笔开销，也没有这份崩溃上下文。**
- 我们此前的 `raw/crash.rs::CrashDiagnostics`（`Mutex<VecDeque<String>>`）整个管线里没有调用点，不是它的等价物，**已删除**；若要补，应实现同样的无锁字节环（约 40 行，用 `AtomicU8` 避免数据竞争 UB）。

### 7.2 RasterPipelineOpts：4 字节位域

- `<RasterPipelineOpts as Default>::default`（公共符号，`0x140be6750`）只做 `mov eax, 0x1ffc00d0; ret`：**整个结构是一个 `u32`，默认值 `0x1FFC00D0`**。骨架把该 RVA 标成 `set_polygon_mode_checked`，是符号归属错位。
- 原作字段（modular-bitfield 生成的访问器）：`topology`、`cull_mode`、`polygon_mode`、`blend_mode`、`depth_test_enable`、`depth_write`、`depth_compare_mode`、`stencil_test_enable`、`stencil_compare_op`、`stencil_pass_op`、`stencil_compare_mask`，各有 `set_*`、`set_*_checked`，枚举类字段还有 `*_or_err`。**没有** `front_face`、`samples`、`line_width`，也没有"未设置"概念（取值由默认位决定）。
- 我们的 `RasterPipelineOpts` 是 14 个 `Option` 字段（约 100 字节），多了 3 个原作没有的字段，且用 `Option` 表达"未设置"，**与原作行为不同**。各字段位宽/位偏移未恢复（setter 全部内联进调用方）。
- **性能含义**：4 字节 `Copy` 值省掉的只是构造/拷贝，相对 `vkCreateGraphicsPipelines`（lavapipe 上 21 µs/个，真驱动更久）可忽略。真正可能有意义的是它能否进管线缓存键：我们当前每次 `create_raster_pipeline`（含缓存命中）都构造一个约 70 个 `u64` 的 `Vec` 键（入口名每个字节占一个 `u64`）。同形状的独立复刻测量约 190–270 ns/次，换成"句柄+哈希"的打包键没有稳定收益（SipHash 本身就要 ~100 ns）。**所以目前没有证据表明这里是热点**；只有在原作每帧大量调用该入口时才值得做，而这点没有证据。

## 八、第一条真实 pass：原作 blur shader 经 rg 图执行（2026-10-13）

`tests/real_pass_blur.rs`：用发行 `_blur.cs.hlsl_cs_main_0.bin` 第 0 个模块（`tools/render-passes/self_test/blur_mod0.spv`，5160 字节，原样 SPIR-V，不是我们重编译的）跑完整条路径：`VulkanGraphBackend` 声明资源 → 上传 pass（`$Globals` UBO + 暂存缓冲→RGBA32F 图像）→ 原 shader dispatch（`ShaderLibrary`/`DescriptorSetState`，绑定 set0 的 b13/b14/b15 稀疏描述符）→ 回读 pass → 与 `blur.cs.hlsl` 的 CPU 参考比对。64×64，专用与子分配两种模式都通过。

- **结果**：内部 2116 个像素中位误差 0（位级一致），最大 4.25e-3（sin/cos 实现不同使个别采样点落到相邻纹素）。整幅 4096 个像素 alpha 都为 1（证明 dispatch 覆盖整图；变异把 x 方向少派一组，测试在 (56,0) 失败）。
- **一个发现**：HLSL 里两层循环用浮点累加器当条件。严格 IEEE f32 是外层 31 次、内层 9 次（`t` 第 10 次累加得 1.0000001 > 1.0）；**lavapipe 实际是外层 31、内层 10**。四个候选中只有 (31,10) 的误差落在 1e-2 内，其余偏差 3%–16%。也就是说这类循环的次数**由驱动编译器决定**，测试因此不写死次数，而是在候选里找最优并要求它显著优于次优。真显卡上也可能不同，不能假设。
- **边界**：边缘 9 像素的 rgb 依赖越界 `OpImageFetch`（Vulkan 未定义），不比较。
- **没做**：没有用 dxc 重编译 `blur.cs.hlsl` 对比（本机无 dxc）；没有基于它改基准。三个 pass 的冷启动耗时约 3 ms（含资源创建），没有稳态数据，不能据此判断并行录制的收益。
