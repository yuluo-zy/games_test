# raw-core 域 审查轮闭合证据（rne-read / 机器码字节级，2026-10-15）

- agent：`rhapsody-rawcore-closure`。基线：B patched EXE；原始指令 oracle 为发行 RNE（`workspace/input/tiny-glade.exe`）。
- 本文件是 ALGORITHMS.md「审查轮闭合」小节的字节级证据附件；只记录机器码/反编译事实，不含推断。
- 工具：`rne-read.py --hex`（原字节）、`scripts/capdis.py`（capstone x64 反汇编，本次新增）、既有 decomp 交叉。
- 全部结论不改任何 reconstruction 源码。

---

## 1. P0-1 Buffer 40B 生命周期链（create_buffer_impl 全链 + 机器码定案）

### 1.1 create_buffer_impl（0x140bee200）结果 0x48B —— 成功路径机器码（0x140bee69c..0x140bee6dd）

```
0x140bee69c  mov ecx, dword ptr [rax + 0xc]      ; ecx = *(slot + 0xc)         [分配内偏移]
0x140bee69f  movzx edx, word ptr [rbx + 0xa50]   ; dx  = *(context + 0xa50)    [设备代号/u16]
0x140bee6a6  mov r8, qword ptr [rbx]             ; r8  = *(context + 0x00)     [ManagedResources+0x00]
0x140bee6a9  mov r9, rax                         ; r9  = slot（FUN_1418ae8a0 返回值）
0x140bee6ac  add r9, 0x10
0x140bee6b0  mov [rsi+0x08], r9                  ; out+0x08 = slot + 0x10
0x140bee6b4  mov [rsi+0x10], r8                  ; out+0x10 = *(context)
0x140bee6b8  mov [rsi+0x18], rax                 ; out+0x18 = slot
0x140bee6bc  mov [rsi+0x20], ecx                 ; out+0x20 u32 = *(slot+0xc)
0x140bee6bf  mov [rsi+0x24], dx                  ; out+0x24 u16 = *(context+0xa50)
0x140bee6c3  mov byte [rsi+0x26], 1              ; out+0x26 u8 = 1
0x140bee6c7  mov rax, [rbp+0x1e8]                ; rax = BufferDesc ptr
0x140bee6ce  movups xmm0, [rax]
0x140bee6d1  movups [rsi+0x28], xmm0             ; out+0x28..0x37 = BufferDesc[0..0x10) = {size u64, usage/align/mem u64}
0x140bee6d5  mov [rsi+0x38], r14                 ; out+0x38 = r14（分配块信息，半）
0x140bee6d9  mov [rsi+0x40], rdi                 ; out+0x40 = rdi（请求 size 等，半）
0x140bee6dd  mov [rsi], 0xb                      ; out+0x00 = 0xb（Ok 判别）
```

- `context` = `device+0x648`（ManagedResources）；`FUN_1418ae8a0` 的 store 实参 = `context+0x180`（= 7 store 中第 2 个，buffer store，0x178B/个，与 managed ALG §1.2 布局一致）。
- `slot` = `ManagedResourceStore<ManagedBuffer>::allocate` 返回的槽指针（AppendOnlyVec 元素，槽 stride 0x80）。

### 1.2 create_buffer（0x140c03460）Ok 返回 48B —— 机器码（0x140c035ab..0x140c035c3）

```
0x140c035ab  mov rax, [rbp+0x48]                 ; = impl.out+0x30
0x140c035af  mov [rsi+0x28], rax                 ; out+0x28 = impl.out+0x30
0x140c035b3  movups xmm0, [rbp+0x28]             ; = impl.out+0x10..0x17
0x140c035b7  movups xmm1, [rbp+0x38]             ; = impl.out+0x20..0x27
0x140c035bb  movups [rsi+0x18], xmm1             ; out+0x18..0x27 = impl.out+0x20..0x27
0x140c035bf  movups [rsi+0x08], xmm0             ; out+0x08..0x17 = impl.out+0x10..0x17
0x140c035c3  mov [rsi], 0xb                      ; out+0x00 = 0xb
```

