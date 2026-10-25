# N2 取证：raw::texture / raw::buffer 机制

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra + `run-lanes.mjs`。产物：`job1-3.json`、`REPORT*.json`、32 份 `0x*.c`。
- 仅取证，未修改任何游戏代码。

## 0. 覆盖与限制

- 反编译成功 **48 个函数**（含 `raw::format` 3 个）。
- **tiny 叶子 getter 因 `run_auto_analysis:false` 未进 Ghidra 函数表**（返回 `No function at 0x…`），其语义由**消费者**推得，**未编造**。

## 1. `TextureDesc` 布局：**32 字节**（观察；父级已核对关键字段）

| 偏移 | 类型 | 含义 |
|---|---|---|
| `+0x00` | u32 | usage → `VkImageCreateInfo.usage`(0x38) |
| `+0x04` | u32 | flags（→ 0x10） |
| `+0x08` | u32 | format（→ 0x18） |
| `+0x0c` | u32 | width（→ 0x1c） |
| `+0x10` | u32 | height |
| `+0x14` | u32 | depth |
| `+0x18` | u32 | `array_elements`（→ arrayLayers） |
| `+0x1c` | u8 | `mip_levels`（→ 0x28） |
| `+0x1d` | u8 | `sample_count`（→ 0x30） |
| `+0x1e` | u8 | `kind` |

来源：`get_image_create_info 0x140bd5ed0`、`array_elements 0x140bd5540`、`div_up_extent 0x140bd55e0`。

**`kind` 枚举**：`0`=1D、`1`=1DArray、`2`=2D、`3`=2DArray、`4`=3D、`5`=Cube（layers=6）、`6`=CubeArray（layers=n×6）。

> **父级复核（`0x140bd5540.c`）**：
> ```c
> if ((param_3 != 1) && ((6 < *(byte *)((longlong)param_2 + 0x1e) ||
>      ((0x4aU >> (*(byte *)((longlong)param_2 + 0x1e) & 0x1f) & 1) == 0)))) { panic }
> *(int *)(param_2 + 3) = param_3;          // 写入字节偏移 0x18 = array_elements
> ```
> - `kind` **是 `+0x1e` 的字节**，判据 `< 6` 且掩码 **`0x4a`**（= 位 1/3/6）—— **正是"仅 kind ∈ {1, 3, 6} 允许 layers > 1"**。**确认。**
> - `array_elements` 写回 `+0x18`。**确认。**
> - 函数把 `+0x1c` **当 u32 整块拷贝** —— 与"`+0x1c`/`+0x1d`/`+0x1e` 是三个相邻 u8"完全自洽（编译器把三个 u8 合并成一次 4 字节拷贝）。**间接确认 `mip_levels`/`sample_count`/`kind` 的连排布局。**

## 2. `BufferDesc` 布局：**16 字节**（观察）

| 偏移 | 类型 | 含义 |
|---|---|---|
| `+0x00` | u64 | size |
| `+0x08` | u32 | usage |
| `+0x0c` | u16 | `alignment`（**须 < 0x10000**） |
| `+0x0e` | u8 | 内存类别枚举（`create_mapped_buffer` 要求 ∈ {2, 4}） |

来源：`create_buffer_impl 0x140bee200`、`alignment 0x140bd4f20`、`create_mapped_buffer 0x140beea80`。

> **对候选的影响**：候选 Rust 把 `BufferDesc` 建模为 `size + usage + required/preferred MemoryPropertyFlags`，**与原 ABI 不符** —— 原作是 **`u16` 对齐 + 一个小枚举**。

## 3. texture 创建路径（观察）

`create_texture`(`0x1405b9b00` 等 4 处单态化)：
`MipChainIter::new`(`0x140bd5a80`) → `get_image_create_info`(`0x140bd5ed0` 生成 `VkImageCreateInfo`, sType=14) → `vkCreateImage`（**设备结构上缓存函数指针 `@+0x1a0`**）→ 经 gpu-allocator 式 vtable 分配 → `vkGetImageMemoryRequirements`(`@+0xf0`) → `vkBindImageMemory`(`@+0xe0`) → `ManagedTexture::new`(`0x140bdc2d0`) 包裹 → 可选 `init_texture_data`(`0x140bfe270`) 上传 → 默认调试名 `"GpuOnly texture"`。

有初始数据时 `usage |= TRANSFER_DST(2)`。

## 4. buffer 路径（观察）

- `create_buffer` / `create_buffer_impl` 经 allocator vtable（`device+0x78` 子对象 `[0x21]`/`[0xe]`/`[0xc]`）分配；
- 对齐 = `max(req, desc.alignment, 默认 0x10 或 usage&0x400 时 0x40)`；
- 自旋锁 + **1 秒超时**暂存区上传（memcpy + record），暂存 desc 用 `new_cpu_to_gpu_low_prio`(`0x140bd4ea0`)；
- `update_buffer_bytes`(`0x140beefa0`)：世代 `@+0x14`、valid `@+0x16`、size 校验、host-visible（`inner+0x28 & 2`）检查后 memcpy 并 flush；
- `device_address`(`0x140bd4c00`)、`mapped_slice_mut`(`0x140bd4cd0`)。

**Texture 与 Buffer 共享同形 0x20 句柄**：`inner@+0x08`、`size@+0x10`、`generation@+0x14`、`flag@+0x16`、`len@+0x18`。

## 5. 是否只是 ash 薄包装：**判定为「否」**

依据：
- `TextureDesc` / `ViewDesc` / `BufferDesc` 是**引擎自有的描述符 ABI**；
- `get_image_create_info` / `MipChainIter` / `div_extent` 是 **desc → `VkImageCreateInfo` 的翻译与 mip/校验逻辑**；
- `create_*` / `update_*` / `set_*_debug_name` / `device_address` / `mapped_slice_mut` 含 **allocator 集成、暂存上传、锁、世代/失效校验、`Managed*` 包裹、调试名门控** —— 均非转发；
- **ash 调用被内联为设备结构上缓存的函数指针**（`+0xe0`/`+0xf0`/`+0x1a0`），**没有任何独立的 1:1 转发函数**。

`raw::format` 三个函数是**按 `VkFormat` 数值索引的内建静态表**（aspect 掩码 / block bytes / format info，覆盖 1..184 与 0..146），属**引擎自有**，非包装。

## 6. format 与既有 249 格式表的关系

`raw::format` 是**按 VkFormat 数值索引的内建静态表**，与上一轮"249 项候选表"（由官方 `vk.xml` 生成）是**不同枚举**，**不冲突**。

## 7. 内存类型

未见直接 `vkGetPhysicalDeviceMemoryProperties`；`vkGetImageMemoryRequirements` 走 `@+0xf0`，buffer 走 `+0x78` 子对象 `[0xe]`。**物理设备/内存类型选择封装在 gpu-allocator 依赖内（依赖边界，不逆向）。**

## 8. 仍未知

- `TextureViewDesc` 的精确二进制布局（全叶子未进 DB）；
- `TextureDesc.usage` 的位构造器（sampled/storage 等 21 字节叶子）；
- `BufferDesc` 内存类别枚举的取值语义；
- raw 句柄 `+0x00` 字段与 `0x50` 字节 create 结果枚举字段的含义；
- 调用图不可得（`auto-analysis` 关闭时空 xref ≠ "无 caller"）。
