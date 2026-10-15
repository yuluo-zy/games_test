# rhapsody raw-core 域 算法深挖（2026-10-15）

- 范围：`skeleton/B/crates/rhapsody/src/raw/{device,buffer,texture,swapchain,shader,command_buffer,barrier,submit,descriptor_set,binding,render_pass,framebuffer,dynamic_constants,debug,profiler,sampler,semaphore,stats,surface,temp_list}.rs` —— 184 compiled / 95 有地址。
- 基线：B patched EXE；原始指令 oracle 为发行 RNE。机器码为权威。
- 标注：**O**=机器码/PDB 实测；**I**=推断；**U**=未知。NotFound 反编译（82B 错误文件）不作为 O 证据。
- 反编译来源：本域 `decomp/decomp-<va>.txt`（95 份，dbid `tg_rh2`）+ `evidence/render-recovery/producer-closure/{n1-device,n2-texture-buffer,a6-allocator-swapchain,a5-descriptor-shader,…}/0x*.c` + CALL-CHAIN `decomp/*.c`。
- 对照基准：`reconstruction/rhapsody-rebuild/src/raw/**`（候选实现，已过 lavapipe 验证；本任务不改其源码）。

---

## 1. 模块总览

### 1.1 文件→职责

| 文件 | 职责 | compiled | 关键点 |
|---|---|---|---|
| `device.rs` | 设备创建/队列/回收/屏障总入口 | 17+ | `GraphicsDevice` 真实结构；ash 函数表内联于 `+0x78`；VkDevice @ `+0x640` |
| `buffer.rs` | 缓冲创建/上传/地址/映射；`BufferDesc` | 21 | **Buffer 40B 句柄**；create_buffer_impl 0x48B 结果 |
| `texture.rs` | 纹理创建/上传/描述符 | 26 | TextureDesc 32B / TextureViewDesc 0x3cB；`get_image_create_info` desc→VkImageCreateInfo |
| `swapchain.rs` | 交换链获取/呈现/失效/租约 | 19 | Swapchain 句柄 flag=2；mark_stale 走全局退役哈希表 |
| `shader.rs` | 描述符布局/管线/附件描述 | 13 | 布局 0x58B；RasterPipelineOpts 29 setter；map_vertex_attribute_format |
| `command_buffer.rs` | 命令缓冲池/录制/序号 | 8 | CommandBuffer 0x58B；SeqTracker CAS |
| `barrier.rs` | 同步作用域；三类屏障 | 9 | SynchronizationScope1 布局；image_aspect_mask_from_format |
| `submit.rs` | 队列提交/时间线推进 | 6 | Queue 0xb0B；时间线信号量弹出环 |
| `descriptor_set.rs` | 持久描述符集 | 3 | create_persistent_descriptor_set |
| `binding.rs` | 绑定管线/描述符 | 8 | bind_descriptor_set 4 游戏闭包实例 |
| `render_pass.rs` | 渲染通道缓存 | 3 | device+0x188 HashMap<RenderPassDesc,RenderPass> |
| `framebuffer.rs` | 帧缓冲创建 | 1 | create_framebuffer |
| `dynamic_constants.rs` | 动态常量环形分配 | 8 | 256KB chunk 对齐游标；双绑定产出 |
| `debug.rs` | 调试标签 | 2 | begin/end_label |
| `profiler.rs` | GPU 时间戳帧 | 5 | TimedFrame 0x18B/项；0x2000B 时间戳缓冲 |
| `sampler.rs` | 内置采样器 | 2 | create_samplers |
| `semaphore.rs` | 时间线信号量 | 1 | sType 9 + timeline 链 |
| `stats.rs` | 渲染统计快照 | 1 | snapshot_and_reset |
| `surface.rs` / `temp_list.rs` | winit 表面 / 临时列表 | 2 | Surface::create / TempList::add |

### 1.2 核心数据结构布局（O 标注来自多函数交叉印证）

**`Buffer` = 40B（0x28）句柄**（O：import_unowned `0x140bd4fc0`、device_address `0x140bd4c00`、mapped_slice_mut `0x140bd4cd0`、set_buffer_debug_name `0x140beec30`、update_buffer_bytes `0x140beefa0`）：

| 偏移 | 类型 | 含义 |
|---|---|---|
| `+0x00` | u64 | 句柄/tag（create_buffer_impl 写常量 `0xb`；游戏侧随 TypedStorageBuffer 整块搬运） |
| `+0x08` | * | `inner` = gpu-allocator 式分配对象指针（`+0x10`） |
| `+0x10` | u32 | 分配内偏移（校验：须 == `inner+0xc`） |
| `+0x14` | u16 | generation（须 == device context `+0x648 → +0xa50` 的设备代号） |
| `+0x16` | u8 | flag（`1`=有效/已就绪；device_address/mapped 均要求 ==1） |
| `+0x18` | u64 | size（`update_buffer_bytes` 用它做越界校验） |
| `+0x20` | u64 | （mapped 基址 / device_address 缓存，import_unowned 整拷） |

`inner` 分配对象：`+0x0c` u32 offset、`+0x20` u64 容量（update_buffer_bytes 边界用）、`+0x30` u64 device_address（vkGetBufferDeviceAddress 缓存）、`+0x38` 嵌套内存块（`0x140c113b0` 取 mapped ptr、`0x140c02280` 取块大小）、`+0x7c` u8 memory_properties（`0x2`=HOST_VISIBLE）。

**`BufferDesc` = 16B**（O：n2 + `alignment 0x140bd4f20` + `create_buffer 0x140c03460`）：`{u64 size@0, u32 usage@0x08, u16 alignment@0x0c（<0x10000）, u8 memory_location@0x0e（1=GpuOnly/2=CpuToGpu/3=CpuToGpuLowPrio/4=GpuToCpu）}`。rebuild 同名枚举 `from_raw` 1..=4 ✓。

**`TextureDesc` = 32B**（O：n2 + `array_elements 0x140bd5540` + `get_image_create_info 0x140bd5ed0`）：`{u32 usage@0, u32 flags@4, u32 format@8, u32 width@0xc, u32 height@0x10, u32 depth@0x14, u32 array_elements@0x18, u8 mip_levels@0x1c, u8 sample_count@0x1d, u8 kind@0x1e}`。`kind`：0=1D/1=1DArray/2=2D/3=2DArray/4=3D/5=Cube(layers=6)/6=CubeArray(n×6)。`array_elements` 校验掩码 `0x4a`（仅 kind∈{1,3,6} 允许 layers>1）。

**`TextureViewDesc` = 0x3cB**（O：country-core `rhapsody_types.rs`，Ghidra 无函数体但消费端/构造器已核）：`{Option<u32> view_type@0, usage@8, format@0x10, aspect_mask@0x18, level_count@0x20, layer_count@0x28, u32 base_mip_level@0x30, u32 base_layer@0x34, bool bind_individual_mips@0x38}`。`Ref::bind_view` 整拷 0x3cB 进 RenderPassBinding。

**`GraphicsDevice`**（O：n1-device findings，三重交叉）：`+0x78` 内嵌 ash 设备函数表（`+0x78+0x108`=vkCreateBuffer、`+0x78+0x5c8`=VkDevice），**VkDevice 句柄 @ `+0x640`**。分派槽：`+0x270` vkCreateFramebuffer、`+0x280` vkCreateRenderPass、`+0x298` vkCreateCommandPool、`+0x2b0` vkAllocateCommandBuffers、`+0x2c0` vkBeginCommandBuffer、`+0x2c8` vkEndCommandBuffer、`+0x2d0` vkResetCommandBuffer、`+0x328` vkCmdBindDescriptorSets、`+0x3e0` vkCmdPipelineBarrier、`+0x268` vkUpdateDescriptorSets、`+0x898/+0x8a0` vkCmdBegin/EndDebugUtilsLabelEXT。字段：`+0x648` 上下文/注册表（`→+0xa50` u16 设备代号）、`+0x658` 配置管理器、`+0x668/+0x670` 每队列序号数组/个数、`+0x678` 全局分配器（锁 `+0x10`）、`+0x720` SubmissionEngine、`+0x730` 图形 Queue、`+0x7e8/+0x7f0` Vec<Queue>(stride 0xb0)、`+0x870` Option<debug-utils>、`+0x810/+0x818/+0x848` 崩溃锁/4096 项 marker 环/写计数、`+0x9b0` 池锁、`+0x9c0/+0x9c8` 池数组/个数、`+0x9f0` setup 锁、`+0x9f8..0xa4f` **内联 setup 命令缓冲记录（0x58B）**、`+0xa50` CommandBufferSeqTracker、`+0xad4` 命令缓冲数/主队列族。`device_uuid/physical_device/debug_utils/min_subgroup_size/refresh_memory_budget`（0x140bf84c0/8950/8960/8fa0/8f80）Ghidra 无函数体（NotFound），仅名可辨。

**`CommandBuffer` = 0x58B**（O：new `0x140bed050`/begin `0x140bed230`/end `0x140bed3d0` + n1）：`{u32 state@0（0=ready/1=recording/2=ended/3=recycled）, +0x08 u64 seq, +0x10 u32 flags, +0x14 u32 queue_index, +0x48 VkCommandBuffer, +0x50 u32 queue_family, +0x54 u16 device_tag, +0x56 u8}`。begin 从 `dev+0x668[uVar6]` 取自增序号（`(flags&1)` 选主/辅队列），end 走 `dev+0x2c8`。

**`Queue` = 0xb0B**（O：n1 + `update_pending_submissions 0x140bf0b90` + `with_raw 0x140c03d70`）：`+0x08` 提交环指针、`+0x10` 头、`+0x18` 待决数、`+0x70` 锁、`+0x80` 最近时间线值、`+0x88` 时间线 VkSemaphore。

**`SynchronizationScope1`**（O：`issue_pipeline_barrier 0x140bee030`）：`{imageBarriers: *@+8 + count@+0x10, bufferBarriers: *@+0x20 + count@+0x28, memoryBarriers: *@+0x38 + count@+0x40, srcStage@+0x48, dstStage@+0x4c}`。

