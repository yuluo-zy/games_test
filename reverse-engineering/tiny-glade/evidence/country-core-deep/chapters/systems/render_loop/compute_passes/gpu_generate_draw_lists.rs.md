# systems/render_loop/compute_passes/gpu_generate_draw_lists.rs

> 文件作用：Nani GPU 绘制列表生成（最高价值模块）：`gpu_generate_draw_lists`（W21/W22/W27/W28，CPU 侧编排第 1/2 轮 GPU draw-list 生成），`cull_and_bucket_external`/`create_cull_and_bucket_pass`/`fill_sorted_instance_list`/`prefix_sum_unsorted_instances_per_atlas_entry`/`group_draw_cmd_prefix_sum`/`generate_draw_lists`（RenderSublistsIter 迭代链），`generate_wall_draw_lists`/`generate_wall_draw_lists_pass2`（墙 draw-list 两轮）。全部 O（wp5 §1 + 本次反编译复证）。

函数总计 46：业务 13 / 内联无独立符号 14 / 派生胶水 19

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…gpu_generate_draw_lists` | 0x140987dd0 | 37 | 主系统（param_2=World，param_3=round）：World 查 GpuWorld subset 槽（FUN_1400dd6f0 hash 0x662978458699a5ef；`world+0x5b0`→subset+0x5c 帧计数）；`BufferDesc::new_gpu_only(0x6c000,0)` 建清空缓冲 + `(subset_inst_count[0x60]<<2)` 第二块；ComputePass::new ×2（clear_draw_lists/clear_histogram）；按 round 选 `AssetSsboLibrary::get_rg_buffer`（round0=0x1412d1180 / round≠0=0x1412ce810）+ `clear_rg_buffer`（0x1409ddbb0）；ComputePass::new cull_and_bucket + bind_external_storage_buffer（subset@+0x1458/+0x1480）→ multiunzip（0x140b2b7a0）→ `create_cull_and_bucket_pass<Main>`（0x140a28950）→ dispatch_with（0x14160a2e0）；`dispatch_indirect_with<group_draw_cmd_prefix_sum::closure_env$0<RenderSublistsIter>>`（0x14160b850）执行排序链 | O | render-loop\decomp\decomp-0x140987dd0.txt（串：Nani list sorting histogram、Nani filtered instance meta、Nani draw count per thread group、Nani group draw subtotals、Nani clear draw lists、/nani/clear_draw_lists.cs.hlsl、Nani clear histograms、/nani/clear_histogram.cs.hlsl、Nani prepare unsorted draw lists、/nani/prepare_unsorted_draw_lists.cs.hlsl、Nani generate draws、/nani/generate_draw_lists.cs.hlsl、Nani fill unsorted instance indirection、/nani/fill_unsorted_instance_indirection.cs.hlsl）；wp5 compute-passes §1 |
| `…cull_and_bucket_external<RoofTileSsbo, NaniCullAndBucketRoofTile>` | 0x140a280a0 | 532 | 外部实例缓冲剔除（屋顶瓦）：`utils::get_res` + `AssetSsboLibrary::get<RoofTileSsbo>` + `create_cull_and_bucket_pass` | O | render-loop\decomp\decomp-0x140a280a0.txt；wp5 §1 |
| `…cull_and_bucket_external<WoodenPlankSsbo, NaniCullAndBucketWoodenPlank>` | 0x140a27b10 | 532 | 外部实例缓冲剔除（木板）：同上 | O | render-loop\decomp\decomp-0x140a27b10.txt；wp5 §1 |
| `…create_cull_and_bucket_pass<NaniCullAndBucketBush>` | 0x140a28c60 | 584 | `ComputePass::new<T>`（400B builder）+ read_texture（hzb 变体）+ uniform；绑定链 FUN_141601770/141608a20/14096e000/14096e2a0 + hzb 分支（param_9==0 → FUN_14160df90/141608e00） | O | render-loop\decomp\decomp-0x140a28c60.txt；wp5 §1 |
| `…create_cull_and_bucket_pass<NaniCullAndBucketFlowers>` | 0x140a28f70 | 584 | 花剔除 pass 构造（同模式） | O | render-loop\decomp\decomp-0x140a28f70.txt；wp5 §1 |
| `…create_cull_and_bucket_pass<NaniCullAndBucketMain>` | 0x140a28950 | 584 | 主剔除 pass 构造（同模式；gpu_generate_draw_lists 调用点） | O | render-loop\decomp\decomp-0x140a28950.txt；wp5 §1 |
| `…create_cull_and_bucket_pass<ref$<dyn$<AsComputeShaderSource>>>` | 0x140a28630 | 584 | 动态分派剔除 pass 构造（trait object 变体） | O | render-loop\decomp\decomp-0x140a28630.txt；wp5 §1 |
| `…fill_sorted_instance_list<RenderSublistsIter>` | 0x140a29280 | 726 | 填充已排序实例列表：`/nani/fill_sorted_instance_list.cs.hlsl`（wp5 §1）；闭包 dispatch | O | render-loop\decomp\decomp-0x140a29280.txt；wp5 §1 |
| `…prefix_sum_unsorted_instances_per_atlas_entry<RenderSublistsIter>` | 0x140a29640 | 822 | 未排序实例按 atlas 项前缀和：`/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl`（wp5 §1）；dispatch_1d | O | render-loop\decomp\decomp-0x140a29640.txt；wp5 §1 |
| `…group_draw_cmd_prefix_sum<RenderSublistsIter>` | 0x140a297b0 | 957 | 组绘制命令前缀和：`/nani/group_draw_cmd_prefix_sum.cs.hlsl`（wp5 §1）；`dispatch_indirect_with`（closure_env$0，RenderSublistsIter 驱动） | O | render-loop\decomp\decomp-0x140a297b0.txt；wp5 §1 |
| `…generate_draw_lists<RenderSublistsIter>` | 0x140a29930 | 1084 | 生成绘制列表：串证据 `/nani/generate_draw_lists.cs.hlsl`；绑定 GpuWorld subset（param_3+0x1450/0x1458）+ 各 SSBO（hashes 0x2a75aeb2d5bdccbd/0x9b5e79943aa2230e/0x86fe9dc51990c5e3/0xd024032eecbe8bdf 等）；dispatch（0x14160afd0） | O | render-loop\decomp\decomp-0x140a29930.txt（串：/nani/generate_draw_lists.cs.hlsl、Nani fill unsorted instance indirection、/nani/fill_unsorted_instance_indirection.cs.hlsl）；wp5 §1 |
| `…generate_wall_draw_lists` | 0x14098aaf0 | 1213 | W21 墙 draw-list：串证据 `Generate wall draw lists/wall/generate_draw_lists.cs.hlsl:cs_pass1` + Wall pass2 reset inst counts；读 wall 实例（UnsafeWorldCell::get_entity 闭包），`ssbo_to_rg_handle` + ComputePass::write_draw_indirect_cmd_buffer | O | render-loop\decomp\decomp-0x14098aaf0.txt；wp5 §1 |
| `…generate_wall_draw_lists_pass2` | 0x14098b2a0 | 1307 | W27 墙 draw-list 第 2 轮：`:cs_pass2` + `/wall/reset_pass2_inst_counts.cs.hlsl`；`reset_wall_draw_indirect_args`（行 1287，DrawBuilder::uniform + dispatch_1d）+ read_texture（hzb_tex@0.19） | O | render-loop\decomp\decomp-0x14098b2a0.txt；wp5 §1 |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `clear_draw_lists`（行 216）— `/nani/clear_draw_lists.cs.hlsl`；dispatch_1d
- `clear_histogram`（行 240）— `/nani/clear_histogram.cs.hlsl`；dispatch_1d
- `cull_and_bucket`（行 260）— `/nani/cull_and_bucket.cs.hlsl`（dispatch_with 闭包 closure_env$1；iter `enum_iterator::All<GpuWorldSubsetKind>` + multiunzip）
- `draw_indexed_indirect_to_dispatch`（行 506）— `/nani/draw_indexed_indirect_to_dispatch.cs.hlsl`；RenderGraphRecorder::create + push_constants + dispatch_1d
- `create_cull_and_bucket_pass`（行 584）— 见业务表（4 单态 + dyn 变体）
- `histogram_prefix_sum`（行 645）— `/nani/histogram_prefix_sum.cs.hlsl`；dispatch_1d
- `IndirectArgsType::buffer_offset`（行 678）— 枚举 `IndirectArgsType`（O 名，语义 I）
- `generate_draw_list_dispatch_args`（行 687）— `/nani/generate_dispatch_args.cs.hlsl`；dispatch_1d
- `clear_draw_cmd_histogram`（行 871）— `/nani/histogram_prefix_sum.cs.hlsl` 清空路径；dispatch_1d
- `clear_draw_cmd_subtotals`（行 891）— `/nani/clear_subtotals.cs.hlsl`；push_constants + dispatch_1d
- `allocate_draw_lists`（行 911）— `/nani/allocate_draw_commands.cs.hlsl`（I，dispatch 在闭包）
- `prepare_empty_unsorted_draw_lists`（行 1033）— `/nani/prepare_unsorted_draw_lists.cs.hlsl`；读 GpuWorld::mesh_atlas_info_buf（O）
- `fill_unsorted_instance_indirection`（行 1165）— `/nani/fill_unsorted_instance_indirection.cs.hlsl`
- `reset_wall_draw_indirect_args`（行 1287）— DrawBuilder::uniform + dispatch_1d

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`next` 等 19 项（RenderSublistsIter::next 等迭代器胶水）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `gpu_generate_draw_lists`（0x140987dd0）—— round0/round≠0 双路径编排
处理链（O，c1 §2 + decomp）：
1. World 查 GpuWorld（TypeId `0x662978458699a5ef`）；subset 槽（`world+0x5b0`→subset+0x5c 帧计数）；
2. 建缓冲：`BufferDesc::new_gpu_only(0x6c000, 0)`（清空）+ `(subset_inst_count[0x60]<<2)` 第二块；
3. `ComputePass::new ×2`：**clear_draw_lists**（`dispatch_1d 108`）/ **clear_histogram**（`dispatch_1d 0x1b000`）——固定形状 `.rdata` 描述符；
4. **round 分叉**（第三参）：round0 → `get_rg_buffer` 0x1412d1180 + `clear_rg_buffer`；round≠0 → 0x1412ce810（带 hzb 的第 2 轮路径）；
5. `ComputePass::new` cull_and_bucket + `bind_external_storage_buffer`（subset@+0x1458/+0x1480）→ multiunzip（0x140b2b7a0）→ `create_cull_and_bucket_pass<Main>`（0x140a28950）→ `dispatch_with`（0x14160a2e0 闭包逐项 push_constants(20B) + `dispatch_3d(count,1,1)`，closure_env$1 0x1415fa2f0）；
6. `dispatch_indirect_with<group_draw_cmd_prefix_sum::closure_env$0>`（0x14160b850）执行排序链。

### 2. 15 个 nani pass 链（O 序，shader URI 全部证实）
```
clear_draw_lists(108) → clear_histogram(0x1b000) → prepare_unsorted_draw_lists → cull_and_bucket*
→ histogram_prefix_sum(dispatch_1d 1) → group_draw_cmd_prefix_sum(间接) → clear_subtotals
→ allocate_draw_commands → generate_draw_lists(间接) → draw_indexed_indirect_to_dispatch
→ fill_sorted_instance_list(间接) → fill_unsorted_instance_indirection(间接)
→ prefix_sum_unsorted_instances_per_atlas_entry → generate_dispatch_args
```
（round≠0 带 hzb：cull_and_bucket 的 hzb 分支读 `hzb_tex`，access 0x19。）

### 3. 墙两轮
- **generate_wall_draw_lists**（0x14098aaf0，W21）：`/wall/generate_draw_lists.cs.hlsl:cs_pass1` + `/wall/reset_pass2_inst_counts.cs.hlsl`；读 wall 实例（UnsafeWorldCell::get_entity 闭包），`ssbo_to_rg_handle` + `ComputePass::write_draw_indirect_cmd_buffer`。
- **generate_wall_draw_lists_pass2**（0x14098b2a0，W27）：`:cs_pass2`（读 `hzb_tex`）+ `reset_wall_draw_indirect_args`（DrawBuilder::uniform + dispatch_1d）。**门控 `*0x14362c0d8（freeze_occlusion_culling）==0`**（round3 O）。
- 帧序：W21 → **W22（round0）** → shadow/main/hzb → W27/W28（round≠0）仅在 `freeze_occlusion_culling==0` 时（render_world 章）。

### 4. 资源/绑定（O）
读 `NaniMeshAtlasPartInfo`、`NaniInstanceIndirectionSsbo`；写 `NaniDrawListSsbo`、`NaniDrawListLengthSsbo`、histogram/counter SSBO；`frame_constants` uniform；`hzb_tex`（第 2 轮）。**RenderSublistsIter**（27 子集迭代器）驱动 `dispatch_indirect_with` 的闭包（PushConstants [u32;5] 20B，c1 纠错）。

## 关键事实

- Shader URI（O）：`/nani/{clear_draw_lists,clear_histogram,prepare_unsorted_draw_lists,cull_and_bucket*,histogram_prefix_sum,group_draw_cmd_prefix_sum,subtotal_prefix_sum,allocate_draw_commands,generate_draw_lists,draw_indexed_indirect_to_dispatch,fill_sorted_instance_list,fill_unsorted_instance_indirection,prefix_sum_unsorted_instances_per_atlas_entry,clear_subtotals,generate_dispatch_args}.cs.hlsl`；`/wall/generate_draw_lists.cs.hlsl:cs_pass1|cs_pass2`；`/wall/reset_pass2_inst_counts.cs.hlsl`。
- 资源读/写（O 句柄 hash）：read `NaniMeshAtlasPartInfo`、`NaniInstanceIndirectionSsbo`；write `NaniDrawListSsbo`、`NaniDrawListLengthSsbo`、histogram/counter SSBO；`frame_constants` uniform；`hzb_tex`（第 2 轮 read_texture）。
- 帧序：W22（round 0，无 hzb）→ W27/W28（round≠0，带 hzb，`\*0x14362c0d8==0` 时）。
