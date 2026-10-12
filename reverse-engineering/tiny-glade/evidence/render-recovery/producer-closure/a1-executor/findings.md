# A1 取证：executor 帧循环真身

- 目标：原发行二进制 `D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`（与 `D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne` 同一文件）。
- 工具：RE-MCP Ghidra，`run_auto_analysis:false` 只读会话，脚本 `scripts/render-recovery/run-job-queued.mjs`，job 见 `job.json`/`job2.json`/`job3.json`，原始返回见 `REPORT*.json`。
- 本目录只放取证产物；未修改任何 Rust 源码、清单或既有报告。

## 交付边界

Ghidra 伪代码不是作者原 Rust 源码；PDB 函数签名普遍是 `void()` 占位。`run_auto_analysis:false` 下 xref 完整性未知，空 xref 不等于无 caller。下文严格区分**观察**、**推断**、**未知**。

## 1. 帧循环顺序（观察）

外层 `tiny_glade::render_loop::render` (`0x1404e8010`) 每帧：

```
compile
→ begin_frame                0x140beabd0
→ execute_compiled           0x1400fa190
→ retire                     0x140be9d00
→ take_resource              0x140be9c80
→ from_owned_rg_resource
→ acquire_next               0x140bfd9d0
→ GraphExecutionRecorder::execute  0x1400fb5b0
→ end_frame                  0x140bec400
→ Swapchain::present         0x140c0ef80
→ stats / mem_budget
→ last_known_frame_duration_ns     0x140beaae0
→ wait_gpu_queue_trim        0x140bea680
```

**注意**：交换链图像获取发生在主体图 execute 之后，与既有架构结论一致；不能概括成"所有工作都要先 acquire 再开始"。

## 2. execute_compiled 的批次循环（观察）

- 按 batch 循环，batch 条目 0x18 字节，**末条 flag == 2** 作为终止标志。
- 每个 batch：`ensure_active_cb` → `CompiledRenderGraph::record_batch` (`0x1402c1d40`) → 批间 `submit_pending_cbs`。
- 末尾：`ResourceRegistry::into_exports` + `ExecutedRenderGraph::new` (`0x140bea180`)。

## 3. 跨帧状态（观察）

- `GraphExecutor` **首字段 = `VecDeque<PendingSubmission>`**（元素 0x90 字节；`[1]`=ptr、`[2]`=head、`[3]`=len）。
- `begin_frame` 与 `wait_gpu_queue_trim` 从该 deque `pop_front`，分别调 `wait_submit_done` / `wait_duration_or_until_submit_done`；`end_frame` `push_back` 本帧记录。**这是帧节流（frame-in-flight 限制）的实现位置。**
- 其它字段：`[0x2e]`=&executor；`+0x48`/`+0x88` 两组统计块每帧交换；每帧哈希表 `memset 0xff`；`[0x19]`=命令池句柄。

## 4. 各函数逐项（观察，除标注外）

| 地址 | 函数 | 观察到的行为 |
|---|---|---|
| `0x140bebc90` | `ensure_active_cb` | `recorder+0x50 == 0` 时：`create_command_buffer`（名称长度 0xe）→ 再调一次（推断为 `CommandBuffer::begin`，flag=0）→ 把默认状态块写入 `Vec`（ptr `+0x48`）→ 置 `+0x50 = 1` |
| `0x140bebdf0` | `submit_pending_cbs` | 见第 4b 节（父级已逐行核对） |
| `0x140bec400` | `end_frame` | 见第 3 节：push_back 本帧 PendingSubmission |
| `0x140be9d00` | `retire` | 按资源 kind 跳表分发销毁（`0x140be9e12` 跳表，Ghidra 报 too many branches）；释放 **stride 0x48 命令缓冲表** 与 **stride 0x40 资源表**；把源 0x90 壳体搬入目标 |
| `0x140be9c80` | `take_resource` | `HashMap<u32, Resource>::remove(idx)` 形态：命中（`!= 6`）则搬出 7 个字段，未命中走 panic 路径（`0x142b14618` + `swi(3)`）→ 即 `.unwrap()` |
| `0x140be2680` | `impl$6::drop` | 断言 `[+0x108] == 2` **且** `[+0x18] == 0`（否则 panic），即只有已完成的执行器壳体可被 drop |
| `0x140beabd0` | `begin_frame` | tracing span 前导 + `gpu_profiler begin_frame/create_marker/write_marker` + 排空等待 + `create_command_buffer`/begin + 把 `param_7` 复制进 recorder（`[0x23..0x31]`、`+0x189]`）；大量字节是内联 Rust 字符串/panic 脚手架 |
| `0x140bebc90`→ | CB 生命周期 | **每个 batch 一个命令缓冲**；`begin_frame` 已预建首个 |

## 4b. `submit_pending_cbs` 逐行核对（父级独立复核）

原始伪代码见 `0x140bebdf0.c`。这是 executor 与 `queue_submit` 的接缝，父级逐行核对，比 A1 原报告更细：

