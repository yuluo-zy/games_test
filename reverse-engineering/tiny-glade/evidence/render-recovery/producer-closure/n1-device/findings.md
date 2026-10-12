# N1 取证：raw::device 底层

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra + `run-lanes.mjs`。产物：`job1–6.json`、`REPORT1–6.json`、`REPORT.json`、**53 份** `0x*.c`。
- 仅取证，未修改任何禁止路径。

## 0. 父级复核

`CommandBuffer::new` / `begin` 的分派槽已逐行核对，**sType 常量与真实 Vulkan 枚举完全吻合**：

| 位置 | 立即数 | 对应 Vulkan sType | 分派槽 | 父级结论 |
|---|---|---|---|---|
| `0x140bed050.c:25` | `0x27` = 39 | `COMMAND_POOL_CREATE_INFO` | `dev+0x298`，首参 `*(dev+0x640)` | **确认** |
| `0x140bed050.c:32` | `0x28` = 40 | `COMMAND_BUFFER_ALLOCATE_INFO` | `dev+0x2b0` | **确认** |
| `0x140bed230.c:25` | `0x2a` = 42 | `COMMAND_BUFFER_BEGIN_INFO` | `dev+0x2c0` | **确认** |

⇒ **`GraphicsDevice+0x640 = VkDevice`** 现为**三重确认**：N1 的调用实参、**R3 独立在 `immediate_empty_bucket` 读出**、以及结构上的 `+0x78 + 0x5c8`。

## 1. 结构总纲（观察，多函数交叉印证）

`GraphicsDevice` 在 `+0x78` 内嵌一个 **ash「设备函数表」**；**VkDevice 句柄落在 `GraphicsDevice+0x640`**（= `+0x78 + 0x5c8`）。
`create_buffer` 把 `param_2+0x78` 传进 `create_buffer_impl`，其中 `param[0x21]`(= `+0x108`) = `vkCreateBuffer`、`param[0xb9]`(= `+0x5c8`) = 句柄；而 `VulkanBackend`(`0x140bff5a0`) 里 `lVar2 = *dev` 时 `vkCreateBuffer` 在 `+0x180`（= `0x78+0x108`）、句柄在 `+0x640`。

⇒ **ash 偏移 X ↔ GraphicsDevice 偏移 `0x78 + X`**。

> **归属澄清（修正前几轮的表述）**：`raw::submit` / `raw::semaphore` / `Queue` 的方法以**「ash 设备（句柄 `+0x5c8`）」**为 `self`；`GraphicsDevice` 的方法以 **GraphicsDevice（句柄 `+0x640`）**为 `self`。因此前一轮的 `dev+0x4f8`/`+0x4f0`/`+0x478` 实际是 **ash 函数表槽位**，不是 GraphicsDevice 字段。

## 2. 分派槽（由 `pCreateInfo` 的 sType 或调用实参确证 —— **观察**）

**GraphicsDevice（self 带句柄 `+0x640`）**：

| 槽 | Vulkan 函数 |
|---|---|
| `+0x270` | `vkCreateFramebuffer`（sType 37） |
| `+0x280` | `vkCreateRenderPass`（38） |
| `+0x298` | `vkCreateCommandPool`（39，含 `queueFamilyIndex`） |
| `+0x2b0` | `vkAllocateCommandBuffers`（40） |
| `+0x2c0` | `vkBeginCommandBuffer`（42） |
| `+0x2c8` | `vkEndCommandBuffer` |
| `+0x2d0` | `vkResetCommandBuffer` |
| `+0x328` | `vkCmdBindDescriptorSets`（8 参） |
| `+0x3e0` | `vkCmdPipelineBarrier`（10 参；full-barrier 用 stage `0x10000` ALL_COMMANDS） |
| `+0x268` | `vkUpdateDescriptorSets` |
| `+0x898` / `+0x8a0` | `vkCmdBeginDebugUtilsLabel` / `vkCmdEndDebugUtilsLabel` |

**ash 函数表（self 带句柄 `+0x5c8`）**：`+0x40` `vkCreateSemaphore`、`+0x60`/`68`/`70`/`78` `vkBindBufferMemory`/`vkBindImageMemory`/`vkGetBufferMemoryRequirements`/`vkGetImageMemoryRequirements`、`+0x90` `vkCreateBuffer`、`+0x478` `vkGetSemaphoreCounterValue`。

**推断**：`+0x88` `vkQueueSubmit`（前轮）、`+0x98` 设备级等待、`+0xe0`/`+0xf0`/`+0x1a0` 图像族、`+0x210`/`+0x1c8`/`+0x200` 管线族、`+0x3a0` crash marker 写入、`+0x4f0` `vkGetSemaphoreCounterValue`、`+0x4f8` `vkWaitSemaphores`、`+0x870` = `Option<debug-utils>`（空值门控；被 `(*(*(dev+0x870)))(*(dev+0x8b0), &nameInfo)` 调用，见 `set_buffer/texture_debug_name` 与 `create_command_buffer`）。

## 3. `GraphicsDevice` 字段（观察）

