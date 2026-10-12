# 原作构图闭包与 rhapsody 机制取证汇总

> **文档关系**：本文（SYNTHESIS）为第一/二轮的机制总账，第十一节为复核后结论；
> **第三波（G1–G8）的详细取证见 [ROUND3_FINDINGS.md](ROUND3_FINDINGS.md)**；
> 缺口登记见 [GAPS.md](GAPS.md)；每个 agent 的原始证据在各自子目录。

> **第二轮（复核 + 补证）已完成，见文末「第十一节 复核轮更新」。** 第一节至第十节为第一轮结论；凡被复核推翻的，均已在原文件加更正块。**读本文时以第十一节为准。**

- 日期：2026-10-12。目标：`D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`（与 `D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne` 同一文件）。
- 方式：**第一轮** 7 个平行取证 agent（共用一个串行 Ghidra 会话）；**第二轮** 4 个对抗式复核 agent + 5 个补证 agent（改用并发 lane 工具）。
- **两轮都只研究、未改动任何游戏代码**：`reconstruction/rhapsody-rebuild/src/**`、`tests/**`、`Cargo.*`、`MODULE_STATUS.md`、`README.md`、`functional-coverage-audit.json` 均未触碰。

## 证据索引

**第一轮**

| 子目录 | 范围 | findings |
|---|---|---|
| `a1-executor/` | 帧循环真身 | [findings.md](a1-executor/findings.md) |
| `a2-graph-closure/` | 游戏构图闭包（核心） | [findings.md](a2-graph-closure/findings.md) |
| `a3-barrier-recorder/` | barrier / recorder 原规则 | [findings.md](a3-barrier-recorder/findings.md) |
| `a4-managed/` | managed 生命周期原 ABI | [findings.md](a4-managed/findings.md) |
| `a5-descriptor-shader/` | descriptor 分配 + shader 反射 | [findings.md](a5-descriptor-shader/findings.md) |
| `a6-allocator-swapchain/` | allocator 分叉 + swapchain/winit | [findings.md](a6-allocator-swapchain/findings.md) |
| `a7-queues-raytracing/` | 多队列 / wait-signal / ray tracing | [findings.md](a7-queues-raytracing/findings.md) |

**第二轮**

| 子目录 | 范围 | findings |
|---|---|---|
| `r1-executor-submit/` | 复核帧循环 + 提交边界 | [findings.md](r1-executor-submit/findings.md) |
| `r2-graph-barrier/` | 复核构图闭包 + barrier | [findings.md](r2-graph-barrier/findings.md) |
| `r3-managed-disposal/` | 复核 managed + **销毁桶排出** | [findings.md](r3-managed-disposal/findings.md) |
| `r4-descriptor-shader-swapchain/` | 复核 descriptor/shader/swapchain/RT | [findings.md](r4-descriptor-shader-swapchain/findings.md) |
| `n1-device/` | `raw::device` 底层 | [findings.md](n1-device/findings.md) |
| `n2-texture-buffer/` | `raw::texture` / `raw::buffer` | [findings.md](n2-texture-buffer/findings.md) |
| `n3-init-instance/` | `init` / `instance` / `physical_device` | [findings.md](n3-init-instance/findings.md) |
| `n4-rg-modules/` | `rg::` 剩余模块函数体 | [findings.md](n4-rg-modules/findings.md) |
| `c1-country-core/` | **country_core 构图生产者** | [findings.md](c1-country-core/findings.md) |

每个子目录另有 job JSON、REPORT*.json（MCP 原始返回）和保留的 `0x*.c`（Ghidra 伪代码原文）。

## 一、游戏构图闭包（原「最大生产者缺口」）

**结论：构图闭包是 country_core 游戏码，不在 rhapsody 里。**

