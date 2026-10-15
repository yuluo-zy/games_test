# wave3：提交路径与设备级剩余未读项闭合 + GPU 提交迁移评估

工具：`/tmp/dis.py`（objdump 反汇编 `tiny-glade.rne`）、全 `.text`（43MB）objdump 流式 grep、ash 0.38.0 `tables.rs` 字段序、`create`（0x140bf2560）全函数反汇编（.asm 为空，现场重新导出）。

## 1. 剩余未证实项

| 项 | 结论 |
|---|---|
| `dev+0x20`/`dev+0x3c` 写入点 | ✅ **[0x140bf2e10..0x140bf2e48, 0x140bf6388..0x140bf639a]** `dev+0x20..+0x3f` 是 32B `Option<QueueFamilyInfo>`：`+0x20` u32 tag（1=Some，读者 0x140bfc750 只测 bit0）、`+0x24..+0x3b` 族属性记录（flags/queueCount/tsBits/granularity）、`+0x3c` u32 族索引。`create` 在物理设备族属性数组（0x1c 步长，`+0x658→+0x38/+0x40`）上找首个 `flags & 3 == 2`（compute 且非 graphics），拷 0x1c 记录到栈 `[rbp+0x2fa4..0x2fc0)` 并置 `[rbp+0x2fa0]=1`（找不到置 0）；尾段两条 `movups`（`[rsi+0x30]`、`[rsi+0x20]`）整块写入设备。bit0 = “存在辅助 compute-only 族”，非队列使能位；与 `QueuePlan::select` 一致。已落 `device.rs` 结构表/`queue_family_index_for` 文档，`wave3_submit::second_queue_family_is_the_first_compute_only_family` 覆盖。（之前全 .text 的 `[reg+0x3c]` 显式写入扫描无命中是因为走 SSE 整块写。） |
| `FUN_1418c0a70` | ✅ 函数名 `rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste`（字符串 0x142d05f38/0x142d06098）。流程：1e9ns 超时自旋锁 `*(dev+0x648)`，`mem::take` 出 `Vec<Item16>`（`{ptr_to_handle@0, ..., tag@+0xe, flag@+0xf}`，tag==8 为结束），逐项经 jumptable `0x142d060f0`（9 项）分派到 `raw::disposal::dispose_cb_resource<T>` 的 7 个实例：tag0→0x1418c65a0（读 `[ptr+0x30]/[ptr+0x38]`）、tag1→0x1418c6af0（读 `[ptr+0x10]/[+0x18]`）、tag2→0x1418c55b0、**tag3→`ud2`（unreachable，0x1418c10f2，文案长 0x13）**、tag4→0x1418c6050、tag5→0x1418c5b00、tag6→0x1418c7040、tag7→0x1418c7590；每个分支传 `(dev+0xa50 SequentialDisposal, 栈上 10×24B 桶 rbp-0x48, 句柄, …, dev+0x648)`；循环后释放 Vec，再 `FUN_140bf92f0(dev,&桶)`（销毁）+ `FUN_140bdfcf0(桶)`（空桶析构）。7 个目标内部的“哪个 tag=哪种资源”：函数体 0x540B 级别泛型实例，未逐个读（⚠️）。 |
| `FUN_140bf92f0` | ✅ = `immediate_empty_bucket`：按固定序对桶的 9 组裸句柄调用 ash 函数表槽（`dev+off`，`dev+0x640` VkDevice）：`0x1c0` destroy_image_view、`0x1a8` destroy_image、`0x188` destroy_buffer、`0x6b8`(ctx `dev+0x6f0`，非 ash：分配器释放)、`0x138` destroy_semaphore、`0x278` destroy_framebuffer、`0x288` destroy_render_pass、`0x228` destroy_sampler、`0x248` destroy_descriptor_pool（偏移均用 ash 字段序核对）。最后处理 0x48B 记录 Vec（锁 `dev+0x678`）。⚠️ 每组与 `Doomed` 变体的一一对应只到 vkDestroy 名，记录 Vec 项内部语义沿用 r3 报告。 |
| setup 记录拼接顺序 | ✅ **[0x140c04e20 @0x140c051b6..0x140c05597]** 先建 0x90B 记录小 Vec：请求 `+0x40` 的可选记录（=`prior`）先压入（0x140c051ee..0x140c05243），随后 `get_setup_cb_submit_info` 结果压其后（0x140c05255..0x140c052e1），最后才压由请求命令数组构造的“当前”记录（0x140c05597）。顺序 = `[prior, setup, current…]`，**不是 wave2 的“setup 最先”**。已改 `submit.rs::queue_submit_with_setup`（prior 在前），测试 `wave3_submit::setup_record_goes_after_prior_and_before_current`；既有 `wave2` 用例（无 prior）不变。⚠️ 新发现的未改差异：`flushed_command` 在原版里是把其句柄追加进“当前”记录的命令句柄 Vec 末尾（0x140c05494..0x140c0551e，值 Vec 压 0），本模型仍把它独立成 `current` 前的一个批次；既有 `original_oracle*` 固定了该形态，未动。 |
| 时间线信号量 全局一个 / 每队列一个 | ✅ **每队列一个**（更正 wave2 的“全局一个”）：`create` 里 `0x1418c8710`（全 `.text` 仅此一个调用点 0x140bf4f08）创建**主队列**的一个 TIMELINE 信号量（`SemaphoreTypeCreateInfo` sType 0x3b9df29a，type=1，initial=0）；辅助队列由 `0x1418b8610`（`(start..end).map(..).collect::<Vec<Queue 0xb0B>>`）→ `0x1418b48f0` 循环，每个 aux 队列各创建一个（全 `.text` 搜 `0x3b9df29a` 仅这两处），存到队列元素 `+0x88`（`mov [rbx+rax+0x88],rcx`，stride 0xb0）。与 `QueueState.timeline_semaphore` 每队列一个一致。 |
| `dev+0x3e0/+0x2c8/+0x88/+0x370` 等函数 | ✅ 用 ash 0.38.0 字段序逐项核对（`wave3_submit::ash_function_table_offsets_match_machine_code` 以 `offset_of!`/`size_of!` 断言）：`dev+0x78` 起是 `ash::Device`，V1_0@+0x78（0x3c0B）、V1_1@+0x438、V1_2@+0x4b8、V1_3@+0x520、VkDevice@+0x640（ash 内偏移 0x5c8）。`+0x88` queue_submit、`+0x98` device_wait_idle、`+0x298/+0x2b0/+0x2c0/+0x2c8/+0x2d0` create_command_pool/allocate_command_buffers/begin/end/reset_command_buffer、`+0x370` cmd_copy_buffer、`+0x3e0` cmd_pipeline_barrier、**`+0x4f0` get_semaphore_counter_value、`+0x4f8` wait_semaphores（10s 超时 0x2540be400，0x140bfcab3）**。**更正**：`device.rs` 旧表的 “`+0x478` vkGetSemaphoreCounterValue” 是 ash 内相对偏移（0x140bf0b90 以 `&ash::Device` 为参数，见 `+0x478`/`+0x5c8`），dev 绝对偏移应为 `+0x4f0`/`+0x640`；已改文档。 |
| 慢提交 20ms | ✅ **[0x140bebffa..0x140bec1f4]**（`recorder+0x188 != 0` 才进入）：Info 日志（level 常量 3，且 log 最大级别 ≥3；target `rhapsody::rg::executor`，`executor.rs:598/603`）`Waiting for the device to be idle...` → `Instant::now` → `wait_idle`(0x140bf8980) → `elapsed`：`FUN_140d63910` 返回 Duration 的**秒部分**（`edx` 纳秒），条件 `secs != 0 || nanos >= 20_000_001`（即 `> 20ms`，wave2 的“第二条件未读”就是它）→ Info `Device idle wait hiccup detected. Suspicious; waiting more.` + `FUN_140d55980(2,0)` = Windows `thread::sleep(2s)`（`CreateWaitableTimerExW`、×0x989680）；**无论慢否再 `wait_idle` 一次**（0x140bec1f4）。wave2 称“warn”，实为 Info。已落 `rg/frame.rs::{DEBUG_SYNC_*, debug_sync_wait}`；⚠️ 未接到执行器提交路径（executor 不归我，且该分支是调试同步模式）；测试 `debug_sync_wait_logs_sleeps_only_when_slow_and_always_waits_again`。 |
| `mark_stale`/`is_stale` 与 `@0xa8` | ✅ `mark_stale`=**0x140c0e950**：校验 Managed 句柄（`+0x14` 代号==`dev+0x648→+0xa50`、`+0x16`==2、`ArcInner+0xc`==句柄`+0x10`，否则 panic），**取写锁**（`lock cmpxchg [inner+0x10] 0→8` @0x140c0e995，慢路径 1e9ns），仅当 `@0x30 == 2` 时把 `@0x08..@0x30`（当前 desc 0x28B）整块拷到 `@0x30`（= `Option<旧 desc>`，2 是 None niche；已有记录不动）；**不碰 `@0xa8`**。故 `@0x30==2` 的真义是“没有挂起重建请求”，不是“已创建”（已改 `swapchain.rs` 文档）。**`is_stale`：❌ 原版无此函数**（符号表无，读不出），重建的读锁版本保留为便利接口（候选）。`@0xa8`（0x1418ccb50 `retire`）：`v-2`（<3 否则 1）分派：**2=活动**（若 device+0x850 回调非空且 @0xb8 门字节非 0 则调回调；随后 `@0xa8 := @0xb9`）、**3=已退役**（幂等，返回 `@0xa0` 句柄）、**4=已释放**（panic `Attempting to retire a disposed swapchain`，target `rhapsody::raw::managed::swapchain`）。已落 `raw/swapchain.rs` 文档；行为代码未改（重建用 bool `stale`，语义仍是“非 2”）。⚠️ `@0xb9` 的写入值（3 还是 4）与 `@0xb8` 门字节的写入点未读。 |