（`[rbp+0x20..0x50)` 由 0x140c03557..0x140c03574 从 impl.out 的 16B 段平铺得到：`[rbp+0x20]=impl.out+0x08`、`[rbp+0x28]=impl.out+0x10`、`[rbp+0x30]=impl.out+0x18`、`[rbp+0x38]=impl.out+0x20`、`[rbp+0x40]=impl.out+0x28`、`[rbp+0x48]=impl.out+0x30`。）

**Ok 载荷（out+0x08..0x2f，40B）逐字段映射**：

| Buffer 偏移 | 类型 | 值 | 写入点（机器码） | 读取点 |
|---|---|---|---|---|
| `+0x00` | u64 | `*(context)`（ManagedResources+0x00 = Mutex\<Vec\<DisposedRawResourceHandle\>\> 头指针；游戏面称 "id"） | create_buffer_impl `mov [rsi+0x10], r8` → create_buffer out+0x08 | import_unowned 0x140bd4fc0 整拷到导出槽+0x10；TypedStorageBuffer 整块搬运 |
| `+0x08` | u64 | **slot**（ManagedResource\<ManagedBuffer\> 槽指针；即消费者侧 `inner`） | create_buffer_impl `mov [rsi+0x18], rax` → create_buffer out+0x10 | device_address/mapped_slice_mut/update_buffer_bytes/set_buffer_debug_name/get_maybe_tracked 解引用 |
| `+0x10` | u32 | `*(slot+0xc)`（分配内偏移） | create_buffer_impl `mov [rsi+0x20], ecx` → create_buffer out+0x18 | 校验 `inner+0xc == Buffer+0x10`（5 个消费者） |
| `+0x14` | u16 | `*(context+0xa50)`（owning_device 代号） | create_buffer_impl `mov [rsi+0x24], dx` → create_buffer out+0x1c | 校验 `Buffer+0x14 == context+0xa50` |
| `+0x16` | u8 | `1`（valid/ready flag） | create_buffer_impl `mov byte [rsi+0x26], 1` → create_buffer out+0x1e | 校验 `Buffer+0x16 == 1` |
| `+0x18` | u64 | `BufferDesc.size` | create_buffer_impl `movups [rsi+0x28], xmm0`（低 8B）→ create_buffer out+0x20 | update_buffer_bytes 越界校验；mapped_slice_mut 界内校验；ssbo.rs `+0x18=desc.size` |
| `+0x20` | u64 | `BufferDesc+0x08..0x0f` = `{usage u32, alignment u16, memory_location u8, pad}` | create_buffer_impl `movups [rsi+0x28], xmm0`（高 8B）→ create_buffer out+0x28 | import_unowned 整拷（槽+0x30）；游戏面 "gap20" |

> **定案**：`+0x00` 的 `0xb` 是 **create_buffer 的 Result 判别位（out+0x00，不属于 40B Buffer）**；Buffer 首字段是 `*(context)`（ManagedResources 退役队列头指针），非 tag。`out+0x00` 由 create_buffer 硬写 `mov [rsi], 0xb`（0x140c035c3），Err 分支写 impl.out+0x00（非 0xb）。**U#1 定案：0xb = Result Ok 判别**；**U#2 定案：80B Result 的 Ok 载荷即 40B Buffer，截取点 = out+0x08..0x2f**。

### 1.3 `inner` = slot，槽内（ManagedResource\<ManagedBuffer\>）布局 —— 消费端交叉