```
country_core pass 构造 (ComputePass / DrawBuilder / UniformBinder)
  → read/write_rg_storage_buffer、multi_draw_indexed_indirect、dispatch_indirect_with
  资源解析：u64 TypeId 键 → swiss-table → u32 resource_index
  → rhapsody::rg::pass_builder::PassBuilder::add_resource_access  0x1409dca90
      账本条目 12B {u32 index, u32 access_type, u8 sync}
      ResourceSyncGroup::is_equivalent 0x140bece60 去重 + 类型合并
  → PassBuilder::render → 0xe8B pass 描述符 → 64B 句柄 → RecordedPass<PassResources>
  → RenderGraphState::compile
      ResourceInfo::calculate          0x140530430
      ResourceRegistry::populate_resources 0x140be7ba0
      CompiledRenderGraph::new         0x1402c4280   （批次 24B {start,end,payload}）
      on_retire                        0x140be9420
```

`RenderGraphState::compile` 三个单态化实例：`0x140161cd0`（9204B，主世界帧图，内联 `render_world` + UI）、`0x1401647b0`（5284B，UI/离线）、`0x140166510`（3338B，present/resolve，**caller 未闭合**）。`render_loop::render`(`0x1404e8010`) 只调前两个。

**实测访问码**（取自 producer 调用点，非枚举名反推）：`0x1a` compute 读、`0x24` draw 读、`0x37` 写 rg、`0x02` 间接绘制/派发。

## 二、帧循环

`render`(`0x1404e8010`) 每帧：
```
compile → begin_frame → execute_compiled → retire → take_resource
→ acquire_next → recorder.execute → end_frame → present → stats
→ last_known_frame_duration_ns → wait_gpu_queue_trim
```

- `GraphExecutor` **首字段 = `VecDeque<PendingSubmission>`**（元素 0x90 字节）；`begin_frame`/`wait_gpu_queue_trim` `pop_front` 并等待，`end_frame` `push_back` → **这就是帧节流**。
- `execute_compiled` 按 batch 循环（条目 0x18 字节）；**每 batch 一个命令缓冲**。
- `submit_pending_cbs`(`0x140bebdf0`) → 设备 vtable `+0x2c8` 结束末 CB → 置 `state = 2` → 组 SubmitInfo → `GraphicsDevice::queue_submit`(`0x140c04e20`) → 回执写回 recorder `+0x108..+0x117`。

## 三、同步与屏障

- `ResourceSyncGroup::new` = 全局 `AtomicU32`(`0x14362c230`) `fetch_add(1)`，初值 **2** → 组 id 从 2 起非零单调。
- 并发判定：`classify(v) = (v-0x28) < 0x10`；两类都不在该区间 → 类相同即可并发；否则需类相同 **且 group != 0 且相等**。分类器是 `vk-sync-fork` 外部依赖，**未逆向重写**。
- `add_pre_pass_barriers`(`0x1402c3d00`)：访问记录 stride **0x14**，`+0x04` 上一访问索引（`0xffffffff` = 无历史）、`+0x0c` group、`+0x10` class。**只有 `can_be_concurrent_with` 返回 false 才生成屏障** `from = prev.class → to = cur.class`。
- 屏障三阶段：**initial（每 batch）→ 逐 pass（下沉 `FUN_1402c3240`）→ export（仅 final batch）**。
- `SynchronizationScope1`：三向量（元素 0x18/0x38/0x48）+ 累计掩码（`+0x48` 初值 `0x0000200000000001`）+ `reset`/`is_empty`。

## 四、managed 生命周期

- 句柄 `RawManagedHandle` = **0x18 字节**：`+0x00` store、`+0x08` inner、`+0x10` u32 generation、`+0x14` u16 store_id、`+0x16` u8 tag。
- inner：`+0x00` atomic u64 tracked 高水位、`+0x08` atomic u32 CPU 引用计数、`+0x0c` generation、`+0x10` 资源值。
- `drop` 递减 `inner+8`；**到 0 时**向 store 待回收 Vec 推入 **16B 记录 `{inner, gen, store_id, tag}`**。
- **generation 仅在 `dispose<T>`（引用计数已为 0）时自增**；`allocate` 不复位 generation。
- **`dispose<T>` 不调用任何 `vkDestroy`** —— 只把裸句柄推入销毁桶（句柄表 Vec + 0x48 字节记录表 Vec）。**CPU 退役早于 GPU 完成由此实现。**

