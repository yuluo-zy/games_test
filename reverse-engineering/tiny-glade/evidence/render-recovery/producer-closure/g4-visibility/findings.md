# G4 取证：GPU-driven 可见性与绘制列表

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 方法：`run-lanes --lanes 2`（database `ccvis`）+ PE 段表读 rodata。
- 产物：`job-g4.json`、`job-g4b.json`、`REPORT-g4.json`、`REPORT-g4b.json`、`decomp/`（26 份 `0x*.c` + `_xrefs`）、`raw-extractions.txt`。
- 仅取证，未修改任何受保护文件。

## 0. 父级复核（三条硬结论）

| 结论 | 证据 | 结果 |
|---|---|---|
| `DrawIndexedIndirectCommand` = **20 字节** | `a2-graph-closure/decomp/0x140907990.c:31` = `param_2 * 0x14` | **确认** |
| 第二轮可见性受**全局字节**门控 | `evidence/render-architecture/0x1404ec110.c:290` = `if (*pcRam000000014362c0d8 == '\0')` | **确认** |
| `visibility_queries` **不是**网格遮挡 | exe 中 `firefly_vis_query` 存在（1 次），周围串为 `critters/firefly.raster.hlsl`、`firefly_glow_shader` | **确认** |

## 1. cull / bucket 算法

**CPU 侧只"编排 dispatch"，算法在 GPU shader。**

`_nani_cull_and_bucket.cs.hlsl`（workgroup **64×1×1**）绑定表：

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

⇒ **逐实例读实例 + atlas 信息，用 HZB 做 HZ 遮挡测试、结合 prev 位掩码，存活者写 `output_histogram`（每 atlas entry 计数）+ next 位掩码。**

**直方图 / 前缀和链**：
- `prefix_sum_unsorted_instances_per_atlas_entry`（串 `"Nani unsorted atlas idx prefix sum"`）：对每 atlas entry 的未排序实例数做前缀和；
- `group_draw_cmd_prefix_sum`（`"Nani draw cmd histogram prefix"` + `"subtotal prefix"`）：**两级**前缀和得每条 draw 的偏移；
- `fill_sorted_instance_list`（`"Nani fill unsorted/sorted instance list"`）：据前缀和 scatter 出排序实例表。

**桶 = atlas entry**（由 shader 名与 binding 名证明，**非编造**）。

## 2. `DrawIndexedIndirectCommand` = **20 字节 = 5×u32**

- `multi_draw_indexed_indirect`(`0x140907990`) 用 `index * 0x14`（**父级确认**）；
- 静态默认表 @ `0x142aaaa78` 共 **24 项 × 20 B**，字段 = `(indexCount, instanceCount=0, firstIndex=0, vertexOffset=0, firstInstance=0)`；
- `reset_indirect_args` 拷 `0x1e0 = 480 = 24 × 20`。

`draw_buffer` / `args_buffer` 是 shader `_nani_draw_indexed_indirect_to_dispatch.cs.hlsl`（**1×1×1**）的 `b13`/`b14`，用于把 draw 命令数转成间接 dispatch 参数。CPU 侧对应 `indirect_params::{Draw,Dispatch}IndirectBuffer`、`AtomicCounterBuffer`，由 `reset_indirect_args`(`0x14090a870`) 复位。

## 3. 两轮可见性

`render_world`(`0x1404ec110`)：

- 行 238：`gpu_generate_draw_lists(graph, world, 0)` → `shadow_pass` / `main_pass`
- 行 292：`generate_hzb` → `auStack_458`
- 行 **290**：`if (*pcRam000000014362c0d8 == '\0')`（**全局字节，为 0 才开**）
- 行 294/295：`generate_wall_draw_lists_pass2(hzb)` + `gpu_generate_draw_lists(graph, world, hzb)`
- 随后 `main_pass_oc2`

**第三参 = HZB（0 = 无）**。第二轮在 `gpu_generate_draw_lists` 内驱动：换缓冲（`FUN_1412d1180` vs `FUN_1412ce810`）、clear、flag（**4 vs 1**）、cull pipeline 分支（`param_9 = 1`）。