**`DynamicConstants`**：`{buf_ptr@0, device@+8, 游标@+0x10, 当前buf@+0x18, 最小对齐@+0x20, 缓冲句柄@+0x28, chunk@+0x30, offset@+0x38, gen@+0x3c u16, flag@+0x3e u8}`（O：allocate `0x140c02740`）。`DynamicConstantsChunk` 0x48B：`{device_ptr@0, size@8, inner+offset@+0x10, size@+0x18, buffer@+0x20, chunk@+0x28, offset@+0x30, gen@+0x34, flag@+0x36, 0@+0x38, offset@+0x40}`。

**`Swapchain` 句柄**（O：on_focus `0x140c0e6b0`/mark_stale `0x140c0e950`/present `0x140c0ef80`）：`{inner*@+8, u32 offset@+0x10, u16 gen@+0x14, u8 flag@+0x16（==2=swapchain）, u8 present-status@+0x1c}`；inner `+0x10` 双用途（锁 + extent 原子）、`+0xc9` u8 焦点标志。

**`RenderPassDesc` / 缓存**（O：skeleton render_pass.rs + CALL-CHAIN §4#14）：`device+0x188` 为 `hashbrown::HashMap<RenderPassDesc, RenderPass, RandomState>`（PDB 证）；键 = `{ManagedHandle@0, u32@0x18}` 双字段 + 内联 `ArrayVec<RenderPassAttachmentDesc, 8>`（arrayvec 0.7.4，cap 8）。`RenderPassAttachmentDesc` = 0x10B：`{format, samples, clear_input, depth_stencil}`（方法面：`new/clear_input/sample_count`，shader.rs line 690/704/714；无 depth_stencil 命名，rebuild 推断字段——见对照）。

### 1.3 处理链

```
【缓冲创建】create_buffer(0x140c03460)
  ├─ 若带初始数据：desc.usage |= TRANSFER_DST(2)
  ├─ create_buffer_impl(0x140bee200)
  │    ├─ ash vkCreateBuffer（+0x78[0x21]，sType=10 区）
  │    ├─ vkGetBufferMemoryRequirements（+0x78[0xe]）
  │    ├─ 对齐 = max(req, desc.alignment, usage&0x400?0x40:0x10)
  │    ├─ allocator 自旋锁 + 1s 超时（FUN_1428cd210）→ gpu-allocator 式分配
  │    └─ vkBindBufferMemory（+0x78[0xc]）；结果 0x48B {tag@0=0xb,…}
  ├─ 有初始数据：建 1 项暂存（size=len, memory_location=3）+ memcpy
  └─ with_setup_cb(0x140c04840) 录 copy 进 setup 命令缓冲

【缓冲上传】update_buffer_bytes(0x140beefa0)
  ├─ 校验 Buffer 句柄（gen/flag/offset 三查）
  ├─ 迭代 (offset,len) 区间数组；越界 panic
  ├─ host-visible 则直接 memcpy+flush；否则建暂存 + with_setup_cb 上传

【纹理创建】create_texture(0x140c07950)
  ├─ desc.usage 非 0；带数据 usage|=TRANSFER_DST
  ├─ get_image_create_info(0x140bd5ed0)（kind 分派，产出 sType=14 VkImageCreateInfo）
  ├─ vkCreateImage(+0x1a0) → vkSetDebugUtilsObjectNameEXT(+0x870, type=10=IMAGE)
  ├─ vkGetImageMemoryRequirements(+0xf0) → allocator(dev+0x678) → vkBindImageMemory(+0xe0)
  └─ ManagedTexture::new(0x140bdc2d0) → 可选 init_texture_data(0x140bfe270)

【命令缓冲】CommandBuffer::new(0x140bed050)[pool sType39 + alloc sType40] → begin(0x140bed230)
  [reset(0x2d0)+begin(0x2c0)+取序号+track(dev+0xa50)] → 录制 → end(0x140bed3d0)[end(0x2c8)]

【提交】ensure_active_cb(0x140bebc90)[空则 create+begin+push] → submit_pending_cbs(0x140bebdf0)
  [end 全部 pending → 0x90B SubmitInfo → queue_submit → reclaim(0x140be14b0)]
  Queue::update_pending_submissions(0x140bf0b90)[读时间线+弹环] ← SubmissionEngine(0x1418c3300)
```

---

## 2. 逐函数表

> 每行：`骨架名（域内名）| 地址 | role | 算法/行为要点 | verdict | evidence`。
> `va=—` 表示 PDB compiled 但无 S_PUB32 地址（多为泛型单态化未成独立函数或 ICF 折叠）。derive 胶水（fmt/hash/eq/clone/default）与 Bevy 注册胶水按 AGENTS 规则不计独立业务，表中仅列显著项。
> NotFound 反编译（82B `{"error":...}`）已用 rne-read/消费者语义复核的标 `O(消费端)`，未复核的标 `U`。

### 2.1 device.rs

| 函数 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `GraphicsDevice::create` | 0x140bf2560 | 设备创建 | 155KB 单态化：枚举队列族属性（stride 0x1c）、特性/扩展筛选、建 Queue 族、`SubmissionEngine`、`GraphicsDevice+0x720` 布局；泛型实例含 `create_raw_device` 等 | O | n1-device/findings + decomp-0x140bf2560 |
| `Queue::with_raw<...>` | 0x140c03d70 | 原始 vk 操作包装 | 队列引用计数（`*+0x10` 原子，0x10 步进，8 位借用标志）→ 队列锁 `+0x70` → puffin scope `0x21` → `dev+0x6d0` 分派（submit/wait），`0x3b9acdeb` 特判；结果 `u64` 打包 | O | decomp-0x140c03d70 |
| `Queue::update_pending_submissions` | 0x140bf0b90 | 时间线推进 | `vkGetSemaphoreCounterValue(+0x478, queue+0x88)` → 存 `+0x80` → 弹提交环（cap-first `{cap@0,ptr@+8,head@+0x10,count@+0x18}`，条目值≤counter 即完成） | O | decomp-0x140bf0b90 + n1 |
| `Queue::reclaim_command_buffer_resources` | 0x140bf0cc0 | 命令缓冲回收 | 25KB：按时间线/序号回收完成的命令缓冲回池、重置、交接资源所有权（含多队列/桶） | O | decomp-0x140bf0cc0 |
| `impl$1::reclaim_command_buffer_resources::closure$0` | — | 回收闭包 | 同上（内联于回收迭代） | I | 同族 |
| `GraphicsDevice::device_uuid` | 0x140bf84c0 | UUID | Ghidra 无函数体；从物理设备属性读 UUID | U | NotFound + n1 |
| `GraphicsDevice::with_setup_cb<*>` ×5 | — | setup 回调 | `with_setup_cb` 在 `0x140c03fa0/0x140c04840` 两实例（`create_buffer::closure`/`update_buffer_bytes::closure`/`init_texture_data::closure$1`/`update_texture_impl::closure$2` 的单态化）；写 `dev+0x9f8..0xa4f` 内联记录 | O(部分) | n1 + CALL-CHAIN §4 |
| `GraphicsDevice::get_setup_cb_submit_info` | 0x140bf84e0 | 取 setup 提交信息 | 锁 `+0x9f0`；读 `+0x9f8` 状态（1=recording→end；4=已完成→空）；先发全量屏障（`+0x3e0` 10 参，stage 0x10000）；把 setup CB 记录打包成 0x90B `VkSubmitInfo` 供并入图提交；空态写 `0x8000000000000000` | O | decomp-0x140bf84e0 + a7 |
| `GraphicsDevice::physical_device` | 0x140bf8950 | 取物理设备 | NotFound；返回 `+0x78+0x5c8` 区物理设备字段 | U | NotFound |
| `GraphicsDevice::debug_utils` | 0x140bf8960 | 取 debug-utils | NotFound；返回 `+0x870` Option 的 DebugUtils 包装 | U | NotFound |
| `GraphicsDevice::wait_idle` | 0x140bf8980 | 等待 | `vkDeviceWaitIdle`（ash `+0x98` 区）+ 清空待决；含时间线/队列收尾 | O | decomp-0x140bf8980 |
| `GraphicsDevice::wait_idle_and_recycle_resources` | 0x140bf8e60 | 等待+回收 | `wait_idle` + 遍历退役桶（seq 顺序）立即排空；游戏每帧 `0xa35b` 调用点 | O | decomp-0x140bf8e60 + CALL-CHAIN |
| `GraphicsDevice::with_global_allocator<…>` ×4 | — | 分配器闭包 | 锁全局分配器（`+0x678`，锁 `+0x10`）执行 body，返回 gpu_allocator `AllocatorReport`（`commander_update`/`shut_down_everything`/`render_gpu_allocator_gui`/`report_bug` 四实例） | O(部分) | n1 §3 |
| `GraphicsDevice::refresh_memory_budget` | 0x140bf8f80 | 刷新预算 | NotFound；调 gpu-allocator `allocator.refresh_memory_budget()` | U | NotFound + 游戏面 |
| `GraphicsDevice::min_subgroup_size` | 0x140bf8fa0 | 子组大小 | NotFound；从 properties2 `subgroup_size_control` 读 | U | NotFound |
| `impl$5::drop` | — | 设备析构 | 释放队列/时间线信号量/命令池；ash `vkDestroyDevice` | I | 结构面 |

### 2.2 buffer.rs

