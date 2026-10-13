# A7 取证：多队列 / wait-signal-prior / ray tracing

- 目标：原发行二进制 `D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra（`run_auto_analysis:false`）+ 对二进制直接 `grep -a`。job/报告见 `job*.json` / `REPORT*.json`，保留伪代码 35 份（`0x*.c`）。
- 仅取证，未修改任何 Rust 源码或清单。

## 交付边界与工具限制

Ghidra 伪代码不是作者原 Rust 源码；PDB 签名普遍 `void()` 占位。`run_auto_analysis:false` 下 xref 完整性未知，空 xref 不等于无 caller。

> **工具限制（实测）**：本 DB 上 `get_strings` / `list_names` / `find_code_by_string` 全部返回空，`disassemble` 返回 0 指令。字符串类证据改用**对二进制直接 `grep -a`**，父级已独立复现（见第 5 节）。

## 1. 队列拓扑（观察）

- `queue_family_index` (`0x140bfc750`)：返回 `dev+0xad4`（**主族**）；若 `kind` bit0 置位则返回 `dev+0x3c`（**第二族**），但要求 `dev+0x20` bit0 置位，否则 panic。
- `create` (`0x140bf2560`, 155KB 伪代码)：枚举队列族属性（**stride 0x1c**），
  - **必建**第一份 `VkDeviceQueueCreateInfo`：`sType=2`、`familyIndex=主族`、`queueCount=1`、`priority 0.5f`；
  - **条件建**第二份：当且仅当存在 `(queueFlags & 3) == 2` 的族（compute 位置位、graphics 位清零 = **专用 compute 族**），`queueCount` = 该族 queueCount。
- 时间线信号量在 `create` 中**只创建一次**（`0x1418c8710`：`VkSemaphoreCreateInfo` + `pNext=SemaphoreTypeCreateInfo{TIMELINE, initialValue=0}` → `vkCreateSemaphore`），**唯一调用点来自 `create`**。

**结论**：设计支持 **graphics + 专用 compute 两队列**。本验证机（Intel UHD 630）无专用 compute 族 → 单队列，与既有 `queue-graph-functional-validation.json` 的 `aux=0` 相符。

**推断**：实测以单队列为主。
**未知**：未定位到把 pass 派发到辅队列的**调度点**；`dev+0x3c` / `dev+0x20` 的写入点在 `create` 反编译中未直接见到。

## 2. `queue_submit` 内部（观察）

`0x140c04e20`（11047B）。**父级确认**：`0x140c04e20.c` 实际落盘 **38453 字节**，即本次反编译**成功**（A1 那一轮返回 error envelope 属瞬时失败/竞争，非函数不可反编译）。**唯一调用者是 `0x140bebdf0`（`submit_pending_cbs`）**——与 A1 的观察互相印证。

内部流程：
1. 取 `dev+0x718` 互斥；
2. 拷 0xd8 字节入参；
3. 调 `0x140bf84e0` `get_setup_cb_submit_info` 造一条 0x90B 内部 setup-CB 的 `VkSubmitInfo`（字段从 `dev+0xa00..a40` 读）；
4. `memmove` 拼 0x90B 提交记录数组；
5. 调 `0x1418c3300` `update_queue_pending_submissions`；
6. 错误路径调 `0x140bf0cc0` reclaim。

真正的 `vkQueueSubmit` 是**间接调用**（ash `DeviceFn` 表），不是直接边。

## 3. wait / signal / prior 生产者（观察，补齐既有缺口）

链路：
```
execute(0x1400fb5b0) → execute_compiled(0x1400fa190)
   每批次：ensure_active_cb(0x140bebc90) → 批次录制器(0x1402c1d40) → submit_pending_cbs(0x140bebdf0)