## 2. 真实 GPU 提交路径迁移：**部分已对齐，retire_timeline/queue_lock 未迁移（有闸门，保持回退）**

读通后的事实（比 wave2 §4 的描述乐观）：
- **会话路径本来就走 `queue_submit` 模型**：`rg/executor.rs:964 self.device.queue_submit(queue_id, input, backend)`（`DeviceState::queue_submit_with_setup`）+ `VulkanGraphBackend::submit`（`rg/vulkan.rs`）以 **`vk::Fence::null()`** 调 `vkQueueSubmit`；完成靠每队列时间线信号量（`backend.timeline` / `QueueState.timeline_semaphore`，本批证实原版也是每队列一个）。所以“fence → NULL fence + 队列时间线”在**会话路径**已经成立；`fence` 只存在于 `VulkanCommandBuffer::submit_to`（宿主读回 `submit()`、`tg-text/gpu.rs`、测试 `draw_support`）。
- 会话路径与原版剩下的差异只有三处：(a) `VulkanGraphBackend::submit` 在本次所有批次末尾追加一个只 signal `retire_timeline` 的空提交（给 `Graveyard` 的 epoch 用）；(b) 提交串行化用单个 `queue_lock: Mutex<()>`（原版 `dev+0x800` RwLock 读锁 + 每队列 `Queue+0x70` 字节锁）；(c) 资源销毁是 epoch 制而非 `cb_seq` 桶（`InOrderRetire` 已有但未驱动销毁）。