| 函数 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `impl$7::clone` | — | BufferDesc clone | derive（TPI 含 AtomicI32 依赖） | O(derive) | skeleton |
| `impl$0::as_managed_handle` | — | Buffer→managed 句柄 | 把 40B Buffer 折叠为 `ManagedHandle`（取 gen/flag） | I | skeleton line 29 |
| `Buffer::device_address` | 0x140bd4c00 | 设备地址 | 三校验（gen==设备代号 / flag==1 / offset==inner+0xc）→ 读 `inner+0x30` 地址 → 经 `dev+0x508` 回调返回；失败 panic | O | decomp-0x140bd4c00 |
| `Buffer::mapped_slice_mut` | 0x140bd4cd0 | 持久映射 | 校验同上 → `0x140c113b0(inner+0x38)` 取 mapped ptr；`0x140c02280` 取块大小；`+0x18` size 界内返回 `&mut`；HOST_VISIBLE(`inner+0x7c`==2) 否则 panic | O | decomp-0x140bd4cd0 |
| `impl$10::fmt` | — | Debug | derive | O(derive) | — |
| `BufferDesc::new` | 0x140bd4de0 | 描述符 | `{size,usage,mem=1,align=0}`（新实例默认 GpuOnly） | O | decomp-0x140bd4de0 |
| `BufferDesc::new_gpu_only` | 0x140bd4e20 | — | mem=1 | O | decomp-0x140bd4e20 |
| `BufferDesc::new_cpu_to_gpu` | 0x140bd4e60 | — | mem=2 | O | decomp-0x140bd4e60 |
| `BufferDesc::new_cpu_to_gpu_low_prio` | 0x140bd4ea0 | — | mem=3（HOST_VISIBLE 非 coherent） | O | decomp-0x140bd4ea0 |
| `BufferDesc::new_gpu_to_cpu` | 0x140bd4ee0 | — | mem=4 | O | decomp-0x140bd4ee0 |
| `BufferDesc::alignment` | 0x140bd4f20 | — | 返回 `+0x0c` u16，`<0x10000` 断言 | O | decomp-0x140bd4f20 + n2 |
| `impl$3::byte_count/copy_to_slice`（×2） | — | 初始数据 trait | `InitialData` 两 impl（enum/&[u8]）：字节数/拷出 | I | skeleton line 143/146/152/156 |
| `GraphicsDevice::create_buffer_impl` | 0x140bee200 | 缓冲创建核心 | 见 §1.3；返回 0x48B 结果（`+0x00`=0xb tag，`+0x08`=inner，`+0x10`=alloc handle，`+0x18`=alloc，`+0x20`=offset，`+0x24`=gen，`+0x26`=flag=1，`+0x28`=size，`+0x30`=usage，`+0x38`=mapped，`+0x40`=size） | O | n2 0x140bee200.c |
| `impl$6::create_buffer_impl::closure$0..2` | — | 创建闭包 | vkCreateBuffer 结果检查 / 分配 / 命名 | I | 同族 |
| `GraphicsDevice::create_mapped_buffer` | 0x140beea80 | 可映射缓冲 | 要求 mem∈{2,4}；建 Buffer 并返回 mapped ptr | O | n2 0x140beea80.c |
| `GraphicsDevice::create_buffer<Option<&[u8]>/&[u8]/()>` ×3 | 0x140c03460 等 | 游戏面创建 | 见 §1.3；带数据时加暂存上传链；返回 80B Result（Ok=40B Buffer + Err 40B） | O | n2 0x140c03460.c |
| `impl$6::create_buffer::closure$0` | — | 上传闭包 | setup CB 录 copy（staging→target） | I | 同族 |
| `GraphicsDevice::set_buffer_debug_name` | 0x140beec30 | 命名 | debug-utils 门控（`+0x870`）→ 校验 → `vkSetDebugUtilsObjectNameEXT`（sType 9=BUFFER，句柄=`inner+0x30`） | O | n1 0x140beec30.c |
| `GraphicsDevice::update_buffer_bytes` | 0x140beefa0 | 区间上传 | 见 §1.3；`inner+0x20` 容量、`inner+0x28` HOST_VISIBLE 判据 | O | n1 0x140beefa0.c |
| `impl$6::update_buffer_bytes::closure$0` + closure$0/1 | — | 上传闭包 | memcpy / flush / 暂存路径 | I | 同族 |
| `GraphicsDevice::get_vk_buffer` | 0x140bef470 | 取 vk 缓冲 | 经 `0x1418aeba0(dev+0x648)` 注册表查找，非 0 即命中（thin lookup） | O | n1 0x140bef470.c |

### 2.3 texture.rs

| 函数 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `MipChainIter::new` | 0x140bd5a80 | mip 迭代器 | 校验 mip≠0、array∈[1,0x100)、extent 各维∈[1,0x10000)；输出 `{w u16,h u16,3rd u16,array u16,mip/sample…}` | O | decomp-0x140bd5a80 |
| `impl$0::drop` | — | Texture 析构 | 走退役链（create_texture 中的 disposal 链表 `ctx[0x2f]`） | I | create_texture |
| `TextureDesc::{new,new_2d,new_2d_array,new_3d}` | 0x140bd5270/52a0/52e0/5310 | 构造 | NotFound；new 三参 extent+2 字节；new_2d 深=1 层=1；3d 层=1 | O(消费端) | rhapsody_types.rs + NotFound |
| `TextureDesc::{usage,sampled,storage,transfer_dst,color_attachment,depth_stencil_attachment,sampled_with_different_format,format,mip_levels,all_mip_levels,array_elements,sample_count}` | 0x140bd5340..55a0 | usage 位构造 | 全部 NotFound（tiny 叶子）；`storage`=`usage\|=8`、`transfer_dst`=`usage\|=2`（i_api 已核）；`all_mip_levels` 返回 `mip_count_1d` 计算值 | O(消费端)/U | rhapsody_types.rs + NotFound |
| `TextureDesc::div_up_extent` | 0x140bd55e0 | 除上取整 | extent 各维 `ceil_div`（2 常量缩放） | O | decomp-0x140bd55e0 |
| `TextureDesc::div_extent` | 0x140bd56a0 | 整除缩放 | extent `div` | O | decomp-0x140bd56a0 |
| `TextureDesc::half_res` | 0x140bd5730 | 半分辨率 | `div_extent(extent,2)`（+ 修正奇尺寸） | U | NotFound + n2 |
| `TextureDesc::extent_2d` | 0x140bd5790 | 取 (w,h) | NotFound | O(消费端) | rhapsody_types.rs |
| `TextureViewDesc::{new,view_type,usage,format,aspect_mask,base_mip_level,level_count,base_layer,layer_count,bind_individual_mips}` | 0x140bd57a0..59f0 | 视图描述 | 全 NotFound；构造器写 Option<u32> 标签+值；`level_count/layer_count` 参数为 `Option<u32>`（r8d 标签/r9d 值） | O(消费端) | rhapsody_types.rs |
| `impl$4::from` | — | desc→view desc | 从 TextureDesc 派生默认视图描述（无格式重定） | I | skeleton line 332 |
| `impl$6::next` | — | MipChainIter 迭代 | 逐 mip 生成 view 描述/阶段 | I | skeleton line 380 |
| `impl$8::next` | — | LinearInitialDataIter | 线性初始数据按 mip 切块 | I | skeleton line 507 |
| `GraphicsDevice::create_texture<TextureInitialData/&[u8]>` ×2 | 0x140c07950 等 | 纹理创建 | 见 §1.3；返回值 80B Result（Ok=40B Texture） | O | n2 0x140c07950.c |
| `impl$9::create_texture::closure$0` | — | 创建闭包 | 调试名 `"GpuOnly texture"`（0xf 字节 strncpy） | O | create_texture |
| `GraphicsDevice::init_texture_data` | 0x140bfe270 | 初始上传 | `*desc&4` 置位走 jumptable 直接写，否则暂存+setup 上传（panic 路径含 `initial_data.byte_count()<=desc.size` 断言） | O | n2 0x140bfe270.c |
| `impl$9::init_texture_data::closure$1` | — | 上传闭包 | setup CB copy image | I | 同族 |
| `GraphicsDevice::set_texture_debug_name` | 0x140bfe650 | 命名 | debug-utils 门控 → `vkSetDebugUtilsObjectNameEXT`（type=10=IMAGE） | O | n2 0x140bfe650.c |
| `GraphicsDevice::update_texture` / `update_texture_impl` | — | 更新上传 | `update_texture` 入口；impl 录 copy 到 setup CB（`closure$1/2`） | O(部分) | n2 + CALL-CHAIN |
| `GraphicsDevice::get_vk_image_view` | — | 取 vk 视图 | 注册表/视图缓存查找（`get_vk_buffer` 同款）；`Result::unwrap` 消费点 | I | CALL-CHAIN §4#6 |
| `get_image_create_info` | 0x140bd5ed0 | desc→VkImageCreateInfo | kind 跳表（0..6，含 `0xd/0xe/0xf/0x10` 坏指令区）；校验 extent/mip；sType=14；`+0x50`=has_data<<3 标志 | O | decomp-0x140bd5ed0 + n2 |

### 2.4 swapchain.rs

| 函数 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `impl$8/impl$9::fmt` | — | Debug | derive | O(derive) | — |
| `Swapchain::enumerate_surface_formats` | 0x140c0e650 | 格式枚举 | 从配置管理器 `+0x658→+0xd0` 与 surface `+0x650` 读 cap/格式，经 `0x1418b0490` 枚举 | O | decomp-0x140c0e650 |
| `Swapchain::on_focus` | 0x140c0e6b0 | 焦点处理 | 校验 → 锁 inner+0x10 → 写 `inner+0xc9=param_3` → 释放 → **走全局退役哈希表**（fibonacci 散列 `-0x61c8864680b583eb`、refcount 环、marker 桶）刷新交换链图像 | O | decomp-0x140c0e6b0 |
| `Swapchain::format` | 0x140c0e7e0 | 取格式 | NotFound | U | NotFound |
| `Swapchain::resize` | 0x140c0e7f0 | 重建 | 校验 → 标记 stale → `create_swapchain`（按 `old_swapchain`）→ 更新 inner | O | decomp-0x140c0e7f0 + MODULE_STATUS |
| `Swapchain::set_vsync` | 0x140c0e8a0 | vsync | 写 vsync 标志 + 重建链 | O | decomp-0x140c0e8a0 |
| `Swapchain::mark_stale` | 0x140c0e950 | 失效 | 校验 → 双锁 inner+0x10 → 若 `state==2` 交换 current/previous（+0x18→+0x40 等 6 字段对拷）→ 全局退役哈希表处置 | O | decomp-0x140c0e950 |
| `Swapchain::extent` | 0x140c0ea90 | 取尺寸 | 校验 → `0x1418cb100(inner+0x10)` 读 extent 原子 | O | decomp-0x140c0ea90 |
| `Swapchain::acquire_next_image` | 0x140c0eb40 | 获取图像 | vkAcquireNextImageKHR（超时/信号量/栅栏），result 分派（SUBOPTIMAL/OUT_OF_DATE/成功），产出租约 | O | decomp-0x140c0eb40 |
| `impl$0::acquire_next_image::closure$0` | — | 获取闭包 | 结果处理 | I | 同族 |
| `Swapchain::any_present_successful` | 0x140c0ef70 | 呈现状态 | NotFound；读 `+0x1c` 标志位 | U | NotFound |
| `Swapchain::present` | 0x140c0ef80 | 呈现 | 校验 → `0x1418cc1b0(inner+0x10)` vkQueuePresentKHR → `*(+0x1c) \|= result` | O | decomp-0x140c0ef80 |
| `SwapchainLeaseSync::into_sync_data` | 0x140c0f040 | 租约→同步数据 | 消费标志 `+0x28`（==1 已用→panic）→ 置 1 → inner refcount `+8`++ → 打包 SyncData（0x28B）→ drop 租约句柄 | O | decomp-0x140c0f040 |
| `impl$2::drop` | — | 租约析构 | 归还交换链租约 | I | 同族 |
| `TrackedSwapchainTextureLease::image` | 0x140c0f120 | 取图像 | NotFound | U | NotFound |
| `TrackedSwapchainTextureLease::into_present_source` | 0x140c0f170 | 取呈现源 | `+0x08` 取 present source → drop `+0x10`/`+0x28` 两个子句柄（image+fence/sem） | O | decomp-0x140c0f170 |
| `GraphicsDevice::create_swapchain` | — | 创建 | 选格式/呈现模式，`old_swapchain` 重建；`"Swapchain creation failed: "` 日志（0x142d08400） | O(部分) | n1 + CALL-CHAIN |
| `impl$4::create_swapchain::closure$0` | — | 创建闭包 | present_mode 筛选 | I | 同族 |
| `GraphicsDevice::validate_swapchain_and_acquire_image` | 0x140bfd9d0 | 校验+获取 | `acquire_next_image` → 判别 `7` 分支 → switch(2=SUBOPTIMAL/4=OUT_OF_DATE/默认=OK) → 产出 TrackedSwapchainTextureLease；调试日志门控 `*0x14362c3a8>3` | O | a6 0x140bfd9d0.c |

