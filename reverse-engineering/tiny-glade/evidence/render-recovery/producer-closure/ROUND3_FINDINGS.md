# 第三波取证详录：country_core 构图机制 + rhapsody 收口 + C 类语义

- 日期：2026-10-12。
- 目标：`D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`（与 `D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne` 同一文件）。
- 本轮 8 个 agent：G1–G8，**全部只读取证，未改动任何游戏代码**（`reconstruction/rhapsody-rebuild/src/**`、`tests/**`、`Cargo.*`、`MODULE_STATUS.md`、`README.md`、`functional-coverage-audit.json`、`scripts/**` 均未触碰）。
- 配套文件：机制总账 [SYNTHESIS.md](SYNTHESIS.md)、缺口登记册 [GAPS.md](GAPS.md)；每个 agent 的原始证据（job JSON / MCP REPORT / 保留 `0x*.c` / `0x*.asm`）在各自子目录。

## 本轮总账

| 指标 | 第二波后 | **第三波后** |
|---|---:|---:|
| rhapsody 域有取证结论的函数 | 291 / 450 | **450 / 450** |
| rhapsody 域已定界字节 | 296,877（86.6%） | **342,968（100%）** |
| country_core 已有取证的不同地址 | 1（`gpu_generate_draw_lists`） | **140** |

---

# 一、G1：rhapsody 微叶子 PE 直读补齐（A 类闭合）

**方法**：PE 节表 RVA→文件偏移 + capstone 反汇编，绕过 Ghidra（对微小函数不建函数）。

**产物**：`g1-pe-leaves/pe-leaves.json`（**159 项，size 合计 46,091 字节**，与 GAPS 登记的缺口全量一致；置信度 high 92 / medium 63 / low 4）、分组 `.asm`、`_tools/pe_dump.py`。

**父级复核**：字段名串在 exe 中实测存在 —— `resource_checksum`(1)、`RuntimeDeviceId`(2)、`ManagedResourceChecksum`(2)、`TextureView`(2)、`PersistentDescriptorSet`(2)、`MarkedUnused`(1)、`memory_location`(3)、`CpuToGpuLowPrio`(2)、`GpuToCpu`(3)。⇒ **字段名类结论有 `.rdata` 实证。**

## 1.1 `RawManagedHandle` 真实字段名（更正前几轮命名）

| 偏移 | **原名** | 前几轮叫法 |
|---|---|---|
| `+0x00` | （**未命名；不参与 `eq`、不打印**） | "store" |
| `+0x08` | **`resource`** | "inner" |
| `+0x10` | **`resource_checksum`** | ~~"generation"~~ |
| `+0x14` | **`device_id`**（newtype `RuntimeDeviceId`） | "store_id" |
| `+0x16` | **`resource_type`** | "tag" |

另有 newtype `ManagedResourceChecksum`（`impl$44`）。**行为不变**（旧句柄靠 `handle+0x10 != inner+0xc` 被拒），**命名须改**。

## 1.2 `handle::clone` / `handle::eq`（此前缺的逐字节语义）

- **`clone`(`0x140bd6b00`, 48 B)**：`mov r8,[rdx+8]; lock inc dword [r8+8]` → **`inner+8` 原子引用计数 +1**，随后按字节整份拷贝 `{store@0, resource@8, checksum@0x10, device_id@0x14, resource_type@0x16}`。**无分配。**
- **`eq`(`0x140bd6b30`, 23 B)**：`mov rax,[rcx+8]; mov rcx,[rcx+0xf]; xor; or; sete` → 相等 ⟺ **`resource` 相等 且 `+0x0f..+0x16` 八字节相等**（覆盖 resource + checksum + device_id + resource_type）。**`+0x00` 不参与。**

## 1.3 资源类型 tag 枚举（更正 GAPS C2 的"跳号 3"）

| 名称表 | 内容 |
|---|---|
| `impl$57`（跟踪态） | `{Untracked=0, Tracked=1, MarkedUnused=2}` |
| `impl$51`（资源类型） | `{Texture=0, Buffer=1, Swapchain=2, **TextureView=3**, RenderPass=4, Sampler=5, PersistentDescriptorSet=6, Framebuffer=7}` |

⇒ **0..7 连续 8 个，无跳号**。A4 观察到的"7 个 store、tag 集合 `{0,1,2,4,5,6,7}`"应改为：**枚举有 8 个变体、store 只有 7 个 —— `TextureView`(tag 3) 无独立 store**（与 `Texture` 共用销毁路径）。

## 1.4 barrier 叶子

- `SynchronizationScope1`：三个 `Vec{cap@0, ptr@8, len@0x10}` 在 `+0x00`/`+0x18`/`+0x30`；`mask@+0x48 = 0x200000000001`；`is_empty` 查三处 len；`reset` 清 len + mask。
- **`image_aspect_mask_from_format`(`0x140bd6630`)** 查表 @ `0x142b0ffb8`：`(fmt - 0x7c) ≤ 6` 时 `0x7c/0x7d/0x7e` → **2(DEPTH)**、`0x7f` → **4(STENCIL)**、`0x80/0x81/0x82` → **6(DEPTH|STENCIL)**；否则 **1(COLOR)**。
- `BufferBarrier::new` = `{u64@0, u64 = -1@8, u8@0x10, u8@0x11}`。
- `ResourceSyncGroup::new` = 对全局 `AtomicU32@0x14362c230` 做 `lock xadd 1`（返回**旧值**）；`is_equivalent` = id 相等。

## 1.5 descriptor 构造器（字段名为 `.rdata` 实证）

- **`BufferDesc` = 16 B**：`{size@0, usage@8, alignment@0xc, memory_location@0xe}`；构造器断言 `size > 0`。
  **`memory_location` 枚举**：`1=GpuOnly, 2=CpuToGpu, 3=CpuToGpuLowPrio, 4=GpuToCpu`。
- **`TextureDesc` = 32 B**：`usage@0`(VK 位)、`format@8`、`width@0xc`、`height@0x10`、`depth@0x14`、`array_elements@0x18`、`mip_levels@0x1c`、`sample_count@0x1d`、`kind@0x1e`。
  **`kind` 枚举**：`{Tex1d=0, Tex1dArray=1, Tex2d=2, Tex2dArray=3, Tex3d=4, Cube=5, CubeArray=6}`（与 N2 的 `0x4a` 掩码"仅 kind 1/3/6 允许 layers>1"一致）。
