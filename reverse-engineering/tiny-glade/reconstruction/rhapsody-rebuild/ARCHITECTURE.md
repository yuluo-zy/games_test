# 架构审视：分层、线程模型、锁清单与重构决定

- 日期：2026-10-13。对象：`rhapsody-rebuild` 候选实现。
- 本文回答三个问题：架构是否合理？哪里需要重构？并行录制怎么落地？每条结论附证据或测量。
- 与原作的关系见各节标注：**还原**（有 PDB/PE 证据）、**候选**（自行设计）。

## 一、分层

```
rg/       渲染图：声明 → 编译 → 录制 → 提交        （executor / graph / recorder / parallel / vulkan）
raw/      Vulkan 封装：设备、资源、命令、同步      （device / buffer / texture / command_buffer / shader …）
raw/managed/  托管生命周期：句柄、控制块、退役桶    （handle / tracking / resources / lifecycle）
```

**合理之处（保留）**
- 执行器与后端分离（`ExecutionChain` + `ExecutionBackend`）：同一套执行逻辑既接 `app.rs` 的 CPU 捕获替身做确定性测试，也接真实 Vulkan 后端。这是整个工程可测试的根基。
- 屏障规划是纯函数（`raw::barrier::pre`）：与录制时机无关，所以能先在串行段规划、后在并行块里发出。
- 图编译是线性复杂度：基准里每 pass 耗时平坦（`cpu/graph_run`），不是瓶颈。

## 二、同名类型的分层（不是重复）

审查时发现多个同名类型。逐个核对后**没有可安全合并的重复**，它们各属一层、各有职责：

| 名字 | 位置 | 职责 | 决定 |
|---|---|---|---|
| `Command`（88 字节） | `raw/managed/lifecycle.rs` | 原作 managed 层命令控制块，逐字节布局 | 保留（ABI） |
| `Command` | `raw/submit.rs` | 提交层的队列命令记录 | 保留 |
| `Command`（句柄+序号+状态） | `rg/recorder.rs` | 录制器轻量视图 | 保留 |
| `Queue`（枚举） | `raw/managed/lifecycle.rs` | 逻辑队列标识 Primary/Aux(i) | 保留 |
| `Queue`（句柄+族+索引） | `raw/device.rs` | 对应原作 PDB 符号 `Queue::update_pending_submissions` / `reclaim_command_buffer_resources` 的名字覆盖 | **保留**：虽无内部引用，删除会让原符号覆盖倒退 |
| `MemoryLocation` | `raw/allocator.rs` | 原 `gpu-allocator` 公共库枚举（含 `Unknown`） | 保留 |
| `MemoryLocation` | `raw/buffer.rs` | 原 `BufferDesc` 私有 ABI 枚举（值 1..=4） | 保留；合并会丢 `Unknown` 或 ABI 数值 |
| `BufferDesc`/`TextureDesc` | `rg/resource_registry.rs` 与 `raw/` | 前者是图描述符的原始字节编码，后者是 Vulkan 创建参数 | 保留 |

真正的问题是**名字混淆**。后续新代码请用模块路径限定（`lifecycle::Command`、`submit::Command`），不要 `use` 裸名。

## 三、并行录制（还原原作）

### 3.1 原作机制（证据：[parallel-recording/FINDINGS.md](../../evidence/render-recovery/parallel-recording/FINDINGS.md)）

`record_batch`（RVA `0x2c1d40`）按**上一帧**各 pass 的 CPU 录制耗时把 batch 切成变长连续块，经
`bevy_tasks::ComputeTaskPool::par_chunk_map_mut` 并行录制，每块一条命令缓冲：

```
budget = clamp(stats.max, stats.total/16, stats.total/2)        // PE 0x1402c21fb..0x1402c2220
块     = 贪心累加上一帧各 pass 耗时（缺失记 0），累计 >= budget 即切块
块数 ≤ 1 时走串行；PGO 统计每帧交换（end_frame 0x140bec400），不跨帧累积
```

### 3.2 本实现的三段式

```
Plan（串行）            Record（并行）                Finalize（串行）
 begin 每块命令           每块：逐 pass                  按块顺序合并耗时/owner
 initial 屏障             发该 pass 的屏障                追加命令、end 非末命令
 资源跟踪                 调回调                          最终批次发导出屏障
 规划每个 pass 的屏障     计时
```