| 偏移 | 内容 |
|---|---|
| `+0x648` | → 上下文/注册表 `S`；`S+0xa50` = u16 设备代号（对象 `+0x14` 的代号须与之相等） |
| `+0x658` | → **配置管理器**（`Vec`，stride `0x1c`，在 `+0x38`/`+0x40`；`+0x78`、`+0x3a8` 各一 u32） |
| `+0x668` / `+0x670` | 每队列 u64 序号数组 / 个数 |
| `+0x678` | → **全局分配器**（锁 `+0x10`，数据 `+0x18`；`with_global_allocator 0x1405b9810` 印证） |
| `+0x718` | 回收锁字节 |
| `+0x720` | `SubmissionEngine{ +0x10 图形 Queue, +0xc8/+0xd0 Vec<Queue> }` |
| `+0x730` | 图形 `Queue`（= `0x720+0x10`，reclaim 目标） |
| `+0x7e8` / `+0x7f0` | `Vec<Queue>`（stride 0xb0）/ 个数 |
| `+0x810`/`+0x818`/`+0x848` | 崩溃锁 / **4096 项 marker 环** / 写计数 |
| `+0x870` | 可选 debug-utils |
| `+0x9b0` | 命令缓冲**池锁** |
| `+0x9c0`/`+0x9c8` | 池数组 `{seq, 空闲链指针, 计数}`（stride 0x18）/ 个数 |
| `+0x9f0` | 安装锁 |
| `+0x9f8..0xa4f` | **内联的「当前 setup CommandBuffer 记录」**（0x58 字节；由 `with_setup_cb 0x140c03fa0`/`0x140c04840` 写入） |
| `+0xa50` | 命令缓冲/提交追踪器 |
| `+0xa88`/`+0xa90`/`+0xa98` | GPU 断言 RwLock / Vec / 长度（`has_any_gpu_assertion_failed 0x140bf0720`） |
| `+0xad4` | 命令缓冲数量 |

> **交叉印证**：`+0x670`/`+0x678` 在本表是「每队列序号个数」与「全局分配器」，而 A6 在 allocator 包装里把 `+0x670`/`+0x678` 读作 `peak_device_local`/`peak_system` —— **两处偏移相同但 self 不同**（A6 的 self 是 `GpuAllocator` 包装，本表是 `GraphicsDevice`）。**勿混。**

## 4. 队列 / 命令池 / fence 路径（观察）

- `CommandBuffer::new`(`0x140bed050`) → `dev+0x298` `vkCreateCommandPool(queueFamily)` + `dev+0x2b0` `vkAllocateCommandBuffers`，句柄存 `CommandBuffer+0x48`。
- `create_command_buffer`(`0x140bef4a0`)：从池空闲链取/建缓冲，锁 `+0x9b0`，分配失败时经 `dev+0x870` 调 debug 命名。
- `begin`(`0x140bed230`)：`dev+0x2d0` reset → `dev+0x2c0` begin，从 `dev+0x668` 取序号，写状态 = 1；`end`(`0x140bed3d0`)：`dev+0x2c8` end，状态 → 2。
- **`CommandBuffer` 大小 0x58**：`+0x00` 状态(0/1/2)、`+0x08` 序号、`+0x10` flags、`+0x14` 队列号、`+0x48` 句柄、`+0x50` 队列族、`+0x54` u16。
- 时间线信号量 `create_raw_timeline_semaphore`(`0x1418c8710`)：`sType=1000207003` 链 + `sType 9` `SemaphoreCreateInfo`，调 ash `+0x40` `vkCreateSemaphore`。
- **`Queue`（0xb0）**：`+0x08` = u64 提交值环指针、`+0x10` 头、`+0x18` 待决数、`+0x80` **缓存计数器**、`+0x88` **时间线 VkSemaphore**；`update_pending_submissions`(`0x140bf0b90`) 用 ash `+0x478` 取值并弹出 ≤ 值的环项。

> **交叉印证**：`queue+0x88` = 时间线信号量 —— R1 与 R3 独立得到同一结论。

## 5. crash 与 profiler（观察）

- `0x140befb00` 记 `{序号, 参数}` 入 `+0x818` 环并写 GPU marker（`dev+0x3a0`）；`0x140befc70` **仅当 `VkResult ∈ {2, -4}`** 触发 `0x140befd60 log_gpu_crash_info`（读环），配套 `0x1418c46c0 log_gpu_debug_info`。
- profiler：`0x140bff4a0` 建 **0x2000 字节** GPU 时间戳缓冲（经 `0x140bff5a0 VulkanBackend`：`vkCreateBuffer` + 全局分配器 + bind），`TimedFrame::new`/`send_to_puffin` 在 `0x140bd3cb0`/`0x140bd4030`。

## 6. 仍未知

- `GraphicsDevice+0x168`/`+0x3f8`/`+0x400`/`+0x408` 在反编译源中**未作为设备字段出现**（疑为栈/常量或子结构）；
- 5 个微函数 `device_uuid`/`physical_device`/`debug_utils`/`min_subgroup_size`/`refresh_memory_budget`（`0x140bf84c0`/`8950`/`8960`/`8fa0`/`8f80`）**Ghidra 无函数体**，decompile 与 disassemble 均失败，**仅名称可辨**；
- `dev+0x808`、`dev+0x3a0`/`+0x200`/`+0x210`/`+0x1c8`、`dev+0x320` 的确切 Vulkan 身份未定；
- ash 函数表内部精确排序未核实。

**诚信说明**：第 1–5 节的"确证"基于 `pCreateInfo` 的 sType 常量或调用实参签名；标"推断"者仅给候选名；未知名不编造。