- **`TextureViewDesc`**：6 个 8 字节 `Option` 字段 @ `0`/`8`/`0x10`/`0x18`/`0x20`/`0x28`（`view_type`/`usage`/`format`/`aspect_mask`/`level_count`/`layer_count`），`base_mip@0x30`、`base_layer@0x34`、`bind_individual_mips@0x38`。

## 1.6 `raw::device` 微函数（关闭 C6）

`device_uuid` = `[*(dev+0x658) + 0x690]` 的 16 字节（`pipelineCacheUUID`）；`physical_device` = `*(dev+0x658) + 0x10`；`debug_utils` = `Option @ dev+0x870`；`min_subgroup_size` = `(dev+0x40, dev+0x44)`；`refresh_memory_budget` 受 `dev+0x1a` 门控后转发。
`impl$5::drop` 揭示 `dev+0x648` managed、`+0x718` Mutex、`+0x730` Queue、`+0x7e8` Queue Vec（stride 0xb0，len `+0x7f0`）。

## 1.7 关闭 C5

| 函数 | 机器码 | 结论 |
|---|---|---|
| `pass_access_range_indices`(`0x140be9690`, 5 B) | `89 C8 89 D2 C3` = `mov eax,ecx; mov edx,edx; ret` | **恒等透传** |
| `BatchCompiler::new`(`0x140be95e0`, 30 B) | 见 G8 §C5 | **清零 Vec + cur_batch** |

## 1.8 其他

- **`get_vk_blend_mode`**：用三张表（@ `0x142b139f8`/`0x13a08`/`0x13a18`）按 `(byte[desc+0x1b] >> 3) & 3` 生成 `VkPipelineColorBlendAttachmentState`；mode 0 = 关闭混合，1 = `{ONE,ONE,ONE,ONE}`，2 = `ONE_MINUS_SRC_ALPHA`，3 = 双源(`0x10`/`0x12`)。
- `Swapchain::format` = `[+0x18]`；`any_present_successful` = `[+0x1c]`；`TrackedSwapchainTextureLease::image` 克隆内嵌 handle（同样 refcount++）。
- `command_buffer` 状态枚举 `{Initial, Recording, Recorded, Submitted, Universal}` + 字段 queue/seq。
- `DeviceDriverVersion::vendor_string` 表 = `{NVIDIA, Intel, Intel, AMD, Qualcomm, Apple, Unknown}`（`is_amd == 3`、`is_apple == 5`）。
- `ResourceHandle::raw` 恒等。

**未确定**：`ImageBarrier::new` 的 `+8`/`+0x10` 字段语义（偏移与 `-1` 哨兵已确证）；`min_subgroup_size` 双 u32 含义；`TextureViewDesc.level_count`/`layer_count` 双参对；`RawManagedHandle +0x00` 身份。

---

# 二、G2：rhapsody 诊断类判定

**结论：必须纳入渲染语义恢复的 —— 无。** 这批全部是 tracing / 日志 / 错误格式化 / 统计 / 配置。

| 项 | 判定 |
|---|---|
| `leek::leekspin_print_dump`(`0x140be2b40`, 11,396 B) | **纯诊断** —— 反编译引用约 172 个事件标签串，即把 leek 追踪缓冲 dump 成大段格式化文本；唯一调用者 `print_log_dump`(`0x140be6b40`, 5 B `jmp`) 在 450 函数图内**无调用者** |
| `raw::error`（21 函数） | **纯格式化**（`BackendError`：`impl$1::fmt`×20 Debug + `impl$3::fmt` Display）；**不存在 VkResult → 行为的映射表** |
| `raw::debug::begin_label`/`end_label` | **确在 record 路径**（`record_batch` 调二者），但被 gate 且**纯发标签**，不改 GPU 工作 |
| `raw::stats::snapshot_and_reset`(34 B) | 原子地把 3×u64 计数器清零并返回旧值 |
| `raw::temp_list::add`(401 B) | 内联 8 槽 + 堆溢出容器 append |
| `leek::impl$0/1/2` | 16 字节定长名字的 Display/From（`impl$0` 非法字节→U+FFFD；`impl$1` UTF-8 解码、>16 panic） |
| `init_logging` / `dump_log` / `print_log_dump` | `RingBuffer::new(0x10000)` 入全局 `0x143634e40` / `to_inner` / `jmp` |
| leek 操作包装（tag `0x12`/`0x13`/`0x14`/`0x2a` 等） | 只把 1 字节 tag（+小 payload）append 进环形缓冲，**不执行真正操作** |

## 2.1 单列开放项：`raw::workarounds`(`0x140082a00`, 477 B)

模块**唯一**函数 = `serde::Serialize for GraphicsWorkarounds`，序列化 **4 个渲染行为开关**：

```
sync_before_rg_pass (19)   sync_before_draw (16)
sync_before_dispatch (20)  serial_cb_recording (19)
```

函数体**无任何 `vkCreate*` / 扩展 / feature 调用** ⇒ 就"是否改设备创建参数"而言：**否**。
但 **rhapsody 域内找不到消费点**（符号表只有 `Serialize`）。App 侧是独立 iced 配置 GUI（`tg_gpu_selector::vulkan_config::Controls`、`tiny_glade::vulkan_config_main::run_vulkan_configuration_app 0x1403e74b0`）。

⇒ **列为开放项**（4 个 flag 的名称预示渲染行为，但消费点未定位）。**不要简单标为"诊断无影响"。**

## 2.2 方法学收获：`.pdata` 陷阱

`get_xrefs_to` 对 `0x140be2b40` 返回的那条"D**DATA 引用**"经查是 **`.pdata` 异常目录条目**（存 RVA `0x00be2b40`），**不是调用边**。⇒ **xref 输出必须用 `calls.json` 交叉验证。**

## 2.3 父级更正

G2 把 `present` 的非致命码写成 `OUT_OF_DATE(-1000001001)`。父级直接读 `a6-allocator-swapchain/0x1418cc1b0.c` 核对真实常量：`0x3b9acdeb`(`SUBOPTIMAL`)、**`-0x3b9acdec`(`OUT_OF_DATE` = `-1000001004`)**、`-0x3b9eae18`(`-1000255000`)。`-1000001001` 是 `NATIVE_WINDOW_IN_USE`，**G2 标错**。