| inner 偏移 | 值/来源 | 消费证据 |
|---|---|---|
| `+0x00` | u64 `0`（allocate 写 0；seq 原子） | FUN_1418ae8a0 `*puVar10 = 0` |
| `+0x08` | u32 `1`（allocate 写 1） | FUN_1418ae8a0 `*(u32)(puVar10+1) = 1` |
| `+0x0c` | u32 偏移（Buffer+0x10 须等于它） | device_address 0x140bd4c00: `*(inner+0xc) == Buffer+0x10` |
| `+0x10` | VkBuffer（描述符 desc[0]） | FUN_1418ae8a0 槽拷贝（desc[0] = r15 = VkBuffer） |
| `+0x18` | 描述符 desc[1]（半） | — |
| `+0x20` | size / 容量（desc[2]） | update_buffer_bytes `puVar6+param_8 <= *(inner+0x20)` |
| `+0x28` | `{usage u32, align u16, mem u8, pad}`（desc[3] = BufferDesc+0x08..0x0f） | update_buffer_bytes `*(inner+0x28) & 2` 门控直接写路径 |
| `+0x30` | device_address 缓存（desc[4] = r12） | device_address 读 `inner+0x30`；set_buffer_debug_name 作命名句柄 |
| `+0x38` | 内存块指针（desc[5]，取 mapped ptr） | mapped_slice_mut `func_0x140c113b0(inner+0x38)` |
| `+0x7c` | u8 memory-properties（desc[13] 高位字节） | mapped_slice_mut `inner+0x7c==2` → panic；set_buffer_debug_name `!=2` → 命名 |

> **U#4 定案（双属性位并存）**：`inner+0x28` 与 `inner+0x7c` 是**两个不同字段**——`+0x28` 是 BufferDesc.usage/align/mem 随描述符拷入槽（位 1 门控 update_buffer_bytes 的写入路径，panic 串 0x142b15a78/8a 引用 "Buffer update data / mapped buffer memory"，panic 文件 raw/buffer.rs）；`+0x7c` 是分配对象的 memory-properties（panic 文件 raw/managed/buffer.rs，`==2` 触发 "不可映射" panic）。两者语义判据存在（usage.TRANSFER_DST vs memory_properties 的精确位义）标 **半**。

### 1.4 `+0x20` 字段定案

`Buffer+0x20 = BufferDesc+0x08..0x0f（usage/align/mem）`，由 create_buffer_impl 写、create_buffer 透传、import_unowned 整拷。**不是 mapped 基址/device_address 缓存**（ALG 原注错误，本次修正）。create_mapped_buffer 的额外字段（out+0x28..0x3f = usage/align/mem、分配块信息、size）是 Buffer 之后追加的 mapped 相关信息，非 Buffer+0x20。

### 1.5 create_mapped_buffer（0x140beea80）返回 0x40B

机器码 0x140beea80：先断言 `desc.memory_location ∈ {2,4}`（panic 0x142b158f8 = "assertion failed: matches!(desc.memory_location, MemoryLocation::CpuToGpu | MemoryLocation::GpuToCpu)"）；调 create_buffer_impl；tag==0xb 时写 `{0xb, impl.out+0x10, impl.out+0x18, impl.out+0x20, impl.out+0x28, impl.out+0x30, impl.out+0x38, impl.out+0x40}`（0x40B）。返回 = 40B Buffer + 24B（usage/align/mem、分配块信息、size；语义半）。

---

## 2. 17 行 verdict U 升级（rne-read 机器码，全部 O）

