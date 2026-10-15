# raw-core 域：命令缓冲 / 提交 / 设备创建尾段 / 交换链 复现报告

图例：✅ 原版对齐 [证据地址]；⚠️ 候选/假设；❌ 读不出。验证：`cargo test --offline -p rhapsody -p tiny-glade -p country-core --all-features`（lavapipe）全绿，无 FAILED。

## 1. create_command_buffer（0x140bef4a0）
- ✅ 池条目 0x18B = `Vec<CommandBuffer>`（`{cap@0, ptr@8, len@0x10}`），**U#10 闭合：初值 `{0,8,0}` 即空 Vec，`8` 是 align=8 的悬垂指针**（0x140bed4c0 逐元素写 `0/8/0`）。
- ✅ 数组长度 = 队列族数；索引 = 队列族索引（0x140bef4a0 同一个 `param_5` 既索引 `dev+0x9c0` 又索引 `*(dev+0x658)+0x38` 的 0x1c 步长族属性数组）。
- ✅ 流程：锁 `+0x9b0`（parking_lot 字节锁）→ 越界 panic → 非空则 LIFO 弹出 → 空则 trace 日志 "Allocating a new command buffer"（0x142b15c50，门控 `*0x14362c3a8>4`）并 `CommandBuffer::new`（每个 CB 一个独立 `vkCreateCommandPool`(sType 39, flags=RESET) + 1 个主级 `vkAllocateCommandBuffers`，0x140bed050，CB 句柄@+0x48，族@+0x50，设备代号@+0x54）→ `dev+0x870` 非空则 debug_utils 命名。
- 落码：`command_buffer.rs::CommandBufferPool`（`new/take/recycle_completed/free_count`），`device.rs::GraphicsDevice::{create_command_buffer(family,name), recycle_command_buffer, free_command_buffers}`。原“每池互斥 + 新分配”候选保留为新建路径。
- ⚠️ 原版 CB 是 0x58B 值类型，重建用 `VulkanCommandBuffer`（持有 `Arc<VulkanCommandPool>`），入池前 reset（原版在 `begin` 里 reset，状态 ∈{0,3}）。旧 `VulkanCommandPool::allocate()` 调用方（rg/vulkan.rs、setup.rs、tg-text）未改，不走设备池。

## 2. reclaim / wait_idle / wait_idle_and_recycle
- ✅ `reclaim_command_buffer_resources`（0x140bf0cc0）：命令环队首项（0x60B，时间线@+0x58）`<= queue+0x80` 才出队；状态必须为 3，否则 `unreachable!()`（"internal error: entered unreachable code"，0x142b15fb0）；`cb.queue_tag==queue.tag`、tag≠0 时索引也相等（assert_eq）；资源移交后锁 `+0x9b0` 把 0x58B 压回 `pool[cb.family]`（越界 panic）。落码 `SubmissionEngine::reclaim_queue`（句柄回收）+ `CommandBufferPool::recycle_completed`（入池）。
- ⚠️ `reclaim_queue` 返回的是 `vk::CommandBuffer` 句柄投影（`submit.rs::Command`），因为 rg/executor 持有的不是 `VulkanCommandBuffer`；“资源移交（FUN_1418c7ae0）”未复现。队列自身标签 `+0x68` 在 `QueueState` 里没有字段（`rg/vulkan.rs` 以字面量构造，不能加字段），只核对“主/辅类别 + 辅助索引”。
- ✅ `wait_idle`（0x140bf8980）：debug 日志 → `+0x800` 锁握手（0→8 CAS）→ `(ash+0x98)(VkDevice)` = vkDeviceWaitIdle → 解锁。**`+0x800` 是 `RwLock<()>`（写锁位 8），`queue_submit` 取读锁**。重建用 `queue_lock: Mutex<()>`（`graveyard.rs::wait_idle_and_collect` 内取得），排他性不弱于原版；未改成 RwLock（`queue_lock` 被他人文件广泛使用）。日志 "Waiting for the device to be idle..."（0x142b16378，debug）已接入 `GraphicsDevice::wait_idle`。
- ✅ `wait_idle_and_recycle_resources`（0x140bf8e60）步骤：wait_idle → 退役排水(FUN_1418c0a70) → 锁 `+0x718` → `update_queue_pending_submissions(+0x720)` → 回收图形 Queue(+0x730) 与 Vec<Queue>(+0x7e8/+0x7f0, stride 0xb0) → 解锁。落码按此顺序，提交引擎经 `attach_submission_engine` 挂接（`Mutex<Option<SubmissionEngine>>`，未挂接则只做前两步）。
- ✅ **U#8 闭合：全局“退役哈希表” 0x1436308f0 就是 `parking_lot_core` 的全局 `HASHTABLE`**。证据：fibonacci 常数 `-0x61c8864680b583eb` = 0x9E3779B97F4A7C15；桶 0x40B = WordLock + 等待队列 head/tail + `FairTimeout{timeout, xorshift seed}`（`puVar2[3]=timestamp, (uint)puVar2[4]=seed 的 xorshift13/17/5`）；0x140bf8e60 与 mark_stale/on_focus 尾部的遍历是 **mutex 解锁慢路径的 `unpark_one`**（用被解锁的锁地址 `+0x718` 作 key）。属依赖库实现，按仓库规则不复现；与 swapchain/disposal 的业务逻辑无关（ALGORITHMS U#8“与 disposal 关联”的猜测不成立）。
- ✅ `+0x9b0/+0x718` 的字节锁 0→1、慢路径带 1e9 超时，= parking_lot `RawMutex`；重建用 `parking_lot::Mutex`。

