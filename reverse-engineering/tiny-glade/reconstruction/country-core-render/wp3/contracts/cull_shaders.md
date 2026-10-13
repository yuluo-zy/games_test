# cull_shaders 契约 — cull_shaders.rs（0 procs，注册 by evidence）

标注：**O**=反编译/SPIR-V 直接支持；**I**=推断；**U**=未恢复。本模块在 PDB 无独立 native procedure（全部泛型/内联），契约按 `gpu_generate_draw_lists`（0x140987dd0）的 cull_and_bucket 闭包接线登记。

## 1. 类型（O，public-symbols + pass-declaration.md §2.1）

`NaniCullAndBucket` 族 compute pass 结构体（`AsComputeShaderSource` 实现）：
| 结构体 | URI | 用途（I） |
|---|---|---|
| `NaniCullAndBucketMain` | `/nani/cull_and_bucket.cs.hlsl:cs_main` | 通用主实例（atlas_part_info/indirection/meta/histogram/draw_list_info/visibility bitmask；一个变体带 `hzb_tex` 一个不带） |
| `NaniCullAndBucketBush` | `/nani/cull_and_bucket_bush.cs.hlsl` | 灌木 |
| `NaniCullAndBucketCurvedHalftimber` | `..._curved_halftimber...` | 弯曲半木 |
| `NaniCullAndBucketFlowers` | `..._flowers...` | 花 |
| `NaniCullAndBucketPlaster` | `..._plaster...` | 石膏 |
| `NaniCullAndBucketRoofTile` | `..._roof_tile...` | 屋顶瓦（瓦变体读 `instance_buffer`） |
| `NaniCullAndBucketSmoke` | `..._smoke...` | 烟 |
| `NaniCullAndBucketStairsFloor` | `..._stairs_floor...` | 楼梯地板 |
| `NaniCullAndBucketWoodenPlank` | `..._wooden_plank...` | 木板 |

SPIR-V（evidence/render-passes INDEX.md §3 / findings.md §5）：compute `64×1×1`，声明 atlas_part_info、filtered instance/allocator/meta、histogram、draw_list_info、prev/next visibility bitmask；`hzb_tex` 变体对 = 编译期特性（HZB 前/后两轮）。

## 2. 闭包接线（O，pass-declaration.md §2.2）

- `ComputePass::dispatch_with<...gpu_generate_draw_lists::cull_and_bucket::closure_env$1>` 0x14160a2e0。
- `ComputePass::dispatch_indirect_with<...group_draw_cmd_prefix_sum::closure_env$0<RenderSublistsIter>>` 0x14160b850。
- `gpu_generate_draw_lists`（0x140987dd0，见 evidence/render-architecture/0x140987dd0.c）：
  1. 造 GPU-only 临时缓冲，清零图缓冲（`FUN_140bd4e20` 分配 0x6c000/4B×…）。
  2. 读 GpuWorld subset（kind 槽），取 `NaniMeshAtlasPartInfo`/`NaniDrawListInfo` TypedStorageBuffer 与 `+0x1450` subset 数组。
  3. 构造 `cull_and_bucket` compute pass（第 1 轮无 hzb；第 2 轮带 hzb，`param_3 != 0` 分支选 `0x1412d1180`/`0x1412ce810` 两个 closure 变体）。
  4. `FUN_140a28950(...)` 执行 cull/bucket + histogram/prefix/allocate_draw_commands/group_draw_cmd_prefix_sum 链（对应 `/nani/{histogram_prefix_sum, group_draw_cmd_prefix_sum, subtotal_prefix_sum, allocate_draw_commands, generate_draw_lists, draw_indexed_indirect_to_dispatch}.cs.hlsl`）。

## 3. 帧序（O，render-passes INDEX.md）

- W22/W28：`gpu_generate_draw_lists` 第 1/2 轮（HZB 前/后）。
- W25/W30/W34/W46/W53：`nani_*` 光栅（uber/albedo_map/vertex_color/...）主绘制 + `draw_ext::nani_draw` 分派。

## 4. Unknown

- 每 kind 的剔除/分桶算法（包围体测试、LOD 选择、两轮新增物件策略、bitmask 传播）U——属 GPU shader 侧，WP3 只负责 CPU 侧接线与缓冲。
- `cull_and_bucket` 闭包的完整参数（view frustum、hzb 采样、每 kind 实例缓冲地址）U。
- `NaniCullAndBucket*` 结构体字段（除名称外）U。