### 2.5 shader.rs

| 函数 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `GraphicsDevice::create_raster_pipeline<Arc<Compiled>>` | — | 光栅管线 | 管线创建算法（a5 §6，O）：leek(0x14) → `create_descriptor_set_layouts(param_3+0x40, 0x1f)` → push constant（仅当 `+0x148 != 0`，stage 0x1f）→ `vkCreatePipelineLayout(+0x210)` → 顶点属性 `map_vertex_attribute_format` → `vkCreateGraphicsPipelines(+0x1f8, cache=device+0x808)` → `create_descriptor_set_states` → 返回 **0x38B** | O | a5 §6 + CALL-CHAIN §3.1 |
| `create_descriptor_set_layout<…>` ×2 | — | 布局构造 | Map 迭代（enumerate bindings / HashMap Iter bindings）；每个 binding **0x18B** `{[0]binding号/[4]type/[8]count/[0xc]stage/[0x10]samplers}`；type 映射（推断）：0=storage buffer(7)/1=uniform(6)/2=combined image sampler/3=storage image/4=sampled image/5=immutable sampler；`_dyn` 尾 4B==`0x5f64796e`→dynamic(+2)；运行时计数 `min(max(P-32,0),8192)`（P=`*(*(device+0x658)+0x680)`）；PARTIALLY_BOUND(4)、sampled image flags 15 + pool UPDATE_AFTER_BIND(2)；set#0 stage 用传入参数，其余 `0x7fffffff` | O | a5 §4 |
| `DescriptorSetState::allocate_descriptor_set` | 0x140be5a40 | 分配描述符集 | **空闲池复用**（非内容寻址）：`+0x58` 自旋锁保护 `+0x60` 空闲池 Vec；池非空弹末尾 reuse(reset)→ 空则从来源补批（`+0x50` 共享源、待用批次 `+0x20/+0x28`）→ 源空则新建；`-0x3b9bd788`(=VK_ERROR_OUT_OF_POOL_MEMORY) / `-0xc`(=VK_ERROR_FRAGMENTED_POOL) 时条目推入调用方回收 Vec 并**循环重试**；第二参 Vulkan 分派 `+0x1c8 分配/+0x1d8 reset/+0x1e0 释放/+0x5c8 vkDevice` | O | a5 §3 + decomp-0x140be5a40 |
| `impl$2/impl$3::deref` | — | Deref | 转发到内部 set | I | skeleton line 154 |
| `create_descriptor_set_layouts` | 0x140be60a0 | 批量布局 | 逐 binding（stride 0x30）→ 建布局信息 → `vkCreateDescriptorSetLayout(+0x230)`，**全局缓存哈希 `0x140d4d740` 去重**；布局对象 0x58B | O | decomp-0x140be60a0 |
| `ComputePipelineDesc::new` | 0x14003bb80 | 计算管线描述 | NotFound（82B）；简单构造器 | U | NotFound |
| `GraphicsDevice::create_compute_pipeline<Arc>` | — | 计算管线 | 同 raster 但 stage `0x20`、`vkCreateComputePipelines(+0x200)` | O | a5 §6 + CALL-CHAIN §3.1 |
| `create_descriptor_set_states` | 0x140be66c0 | 池状态 | 布局 slice（stride 0x30）+ binding 计数（stride 8）→ `0x1418b6140` 建 DescriptorSetState（pool 尺寸表） | O | decomp-0x140be66c0 |
| `impl$7::default` | — | RasterState 默认 | 默认光栅状态（draw_stream PassHost `default_raster_state` 消费） | I | CALL-CHAIN §3.1 |
| `RasterPipelineDesc::new` | 0x140be6760 | 描述符 | NotFound | U | NotFound |
| `RenderPassAttachmentDesc::{new,clear_input,sample_count}` | 0x140be6780/67a0/67c0 | 附件描述 | NotFound；new(format)、clear_input(bool)、sample_count(flag) 三方法面 | O(消费端) | skeleton shader.rs 690/704/714 |
| `get_vk_blend_mode` | 0x140be67f0 | 混合模式映射 | NotFound；BlendMode→vk blend factor/op | U | NotFound |
| `map_vertex_attribute_format` | 0x140be6850 | 顶点格式映射 | 按 format 类别（2/3/4）+ 元素字节（uVar3=(bytes-4)>>2∈[0,4)）→ `result = uVar3*3 + base(98/99/100)` 查 vk::Format | O | decomp-0x140be6850 |

### 2.6 command_buffer.rs

| 函数 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `impl$11::fmt` | — | Debug | derive | O(derive) | — |
| `CommandBufferSeqTracker::track` | 0x140becfd0 | 序号跟踪 | 校验 cb 状态==1 → CAS 环 `current = max(current, cb.seq)`（`*param_1` 原子） | O | decomp-0x140becfd0 |
| `CommandBufferSeqTracker::current` | — | 取当前 | 原子读 | I | skeleton line 68 |
| `impl$4::drop` | — | 析构 | 归还命令池/缓冲 | I | 同族 |
| `CommandBuffer::new` | 0x140bed050 | 创建 | `vkCreateCommandPool`(sType39, flags=2, queue_family) + `vkAllocateCommandBuffers`(sType40, 1) → 句柄@+0x48，初始 `{0,8,…}` 模式 + 设备代号@+0x54 | O | decomp-0x140bed050 + n1 |
| `CommandBuffer::raw` | — | 取 vk 句柄 | `+0x48` | I | skeleton line 179 |
| `CommandBuffer::begin` | 0x140bed230 | 开始录制 | state∈{0,3} → `vkResetCommandBuffer(+0x2d0)` → 级别由 `*(dev+0x658)+0x78` 字段派生（`(val==0)?1:4`，U）→ `vkBeginCommandBuffer(+0x2c0)` → 从 `dev+0x668[queue]` 取序号 → state=1 → `dev+0x320` 打标 → `track(dev+0xa50)` | O | decomp-0x140bed230 + n1 |
| `CommandBuffer::end` | 0x140bed3d0 | 结束录制 | state==1 → `vkEndCommandBuffer(+0x2c8)` → state=2；否则 panic（`FUN_140bed620` assert） | O | decomp-0x140bed3d0 |
| `CommandBuffer::set_stencil_reference` | — | 模板引用 | `vkCmdSetStencilReference` | I | skeleton line 251 |
| `CommandBufferPool::new` | 0x140bed4c0 | 池 | 建 Vec<0x18B 条目>，cap-first `{cap@0,ptr@8,len@0x10}`，元素初值 `{0,8,0}` | O | decomp-0x140bed4c0 |
| `CommandBufferPool::recycle` | — | 回收 | 归还/重置命令缓冲 | I | skeleton line 278 |
| `GraphicsDevice::create_command_buffer` | 0x140bef4a0 | 池分配 | 从池空闲链取/建（锁 `+0x9b0`），命名经 `+0x870`，分配失败 debug | O | n1 0x140bef4a0.c |
| `impl$7::create_command_buffer::closure$0/1` | — | 分配闭包 | 取/建缓冲 | I | 同族 |

### 2.7 barrier.rs

| 函数 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `ImageBarrier::new` | 0x140bd65e0 | 图像屏障 | NotFound；`{image, old/new layout, src/dst stage, aspect}` 构造 | U | NotFound |
| `BufferBarrier::new` | 0x140bd6610 | 缓冲屏障 | NotFound；`{buffer, offset, size, src/dst stage}` | U | NotFound |
| `impl$2::record` | — | 录制 | `vkCmdPipelineBarrier` 单条录入 | I | skeleton line 87 |
| `image_aspect_mask_from_format` | 0x140bd6630 | aspect 掩码 | 按 VkFormat 查内建表（深度/模板/颜色 aspect）；NotFound | O(消费端) | n2 + NotFound |
| `impl$3::default` | — | 默认作用域 | 空作用域 | I | skeleton line 152 |
| `impl$5::{image_barrier,buffer_barrier,reset,is_empty}` | — | 作用域增删 | 追加 image/buffer 屏障；reset 清空；is_empty 判 | I | skeleton 170/206/233/241 |
| `GraphicsDevice::issue_and_reset_pipeline_barrier` | — | 下发+清空 | `issue` 后 `reset` | I | skeleton line 419 |
| `GraphicsDevice::issue_pipeline_barrier` | 0x140bee030 | 下发屏障 | `(cmd@param_2+0x48, src@+0x48, dst@+0x4c, 0, imageCount@+0x10, images@+8, bufferCount@+0x28, buffers@+0x20, memCount@+0x40, mems@+0x38)` 10 参 → `dev+0x3e0` | O | decomp-0x140bee030 |
| `GraphicsDevice::issue_full_pipeline_barrier` | 0x140bee090 | 全屏障 | 单条 memory barrier（sType 44=0x2c，access mask `0x1800000018000`）+ stage 0x10000×2 → `dev+0x3e0`；`[device+0xaba]` 开关由调用方控制 | O | decomp-0x140bee090 + CALL-CHAIN |