| VA | 函数 | 机器码结论 |
|---|---|---|
| 0x140bf84c0 | `device_uuid` | `mov rcx,[rdx+0x658]; movups [rax], [rcx+0x690]` → 返回 16B UUID（device+0x658 配置管理器 +0x690） |
| 0x140bf8950 | `physical_device` | `mov rax,[rcx+0x658]; add rax,0x10` → 返回 `(device+0x658)+0x10` |
| 0x140bf8960 | `debug_utils` | `mov rdx,[rcx+0x870]; lea rax,[rcx+0x870]; cmovz rax,rdx` → Option：非空返回 +0x870，空返回 0 |
| 0x140bf8f80 | `refresh_memory_budget` | `cmp byte [rcx+0x1a],0; je ret; mov rcx,[rcx+0x658]; add rcx,0x10; jmp 0x140c0160f` → **stub**：device+0x1a 门控，尾调 raw-init 侧滞回算法（0x140c0160f，与 raw-init 0x140c01620 区一致） |
| 0x140bf8fa0 | `min_subgroup_size` | `mov eax,[rcx+0x40]; mov edx,[rcx+0x44]; ret` → 返回 {u32@+0x40, u32@+0x44} |
| 0x140be67f0 | `get_vk_blend_mode` | `movzx ecx,byte [rdx+0x1b]; shr ecx,3; and ecx,0xc;` 3 表查 `{0x142b139f8, 0x142b13a08, 0x142b13a18}` → 输出 0x1cB `{u32 src, u32 1, u32 dst, …, u32 op, u64 0xf00000000}`；3 表内容 = `{0,1,1,1}/{0,1,7,0x10}/{0,1,7,0x12}`（vulkan 枚举映射半） |
| 0x140c02da0 | `RenderStats::snapshot_and_reset` | 3 × `xchg [rdx+{0,8,0x10}], {0,0,0}` 原子交换 → `{out+0, out+8, out+0x10}` 三个 u64 快照并清零 |
| 0x140c0e7e0 | `Swapchain::format` | `mov eax,[rcx+0x18]; ret` → 返回 u32 @ Swapchain+0x18 |
| 0x140c0ef70 | `any_present_successful` | `movzx eax,byte [rcx+0x1c]; ret` → 返回 u8 @ +0x1c 呈现状态 |
| 0x140bd65e0 | `ImageBarrier::new` | 0x17B：`{u64@0=img, u64@8=param_6, u64@0xc=-1(哨兵), u8@0x14=param_3, u8@0x15=param_4, u8@0x16=0}` |
| 0x140bd6610 | `BufferBarrier::new` | 0x12B：`{u64@0=buf, u64@8=-1(哨兵), u8@0x10=param_3, u8@0x11=param_4}` |
| 0x140bd5730 | `TextureDesc::half_res` | SSE：w/h = `x - (x>>1)`（= ceil(x/2)），0 修正（pcmpeqd+psubd 使 0→1）；depth = `>>1`；写回 +0x0c/+0x10/+0x14 |
| 0x140c02280 | `DynamicConstantsChunk::device_address` | `mov rax,[rcx+0x10]; ret` → 返回 chunk+0x10 |
| 0x140c02290 | `into_gpu_lifetime_tracker` | `movups xmm0,[rdx+0x20]; movups xmm1,[rdx+0x30]; movups [rcx+0x10],xmm1; movups [rcx],xmm0; ret` → 拷 32B（{src+0x20, src+0x30} → out） |
| 0x14003bb80 | `ComputePipelineDesc::new` | `c3` 纯 ret → 空构造（结构体零初始化） |
| 0x140be6760 | `RasterPipelineDesc::new` | 0x1cB：`{src[0..0x10) → out, src+0x10 → out+0x10, param_3(u32) → out+0x18}` |
| 0x140c0f120 | `TrackedSwapchainTextureLease::image` | 克隆：`lock inc [*(lease+0x18)+8]`（refcount++）+ 拷贝 `{lease[0..0x10), lease+0x10, lease+0x18, +0x20, gen@+0x24, flag@+0x26, &lease+0x28}` → 新租约 |

---

## 3. 30 行 O(消费端) getter 复验（rne-read --hex 批量，全部 O）

### 3.1 TextureDesc 构造/设置器（0x140bd5270..0x140bd5460 区）