1. **入口先把 recorder 里的待提交状态搬出并清零**：`param_1[8]=0; param_1[9]=8; param_1[10]=0`（即 CB `Vec{ptr,len,cap}` 复位），`param_1[3]=0`（len 清 0，与 A1 结论一致），并把 `param_1` 开头 16 字节与 `param_1+0xb` 处的 **0x90 字节块** 复制到栈上。
2. **末 CB 结束走设备 vtable 间接调用**：`iVar6 = (**(code **)(lVar1 + 0x2c8))(*(undefined8 *)(lVar12 + -0x10))`，其中 `lVar1 = param_1[0x2f]`。→ A1 的"设备 vtable `+0x2c8`"**确认**；参数是 CB 句柄。
3. **成功路径置状态 2**：`iVar6 == 0` 时 `piVar11[0] = 2; piVar11[1] = 0;` —— 与 `retire`/`drop` 断言的 `state == 2` 语义呼应。
4. **真正调用 `queue_submit`**：`FUN_140c04e20(&uStack_68, param_2, 0)`，即 `queue_submit(out_receipt, device, 0)`。入参结构由栈上拼装：0x90 字节块副本 + CB `Vec{ptr,len,cap}`（`lStack_140/lStack_138/puStack_130`）+ 开头 4 个 dword + 若干字段。→ **A1 的"组 SubmitInfo → 调 queue_submit"确认**，且给出 `queue_submit` 的真实签名与调用点。
5. **回执写回 recorder**：`uStack_68`/`uStack_60`（2×u64 回执）落到 `param_1+0x108`、`+0x10c`、`+0x110`、`+0x114`（4 个 dword）。
6. **profiler 收尾**：`func_0x00014067dd80`（= `rhapsody::raw::allocator::GpuAllocator::inner`）取统计，`FUN_140d635e0`/`FUN_140d63910` 计时，`20000000 < uVar9`（20ms）触发慢提交告警分支，`FUN_140d55980(2,0)` 收尾。
7. `param_1+0x31`（字节偏移 0x188）为 0 时提前 return —— 一个开关字段，语义未知。

**边界**：`queue_submit` 内部仍未取证（11047B，Ghidra 反编译失败），因此 `VkSubmitInfo` 的 wait/signal/timeline 字段来源仍未知。第 4 步观察到的入参结构是**栈上拼装的候选形态**，未与 Vulkan 结构体逐字段对齐。



**观察**：帧内顺序；`ensure_active_cb`/`submit_pending_cbs`/`end_frame` 行为；执行器首字段 `VecDeque`；`take_resource` 与 `drop` 的断言；`retire` 的 stride 与壳体搬移。

**推断**（未直接证据）：设备 vtable 偏移 `+0x2c8`/`+0x400`/`+0x408` 对应哪些具体 Vulkan 调用；`SubmitInfo` 的 wait/signal 来源；batch flag 语义；`param_5`（recorder `[0x31]`）。

**未知**：`PassPgoDuration::new` (`0x140be9c70`, 仅 7B) 无法取证。

> **更正（A7 复核后）**：本报告原写 `queue_submit` (`0x140c04e20`, 11047B) 「反编译失败」。A7 在同一 DB 上**成功**取得 **38453 字节**伪代码（见 `../a7-queues-raytracing/0x140c04e20.c`）。因此那是**瞬时失败/会话竞争**，不是函数不可反编译。`queue_submit` 内部机制见 A7 findings 第 2 节；**唯一调用者确认为 `submit_pending_cbs`**，与本报告一致。

## 6. xref 情况（观察 + 限制）

上述函数各**仅 1 条数据表引用**（vtable 槽 `0x1436A55xx`–`0x1436A5AF8`），无代码 caller。这是**间接派发**的表现；在 `run_auto_analysis:false` 下 xref 不完整，**不能据此断言没有直接调用者**。

## 7. 候选实现差异（`reconstruction/rhapsody-rebuild/src/rg/executor.rs`）

- 候选 `ExecutionChain` **没有原版的 begin_frame / end_frame 帧边界**。
- 候选**没有**执行器首字段 `VecDeque<PendingSubmission>`、**没有** `wait_submit_done` / `wait_duration_or_until_submit_done` 帧节流。
- `GraphBatchSync`（外部 waits/signals）是**候选新增**，原机制中未见对应物。
- `rg/recorder.rs` 只产字符串 events，是纯投影，**没有真 `VkSubmitInfo`、也没有 vtable end-command**。
- `queue_submit` adapter 无法与原文对照。
- `BatchCompiler` 基本一致；屏障辅助部分已被候选化。

## 8. 父级抽验记录

父级独立读了 `0x140bebc90.c`、`0x140be9c80.c`、`0x140be2680.c`、`0x140be9d00.c` 四份伪代码，核对结论：

- `ensure_active_cb`：`param_1+0x50 == 0` 分支、写入 `param_1+0x48` 指向的 Vec 后置 `+0x50 = 1` — **一致**。名称长度常量确为 `0xe`。
- `take_resource`：命中路径搬 7 个字段，未命中走 `swi(3)` panic — **一致**（`6` 为 Option 的 None 判别值属推断）。
- `drop`：`[+0x108] == 2` 且 `[+0x18] == 0` 才 return，否则 panic — **一致**。
- `retire`：命令缓冲表步长 `0x48`、资源表 `>> 6`（即 0x40）、末尾搬 0x90 字节 — **一致**。
- `submit_pending_cbs`：设备 vtable `+0x2c8` 间接调用、`queue_submit`(`0x140c04e20`) 调用点与签名、回执写回位置 — **一致**，详见第 4b 节（比 A1 原报告更细）。

未抽验：`begin_frame`(4282B)、`end_frame`、`GraphExecutor::new`、`wait_gpu_queue_trim`、`last_known_frame_duration_ns`、`PassPgoStats::duration_ns` — 这些的"观察"结论按 A1 原始报告保留，未获父级二次确认。

## 9. 仍阻塞

1. `queue_submit` (`0x140c04e20`) 内部 — **已由 A7 补齐**（见 `../a7-queues-raytracing/findings.md`），本报告的阻塞项关闭；`VkSubmitInfo` 的具体 wait/signal 值仍未逐值恢复。
2. `PassPgoDuration::new` 函数体过短无法取证。
3. xref 完整性：vtable 间接派发未解析，直接调用者未知。