### 2.8 submit.rs

| 函数 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `impl$3::default` / `SubmitInfo::{new,command_buffer(s),wait_semaphore,signal_semaphore}` | — | 构建器 | builder 收集命令/信号量；原版为内联单态 | O(derive/内联) | skeleton line 10..57 |
| `carve_off_front` | — | 队首切出 | `VecDeque` pop_front | I | skeleton line 70 |
| `impl$7::eq/impl$9::fmt/impl$17::partial_cmp` | — | 排序 | PendingSubmission 比较（按时间线） | O(derive) | skeleton 76/82 |
| `SubmissionEngine::update_pending_submission_timestamp` | — | 时间戳 | 更新队首提交时间线 | I | skeleton line 107 |
| `SubmissionEngine::update_queue_pending_submissions` | 0x1418c3300 | 推进队列 | 图形 Queue（`+0x10`）+ Vec<Queue>（`+0xc8/+0xd0`，stride 0xb0）逐个 `update_pending_submissions`；`0x140c16a00()` 为日志级别门控 | O | decomp-0x1418c3300 + n1 |
| `SubmissionEngine::reclaim_command_buffer_resources_all_queues` | — | 回收 | 各队列回收完成命令缓冲 | I | skeleton line 135 |
| `SubmissionEngine::{queue,queue_mut}` | — | 取队列 | 主/辅选择 | I | skeleton 144/151 |
| `GraphicsDevice::queue_family_index` | 0x140bfc750 | 队列族 | `dev+0xad4` 主族；kind bit0 时 `dev+0x3c` 辅族（要求 `dev+0x20` bit0） | O | a7 findings |
| `GraphicsDevice::queue_submit<Vec<SubmitInfo>,closure>` | — | 提交 | 4 寄存器参 = `(&device, Option<u32>)`+返回槽；把 setup CB 并入图提交 | O(部分) | MODULE_STATUS |
| `GraphicsDevice::wait_submit_done` | — | 等待栅栏 | `vkWaitForFences` | I | skeleton line 363 |
| `GraphicsDevice::wait_duration_or_until_submit_done` | — | 有界等待 | 超时→false | I | skeleton line 394 |

### 2.9 descriptor_set.rs / binding.rs

| 函数 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `impl$1::clone` | — | DescriptorSet clone | derive（Arc 增引用） | O(derive) | skeleton line 11 |
| `GraphicsDevice::create_persistent_descriptor_set` | 0x140bfef70 | 持久集 | `create_descriptor_set_layout`（set 切片 stride 0x28，stage `0x7fffffff`，immutable sampler 缓存）→ 按 type 累加 → `vkCreateDescriptorPool(+0x240, maxSets=1, flags=2)` → `vkAllocateDescriptorSets`（变量计数）→ `ManagedResourceStore::allocate(0x1418aead0)` → 结果 0x18B，managed tag=`6` | O | a5 §5 |
| `GraphicsDevice::update_persistent_descriptor_set<Map<Drain<AssetIndex>,…>>` | 0x1416a2320 | 更新持久集 | 构造 `VkWriteDescriptorSet`(0x40) → `vkUpdateDescriptorSets(+0x268)`，`device+0xab0` 原子 +1；游戏 bindless 纹理表更新（`update_bindless_texture_table::closure_env$0`） | O | a5 §5 + CALL-CHAIN |
| `GraphicsDevice::get_vk_descriptor_set` | — | 取 vk 集 | 校验 `+0x16 == 6`（managed tag）且 `+0x14 == *(store+0xa50)` | O | a5 §5 |
| `impl$1::fmt/impl$2::clone`（DescriptorSetBinding） | — | Debug/Clone | derive | O(derive) | skeleton line 17 |
| `GraphicsDevice::bind_pipeline` | — | 绑定管线 | `vkCmdBindPipeline`（管线族 `+0x1f8/+0x200` 区） | I | skeleton line 40 + a5 §6 |
| `GraphicsDevice::bind_descriptor_set` ×4 | 0x1416a5380 / 0x1416a5910 / 0x1416a6010（3 实例已列，第 4 实例未列址） | 绑定描述符 | 4 游戏闭包实例：`Chain<Map<IntoIter<(BindingIndex,PassResourceSlot)>,…>, Map<Iter<(BindingIndex,GpuOwned<DescriptorSetBinding>)>,…>>`（dispatch_impl/draw_stream）与 `Map<Enumerate<Iter<DescriptorSetBinding>>,…>` ×2；内部逐 (idx,slot) → `vkCmdBindDescriptorSets(+0x328)` | O | a5 §2 + CALL-CHAIN §4#4 |
| `GraphicsDevice::update_descriptor_set` | — | 更新 | `vkUpdateDescriptorSets(+0x268)`（inline 1 闭包） | I | skeleton line 96 |
| `GraphicsDevice::bind_persistent_descriptor_set` | 0x140bee150 | 绑定持久集 | `get_tracked`（`0x1418aee50` 注册表查找）→ `*set==1` → `vkCmdBindDescriptorSets(+0x328)`(8 参, count=4) | O | decomp-0x140bee150 + a5 §5 |

### 2.10 render_pass.rs / framebuffer.rs

| 函数 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `impl$4::hash / impl$8::clone / impl$9::hash / impl$11::eq / impl$13::default`（RenderPassDesc） | — | 键 derive | ArrayVec<AttachmentDesc,8> 哈希/相等（sip hasher），键含 `(ManagedHandle,u32)` | O(derive) | skeleton render_pass.rs |
| `GraphicsDevice::create_render_pass` | — | 缓存创建 | 校验 → `HashMap::rustc_entry(RenderPassDesc)`（device+0x188）→ 命中返回；未命中 `Chain<Map<Iter<AttachmentDesc>,closure0>, IntoIter<AttachmentDescription>>` 转 `vk::AttachmentDescription` → `vkCreateRenderPass(+0x280)` → 插缓存 | O | skeleton + CALL-CHAIN §4#14 |
| `impl$1::create_render_pass::closure$0/1` | — | 附件映射 | 附件→AttachmentDescription（load/store/layout） | I | skeleton line 67/76 |
| `FramebufferDesc::new` / `impl$0::new::closure$0` | — | 帧缓冲描述 | 附件 + 尺寸 + renderpass；`"Must have at least one framebuffer attachment"` 断言（0x142fd9448） | I | skeleton + CALL-CHAIN |
| `GraphicsDevice::create_framebuffer` | — | 创建帧缓冲 | `vkCreateFramebuffer(+0x270)`，sType 37 | O(部分) | n1 §2 |

### 2.11 dynamic_constants.rs

| 函数 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `DynamicConstantsChunk::write/with_data` | — | 写/构造 | 映射写数据 | I | skeleton line 26/30 |
| `DynamicConstantsChunk::device_address` | 0x140c02280 | 设备地址 | NotFound | U | NotFound |
| `DynamicConstantsChunk::into_gpu_lifetime_tracker` | 0x140c02290 | 生命周期 | NotFound | U | NotFound |
| `DynamicConstantsChunk::into_uniform_buffer_binding` | 0x140c022b0 | 绑定点 | `0x1418aeba0` 注册表查 buffer → 输出 `DescriptorSetBinding`（tag 6）+ {device_address, offset, size} | O | decomp-0x140c022b0 |
| `DynamicConstantsChunk::into_uniform_and_storage_buffer_binding` | 0x140c02370 | 双绑定点 | 同上但产出**两个**绑定（tag 6 与 5，各含 device_address/offset/size）供 uniform+storage 同时使用 | O | decomp-0x140c02370 |
| `DynamicConstantsChunk::into_uniform_buffer_binding_and_data` | 0x140c02450 | 绑定+数据 | 绑定 + 数据切片 | O | decomp-0x140c02450 |
| `DynamicConstants::new` | 0x140c02570 | 常量池 | 建 chunk 缓冲（约 256KB） | O | decomp-0x140c02570 |
| `DynamicConstants::push/push_slice` | — | 推入 | 对齐+游标推进 | I | skeleton line 144/150 |
| `DynamicConstants::allocate` | 0x140c02740 | 分配 | 见 §1.3；`param_4<0x40001`（256KB 上限）、对齐 `max(0x10, min_align, chunk 要求)`、溢出时 `FUN_140c029f0` 建新 chunk + `0x140bd4cd0` 映射 + `dev+0x508` 登记 | O | decomp-0x140c02740 |
| `DynamicConstants::allocate_buffer` | — | 缓冲分配 | 内部建缓冲 | I | skeleton line 200 |

### 2.12 debug.rs / profiler.rs / sampler.rs / semaphore.rs / stats.rs / surface.rs / temp_list.rs