| VA | 函数 | 机器码 |
|---|---|---|
| 0x140bd5270 | `new(extent, kind, format)` | `kind→+0x1e, format→+0x08, extent→+0x0c, depth→+0x14, mip/sample=1@+0x1c` |
| 0x140bd52a0 | `new_2d(w,h,format)` | `format→+0x08, w→+0x0c, h→+0x10, 0x100000001→+0x1c`（mip=1,sample=1） |
| 0x140bd52e0 | `new_2d_array(w,h,array,format)` | `format→+0x08, w/h→+0x0c/+0x10, depth=1@+0x14, array→+0x18` |
| 0x140bd5310 | `new_3d(extent, format)` | `kind=4@+0x1e, format→+0x08, extent→+0x0c, depth→+0x14, mip/sample=1` |
| 0x140bd5340 | `format` 设置器（copy desc + 换 format） | `r8d→[rdx+0x08]`，整拷 0x20B |
| 0x140bd5370 | `sampled` | `or byte [rdx], 4`（usage \|= SAMPLED） |
| 0x140bd5390 | `storage` | `or byte [rdx], 8` |
| 0x140bd53b0 | `transfer_dst` | `or byte [rdx], 2` |
| 0x140bd53d0 | `color_attachment` | `or byte [rdx], 0x10` |
| 0x140bd53f0 | `depth_stencil_attachment` | `or byte [rdx], 0x20` |
| 0x140bd5410 | `sampled_with_different_format` | `or [rdx], 0x108000004`（usage \|= 4(SAMPLED)，flags \|= 0x108） |
| 0x140bd5430 | `format`（设置器同族） | 同上格式 |
| 0x140bd5460 | `mip_levels(n)` | `n→+0x1c`，整拷 |
| 0x140bd54a0 | `all_mip_levels` | bsr(log2) + 修正 → `mip_count = floor(log2(max(w,h))) + 1` |
| 0x140bd5540 | `array_elements(n)` | `n==1` 直接写；否则 `kind<6 && (0x4a >> kind & 1)`（kind∈{1,3,6}）校验后 `n→+0x18`（掩码 0x4a 机器码确认） |
| 0x140bd55a0 | `sample_count(n)` | `n→+0x1d`，整拷 |
| 0x140bd5790 | `extent_2d` | `mov rax,[rcx+0x0c]; ret` → `{w@+0xc, h@+0x10}` |

> usage 位掩码逐值确认：sampled=4、storage=8、transfer_dst=2、color_attachment=0x10、depth_stencil=0x20（i_api 断言一致）。

### 3.2 TextureViewDesc（0x140bd57a0..0x140bd59f0，0x3cB 布局机器码确认）

布局：`{Option<u32> view_type@+0x00, Option<u32> usage@+0x08, Option<u32> format@+0x10, Option<u32> aspect_mask@+0x18, Option<u32> level_count@+0x20, Option<u32> layer_count@+0x28, u32 base_mip_level@+0x30, u32 base_layer@+0x34, bool bind_individual_mips@+0x38}`。每个 Option = `{u32 tag(0=None/1=Some), u32 value}`。

| VA | 函数 | 机器码 |
|---|---|---|
| 0x140bd57a0 | `new` | 6 个 Option tag 清零 + `+0x30..`（base_mip/base_layer/bind） |
| 0x140bd57e0 | `view_type` | `{1, r8d}→+0x00/+0x04` |
| 0x140bd5820 | `usage` | `{1, r8d}→+0x08/+0x0c` |
| 0x140bd5860 | `format` | `{1, r8d}→+0x10/+0x14` |
| 0x140bd58a0 | `aspect_mask` | `{1, r8d}→+0x18/+0x1c` |
| 0x140bd5920 | `level_count` | `{1, r8d}→+0x20/+0x24` |
| 0x140bd59a0 | `layer_count` | `{1, r8d}→+0x28/+0x2c` |
| 0x140bd58e0 | `base_mip_level` | `r8d→+0x30` |
| 0x140bd5960 | `base_layer` | `r8d→+0x34` |
| 0x140bd59f0 | `bind_individual_mips` | `r8b→+0x38` |