## 五、descriptor 与 shader

- 反射来源 = 外部 crate `CompiledShaderPipeline`（**未逆向**）；游戏侧只消费。
- `DescriptorSetState::allocate_descriptor_set`(`0x140be5a40`) = **空闲池复用**：取锁 → 弹池末尾 reuse(reset) → 空则补批 → 源空则新建；`OUT_OF_POOL_MEMORY`/`FRAGMENTED_POOL` 时回收并**循环重试**。
- 运行时 descriptor 计数 `min(max(P-32,0),8192)`，`P = *(*(device+0x658)+0x680)`（property 名 unknown）。
- 变体/阶段选择依据**完全来自外部 `CompiledShaderPipeline`**。

## 六、多队列与 ray tracing

- 队列：`create`(`0x140bf2560`) **必建**主族（`queueCount=1`），**条件建**第二份当且仅当存在 `(queueFlags & 3) == 2` 的专用 compute 族。时间线信号量只创建一次。本机 UHD 630 无专用 compute 族 → 单队列。
- **wait/signal 由批次路径 + `queue_submit` 内部时间线分配器产生，不是外部注入。**
- 无并行录制证据（单活跃 CB）。
- **ray tracing：本构建无硬件光追**（用户可自行复核）：
  - `VK_KHR_acceleration_structure` / `ray_query` / `ray_tracing_pipeline` 字符串计数 **全 0**；
  - 正向对照 `VK_KHR_swapchain` = 1、`VK_EXT_descriptor_indexing` = 1；timeline_semaphore/synchronization2/dynamic_rendering = 0（1.3 核心化，不需要扩展名）；
  - exe 内 `obvhs` 命中 6 处，上下文是调试信息里的源码路径 `.../obvhs-8e4538d3c2ff8290/a3f3b3b/src/cwbvh/builder`。
  - 原侧是 **obvhs 软件 CW-BVH（CPU 建，`build_gpu_bvh` 命名有误导）+ compute 遍历**。

## 七、窗口呈现

- `Surface::create`(`0x1400315e0`) 走 `raw-window-handle` 的 `HasDisplayHandle`/`HasWindowHandle`（winit `Window`）**trait 对象间接调用**（判别值 `0xc`/`0xf`），**不是裸 HWND**。
- managed `acquire_next`(`0x1418cb520`)：超时硬编码 **3 秒**；`SUBOPTIMAL` **当成功**交付图像（游标 `(idx+1)%count`）；`NOT_READY`/`TIMEOUT`/`OUT_OF_DATE` → Err。
- managed `present`(`0x1418cc1b0`)：`SURFACE_LOST`/`OUT_OF_DATE` **非致命**；其他 → 设备崩溃 + panic。
- 重建闭环：`validate_swapchain_and_acquire_image`(`0x140bfd9d0`) 在 OUT_OF_DATE（态 7）时 `create_swapchain`（先 retire 再 new）→ drop 旧 → 再 acquire。
- `resize` 只登记新 desc（**不 `device_wait_idle`**）。

## 八、候选实现与原机制的系统性偏差

