# 保真度全量扫描（dxc 1.9.2609；638 模块 / 404 bin）

日期 2026-10-13。比值 = 重建/目标的"实指令"数；dist = opcode 直方图距离（目标指令数归一）。

- 硬门（entry/local_size/bindings）通过且比值 ≥ 0.8：**637/638**
- 被提升/打补丁的文件：113/262 main；inc 层 153 个文件，24 个不再被任何 main 引用（推断内容，非原作）

## 未通过项

- `final_post.raster.hlsl` ps_main `_final_post.raster.hlsl_f0adff76827e73ff.bin` ratio=0.051 hard=False
  - final_post f0ad ps：原作为无输出空壳，dxc 1.9 将其主体整体 DCE，绑定无法复现（已知例外，见文件头注释）

## 变体宏
每个 bin 的提升块由 `TGV_<id>` 保护；id = 16 位 hash，hash 缺省为 `0`，非 cs_main 入口加前缀（如 `cs_pass1_0`）。`sweep_all.py` 自动传入。

## inc 层未被引用的文件

- buildings/curved_halftimber.inc.hlsl
- clearing/canopy_vert.inc.hlsl
- clearing/inc/forest_floor.hlsl
- fxaa.inc.hlsl
- inc/parallax_mapping.hlsl
- inc/roof_colors_include.hlsl
- inc/wall_color_customization.hlsl
- nani/cull_and_bucket.inc.hlsl
- nani/inc/cull_and_bucket_owned_custom.hlsl
- nani/inc/trimesh.hlsl
- plaster.inc.hlsl
- roof/dormer_hole.inc.hlsl
- roof/stone_floor.inc.hlsl
- roof/wooden_floor.inc.hlsl
- roof_material_id.inc.hlsl
- smoke.inc.hlsl
- terrain_editing/inc_terrain_stroke.inc.hlsl
- terrain_editing/path_cobblestone_surface.inc.hlsl
- terrain_editing/sdf_utils.inc.hlsl
- wall/inc/brick_frag.hlsl
- wall/inc/brick_vert.hlsl
- wall/inc/highlight_mode.hlsl
- water/water_surface_frag.inc.hlsl
- wooden_plank.inc.hlsl