### 3.3 RenderPassAttachmentDesc（0x140be6780..0x140be67c0，0x10B 机器码确认）

布局：`{u32 format@+0x00, u32 clear_input@+0x04, u32 @+0x08(U), u8 sample_count@+0x0c, +0x0d..0x0f 3B(U)}`。

| VA | 函数 | 机器码 |
|---|---|---|
| 0x140be6780 | `new(format)` | `format→+0x00, +0x04=0, +0x08=0, sample_count=1@+0x0c` |
| 0x140be67a0 | `clear_input` | `+0x04 = 1`（置位） |
| 0x140be67c0 | `sample_count(n)` | `n→+0x0c`，整拷 0x10B（含 +0x0d..0x0f） |

> 修正：ALG/rebuild 的 `{format, samples, clear_input, depth_stencil}` 字段序与机器码不符（clear_input 在 +0x04、sample_count 在 +0x0c）；`+0x08` 与 `+0x0d..0x0f` 身份 U。

### 3.4 image_aspect_mask_from_format（0x140bd6630）

`ecx = format - 0x7c; if (ecx > 6) return 1(COLOR_BIT); else return table[format-0x7c]`（7 项表，覆盖 D16/X8_D24/D32/S8/D16S8/D24S8/D32S8 深度/模板格式）。

---

## 4. 大函数分段纪要（反编译 grep + 字符串解析）

### 4.1 GraphicsDevice::create（0x140bf2560，155KB）

字符串分段（--unks 解析）：

| 段 | 证据（字符串/常量） | 内容 |
|---|---|---|
| A 队列族枚举 | 0x142b16008 "Universal queue family: "、0x142b16060 "Compute queue family: "、0x142b16030 "rhapsody::raw::device"（puffin scope） | 两次 0x1c-stride 扫描：① `(*byte & 1)==0` 选 GRAPHICS 通用队列；② `(*u32 & 3)==2` 选 COMPUTE-only 队列。断言 "universal_queue.can_present"（0x142b162a0） |
| B 扩展/特性 | 0x142b16726..0x142b1685a（VK_KHR_swapchain、VK_KHR_maintenance1/2/3、VK_EXT_descriptor_indexing、VK_KHR_image_format_list、VK_EXT_shader_demote_to_helper_invocation、VK_KHR_buffer_device_address、VK_EXT_full_screen_exclusive、VK_EXT_memory_budget、VK_EXT_subgroup_size_control、VK_KHR_external_memory_win32、VK_NV_device_diagnostic_checkpoints） | 扩展名表 + 特性结构 pNext 链（0x142b16695 runtime_descriptor_array/buffer_device_address/shader_demote_to_helper_invocation） |
| C 设备创建 | "Created a Vulkan device"（0x142b160b8） | vkCreateDevice；swapchain 扩展名 0x142b16718 |
| D gpu-allocator | "device_memblock_size: "（0x142b16140）、"Device memory properties: "（0x142b161a8）、"Device memory budget: "（0x142b161e8）、"gpu-allocator creation"（0x142b16210，puffin）、"Initialized gpu-allocator"（0x142b16260） | 分配器初始化：读设备内存属性/预算，建 gpu-allocator |
| E 管线缓存 | "vkCreatePipelineCache"（0x142b16360） | 建管线缓存 |
| F 描述符索引 | "maxDescriptorSetUpdateAfterBindSampledImages is too low ("（0x142b16420） | bindless sampled-images 上限校验 |
| G 收尾 | "crash tracking buffer" / "Backend device fully initialized."（0x142b16320） | 崩溃跟踪缓冲 + 完成日志 |

（含 "Waiting for the device to be idle..." 0x142b16360 区——create 内建 wait_idle。）

### 4.2 Queue::reclaim_command_buffer_resources（0x140bf0cc0，25KB）