---

# 三、G3：`draw_passes` 族（混合管线）

**统一惯用法**：全部 draw pass 走「录制 → render」：
`FUN_140867970`（登记 RasterPass，传 `&name, len`）→ 逐条 **0x80 字节**绘制记录：`FUN_140905be0`（建 `DrawBuilder`）+ `FUN_14090b460`（`bind_common`）+ 回调 `*(rec+0x70)` → `FUN_140868370`（`RasterPass::render`）。

| pass | 地址 | 登记的 RasterPass |
|---|---|---|
| `main_pass` | `0x140833bc0` | **6 个**：`G-buffer`(+0x20)、`G-buffer emissive`(+0x38)、`G-buffer velocity`(+0x50)、`Forward`(+0x68)、`ForwardVelocity`(+0x80)、`Velocity`(+0x98) |
| `main_pass_oc2` | `0x140835a40` | **5 个**：同名 + `OC/2`（+0xb0/+0xc8/+0xe0/+0xf8/+0x110） |
| `late_forward_and_deferred_pass` | `0x1408373b0` | `Late Deferred`(+0x128/+0x130，非零才渲染) → `Late Forward`(+0x140/+0x148) |
| `deferred_raster` | `0x1409d20b0` | **单个** `Apply deferred lighting`；循环 **5 个 G-buffer 附件**（`lStack_98 + i*0x10` 存在位），串联约 11 次资源绑定后 `FUN_140906a90(..., 3)` 收尾 |

**分工（推断）**：`main_pass` 首批——不透明体按通道分别写入 G-buffer（color/emissive/velocity）或直接 forward；`oc2` = **HZB 遮挡剔除后的第二批**；`deferred_raster` 消费 G-buffer 计算光照；`late_forward` 处理需 GI/后置光照的迟渲染。四者**同时存在**于 `render_world` 顺序中。

**`shadow_pass`(`0x1409cc270`)**：名 `Shadows`。按 TypeId `0xa68561b396784eaa`（= `PassResources` 槽 `[1]`）取 `Res`；`do{…}while(count<3)` → **3 级联**，级联槽位写 `uVar16 + 2`（槽 2/3/4，父级已核对）；每级联 `bind_common` + 回调 + render。**投影矩阵未读。**

**transparency 家族**：

| PASS 名 | 地址 |
|---|---|
| `Copy refraction target` | `0x1409bf320` |
| `Water plane` | `0x1409bf800` |
| `Ice plane` | `0x1409c08d0` |
| `Pre-TAA transparency` | `0x1409c3a80` |
| `Post-TAA transparency` | `0x1409c40b0` |

- `water_surface` = RasterPass `Water plane` + **2 个 ComputePass**：`Water deferred rt args`(`/water/calc_deferred_rt_args.cs.hlsl`)、`Water deferred rt`(`/water/deferred_rt.cs.hlsl`)。
- `ice_surface` = RasterPass `Ice plane` + **4 个 ComputePass**：`Ice refl march`(`/water/ice_refl_march.cs.hlsl`)、`Ice refl trace`(`/water/ice_deferred_rt.cs.hlsl`)、`Ice refr resolve`(`/water/ice_refr_resolve.cs.hlsl`)、`Ice refl resolve`(`/water/ice_refl_resolve.cs.hlsl`)。
- **TAA 顺序**：`refraction_copy` → `water`|`ice` → **`pre_taa_transparency`** → … → `calculate_taa` → **`post_taa_transparency`**。

**UI**：`ui_pass`(`0x140837c40`) 与 `loading_screen`(`0x140837ff0`) **几乎逐字相同、同一 RasterPass 名 `UI`**（仅组偏移不同：`+0x1d0/+0x1d8` vs `+0x1b8/+0x1c0`）⇒ **同一渲染图**。`ui_2d_pass` = `UI 2D`（形/字 SSBO 流，0x30 步长）。`mei_pass`(`0x1409c6700`) = `Mei 2D`，用**另一组 TypeId**（`0x8597ee0727b461b3` 槽 `[0]`、`0x864ddca98c2312a6`、`0x958c258b851a69dc`），**不同资源路径但仍在同一 RG**。

**`RasterPass::render`(`0x140868370`) 不是简单提交**：入口 `param_1[0x3e][4] = 1`；遍历录制命令（`+0x20` 起、数量 `[0x3e]`、**步长 0x60**），打包 **0x10 字节**记录（上限 **0x80 条**），调 `FUN_141524830` 冲刷，并用**开放寻址哈希表**（`& uVar7` 掩码、写 `[uVar12]`、计数 `puVar13[2]/[3]`）**去重/合并**，再走 `FUN_1409dcd10` / `FUN_140bfc0d0` / `FUN_140bd6b00`。⇒ **对录制流做批次归并后转为 GPU 绘制。**

**其他**：`final_post`(`0x14090d000`) 用 shader `raster_resolve_msaa_to_swapchain`（**MSAA 解析到 swapchain**，**非** RasterPass 构造器路径）；`draw_lists::build_draw_lists`(`0x1409ba540`, 181 KB) **纯 CPU**（零 pass 调用）—— 是 GPU `gpu_generate_draw_lists` 的 **CPU 对应物**；`render_shader_refactoring`(`0x14098f9f0`) 本体几乎仅 unwrap/自调用（**调试工具，非逐帧 pass**）。

**访问码**：draw pass 体内**不写死**，一律经 `bind_common` 隐式声明。显式码在绑定器第 4 参：`0x1d`（`FUN_1408ee800`/`FUN_140908c50`/`FUN_140908610`）、`0x19`（`FUN_1408ee240`/`FUN_1409056d0`/`FUN_140905260`）、`0x32`（`FUN_1408edd00`）、`0x1e`（ice 内 1 次）。

---

# 四、G4：GPU-driven 可见性与绘制列表

**cull/bucket 算法在 GPU shader**：CPU 侧 `create_cull_and_bucket_pass` 只编排 dispatch。`_nani_cull_and_bucket.cs.hlsl`（workgroup **64×1×1**）绑定表：

