# wave2：queue_submit 本体 / 设备级 setup 槽 / reclaim 资源移交

证据根：`producer-closure/a7-queues-raytracing/{0x140c04e20,0x140bf84e0,0x140bf0cc0,0x140bf2560}.c`、`producer-closure/n1-device/{0x140c03fa0,0x140c04840}.c`、`a4-managed/0x1418c0a70.c`、`g8-semantics/decomp/0x1418c4100.c`、`*/0x1418c7ae0.c`；字符串用 `pe_readonly.PE(tiny-glade.rne).read(va,n)` 读出。

## 0. 地址更正
- `0x140bfc750` = `queue_family_index`（上一轮已纠正），**`GraphicsDevice::queue_submit` = `0x140c04e20`**（11047B，字符串 `rhapsody::raw::submit::<impl GraphicsDevice>::queue_submit`、`vkQueueSubmit`(0x142b18800,13B)、`main queue_submit failed`）。
- 签名（反编译）：`queue_submit(out: *mut SubmissionIndex, dev, queue_kind: u32 /*bit0=辅助*/, queue_index: u32, req: *const Request /*0xd8B*/) -> SubmissionIndex{queue:(u32,u32), value:u64}`。返回值 `(kind,index)` + `最后一个时间线值`（= 分配计数 - 1，0 则 panic，字符串 0x142b18840）。
- **`dev+0xa00..0xa4f` 不是 wait/signal 模板。** 它是设备级 **setup 命令缓冲槽** `Mutex<Option<CommandBuffer>>`（`+0x9f0` 字节锁、`+0x9f8` 状态、`+0xa00..` 载荷，句柄在 `+0xa40`），wait/signal 并不在设备上。上一轮对该区域的命名要改。

## 1. queue_submit 逐步骤

| # | 步骤 | 状态 |
|---|------|------|
| 1 | 入口（调试 span，`func_0x140c16a00` 开关；文件名 `…/submit.rs`） | ✅ 与行为无关，未落码 |
| 2 | `FUN_140bf0720(dev)`：读 `dev+0xa88` 读写锁里的“设备丢失”标志（读锁 CAS +0x10，`cmp rax,-0x10`/bit3 判写者）。为真则 `FUN_140befd60(dev)`（崩溃处理）+ 日志（`rhapsody::raw::submit`，0x142b18990）+ `FUN_140d63150(1)`（退出） | ✅ 机器码已读；落码 ❌（`device_lost` 状态在 device.rs，未接） |
| 3 | 取 `dev+0x718` 字节自旋锁（CAS 0→1，超时 1e9ns） | ✅ 已有 `queue_lock`/本类 `&mut self` 独占等价 |
| 4 | 拷 0xd8B 请求；`FUN_140bf84e0(dev)` = `get_setup_cb_submit_info`：锁 `+0x9f0`；`+0x9f8` 置 4 取旧值；旧值≠4 则 `vkCmdPipelineBarrier`（`dev+0x3e0`；stage `0x10000`→`0x10000`，`VkMemoryBarrier(0x2e){0x18000,0x18000}`）、状态必须 1（else `unreachable!`）、`vkEndCommandBuffer`（`dev+0x2c8`）非 0 → unwrap panic，状态置 2，造 0x90B 记录 | ✅ 落码 `SetupCommandSlot::take_for_submit` |
| 5 | 拼 0x90B 记录数组（记录 0x90B：内含 cb Vec、信号量 Vec 等；setup 记录与请求内一条 Option 记录压入同一个小 Vec 后再与 current 命令展开拼接） | ⚠️ setup 相对 prior/flushed/current 的**最终顺序未逐指令证实**；落码取 setup 最先 |
| 6 | `FUN_1418c3300(dev+0x720, dev+0x78)` = `update_queue_pending_submissions`（先于时间线分配） | ✅ 已有；新增 `SubmissionEngine::queue_submit` 保证这一顺序 |
| 7 | 分配：`dev+0x7f8 += 总命令数`（u32 全局计数）；`FUN_1418c3b80` 把 `[旧值,新值)` 压入所选队列时间线环（`dev+0x730` 图形，或 `dev+0x7e8 + idx*0xb0`，越界 panic `FUN_1428d9518`）；重算 `dev+0x720/0x728`（全队列环首最小值）；profiler 事件 0xd，载荷 `dev+0x7f8 - 最小值` | ✅ 已有 + 新增 `Outcome.in_flight` |
| 8 | 对每条命令：状态必须 2（else `FUN_1428d9390` panic）；队列标签 == `kind` 且（kind≠0 时）索引相等（assert_eq）；状态置 3；记录里压入 `queue+0x88` 时间线信号量句柄与值；0x60B 项（时间线 `@+0x58`）压入 `queue+0x20` 命令环 | ✅ 已有（`submit.rs` 原实现正确） |
| 9 | 构造 `VkSubmitInfo`/`VkTimelineSemaphoreSubmitInfo`（`FUN_1418b8f20`…）；**取 `dev+0x800` 读锁**（`val< -0x10 && !bit3`，CAS +0x10）→ **取队列字节锁 `Queue+0x70`**（`puVar24+0xe`）→ `(dev+0x88)(Queue+0x78, submitCount, pSubmits, fence=0)` = `vkQueueSubmit`，**fence 为 NULL** → 解锁 | ✅ 机器码；GPU 路径落码 ❌（见 §4） |
| 10 | 返回码非 0：`FUN_140bf0860` 转换，结果标签 `0xb` 视为可容忍，其余 panic（`main queue_submit failed`） | ✅（已有 `convert-error` 模型） |
| 11 | `FUN_1418c0a70(dev+0x648, dev)`：取走 `dev+0x648` 互斥内的延迟操作 Vec（16B 项，按 `+0xe` 字节分派，反编译对 jumptable 截断）并 `FUN_140bf92f0(dev)` 销毁 | ⚠️ 作用推断（延迟资源操作/清理），jumptable 分支未读出 |
| 12 | `FUN_140bf0cc0(queue, dev)`：reclaim（见 §3） | ✅ |
| 13 | 解锁 `dev+0x718`，写返回值 | ✅ |

