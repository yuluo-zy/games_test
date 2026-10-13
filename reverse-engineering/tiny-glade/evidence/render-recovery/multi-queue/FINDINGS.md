# 跨队列屏障与多队列使用：原作追查（2026-10-13）

- 对象：`tiny-glade.rne`（SHA `f507a6ce…`，B 原版），只读。方法：`workgraph/calls.json` 的 450 个函数 + `scripts/render-recovery/pe_readonly.py` 反汇编。
- 标注：**O** 观察 / **I** 推断 / **U** 未知。

## 1. 屏障里没有队列族所有权转移（O，限已索引范围）

`vkCmdPipelineBarrier` 的调用点（经 ash 设备表 `+0x3e0`）与屏障构造函数的全部调用者：

| 位置 | 做什么 | 队列族字段 |
|---|---|---|
| `ImageBarrier::new`（`0x140bd65e0`） | 构造时把 `+0xc` 的 8 字节（src,dst 族）写成 `0xFFFFFFFF_FFFFFFFF` | **IGNORED** |
| `BufferBarrier::new`（`0x140bd6610`） | 同上，`+0x8` | **IGNORED** |
| 调用者 | **只有** `record_batch`（`0x1402c1d40`）与 `add_pre_pass_barriers`（`0x1402c3d00`）。两处都是 `new` 之后紧接 `image_barrier`/`buffer_barrier`（`0x140bd66a0`/`0x140bd6810`），中间没有任何写 `+0xc`/`+0x8` 的指令（`0x1402c1faf→1fc2`、`0x1402c1f19→1f2c`、`0x1402c3db7→3dc4`） | 全程 IGNORED |
| `issue_pipeline_barrier`（`0x140bee030`） | 只转发批次 | — |
| `issue_full_pipeline_barrier`（`0x140bee090`） | 单个全局 `VkMemoryBarrier`（srcAccess=dstAccess=`0x18000`，两端阶段 `ALL_COMMANDS`），缓冲/图像屏障数为 0 | 无 |
| `get_setup_cb_submit_info`（`0x140bf84e0`）、`update_buffer_bytes::closure`（`0x140c03a00`，两处） | 同样的全局 `VkMemoryBarrier` | 无 |
| `init_texture_data::closure`（`0x140c08560`）、`update_texture_impl::closure`（`0x140c08830`） | 经 `vk_sync_fork::cmd::pipeline_barrier`；构造处写 `0xFFFFFFFF_FFFFFFFF` | IGNORED |

**佐证**：`raw::texture::get_image_create_info`（`0x140bd5ed0`）把 `sharingMode`（`+0x3c`）与 `queueFamilyIndexCount`（`+0x40`）整体写 0，即**所有图像都是 `SHARING_MODE_EXCLUSIVE` 且不列族**——跨族使用必须所有权转移，而原作没有转移，所以图像实际只会在一个族内使用。GPU profiler 的缓冲（`0x140bff5a0`）同样 EXCLUSIVE。通用缓冲的创建信息未定位（**U**）。

**限定**：覆盖的是 450 函数工作图里的全部 `+0x3e0` 调用点。若有未收入工作图的调用者、或 `vkCmdPipelineBarrier2`（同步2）路径，这里没看到（**U**）。

## 2. 多队列：基础设施齐全，但渲染图路径写死主队列（O）

**有的（设备层）**
- `create`（`0x140bf2560`）：存在"计算位置位、图形位清零"的专用族时，额外建该族**全部**队列；每个辅助队列有独立时间线信号量（`queue+0x88`）、待完成队列、独立的 CB 序号计数器（`dev+0x668/0x670`，`aux_count+1` 个，各自从 1 起）。
- `queue_submit` 按 `QueueId` 在 `dev+0x7e8/0x7f0` 的辅助队列向量里取队列，并校验"命令缓冲所属队列 == 请求队列"。
- `wait_submit_done`、`wait_duration_or_until_submit_done`、`wait_idle_and_recycle_resources`、`present`、`drop` 都遍历/索引该向量。

**没有的（使用层）**
- `GraphExecutionRecorder::ensure_active_cb`（`0x140bebc90`）**没有队列参数**：命令缓冲族固定取 `dev+0xad4`（主族），然后 `CommandBuffer::begin(cb, device, r8d=0)`，即 `Primary`。全函数没有按队列分支。
- `begin_frame`（`0x140beabd0`）同样读 `dev+0xad4`；5 个 `with_setup_cb` 单态化（`0x140c03fa0/04280/04560/04840/1416a3220`）也读 `dev+0xad4`。
- 既有取证（`parallel-batch-05/queue-callers`）：8 个直接 `begin` 调用点 + 1 个 `queue_submit` 调用点，调用前 `r8d` 全是 `xor r8d,r8d`。
- 公共符号里 `queue_submit`、`CommandBuffer::begin`、`submit_pending_cbs`、`ensure_active_cb` 各只有 **1 个**单态化，没有第二条提交路径。
- 查找辅助族下标的 `queue_family_index`（`0x140bfc750`）只有 `with_setup_cb`（`create_buffer` 那个实例）调用。

**结论（I）**：原作的渲染图录制、提交都走主（图形）队列；辅助计算队列的支持存在于设备层，但在已索引的代码里没有任何调用者选择它。这与"原作没有实现跨队列异步计算调度"一致，也与"屏障无所有权转移、图像 EXCLUSIVE"互相印证。

**U（不能排除）**：内联/未索引的调用者；运行时数据驱动的队列选择（本机验证设备没有专用计算族，无法动态验证）；`leek` 事件名里的 `read_submit_timeline_semaphore_compute/_universal` 说明设计上区分两类队列，但未找到写入它们的上层调用。

## 3. 对我们实现的含义

- 我们**不做**多队列调度与所有权转移，与原作行为一致，不是缺口。此前"跨队列所有权转移未恢复"应改为"原作屏障路径没有此机制"。
- 若要加多队列异步计算，那是**超出原作**的新设计：必须同时引入所有权转移（或并发共享模式）、跨队列时间线等待、逐队列 CB 序号，且本机无法验证。