- 提交环：`Queue+0x20` cap、`+0x28` 环基址、`+0x30` 头、`+0x38` 待决数、`+0x80` 最近完成时间线。
- 循环（pending != 0）：环项 0x60B，时间线 @+0x58；`queue+0x80 < 项时间线` 则 break（未完成）。
- 完成项分派：标记 `0x4`（特殊路径）；`3` = 命令缓冲回收：
  - `FUN_1418c7ae0(queue+0x40, …)` = CommandBufferDisposalBuckets 销毁该 cb 的退役资源；
  - 锁 `device+0x9b0`（池锁 0→1）；池 = `device+0x9c0` 数组（0x18B/项：`{状态, cb 数组基址@+8, next@+0x10}`）、`+0x9c8` 个数；
  - 回写 cb（0x58B）到 `池.cb_array + next*0x58`（state/seq/flags 各字段逐拷）；`next+1`；
  - 解锁池；`FUN_1405df600` 释放项资源；项尾部 `func_0x140613c20(.., flags<<3, 8)` 释放。

### 4.3 GraphicsDevice::wait_idle（0x140bf8980，6.7KB）

- 日志门 `func_0x140c16a00()`（>0 才发）；消息 "Waiting for the device to be idle..."（0x142b154fe 区）+ puffin scope "rhapsody::raw::device"。
- `FUN_1418c1380(0x143634e40, &e, 1)`：puffin marker 写全局 0x143634e40（tag=5）。
- 提交锁握手：`device+0x800` 自旋 CAS（0→8）；持锁后 `(**(code**)(device+0x98))(*(device+0x640))` = **vkDeviceWaitIdle**；解锁 8→0。
- 收尾 drop 日志 guard。

### 4.4 GraphicsDevice::wait_idle_and_recycle_resources（0x140bf8e60，7.8KB）

```
wait_idle()                              # 4.3
FUN_1418c0a70(device+0x648, device)      # ManagedResources::take_out_dropped_waste（退役队列排水）
锁 device+0x718（0→1）                    # 回收锁
FUN_1418c3300(device+0x720, device+0x78) # SubmissionEngine::update_queue_pending_submissions
FUN_140bf0cc0(device+0x730)              # 回收图形 Queue
for (i < device+0x7f0) FUN_140bf0cc0(device+0x7e8 + i*0xb0)   # 回收 Vec<Queue>
解锁 0x718
# 尾部：全局退役哈希表 0x1436308f0 遍历（见 §5）
```

---

## 5. U#8 全局退役哈希表（0x1436308f0）

来自 wait_idle_and_recycle_resources 尾部（0x140bf8e60）+ mark_stale/on_focus 同源：

- 根指针 `plRam00000001436308f0`；表为空时 `FUN_14290b400()` 创建。
- 桶散列：`key * -0x61c8864680b583eb >> shift`（fibonacci 黄金比散列）；key = 设备指针（`pcVar1`）。
- 桶数组元素 **0x40B**：`{lock u8@0, next@0x08, key(设备)@0x10, 链 prev@0x18, …}`；`plVar6[1]`=桶数、`plVar6[3]`=shift/mask。
- 遍历逻辑：按设备指针找到桶 → 锁桶（`*桶=1`）→ 摘链（匹配 key）→ 递减桶引用计数（`*puVar2 - 1`）→ 时间戳（`FUN_140d635e0` 当前时间、`FUN_140d63a20` 下一 tick）→ 条目 `[3]/[4]` 记时戳、`[5]` 随机种子滚动。
- 共享者：`mark_stale`（0x140c0e950）、`on_focus`（0x140c0e6b0）、`wait_idle_and_recycle_resources`（0x140bf8e60）三处；用途 = 按设备把退役交换链图像/资源记入全局哈希表供后续刷新。条目精确字段表 U（构造入口 0x14290b400/0x14290b740 族未深挖，属 raw-init/managed 交叉）。