| 函数 | 地址 | role | 算法/行为要点 | verdict | evidence |
|---|---|---|---|---|---|
| `GraphicsDevice::begin_label` / `end_label` | 0x140bf09b0/0x140bf0b40 | 调试标签 | `vkCmdBeginDebugUtilsLabelEXT`（`+0x898`）/`vkCmdEndDebugUtilsLabelEXT`（`+0x8a0`） | O | n1 0x140bf09b0.c/0x140bf0b40.c |
| `GraphicsDevice::create_gpu_profiler_output_buffer` | 0x140bff4a0 | 时间戳缓冲 | **0x2000B** GPU 时间戳缓冲（STORAGE|TRANSFER_DST，经 `0x140bff5a0 VulkanBackend`：vkCreateBuffer+全局分配器+bind） | O | n1 §5 |
| `impl$1::create_query_result_buffer` | — | 查询缓冲 | 同族 | I | skeleton line 30 |
| `impl$2::mapped_slice/raw` | — | 映射 | profiler 缓冲映射切片 | I | skeleton 75/79 |
| `TimedFrame::new` | 0x140bd3cb0 | 帧时间戳 | 输入时间戳数组 → Vec<0x18B 条目>（cap-first），每项 `{计时源, 原始, 换算时长}`（`0x140c10e50/60` 时间换算） | O | decomp-0x140bd3cb0 |
| `TimedFrame::send_to_puffin` | 0x140bd4030 | puffin 输出 | 迭代条目写 puffin scope（字节 `0x29` + 名称长度 + 名称 ptr），`0x140c16a10` 冲刷 | O | decomp-0x140bd4030 |
| `GraphicsDevice::create_samplers` / `impl$0::closure$0/1` | — | 内置采样器 | 最近邻/线性/深度比较 3 采样器（`vkCreateSampler`） | O(部分) | rebuild 对映 + CALL-CHAIN |
| `GraphicsDevice::get_built_in_sampler` | — | 取采样器 | 按索引取内置 | I | skeleton line 197 |
| `create_raw_timeline_semaphore` | 0x1418c8710 | 时间线信号量 | timeline 扩展结构（sType 1000207003 链）+ `SemaphoreCreateInfo`(sType 9) → `vkCreateSemaphore`(ash +0x40) | O | decomp-0x1418c8710 + n1 |
| `RenderStats::snapshot_and_reset` | 0x140c02da0 | 统计快照 | NotFound | U | NotFound |
| `Surface::create<Arc<Window>,Arc<Window>>` | — | 表面创建 | winit Window → `vkCreateSurfaceKHR` | I | skeleton line 10 |
| `TempList::{default,new,add}` / `add<CString>` | — | 临时列表 | 生命周期守卫列表（pNext 链临时对象） | I | skeleton |

---

## 3. 原版 ↔ rebuild 对照表

> 核 CALL-CHAIN §4 差距 #6/#7/#8/#9（Buffer 与 GraphicsDevice），并补充其余。**✓**=对齐，**半**=部分对齐，**✗**=缺失/偏差。rebuild 路径为 `reconstruction/rhapsody-rebuild/src/raw/**`。

| 原版（域内） | 地址 | rebuild 对应 | 判定 | 差异说明 |
|---|---|---|---|---|
| `Buffer` 40B（0x28）句柄 | 0x140bd4c00 等 | `raw/buffer.rs::Buffer`（16B `{device Arc, vk::Buffer, size, usage, allocation, mapped_ptr}`） | **✗**（#7） | 原为 40B 整块（`{tag@0, inner@8, offset@0x10, gen@0x14, flag@0x16, size@0x18, +0x20}`），rebuild 是 16B desc+句柄。游戏面 `TypedStorageBuffer` 直接搬运 40B。接续需在 rebuild 提供 40B 转换或改 game 侧。`inner+0xc/0x30/0x7c` 校验字段 rebuild 无对应 |
| `Buffer::mapped_slice_mut(&self,device)` | 0x140bd4cd0 | `Buffer::mapped_slice_mut(&mut self,offset,size)` | **半**（#8） | 原方法在 Buffer 上取 `&self`+device、全长切片、返回 `Option<&mut [u8]>`（mapping 未建立返 0）；rebuild 是 `(&mut self,offset,size)->Result`。内存种类 2/4 语义（`MAPPED_MEMORY_UPLOAD/READBACK`）仍 U |
| `Buffer::device_address(&self,device)` | 0x140bd4c00 | `Buffer::device_address(&self)->Result<vk::DeviceAddress>` | **半**（#9） | 原经 `inner+0x30` 缓存读（校验 gen/flag/offset）；rebuild 每次调 `vkGetBufferDeviceAddress`。`nani::trimesh` 块地址加法消费原语义 |
| `GraphicsDevice::create_buffer/create_buffer_impl/update_buffer_bytes` | 0x140c03460/0x140bee200/0x140beefa0 | `GraphicsDevice::{create_mapped_buffer,create_buffer_impl}` + `Buffer::{write_bytes,write_bytes_unsynchronized}` | **半**（#7） | 原 `update_buffer_bytes(device,&buf,data,ranges,offset)` 支持多区间；rebuild 无同名；`create_buffer(device,&desc,name,Option<&[u8]>)` 带暂存+setup 上传链，rebuild 的 `Buffer::new` 无 name/初值路径 |
| `get_vk_buffer` | 0x140bef470 | 无 | **✗**（#6） | 原注册表查找（`0x1418aeba0(dev+0x648)`）；rebuild 缺 |
| `get_vk_image_view` | — | 无 | **✗**（#6） | 消费点 `Result::unwrap`（原 Err 载荷） |
| `create_render_pass`（device+0x188 缓存） | — | `raw/render_pass.rs::RenderPass::from_key` | **半**（#6/#14） | 原键=`(ManagedHandle@0, u32@0x18)` 对 + `ArrayVec<AttachmentDesc,8>`；rebuild 键=`RenderPassKey(Vec<AttachmentKey>)`（无 managed 句柄维度）。`RenderPassAttachmentDesc` 0x10B：rebuild `{format,samples,clear_input,depth_stencil}` 为推断，原方法面仅 new/clear_input/sample_count（无 depth_stencil 名，U） |
| `create_framebuffer` | — | 无（`raw/framebuffer.rs` 仅 desc 构建） | **✗**（#6） | 原 `vkCreateFramebuffer(+0x270)`；rebuild 缺设备入口 |
| `create_command_buffer` | 0x140bef4a0 | `raw/command_buffer.rs::{VulkanCommandPool::allocate,VulkanCommandBuffer}` | **半** | 原从池空闲链（`dev+0x9b0` 锁）取；rebuild 每池互斥 + 新分配 |
| `bind_descriptor_set` ×4 | — | `raw/binding.rs` free fn（无设备方法） | **✗**（#4） | 原 4 游戏闭包实例（Chain<Map<IntoIter<(BindingIndex,PassResourceSlot)>,…>> 与 Map<Enumerate<Iter<DescriptorSetBinding>>,…> 两类）；rebuild 只有 rg/binding 自由函数，缺 `GraphicsDevice::bind_descriptor_set` |
| `bind_pipeline` | — | `raw/shader.rs::RasterPipeline::bind_pipeline` | **半** | 原在 GraphicsDevice 上；rebuild 在管线对象上 |
| `bind_persistent_descriptor_set` | 0x140bee150 | 无 | **✗** | 原注册表查找→`vkCmdBindDescriptorSets(+0x328)`；rebuild 缺 |
| `update_descriptor_set` / `update_persistent_descriptor_set` | — | `raw/descriptor_set.rs::VulkanDescriptorSet` 内部 | **半** | 原设备方法（`+0x268` vkUpdateDescriptorSets）；rebuild 走 allocate_descriptor_set 路径 |
| `refresh_memory_budget` | 0x140bf8f80 | `raw/physical_device.rs` static fn | **半** | 原 `GraphicsDevice` 方法（NotFound 体，名可辨） |
| `GraphicsDevice` 结构 | 0x140bf2560 区 | `raw/device.rs::GraphicsDevice{inner:Arc<VulkanDevice>,…}` | **半**（#18） | 原真实结构（VkDevice@+0x640、ash 表@+0x78、setup CB 内联@+0x9f8、SubmissionEngine@+0x720 等）；rebuild 是 `Arc<VulkanDevice>` 包装 + 5 字段。游戏侧 `Arc<GraphicsDevice>`（+0x10 数据偏移）可用 rebuild 类型替换 |
| `BufferDesc` 16B | 0x140bd4de0.. | `raw/buffer.rs::BufferDesc`（16B 编译期断言） | **✓** | `{size,usage,u16 alignment,u8 memory_location 1..=4}`；rebuild `MemoryLocation::from_raw` ✓ |
| `TextureDesc` 32B | 0x140bd5270.. | `raw/texture.rs::TextureDesc`（32B，country-core rhapsody_types 已核） | **✓** | usage/flags/format/extent/array/mip/sample/kind 全对 |
| `TextureViewDesc` 0x3cB | 0x140bd57a0.. | `raw/texture.rs::TextureViewDesc`（0x3cB） | **✓** | 6 Option<u32> + base_mip + base_layer + bind_individual_mips |
| `Texture`（create/init/set_debug_name/update） | 0x140c07950 等 | `raw/texture.rs::Texture` + `raw/setup.rs` | **半** | 原 create 返回 80B Result（40B Texture+40B Err）、默认名 `"GpuOnly texture"`、init/update 走暂存+setup 并入图提交；rebuild setup 为同步提交（MODULE_STATUS 登记），Texture 布局 0x38B 待核对 |
| `CommandBuffer` 0x58B / SeqTracker / Pool | 0x140bed050.. | `raw/command_buffer.rs` | **✓** | state 机（0/1/2/3）、seq CAS track、池元素 0x18B 对齐（rebuild 用 `CommandSnapshot` 88B=0x58） |
| `SynchronizationScope1` + issue_pipeline_barrier/full | 0x140bee030/090 | `raw/barrier.rs::SynchronizationScope` + GraphicsDevice issue_* | **✓** | 布局与 10 参下发一致；full barrier 用单 memory barrier+sType 44（rebuild 用 0 数组+ALL_COMMANDS，语义等价） |
| `SubmissionEngine::update_queue_pending_submissions` | 0x1418c3300 | `raw/submit.rs::SubmissionEngine::update_queue_pending_submissions` | **半** | 原图形 Queue@+0x10 + Vec<Queue>（stride 0xb0）逐个推进 + 时间线弹出；rebuild 用 `VecDeque::retain`（语义等价，原为 ring+cap-first 环） |
| `SubmitInfo` / `carve_off_front` | — | `raw/submit.rs::{SubmitInfo,carve_off_front}` | **✓** | builder 形状对 |
| `DynamicConstants::{new,allocate}` / Chunk 绑定 | 0x140c02740 | `raw/dynamic_constants.rs` | **✓** | 256KB 上限、对齐游标、双绑定（uniform+storage）对齐 |
| `Swapchain::{extent,present,acquire,mark_stale,on_focus}` | 0x140c0e650.. | `raw/swapchain.rs` + `raw/managed/swapchain.rs` | **✓** | 句柄 flag=2、`+0x1c` 呈现状态、mark_stale 全局退役哈希表；rebuild `retire`=下次 acquire 重建（有意偏差，登记） |
| `validate_swapchain_and_acquire_image` | 0x140bfd9d0 | 无（`raw/swapchain.rs` 内） | **半** | 原 SUBOPTIMAL/OUT_OF_DATE 分派 + 租约产出；rebuild 在 managed 层近似 |
| `TrackedSwapchainTextureLease::into_present_source` | 0x140c0f170 | `raw/managed/swapchain.rs` | **✓** | `+0x08` 呈现源 + 双子句柄 drop |
| `create_raw_timeline_semaphore` | 0x1418c8710 | `raw/semaphore.rs` | **✓** | sType 9 + timeline 链 |
| `TimedFrame::{new,send_to_puffin}` | 0x140bd3cb0/4030 | `raw/profiler.rs::TimedFrame` | **✓** | 0x18B/项 + puffin scope 输出 |
| `create_gpu_profiler_output_buffer` | 0x140bff4a0 | `GraphicsDevice::create_gpu_profiler_output_buffer` | **✓** | 0x2000B 时间戳缓冲（rebuild 经 `Buffer::new` new_gpu_to_cpu） |
| `begin_label/end_label` | 0x140bf09b0/0b40 | `raw/debug.rs` | **✓** | `+0x898/+0x8a0` 分派槽一致 |
| `create_descriptor_set_layouts`（+0x230, 全局缓存） | 0x140be60a0 | `raw/shader.rs::create_descriptor_set_layouts`（rebuild 有） | **半** | 原布局 0x58B + 全局缓存哈希 `0x140d4d740`；rebuild 逐布局创建、缓存策略 U |
| `DescriptorSetState::allocate_descriptor_set` | 0x140be5a40 | `raw/descriptor_set.rs::DescriptorSetState::allocate_descriptor_set` | **✗** | 原侧**空闲池复用**（`+0x60` 空闲池弹末尾 reset + OUT_OF_POOL/FRAGMENTED_POOL 循环重试）；rebuild 用**按写入内容寻址的缓存**（`BTreeMap<Vec<u64>,Arc<Set>>`）——**候选自造机制，非原作行为**（a5 §7 明确） |
| `map_vertex_attribute_format` | 0x140be6850 | `raw/shader.rs::map_vertex_attribute_format` | **✓** | `uVar3*3+base(98/99/100)` 查表语义（rebuild 有同查表） |
| `RenderStats::snapshot_and_reset` | 0x140c02da0 | `raw/stats.rs::RenderStats::snapshot_and_reset` | **半** | 原 NotFound 体；rebuild 提供候选实现，O 证据不足 |

