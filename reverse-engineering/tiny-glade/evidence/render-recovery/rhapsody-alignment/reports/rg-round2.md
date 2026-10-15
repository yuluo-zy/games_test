# rg 域第二轮（工作包 2、5 与帧顺序）

图例：✅原版对齐[证据地址] / ⚠️候选或假设 / ❌读不出及原因。
验证：`cargo test --offline -p rhapsody -p tiny-glade -p country-core --all-features`（lavapipe），40 个测试目标全部 ok，无 FAILED；`original_oracle*` 无回归。

## 本轮最重要的两个纠错（机器码直读，推翻既有结论）

1. `GraphExecutor::last_known_frame_duration_ns`（0x140beaae0）**不是 bool**，返回 `Option<u64>`（rax=标志，rdx=值）。
   Ghidra 伪代码丢了 rdx。机器码：读映射切片（长度 ≤ 0xf 则 panic，`cmp rdx,0xf; jbe`）的两个 u64，
   `v = (f32)[[dev+0x658]+0x3a8] as f64 × (end − begin) as f64`；`ucomisd 1.0e9(0x1429a25a8), v` 不成立（v ≥ 1e9 或 NaN）→ `xor eax,eax`（None）；
   否则 `rax=1`，`rdx = v as u64`（饱和：负 → 0、超大 → u64::MAX，对应 0x1429cfca8/0x1429cfcb0 两个常量）。
   country-core 的 `RenderHost::last_gpu_frame_ns -> Option<u64>` 早就是这个形态，A1/上一轮 rg 报告里的"bool"是错的。
2. `PassPgoStats::duration_ns`（0x140bea1e0）同样是 `Option<u64>`，**不是** `contains_key`：命中时 `mov rdx,[bucket-0x8]; mov eax,1`，
   未命中 `xor eax,eax`。唯一调用点 record_batch 0x1402c22c9 之后 `test al,1; jne use_rdx; xor edx,edx`，即 `unwrap_or(0)`。
   `rne/rg-notfound-closure.md` §12 的"实为 contains_key 返回 bool"需作废。现有 `duration_ns -> u64`（`get().unwrap_or(0)`）与原版一致，`get -> Option<u64>` 即原返回形态。

## 工作包 1：帧顺序

