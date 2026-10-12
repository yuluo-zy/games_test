# G1 取证：PE 直读补齐 rhapsody 微叶子

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 方法：**PE 节表 RVA→文件偏移 + capstone 反汇编**，绕过 Ghidra（对微小函数不建函数）。
- 产物：`pe-leaves.json`（159 项）、分组 `.asm`（barrier/handle/buffer/texture/device/swapchain/shader/binding/rg/raw/misc）、`_tools/pe_dump.py`、`_tools/build_leaves.py`、`_dump.json`。
- 仅取证，未改任何禁改文件。

## 0. 父级复核

- `pe-leaves.json`：**159 项，size 合计 = 46,091 字节**，与 GAPS.md 登记的 A 类缺口**完全一致**。
- 置信度分布：**high 92 / medium 63 / low 4**。
- 字段名串在 exe 中实测存在：`resource_checksum`(1)、`RuntimeDeviceId`(2)、`ManagedResourceChecksum`(2)、`TextureView`(2)、`PersistentDescriptorSet`(2)、`MarkedUnused`(1)、`memory_location`(3)、`CpuToGpuLowPrio`(2)、`GpuToCpu`(3)。**⇒ 字段名类结论有 `.rdata` 实证，非推断。**

> ### **⇒ rhapsody 域的字节面已 100% 反汇编完成。**
> 291（Ghidra 伪代码）+ 159（PE 反汇编）= **450/450 函数、342,968/342,968 字节**。

## 1. `RawManagedHandle` 的真实字段名（**更正 A4 的"generation"**）

`handle::fmt`(`0x140bd69a0`) 从 `.rdata` 解出结构字段名：

| 偏移 | **原名** | A4/A5 的叫法 |
|---|---|---|
| `+0x00` | （**未命名，且不参与 `eq`、不打印**） | "store" |
| `+0x08` | **`resource`** | "inner" |
| `+0x10` | **`resource_checksum`** | ~~"generation"~~ |
| `+0x14` | **`device_id`**（newtype `RuntimeDeviceId`） | "store_id" |
| `+0x16` | **`resource_type`** | "tag" |

另有 newtype `ManagedResourceChecksum`（`impl$44`）。

> **更正**：A4/A5 的"generation 计数器"在原文里叫 **`resource_checksum`**；A4 的"store id"叫 **`device_id`**。**行为不变**（旧句柄靠 `handle+0x10 != inner+0xc` 被拒的机制成立），**但命名须改**。
> `+0x00` 字段**既不参与相等判定、也不被打印** ⇒ 其身份仍未知（GAPS C 类保留一项）。

## 2. `handle::clone` / `eq`（此前一直缺的逐字节语义）

- **`handle::clone`(`0x140bd6b00`, 48 B)**：`mov r8,[rdx+8]; lock inc dword [r8+8]` → **`inner+8` 的原子引用计数 +1**，随后按字节整份拷贝 `{store@0, resource@8, checksum@0x10, device_id@0x14, resource_type@0x16}`。**无分配。**
- **`handle::eq`(`0x140bd6b30`, 23 B)**：`mov rax,[rcx+8]; mov rcx,[rcx+0xf]; xor; or; sete` → 相等 ⟺ **`resource` 相等 且 `+0x0f..+0x16` 的 8 字节相等**（覆盖 resource + checksum + device_id + resource_type）。**`+0x00`(store) 不参与。**

## 3. **消解 store tag 语义（更正 GAPS C2 的"跳号 3"）**

| 名称表 | 内容 |
|---|---|
| `impl$57` | **跟踪态**：`{Untracked=0, Tracked=1, MarkedUnused=2}` |
| `impl$51` | **资源类型**：`{Texture=0, Buffer=1, Swapchain=2, TextureView=3, RenderPass=4, Sampler=5, PersistentDescriptorSet=6, Framebuffer=7}` |

> **更正**：tag 空间是 **0..7 连续 8 个，没有跳号**。A4 观察到的"7 个 store、tag 集合 `{0,1,2,4,5,6,7}`"与 A4 的"跳 3"应改为：**枚举有 8 个变体，而 store 数组只有 7 个 —— `TextureView`(tag 3) 没有独立 store**（与 `Texture` 共用销毁路径）。这与 R3 的"桶有 9 个 destroy 槽"之间的差值（9 vs 8）仍需单独解释。

## 4. barrier 叶子（全部解出）

- `SynchronizationScope1`：三个 `Vec{cap@0, ptr@8, len@0x10}` 位于 `+0x00` / `+0x18` / `+0x30`；`mask@+0x48 = 0x200000000001`；`is_empty` 查三处 len；`reset` 清 len + mask。
- **`image_aspect_mask_from_format`(`0x140bd6630`)** 查表 @ `0x142b0ffb8`：`(fmt - 0x7c) ≤ 6` 时 `0x7c/0x7d/0x7e` → **2(DEPTH)**、`0x7f` → **4(STENCIL)**、`0x80/0x81/0x82` → **6(DEPTH|STENCIL)**；否则 **1(COLOR)**。
- `BufferBarrier::new` = `{u64@0, u64 = -1@8, u8@0x10, u8@0x11}`。
- `ResourceSyncGroup::new` = 对全局 `AtomicU32@0x14362c230` 做 `lock xadd 1`（返回**旧值**）；`is_equivalent` = id 相等。