**慢提交 20ms 阈值**不在 `queue_submit`：位于 `submit_pending_cbs`（`0x140bebdf0` 反编译尾部）。该分支仅当 recorder 的 debug 字节（`param_1+0x31*8`）为真时才进入：日志 → 计时 → `FUN_140bf8980`（`wait_idle`）→ 若 `elapsed_ns > 20000000`（`20000000 < uVar9`）或计时函数 `FUN_140d63910` 返回非 0 → warn 日志（0x142b14878）+ `FUN_140d55980(2,0)`；再 `wait_idle`。即是 “调试同步模式”，不是常规提交路径。✅ 阈值与结构；落码 ❌（`rg/frame.rs`/executor 不属于该调试路径，未接）。

## 2. dev+0x9f0..0xa4f / +0x3c / +0x20

- ✅ 布局（见 `submit.rs::SetupCommandSlot` 文档）：`+0x9f0` 字节锁、`+0x9f8` 状态（1 录制/2 结束/3 提交/**4 = None**）、`+0xa10/0xa18/0xa20` 所有者 Vec（cap/ptr/len；覆盖旧值时 dealloc `cap<<3`）、`+0xa28` 另一 Vec（`FUN_1405df600` drop）、`+0xa40` 句柄。
- ✅ 创建写入点：`GraphicsDevice::create`（0x140bf2560）尾段，反编译第 2324 行 `stack0x22f0=0; stack0x22f8=4` → 第 2447–2448 行 `puVar62[0x13e]=…22f0; puVar62[0x13f]=…22f8` → 初值 = **{锁 0, 状态 4(None)}**，其余 0x50B 未初始化。
- ✅ 惰性创建：`0x140c03fa0`、`0x140c04840`（setup 访问器）：锁 → 若状态==4：`create_command_buffer(dev, "setup cb"(0x142b18788), 族=dev+0xad4)`（`FUN_140bef4a0`）+ `begin`（`FUN_140bed230`）→ 写槽；0x140c04840 里还把 buffer-to-buffer 拷贝记入（`dev+0x370` = `vkCmdCopyBuffer`）；覆盖旧值时旧状态必须 0 或 3，否则 unreachable（0x142b12b58, 0x6e 长）。
- ❌ `dev+0x3c` / `dev+0x20` bit0 的写入点：create 反编译里 `puVar62+0x3c` 的写入（第 1829 行）是对一块 `.rdata` 常量（0x142b14e90/98）的拷贝，且该 `puVar62` 在该处是别的临时结构，不能当作 `GraphicsDevice+0x3c`。`+0x20` 同样没有对应的设备级写入。栈复用严重，Ghidra 伪代码不足以确认；**未证实**，保持上一轮 `queue_family_index_for` 的 ⚠️ 结论。需要 `.asm`（create 的 .asm 文件为空，需重新导出）或 Frida/调试器 trace。

## 3. reclaim 资源移交 FUN_1418c7ae0

✅ 读通：`FUN_1418c7ae0(queue+0x40, cb_seq = 命令项+0x08, dev)` = **按 cb_seq 严格顺序退役**：结构 `{VecDeque<u64>(cap,ptr,head,len)@+0x00, next@+0x20}`；
- `seq <= next` → panic（0x142d06fd8，长 0x29）；
- `seq == next+1`：`next=seq`；profiler 事件 9（`0x1e, seq`）；`FUN_1418c4100(&out, dev+0xa50, seq)` = `SequentialDisposal::dispose_in_order`（取出该序号桶）；`FUN_140bf92f0(dev,&out)` 销毁；弹队首空洞；循环“队首非 0 则弹出、`next+=1`、断言连续、同样退役”；
- `seq > next+1`：暂存于下标 `seq-next-1`（不足补 0；非 0 槽重复 → panic）。
落码：`graveyard.rs::InOrderRetire`（纯保序逻辑）+ `SubmissionEngine::reclaim_command_buffer_resources(id) -> ReclaimOutcome{handles, retired_sequences}`（每队列一个 `InOrderRetire`）。
⚠️ “桶销毁”：原版桶按 cb_seq 归属、由 `dev+0xa50` 的 `SequentialDisposal` 持有；重建的 `Graveyard` 是 epoch 制（语义等价于“序号 ≤ 完成线才销毁”），未改成按 cb_seq 桶（需要 device/bury 的归属改造，且 `Doomed` 入场点分散在非我拥有的文件里）。`retired_sequences` 暴露给调用方，可在桶化时直接驱动销毁。
注：未把 `InOrderRetire` 塞进既有 `reclaim_queue`（既有测试用 `sequence=timeline` 的重复/乱序值，会触发原版同款 panic），而是新增独立入口，`reclaim_queue` 行为不变。

## 4. 与 retire_timeline 的冲突与迁移方案（未删除任何东西）

冲突点（真实 GPU 路径 `command_buffer.rs::submit_to` + `graveyard.rs`）：
1. 原版 `vkQueueSubmit(…, fence=NULL)`，完成靠**每队列时间线**（`Queue+0x88`，每条命令 signal 一个全局单调值）；重建是 **每次提交建一个 fence**（`PendingCommand.fence`）+ **设备级 `retire_timeline`**（每次提交一个 epoch）。
2. 原版 `dev+0x800` 是 RwLock（提交取读锁，`wait_idle` 取写锁），队列自己另有字节锁 `Queue+0x70`；重建只有 `queue_lock: Mutex<()>`，提交互斥且串行（更保守）。
3. 原版销毁用 `cb_seq` 桶 + 队列时间线完成值（reclaim 路径）；重建用 epoch + `retire_timeline`。
迁移方案（需 device.rs/init.rs 配合）：
- device 创建时为每个队列创建一个时间线信号量（`TimelineSemaphore::new(device, 0)`；原版在 `create` 内只创建**一个**时间线 `0x1418c8710`，且 `dev+0x78` 起 `Queue` 里 `+0x88` 指向它——即全局一个，非每队列一个，此点 ⚠️ 与“每队列”的措辞需再核：`0x1418c8710` 唯一调用点在 create，被各 Queue 共享还是各拷一份句柄未证实）。
- `submit_to` 改为走 `SubmissionEngine::queue_submit`：取读锁→队列锁→`vkQueueSubmit(fence=NULL)`，值由 `DeviceState::next_timeline` 分配；`PendingCommand` 的 fence 等待改为 `TimelineSemaphore::wait(value)`。
- `Graveyard` 的 epoch 对应 `cb_seq`；`bury` 登记时记录当前 `cb_seq`，`reclaim` 返回的 `retired_sequences` 驱动销毁。
- 在以上全部接通且 `device.rs` 的 `idle_if_in_flight_locked` 等宿主同步改为等队列时间线之前，**不能**删 `retire_timeline`（现有宿主读回、swapchain 等依赖它），故本轮保持不动。

## 5. 改动文件
- `crates/rhapsody/src/raw/submit.rs`：新增 `SetupCommandSlot`/`SetupTakeError`/`SETUP_*` 常量/`ReclaimOutcome`；`DeviceState::queue_submit_with_setup`（原 `queue_submit` 委托，API 兼容）；`Outcome.in_flight/setup_submitted`；`SubmissionEngine::{reclaim_command_buffer_resources, queue_submit}` + 每队列 `InOrderRetire`。
- `crates/rhapsody/src/raw/graveyard.rs`：新增 `InOrderRetire`。
- `crates/rhapsody/src/raw/command_buffer.rs`：仅 `submit_to` 文档（差异说明）。
- `crates/rhapsody/src/rg/frame.rs`、`semaphore.rs`：未改（无对应原版行为需要落地）。
- 新测试 `crates/rhapsody/tests/wave2_queue_submit.rs`（10 用例：setup 槽空/屏障先于状态检查/状态错误/end 失败；InOrderRetire 顺序+缺口+重复；setup 最先与时间线分配、in_flight；update 先于分配、尾部回收与序号退役；失败不回滚）。
- 验证：`-p rhapsody -p country-core --all-features` 全绿（含 `original_oracle*`、`raw_core_commands_device`、`wave2_structure`）。workspace 其余失败（`asset`/`asset-pipe`/`shader-compiler`/`tg-text` 若干）为缺少原作资源文件所致，与本改动无关（未触碰这些 crate）。

## 6. 对 device.rs / init.rs 的配合需求（补丁文本）
1. `GraphicsDevice` 增加字段 `setup_slot: Mutex<submit::SetupCommandSlot>`，初值 `SetupCommandSlot::default()`（对应 create 尾段 `{0,4}`）：
```rust
// device.rs（GraphicsDevice 字段，对应 +0x9f0..+0xa4f）
pub setup_slot: parking_lot::Mutex<crate::raw::submit::SetupCommandSlot>,
// 构造处
setup_slot: parking_lot::Mutex::new(Default::default()),
```
2. `queue_submit` 的设备侧封装（若 device.rs 持有 `SubmissionEngine`）：
```rust
let setup = {
    let mut slot = self.setup_slot.lock();
    slot.take_for_submit(
        |cb| unsafe { /* vkCmdPipelineBarrier(cb, ALL_COMMANDS, ALL_COMMANDS, 0, &[MemoryBarrier{src=dst=MEMORY_READ|MEMORY_WRITE}]) */ },
        |cb| unsafe { /* vkEndCommandBuffer(cb) as i32 */ },
    ).expect("setup cb")
};
let (out, reclaimed) = engine.queue_submit(id, setup, input, |q| /* get_semaphore_counter_value(q.timeline_semaphore) */, backend);
```
3. `wait_idle` 取写锁、`queue_submit` 取读锁：把 `queue_lock: Mutex<()>` 换成 `RwLock<()>`（`queue_submit` 读、`wait_idle`/graveyard 写）——会影响 `graveyard.rs` 里 `queue_lock.lock()` 的调用（我这边的 `collect_garbage`/`wait_idle_and_collect` 需要同步改为 `.write()`，等 device.rs 动手时一起改）。
4. 设备创建尾段里设备丢失标志（`+0xa88` RwLock）若 structure agent 已有字段，`queue_submit` 入口按 §1 步骤 2 先检查。

## 7. 未读通 / 诚实清单
- ❌ `dev+0x3c`、`dev+0x20` bit0 写入点。
- ⚠️ setup 记录相对其它记录的最终拼接顺序。
- ⚠️ `FUN_1418c0a70` 延迟操作 jumptable 分支内容、`FUN_140bf92f0` 的销毁细节。
- ⚠️ `dev+0x3e0` / `+0x2c8` / `+0x88` / `+0x370` 对应 Vulkan 函数：由参数形态推断（`vkCmdPipelineBarrier`、`vkEndCommandBuffer`、`vkQueueSubmit`、`vkCmdCopyBuffer`），未对 ash `DeviceFn` 表偏移做逐项核对。
- ⚠️ 时间线信号量“全局一个 vs 每队列一个”。