| 项 | 状态 | 说明 |
|---|---|---|
| 顺序 compile → begin_frame → execute → retire → acquire → execute(展示图) → end_frame → present → wait_gpu_queue_trim | ✅ [0x1404e8010 `render`；`render-passes/cpu-frame/sequence.md` §A；country-core `render_frame.rs` 已按此编排] | rhapsody 侧没有 acquire/present，它们在 raw/swapchain；新增 `rg::frame::{FrameStage, FrameSequence}` 作顺序检查器：acquire 没拿到图像（返回字节 == 7）则展示图与 present 都不允许；OOM 降分辨率重试帧不 acquire；trim 仅在有 GPU 时长时。6 个测试。⚠️ 检查器只被测试/调用方使用，`VulkanGraphSession` 本身不知道 acquire/present，没有强制 |
| `GraphExecutor::new`（0x140bea2d0） | ✅ | `vkCreateQueryPool{type=TIMESTAMP(2), count=2}`（device vtable +0x168，失败 panic）；`create_buffer("top_level_timing_query_buffer", size 0x10)`；对象 0xd0 字节：`+0x00` VecDeque（cap 0、ptr 8、head 0、len 0）、`+0x20` 时间戳缓冲（0x28 字节句柄）、`+0x48`/`+0x88` 两份 0x40 字节 `PassPgoStats`（各含 RandomState 种子 k0,k1 取自线程局部递增计数）、`+0xb8/+0xc0` 清零、`+0xc8` 查询池。落成 `frame::EXECUTOR_LAYOUT` 与名字常量 |
| `begin_frame`（0x140beabd0） | ✅ 行为 / ⚠️ 未改 session 的等待实现 | 弹出在途直到只剩 1，承接弹出项的 ArrayVec 容量 2（第三次弹出 panic）→ 新增 `FrameThrottle::begin_frame_bounded` + 测试；首条命令缓冲标签 `"profiler start"`、marker `"Frame setup"`、之后 `+0x108=2`（"本帧无提交"）等写入 recorder。session 的 `begin_frame` 仍用自己的 `wait_frame` 循环（`&mut self.backend` 借用冲突，见上轮报告），语义等价 |
| `wait_gpu_queue_trim`（0x140bea680） | ✅ | 超时非零且在途 > 1 时只等队首、不弹出；puffin scope `"Wait for GPU queue trim"`。上轮已接入，本轮复核机器码无出入 |
| `last_known_frame_duration_ns`（0x140beaae0） | ✅ | 见上；落成 `frame::last_known_frame_duration_ns(Option<&[u64]>, f32) -> Option<u64>`（含 <16 字节 panic、NaN、回绕、饱和），`VulkanGraphSession::last_known_gpu_frame_duration_ns()`（取顶层时间戳查询，≥1s 视为无效）。旧的墙钟版 `last_known_frame_duration_ns -> u64` 保留（候选统计） |
| `GraphExecutionRecorder::ensure_active_cb`（0x140bebc90） | ✅（上轮已读，本轮核对） | `recorder+0x50==0` 时建命令缓冲，标签 `"rg executor cb"`（0xe 字节）、`create_command_buffer` 第 5 参为 `[device+0xad4]`；落成 `frame::EXECUTOR_CB_LABEL` |
| `submit_pending_cbs`（0x140bebdf0） | ✅ 结构 / ⚠️ 慢提交语义 | 入口把 recorder 待提交状态搬出并清零；末条命令缓冲状态 == 1 才经 device vtable +0x2c8 结束它并置 2；`queue_submit(0x140c04e20)`；回执写回 `+0x108..+0x118`；`[+0x188]`（= `begin_frame` 的 `param_5`，与 `execute_compiled` 里"每 pass 一个 batch"同一个标志）为 0 时到此即返回，否则做慢提交日志。阈值 `20000000 < elapsed`（`SLOW_SUBMIT_THRESHOLD_NS`、`is_slow_submit`）。⚠️ 被计时的是 `FUN_140bf8980(device)`（queue_submit 之后、含义未读）而不是 queue_submit，另有一个未读的第二条件 `FUN_140d63910(..) != 0`，所以只落常量，没有接到提交路径上 |
| `execute_compiled`（0x1400fa190）的"计数置 1 否则 panic" | ✅ 重新解读 | 那不是"帧提交计数"：recorder `+0x00` 与 `+0x18` 是两个 `Option` 槽（acquire 等待信号量，阶段固定 0x400=COLOR_ATTACHMENT_OUTPUT；呈现信号量），来自 `SwapchainLeaseSync::into_sync_data`（0x140c0f040）；每槽一帧内只能写一次，旧值非空 panic（`FUN_1428d9430` 长 0x20 断言串）。落成 `frame::SwapchainSyncSlots`（带测试）。同时说明 A1 报告把 `GraphBatchSync` 称"原机制无对应物"不准确——acquire 等待 + 呈现信号就是它在原版的位置，但现 `GraphBatchSync` 是更一般的时间线等待/信号，未改 |
| `end_frame`（0x140bec400） | ✅ 结构 / ⚠️ 部分接入 | ① `ensure_active_cb`；② 对 pending 条目（0x18 步长）：队列号相同则用 CAS 把时间线值抬到 `max(旧, 新)`（要求末条命令缓冲状态 == 1，否则 panic）→ `frame::raise_timeline_to`（测试）；③ vtable +0x400 / +0x408 结束与提交，`submit_pending_cbs`；④ 回执标签仍为 2（本帧无提交）→ panic，`+0x164` 已为 2（重复 end）→ panic → `frame::FrameLedger`（`FrameLedgerError::{NoSubmission, EndedTwice}`）；⑤ push 回执入队；⑥ `PassPgoFrames` 前后帧交换并清空新 current（现 `PassPgoFrames::end_frame` 一致）。账本已接入 `VulkanGraphSession`（begin 清零、execute 记提交、end 校验），但**只告警不 panic**：现 session 的空图可能没有回执，原版 `ensure_active_cb` 保证至少一条命令缓冲。⑤⑥ 之前已做；② 的 CAS 目标是 swapchain 租约的时间线，session 里该机制由 `submit` 引擎承担，未改写 |
| pending submissions 原子 CAS 收时间线 | ⚠️ | 函数已有、测试已有，但没有接到 session 的真实提交路径（现路径用 `raw::submit` 的计数器/时间线，语义相同方向） |