| binding | 资源 |
|---|---|
| 13 | `atlas_part_info` |
| 14 | `filtered_instance_buffer` |
| 15 | `allocator` |
| 16 | `meta` |
| 17 | `output_histogram` |
| 18 | `prev_visibility_bitmask` |
| 19 | `next_visibility_bitmask` |
| 20 | `draw_list_info` |
| 21 | `hzb_tex`（采样图像） |

⇒ 逐实例读实例 + atlas 信息，用 **HZB 做 HZ 遮挡测试**、结合 prev 位掩码，存活者写 `output_histogram`（每 atlas entry 计数）+ next 位掩码。

**直方图 / 前缀和链**：`prefix_sum_unsorted_instances_per_atlas_entry`（`"Nani unsorted atlas idx prefix sum"`）→ `group_draw_cmd_prefix_sum`（**两级**：`"Nani draw cmd histogram prefix"` + `"subtotal prefix"`）→ `fill_sorted_instance_list`（`"Nani fill unsorted/sorted instance list"`，据前缀和 scatter）。**桶 = atlas entry**（由 shader 名与 binding 名证明）。

**`DrawIndexedIndirectCommand` = 20 字节 = 5×u32**（**父级核对**：`0x140907990.c:31` 用 `param_2 * 0x14`；静态默认表 @ `0x142aaaa78` 共 24 项 × 20 B；`reset_indirect_args` 拷 `0x1e0 = 480 = 24×20`）。字段 = `(indexCount, instanceCount=0, firstIndex=0, vertexOffset=0, firstInstance=0)`（**名字属 Vk 常见约定，未证**）。

**两轮可见性**（`render_world`）：
- 行 238 `gpu_generate_draw_lists(graph, world, 0)` → `shadow_pass`/`main_pass`
- 行 292 `generate_hzb` → `auStack_458`
- 行 **290** **`if (*pcRam000000014362c0d8 == '\0')`**（**全局字节，为 0 才开**；**父级核对**）
- 行 294/295 `generate_wall_draw_lists_pass2(hzb)` + `gpu_generate_draw_lists(graph, world, hzb)` → 随后 `main_pass_oc2`
- **第三参 = HZB（0 = 无）**；第二轮内部换缓冲（`FUN_1412d1180` vs `FUN_1412ce810`）、换 flag（**4 vs 1**）、切 cull pipeline（`param_9 = 1`）

**`generate_hzb`(`0x1409286f0`)**：从深度建 **mip 金字塔**，写 **2 张 storage image**（access `0x37`），push args 24 B；输出即 cull shader 的 `hzb_tex`。

**`visibility_queries`(`0x140a2f2e0`) 实名 `"Firefly vis query"`（`/critters/firefly_vis_query.cs.hlsl`）** —— **萤火虫**可见性查询，**不是网格遮挡**（access `0x19`×3）。**父级已核对**（exe 中 `firefly_vis_query` 存在，周围串为 `critters/firefly.raster.hlsl`、`firefly_glow_shader`）。⇒ **重新分类**。

**实例布局（类型来自符号面）**：`render::ssbo::InstanceBuffer<T>`、`sparse_instance_buffer::SparseInstanceBuffer<T>`；atlas = `nani::mesh_atlas::{MeshAtlas, MeshAtlasPart, InstancedMeshAtlas, NaniMeshAtlasPartInfo}`；`nani::buffers::{NaniDrawListSsbo, Params, LengthSsbo, ListDrawCmdOffsets, OffsetDefaults, NaniInstanceIndirection{,Allocator}Ssbo, NaniFilteredInstanceAllocatorSsbo, GpuInstance, NaniNext/PrevVisibilityBitmaskSsbo}`。`GpuWorld` 在 `World+0x1450`；`*(u32*)(GpuWorld+0x60)` = 实例/槽计数；allocator 槽 **stride 0x20**。**字段级布局未逐字段证实。**

**`create_cull_and_bucket_pass`(`0x140a28950`)** 是 **4 个单态化之一**，差异**仅** shader ctor（`FUN_1415ff350`/`141600570`/`141601770`/`1415fcf10`）。建 `"Nani cull & bucket"` ComputePass，绑 6 组 gpu buffer/uniform，`param_9` 选两套 dispatch。前置 **3 个 pass**：`"Nani clear draw lists"` / `"Nani clear histograms"` / `"Nani prepare unsorted draw lists"`。`cull_and_bucket_external` 泛型 `T = {WoodenPlankSsbo, RoofTileSsbo}`。

**访问码**：`0x19`（cull 全 6 变体、pass2、`generate_hzb`、`run_visibility_queries`×3）、`0x37`（HZB 写）、`2`（draw builder 间接）、`0x34`（`reset_indirect_args`，**复证 C1**）。

---

# 五、G5：GI 与光照

**关键地址**：`build_cwbvh_from_tris` = `0x1417c93d0`；`build_gpu_bvh` = `0x140b0da50`；`calc_terrain_lighting_texture` = `0x140a51440`；`calculate_ssgi` = `0x1408743b0`；`ray_trace_gi` = `0x140a01cf0`；`eval_gi_screen_radiance` = `0x140a44cd0`；`bin_lights` = `0x1409e7770`；`restir_sample_direct_lights` = `0x140a6fec0`；`apply_screen_shadows` = `0x140a39e70`；`compute_shadow_mask_mean` = `0x140a3a6f0`。

## 5.1 BVH —— CPU 建、软件遍历（**第四条独立证据**）

- `build_cwbvh_from_tris`：用 **SIMD**(`minps`/`maxps`) 算 AABB + 面积，**SAH 常数 `0x3fcccccd` / `0x3fe66666`**（≈1.6 / 1.8），调 obvhs 内部 `FUN_1420e9710`/`FUN_1420e5450`/`FUN_1420f2480`，读 config `+0x19`(bool)/`+0x1a`/`*config`(leaf)，返回**节点 + primitive-index 数组**。
- `build_gpu_bvh`：把三角形 **stride 0x28 → 0x30 重排**后调用之，输出节点数组 + 重排索引。
- 上传属 `resources::rt`（`RtWorld`；系统 `upload_rt_world` 取 `RtBvhBuilderGlueResource` + `AssetSsboLibrary` → `RtBvhSsbo`/`RtTrianglesSsbo`/`RtTriangleMetaSsbo`）。
- 着色器 `_gi_trace_gi_candidates` 声明 `rt_bl_bvh`/`rt_triangles`/`rt_triangle_meta`，**无 RayTracing cap**。

