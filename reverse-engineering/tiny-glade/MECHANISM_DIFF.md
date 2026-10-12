# 候选实现 vs 逆向机制：差异分析

- 日期：2026-10-12。
- 对照对象：候选实现 `reconstruction/rhapsody-rebuild/src/**`（**能编译、能跑**） ↔ 本工程逆向出的原作机制（见 [SYNTHESIS.md](evidence/render-recovery/producer-closure/SYNTHESIS.md) / [ROUND3_FINDINGS.md](evidence/render-recovery/producer-closure/ROUND3_FINDINGS.md)）。
- **回答的问题：差别大吗？**

---

## 一、结论

> **差别不小 —— 但差别分两种，性质完全不同。**
>
> - **"策略不同"那一类不影响可观察行为**，属于实现自由，**不用管**（7 处）。
> - **"结构性缺失 / 与 GPU 可见语义冲突"那一类会真实影响正确性或功能**（8 处），**必须修**。
>
> 而且要注意：**候选"能跑"来自端到端链路是通的，不来自机制对**。两者要分开记。

---

## 二、分类

### A 类：**必须修**（影响正确性、功能或 GPU 可见语义）—— 8 处

| # | 候选 | 原作 | 为什么必须修 |
|---|---|---|---|
| 1 | **无 `begin_frame`/`end_frame` 帧边界**，无 `VecDeque<PendingSubmission>`，无帧节流 | 三者皆有；`begin_frame` 把 deque 排空到 ≤1，`end_frame` push 本帧记录 | **帧在途数不受控** ⇒ GPU 未完成就被复用/回收。这是生命周期正确性的根 |
| 2 | **`GraphBatchSync` 从外部注入 waits/signals** | waits/signals 由 **`dev+0xa00..0xa4f` 装置级模板**（`get_setup_cb_submit_info 0x140bf84e0`）+ `queue_submit` 内部时间线分配器产生，**不在图数据里** | 外部注入的同步语义**必然**与原作不同 ⇒ 屏障/信号错位 |
| 3 | **`managed/swapchain.rs` 仅 13 行 stub**（只有类型别名 + `manage_swapchain` + `raw()`） | `new`/`acquire_next`/`present`/`retire`/`dispose`/`desc`/`resize`/`set_vsync` **全部有实现** | **窗口呈现根本没接上**。这是最大的单点功能缺口 |
| 4 | **每帧新建 2 个 semaphore + fence**；无 `OUT_OF_DATE` 重建闭环 | **逐槽复用 + 队列锁**；`validate_swapchain_and_acquire_image 0x140bfd9d0` 在 OUT_OF_DATE 时 `create_swapchain`（先 retire 再 new）→ drop 旧 → 再 acquire | 窗口 resize / 全屏切换 / 驱动返回 OUT_OF_DATE 时**会挂或崩** |
| 5 | **屏障用 write 位近似**"是否需要同步组"；逐 access 展开 transition；**无 `SynchronizationScope`** | `classify(v)=(v-0x28)<0x10` + 上一访问索引 `+0x04` + `can_be_concurrent_with`，**仅不并发才生成**；有 `SynchronizationScope`（三向量 0x18/0x38/0x48 + 累计掩码 + `reset`/`is_empty`） | **屏障过少 ⇒ 数据竞争 / 渲染错乱**；过多 ⇒ 性能塌 |
| 6 | **`BufferDesc` = `size` + `usage` + `required/preferred MemoryPropertyFlags`** | **16 B = `u64 size` + `u32 usage` + `u16 alignment` + `u8 memory_location` 枚举**(`1=GpuOnly,2=CpuToGpu,3=CpuToGpuLowPrio,4=GpuToCpu`) | **GPU 可见布局**。对齐与内存位置语义不同 ⇒ 分配行为/性能/正确性偏差 |
| 7 | **直接 `drop(Arc)` 释放** | **延迟入销毁桶**；`dispose<T>` **不调任何 `vkDestroy`**，只把裸句柄推入 9 个句柄 Vec + 0x48 记录 Vec，由 `queue_submit` 尾部按"序列号 ≤ `queue+0x80`"排出 | **CPU 释放早于 GPU 完成 ⇒ 用后即毁**（use-after-free on GPU） |
| 8 | **跟踪集合 `BTreeMap<domain, seq>`** | **单个 atomic u64 高水位**，**无 domain 字段** | 原作**仅凭一个 u64 就能判完成**；候选引入 domain 意味着**在解决原作没有的问题**，而它是否覆盖真实的多队列场景未验证 |