## 工作包 2：回调签名与 compile 入口

| 项 | 状态 | 说明 |
|---|---|---|
| 回调 `FnOnce(u64)->bool` | ✅ | `PassBuilder::render(FnOnce(u64)->bool)` 早已是原签名，`render_with_context(FnOnce(PassContext)->bool)` 为兼容扩展；`graph::RenderCallback` 内部仍是 `FnMut(PassContext)`（需要多次重试语义），未改 |
| `compile<Export,UD,F>` 三个单态 | ✅ 入口 / ❌ 返回布局 | `RenderGraphState::{compile_loading_graph(0x1401647b0), compile_world_graph(0x140161cd0), compile_presentation_graph(0x140166510, Export=())}`，都转给原 `compile`；`pass_entries::CompileInstance` 带地址。❌ 原版返回 `Result<(CompiledRenderGraph 0x218B, Export), CompileError>` 经 out 指针，成功/失败标签是 `i64::MIN` 小值优化（加载图 0x1401647b0 末尾写 `*out = 0x8000000000000000` 是失败路径），`Export`=`ResourceExportHandle<Texture>` 的 Rust 定义在 compile 反编译里没有字段级证据，所以没有造这个类型 |
| `CompiledRenderGraph` 0x218B 布局 | ✅ 并纠正一处 | `compiled::original_layout`：`+0x00..0x90` 图记录、`+0x90..0xa8` `Vec<Pass>`（cap@0x90 ptr@0x98 **len@0xa0**，元素 0x40）、`+0xa8..0x1b0` ResourceRegistry(0x108)、`+0x1b0..0x200` SynchronizationScope(0x50)、`+0x200..0x218` `Vec<u32>`。**纠正**：ALGORITHMS 写"+0x90 5 个 qword Pass 头"与 `+0xa8` 起的 Registry 重叠（0x90+0x28 > 0xa8），按 3 个 qword 读；`execute_compiled` 用 `[+0xa0]` 作 pass 数。段首尾相接的测试通过 |
| 现 `graph::CompiledGraph` 与 0x218B 的关系 | ❌ | 现类型是语义候选，没有偏移对应，故只给常量不给 `repr(C)` 类型 |

## 工作包 3：PGO

| 项 | 状态 | 说明 |
|---|---|---|
| `PassPgoNameId` 指针恒等 | ✅ 行为 | 原 `from_name` = `mov rax,rcx; ret`（0x14067dd80），键即 `&str` 数据指针；`PassPgoStats` 桶比较只比这个 u64（0x140bea270）。现改为**驻留池**：同名只留一份 `&'static str`，ID = 其数据指针（`PassPgoNameId(pub usize)`，原 `pub u32`；仓库内 `.0` 只在 executor.rs 里，调用方只用 `from_name`，API 兼容）。新增 `from_static(&'static str)`（不分配）。池只会随不同 pass 名个数增长。ID 数值无原版对应物（进程地址）。测试：跨来源同名同键、指针稳定 |
| `PassPgoStats::duration_ns` | ✅ | 见纠错 2；现返回 `u64` = 原 `Option` 的 `unwrap_or(0)`，`get` 返回原形态 |
| `PassPgoStats::reset` | ✅ 语义 | = `end_frame` 内联的清空：控制字节 memset 0xff、len=0、重算 growth_left、`total@0x30 = max@0x38 = 0`，哈希种子保留。`counts` 表是候选自加 |
| `PassPgoStats::extend` | ❌ | 无符号、`end_frame` 里没有对应内联体（只有整块交换 + 清空）；逐块耗时并入 current 的位置在并行块回调 closure$1 里，未直证。保持候选 `total += d / max = max(max,d)` |
| `PassPgoDuration::new` | ✅ [0x140be9c70 `mov rdx,r8; mov rax,rcx; ret`] | 透传 |
| `PassPgoFrames` 交换 | ✅ [0x140bec400] | `+0x48 ↔ +0x88`（含 4 对 qword，即 0x40 字节整块，含种子）后清空新 current；现实现一致 |

## 工作包 4