---

## 4. U 卡点清单

| # | 卡点 | 卡在哪儿 | 建议路径 |
|---|---|---|---|
| 1 | **Buffer+0x00 的 0xb tag 语义** | `create_buffer_impl`（0x140bee200）成功/失败路径都写 `+0x00=0xb`；既是句柄首字段又是判别位，无法确定是 `Result` 判别还是 `Buffer` 的 ManagedHandle 字段 | 反编译 0x140c03460 的 `Result` 展开 + 游戏侧 `TypedStorageBuffer::new_storage_buffer` 消费点 |
| 2 | **create_buffer 返回 80B 的字段映射** | 80B = 0x48B impl 结果 + 1 qword；无法确定 40B Buffer 如何从 0x48B 结果截取（`+0x30 usage/+0x38 mapped/+0x40 size` 是否在 Buffer 外） | 对游戏面 `SsboType` 逐字段写入点 + import_unowned 0x140bd4fc0 对照 |
| 3 | **Buffer `+0x20` 字段** | import_unowned 整拷 `+0x20`，但 device_address/mapped 不读它；是 mapped 基址还是 device_address 缓存 U | rne-read `create_mapped_buffer 0x140beea80` 的字段回填 |
| 4 | **`inner+0x7c` 与 `inner+0x28` 的内存属性判据并存** | set_buffer_debug_name 用 `+0x7c`、update_buffer_bytes 用 `+0x28`；同一分配对象两个属性位，关系 U | rne-read 分配器返回结构（`0x140c113b0/0x140c02280` 契约） |
| 5 | **`GraphicsDevice` 微函数体** | `device_uuid/physical_device/debug_utils/min_subgroup_size/refresh_memory_budget`（0x140bf84c0/8950/8960/8fa0/8f80）Ghidra 无函数体（82B NotFound） | rne-read 原字节 + `physical_device` 消费点（物理设备属性缓存位置） |
| 6 | **`create_render_pass` 缓存键的 attachment 部分** | 键 = `(ManagedHandle,u32)` + `ArrayVec<AttachmentDesc,8>`；`RenderPassAttachmentDesc` 0x10B 具体字段（load/store/layout 是否含）U | 反编译 create_render_pass（无地址；从 `draw_stream::impl$11::render` 的 PassHost::create_render_pass 调用点反推） |
| 7 | **`bind_descriptor_set` 第 4 实例地址** | 3 实例地址已定（0x1416a5380/0x1416a5910/0x1416a6010，a5 §2）；第 4 实例（Map<Enumerate<Iter<DescriptorSetBinding>>> 之一）地址未列；`vkCmdBindDescriptorSets` 具体计数/首集 U | rne-read `draw_stream::impl$11::render::closure$0` 与 `compute_pass_builder::dispatch_impl::closure$0` 内联调用点 |
| 8 | **swapchain 全局退役哈希表** | mark_stale/on_focus 尾部 200 行是全局（`plRam00000001436308f0`）哈希表（fibonacci 散列 + refcount + marker 桶），具体结构（条目 0x40B）与 disposal 关联 U | 该哈希表的构造入口（`0x14290b400/0x14290b740/0x14290b9d0` 族） |
| 9 | **`init_texture_data` jumptable** | `*desc&4` 分支的 `UNK_142b174fc` 跳表（按 `*param_4` 分派）恢复失败；各 sample/格式路径 U | rne-read 跳表 + 各目标反编译 |
| 10 | **`CommandBufferPool` 0x18B 元素初值 `{0,8,0}`** | `8` 的含义（默认 seq？队列号？）U；与 CommandBuffer 0x58B 的映射（pool 元素≠CommandBuffer）U | rne-read `create_command_buffer 0x140bef4a0` 的空闲链构造 |
| 11 | **`DynamicConstantsChunk` 双 size/双 offset 字段** | allocate 输出 `+0x08/+0x18` 均写 size、`+0x30/+0x40` 均写 offset；是 `{uniform,storage}` 双 region 还是对齐冗余 U | `into_uniform_and_storage_buffer_binding 0x140c02370` 的绑定区域读取对照 |
| 12 | **`queue_family_index` 辅族语义** | `dev+0x3c` 第二族 + `dev+0x20` bit0 门控；实际提交路径（graphics vs auxiliary）U | 反编译 0x140bfc750 全文 + `GraphExecutor` 辅助队列使用点 |
| 13 | **`RenderStats::snapshot_and_reset`** | 0x140c02da0 NotFound；统计字段布局 U | rne-read + `snapshot_render_stats`（render_frame.rs）消费点 |
| 14 | **`Texture` 0x38B 布局** | 任务给出"0x38B 已在 country-core rhapsody_types.rs 核对"；但本域 decomp 中 create_texture 输出 80B（40B Texture+40B Err），0x38B 与 40B 关系待对齐 | 对照 rhapsody_types.rs 的 `Texture{id,width,height,format}` 与 create_texture 字段回填 |
| 15 | **NotFound 微 getter 族** | `BufferDesc/TextureDesc/TextureViewDesc` 大部分 tiny getter（0x140bd4de0..0x140bd59f0 与 0x140bd5270..0x140bd55a0）Ghidra 无函数体；本表以消费端/rebuild 推断标 O(消费端)/I，未逐字节复验 | 有需时按 `rne-read.py --hex` 批量复核（PLAYBOOK §3.2） |

---

## 5. 审查轮闭合（2026-10-15，rhapsody-rawcore-closure）

> 依据：`rne-read-closure-2026-10-15.md`（本域 decomp/，字节级证据）；`capdis.py`（本次新增 capstone 反汇编脚本）。不改任何既有结论行，仅追加。

### 5.1 P0-1 Buffer 40B 生命周期链 —— 定案（U#1/#2/#3/#4 全闭合）

**`Buffer` 40B = `create_buffer`（0x140c03460）Result 的 Ok 载荷（out+0x08..0x2f）；out+0x00 的 `0xb` 是 Result Ok 判别位（不属于 Buffer）**。机器码：create_buffer_impl（0x140bee200）0x140bee69c..0x140bee6dd 写 0x48B 结果；create_buffer 0x140c035ab..0x140c035c3 截取 out+0x08..0x2f。

逐字段写入点/读取点：

| Buffer 偏移 | 类型 | 值 | 写入点 | 读取点 |
|---|---|---|---|---|
| `+0x00` | u64 | `*(context)`（ManagedResources+0x00 = 退役队列 Mutex 头；游戏面 "id"） | create_buffer_impl out+0x10 → create_buffer out+0x08 | import_unowned 整拷；TypedStorageBuffer 整块 |
| `+0x08` | u64 | **slot**（= 消费者侧 `inner`） | create_buffer_impl out+0x18 → create_buffer out+0x10 | 5 个消费者解引用 |
| `+0x10` | u32 | `*(slot+0xc)`（分配内偏移） | create_buffer_impl out+0x20 → create_buffer out+0x18 | `inner+0xc == Buffer+0x10` 校验 |
| `+0x14` | u16 | `*(context+0xa50)`（owning_device） | create_buffer_impl out+0x24 → create_buffer out+0x1c | `Buffer+0x14 == context+0xa50` 校验 |
| `+0x16` | u8 | `1`（valid） | create_buffer_impl out+0x26 → create_buffer out+0x1e | `Buffer+0x16 == 1` 校验 |
| `+0x18` | u64 | `BufferDesc.size` | create_buffer_impl out+0x28 → create_buffer out+0x20 | update_buffer_bytes / mapped_slice_mut 界校验 |
| `+0x20` | u64 | `BufferDesc+0x08..0x0f` = `{usage, alignment, memory_location}` | create_buffer_impl out+0x30 → create_buffer out+0x28 | import_unowned 整拷（槽+0x30） |