| # | 候选 | 原作 | 来源 |
|---|---|---|---|
| 1 | 无 begin_frame/end_frame 帧边界、无 `PendingSubmission` deque、无帧节流 | 三者皆有 | A1 |
| 2 | `GraphBatchSync` 外部注入 waits/signals | waits/signals 内部产生 | A1/A7 |
| 3 | `compile_frame` 注入 trait 对象 | **单态化**直接调用 | A2 |
| 4 | 资源表**按名字**索引 | **u64 TypeId** 键 | A2 |
| 5 | `DescriptorSetState` 用 `BTreeMap<Vec<u64>,Arc<Set>>` 内容寻址 | **空闲池复用 + 池耗尽重试** | A5 |
| 6 | `ray_tracing.rs` ash 硬件加速结构包装 | **无任何对应** | A7 |
| 7 | 跟踪集合 `BTreeMap<domain,seq>` | **单个 atomic u64 高水位** | A4 |
| 8 | 7 个独立 Manager | **共享 store + 共享待回收 Vec** | A4 |
| 9 | 直接 `drop(Arc)` | **延迟入销毁桶**（不调 vkDestroy） | A4 |
| 10 | generation 递增在 slot | 递增在 **`dispose<T>`（inner）** | A4 |
| 11 | 屏障用 write 位近似"需同步组"；逐 access 展开 transition；无 `SynchronizationScope` | classify 区间 + 上一索引 + `can_be_concurrent_with` | A3 |
| 12 | `managed/swapchain.rs` **仅 13 行 stub** | `new`/`acquire_next`/`present`/`retire`/`dispose`/`desc`/`resize`/`set_vsync` 全有 | A6 |
| 13 | 每帧新建 2 semaphore + fence；无 OUT_OF_DATE 重建闭环 | **逐槽复用 + 队列锁 + validate 重建** | A6 |
| 14 | surface 只有 `from_win32`；无 `surface.rs` | trait 对象 winit 绑定 | A6 |

## 九、未闭合项（不编造）

1. **A2 访问码 与 A3 class 区间 `[0x28,0x37]` 的对应关系未闭合** —— A2 实测的 `0x1a/0x24/0x02` 都不在该区间，只有 `0x37` 是。要么有未观察到的访问码，要么账本 access_type 进入 compiled 记录前还有一次变换。
2. 访问类别枚举（0x17/0x19/0x1d/0x1e/0x24 等）语义未命名。
3. `BatchCompiler::new`(`0x140be95e0`) 与 `pass_access_range_indices`(`0x140be9690`) decompile **与** disassemble 均失败。
4. managed store 类型 tag **跳号 3**，跳表 `case 9` 未恢复。
5. swapchain 槽结构 0x38 字节字段语义未读全；present mode 精确枚举未逐证明。
6. `compile` 实例 #3(`0x140166510`) 的 caller 未闭合。
7. winit fork commit（`d03de0a0…`）未核实；`with_global_allocator` 四个同名函数名不符实。
8. 分叉分配器策略（`FUN_140c120d0/12dc0/14680/15150`）按依赖边界**未逆向**。
9. `queue_submit` 内 `VkSubmitInfo` 的 wait/signal 具体值与 stage 未逐值恢复。
10. 全部 xref 在 `run_auto_analysis:false` 下只有 DATA 引用，**caller 关系未闭合**；这不等于无 caller。

## 十、方法学限制

Ghidra 伪代码不是作者原 Rust 源码；PDB 函数签名普遍 `void()` 占位。本轮**没有任何动态证据**（未运行原游戏、未捕获帧）。所有"观察"为静态机器码推断，"推断"已在各 findings 中单独标注。父级对 A1(4 函数)、A2(1)、A3(1)、A4(2)、A5(1)、A6(1)、A7(2) 的关键论断做了独立逐行复核，发现的转录错误已更正（见 A2 findings 第 5 节）。

---

# 第十一节 复核轮更新（第二轮）

第二轮为**对抗式复核 + 补证**，共 9 个 agent。以下是推翻/新增的结论，**以本节为准**。

## 11.1 重要修正：覆盖口径（先前给用户的数字是错的）

先前给出的"rhapsody 函数面覆盖 49.6%"**只把 rhapsody 域（450 函数 / 342,968 字节）当分母**。C1 测出：

| 层 | 符号数 | 已定界字节 |
|---|---:|---:|
| rhapsody（workgraph 域） | 450 | 342,968 |
| **country_core 渲染面** | **1,829** | **475,588** |

**country_core 的渲染代码比整个 rhapsody 还大**，而它此前完全不在分母内。构图闭包正是这一层。
索引：`c1-country-core/country_core_render_symbols.json`（VA→名，1.0 MB，53 个模块分组）。