### B 类：**不用管**（实现自由，不影响可观察行为）—— 7 处

| # | 候选 | 原作 | 为什么无所谓 |
|---|---|---|---|
| 9 | `compile_frame` 注入 trait 对象 | **单态化**直接调 | 纯内部结构，CPU 侧行为等价 |
| 10 | 资源表**按名字**索引 | **u64 TypeId** 键 | 只要解析结果一致，索引方式自由 |
| 11 | `DescriptorSetState` 用 `BTreeMap<Vec<u64>, Arc<Set>>` **内容寻址** | **空闲池复用 + 池耗尽重试** | **策略不同，结果可等价** —— 但注意 `OUT_OF_POOL_MEMORY`/`FRAGMENTED_POOL` 的**重试行为要覆盖到**，否则池耗尽即失败 |
| 12 | **7 个独立 Manager** | **共享 store + 共享待回收 Vec** | 组织方式自由 |
| 13 | generation 递增在 slot | 递增在 **`dispose<T>`（inner）** | 只要"旧句柄被拒"的判定成立即可 |
| 14 | `surface` 只有 `from_win32` | **trait 对象** winit 绑定（`HasDisplayHandle`/`HasWindowHandle`） | 平台适配方式自由 —— **但候选因此绑不到真窗口**（与第 3 条叠加） |
| 15 | 用了 `raw::texture`/`raw::buffer` 的**候选描述符** | 同上 | —— |

### C 类：**纯多余/误导** —— 1 处

| # | 说明 |
|---|---|
| 16 | **`ray_tracing.rs`（ash 硬件加速结构包装）在原作里没有任何对应** —— 本构建**无硬件光追**（四重独立确认）；GI 走 **obvhs 软件 CW-BVH**。这个文件应视为**占位/误导**，不该计入"原作恢复范围" |

---

## 三、"差别大吗"——分问题回答

| 如果你的目标是… | 差别大吗 | 说明 |
|---|---|---|
| **跑起来、出画面** | **不大** | 16 处差异里 7 处无所谓；剩下的多数在边缘路径（resize、池耗尽、窗口重建）上才暴露 |
| **长时间稳定运行** | **大** | A 类 #1/#4/#7 是**在途帧与生命周期**问题，短跑看不出来，跑久了必炸 |
| **与原作行为等价** | **很大** | A 类 8 处 + 签名/类型不可恢复（见 [ASSESSMENT.md](ASSESSMENT.md) 障碍 1），**等价不可达** |
| **作为"自己写的实现"交付** | **可以接受** | 但**第 3/#6 这两条是硬边界** —— swapchain 必须真接上，`BufferDesc` 必须对上 |

---

## 四、最需要先做的三件事

按"投入产出比 × 阻塞程度"排：

1. **把 `managed/swapchain.rs` 从 13 行 stub 补成真实现**（A#3/#4）—— 它同时卡住"能出画面"和"能 resize"。原作机制已完整逆向（`new 0x1418ca030` / `acquire_next 0x1418cb520` / `present 0x1418cc1b0` / `retire 0x1418ccb50` / `validate_swapchain_and_acquire_image 0x140bfd9d0`），**照着写即可**。
2. **把释放路径改成延迟入桶**（A#7）—— 原作的 `dispose<T>` 不调 `vkDestroy`、由 `queue_submit` 尾部按序列号排出，机制已完整逆向（见 `r3-managed-disposal/findings.md`）。
3. **把 `BufferDesc` 对齐原作 16 字节布局**（A#6）—— 这是 GPU 可见边界，改起来成本低。

**不建议优先动的**：`GraphBatchSync`（A#2）—— 它牵扯 executor 的帧边界（A#1），要一并重构；而 `BTreeMap<domain,seq>`（A#8）需要先想清楚原作单 u64 高水位怎么覆盖多队列，否则改了更糟。

---

## 五、诚实边界

- 本对照表的"原作"一侧全部来自**静态逆向**（伪代码 + PE 字节），**零动态证据**。
- 候补一侧只读了 `src/`，**没有读它的测试与 examples**；差异清单可能不完整。
- 差异的**严重度分级是我的判断**，不是原作的显式声明。分类依据是"是否影响可观察行为（GPU 可见布局 / 生命周期 / 同步语义）"，这个判据本身没有实验验证。
- 候选"能跑"是既成事实（前序会话记录：整图创建/填充/拷贝/屏障/时间线提交/256 字节读回通过），但**跑通不代表机制对** —— 这正是本表的用途。