⇒ **第二轮 = 用第一轮 `main_pass` 深度建的 HZB 做遮挡复算（occlusion pass 2），供 `main_pass_oc2` 使用。**
**推断**（未证实）："补新显露物件"。

## 4. HZB 与 visibility_queries

- `generate_hzb`(`0x1409286f0`)：从深度建 **mip 金字塔**，写 **2 张 storage image**（access `0x37`），push args 24 B；其输出即 cull shader 的 `hzb_tex`。
- **`visibility_queries`(`0x140a2f2e0`) 实名 `"Firefly vis query"`（`/critters/firefly_vis_query.cs.hlsl`）** —— 是**萤火虫**可见性查询（在 `main_pass_oc2` 之后），**不是网格遮挡**（access `0x19`×3）。

> **父级注记**：这条**重新分类**了 `render_world` 步骤表里的 `visibility_queries` —— 此前按名字容易误读为网格遮挡查询。串证据（`firefly_vis_query` + `critters/firefly.raster.hlsl` + `firefly_glow_shader`）在 exe 中实测存在。

## 5. 实例布局（类型来自符号面）

- `render::ssbo::InstanceBuffer<T>`、`sparse_instance_buffer::SparseInstanceBuffer<T>`；
- atlas = `nani::mesh_atlas::{MeshAtlas, MeshAtlasPart, InstancedMeshAtlas, NaniMeshAtlasPartInfo}`；
- `nani::buffers::{NaniDrawListSsbo, Params, LengthSsbo, ListDrawCmdOffsets, OffsetDefaults, NaniInstanceIndirection{,Allocator}Ssbo, NaniFilteredInstanceAllocatorSsbo, GpuInstance, NaniNext/PrevVisibilityBitmaskSsbo}`。

`GpuWorld` 在 `World+0x1450`；`*(u32*)(GpuWorld+0x60)` = 实例/槽计数，count×4 的 u32 缓冲；allocator 槽 **stride 0x20 (32 B)** + 每槽 byte（`0x1412e3a60`）。

**字段级布局未逐字段证实。**

## 6. `create_cull_and_bucket_pass` 编排

`0x140a28950` 是 **4 个单态化之一**，差异**仅** shader ctor（`FUN_1415ff350`/`141600570`/`141601770`/`1415fcf10`，对应不同的 `NaniCullAndBucket*` 类别）。

建 `"Nani cull & bucket"` ComputePass，绑 6 组 gpu buffer/uniform，`param_9` 选两套 dispatch（`FUN_14160df90`+`141608e00` / `14160cd70`+`1416091e0`）。

`gpu_generate_draw_lists` 在其前跑 **3 个前置 pass**：`"Nani clear draw lists"` / `"Nani clear histograms"` / `"Nani prepare unsorted draw lists"`。

`cull_and_bucket_external` 泛型 `T = {WoodenPlankSsbo, RoofTileSsbo}`（按组件类型名匹配），走各自 SSBO。

## 7. 访问码实测

| 码 | 出处 |
|---|---|
| **`0x19`** | `FUN_1408ee240`/`FUN_140905260`：cull 全 6 变体、pass2、`generate_hzb`、`run_visibility_queries`×3 |
| **`0x37`** | `FUN_1408edd00`：HZB 写 |
| **`2`** | `FUN_1409dca90`：draw builder 间接 |
| **`0x34`** | `FUN_1409dca90`×4：`reset_indirect_args`（**复证 C1 的新增码**） |

## 8. 观察 / 推断 / 未知

**观察**：上述全部地址、字符串、20 B 命令布局、访问码、条件全局、pass 名。

**推断**：两轮 = HZB 复算；`0x19` 的语义。

**未知**：
1. `DrawIndexedIndirectCommand` 的**字段名**（属 std/Vk 常见约定，**未证**）；
2. **7 个 TypeId 的类型名**；
3. **`0x14362c0d8` 由谁写、含义为何**；
4. cull 的 push-constant 门控；prev/next 位掩码是**帧间**还是**轮间**；
5. **遮挡判据在 SPIR-V 内**（本轮仅有反射，无算法）。

**未宣称构图闭包已闭合。**
