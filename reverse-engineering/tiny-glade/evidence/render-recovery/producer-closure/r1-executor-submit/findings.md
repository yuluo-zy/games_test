# R1 复核：executor 帧循环 + submit/queue 边界

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra + `run-lanes.mjs`（2 lane）。产物：`job.json`、`REPORT.json`。
- **本轮只取 xref，未取新伪代码**；行号指 `../a1-executor/0x*.c` 与 `../a7-queues-raytracing/0x140c04e20.c`。
- 仅取证，未修改任何源码。

## 0. 父级复核

R1 的核心新论断经父级独立核对：`0x140c04e20.c:925-926` 为
```c
iVar18 = (**(code **)(uStack_80 + 0x88))(puVar24[0xf], (ulonglong)ppcStack_238 & 0xffffffff, pcStack_240, 0);
```
四参 = `vkQueueSubmit(queue, submitCount, pSubmits, fence=0)`；`L716` 的 `... * 0x60` 确认 pending ring 元素步长 0x60。**确认。**

## 1. 确认（前一轮结论成立）

- `begin_frame` `0x140beabd0` 建 recorder：`+0x170`=executor、`+0x178`=device、`+0x188`/`+0x189`=参数 5/6(byte)、`+0x108` 初始 = 2。开帧把 executor 的 deque 排空到 ≤1：`while len > 1 { pop_front → FUN_140bfc780 }`。4282 字节大半是 tracing/panic 脚手架。
- `ensure_active_cb` / `retire`（0x48 命令表 + 资源表）/ `take_resource`（7 字段，`6` = None）/ `drop`（`[+0x108]==2 ∧ [+0x18]==0`）均确认。
- `queue_family_index` `0x140bfc750`：`kind&1 == 0` → `dev+0xad4`；`== 1` 需 `dev+0x20` bit0 且返 `dev+0x3c`，否则 panic。
- 时间线信号量 `0x1418c8710`：`SemaphoreTypeCreateInfo`（sType `0x3b9df29a`、TIMELINE=1、initial=0），外层 `SemaphoreCreateInfo` sType=9，经 `*(dev+0xb8)` = `vkCreateSemaphore`。

## 2. 更正（前一轮的错误）

| # | 前一轮 | 实际 |
|---|---|---|
| 1 | A1：deque 头是 `[1]ptr/[2]head/[3]len` | 头是 **`{cap@+0x00, ptr@+0x08, head@+0x10, len@+0x18}`** —— A1 漏了 `cap@+0` |
| 2 | A1：`wait_gpu_queue_trim` 从 deque `pop_front` | **它不 pop**。只 peek head 元素 `+0x80` 的 4 个 dword，然后调 `0x140bfcf80`（有界等待）。**只有 `begin_frame` pop**。 |
| 3 | A7："唯一 caller" 是 xref 证据 | 这些函数各只有 **1 条 DATA 引用**（vtable 区），`get_xrefs_from` 全空 ⇒ `run_auto_analysis:false` 下**没有调用边**。A7 的"唯一 caller"是**伪代码直读推断**，不是 xref 证据。 |

## 3. 新增（重要）

### 3.1 帧节流的确切落点

- `0x140bfc780` = **`wait_submit_done`**：造 `VkSemaphoreWaitInfo{sType=0x3b9df29c, count=1, pSems=&queue+0x88}` → `(*(dev+0x4f8))(dev, info, 10^10 ns)` = **`vkWaitSemaphores`**（超时 10 秒）。
- `0x140bfcf80` 轮询 `(*(dev+0x4f0))` = **`vkGetSemaphoreCounterValue`**（`0x140bf0b90` 走的是另一个槽 `dev+0x478`）。
- **`queue` 记录 `+0x88` = timeline 信号量。**

### 3.2 `queue_submit` 的真正提交点与 wait/signal 来源（补上 A7 的缺口）

- 提交点：`(*(dev+0x88))(queue=puVar24[0xf], count, pSubmits, fence=0)`，即 **`vkQueueSubmit`**；`pSubmits` 来自 queue 记录的 `+0x20..+0x38` **pending 环（元素 0x60）**。
- **wait/signal 不在渲染图数据里** —— 它们来自 **`dev+0xa00..0xa4f` 的装置级模板**，由 `get_setup_cb_submit_info`(`0x140bf84e0`) 逐字段读出。**这就是 A7 留下的"wait/signal 具体值/stage 未恢复"缺口。**

### 3.3 `device+0x20` / `device+0x3c` 的写入点（A7 未找到）

在 `create` `0x140bf2560` 里（约 L2360-63）整块写 `dev+0x20..0x3f`（源 `stack+0x02fe0..`）：
- `dev+0x20` 低 dword = **「存在 `(queueFlags & 3) == 2` 族」标志**（L530-536 的搜索循环）；
- `dev+0x28` = 该族 `queueCount`；
- **`dev+0x3c` = 该族 `familyIndex`**（= `stack+0x02ffc`）；
- `dev+0xad4` = 主族（L2517）。

> **易误判点**：L1829 处的「`+0x3c`」写的是**另一个 0x4e0 堆对象**，不是 device。

### 3.4 pending 环与回收

queue 记录另有 pending 环（`+0x20..+0x38`，元素 **0x60**，`+0x58` = 完成值），由 reclaim `0x140bf0cc0` 回收。

### 3.5 结构补充

- deque 元素 0x90：**首 u64 = Option/tag**（拒 `0x8000000000000001`），后 0x88 payload；payload 首字节 bit0 = **主/副队列**、`+4` u32 = **副队列下标**，尾部 `+0x80`/`+0x88` = 该键 + 目标 timeline 值。
- batch 条目 0x18：循环以 **`+0x10` 字节 == 2`** 终止；构建者把**末条** `+0x10` 置 **1**（2 是独立哨兵）。载入为 3×u64（第 3 个 qword 低字节 = kind）。
- `end_frame` 另有一个 **0x18 步长**的数组，以 `+0x16` 字节 == 8 终止。

## 4. 观察 / 推断 / 未知

**观察**：第 1、3 节全部；第 2 节的更正。

**推断**：`dev+0x88`/`dev+0x4f8`/`dev+0x4f0`/`dev+0x478` 等设备分派槽对应的具体 Vulkan 函数（由参数形状与用途判断）；"payload 首字节 bit0 = 主/副队列"。

**未知**：
- `dev+0x168` / `0x3f8` / `0x400` / `0x408` 的 Vulkan 身份；
- `submit_pending_cbs` 内 **20ms 阈值** `uVar9` 的来源（Ghidra 记为 `param_2` 低 32 位，**疑反编译失真**）；
- batch kind `1`/`2`/`8` 的命名；
- **签名不一致**：`0x140c04e20` 恢复出 5 参，但唯一可见调用点只传 3 参 `(out, device, 0)` —— 存疑；
- 多线程 / 多 CB 并行录制**仍无证据**。