| 项 | 状态 | 说明 |
|---|---|---|
| `ImportedBuffer` 20B | ✅ 布局 / ⚠️ 任务里的"+0xe=5" | 0x140bd4fc0 返回值 = `{size u64@0, desc_tail u64@8, resource_index u32@0x10}`（前两个来自 `BufferRecord` 的 +0x18 / +0x20）。`import_export::ImportedBuffer`（`repr(C, packed(4))`，20 字节，`from_record`，测试）。同函数还把 `BufferRecord` 搬进 0x48 字节 `ResourceOrigin`（tag 1，`+8` 字节 {buffer=1, initial_access}），对 slot 引用计数 `lock inc [slot+8]`，并在 `+0x30` 的 16 字节条目 Vec 追加 `{0xffffffff,0}`。**`+0xe=5` 在这两个函数里没有出现**；"5"只在 ResourceRegistry 后备资源 tag 里（5=Empty），与 ImportedBuffer 无关，故未引入。Texture 版 0x140bd60a0 返回 36 字节，未建模 |
| `RenderGraphRecord<PassResources>` 命名 | ⚠️ | 命名证据仍是 execute_compiled 内的 drop_in_place 类型名；现 `rg::recorder::Recorder` 与它同名不同物，未改名（改名会破坏 `recorder` 的公开 API，且原类型的字段布局未读） |
| `ResourceSyncGroup` | ✅ 部分 | `new`/`default` = 全局 `AtomicU32` `fetch_add(1)`（0x140bece50，计数器 0x14362c230 初值 2）——现实现早已是全局计数器、初值 2。`is_equivalent` u32 相等（0x140bece60）。❌ 组级 `can_be_concurrent_with`：机器码里**只有** 0x140bece70 一个同名函数，入参是 `{u32 group@0, u8 access@4}`（即 `PassResourceAccessType`），先 `is_write(self)`，为假再 `is_write(other)`，都读不出独立的"组级"版本；0x140bece70 与现 `PassResourceAccessType::can_be_concurrent_with` 逐条一致，无需改 |
| `validate_read_access` | ❌ | 无编译符号、无地址、两个内联实例在调用点里没有可识别的指令序列（被优化掉的话也就是"无指令"，与"no-op"无法区分）。保持现 no-op（`resource.rs`），不升级结论 |
| `TemporalRenderGraphRecorder +0xc8..` 与 Record 0x68B | ✅ 部分 | 按 `export_temporal`（0x1404f7ca0）重新读：Recorder `+0x00..0xa8` 是被拷出拷回的 `RenderGraph`，`+0xa8` 控制字节指针、`+0xb0` mask、`+0xb8` growth_left、`+0xc0` len、`+0xc8/+0xd0` 哈希种子；`+0xd8..0x108` 另有字段只被原样搬运，**含义未读出**。Record：`+0x00` Ustr、`+0x08` 状态 u32（0/1/2）、`+0x0c` 起为负载（状态 0/1：`{u32 资源下标, 4×u64}` 喂给 `Texture::export` 0x140bd6380 / `Buffer::export` 0x140bd5120，访问码 0x37；写回状态 2、`+0x0c` 字节=0x37 及 export 返回载体）。其余字段名未读。落成 `temporal::original_layout`。注意：旧文档的"表 +0xb0/+0xb8/+0xc8"是以数据区指针为基准的读法，与此处以控制字节指针为基准不同，不是矛盾 |
| OOM 降分辨率全链 | ✅（在 country-core，非 rhapsody） | `country-core/src/systems/render_loop/render_frame.rs::recompile_at_reduced_scale`（0x1404e8a3d..0x1404e9516）+ `reduced_scale` + 文案 + `reset_temporal_tables`/`clear_resource_pool`/`wait_idle_and_recycle_resources`/`set_reduced_state` 已按原顺序实现并有测试；rhapsody 侧的参与者只有 compile 返回 Err、`ResourcePool::clear`（0x140be76f0）、`wait_idle_and_recycle_resources`（raw，&self 由 raw 负责人改）。本轮在 `FrameSequence` 里补上"OOM 重试帧不 acquire/不 present"的顺序测试。❌ 未再向下读 CompileError 载荷的字段 |
| resource key（原按 u64 TypeId，现大量 `BTreeMap<String,…>`） | ⚠️ 判断 | rg 内用 `String` 键的是主机侧便利层（`VulkanGraphBackend` 的 `ResourceResolver`/`TemporalGpuResources` 按名字登记），原版图内资源是逻辑 u32 句柄；时态键已是 `Ustr`（与原 0x1404f6e50 `Ustr::from` 一致）。u64 TypeId 查表（`Components::get_resource_id` 哈希）是 ECS 世界侧的事，发生在 country-core/bevy，不在 rg。所以**没有改**：改成 u64 只会让主机 API 失去可读名，且无原版对应 |