## 5.2 candidate 链

- **`ray_trace_gi`**（建 2 个 ComputePass）：`_gi_trace_gi_candidates` 输入 `terrain_heightmap`、`visual_terrain_heightmap`、`blue_noise`、`terrain_lighting_texture`、`color_library`、`shadow_map`、`wall_color_ids`、`rt_*`、**`taa_history_tex`**、**`reprojection_tex`**、**`half_depth_tex`**、**`half_view_normal_tex`** → 输出 `candidate_radiance_tex`、`candidate_hit_tex`。
- **radiance 更新**：`_gi_update_gi_radiance` 读 `depth_tex` + candidates → 写 `restir_radiance_out_tex` / `restir_ray_dir_out_tex` / `restir_reservoir_out_tex`。
- **`eval_gi_screen_radiance`**（建 3 个子 dispatch）：`_gi_eval`（读 `blue_noise`+`half_depth`+`half_normal`+**`ssgi_tex`**+`restir_*`）、`_gi_blur`（另写 `temporal_stats_out`）、`_gi_reproject`（读 `history`+`reprojection`+`temporal_stats`）。

## 5.3 `restir_di`

单 compute pass（`0x140a6fec0`）。读 TypeId `0x8226cc786e679355`（与 `bin_lights`、`deferred_raster` **共用**），bind 均 `0x19`：`0x815bf5036966978c`、`0xb8a74ccd835a6be3`、`0x2a333c3a28028230`、`0x350f8dc714051d10`、`0x4da9887cf3dc26f4`；另 `0x8daeeefe7463dee0`(`0x32`)。
> **reservoir 字段 / 变体 / 权重 / 偏差修正 = unknown**（蓄水池是 GPU 纹理，CPU 侧无结构体）。**不套论文。**

## 5.4 `ssgi` 与 `ray_trace_gi`：**互补**

- `ssgi`（`_ssgi_ssgi`：depth+half_normal→output；`_ssgi_temporal_filter`：input+history+reprojection→final/history；`_ssgi_upsample`）负责**屏幕空间项**；
- `ray_trace_gi` 负责**世界空间 BVH 追踪 + ReSTIR**；
- `eval_gi` **同时读 `ssgi_tex` 与 `restir_*`** 合成。**非二选一。**

## 5.5 `bin_lights` / terrain lighting / screen shadows

- `bin_lights`：**光数为 0 → 仅建小 raw resource 返回 3 句柄**；否则建多个 compute 子 pass，bind SSBO key `0x63e587f19a869d94`/`0xaab861d318ad7248`/`0xc3674440971f6990`/`0xe3b8c78bd96aa31d`，raw resource 尺寸 `0x44`、`0x100000`，uniform `0x9b88329e408ff69f` = 光数。**分桶字段布局 unknown。**
- `calc_terrain_lighting_texture`：1 个 compute pass，bind `0xb981790202418bc8`(`0x32`，输出)、`0x61b08b5e0470c5f2`(`0x19`，= `terrain_lighting_texture`，`ray_trace_gi` 也读)、`0x297d1e4cdae8f657`(`0x19`)。**在光栅 GI 之前算出。**
- `apply_screen_shadows`：1 个 compute pass。`compute_shadow_mask_mean` 把阴影掩码直方图归约为均值 SSBO（`ShadowMaskMeanSsbo`/`ShadowMaskHistogramSsbo`）。

## 5.6 GI 的半分辨率与跨帧

两轮光栅可见性及 `late_forward_and_deferred` **之后**才 `half_res`；GI trace 着色器**显式声明 `half_depth_tex`/`half_view_normal_tex`** ⇒ **GI 用半分辨率 depth/normal**。
**跨帧**：trace 读 `taa_history_tex`+`reprojection_tex`；blur/reproject 用 history+reprojection+temporal_stats ⇒ **GI 做时域重投影**。

---

# 六、G6：场景数据 / 程序化生成

## 6.1 ⚠️ 关于"write = 0x32"的冲突与最终裁定

G6 称"实测普通 write = `0x32`、并发 write = `0x37`"，并当作对 C1 的**更正**。

- **第一步核查（属实）**：该说法在 G6 自己的保留证据中**找不到支撑** —— `decomp-lib/` 中 `add_resource_access` 的全部出现只有 `0x140905260.c:132`/`:152` 两处**透传**（码来自 `param_4`）；`REPORT-g6*.json` 同样无 `0x32` 立即数；它声称的两个函数 `0x140975320`/`0x140975e40` **根本没被反编译**。
- **第二步核查（父级更正自己）**：**G8 用 PE 扫描 167 个调用点独立得出"访问器：写 = `0x32`、并发写 = `0x37`"** ⇒ **G6 的实质结论成立**；错的是**框架**。

**裁定 —— 两套不同 API，不构成矛盾**：

| API | 码 |
|---|---|
| builder `write_rg_storage_buffer`(`0x14096dd60`) | **`0x37`** |
| accessor `write` | **`0x32`** |
| accessor `write_concurrent` | **`0x37`** |

## 6.2 pass 登记机制

`ComputePass::new`（7 参 `0x1416032a0` / 6 参 `0x1405876a0`）→ `read_storage_buffer` / `write_storage_buffer(_concurrent)` / `bind_texture_to_compute_pass` / `UniformBinder::write_uniform` → `dispatch_2d` / `dispatch_with` → `PassBuilder::render`(`0x1409dd4d0`)。资源用 **TypeId** 经 `Components::get_resource_id`(`0x1400dd6f0`) 解析。（`dispatch_*` 只有分派、**无访问码**。）

## 6.3 `upload_raster` × 6

`upload_world_raster::upload_raster_staging_to_texture` 的 **6 个单态**：`0x85c987d23d58d2de`(`0x1403D8BD0`)、`0x6dc90dcfd23786fa`(`0x1403D90F0`)、`0x1ad182936286a0ee`(`0x1403D9610`)、`0x979ffa6288aa259a`(`0x1403D9B30`)、`0xbb8761bcc7907109`(`0x1403DA050`)、`0x86486f3e82eb6bbe`(`0x1403DA570`)。
后两组 TypeId 与 `water_rasterize_from_cpu`/`garden_rasterize_from_cpu` **相同** ⇒ water/garden 栅格（观察）。对照源码 6 个 `MappedStorage *RasterStagingSsbo`（Path / Cobblestone1/2/3 / Water / Garden）⇒ **推断**六次上传即这 6 张 CPU 栅格。

