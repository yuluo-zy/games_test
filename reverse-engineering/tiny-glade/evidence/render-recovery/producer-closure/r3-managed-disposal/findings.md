# R3 复核：managed 生命周期 ABI + **销毁桶排出机制**

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra + `run-lanes.mjs`。产物：`job.json`/`job2.json`/`job3.json`、`REPORT*.json`、29 份 `0x*.c`。
- **本批 `disassemble_function` 对全部目标返回 0 条指令**（本库无汇编可读），因此跳表类分支无法坐实。
- 仅取证，未修改任何游戏代码。

## 0. 父级复核（独立验证）

1. `raw::disposal` 清单：父级从 `workgraph/functions.json` 独立枚举，**恰 10 个函数、合计 12,263 字节**，与 R3 的报告完全一致：
   `remove 0x1418c4100`(346) / `insert 0x1418c4260`(522) / `dispose_cb_resource<T>` ×7（`0x1418c55b0`/`5b00`/`6050`/`65a0`/`6af0`/`7040`/`7590`，各 1348）/ `dispose_in_order 0x1418c7ae0`(1959)。
2. 排出条件：`0x140bf0cc0.c:97` = `uVar24 = *(ulonglong *)(param_1 + 0x80);`（完成高水位）；`:111` = `if (uVar24 < *(ulonglong *)(lVar18 + 0x58 + (uVar26 - uVar27) * 0x60)) break;`（**元素步长 0x60、序列号 @+0x58**）；`:502` = `if (iStack_1a0 != 3) goto ...`（**仅 `elem[0]==3` 回收**）。**全部确认。**

## 1. 销毁桶真身（观察，逐字段）

桶 = **9 个「同类型裸句柄 `Vec`」+ 1 个 0x48 字节记录 `Vec`**；`remove`(`0x1418c4100`) 按值返回 0xf8 字节。

- 句柄 `Vec` 位于 `+0x00, +0x18, …, +0xd0`（`immediate_empty_bucket` 的检查点 `[2],[5],[8],[0xb],[0xe],[0x11],[0x14],[0x17],[0x1a]` 即 9 个）；
- 记录 `Vec` 位于 `+0xd8`/`+0xe0`/`+0xe8`；
- `insert`/`remove` = 1 秒超时自旋锁 + hashbrown swiss-table（`FUN_1418adf90`/`FUN_1417e7ad0`）。

> **更正 A4**：A4 的「句柄表 + 0x48 记录表」是**部分视图**。`ManagedBuffer::dispose`(`0x1418afb90`) 只写 buffer 那一组（`+0x30`）+ 共享记录 `Vec`（`+0xd8`），实际有 **9 组**。

## 2. 排出机制（重点，观察）

### 时机 —— 提交驱动，不是定时/计数阈值

`GraphicsDevice::queue_submit`(`0x140c04e20`) **尾部**调 `FUN_140bf0cc0(&device+0x730, device)`，即 `Queue::reclaim_command_buffer_resources`。**每次提交后都跑。**

### 条件 —— 序列号 ≤ 完成高水位

- `w = *(queue + 0x80)`（u64 完成高水位）；
- 循环弹 deque 元素（**0x60 字节**：cap@+0x20 / ptr@+0x28 / head@+0x30 / len@+0x38，**序列号 @+0x58**）；
- `if (w < elem.seq) break;` —— 未完成的就停；
- **仅 `elem[0] == 3` 的元素被回收**。

→ **排出充要条件：该提交序列号 ≤ `queue+0x80`。**

### 顺序

1. 到期 seq → `FUN_1418c7ae0(queue+0x40, seq, device)` = `SequentialDisposal::dispose_in_order`；
2. `dispose_in_order` 维护**有序 seq 集 + 单调已排出水位 `self[4]`**（入口断言 `seq > self[4]`）；对每个 seq：
   `hash 9 字节键 → 0x1418c4100(&bucket, device+0xa50, seq)`（remove 取回桶）
   → `0x140bf92f0(device, &bucket)`（**真销毁**）
   → `0x140bdfcf0(&bucket)`（空桶析构）；
3. `immediate_empty_bucket`(`0x140bf92f0`) 按**固定字段序**排空 9 组，每组调 **device vtable 的不同槽**：
   `(*(*(device+off))(*(device+0x640) /*VkDevice*/, handle, 0))`，
   `off` = **`0x1c0 / 0x1a8 / 0x188 / 0x6b8(目标 *(device+0x6f0)) / 0x138 / 0x278 / 0x288 / 0x228 / 0x248`** ⇒ **9 个真正的 `vkDestroy*` 派发槽**；