## 工作包 5：pass 入口清单

✅ `reports/rg-pass-entries.md`：3 个 compile 实例（地址/Export/闭包内容/调用方）+ `render_world` 59 个调用点（地址、类型、PDB 名、证据里出现的 API 备注）。`rg/pass_entries.rs`：`CompileInstance`、`PassKind`、`PassEntryInfo`、`WORLD_GRAPH_PASS_ENTRIES`（59 项）、`trait PassEntry`（占位，无业务），2 个测试。
❌ "每个 pass 调用了 RenderPassApi 的哪些方法"：证据只到 pass 名/绑定层（`bind_common`、`render`、`dispatch_indirect_raw`、`clear_rg_buffer` 等），逐回调体未反编译；清单里该列空白 = 未读。加载图/展示图闭包内的入口只有 findings 级地址，没有逐项编号。

## 工作包 6：回放覆盖

✅ 新增 `crates/country-core/tests/rg_should_reset_temporal_oracle.rs`：`FrameConstants::should_reset_temporal`（原叶 0x140976880，`[+0x80] u32 == 0`）回放 `parallel-batch-06/temporal-lifecycle/native-oracle.json` mode2 的 1005 例，全部一致。

## 改动文件

- 新增：`crates/rhapsody/src/rg/frame.rs`、`crates/rhapsody/src/rg/pass_entries.rs`、`crates/rhapsody/tests/rg_frame_round2.rs`、`crates/country-core/tests/rg_should_reset_temporal_oracle.rs`、`reports/rg-pass-entries.md`、本报告。
- 修改（均为 `rg/**`）：`mod.rs`（注册 frame、pass_entries）、`executor.rs`（`PassPgoNameId` 驻留池 + 文档）、`vulkan.rs`（账本、GPU 时长、`last_known_gpu_frame_duration_ns`、`frame_ledger`）、`frame_throttle.rs`（`begin_frame_bounded`）、`import_export.rs`（`ImportedBuffer`）、`state.rs`（三个 compile 入口）、`compiled.rs`（`original_layout`）、`temporal.rs`（`original_layout`）。
- 另有 `attachments.rs`/`pass_bindings.rs`/`tests/state.rs` 在工作区里有他人/上轮的未提交改动，不是本轮所为。

## 新增测试

`rg::frame` 11 个单测（时长 Option 语义、panic、CAS、顺序、账本、同步槽、20ms 阈值、布局）、`frame_throttle` 1 个、`pass_entries` 2 个、`compiled::original_layout` 1 个、`temporal` 1 个、`import_export` 1 个、`tests/rg_frame_round2.rs` 4 个（含 lavapipe 会话账本）、country-core 1 个（1005 例）。

## 遗留

- `last_known_frame_duration_ns` 的 `[device+0x658]+0x3a8` 是 f32 timestampPeriod 的推断（名字未读出）；session 里用 `end_top_level_timestamp_query` 的 ns 结果代替读缓冲，没有真去读映射切片。
- 慢提交日志、pending 时间线 CAS 没有接到 session 的真实提交路径。
- `execute_compiled` 的 acquire/present 信号量两槽（`SwapchainSyncSlots`）没有接 `raw::swapchain`——那边只暴露 `SwapchainLeaseSync::into_sync_data`，按任务约定不改 raw；接入点是 `submit` 时把槽里的 acquire 等待（阶段 0x400）与呈现信号并入首/末批同步。
- `ResourceExportHandle<Texture>` 的 Rust 定义、CompileError 载荷、`RenderGraphRecord` 的命名与字段、Record 0x68B 的剩余字段、`validate_read_access`、`PassPgoStats::extend` 仍读不出。
- 需 raw 配合：`wait_idle_and_recycle_resources` 改 `&self`（session 里 `recycle_submission_engine` 的替代实现届时可删）。