## 11.2 重要修正：barrier 账本字段标反（A2 结论作废一处）

**正确布局：`{+0x00 index:u32, +0x04 sync_group:u32, +0x08 access_code:u8}`。**
A2（及父级当时的"逐行核对"）把 `+4` 与 `+8` 标反了。

- 根因：`PassResourceAccessType::new`(`0x140b25c90`) 机器码 = `89 D0 89 CA C3` = `mov eax,edx; mov edx,ecx; ret` —— **双参数寄存器交换**，Ghidra 误建模为单参数，导致 `+8 = (char)param_4` 的**错误绑定**。
- 实参序实为 `(builder, resource_index, access_code, sync_group)`。
- **⇒ A2 当初提出的「访问码 vs class 区间 `[0x28,0x37]` 矛盾」不存在** —— 二者本来就是同一字段。`classify(v) = (v-0x28) < 0x10` 挑出的是"需要同步组"的访问码（目前含 `0x37` 与 C1 新发现的 `0x34`），这正是 `write_rg_storage_buffer_concurrent` 的用途。

## 11.3 新方法：直接读 PE 跳表（解锁此前"unknown"的一类问题）

Ghidra 报 `Bad instruction` 的根因是**int32 相对跳表落在 `.text` 段内、被反汇编器当代码解码**。解决办法：按 PE 节表把 RVA 映射到文件偏移，读 int32 相对偏移，`目标 = imagebase + table_rva + rel`。

已验证（父级独立复现）：

| 表 | 项数 | 用途 |
|---|---|---|
| `rg_binding_to_descriptor_binding` `0x14205eff0` | 4 | tag 3/4/5 分派 |
| sparse `create_descriptor_set_layout` `0x1418c1a10` | 12 | tag 0..11 |
| compact `create_descriptor_set_layout` `0x1418c2150` | 6 | tag 0..5 |

**同类问题的后续处理一律改走此法，不再记为 unknown。**

## 11.4 工具事实（影响旧证据可信度）

- **`disassemble_function` 在本配置恒返回 0 条指令**（函数表 116,272 项，但指令未持久化）。⇒ 任何"看到汇编"的断言都不可由该工具复现。**汇编类断言改由直接读 PE 复核**（父级已用此法验证 5 个叶子函数）。
- `get_xrefs_to`/`get_xrefs_from` **只有 DATA 引用，没有 CALL 边**。⇒ 前几轮所有"唯一 caller"的说法都是**伪代码直读推断，不是 xref 证据**，证据等级需降。

> ### ⚠️ 第三次补正（T5，已由父级按节表复核）
> 本文与前几轮 findings 里反复出现的表述 —— **"这些函数各仅 1 条 DATA 引用（vtable 槽 `0x1436A55xx`–`0x1436A5AF8`）⇒ 经 vtable 间接派发"** —— **是错的**。
>
> 父级按节表映射 RVA 复核：`0x1436A55F8` → RVA `0x36a55f8` → **`.pdata`**；`0x1436A5AF8` → RVA `0x36a5af8` → **`.pdata`**。
> 那两个地址在 **`.pdata` 异常目录**里（`RUNTIME_FUNCTION` 的 `BeginAddress` 字段），**不是虚表槽**。所谓"1 条 DATA 引用"**就是 `.pdata` 里指向该函数的 `BeginAddress`**，是异常处理元数据，**与调用无关**。
>
> **正确结论（T5）**：`begin_frame`、`immediate_empty_bucket`、`queue_submit` **都不是 vtable 槽**；后两者有**直接 `E8` 调用边**；`begin_frame` 在 450 函数集内无入边，是因为它的调用者 `render_loop::render`(`0x1404e8010`) 属 **tiny_glade**，**不在 `calls.json` 的函数集里**。
>
> **本镜像真正的 dyn 虚表**：`.rdata`/`.data` 中 **64 位绝对 VA** 编码的连续 `.text` 指针段，高置信 **772 张**，其中 **390 张**带标准 Rust 头 `[drop_in_place, size, align]`。**450 个 rhapsody 函数中只有 13 个确是虚表槽**（`raw::error::impl$N::fmt`、`rg::recorder::impl$1::render<…>`、`GpuAllocator::inner` 等）。
>
> **另**：`call [rip+disp]` 全镜像 4,135 条，**目标全部是 IAT 数据槽，产出 0 条内部调用边**；内部间接边静态只能解出 **65 / 18,593（0.35%）**，主因是 **14,258 条的基址是 `this` 入参**。
> 完整报告见 [t5-indirect-vtable/findings.md](t5-indirect-vtable/findings.md)。