4. 最后才处理 0x48 记录 `Vec`(`+0x1d`)：锁 `device+0x678`，`r+0x44 == 2` 则停，逐条 `FUN_140c0e150`，收尾 `FUN_140be1cf0`（对元素 `+0x38` 的 `Arc` 做 refcount--）。

> **更正 A4**：`FUN_140bdfcf0` **不是 timeline 判定器**，而是**空桶析构**（对 9 组 `(len, ptr)` 逐个 `HeapFree`，并递减记录 Vec 元素的 `Arc`）。**其中不含任何 timeline 比较** —— 判定发生在 `reclaim`。

### 与 GPU 完成的对接

- `GraphExecutor::wait_gpu_queue_trim`(`0x140bea680`) 的 `PendingSubmission` 是 **0x90 字节**结构（**与 queue 的 0x60 字节结构不同**），取队首 `+0x80` 的 16 字节等待描述 → `FUN_140bfcf80`：`lock self+0x718; (*(*(self+0x4f0))(*(self+0x640), value, &out));`，超时 `param_4 + 1e9`。
- ⇒ **GPU 完成对接 = vtable `+0x4f0` 的 timeline 信号量等待**（**推断**，槽名未由符号证实）。
- **排出侧不等待**，只读已完成高水位 `queue+0x80`。

## 3. managed ABI 逐条

**确认（无遗漏字段）**：
- 句柄 0x18：`+0x00` store / `+0x08` inner / `+0x10` u32 gen / `+0x14` u16 store_id / `+0x16` u8 tag；
- 16 字节退役记录 `{inner@0, gen@8, store_id@0xc, tag@0xe}`（`dispose<Buffer>` `0x1418af800` 校验 `record+0xc == MR+0xa50`、`record+0xe == 1`）——**末 1 字节为 padding，无遗漏字段**；
- inner：`+0x00` atomic u64 tracked / `+0x08` atomic u32 refcount / `+0x0c` gen / `+0x10` value；
- **generation 仅在 `dispose<T>` 且 refcount == 0 时 `inner+0xc += 1`**，随后压入 free-list（**Treiber 栈**，头 = `store_base+0x170`，`next` 复用 `inner+0x10`）。

**新增/更正**：
- `ManagedResources` = 7 store @ `+0x08`，步长 **0x178**；`store_id` u16 @ `+0xa50`；
- 每 store = **44 个块指针 @ `+0x00..+0x158`**（判据 `if (idx < 0x2c)`）+ **容量 u64 @ `+0x160`** + free-list 头 @ `+0x170`；槽尺寸随类型不同（观察到 0x18/0x80/0xd0）；
- **tracked 无 domain 字段**：`track 0x140bd6a70` 与 `get_* 0x1418aeba0` 都只对 `inner+0` 做 `atomic_max`（`param_2` 是 `int*`，`*p == 1` 是 tag 断言）。⇒ **仅凭这一个 u64 不能做跨队列复用**（这是对候选设计的一个重要约束）。
- `clone 0x140bd6b00` / `eq 0x140bd6b30` **确认无函数体**（decompile 报 `No function`；`0x141620a10` 以 `func_0x000140bd6b00(&out, &handle)` 调用之）。

## 4. 观察 / 推断 / 未知

**观察**：第 1、2 节全部（含父级复核的两条）；第 3 节的确认项与新增项。

**推断**：vtable `+0x4f0` = timeline 等待；`queue+0x80` = 已完成 timeline 计数。

**未知**：
- **`queue+0x80` 的写者未定位**（`reclaim`/`queue_submit` 都只读它）；
- **tag 跳号 3 与 `case 9` 未恢复**：7 个 store 对应 tag `{0,1,2,4,5,6,7}`，而桶有 **9 个 destroy 槽** ⇒ tag 空间 ≥ 10，3/8/9 无对应 store。`case 9` 跳表 Ghidra 报 `Bad instruction`，且**本 build 无汇编可读**；
- `clone`/`eq` 逐字节语义；0x48 记录字段名（`+0x40` 状态字节、`+0x44 == 2` 终止、`+0x38` Arc）；
- 9 个 destroy 槽与 7 个 store 的对应关系未逐个证实。

## 5. 关键地址

`immediate_empty_bucket 0x140bf92f0`｜`reclaim_command_buffer_resources 0x140bf0cc0`｜`queue_submit 0x140c04e20`｜`wait_gpu_queue_trim 0x140bea680`｜timeline wait `0x140bfcf80`｜`dispose<Buffer> 0x1418af800`｜`take_out_dropped_waste 0x1418c0a70`｜`dispose_in_order 0x1418c7ae0`｜记录处理 `0x140c0e150` / 后处理 `0x140be1cf0`
