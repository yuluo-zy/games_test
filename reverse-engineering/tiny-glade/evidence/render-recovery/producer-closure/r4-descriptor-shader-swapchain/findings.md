# R4 复核：descriptor / shader / swapchain / ray tracing

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra + `run-lanes.mjs`（两个独立 database `revgfx`/`revgfx2`，各 `--lanes 1`）+ **直接读 PE 镜像**。产物：`job-a/b.json`、`REPORT-a/b.json`、27 份 `0x*.c`。
- **未改动任何受限文件。**

## 0. 工具事实

- `disassemble_function` **全库返回 0 条指令**（`0x14205eff0`/`0x1418c1a10`/`0x1418c2150`/`0x140be67f0`/`0x140be6850`/`0x1418cc1b0` 均 0）——A5 的"无机器码"限制仍然成立。
- 小函数 `0x140bf8950`/`0x140be67f0`/`0x140698320`/`0x140c0e7e0`/`0x140c0ef70`/`0x140c0f120` decompile 失败。

## 1. 新技术：**直接读 PE 跳表**（本轮最有价值的方法交付）

Ghidra 对跳表报 `Bad instruction` / "jump table corrupted" 的根因是：**这些 int32 相对跳表落在 `.text` 段内，被反汇编器当成代码解码了**。绕开办法：按 PE 节表把 RVA 映射到文件偏移，直接读 int32 相对偏移再与表基址相加。

**父级已独立复现**（脚本：解析 `e_lfanew` → 节表 → RVA→file offset，读 int32，`目标 = imagebase + table_rva + rel`）：

| 表 | RVA | 项数 | 结果 |
|---|---|---|---|
| `rg_binding_to_descriptor_binding` `0x14205eff0` | `0x2fcf75c` | **4** | `[0]`→`0x14205f03f`(else)、`[1]`→`0x14205f0f4`(**tag3**)、`[2]`→`0x14205f050`(**tag4**)、`[3]`→`0x14205f4c0`(**tag5**) |
| sparse `create_descriptor_set_layout` `0x1418c1a10` | `0x2d06398` | **12** | `[0..11]`→`0x1418c1cb3`/`1d5e`/`1dc3`/`1da0`/`1cf6`/`1e00`/`234d`/`23f3`/`245b`/`2438`/`238e`/`2495` |
| compact `create_descriptor_set_layout` `0x1418c2150` | `0x2d063b0` | **6** | `[0..5]`→`0x1418c2365`/`240b`/`2473`/`2450`/`23a6`/`24ad` |

**与 R4 报告逐项一致。**

⇒ 此前标为"跳表损坏 → unknown"的**一类问题现在是可解的**。后续遇到 `Bad instruction` 的 switch 都应改用此法。

## 2. descriptor（A5 复核）

**A. 确认**：池耗尽重试机制如实。`0x140be5a40` 遇 `-0x3b9bd788`(VK_ERROR_OUT_OF_POOL_MEMORY) / `-0xc`(VK_ERROR_FRAGMENTED_POOL) 时，从空闲池弹出 set，以 **0x10 字节元素 `(handle, Arc<shared pool>)`** 推入**调用方持有**的 `Vec`(`param_3`)，再 while 重试；该 `Vec` 由 caller 消费。

**B. 更正（关键）**：`device+0x658` → **PhysicalDevice 包装**，其 `+0xd0` = `VkPhysicalDevice` 句柄（`0x140c0e650` 与 `0x1418ca030` 都用它）。
前一轮的"property 名 unknown" → **坐实为 `maxDescriptorSetUpdateAfterBindSampledImages`**。
**父级已独立复核证据**：exe 的 `.rdata` 里有 **rhapsody 自有的 panic 串** `"maxDescriptorSetUpdateAfterBindSampledImages is too low (). Must be at least "`；且**兄弟 limit 名全为 0**（`maxDescriptorSetUpdateAfterBindUniformBuffers` / `...StorageBuffers` / `maxPerStageDescriptorUpdateAfterBindSamplers` 均 `grep -c` = 0），因此**排除 ash 的 Debug 名称表**，确属 rhapsody 私有串。
（`O + 0x680` → 该字段的精确映射**仍属推断**。）

**C. 新增（两特化真正的关系）**：**不是"稀疏 / 紧凑二选一"**，而是**同一泛型的两种单态化**：
- `0x1418c1a10`（HashMap）由 `create_descriptor_set_layouts 0x140be60a0` 调用（非空 set；**空 set 直接 `vkCreateDescriptorSetLayout` 空布局**）；
- `0x1418c2150`（Vec，步长 0x28）由 `create_persistent_descriptor_set 0x140bfef70` 调用。