- 串行段与并行段之间只通过**不可变数据**交接：`Vec<Vec<ChunkTransition>>`（已规划屏障）、每块独占的 `&mut [回调]`。
- 屏障构造 `add_transition` 由串行 `transition` 与并行 `VulkanChunkView` 共用，保证两条路径发出的屏障逐条一致。
- **命令池**（原作未知，候选）：`pools[块序号]`，与线程身份无关。Vulkan 要求录制命令的 `VkCommandPool` 外部同步，所以同一 batch 的各块必须来自不同池；回收后命令回到原池。
- **执行器可注入**：`TaskRunner::Inline`（当前线程顺序执行，确定性测试）与 `ComputePool`（原作的全局池，与 Bevy ECS 共用，不另起线程池抢核）。

### 3.3 配置

| 配置 | 行为 |
|---|---|
| `ParallelRecording::SERIAL` | 不分块（`ExecutionChain::new` 的默认，CPU 测试保持确定性） |
| `ParallelRecording::ORIGINAL` | 原分块算法 + 全局池，无门槛（原作行为） |
| `ParallelRecording::ADAPTIVE` | 原分块算法 + 轻帧退回串行（**候选**；`VulkanGraphSession::new` 的默认） |
| `ChunkStrategy::Balanced{..}` | 候选改进：块数上限、未知耗时按均值估计、二分预算最小化最大块 |

### 3.4 实测（lavapipe，64 个 pass，`cargo bench --bench perf`）

| 每个 pass 回调耗时 | 串行 µs/帧 | 并行（原作）µs/帧 | 结论 |
|---:|---:|---:|---|
| 0（空回调） | 117 | 181 | **并行更慢**（固定开销约 60 µs/帧） |
| 2 µs | 243 | 231 | 持平（临界点） |
| 10 µs | 760 | 349 | 并行快 2.2× |
| 50 µs | 3328 | 682 | 并行快 4.9× |

原作没有"轻帧退回串行"的门槛，所以加了 `ADAPTIVE`：上一帧总录制耗时 < 200 µs 就走串行。

**这是人造的回调耗时。** 本工程目前没有任何真实游戏 pass，不能把加速比当作真实收益；它只说明机制正确、临界点在哪。

### 3.4.1 已知的原作算法特性（推断自 PE，未动态验证）

- **第一帧/新 pass**：预算为 0 ⇒ 每个 pass 一块（200 个 pass 就是 200 个命令缓冲）。`ADAPTIVE` 在第一帧走串行规避。
- **重 pass 排在最后**：预算被压到 `total/2`，前面的轻 pass 全被吸进最后一块，**整个 batch 只有 1 块**，无法并行。`Balanced` 策略不受此影响（测试 `heavy_pass_at_the_end_swallows_everything...`）。

### 3.5 一条未强制的硬约束

**块边界不能切断一个 `VkRenderPass` 实例**（主命令缓冲之间不能延续同一个 render pass）。当前每个图 pass 自成完整 render pass 实例，任意边界都可切，所以分块函数接受 `split_allowed` 标记但目前传空。**当图 pass 开始共享 render pass 实例时，必须填这个标记**，否则会产生非法命令流。

## 四、锁清单与改动

| 锁 | 位置 | 评估 | 处理 |
|---|---|---|---|
| 控制块互斥锁 | `managed/handle.rs`、`resources.rs` | **热点**：同一句柄 4 线程并发 `track`，168 ns/次（单线程 20 ns） | **已去锁**：`resource_checksum` 改 `AtomicU32`，`Arc<ControlBlock>`；`clone_if_live` 用 CAS 保持"退役与复活互斥"。实测 40 ns/次（4.2×） |
| 托管槽位表 | `managed/resources.rs` | 原先整个 `ResourceState` 在一把互斥锁里，`get`/`generation_at` 也要拿锁 | **已改**（见 [DEPENDENCY_AUDIT.md](DEPENDENCY_AUDIT.md)）：槽位表换成原作同款 `AppendOnlyVec`，读取无锁；只有空闲链表保留短暂互斥 |
| 管线创建 | `shader.rs` | **串行化所有创建，且命中缓存的查找也被阻塞** | **已改**：按键一个槽位，map 只短暂持锁；同键去重、不同键并行、失败不缓存。4 线程 24.1→11.1 µs/管线 |
| `queue_lock` | `device.rs` | 所有队列的提交与 idle 共用一把 | **未改**（需逐队列锁 + 逐队列完成线，见第六节） |
| `ResourceResolver` 的 `RwLock` | `rg/vulkan.rs` | 回调内按字符串查资源，读锁 + 字符串比较 | **未改**（需改回调接口，见第六节） |
| 分配统计互斥锁 | `allocator.rs` | 已 O(1)，临界区很短 | 保留 |