## 3. SubmissionEngine（0x1418c3300 / 0x140bf0b90）
- ✅ 原“ring + cap-first 环”**就是 std `VecDeque`**（`{cap, ptr, head, len}`，下标 `idx>=cap → idx-cap`），故数据结构不用改；真正差异是语义：`Queue::update_pending_submissions` = `vkGetSemaphoreCounterValue(队列时间线@+0x88)` → **无条件**存 `+0x80` → 从**队首**弹出直到遇到第一个 `counter < 值` 就停（非 `retain`，旧代码对乱序值会多弹）。`update_queue_pending_submissions` 先图形队列、再按 Vec 顺序推进每个辅助队列。已按此重写（`update_pending_submissions(id, counter)`、`update_queue_pending_submissions_with(|q| counter)`、旧入口 `update_queue_pending_submissions(completed)` 保留并转发）。
- ✅ 旧实现的 `update` 会顺带丢弃 `pending_commands`；原版 update 不碰命令环，命令只在 reclaim 里出队，已纠正。`completed`（原 `+0x80`）存于引擎内部 `Vec<u64>`。
- ⚠️ 提交 wait/signal 模板 `dev+0xa00..0xa4f` 与 `queue_submit` 本体（0x140bfc750 反编译全文）：**未重新读通**。注意 0x140bfc750 实际是 `queue_family_index`（见下），不是 queue_submit；`queue_submit` 仍沿用已有 CPU 模型 `DeviceState::queue_submit`（r8 oracle）。❌ 未做。
- ✅ `queue_family_index`（0x140bfc750，机器码 `mov eax,0xad4; test dl,1; je; mov eax,0x3c; test [rcx+0x20],1; je panic`）：kind bit0 清 → `dev+0xad4`；置 → `dev+0x3c`，且要求 `dev+0x20` bit0，否则 panic（0x142b16c60，unreachable）。落码 `GraphicsDevice::queue_family_index_for(kind)`；“第二族”取首个附加队列的族 ⚠️（`+0x3c/+0x20` 的写入点仍未在创建函数里找到，U#12 对“实际提交走哪条队列”仍未证实）。

## 4. 设备创建尾段（0x140bf2560 段 C–G）
- ✅ 字符串（0x142b160a0..0x142b16378）：Created a Vulkan device / "device_memblock_size: {}MB, host_memblock_size: {}MB" / "Device memory properties: {:?}" / "Device memory budget: {:?}" / "gpu-allocator creation"（puffin/错误上下文，0x16 字符）/ "Initialized gpu-allocator" / "crash tracking buffer" / "Backend device fully initialized." / "vkCreatePipelineCache" / "Waiting for the device to be idle..."。日志级别门控按反编译：`>4`=trace（Created、Initialized），`>3`=debug（memblock/properties/budget）。用 `log` 宏（crate 已依赖），target `rhapsody::raw::device`。
- ✅ 顺序：分配器创建 → `Initialized gpu-allocator` → `crash tracking buffer`（`create_buffer_impl`，BufferDesc size=4，名字长 0x15）→ `vkCreatePipelineCache`（sType 17，空 initial data，flags 0；失败上下文 "vkCreatePipelineCache"）→ 返回。落码 `GraphicsDevice::finish_creation`（只在 `GraphicsDevice::create` 路径；`from_device` 保持原样，不建缓存/缓冲，避免改变现有测试的分配统计），新增 `pipeline_cache()/crash_tracking_buffer()` 访问器。
- ⚠️ memblock 大小：原版 = 0x140c00530（按堆求和）÷ `(+0x224 字段>>1)`，向上取整到 4 MiB（`(x+0x3fffff+(x==0)) & ~0x3fffff`）；除数字段 `+0x224` 的身份未还原，故日志里写的是重建**实际使用**的 gpu-allocator 默认块大小（256/64 MB），不是原公式结果；suballoc.rs 非本人文件，未改块大小。
- ⚠️ "Backend device fully initialized." 在 0x140bf2560 的 0x2a000 字节范围内没有 rip 相对引用（扫描 lea），真实调用点未定位，放在 `finish_creation` 末尾（⚠️位置）。“Device memory budget” 仅在启用了 `VK_EXT_memory_budget` 时输出。
- ✅/记录：`GraphicsDevice` 真实字段布局表已写入 `device.rs` 的 `GraphicsDevice` 文档注释（`+0x1a/+0x20/+0x3c/+0x40/+0x78/+0x188/+0x640/+0x648/+0x658/+0x668/+0x718/+0x720/+0x800/+0x870/+0x9b0/+0x9c0/+0x9f8/+0xa00/+0xa50/+0xad4`），未改成原 ABI。`VulkanDevice` 新增 `enabled_extensions: Vec<String>`（记录 `ppEnabledExtensionNames`）。