没有迁移的原因（具体阻碍）：
1. `retire_timeline`/`note_submit_on` 同时被 **非我拥有** 的 `raw/setup.rs:87`、`raw/buffer.rs:581/650`、`raw/swapchain.rs`（present 路径，我拥有但与 present 队列同锁）使用，且 `Graveyard` 的 `completed_epoch` 推进依赖它；改成“按队列时间线值门控销毁”需要同时改 `buffer.rs/setup.rs/init.rs` 的提交点，超出本批文件归属；闸门明确禁止为迁移而删除 `retire_timeline`。
2. `submit_to` 的 `PendingCommand.fence`（pub 字段）是外部可见 API，非主队列提交（`additional_queues`，不在 `retire_timeline` 上）没有可替代的完成判据；去掉 fence 需要给每个 aux 队列建时间线（原版每队列一个，但重建的 `VulkanDevice` 没有）并改 `wait_idle`/`idle_if_in_flight_locked` 的宿主同步。
3. **队列锁**：`queue_lock` 是 `pub` 字段，被 `buffer.rs`（上传拷贝）、`setup.rs`、`swapchain.rs`（present，同一 VkQueue）共用以满足 Vulkan 对 VkQueue 的外部同步。若改读写锁而没有“每队列字节锁”，会让 `vkQueueSubmit` 与 `vkQueuePresentKHR` 在同一 VkQueue 上并发 → 违反外部同步。正确做法是先加每队列锁再把全局锁降为 RwLock，且所有非我拥有的调用点要同步改；本批不做。
结论：不放宽任何断言、不删 `retire_timeline`；`queue_lock` 角色已核对（见上）但不改。迁移前提清单：①`VulkanDevice` 持每队列时间线 + 每队列字节锁；②`buffer.rs/setup.rs` 改走 `SubmissionEngine::queue_submit`；③`Graveyard` 以 `(queue, value)` 门控替代 epoch；④再去掉 `retire_timeline` 与追加的空提交。

