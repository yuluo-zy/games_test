# 候选实现 vs 逆向机制：差异分析

- 日期：2026-10-12（**第二次修订**，对照当前源码逐文件核对）。
- 对照对象：候选实现 `reconstruction/rhapsody-rebuild/src/**`（**能编译、能跑**） ↔ 本工程逆向出的原作机制（见 [SYNTHESIS.md](evidence/render-recovery/producer-closure/SYNTHESIS.md) / [ROUND3_FINDINGS.md](evidence/render-recovery/producer-closure/ROUND3_FINDINGS.md)）。
- **回答的问题：差别大吗？**

---

## 〇、本次修订说明（重要）

初版列了 16 处差异（8 A 类 + 7 B 类 + 1 C 类）。本次逐文件核对后发现：**候选在初版之后被大幅修正，以下条目已不再成立**：

- ~~"`managed/swapchain.rs` 仅 13 行 stub"~~ → raw 层已真实现 794 行
- ~~"屏障用 write 位近似"~~ → `barrier.rs::concurrent()` 已用 `wrapping_sub(0x28) < 0x10`
- ~~"无 `SynchronizationScope`"~~ → 已有三向量 + src=1/dst=0x2000
- ~~"直接 `drop(Arc)` 释放"~~ → managed 层已改 `enqueue_retirement` 延迟入桶
- ~~"`BTreeMap<domain,seq>` 跟踪"~~ → `ControlBlock` 是单 `last_sequence`，`track_on` 拒绝 domain≠0
- ~~"DescriptorSetState 内容寻址"~~ → 已改空闲池复用 + 池耗尽重试
- ~~"7 个独立 Manager"~~ → tag 表就是 7 类枚举，共享 `SharedResources.dropped` 桶
- ~~"generation 递增在 slot"~~ → 已在 `dispose_control_kind` 里对 inner 递增

**结论因此从"差别不小"修正为"差别在显著收窄，且收窄方向是对的"。**

---

## 〇之二、第三次修订（2026-10-13，P4 取证 + 实现后）

| 项 | 现状 |
|---|---|
| A1 帧边界 | **已补**：`rg/vulkan.rs` 的 `begin_frame`/`end_frame`/`wait_gpu_queue_trim` |
| A2 同步来源 | **判断修正**：真实后端的内部时间线已由提交生产者管理（`check_synchronization` 拒绝把队列内部时间线作为外部输入）；`GraphBatchSync` 只承载**外部**信号量。PE 取证见 [p4-sync-setup-cb](evidence/render-recovery/p4-sync-setup-cb/FINDINGS.md)：原 `queue_submit` 为 `(device, Option<queue>) -> {queue, seq}`，设备级计数器 `device+0x7f8` 一次预留连续区间。剩余偏差在于签名/入口形态，不是"同步被外部注入" |
| A3 `BufferDesc` | **已对齐** 16 字节与 `memory_location` 枚举 |
| A4 raw 释放 | **改为候选优化**：延迟销毁场（epoch），去掉 13 类 Drop 的整设备 idle；仍非原作序列号桶 |
| A5 swapchain retire | **有意偏差**，见 MODULE_STATUS |
| B3 `Access.write` | **已删除** |
| C1 `ray_tracing.rs` | **已删除** |
| C17/C18 | C17 已解（`Option<u32>` 拆两寄存器）；C18 的 20ms 阈值在 PE 中不存在，判为反编译失真 |

## 一、结论

> **当前真正剩下的核心差异只有 5 处 A 类（必须修）+ 1 处 C 类（应删除），外加 3 处 B 类（不用管）。**
>
> - A 类集中在 **executor 帧边界、同步来源、GPU 可见布局、raw 层释放、swapchain retire**。
> - 候选"能跑"来自端到端链路通，不来自机制对；两者分开记。

---

## 二、分类

### A 类：必须修（影响正确性 / 稳定性 / GPU 可见语义）—— 5 处