## 5. 交换链
- ✅ `validate_swapchain_and_acquire_image`（0x140bfd9d0）已按原分派落到 `swapchain.rs`（返回 `Result<Option<AcquiredSwapchainFrame>>`，`None` = 外层判别 7 “本帧无图像”）：租约（含 SUBOPTIMAL）原样产出；stale → debug "Re-creating a stale swapchain (…)."；retired(OUT_OF_DATE) → debug "Re-creating a retired swapchain."（0x142b16d10/0x142b16d70）→ 重建 → 再取一次：新链 stale → warn "The newly-created swapchain was reported stale: …" 返回 None；新链 retired / 其它错误 → 原版 panic（"Newly created swapchain claims to have been retired"、"Vulkan swapchain error after re-creation:"、"Vulkan swapchain re-creation error:"），重建改为返回 `Err`；首次获取其它错误原版 panic "Vulkan swapchain error:"，同样改 `Err`。⚠️ tag↔vk 码映射（2=NOT_READY/TIMEOUT→None、4=OUT_OF_DATE=retired、默认=stale）是由字符串与分支结构推断，未逐个核对 `acquire_next_image`(0x140c0eb40) 返回枚举。`managed/swapchain.rs` 同步（重建后 `allocate_shared` 换入新托管句柄）。旧签名无外部调用者。
- ✅ `Swapchain::format`（`+0x18` u32）新增方法；`any_present_successful`（`+0x1c` u8，`present` 里 `|= result`）与 `TrackedSwapchainTextureLease::{image,into_present_source}` 已存在且语义一致（into_present_source 取 `+0x08` 并 drop 两个子句柄）——核对，无改动。
- ✅ `RenderStats::snapshot_and_reset`（0x140c02da0 机器码：3×`xchg [rdx+0/8/0x10]` 后写返回槽）：`stats.rs` 改成 3 字段 `counter0..2`（字段语义名未证实），4 字段旧版移除（无外部调用者；country-core 的 `snapshot_render_stats` 仅为 trait 方法）。

## 6. 设备微函数
- ✅ `device_uuid`（0x140bf84c0）= 复制 `*(dev+0x658)+0x690` 的 16 字节（= pipeline_cache_uuid）。✅ `physical_device`（0x140bf8950）= `*(dev+0x658)+0x10`。✅ `debug_utils`（0x140bf8960）= `dev+0x870` 非空才返回（`Option`）→ 新增 `debug_utils_option()`。✅ `min_subgroup_size`（0x140bf8fa0）= `Option<u32>`（`[+0x40]` 标志 / `[+0x44]` 值，与 country-core taa.rs 注释一致）→ 新增 `min_subgroup_size_option()`（`VK_EXT_subgroup_size_control` 启用时才有值），数值版 `min_subgroup_size()` 保持旧返回（无值→1）。
- ✅ `refresh_memory_budget`（0x140bf8f80：`cmp byte [rcx+0x1a],0; je ret; rcx=*(dev+0x658)+0x10; jmp 0x140c01620`）→ `GraphicsDevice::refresh_memory_budget`：门（`memory_budget_enabled`）→ 委托 `PhysicalDevice::refresh_memory_budget`；PhysicalDevice 只能在 init 枚举里构造，故加 `attach_physical_device_info`/`physical_device_info`，由 init 路径挂接（init.rs 非我的文件，需要他们调用；未挂接返回 `Ok(None)`）。

## 改动文件
`crates/rhapsody/src/raw/{device.rs, command_buffer.rs, submit.rs, swapchain.rs, stats.rs, managed/swapchain.rs}`；新增测试 `crates/rhapsody/tests/raw_core_commands_device.rs`（10 用例：stats 三字段、update 队首弹出语义、辅助队列顺序、reclaim 完成值/unreachable、池空/越界、LIFO 复用、微函数、创建尾段）。
未改：semaphore.rs / profiler.rs / debug.rs / barrier.rs / graveyard.rs / dynamic_constants.rs（本包范围内无必改项）。

## 遗留 / 需要他人配合
1. init.rs 在创建 `GraphicsDevice` 后应调用 `attach_physical_device_info(Arc<PhysicalDevice>)`，`refresh_memory_budget` 才真正委托；rg/vulkan.rs 创建引擎后可 `attach_submission_engine`，`wait_idle_and_recycle_resources` 才推进/回收队列。
2. 旧的 `VulkanCommandPool::allocate()` 调用方（rg/vulkan.rs、setup.rs、tg-text）可改走 `GraphicsDevice::create_command_buffer` 以真正使用设备池。
3. ❌ `queue_submit` 本体与 `dev+0xa00..0xa4f` 模板未读通；❌ `dev+0x3c/+0x20` 写入点；❌ memblock 公式的 `+0x224` 字段；❌ "Backend device fully initialized." 调用点；❌ 回收的“资源移交”（FUN_1418c7ae0）。
4. `+0x800` 实为 RwLock，重建仍是 Mutex（行为更保守）。