---

## 6. U#9 init_texture_data 跳表（0x140bfe270）

- `if (*desc & 4)`（desc 字节位 2 门控）→ **jumptable 0x142b174fc**：4 个 4B 有符号偏移，索引 = `*param_4`（0..3）。
- 已解析目标（rne-read 计算）：

| 索引 | 目标 | 内容 |
|---|---|---|
| 0 | 0x140bfe5af | panic（`lea` 消息串 + `mov edx,0x28` 行号；"not implemented" 类） |
| 1 | 0x140bfe2c0 | 调 `0x141738fe0`（format 助手）直接写 |
| 2 | 0x140bfe30b | 调 `0x1417390e0`（format 助手）直接写 |
| 3 | 0x140bfe3aa | 本地组装 + 调 format 助手直接写 |

- `*desc & 4 == 0` 分支 = panic 链（0x142b16ff8 "not implemented" 区 → `Result::unwrap()` Err 0x142b15188）。**跳表 U#9 闭合：4 目标全部恢复**；各 format 助手的逐格式写算法（mip 切块/对齐）仍 U。

---

## 7. U#11 DynamicConstantsChunk 双 region 定案（0x140c02370）

`into_uniform_and_storage_buffer_binding`（0x140c02370）：

- 读 chunk：`+0x18`（size）、`+0x20..0x2f`（buffer 句柄区）、`+0x30`（offset）、`+0x38`（u8 flag）、`+0x40`（offset）。
- `get_maybe_tracked(store, &buffer, 0)` 取资源 → `资源+0x20` = device_address（= slot+0x30）。
- 输出（0x2cB）：
  - **uniform 绑定** @+0x00：`{tag=6, device_address@+0x08, offset=chunk+0x40@+0x10, size=chunk+0x18@+0x18}`
  - **storage 绑定** @+0x20：`{tag=5, device_address@+0x28, offset=chunk+0x40@+0x30, size=chunk+0x18@+0x38}`
  - 尾部：`chunk+0x30`（offset）@+0x50、buffer 区 @+0x40/+0x48、flag@+0x5c。

> **U#11 定案**：chunk 的 `+0x08/+0x18` 双 size 与 `+0x30/+0x40` 双 offset **不是双 region**——两个绑定共享同一 region `{offset=chunk+0x40, size=chunk+0x18}`；`+0x18` 是绑定 size、`+0x40` 是绑定 offset；`+0x08`（分配 size）、`+0x30`（chunk 记账 offset）为簿记字段。`DynamicConstantsChunk::device_address`（0x140c02280）= 返回 `chunk+0x10`。

---

## 8. STALE 3 项复核（rebuild 当前代码，2026-10-15）

| 原版 | ALG §3 旧判 | rebuild 现状（复核） | 改判 |
|---|---|---|---|
| `DescriptorSetState::allocate_descriptor_set` | ✗（BTreeMap 内容寻址自造） | `descriptor_set.rs:653..678` = `DescriptorPoolQueue` 空闲池复用 + 来源返还 + `state.allocate` 内 `ERR_OUT_OF_POOL_MEMORY \| ERR_FRAGMENTED_POOL` 循环重试（line 621）；`cached_set_count()` 注释"内容缓存已经移除" | **✓/半**（原 +0x60 空闲池 Vec vs rebuild 池栈 + 来源批处理细节） |
| `bind_persistent_descriptor_set` | ✗（缺） | `binding.rs:89` 已有 `bind_persistent_descriptor_set` free fn（同底层 cmd_bind_descriptor_sets） | **半**（缺"GraphicsDevice 方法"身份，非功能） |
| `bind_descriptor_set` ×4 | ✗（#4） | `binding.rs:112` 已有 `bind_descriptor_set` free fn | **半**（缺 4 游戏闭包单态 + 设备方法身份） |