## 11.5 各处修正与新增（摘要）

| 主题 | 结论 |
|---|---|
| 帧节流 | `wait_gpu_queue_trim` **不 pop_front**（只 peek + 有界等待）；**只有 `begin_frame` pop**。A1 说两者都 pop 是错的 |
| deque 头 | `{cap@+0, ptr@+8, head@+0x10, len@+0x18}`（A1 漏 `cap`） |
| `vkQueueSubmit` | 提交点 `(**(dev+0x88))(queue, count, pSubmits, fence=0)`；pSubmits 来自 queue 记录的 pending 环（元素 **0x60**，seq@`+0x58`） |
| wait/signal 来源 | **不在图数据里**，来自 **`dev+0xa00..0xa4f` 的装置级模板**（`get_setup_cb_submit_info`）。候选的 `GraphBatchSync` 从根上不对 |
| device 队列字段写入点 | `create 0x140bf2560` 写 `dev+0x20`(标志)/`+0x28`(count)/`+0x3c`(familyIndex)/`+0xad4`(主族) |
| **销毁桶排出** | 桶 = **9 个句柄 Vec + 1 个 0x48 记录 Vec**（A4 的"句柄表+记录表"是部分视图）。**时机 = 每次 `queue_submit` 尾部**；**条件 = 序列号 ≤ `queue+0x80`**；**排出侧不等待**；9 个 `vkDestroy*` 派发槽 |
| `FUN_140bdfcf0` | **不是 timeline 判定器，是空桶析构**（A4 说法作废） |
| tracked 集合 | **无 domain 字段**（单个 atomic u64 高水位）⇒ **仅凭它无法做跨队列复用** |
| `device+0x658→+0x680` | 坐实为 **`maxDescriptorSetUpdateAfterBindSampledImages`**（exe `.rdata` 有 rhapsody 私有 panic 串；兄弟 limit 名全 0，排除 ash 表） |
| descriptor 两特化 | **不是"稀疏/紧凑"**，是**同一泛型的两种单态化**；sparse 支持 tag 0..11，compact 仅 0..5 |
| **窗口句柄 tag** | `0x1406982e0` 填 `RawWindowHandle::Win32{hwnd,hinstance}`，**首 dword tag = 8，不是 0xf**；`0xc`/`0xf` 触发的是 `Result::unwrap()` panic。**A6 的"0xc/0xf = Win32/Windows"作废，回 unknown** |
| `present` 返回码 | 第二个非致命码是 **`-1000255000`**，不是 `SURFACE_LOST(-1000001000)`（A6 错） |
| `GraphicsDevice` 总纲 | `+0x78` 内嵌 ash 设备函数表 ⇒ **ash 偏移 X ↔ GD 偏移 `0x78+X`**；**VkDevice 在 `+0x640`**（三重确认）。前轮的 `dev+0x4f8/0x4f0/0x478` 实为 **ash 表槽位** |
| 分派槽 | `+0x298` `vkCreateCommandPool`、`+0x2b0` `vkAllocateCommandBuffers`、`+0x2c0` `vkBeginCommandBuffer`、`+0x2d0` `vkResetCommandBuffer`、`+0x2c8` `vkEndCommandBuffer`、`+0x3e0` `vkCmdPipelineBarrier` 等（sType 常量与真实 Vulkan 枚举吻合） |
| `populate_resources` | registry `[1]`=每 index u16 句柄(步 4)、`[7]`=物理记录(步 **0x40**)、`[0x19]/[0x1a]`=待处理列表；`ResourceInfo` 元素步 **0x48**，`+0x36`=用途种类 |
| `resource_pool` | 两张 SwissTable，**每键一个独立 LIFO 空闲栈**（非模糊匹配） |
| `temporal` | 状态机 `new(0) → export → 2 → 次帧 reimport → 1`；`export_temporal` 传访问码 **`0x37`**（独立印证） |
| `dynamic_constants` | 覆盖 uniform/storage 的 **bump 分配器，页 256 KiB，usage `0x20033`** |
| `TextureDesc` | **32 字节**，`kind`@`+0x1e`（掩码 `0x4a` = 仅 kind 1/3/6 允许 layers>1）、`array_elements`@`+0x18` |
| `BufferDesc` | **16 字节** = `u64 size` + `u32 usage` + `u16 alignment` + u8 内存类别。**候选建模不符** |
| texture/buffer 性质 | **不是薄 ash 包装**（desc 是引擎自有 ABI；ash 调用内联为设备函数表槽位，无 1:1 转发） |
| 实例 | 扩展/图层清单已列；**apiVersion = 1.2**（常量 `0x00402000`）；含 `VK_LAYER_KHRONOS_validation` + `VK_EXT_debug_utils`（条件） |
| 物理设备 | 选择阈值、`is_old_intel` 判据、device extensions 表、enabled features 表均已列 |
| **ray tracing** | **第四次独立确认「无硬件光追」**（本次证据面 = device extensions 表 + enabled features 表，比扩展名字符串更直接） |
| **C1 构图顺序** | **compile 闭包 36 步**、**render_world 59 步**，全部 VA→名见 `c1-country-core/frame-composition.json` |

