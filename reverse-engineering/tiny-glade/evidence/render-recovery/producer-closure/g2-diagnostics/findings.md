# G2 取证：rhapsody 诊断/日志类函数判定

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 方法：`workgraph/calls.json` 静态调用图 + Ghidra 反编译（database `ccdiag`，2 lanes，`REPORT.json` 46 条）+ **PE 直读**机器码与字符串。
- 产物：`job.json`、`REPORT.json`、`decomp/0x*.c`、`asm/0x*.asm`、`_tools/pe_dis.py`、`_tools/pe_str.py`。
- 仅取证，未修改任何受保护文件。

## 0. 核心判定

**必须纳入渲染语义恢复的：无。** 这批全部是 tracing / 日志 / 错误格式化 / 统计 / 配置。

## 1. `leek::leekspin_print_dump`(`0x140be2b40`, 11,396 B) = 纯诊断

反编译引用 **约 172 个字符串常量**，全部是引擎事件标签：`update_pending_submission_timestamp`、`read_submit_timeline_semaphore_universal/_compute`、`dispose_resource_to_cb_bucket`、`destroy_image_views/semaphores/framebuffers/render_passes/samplers/descriptor_pools/swapchain`、`create/reset_descriptor_pool`、`allocate_descriptor_set`、`create/reuse/begin/end_command_buffer`、`wait_submits_done`、`wait_gpu_submit`、`wait_gpu_queue_trim`、`bucket_dispose_in_order`、`acquire_next_image`、`present_out_of_date/suboptimal`、`swapchain_retire/pending_recreation`、`allocate_dynamic_constants_chunk`、`full_pipeline_barrier`。

即：**把 leek 追踪缓冲 dump 成大段格式化文本**（调 `leek::impl$0::fmt`×3、`impl$2::fmt`×3 及 fmt 帮助函数）。**只读状态去打印，不改渲染状态**；其中的状态判定仅用于"是否有值可打印"。

- `calls.json` 唯一调用者 = `rhapsody::print_log_dump`(`0x140be6b40`，**5 字节 = `jmp 0x140be2b40`**)，而后者在 450 函数图内**无任何调用者**。
- ⇒ 这个"单函数 11,396 字节"是**巨型字符串格式化**，不是渲染逻辑。

## 2. `raw::error`（21 函数）= 纯诊断格式化

类型 = `rhapsody::raw::error::BackendError`：`impl$1::fmt`×20（Debug，多 CGU 单态化副本）+ `impl$3::fmt`(`0x140bdd3f0`，Display)。

反编译显示它们按枚举 tag（`*p - 9 < 2`）分派、拼出变体名，引用串：`Allocation{inner,name,requirements}`、`Vulkan{err,trace}`、`ResourceAccess{info}`。

> ### **`raw::error` 里不存在 VkResult → 行为的映射表。**
> 没有任何"可恢复 / 致命"判定。`Vulkan{err}` 只是把 err 字段交给其自身 Display 打印。
> **错误语义在别处** —— 见第 5 节的 swapchain 结论。这**关闭了 GAPS.md 里"`raw::error` 是否含结果码映射"的问题**。

## 3. `raw::workarounds`(`0x140082a00`, 477 B) = 配置数据 + serde

该模块**唯一**函数 = `serde::Serialize for GraphicsWorkarounds`，写出 **4 个字段**（PE 直读字段名）：

| 字段 | 名字长度 |
|---|---|
| `sync_before_rg_pass` | 19 |
| `sync_before_draw` | 16 |
| `sync_before_dispatch` | 20 |
| `serial_cb_recording` | 19 |

函数体**无任何 `vkCreate*` / 扩展 / feature 调用**，纯 serde 写字段 ⇒ 就"是否改设备创建参数"而言：**否**（否定任务里的假设）。

**但这 4 个是渲染行为开关**（在 rg-pass / draw / dispatch 前插同步；串行 CB 录制）。rhapsody 域内**找不到消费点**（无 `GraphicsDevice` 方法引用它，符号表只有这一个 `Serialize`）。App 侧是独立的 iced 配置 GUI（`tg_gpu_selector::vulkan_config::Controls::{new 0x140df35e0, workarounds_view 0x141a64e10, allowlist 0x141a64e30}`、`tiny_glade::vulkan_config_main::run_vulkan_configuration_app 0x1403e74b0`）。