## 3. 会话 `begin_frame` 容量语义

- ✅ 0x140beabd0 的弹出承接项是容量 2 的 ArrayVec（`FrameThrottle::begin_frame_bounded` 已有，第三次弹出 panic）。
- **会话已与原版一致（证明不需要改变等待结构，但补了同款断言）**：`execute_frame` 在在途 > 1 时返回 `Err`（要求先 `begin_frame`），`end_frame` 之后在途最多 2，故 `begin_frame` 每次最多弹 1 个，永远到不了第三次弹出；无法通过公共 API 构造在途 ≥ 4。我在 `VulkanGraphSession::begin_frame` 的排空循环加了与原版同形的 `assert!(drained < BEGIN_FRAME_DRAIN_CAPACITY)`（正常不可达，越界时同样 panic 而不是静默多排）。聚焦测试：`wave3_submit::begin_frame_bounded_drain_matches_the_two_slot_arrayvec`（在途 2 弹 1、在途 4 panic）；会话在途峰值 2 由既有 `triangle_animation_graph` 断言。

## 4. 改动与验证

- `src/raw/device.rs`：仅文档（`+0x20..+0x3f` 行、`+0xa00..` 行更正、ash 函数表行、`queue_family_index_for`）。
- `src/raw/submit.rs`：`queue_submit_with_setup` 批次顺序 `[prior, setup, flushed, current]` + 文档。
- `src/raw/swapchain.rs`：仅文档（`mark_stale`、`@0x30`、`@0xa8`）。
- `src/rg/frame.rs`：新增 `DEBUG_SYNC_*` 常量与 `debug_sync_wait`。
- `src/rg/vulkan.rs`：`begin_frame` 排空断言。
- 新测试 `tests/wave3_submit.rs`（5 个）。
- 验证：`cargo test --offline -p rhapsody -p tiny-glade -p country-core -p shader-pipeline --all-features`（lavapipe）：**57 个目标 / 1873 passed / 0 failed**（含 `original_oracle*`、`triangle_*`、`token_space`、`wave2_queue_submit`）。

## 5. 仍未读通
- ⚠️ `take_out_dropped_waste` 7 个 `dispose_cb_resource<T>` 实例分别对应哪种托管资源（tag→类型）。
- ⚠️ `flushed_command` 在原版里并入“当前”记录句柄 Vec 末尾（见 §1），模型仍为独立批次。
- ⚠️ `@0xb9`/`@0xb8` 写入点；`queue_submit` 步骤 2 的“设备丢失”检查（`dev+0xa88`）未接入提交路径。
- ❌ `is_stale` 在原版中不存在（非缺口，是重建便利接口）。