## 11.6 修正后的候选实现偏差（合并两轮）

| # | 候选 | 原作 |
|---|---|---|
| 1 | 无 begin/end_frame 边界、无 `PendingSubmission` deque、无帧节流 | 三者皆有 |
| 2 | `GraphBatchSync` 外部注入 waits/signals | 来自 `dev+0xa00..a4f` 装置模板 |
| 3 | `compile_frame` 注入 trait 对象 | **单态化**直接调用 |
| 4 | 资源表按名字索引 | **u64 TypeId** 键 |
| 5 | `DescriptorSetState` 用 `BTreeMap` 内容寻址 | **空闲池复用 + 池耗尽重试** |
| 6 | `ray_tracing.rs` 硬件加速结构包装 | **无任何对应** |
| 7 | 跟踪集合 `BTreeMap<domain,seq>` | **单个 atomic u64 高水位（无 domain）** |
| 8 | 7 个独立 Manager | **共享 store + 共享待回收 Vec** |
| 9 | 直接 `drop(Arc)` | **延迟入销毁桶**（不调 vkDestroy） |
| 10 | generation 递增在 slot | 递增在 **`dispose<T>`（inner）** |
| 11 | 屏障用 write 位近似 | classify 区间 + 上一索引 + `can_be_concurrent_with` |
| 12 | `managed/swapchain.rs` 仅 13 行 stub | 全生命周期实现 |
| 13 | 每帧新建 2 semaphore + fence；无重建闭环 | **逐槽复用 + 队列锁 + validate 重建** |
| 14 | surface 只有 `from_win32` | trait 对象 winit 绑定 |
| 15 | `BufferDesc` = size+usage+MemoryPropertyFlags | **u64 + u32 + u16 + u8 枚举** |

## 11.7 仍未闭合（修正后）