> **判定**：模块本身是配置/序列化（**非**设备创建语义），**不是"必须纳入渲染恢复"**；但 4 个 flag 的**名称预示渲染行为**而**消费点未定位** ⇒ **列为开放项，不简单标为"诊断无影响"**。

## 4. 其余：全部诊断

| 项 | 判定 |
|---|---|
| `raw::debug::begin_label 0x140bf09b0` / `end_label 0x140bf0b40` | 先判 `GraphicsDevice+0x870 != 0`（debug-utils 加载器），再调 `+0x898`/`+0x8a0`（`vkCmdBegin/EndDebugUtilsLabelEXT`）。**确在 record 路径**（`calls.json` 显示 `CompiledRenderGraph::record_batch 0x1402c1d40` 调二者），但**被 gate 且纯发标签**，不改 GPU 工作 ⇒ 诊断 |
| `raw::stats::snapshot_and_reset 0x140c02da0`(34 B) | 原子地把 3×u64 计数器清零并返回旧值 |
| `raw::temp_list::add 0x14090dda0`(401 B) | 内联 8 槽 + 堆溢出容器 append |
| `leek::impl$0 0x140be2710` / `impl$1 0x140be2880` / `impl$2 0x140be2990` | 16 字节定长名字的 Display/From（`impl$0` 把非法字节替换成 U+FFFD；`impl$1` UTF-8 解码、>16 panic） |
| `init_logging 0x140be6a90` | `leekspin::ringbuffer::RingBuffer::new(0x10000)` 写入全局 `0x143634e40`，断言 `prev.is_none()` |
| `dump_log 0x140be6b50` / `print_log_dump 0x140be6b40` | `RingBuffer::to_inner` / `jmp leekspin` |
| leek 操作包装（`update_persistent_descriptor_set 0x140be2a40`=tag `0x12`、`create_compute_pipeline`=`0x13`、`create_raster_pipeline`=`0x14`、`begin_rg_pass`=`0x2a`、`end_rg_pass`） | 只把 1 字节 tag（+小 payload）append 进 `0x143634e40` 环形缓冲（`RingBuffer::append 0x1418c1380`），**不执行真正操作**，是追踪埋点 |

## 5. 错误语义的实际位置（父级复核 + 更正）

G2 引述的 swapchain 结论基本正确，但**一处常量标注有误**。父级直接读 `../a6-allocator-swapchain/0x1418cc1b0.c`（`ManagedSwapchain::present`）核对的真实常量：

| 常量 | 值 | 语义 |
|---|---|---|
| `0x3b9acdeb` | `1000001003` | **`VK_SUBOPTIMAL_KHR`**（正数） |
| `-0x3b9acdec` | **`-1000001004`** | **`VK_ERROR_OUT_OF_DATE_KHR`** |
| `-0x3b9eae18` | `-1000255000` | 非致命 |

⇒ **G2 报告里的 `OUT_OF_DATE(-1000001001)` 是标注错误**：`-1000001001` 是 `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`。正确值是 **`-1000001004`**（与 R4 的更正一致）。`acquire_next` 侧的超时 3 s、`SUBOPTIMAL` 当成功等结论不变。

## 6. 方法学收获（值得记下）

G2 核实：`get_xrefs_to` 对 `0x140be2b40` 返回的"D**DATA 引用**"经查是 **`.pdata` 异常目录条目**（存 RVA `0x00be2b40`），**不是调用边**。这进一步坐实 GAPS.md 的 D2：**`get_xrefs_*` 的输出必须用 `calls.json` 交叉验证**，否则会把异常目录当成引用。

## 7. 仍未知

1. `sync_before_rg_pass` / `sync_before_draw` / `sync_before_dispatch` / `serial_cb_recording` **是否被任何代码读取并生效**（未定位消费者；符号表只有 `Serialize`）。
2. `BackendError` 枚举 tag → 变体名的完整对应（只观察到 Allocation / Vulkan / ResourceAccess 三组格式串）。
3. `VkResult` → 名称的 Display 实现在何处（**不在** `raw::error`）。
