# C1 取证：country_core 构图生产者闭包

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra + `run-lanes.mjs`（database `newcountry`）。
- 产物：`country_core_render_symbols.json`（**VA→名字索引，1.0 MB**）、`frame-composition.json`、`job-c1a-passbuilder.json`、`job-c1b-layout-xref.json`、`REPORT-c1a.json`/`REPORT-c1b.json`、`decomp/0x*.c`（17 份）。
- 仅取证，未修改任何游戏代码。

## 0. 父级复核

- `country_core_render_symbols.json` 结构完整（`counts` / `module_breakdown`(53 项) / `render_symbols`(1829) / `pass_functions`(195)），统计与报告一致。
- `frame-composition.json` 含 `render_world_pass_order`(59) 与 `compile_closure_order`(36)，与报告一致。
- **新增访问码 `0x34`**：父级在 `decomp/0x1409DDBB0.c:71` 独立确认 `FUN_1409dca90(&lStack_d8, uVar2, 0x34, 0)`。

## 1. 渲染函数面规模（观察）

来源 `evidence/render-architecture/public-symbols.json`（PDB `S_PUB32`，串内嵌 `[size]`）。

| 范围 | 符号数 | 带 VA | 已定界字节 |
|---|---:|---:|---:|
| `country_core::` 全部 | 6,298 | 6,250 | 1,824,488 |
| **渲染相关**（`render::` / `systems::render_loop`） | **1,843** | 1,829 | **475,588** |
| `systems::render_loop` | 273 | — | 86,840 |
| 具名 pass 函数 | 195 | — | — |

> **父级注记（对之前覆盖口径的重要修正）**：`country_core` 渲染面 **475,588 字节**，比整个 rhapsody 面（342,968 字节）**还大**。而我此前给的"rhapsody 覆盖 49.6%"只覆盖 rhapsody 域，**没有把这一层计入分母**。构图闭包正是这一层。

两大命名空间：
- `compute_passes::*`：`gpu_generate_draw_lists`、`rasterize_terrain_stroke`、`compute_water/garden`、`rasterize_buildings`、`read/mouse_read_terrain_*`、`roof_tiles_gravity`、`grass`、`stairs`、`ssgi`/`ray_trace_gi`/`restir_di`、`taa`/`dof`/`dof_autofocus`、`bin_lights`、`blur_pyramid`、`apply_screen_shadows`/`volumetrics`、`half_res`、`generate_mips`/`hzb`、`prefix_scan`、`reset_indirect_args`、`shader_refactoring`、`upload_world_raster` …
- `draw_passes::*`：`main_pass`/`main_pass_oc2`/`late_forward_and_deferred`、`shadow_pass`、`ui_pass`/`loading_screen`、`ui_2d_pass`/`mei_pass`、`wireframe`、`custom_cursor`、`deferred_raster`、`transparency_pass`(refraction/water/ice/pre_taa/post_taa)、`final_post`、`logo_overlay`、`feedback_report_bg`、`foliage_exclusion_mask`、`object_grass_push`、`draw_lists`、`draw_shaders::*` …

**归类局限**：PDB 反修饰混用 `::` / `..`（已归一）；1171 条泛型实例按前 3 段落 `??` 归类。

## 2. 帧构图顺序（观察）

帧入口属 **tiny_glade**，不是 country_core：`tiny_glade::render_loop::render`(`0x1404e8010`) → 调 `render_world`(`0x1404ec110`) 与 `RenderGraphState::compile` 两实例（`0x140161cd0` 主世界 / `0x1401647b0` UI），经 `execute_compiled`(`0x1400fa190`) 执行。

**真正的"登记哪些 pass"闭包被内联进 compile 实例 `0x140161cd0`**；其中 `render_world` 只被调用一次。