## 6.4 地形 / 建筑 / 植被 / 屋瓦 / 楼梯

- `compute_path_masks`(`0x1409014E0`)：读 **5 张**栅格，注册 **5 个** pass 产路网掩码。
- `rasterize_terrain_stroke`(`0x1408C7120`)：查 `PassResources(0x47c94cb78283a72e)` + `0x1e38c16726d0fb8d` + `0xcda6af2f6afb130e`，注册 **3 个** pass（含 `GaussBlur`；另有大量 **CPU 端**计算）。
- `displace_rocky_terrain`(`0x140A3EE80`)：读 `0xfa44538b95ae76c6`、写 2 buffer、绑 5 张地形纹理、uniform = `count/3`、单 pass。
- **帧序**：`rasterize_terrain_stroke` → `displace_rocky_terrain` → … → `compute_path_masks` → … → `path_pebbles`。
- `rasterize_buildings`(`0x140A32770`)：4 pass，产 `GroundedBuildingsSsbo`(`InstanceBuffer`) —— **产实例数据**（vs `gpu_generate_draw_lists` 消费之组织提交）。
- 植被：`water_rasterize_from_cpu`(4 pass)、`garden_rasterize_from_cpu`(2)、`water_spawn_vegetation`(2，11 纹理)、`spawn_vegetation_around_buildings`(2，12 纹理)、`garden_spawn_vegetation`(4，15 纹理)、`read_bush_positions_for_cpu`(`0x140866540`，**回读**到 CPU)、`leaf_particles`(`0x1408F4D00`)。
- **`roof_tiles_gravity`(`0x140A29B10`)**：遍历 ECS 屋瓦表（魔数 `0x65cfda6a55a841ac`），单 compute pass。shader `_roof_tiles_gravity.cs` 消费 `RWStructuredBuffer<RoofShingle>`（**stride 64 B**），`PushConstants{first_instance, instance_count, wall_height_delta, roof_pivot_y, animation_t}` ⇒ 更新 `current_height`/`velocity__rotation`/`animation_t`。**与 `evidence/roof-tiles/` 的 64 字节记录互证。**
- **`stairs`(`0x140A68A50`)**：2 个 compute pass。与 **`construct_door_stairs`(`0x14200C7F0`) 无关** —— 后者纯 CPU（只调 `random_splits_into`/碰撞，**零 pass-builder 调用**）。⇒ **「CPU 装配 → GPU stairs pass 逐帧处理」两件事。**

## 6.5 `GpuWorld` 数据模型

- `GpuWorld::import_to_rg`(`0x1409921C0`, 100 B)：遍历固定 **21 项**子对象数组（**stride 0xc0 = 192 B**，偏移 `0xa0..0x14e0`），对每项调虚表 `+0x40` 的 import（传 render graph）。
- **`TypedStorageBuffer<T>` 共 87 个单态**（raster staging 6、`RoofTileSsbo`、地形/水/园/路/建筑/粒子/卵石/鼠标射线、`Nani*` 族、`RtBvh`/`RtTriangles` 等）。

---

# 七、G7：后处理 / TAA-TGSR / DoF / UI

## 7.1 TAA vs TGSR（互斥）

`render_world` 读配置 `param_6`：`cStack_4a = param_6[0xab]`（L318；**父级核对** `0x1404EC110.c:318`）。

| 条件 | 走 |
|---|---|
| `!= 0`（L464） | `calculate_tgsr`(`0x140A3BA60`) |
| `== 0`（L509） | `calculate_taa`(`0x140A3B2F0`) |

`param_6[0x9c]`（float）**同时**传给两者。符号含 `UpscalingPreset::tgsr_resolution_scale`、`TgsrMain` ⇒ **推断**为时域超分/上采样；**缩写确切义 unknown**。

**跨帧**：`calculate_tgsr` 调 `get_or_create_temporal_pair`(`0x14096D490`，parity 双缓冲)；**`calculate_taa` 内未见 `get_or_create_temporal*`**，其历史由 `render_world` 按 key 传纹理（读 `0xb8a74ccd835a6be3` 码 `0x19`，写 `0xaa75015a6ad61913`/`0x88219a3ae69892b2` 码 `0x32`）。**TAA 的 parity 管理位置 unknown。**

## 7.2 pre/post transparency 与 DoF

- 顺序：`refraction_copy` → `water`|`ice`（由 `cStack_49 = func_140ae8320(res+0x58)` 选，非 0 = ice；**整体由全局字节 `*14362C098 == 0` 门控**）→ `pre_taa` → … → taa/tgsr → `post_taa` → `refactoring`。pre/post 均为 DrawPass，遍历 draw-item（pre：`param_8+0x188`，计数 `@+400`；post：`param_4+0x1a0`，计数 `@+0x1a8`，**计数 0 则 return**）。
- **DoF**：`dof_autofocus`(`0x140A17880`，无条件) 读模式资源 `0xc2b8e941c91a6428` 首 int `iVar3`，`bVar14 = (iVar3 == 0x13)`；`dof_prepare`(`0x140A5A020`) 出 **2 张 COC 纹理**；`calc_dof`(`0x140A5A570`) 由全局 `*14362C080` 分两路，写 `0xb981790202418bc8`(`0x32`)。`iVar3 == 0x13` 是**资源内模式枚举**（控制 DoF 变体与 `param_6[0xa9]`/`[0xaa]`），**语义 unknown**。

## 7.3 其余 pass

| pass | 结论 |
|---|---|
| `apply_volumetrics`(`0x1408A44C0`) | 读大气资源 `0xbba451ee6a08a3ff`，按 `+0x4d0` 字节(∈{4,1})选常量，把体积光并入场景色 |
| `compute_shadow_mask_mean`(`0x140A3A6F0`) | 单 pass 求阴影遮罩均值 |
| `bidirectional_blur_pyramid`(`0x1409ED860`) | 按 mip 级循环（down+up），读 `0x11ba5697cc40b3dd`，写 `0xdcdfcf289eb02e68`/`0x88219a3ae69892b2`(`0x37`) |
| `half_res`(`0x1408F6180`) | = `extract_half_res_depth_normals`（半分辨率深度+法线，2 输出） |
| `generate_mips`(`0x1400FB890`) | 通用 mip 链（限 `mips < 14`、格式 `0x7a`），写 `0xb466b30ac6fc0292`/`0xed9d1f0a5683f687`/`0x45f26f91023c2ec3`(`0x37`)+缓冲 `0x68d4413b4de4f24` |
| `reprojection`(`0x140A0D190`) | 两 pass，由深度+相机算重投影/motion 图，写 `0x88219a3ae69892b2`(`0x32`) |