全部互斥/读写锁已改用 `parking_lot`（原作 16 个模块使用）：不中毒，原先约 90 处中毒绕过与 10 处"锁失效"错误分支已删除。

## 五、并行引入的 API 变化

- 渲染回调改为 `FnMut(PassContext) -> bool + Send`（`RenderCallback`、`pass_builder::render*`）。原因：不同块的回调在任务池线程上执行。`RetireCallback` 在录制后串行执行，不需要 `Send`。
- 唯一受影响的现有代码是 `examples/pass_bindings_gpu.rs`：把回调结果传回外部用了 `Rc<RefCell>`，改为 `Arc<Mutex>`——这正是 `Send` 约束该拦下的写法。
- `VulkanGraphSession::execute_frame` 在回调 panic 展开时把会话标记为 poisoned（`PoisonOnUnwind`）。panic 是展开而不是 `Err`，原先会绕过 poisoned 处理并留下半录制的命令缓冲。串行路径本来就有此问题，并行没有让它变差，但现在两条路径都被保护。

## 六、未做的重构与理由

| 项 | 为什么没做 | 何时该做 |
|---|---|---|
| 拆分 `VulkanGraphBackend`（28 字段上帝对象，1300+ 行） | 并行录制不再依赖它：所需的只读视图（`VulkanChunkView`）与每块命令池已独立出来。拆分是纯整理，收益是可读性，风险是大面积改动 | 当要在它上面再加功能、或改动开始频繁冲突时 |
| 回调直接拿解析好的资源，取代字符串查找 | 需改所有回调写法（`resolver.buffer("name")` → 按声明顺序取）。收益在回调很多时才显现，目前没有真实 pass 可测 | 接入真实 country_core pass 时一并做 |
| 逐队列锁 + 逐队列完成线 + 合并两套延迟释放机制 | 影响面大；本机只有 1 个队列族，无法验证多队列竞争；延迟销毁场的完成线目前只能靠整设备 idle 推进 | 有多队列硬件、或确认销毁场回收不及时成为瓶颈时 |
| 多队列异步计算调度 | **原作也没有**：渲染图录制/提交写死主队列，屏障无所有权转移、图像 EXCLUSIVE（证据 [multi-queue/FINDINGS.md](../../evidence/render-recovery/multi-queue/FINDINGS.md)）。本机无多队列硬件，加它是超出原作的新设计 | 有真机、且愿意引入所有权转移时再量化 |
| 编译结果缓存 | `cpu/graph_run` 每 pass 约 5 µs，200 个 pass 约 1 ms，不是瓶颈 | 真实帧编译占比明显时 |
| 显存子分配器 | 不属于本轮；是上一轮提案 A，预期收益最大 | 独立排期 |

## 七、验证口径

- 串行与并行（Inline、真实任务池）产生的"屏障 + pass 回调"序列逐条相同：`tests/parallel_equivalence.rs`（含 200 次重复、变异验证：故意打乱并行块内顺序，3 个测试失败）。
- GPU 读回逐字节一致，且断言真的走了并行路径（块数 ≥ 2）：`tests/parallel_gpu.rs`。
- 分块公式逐例对照 PE 推导：`tests/parallel_chunks.rs`。
- 并发协议：`tests/managed.rs`（新增 3 个，150 次重复 0 失败）、`tests/pipeline_cache.rs`。
- **未验证**：真显卡、多队列、validation layer 下的并发同步正确性（本机没有）。并行路径的同步正确性目前只由"命令流与串行逐条相同"和 lavapipe 读回保证。