- **compile 闭包顺序（36 步）**：`import_texture_presets_to_rg` → `bindless_textures_dset` → `GpuWorld::import_to_rg` → `DynamicConstants` → `GpuLutTracker::render_luts` → `reset_indirect_args` → `primary_rt_debug` → **`render_world`** → `ColorGradingState::flattened` → `bidirectional_blur_pyramid` → `final_post` → [截图钩 `0x14031d140`/`0x1404ebfb0`] → `render_menu_bg` → `painterly_pass` → `vermeer_pass`(返 bool) → `render_logo_overlay` → `session_selection` → `photomode_lut_gallery` → `feedback_report_bg` → `wireframe_pass` → `ui_pass` → `ui_2d_pass`/`mei_pass` → `custom_cursor` → `loading_screen` → `RenderContext::finish` → `export_*_from_rg` → `ResourceInfo::calculate` → `populate_resources` → `CompiledRenderGraph::new`。
- **render_world 顺序（59 步）**：`upload_raster`×6 → `water_rasterize_from_cpu` → `rasterize_terrain_stroke` → `displace_rocky_terrain` → `read_terrain_data` → `mouse_read_terrain_stroke_index` → `foliage_exclusion_mask` → `highlight_active_terrain_stroke` → `object_grass_push` → `compute_path_masks` → `rasterize_buildings` → `garden_rasterize_from_cpu` → `water_spawn_vegetation` → `spawn_vegetation_around_buildings` → `read_bush_positions_for_cpu` → `garden_spawn_vegetation` → `path_pebbles` → `leaf_particles` → `roof_tiles_gravity` → `stairs` → `generate_wall_draw_lists` → **`gpu_generate_draw_lists`(0)** → `shadow_pass` → `calc_terrain_lighting_texture` → `main_pass` → `generate_hzb` → [cond] `generate_wall_draw_lists_pass2` + `gpu_generate_draw_lists` → `grass_compute` → `main_pass_oc2` → `visibility_queries` → `dof_autofocus` → `mouse_depth` → `late_forward_and_deferred` → `half_res` → `reprojection` → `ssgi` → `ray_trace_gi` → `eval_gi_screen_radiance` → `bin_lights` → `restir_di` → `apply_screen_shadows` → `deferred_raster` → `refraction_copy` → `water`|`ice_surface`（由 `cStack_49` 分支）→ `pre_taa_transparency` → `shadow_mask_mean` → `apply_volumetrics` → `dof_prepare` → [`iVar3 == 0x13`] `mips` + `calc_dof` → `calculate_taa` → `post_taa_transparency` → `render_shader_refactoring`。（全部 VA→名见 `frame-composition.json`。）

## 3. 一个 pass 的完整解剖：`gpu_generate_draw_lists`(`0x140987dd0`)

签名 `(graph, World, subset)`。动作序列：

1. 用**类型哈希 `0x662978458699a5ef`** 经 `bevy_ecs::Components::get_resource_id`(`0x1400dd6f0`, `World+0x38`) 取资源 → GpuWorld（子对象 `+0x1450`）；
2. `RenderGraph::create_raw_resource`(`0x140be94d0`) 建两个 RG 资源（名字 27 字节）得 u32 索引；
3. `ComputePass::new`(`0x1416032a0`，双名串 0x15/0x1e) + 链式 `write_storage_buffer`(`0x141606900`/`0x141604a30`/`0x141606730`) 绑常量；
4. `PassBuilder::render`(`0x1409dd4d0`) 装箱 **0xe8 字节描述符**并追加 **64 字节(0x40)句柄**进 `graph+0x90/0x98/0xa0`；
5. 第二个 ComputePass → `write_rg_storage_buffer`(`0x14096dd60`, 键 `0xfee3c4c9d72cf101`) → render；
6. `AssetSsboLibrary::get_rg_buffer`(`0x1412d1180`/`0x1412ce810`) → `clear_rg_buffer`(`0x1409ddbb0`)；
7. 第三个 ComputePass → `bind_external_storage_buffer`(`0x14096e2a0`) → `write_storage_buffer`(`0x141608090`) → `dispatch_with`(`0x14160a2e0`)；
8. `create_cull_and_bucket_pass`(`0x140a28950`) 编排 cull/bucket 子 pass。