## 5. descriptor 构造器全解（字段名为 `.rdata` 实证）

- **`BufferDesc` = 16 B**：`{size@0, usage@8, alignment@0xc, memory_location@0xe}`；构造器断言 `size > 0`。
  **`memory_location` 枚举**：`1=GpuOnly, 2=CpuToGpu, 3=CpuToGpuLowPrio, 4=GpuToCpu`。
- **`TextureDesc` = 32 B**：`usage@0`(VK 位)、`format@8`、`width@0xc`、`height@0x10`、`depth@0x14`、**`array_elements@0x18`**、`mip_levels@0x1c`、`sample_count@0x1d`、`kind@0x1e`。
  **`kind` 枚举**：`{Tex1d=0, Tex1dArray=1, Tex2d=2, Tex2dArray=3, Tex3d=4, Cube=5, CubeArray=6}`（与 N2 的 `0x4a` 掩码"仅 kind 1/3/6 允许 layers>1"完全一致）。
- **`TextureViewDesc`**：6 个 8 字节 `Option` 字段 @ `0`/`8`/`0x10`/`0x18`/`0x20`/`0x28`（`view_type`/`usage`/`format`/`aspect_mask`/`level_count`/`layer_count`），`base_mip@0x30`、`base_layer@0x34`、`bind_individual_mips@0x38`。

## 6. `raw::device` 微函数（**关闭 GAPS C6**）

| 函数 | 结论 |
|---|---|
| `device_uuid` | `[*(dev+0x658) + 0x690]` 的 **16 字节**（`pipelineCacheUUID`） |
| `physical_device` | `*(dev+0x658) + 0x10` |
| `debug_utils` | `Option` @ `dev+0x870` |
| `min_subgroup_size` | `(dev+0x40, dev+0x44)`（双 u32，**含义仍未知**） |
| `refresh_memory_budget` | 受 `dev+0x1a` 门控后转发 |
| `impl$5::drop` | 揭示 `dev+0x648` managed、`+0x718` Mutex、`+0x730` Queue、`+0x7e8` Queue Vec(stride 0xb0，len@`+0x7f0`) |

## 7. **关闭 GAPS C5：两个"不可取"函数体已解出**

| 函数 | 机器码 | 结论 |
|---|---|---|
| `pass_access_range_indices`(`0x140be9690`, **5 B**) | `mov eax,ecx; mov edx,edx; ret` | **恒等** |
| `BatchCompiler::new`(`0x140be95e0`, 30 B) | — | **构造空 vec** |

## 8. shader / swapchain / 其他

- **`get_vk_blend_mode`**：用三张表（@ `0x142b139f8`/`0x13a08`/`0x13a18`）按 `(byte[desc+0x1b] >> 3) & 3` 生成 `VkPipelineColorBlendAttachmentState`；mode 0 = 关闭混合，1 = `{ONE,ONE,ONE,ONE}`，2 = `ONE_MINUS_SRC_ALPHA`，3 = 双源(`0x10`/`0x12`)。
- `Swapchain::format` = `[+0x18]`；`any_present_successful` = `[+0x1c]`；`TrackedSwapchainTextureLease::image` 克隆内嵌 handle（同样 refcount++）。
- `command_buffer` 状态枚举 `{Initial, Recording, Recorded, Submitted, Universal}` + 字段 queue/seq。
- `DeviceDriverVersion::vendor_string` 表 = `{NVIDIA, Intel, Intel, AMD, Qualcomm, Apple, Unknown}`（`is_amd == 3`、`is_apple == 5`）。
- `ResourceHandle::raw` 恒等。

## 9. 仍未确定（已标注 medium/low，未臆断）

1. `ImageBarrier::new` 的 `+8`/`+0x10` 字段语义（偏移与 `-1` 哨兵已确证）。
2. `min_subgroup_size` 返回的双 u32 含义。
3. `TextureViewDesc.level_count`/`layer_count` 的双参对语义。
4. **`RawManagedHandle +0x00` 字段的身份**（既不参与 `eq` 也不打印）。
5. 4 类**非叶子**大函数（`create_texture`×4、`create_buffer`×4、`with_setup_cb`×3、`leekspin_print_dump` 11,396 B）**已完整反汇编但未做逐语句语义归纳**（超出"微叶子"范围）。
6. 4 个 low 仅因属诊断 fmt/日志。

**机器码 ≠ 原 Rust 源码；本轮结论均为"从字节直接解出"，字段名类有 `.rdata` 实证，其余语义标注为推断。**