| # | 候选 | 原作 | 为什么必须修 |
|---|---|---|---|
| A1 | [`rg/executor.rs`](reconstruction/rhapsody-rebuild/src/rg/executor.rs) 的 `ExecutionChain` 只有 `execute_graph`，**无 `begin_frame`/`end_frame`/`wait_gpu_queue_trim`**，无 `VecDeque<PendingSubmission>` | `GraphExecutor` 首字段是 `VecDeque<PendingSubmission>`（元素 0x90 字节）；`begin_frame`(`0x140beabd0`) 排空 deque 到 ≤1 并预建首个 CB；`end_frame`(`0x140bec400`) push 本帧；`wait_gpu_queue_trim`(`0x140bea680`) peek 队首有界等待 | 帧在途数不受控 ⇒ GPU 未完成就被复用/回收。短跑看不出来，长跑必炸 |
| A2 | [`executor.rs:54-57`](reconstruction/rhapsody-rebuild/src/rg/executor.rs) `GraphBatchSync` 由 `backend.batch_synchronization()` **每批外部注入** waits/signals | waits/signals **不在图数据里**，来自 **`dev+0xa00..a4f` 装置级模板**（`get_setup_cb_submit_info 0x140bf84e0`）+ `queue_submit` 内部单调时间线分配器 | 同步语义的**来源**不同 ⇒ 依赖关系错位。与 A1 同属 executor 子系统 |
| A3 | [`buffer.rs:10-15`](reconstruction/rhapsody-rebuild/src/raw/buffer.rs) `BufferDesc = {size, usage, required/preferred MemoryPropertyFlags}` | **16 字节** `{u64 size, u32 usage, u16 alignment, u8 memory_location 枚举(1..4)}` | GPU 可见布局；对齐与内存位置语义不同 |
| A4 | [`buffer.rs:196-208`](reconstruction/rhapsody-rebuild/src/raw/buffer.rs) raw `Buffer` 的 `Drop` 直接 `device_wait_idle()` + `destroy_buffer` | managed `dispose<T>` **不调任何 `vkDestroy`**，推入销毁桶由 `queue_submit` 尾部按序列号排出 | raw 层仍是直接销毁，且 `device_wait_idle` 全设备同步，每释放一次全停一次 |
| A5 | [`managed/swapchain.rs:28-31`](reconstruction/rhapsody-rebuild/src/raw/managed/swapchain.rs) `retire() = mark_stale()`（注释自认"非原 ABI 镜像"） | `ManagedSwapchain::retire`(`0x1418ccb50`, 639B) 是"取回桶 + 重建"一部分 | raw 层已真实现（794 行），**只剩 retire 收尾**未对齐 |

### B 类：不用管（实现自由，不影响可观察行为）—— 3 处

| # | 候选 | 原作 | 为什么无所谓 |
|---|---|---|---|
| B1 | [`state.rs:25-29`](reconstruction/rhapsody-rebuild/src/rg/state.rs) `compile_frame` 注入 `impl ResourceBackend`/`impl AccessDependency` | **单态化**直接调具体 `ResourceInfo::calculate`/`populate_resources` | 纯内部结构，CPU 行为等价 |
| B2 | 资源表 / temporal 用 `Ustr` **名字**索引 | **u64 TypeId** 键（`Components::get_resource_id`） | 解析结果一致即可 |
| B3 | [`barrier.rs:4-8`](reconstruction/rhapsody-rebuild/src/raw/barrier.rs) `Access` 仍有 `write` 字段，[`executor.rs:169`](reconstruction/rhapsody-rebuild/src/rg/executor.rs) 仍在填 | 只有 `code`+`group`，**无 write 位** | `concurrent()` 已不读它，死数据，建议顺手清掉 |

### C 类：应删除（纯多余/误导）—— 1 处

| # | 说明 |
|---|---|
| C1 | **`ray_tracing.rs`（ash 硬件加速结构包装）在原作里没有任何对应** —— 本构建**无硬件光追**（四重独立确认），GI 走 **obvhs 软件 CW-BVH**。文件头部注释已自认"不属于本构建已证实的原作机制" |

---

## 三、"差别大吗"——分问题回答

| 如果你的目标是… | 差别大吗 | 说明 |
|---|---|---|
| **跑起来、出画面** | **小** | A 类 5 处多数在边缘路径（帧节流、raw 释放、retire）才暴露 |
| **长时间稳定运行** | **中** | A1/A4 是**在途帧与生命周期**问题，长跑必暴露 |
| **与原作行为等价** | **大（且不可达）** | A 类 5 处 + 签名/类型不可恢复（见 [ASSESSMENT.md](ASSESSMENT.md) 障碍 1） |
| **作为"自己写的实现"交付** | **可接受** | 只剩 `BufferDesc`（A3）是硬 GPU 边界，其余可边跑边补 |