子模块：`cull_and_bucket(_external)`、`fill_sorted_instance_list`、`prefix_sum_unsorted_instances_per_atlas_entry`、`group_draw_cmd_prefix_sum`、`generate_draw_lists`。

### 访问码实测（独立复核 A2）

| 调用 | 访问码 |
|---|---|
| `ComputePass.read` | `0x1a` |
| `ComputePass.write` | `0x37` |
| `DrawBuilder.read` | `0x24` |
| `DrawBuilder.write` | `0x37` |
| `multi_draw_indexed_indirect` | `2` |
| `dispatch_indirect_with` | `2` |
| **`clear_rg_buffer`** | **`0x34`**（**新增观测**，父级确认） |

`add_resource_access` 的 `self` 偏移：ComputePass 内嵌 builder 在 `+0x60`，DrawBuilder 在 `+0xe0 → +0x90`。

> **父级注记（关系到 R2 在查的矛盾）**：`0x34` **落在** A3 的并发分类区间 `[0x28, 0x37]` 内。这是第二个落在该区间的访问码（此前只知 `0x37`）。**这倾向于支持"区间里有尚未枚举完的访问码"这一解释**，而不是"账本 access_type 在进入 compiled 前还有一次变换"。R2 正在做全量调用点枚举，以它的结论为准。

`UniformBinder::write_uniform`(`0x14096b660`) 同样 swiss-table 查类型键，但**未见 access 声明调用**。

## 4. `PassResources::new_from`(`0x140909310`)

`(out, World)`。按 **7 个 u64 TypeId** 经 `Components::get_resource_id` 取 `Res`，写入 7×u64 结构。槽位 ↔ TypeId：

```
[0]=8597ee0727b461b3  [1]=a68561b396784eaa  [2]=8226cc786e679355  [3]=874b1541877dacfd
[4]=b65bf3141f93e938  [5]=47c94cb78283a72e  [6]=f0b963d163fe760f
```

任一缺失即 panic。`ComputePass::new`(`0x1416032a0`) 内部即调它（`world = param_6`）。**7 个具体类型名未知**（TypeId 不可反推）。

## 5. 结构布局（观察）

- **`ComputePass` = 0x190（400 字节）**（read/write 按值拷贝长度，`self+0` 起）；内嵌 `PassBuilder` 在 `+0x60`。两个单态化：`0x1405876a0`(6 参) / `0x1416032a0`(7 参)，参数 = `(out, 名字串, 描述符, World, graph)`。
- **`DrawBuilder` = 0xf8（248 字节）**；偏移：`+0x40` 绘制种类(4/1/7)、`+0x44` 资源、`+0x48` 计数、`+0x50`/`+0x58` 参数、`+0x60`=1、`+0x64` 表项、**`+0xe0` PassBuilder 指针、`+0xe8` 资源表**；录制的 draw 命令 **0xb8（184 字节）**。
- **pass 描述符 = 0xe8（232 字节）**（`render` 装箱 `memcpy`）。

## 6. 观察 / 推断 / 未知

**观察**：符号面与索引、两条 pass 顺序、pass 解剖与访问码（含新增 `0x34`）、7 个 TypeId 槽位、三处尺寸（0x190/0xf8/0xe8）与 DrawBuilder 偏移、`Components::get_resource_id` 为解析入口、`export_temporal`(`0x1404f7ca0`) 属 rhapsody。

**推断**：`0x14031d140`/`0x1404ebfb0` 为截图钩（名字来自 PDB，未逐步验证）；compile 实例 #3(`0x140166510`) 的角色。

**未知**：7 个 TypeId 的类型名；ComputePass/DrawBuilder 的字段名（0x190/0xf8 是"按值拷贝长度"，**未必等于分配尺寸**）；访问码 `0x1d`/`0x1e`/`0x17`/`0x19`/`0x34` 的命名；`UniformBinder` 的访问声明点；xref 仅 DATA、caller 未闭合。

**未宣称恢复完整 compile ABI，未宣称构图闭包已闭合。**