## 7.4 UI 与 painterly / vermeer

- UI 节点（`ui_pass`/`loading_screen`/`wireframe`/`post_taa`）**共用同一 DrawPass 模板**（同读世界资源 `0x4bc362195b80f729`；**`ui_pass` 与 `loading_screen` 连 pass 名字面量 `&UNK_142A92BE0`（长 2）都相同**）⇒ **同一 RenderGraph 的节点**，非独立路径。
- `mei_pass`(`0x1409C6700`) 属 `ui_2d_pass` 模块，读槽 `[0]` = `0x8597ee0727b461b3`，绑图集 `0x815bf5036966978c`(`0x1d`)。`custom_cursor`(`0x1409D1710`) 读槽 `[0]`、校 13 字节标记（`0x9f0d228ee3dba429`，`0x1d`）。
- **`painterly_pass`(`0x1408A6410`)**：多 compute + 1 draw，用 `get_or_create_temporal`(`0x14096C890`/`0x14096C310`) ⇒ **带时域状态的风格化滤镜，算法 unknown**。
- **`vermeer_pass`(`0x1408A82F0`，返 bool)**：用 `get_or_create_temporal`（名长 `0xe`），**维护帧计数并累加 float 至 ≥ 阈值(`_UNK_142925904`)**，置 `state = 2` 并推进环形缓冲，**返回 `state != 2`**。
- **关键门控（父级核对 `0x140161CD0.c:1279`）**：**`if ((bVar27 & iVar3 != 0x13) == 0)`** —— **vermeer 仍在收敛时整段 UI 注册被跳过**。

---

# 八、G8：C 类语义攻关

## 8.1 C1 —— **攻克**：`queue+0x80` 写者

**写者 = `Queue::update_pending_submissions`(`0x140bf0b90`)**：第 19 行调设备 ash 槽 `+0x478`（`vkGetSemaphoreCounterValue`），参数 `(device+0x5c8, queue+0x88, &out)`；**第 42 行把返回的 u64 写入 `param_1[0x10]`（= `queue+0x80`）**。
（**父级核对** `n1-device/0x140bf0b90.c:19` 结构成立。）`reclaim`/`queue_submit` 只读它。

## 8.2 C5 / C6 —— **攻克**（字节级；与 G1 一致）

- `BatchCompiler::new`：`48 89 C8` / `48 C7 01 00000000` / `48 C7 41 08 08000000` / `0F 57 C0` / `0F 11 41 10` / `0F 11 41 19` / `C3` ⇒ 清零 `{Vec cap@+0x00, ptr@+0x08(=8，悬垂), len@+0x10}` 与 `cur_batch{start@+0x18, end@+0x20, flag@+0x28}`。`flush_batch` 独立印证批次 = **24 B `{start, end, payload}`**。
- `pass_access_range_indices` = `89 C8 89 D2 C3` ⇒ **(start, end) 恒等**；调用者读 **stride-8** 表 `{start@+0, end@+4}` 并做 `end < start` 越界 panic。
- C6：`device_uuid` = 拷 `P+0x690` 16 B；`physical_device` = `P+0x10`；`min_subgroup_size` = `8B 41 40 / 8B 51 44 / C3`；`refresh_memory_budget` = `80 79 1A 00 / 74 10` 门控后尾跳 `0x140c01620`。

## 8.3 C3 —— 部分攻克

**合并格完整恢复**（`0x1409dca90` 与 `0x140533b10` **逻辑相同**）：

| 组合 | 结果 |
|---|---|
| `0x17` + `0x1d`（任序） | `0x1e` |
| `0x1a` + `0x02` | `0x02` |
| `{0x19,0x1a}` + `0x37` | `0x37` |
| 其它非同组合 | **panic** |

⇒ **`0x1e` = 读 \| 写 合并态。**

**生产者归属**（PE 扫 **167 个**调用点的 `mov r8b, imm`）：见第 9 节总表。

**账本布局**经比较指令确认 = `{index@+0, sync_group@+4, access_code@+8}` ⇒ **印证 SYNTHESIS 11.2 的更正**。

**未攻克**：`0x19`/`0x1d` **扫不到任何字面生产者**（应为**运行期计算值**）。

## 8.4 C2 —— 部分攻克

- 桶 = **9 个句柄 `Vec` `{ptr, cap, len}`（步长 0x18）+ 第 10 个 0x48 字节记录 `Vec` `{ptr@[0x1c], len@[0x1d]}`**，元素 tag `@+0x44`（`== 2` 即停）。
- 9 个 `Vec` 各由独立 device vtable 槽排出：`+0x1a8`、`+0x1c0`、`+0x188`、`+0x138`、`+0x228`、`+0x248`、`+0x278`、`+0x288`，另一路走第二张表（`+0x6b8`，器件句柄 `+0x6f0`）。
- **已定名**：**`+0x278` = `vkDestroyFramebuffer`、`+0x288` = `vkDestroyRenderPass`**（用 create 槽 `+0x270`/`+0x280` 标定）。
- **9 ≠ 7**：多出的 2 个排**非 store 资源**（`+0x6b8`/`+0x6f0` 疑为命令缓冲/池）。
- **`case 9`**：0x48 记录路径断言派发判别值 `== 9`；`FUN_140c0e150 → FUN_140c14680`，后者写 `*out = 9`。⇒ **是 trait/vtable 派发判别值，不是 int32 跳表字面 case。**

## 8.5 C4 / C10