- **U#1**：`0xb` = Result Ok 判别（create_buffer 硬写 `mov [rsi], 0xb`）。
- **U#2**：80B Result 的 Ok = 40B Buffer（out+0x08..0x2f 截取）。
- **U#3**：`+0x20` = BufferDesc 尾（usage/align/mem），**不是 mapped/device_address 缓存**（修正 ALG 原注）。
- **U#4**：`inner+0x28`（BufferDesc usage/align/mem 随描述符拷入槽，位 1 门控 update 路径）与 `inner+0x7c`（分配对象 memory-properties，`==2` → "不可映射" panic，文件 managed/buffer.rs）是**两个不同字段**，非并存矛盾。

`inner` = slot（`ManagedResource<ManagedBuffer>`，FUN_1418ae8a0 返回）：`+0x0c` 偏移、`+0x10` VkBuffer、`+0x20` 容量、`+0x28` {usage/align/mem}、`+0x30` device_address、`+0x38` 内存块、`+0x7c` mem-props。

### 5.2 P0-2 GraphicsDevice::create 分段纪要（0x140bf2560，155KB）

| 段 | 证据（--unks 字符串） | 内容 |
|---|---|---|
| A 队列族枚举 | "Universal queue family: "（0x142b16008）、"Compute queue family: "（0x142b16060）、scope "rhapsody::raw::device" | 两次 0x1c-stride 扫描：`(*byte&1)==0` 选 GRAPHICS 通用队列；`(*u32&3)==2` 选 COMPUTE-only 队列；断言 `universal_queue.can_present`（0x142b162a0） |
| B 扩展/特性 | VK_KHR_swapchain / maintenance1-3 / EXT_descriptor_indexing / KHR_image_format_list / EXT_shader_demote_to_helper_invocation / KHR_buffer_device_address / EXT_full_screen_exclusive / EXT_memory_budget / EXT_subgroup_size_control / KHR_external_memory_win32 / NV_device_diagnostic_checkpoints | 扩展名表 + 特性 pNext 链（runtime_descriptor_array / buffer_device_address / shader_demote_to_helper_invocation，0x142b16695 区） |
| C 设备创建 | "Created a Vulkan device"（0x142b160b8） | vkCreateDevice + swapchain 扩展 |
| D gpu-allocator | "device_memblock_size: " / "Device memory properties: " / "Device memory budget: " / "gpu-allocator creation" / "Initialized gpu-allocator" | 分配器初始化（读内存属性/预算） |
| E 管线缓存 | "vkCreatePipelineCache"（0x142b16360） | 建管线缓存 |
| F 描述符索引 | "maxDescriptorSetUpdateAfterBindSampledImages is too low (" | bindless sampled-images 上限校验 |
| G 收尾 | "crash tracking buffer" / "Backend device fully initialized." | 崩溃缓冲 + 完成日志 |

### 5.3 P1 大函数展开

- **reclaim_command_buffer_resources（0x140bf0cc0）**：提交环 `Queue+0x20/0x28/0x30/0x38`（cap/基址/头/待决）、`+0x80` 完成时间线；环项 0x60B 时间线@+0x58；完成判定 `queue+0x80 >= 项时间线`；标记 `3` 回收命令缓冲（0x58B 回写池 `device+0x9c0` 数组 @next*0x58，池条目 0x18B `{状态, cb 数组, next}`，锁 `+0x9b0`），`4` 特殊路径；回收前 `FUN_1418c7ae0(queue+0x40)` 销毁该 cb 退役资源；尾部 `FUN_1405df600` + `func_0x140613c20(.., flags<<3, 8)` 释放项。
- **wait_idle（0x140bf8980）**：日志门 + puffin marker（0x143634e40，tag=5）+ **提交锁握手 `device+0x800`（0→8 自旋 CAS）→ `(ash+0x98)(VkDevice)` = vkDeviceWaitIdle → 解锁**。"清空待决" = 与 queue_submit 共享的提交锁握手。
- **wait_idle_and_recycle_resources（0x140bf8e60）**：`wait_idle()` → `FUN_1418c0a70(context, device)`（退役排水）→ 锁 `+0x718` → `FUN_1418c3300(+0x720)`（SubmissionEngine 推进）→ 回收图形 Queue(+0x730) + Vec\<Queue\>(+0x7e8/0xb0-stride/+0x7f0) → 解锁 → **全局退役哈希表 0x1436308f0 遍历**。

### 5.4 P1 17 行 verdict U → O（rne-read 机器码，见证据 §2）

5 个设备微函数 + get_vk_blend_mode + RenderStats + Swapchain format/any_present_successful + ImageBarrier/BufferBarrier + half_res + DynamicConstantsChunk 2 个 + ComputePipelineDesc/RasterPipelineDesc new + TrackedSwapchainTextureLease::image —— **全部机器码验证为 O**（摘要见证据 §2，逐条有 capdis/rne-read 字节）。

- `refresh_memory_budget` = **stub**：`device+0x1a` 字节门控 → 尾调 raw-init 侧滞回算法 0x140c0160f。
- `get_vk_blend_mode` = `(desc+0x1b)>>3 & 0xc` → 3 表查（0x142b139f8/0x142b13a08/0x142b13a18，内容 `{0,1,1,1}/{0,1,7,0x10}/{0,1,7,0x12}`）→ 0x1cB 输出；vulkan 枚举映射半。
- `RenderStats::snapshot_and_reset` = 3×u64 原子 xchg 快照+清零。
- `Swapchain::format` = `+0x18` u32；`any_present_successful` = `+0x1c` u8。
- `ImageBarrier::new` 0x17B / `BufferBarrier::new` 0x12B（+0xc/+0x08 的 `-1` 哨兵）。

### 5.5 P1 30 行 O(消费端) → O（rne-read --hex 批量，证据 §3）

- **TextureDesc**：构造器（new/new_2d/new_2d_array/new_3d）与 usage 位（sampled=4 / storage=8 / transfer_dst=2 / color_attachment=0x10 / depth_stencil=0x20）、sampled_with_different_format（usage|=4, flags|=0x108）、mip_levels、all_mip_levels（`floor(log2(max(w,h)))+1`）、array_elements（掩码 0x4a 机器码确认）、sample_count、extent_2d、half_res（SSE ceil/2 + 0 修正）——**全部 O**。
- **TextureViewDesc** 0x3cB 布局 + 9 个构造器机器码确认（6 个 `Option<u32>={tag,value}` @+0x00/+0x08/+0x10/+0x18/+0x20/+0x28 + base_mip_level@+0x30 + base_layer@+0x34 + bind_individual_mips@+0x38）。
- **RenderPassAttachmentDesc** 0x10B：`{format@0, clear_input@4, u32@8(U), sample_count@0xc, 3B@0xd..0xf(U)}`；**修正** rebuild/ALG 的字段序（clear_input 在 +0x04 非 +0x08；sample_count 在 +0x0c 非 +0x04）。
- **image_aspect_mask_from_format**：`format-0x7c`，>6 → COLOR_BIT(1)，否则 7 项深度/模板格式表查。

### 5.6 P1 三处卡点

- **U#8 全局退役哈希表 0x1436308f0**：fibonacci 散列（`-0x61c8864680b583eb`）、桶 0x40B、key=设备指针、lock/next/key/链 prev/时间戳字段；mark_stale / on_focus / wait_idle_and_recycle 三处遍历；构造 0x14290b400。条目精确字段表 U（与 managed/raw-init disposal 交叉）。
- **U#9 init_texture_data 跳表**：`*desc&4` 门控 → 跳表 0x142b174fc 4 目标全恢复：0x140bfe5af（panic）/ 0x140bfe2c0（0x141738fe0 助手）/ 0x140bfe30b（0x1417390e0 助手）/ 0x140bfe3aa（本地组装）；`&4==0` = panic 链。各 format 助手逐格式写算法仍 U。
- **U#11 DynamicConstantsChunk 双 region**：`into_uniform_and_storage_buffer_binding`（0x140c02370）证明 **uniform/storage 两绑定共享同一 region `{offset=chunk+0x40, size=chunk+0x18}`**（tag 6 / 5 各一，device_address 同源）；`+0x08`/`+0x30` 是分配/记账字段，非双 region。

### 5.7 STALE 3 项改判（对照表修正，不改代码）

| 原版 | ALG §3 旧判 | 复核（rebuild 当前代码 2026-10-15） | 改判 |
|---|---|---|---|
| `DescriptorSetState::allocate_descriptor_set`（0x140be5a40） | ✗（BTreeMap 内容寻址自造） | `descriptor_set.rs:653..678` 空闲池复用 + OUT_OF_POOL/FRAGMENTED_POOL 循环重试（line 621）+ 来源返还 | **✓/半**（原 +0x60 空闲池 Vec vs rebuild 池栈/来源批处理细节） |
| `bind_persistent_descriptor_set`（0x140bee150） | ✗（缺） | `binding.rs:89` 已有 free fn | **半**（仅缺"GraphicsDevice 方法"身份） |
| `bind_descriptor_set` ×4 | ✗（#4） | `binding.rs:112` 已有 free fn | **半**（缺 4 游戏闭包单态 + 设备方法身份） |

### 5.8 遗留 U

`inner+0x28` 位 1 的精确语义（usage.TRANSFER_DST vs memory_properties）半；`inner+0x0d..0x0f` 与 RenderPassAttachmentDesc `+0x08`/`+0x0d..0x0f` 身份 U；`create_mapped_buffer` 额外 24B（分配块信息/size）语义半；退役哈希表条目字段表 U；`get_vk_blend_mode` 表值→Vk 枚举映射半。

### 5.9 数据卫生（回填）

- **decomp 引用错域**：ALG §2.7 `issue_pipeline_barrier 0x140bee030` / `issue_full_pipeline_barrier 0x140bee090`（及 get_vk_buffer 0x140bef470 等）的本域 `decomp/` 无文件，真实反编译在 `evidence/render-recovery/producer-closure/n1-device/0x140bee0*.c`；本闭合轮未改既有行，仅在此登记引用路径应指向 n1-device。
- **compiled 计数**：ALG §1.1 表内 texture 26 / shader 13 / swapchain 19 / barrier 9 / binding 8 / render_pass 3 / sampler 2 与骨架实际 compiled 数（47/17/18/10/7/1/1）不符，为登记偏差；本闭合轮不追溯骨架计数来源，仅留档。
- 新增共享工具 `reconstruction/../scripts/capdis.py`（capstone x64 反汇编，`python scripts/capdis.py <va> <nbytes>`），供后续 rne-read 字节反汇编复核。