end_frame(0x140bec400) 同路径
```

- `execute_compiled` 把 **队列 id = `recorder[0x2f]`(+0x178)** 与 **`recorder[0x2e]`(+0x170)** 的 queue 状态（含 `+0x88` 时间线信号量）交给批次录制器。
- `submit_pending_cbs` 经设备 vtable `+0x2c8` 结束末条 CB、置 `state=2`，拷 `recorder+0x58` 的 0x90B SubmitInfo，调 `queue_submit`。（与 A1 及父级逐行核对一致。）
- `0x140bf0b90` 用 ash `getSemaphoreCounterValue`（dev fn `+0x478`）读 `queue+0x88` 时间线信号量，弹 pending 环（`[0]`cap / `[1]`ptr / `[2]`head / `[3]`count），已完成写 `+0x80`。

**结论**：原侧 wait/signal 生产在**渲染图批次路径 + `queue_submit` 自身的单调时间线分配器**，**不是外部注入**。这与候选实现里 `GraphBatchSync`（外部 waits/signals）形成直接冲突——候选是自造机制。

**未知**：具体 wait 值 / stage 未逐值恢复（落在 `FUN_1402c1d40` 的 helper `0x1402c3240` 与 `queue_submit` 内部）。

## 4. 并行录制（观察）

- 观测到 `CommandBufferSeqTracker::track` (`0x140becfd0`)、`CommandBuffer::begin`/`end`、`ensure_active_cb` 单活跃 CB、recorder 单命令 `Vec`。
- ~~**未见**多线程 / 多 CB 并行录制的证据。~~ **更正（2026-10-13）**：`record_batch` 经 `bevy_tasks` 按上一帧耗时分块并行录制，见 [parallel-recording/FINDINGS.md](../../parallel-recording/FINDINGS.md)；本节的"单活跃 CB"只描述 `ensure_active_cb` 的串行路径。多队列使用另见 [multi-queue/FINDINGS.md](../../multi-queue/FINDINGS.md)。
- 命令→时间线映射：`queue_submit` 为每命令追加队列时间线信号量的递增信号值 + pending 环。

## 5. ray tracing 结论：本构建无硬件光追

**三重证据（全部观察；父级已独立复现第 1 项）**

1. **扩展名字符串**（对 exe `grep -a`，父级复现）：

   | 字符串 | 计数 | 说明 |
   |---|---|---|
   | `VK_KHR_acceleration_structure` | **0** | |
   | `VK_KHR_ray_query` | **0** | |
   | `VK_KHR_ray_tracing_pipeline` | **0** | |
   | `VK_KHR_swapchain` | 1 | 正向对照：确实在用 |
   | `VK_EXT_descriptor_indexing` | 1 | 正向对照：确实在用 |
   | `VK_KHR_timeline_semaphore` | 0 | Vulkan 1.3 已并入核心，故不需要扩展名 |
   | `VK_KHR_synchronization2` | 0 | 同上 |
   | `VK_KHR_dynamic_rendering` | 0 | 同上 |
   | `VK_EXT_mesh_shader` | 0 | 未用对照 |
   | `VK_KHR_video_queue` | 0 | 未用对照 |

   正向对照有效 → 该 grep 只反映**真实引用**的扩展名。三个光追扩展全 0 = **有意义的负证据**。

2. **特性字符串**中无 `acceleration_structure` / `ray_query`。

3. **符号层**：无 `vkCreateAccelerationStructureKHR` / `vkCmdTraceRaysKHR` 等函数名字串；无 `ash::khr::acceleration_structure` 符号；无 `rhapsody::raw::ray_tracing::*` 符号。GI shader `_gi_trace_gi_candidates` 的 SPIR-V 能力仅 `Shader` + `RuntimeDescriptorArray`。

   > 二进制里出现的 `AccelerationStructureKHR` / `RayTracingShaderRead…` 字符串是 **ash 库自带的枚举表**，不是使用证据——不能用它反推硬件光追。

**正向的软件路径（观察）**：
- `0x140b0da50` `build_gpu_bvh` 调 `0x1417c93d0` = **`obvhs::cwbvh::builder::build_cwbvh_from_tris`**（公开库 obvhs 的 **CPU** CW-BVH 构建器）。
- `0x140b24d50` `bvh_builder_worker` 是 `std::thread` 工作循环（futex/condvar/`thread::sleep`）→ **后台 CPU 线程建 BVH**。
- 父级复核：对 exe `grep -a -o "obvhs"` 命中 **6 处**，且上下文是**源码路径**：
  `/h3/.cargo/git/checkouts/obvhs-8e4538d3c2ff8290/a3f3b3b/src/{cwbvh/buil[der], bvh2/reins[ert], heapstack, bvh2/mod, splits}` —— 调试信息里确实带着 obvhs 的源码路径，与 `Cargo.lock` 的 `obvhs` git 依赖（`h3r2tic` rev `a3f3b3b`）一致。
- `country_core::resources::rt` 模块 + `InstanceBuffer<RtBvhSsbo/RtTrianglesSsbo/RtTriangleMetaSsbo>` + `bind_ray_tracing` (`0x1415fc230`, 400B 描述符绑定，**非 AS**) + `ray_trace_gi` pass (`0x140a01cf0`)。

**判定**：候选 `src/raw/ray_tracing.rs`（ash 硬件加速结构包装）在原侧**无任何对应**（无扩展 / 无符号 / 无函数名）→ 属**多余/占位**。原侧光追是 **obvhs 软件 CW-BVH + compute 遍历**。`build_gpu_bvh` 这个命名有误导性：实为 **CPU 构建**、产物上传 SSBO。
`GPU-Raytracer 1.3.0` 是 `Cargo.lock` 里无 source、无依赖的**占位包**，不能据此断硬件光追。

## 6. 观察 / 推断 / 未知

**观察**：第 1 节队列族判据与时间线信号量创建点、第 2 节 `queue_submit` 内部流程与唯一 caller、第 3 节 wait/signal 链路、第 4 节无并行录制、第 5 节全部三重证据。

**推断**：实测以单队列为主（验证机无专用 compute 族）。

**未知**：
- 第二族字段 `dev+0x3c` / `dev+0x20` 的写入点。
- 运行时是否真的使用辅队列。
- wait/signal 的具体信号值与 stage。
- `GPU-Raytracer` 占位包与实际代码的关系。

## 7. 对候选实现的影响

- `GraphBatchSync`（外部 waits/signals / `SubmitInfo` 注入）**不是原机制**；原侧 waits/signals 由批次路径 + `queue_submit` 内部时间线分配器产生。
- 候选 `ray_tracing.rs` 应视为**无原侧对应**的占位，不应计入原作恢复范围。