- **C4（登记边界）**：`+0x48` 的 u64 = **`{低 32 = access 掩码, 高 32 = stage 掩码}`**，初值 `{1, 0x2000}`。其余位由**外部 `vk-sync-fork` 的 `FUN_140c10dd0(code) -> (access, stage)`** OR 进来 —— **按约束不逆向该库**。
- **C10**：`device+0x658` = 指针 `P`；`P+0x690` = 16 B `deviceUUID`；`physical_device()` = `P+0x10`。`GraphicsDevice::create` **直接读** `*(u32)(PhysDev+0x680)` 并有 **`< 0x2000 → 报错`** 保护（与 `maxDescriptorSetUpdateAfterBindSampledImages` 吻合），并把 `PhysicalDevice` 存入 `device+0x658`。⇒ 该 limit 是**包装体的缓存字段**（create 时直读，站点不再调 `vkGetPhysicalDeviceProperties`），与 `+0x690` 同级，**不经 `+0xd0` 的 VkDevice 句柄推导**。**遗留**：`+0x680` 的绝对基址仍有 `P` vs `P+0x10` 歧义。

---

# 九、访问码总表（三轮累计，本轮补齐）

## 9.1 生产者归属（G8 PE 扫 167 个调用点 + G3/G4/G5/G6/G7 实测）

| 码 | 生产者 / 含义 |
|---|---|
| `0x02` | 间接：`multi_draw_indirect` / `draw_indexed_indirect` / `dispatch_indirect` |
| `0x17` | `RasterPass::new_impl` |
| `0x19` | 资源绑定 / 读（`FUN_1408EE240`/`FUN_14096F090`/`FUN_140905260`/`FUN_1409056D0`）；**生产者扫描不到** |
| `0x1a` | `ComputePass::read_rg_storage_buffer` |
| `0x1d` | DrawPass 纹理绑定（`FUN_1408EE800`/`FUN_140908610`）；**生产者扫描不到** |
| `0x1e` | **合并态 = 读 \| 写**（`0x17`+`0x1d`） |
| `0x24` | `DrawBuilder::read_rg_storage_buffer` |
| `0x32` | **accessor `write`** |
| `0x34` | 写 / 清：`reset_indirect_args`、`clear_rg_buffer`、`rg_buffer_copy_from_slice` |
| `0x37` | `write_rg_storage_buffer` / accessor `write_concurrent` / HZB 写 |

**合并规则**：`0x17`+`0x1d`→`0x1e`；`0x1a`+`0x02`→`0x02`；`{0x19,0x1a}`+`0x37`→`0x37`；**其它非同组合 panic**。

## 9.2 关键结构发现

> **访问码是「逐调用点传入」的参数，不存在「某函数 = 某码」的映射。**
> 证据：`FUN_1408edd00` 在多数调用点传 `0x32`，但 `decomp:281` 传 `0x37`（**父级核对**）。`0x140905260.c` 的两处调用也对同一函数传 `param_4` 与不同的 sync 标志（`1`/`0`）。

**并发分类**（A3）：`classify(v) = (v - 0x28) < 0x10` ⇒ 只有 `[0x28,0x37]` 内的码需要"非零且相等 group"。目前已观测落在该区间的有 **`0x32`、`0x34`、`0x37`**。

---

# 十、本轮的方法学沉淀

1. **PE 直接读跳表**：Ghidra 报 `Bad instruction` 的根因是 **int32 相对跳表落在 `.text` 内被当代码解码**。按节表映射 RVA→文件偏移、读 int32、`目标 = imagebase + table_rva + rel`。已验证 3 张表（R4）。
2. **PE 直接读机器码**：绕过 Ghidra 对微小叶子的建函数/反汇编失败（G1 用它闭合了 **159 个函数 / 46,091 字节**）。**汇编类断言一律走此法，并给出字节便于复核。**
3. **PE 扫描调用点立即数**：G8 用此法扫 167 个 `add_resource_access` 调用点，得出生产者归属表 —— 这是本轮的**新增手段**。
4. **`.pdata` 陷阱**：`get_xrefs_to` 会把 `.pdata` 异常目录条目报成"DATA 引用"，**不是调用边**。xref 输出必须用 `calls.json` 交叉验证。
5. **并发 lane 工具**：`scripts/render-recovery/run-lanes.mjs`（每 lane 独立 `database_id` → 独立 Ghidra 项目）。

---

# 十一、本轮的判断偏差记录（自我更正）

| # | 我的初始判断 | 更正 | 依据 |
|---|---|---|---|
| 1 | G6 的 `write = 0x32` **无证据、不予采信**（措辞过重） | 其实质结论**成立**；错的是"更正 C1"的框架 —— builder 与 accessor 是**两套 API** | G8 独立 PE 扫 167 个调用点 |
| 2 | G2 报的 `present` 非致命码 `OUT_OF_DATE(-1000001001)` | 正确值为 **`-1000001004`**；`-1000001001` 是 `NATIVE_WINDOW_IN_USE` | 父级读 `0x1418cc1b0.c` |
| 3 | 第二轮我曾"逐行核对确认" A2 的账本 `{index, access_type, sync}` | **字段标反**；正确为 `{index@+0, sync_group@+4, access_code@+8}`；我的确认**继承了同一个 Ghidra 绑定错误** | R2 复核 + 父级 PE 读 `PassResourceAccessType::new` = `mov eax,edx; mov edx,ecx; ret`（双参交换） |

---

# 十二、本轮后仍缺（详见 [GAPS.md](GAPS.md)）

| 类别 | 剩余 |
|---|---|
| **country_core** | 195 个具名 pass 中**尚未逐个体解剖的部分**；**7 个 `PassResources` TypeId 的类型名（不可反推）**；`0x1647B0`（第二个 compile 实例）的角色 |
| 访问码 | **`0x19`/`0x1d` 的语义**（扫描不到字面生产者 ⇒ 运行期计算值） |
| 结构 | `RawManagedHandle +0x00` 身份；`+0x680` 绝对基址；`ImageBarrier::new` 的 `+8`/`+0x10`；9 个 destroy 槽中其余 7 个的 `vkDestroy` 名；swapchain 槽 `[0x18..0x37]` |
| 算法 | `restir_di` reservoir 细节；`bin_lights` 分桶；`painterly`/`vermeer` 算法；SPIR-V 内的遮挡判据 |
| **方法性（D 类）** | `disassemble_function` **恒返 0 指令**；`get_xrefs_*` **无 CALL 边**；**零动态证据**；PDB 签名恒 `void()` |

**所有结论均为静态机器码/伪代码推断；本轮未运行游戏、未捕获帧。**