跳表解码显示：**sparse 支持 tag 0..11（12 臂），compact 仅 tag 0..5（6 臂）**。
tag→type 确认：`0`→7/9、`1`→6/8、`2`→1、`4`→2；`"_dyn"` 经 bswap 比对 `0x5f64796e`。**tag 3、6..11 仍 unknown。**

## 3. swapchain / surface（A6 复核）

**A. 确认**：
- `acquire_next`：槽 `base = managed+0x60`、**stride 0x38**，字段 `[0]`=VkImage、`[8]`=Arc 引用计数（`+8` 处 +1）、`[0x10]` u32、`[0x14]` u16、`[0x16]` u8、`[0x18/0x28]`=2×16B；`cursor=+0xb0`、`image_count=+0x68`、`sem/fences` 数组 `+0x78`/`+0x90`、`swapchain=+0xa0`、`state` 字节 `+0xa8 == 2`；超时 `3e9`；`SUBOPTIMAL(0x3b9acdeb)` **与** `SUCCESS` 均 Ok 且 `cursor=(i+1)%count`。
- `present`：aux 队列 `param_6*0xb0 + *(device+0x7e8)`、界 `device+0x7f0`、默认 `device+0x730`、锁 `device+0x718`；`SUBOPTIMAL` → 置 bit0。
- `resize` 只写 `desc@managed+0x48`；`set_vsync` 写 `+0x50`；`on_focus` 写 `+0xc9` + hashbrown 排空（乘子 `0x9E3779B97F4A7C15`）。
- `raw::Swapchain.{+8}` = `Arc<ManagedSwapchain>`（data@`+0x10`），`version` = `raw+0x14`、`state` = `raw+0x16`。
- **父级确认**：`0x140698320` / `0x1406982e0` 经 PDB **确为 winit `HasDisplayHandle::display_handle` / `HasWindowHandle::window_handle`**。

**B. 更正（重要）**：
- `0x1406982e0` 实填 **`RawWindowHandle::Win32{hwnd, hinstance}`**（`hwnd = Window+8`，`hinstance = GetWindowLongPtrW(hwnd, GWLP_HINSTANCE)`），**首 dword tag = 8，并非 `0xf`**。
- `Surface::create` 里 `tag == 0xc`(display) / `!= 0xf`(window) 触发的是 **`Result::unwrap()` panic**（`.rdata` 串证实）；**Windows 下二者均不命中**。
- ⇒ **A6 的"`0xc`/`0xf` = `Win32`/`Windows`"不成立**，已作废。
- `present` 的第二个非致命返回码实为 **`-0x3b9eae18` = `-1000255000`**；A6 写的 `SURFACE_LOST (-1000001000)` **与常量不符**。`OUT_OF_DATE = -1000001004` 非致命，此项正确。

## 4. ray tracing 独立复核（新方法，**无反例**）

四条证据，**父级已复核第 ①、③ 条**：

1. **扩展名 grep**：`acceleration_structure` / `ray_query` / `ray_tracing_pipeline` / `position_fetch` / `opacity_micromap` 全 **0**；正向对照 `VK_KHR_swapchain`=1、`VK_EXT_descriptor_indexing`=1。
2. **符号表**：`ray_tracing` 仅 `bind_ray_tracing`(2 单态) + `ray_trace_gi`；`acceleration` 仅 `ash::vk::definitions::AccelerationStructureKHR/NV` 的 Debug/Pointer impl（**ash 自带，非使用**）。
3. **新证据（本轮新增，父级确认）**：exe 内嵌的**启用特性串**为
   `PhysicalDeviceVulkan12Features…timeline_semaphore…descriptor_indexing…descriptor_binding_partially_bound…buffer_device_address…PhysicalDeviceShaderDemoteToHelperInvocationFeatures…VK_KHR_swapchain`
   —— **其中无任何 acceleration_structure / ray_tracing 特性**。这是比扩展名更直接的证据。
4. **obvhs 符号 14 个**；`build_gpu_bvh 0x140b0da50` 调 `build_cwbvh_from_tris 0x1417c93d0` → **CPU CW-BVH**。

**结论：A7 成立，且证据更强。**

## 5. 仍未知

- `O + 0x680` 的精确字段映射（推断）；
- tag `3`、`6..11` 的语义；
- `0xc`/`0xf` 的精确枚举（**A6 的映射已作废，目前 unknown**）；
- `present` 中 `managed+0xb9` 标志；
- swapchain 槽 `[0x18..0x37]` 的成员名。