---

## 四、已经对齐的（本次修订的净收益）

逐文件核对后，以下**此前被列为差异、现在已对齐原作机制**：

| 项 | 当前源码证据 |
|---|---|
| barrier 并发判定 | [`barrier.rs:60-66`](reconstruction/rhapsody-rebuild/src/raw/barrier.rs) `concurrent()` 用 `code.wrapping_sub(0x28) < 0x10` —— 正是 `can_be_concurrent_with` 的 `classify(v)=(v-0x28)<0x10` |
| `SynchronizationScope` | [`barrier.rs:11-33`](reconstruction/rhapsody-rebuild/src/raw/barrier.rs) memory/buffers/images 三向量 + `source_stages=1`/`destination_stages=0x2000` —— 正是 `mask@+0x48 = 0x0000200000000001` |
| managed 延迟退役 | [`resources.rs:230-294`](reconstruction/rhapsody-rebuild/src/raw/managed/resources.rs) `enqueue_retirement` → `dispose_control_kind` → `buckets.append_existing(sequence, …)` |
| generation 在 inner 递增 | [`tracking.rs:124`](reconstruction/rhapsody-rebuild/src/raw/managed/tracking.rs) `generation.wrapping_add(1)` |
| 无 domain 跟踪 | [`handle.rs:26-31`](reconstruction/rhapsody-rebuild/src/raw/managed/handle.rs) `track_on(domain)` 对 `domain != 0` 返回 `CrossDomainMappingUnknown`；`ControlBlock = {last_sequence, references, generation}` 与 inner 块一致 |
| 7 类 tag 枚举 | [`resources.rs:314-334`](reconstruction/rhapsody-rebuild/src/raw/managed/resources.rs) `{Texture0, Buffer1, Swapchain2, RenderPass4, Sampler5, DescriptorSet6, Framebuffer7}` —— 连跳过 `TextureView=3` 都对齐 |
| descriptor 空闲池复用 | [`descriptor_set.rs:569-612`](reconstruction/rhapsody-rebuild/src/raw/descriptor_set.rs) `DescriptorPoolQueue::allocate` 空闲池复用 + `OUT_OF_POOL_MEMORY`/`FRAGMENTED_POOL` 重试 |
| handle clone / drop | [`handle.rs:68-105`](reconstruction/rhapsody-rebuild/src/raw/managed/handle.rs) `clone`=refcount++、`drop`=`enqueue_retirement` |
| temporal 状态机 | [`temporal.rs`](reconstruction/rhapsody-rebuild/src/rg/temporal.rs) `TEMPORAL_EXPORT_ACCESS=0x37`、状态 0/1/2、`parity` + `get_and_flip_parity` |
| raw swapchain 真实现 | [`raw/swapchain.rs`](reconstruction/rhapsody-rebuild/src/raw/swapchain.rs) 794 行，acquire/present/resize/`OUT_OF_DATE` 分支 |

---

## 五、最需要先做的

按"投入产出比 × 阻塞程度"排：

1. **raw `Buffer` 释放改走销毁桶 + 去 `device_wait_idle`**（A4）—— 独立、成本低、收益大。
2. **`BufferDesc` 对齐 16 字节布局**（A3）—— GPU 可见边界，直接改。
3. **删 `ray_tracing.rs`**（C1）—— 纯清理。
4. **swapchain `retire` 收尾**（A5）—— raw 已大部分对齐，补 retire。
5. **executor 帧边界 + 同步来源**（A1+A2）—— 一个需要一并重构的子系统，最后动。

---

## 六、诚实边界

- "原作"一侧全部来自**静态逆向**（伪代码 + PE 字节），**零动态证据**。
- 候选一侧只读了 `src/`，**没有读 tests 与 examples**；清单可能仍不完整。
- 严重度分级是**判断**，非原作显式声明；判据"是否影响可观察行为"本身未经实验验证。
- 候选"能跑"是既成事实，但**跑通 ≠ 机制对** —— 这正是本表用途。