1. **`queue+0x80` 的写者未定位**（`reclaim`/`queue_submit` 只读它）。
2. **managed store 类型 tag 跳号 3**；桶有 **9 个 destroy 槽**但只有 7 个 store；`case 9` 未恢复。
3. **tag `0x17`/`0x19`/`0x1d`/`0x1e` 语义未命名**；`SynchronizationScope1` 掩码 `+0x48` 其余位义未定。
4. `BatchCompiler::new`(`0x140be95e0`, 30B) 与 `pass_access_range_indices`(`0x140be9690`, **5B**) **函数体不可取**（只有 size 与调用边）；5 个 `raw::device` 微函数同样无函数体。
5. **7 个 `PassResources` TypeId 的具体类型名不可反推**。
6. `Surface::create` 的 `0xc`/`0xf` 精确枚举**回到 unknown**。
7. `GraphicsDevice+0x168`/`0x3f8`/`0x400`/`0x408` 身份未定。
8. **实例的 `ppEnabledExtensionNames`/`ppEnabledLayerNames` 实际数组内容**、`applicationName`/`engineName` 未恢复。
9. **调用关系普遍未闭合**：`get_xrefs_*` 无 CALL 边；A2/A3/A4 的"caller"多为伪代码直读推断。
10. **零动态证据** —— 所有结论均为静态机器码推断。

## 12. 依据汇总实施时的再核对（2026-10-12）

本节不把候选GPU运行扩大为原游戏动态证据。最新实施在 `reconstruction/rhapsody-rebuild`，当前状态以其统一功能验收报告为准；上面候选偏差表是取证时的源码快照，不代表后续代码一直未修复。

### 12.1 两处需要更正的取证总结

- 第11.7节“queue+0x80写者未定位”已被既有原码证据补上：`0x140bf0b90` 函数内 `0x140bf0c0f` 的PE原字节为 `48 89 be 80 00 00 00`，即写入 `[rsi+0x80]`；入口把queue参数置rsi。见 `consumer-chain-review-2026-10-12/dependencies/managed-repair.json`。这只定位该函数的写点，不证明完整caller/帧调度；禁止据此把queue完成值与command sequence混为一域。
- 第11.5节将 `device+0xa00..a4f` 称为wait/signal模板不充分。`get_setup_cb_submit_info 0x140bf84e0` 先take `device+0x9f8` 的state（写4），复制+8至+0x50的命令字段，`+0xa40`正对应88B Command的handle@+0x48。随后构造5个空同步向量及1条命令向量。该片段支持“活动setup command的take/end/flush”，不能直接支持“这80字节就是wait/signal数据”。同步来源应继续按完整submit caller与批次字段核实；外部timeline扩展不能记为原producer已恢复。

### 12.2 已落实的源码改动

- 构图账本已接入真实PassBuilder：按index/group/code合并、同组去重、错误保留；读→写升级保留原两条0x37/group0记录。ResourceSyncGroup从2开始。
- barrier并发判断改回访问码区间；实际后端以SynchronizationScope汇总数组和阶段掩码，在initial/pre/export对应的callback/end边界提交。
- 主图callback获得本次counter产生的CommandState、compiled registry和managed资源，descriptor保活跟随实际CB；GPU例移除第二counter/第二提交，直接读回25+17=42。
- descriptor改为pool LIFO与容量错误重试；相同内容不返回相同set。运行时计数P由真实 `maxDescriptorSetUpdateAfterBindSampledImages` 查询，布局规则再减32/cap8192。
- managed实际句柄改为单atomic高水位、共享退役来源与命令桶；generation在CPU dispose内递增，GPU排桶由已验证的sequence prefix驱动。R8生命周期资源投影仍是适配边界，不宣称全部graph资源只剩一个原ABI控制块。
- 帧pending改为deque；trim不弹出，begin节流保留最近一帧。WSI改为3秒acquire、正确SUBOPTIMAL/present错误策略、按槽同步复用及validate重建。
- 硬件RT包装保留为额外适配，排除在本构建原作功能验收之外；原软件BVH依赖/游戏consumer仍待接线。

原country_core TypeId/构图生产者、setup命令完整生产者、九类销毁桶固定次序、跨族所有权、完整原帧/画面及本地分叉依赖等仍有缺口。上述实现与原完整等价性分别记录。
