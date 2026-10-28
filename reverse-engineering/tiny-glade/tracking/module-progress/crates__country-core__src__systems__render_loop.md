# crates/country-core/src/systems/render_loop

状态：记录有交付，本机缺失。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

render_world帧顺序、54pass调用、8条件分支及CPU组合有分析/交付记录。

## 接线与验证

接线：not_established。

B原二进制+404shader静态分析；没有整帧GPU验收。

## 已恢复的函数和类型范围

- render_world
- HZB/GI/透明/DoF/TAA或TGSR及呈现调用顺序

## 明确缺口

- CPU交付源码本机缺失
- 逐pass变体、descriptor/resource handle、GPU时序与整帧画面对照

下一闭包：CPU交付源码本机缺失

## 证据

- [evidence/render-passes/INDEX.md](../../evidence/render-passes/INDEX.md)
- PROJECT_MEMORY.md

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/bin_lights.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/compute_water.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/displace_rocky_terrain.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/dof.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/dof_autofocus.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/draw_path_mask.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/eval_gi_screen_radiance.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/generate_mips.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/grass.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/grass2.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/half_res.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/image_baker.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/leaf_particles.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/mouse_read_terrain_stroke_index.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/path_pebbles.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/prefix_scan.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/primary_rt_debug.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/ray_trace_gi.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/read_terrain_data.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/reprojection.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/reset_indirect_args.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/restir_di.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/roof_tiles_gravity.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/shader_refactoring.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/spawn_boats.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/ssgi.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/stairs.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/taa.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/terrain_lighting_texture.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/compute_passes/visibility_queries.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/blit.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/deferred_raster.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/clouds.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/floor_shader.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/leaf_particle_draw.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/path_pebble_shader.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/stars.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/feedback_report_bg.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/final_post.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/foliage_exclusion_mask.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/grass_velocity_reconstruct.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/logo_overlay.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/material_id.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/object_grass_push.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/shadow_pass.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/draw_passes/wireframe_pass.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/find_crash.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/indirect_params.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/lut_gallery.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/menu_background.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/mod.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/painterly/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/resources/critters.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/session_selection/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/render_loop/vermeer.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs)

SHA256：`8c555761ae17ea1964c80825b7b7212c0ca86db6dce29532a64f8a880867d7b6`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `apply_screen_shadows` | 15 | 未由扫描证明；以证据限定范围为准 |
| `compute_shadow_mask_mean` | 20 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：use crate::typed_gap::Gap;
- L17：unimplemented!()
- L22：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs)

SHA256：`1aefefc54b27a057fc794c70e81f9995989a61b1de6a0900047c97e905894ba3`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `apply_volumetrics` | 6 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：pub fn apply_volumetrics(world: &mut crate::typed_gap::Gap, _frame: &crate::consume::core::FrameConstants) {
- L8：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/bin_lights.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/bin_lights.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/bin_lights.rs)

SHA256：`af0c77efc75da20a39a3787996d5942f359613a71711a12b215bcda3efb045b7`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `source_hash` | 15 | 未由扫描证明；以证据限定范围为准 |
| `as_compute_pipeline` | 18 | 未由扫描证明；以证据限定范围为准 |
| `bin_lights` | 26 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L26：pub fn bin_lights(world: &mut crate::typed_gap::Gap, _light_count: u32) {
- L28：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs)

SHA256：`81db163064223a35d4a1e2eed09ee1ed074a163aa438eb142a03a5a1db7d24ea`；缺口注释标记数：7。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `blur_pyramid` | 9 | 未由扫描证明；以证据限定范围为准 |
| `rev_blur_pyramid` | 14 | 未由扫描证明；以证据限定范围为准 |
| `bidirectional_blur_pyramid` | 19 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：use crate::typed_gap::Gap;
- L9：pub fn blur_pyramid(world: &mut crate::typed_gap::Gap, _src: &Gap, _levels: u32) {
- L11：unimplemented!()
- L14：pub fn rev_blur_pyramid(world: &mut crate::typed_gap::Gap, _src: &Gap, _levels: u32) {
- L16：unimplemented!()
- L19：pub fn bidirectional_blur_pyramid(world: &mut crate::typed_gap::Gap, _src: &Gap, _levels: u32) {
- L22：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/compute_garden.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/compute_garden.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/compute_garden.rs)

SHA256：`117bdc968608e0a30e73354a92fbb711fbca78f88e2cf890578948a2b0b2370b`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `garden_rasterize_from_cpu` | 17 | 未由扫描证明；以证据限定范围为准 |
| `garden_spawn_vegetation` | 22 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L9：use crate::typed_gap::Gap;
- L19：unimplemented!()
- L24：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs)

SHA256：`48b027d526decdf47c21a01b7769cd668ed28a9173fc42d380f4e4aa5b74ffb0`；缺口注释标记数：6。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `compute_path_masks` | 8 | 未由扫描证明；以证据限定范围为准 |
| `blur_exclusion_mask` | 13 | 未由扫描证明；以证据限定范围为准 |
| `blur_path_mask` | 18 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：pub fn compute_path_masks(world: &mut crate::typed_gap::Gap, _paths: &crate::typed_gap::Gap) {
- L10：unimplemented!()
- L13：fn blur_exclusion_mask(world: &mut crate::typed_gap::Gap, _mask: &crate::typed_gap::Gap) {
- L15：unimplemented!()
- L18：fn blur_path_mask(world: &mut crate::typed_gap::Gap, _mask: &crate::typed_gap::Gap) {
- L20：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/compute_water.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/compute_water.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/compute_water.rs)

SHA256：`2bbbe3a13395d36a636225cc67fa6311b4931b9ebdbb40ce2f91c27d324c9caa`；缺口注释标记数：6。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `rg_buffer_hash` | 33 | 未由扫描证明；以证据限定范围为准 |
| `rg_buffer_hash` | 38 | 未由扫描证明；以证据限定范围为准 |
| `rg_buffer_hash` | 43 | 未由扫描证明；以证据限定范围为准 |
| `world_to_water_grid` | 50 | 未由扫描证明；以证据限定范围为准 |
| `water_rasterize_from_cpu` | 58 | 未由扫描证明；以证据限定范围为准 |
| `water_spawn_vegetation` | 76 | 未由扫描证明；以证据限定范围为准 |
| `water_level_at_xz` | 85 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L24：use crate::typed_gap::Gap;
- L32：/// typed gap：原值 = `utils::name_hash("…LilypadSsbo")`。
- L65：// 纹理槽：preset 9/10/0x13（水 mask 输出）。rhapsody PassBuilder 接入边界 todo。
- L67：unimplemented!()
- L79：unimplemented!()
- L84：/// 数值语义（0..255 水位码值）U；此处以 `WorldRaster` typed-gap 呈现。
### reconstruction/country-core/src/systems/render_loop/compute_passes/displace_rocky_terrain.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/displace_rocky_terrain.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/displace_rocky_terrain.rs)

SHA256：`37cafc5ed61d6d9fef0e8bb63edfa469e2bbe5fd5cd4849b92cb77d5e8f25b5b`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `displace_rocky_terrain` | 23 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L20：use crate::typed_gap::Gap;
- L26：unimplemented!(
### reconstruction/country-core/src/systems/render_loop/compute_passes/dof.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/dof.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/dof.rs)

SHA256：`a31e1347fb79b94840b49b595373551e2c9e7c7e495478babaafd7a83bf7e17e`；缺口注释标记数：5。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `source_hash` | 26 | 未由扫描证明；以证据限定范围为准 |
| `as_compute_pipeline` | 29 | 未由扫描证明；以证据限定范围为准 |
| `dof_prepare` | 49 | 未由扫描证明；以证据限定范围为准 |
| `calc_dof` | 56 | 未由扫描证明；以证据限定范围为准 |
| `calc_dof_multires` | 63 | 未由扫描证明；以证据限定范围为准 |
| `calc_dof_fullres` | 70 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L12：use crate::typed_gap::Gap;
- L51：unimplemented!()
- L57：unimplemented!()
- L66：unimplemented!()
- L71：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/dof_autofocus.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/dof_autofocus.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/dof_autofocus.rs)

SHA256：`0e523e15b2260b89335d0ea256a71097a45ae7fd0d239f347969c3bfe09fa942`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `calculate_dof_autofocus` | 15 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：use crate::typed_gap::Gap;
- L17：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/eval_gi_screen_radiance.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/eval_gi_screen_radiance.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/eval_gi_screen_radiance.rs)

SHA256：`7b83c40f74b902bedf363a3e6eb78eadeb5669e66e0ceaa3103da3a46ee1f403`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `eval_gi_screen_radiance` | 7 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：pub fn eval_gi_screen_radiance(world: &mut crate::typed_gap::Gap, _frame: &crate::consume::core::FrameConstants) {
- L9：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs)

SHA256：`3d9cbfc2d6e6808a4baddf9666c591a57f15d2f8e984ae773bd2589112d6c23a`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `generate_hzb` | 13 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L11：use crate::typed_gap::Gap;
- L18：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/generate_mips.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/generate_mips.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/generate_mips.rs)

SHA256：`42754b3eb8478f9c36ffe27e78cb99d0a96a14650232e25cc9a82f7b36f3718c`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `generate_mips` | 8 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：pub fn generate_mips(world: &mut crate::typed_gap::Gap, _texture: &crate::typed_gap::Gap) {
- L10：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs)

SHA256：`9dee6410aa72ec817236337a38a0d6e5ebc2e1d3280593b28e19c257aed35a4e`；缺口注释标记数：10。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `buffer_offset` | 38 | 未由扫描证明；以证据限定范围为准 |
| `build_push_constants` | 54 | 未由扫描证明；以证据限定范围为准 |
| `next` | 76 | 未由扫描证明；以证据限定范围为准 |
| `gpu_generate_draw_lists` | 83 | 未由扫描证明；以证据限定范围为准 |
| `clear_draw_lists` | 101 | 未由扫描证明；以证据限定范围为准 |
| `clear_histogram` | 106 | 未由扫描证明；以证据限定范围为准 |
| `prepare_empty_unsorted_draw_lists` | 112 | 未由扫描证明；以证据限定范围为准 |
| `cull_and_bucket` | 119 | 未由扫描证明；以证据限定范围为准 |
| `draw_indexed_indirect_to_dispatch` | 130 | 未由扫描证明；以证据限定范围为准 |
| `histogram_prefix_sum` | 137 | 未由扫描证明；以证据限定范围为准 |
| `generate_draw_list_dispatch_args` | 142 | 未由扫描证明；以证据限定范围为准 |
| `clear_draw_cmd_histogram` | 147 | 未由扫描证明；以证据限定范围为准 |
| `clear_draw_cmd_subtotals` | 152 | 未由扫描证明；以证据限定范围为准 |
| `allocate_draw_lists` | 158 | 未由扫描证明；以证据限定范围为准 |
| `fill_unsorted_instance_indirection` | 164 | 未由扫描证明；以证据限定范围为准 |
| `create_cull_and_bucket_pass` | 171 | 未由扫描证明；以证据限定范围为准 |
| `cull_and_bucket_external` | 184 | 未由扫描证明；以证据限定范围为准 |
| `generate_wall_draw_lists` | 198 | 未由扫描证明；以证据限定范围为准 |
| `generate_wall_draw_lists_pass2` | 206 | 未由扫描证明；以证据限定范围为准 |
| `reset_wall_draw_indirect_args` | 212 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L20：//! （enum-iterator 离线缓存 env gap）；pass 构造路径为 todo 标记。
- L24：use crate::typed_gap::Gap;
- L32：// Variants not recovered (typed gap); each maps to an indirect-buffer slot.
- L39：unimplemented!()
- L77：unimplemented!()
- L97：unimplemented!()
- L176：todo!(
- L190：todo!(
- L200：unimplemented!()
- L208：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/grass.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/grass.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/grass.rs)

SHA256：`174a7833473bd00b8387c275478555c9fdbbed6f4092c9885662896c13c056d3`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `register_required_components` | 17 | 未由扫描证明；以证据限定范围为准 |
| `grass_precompute` | 23 | 未由扫描证明；以证据限定范围为准 |
| `grass_compute` | 29 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L9：use crate::typed_gap::Gap;
- L18：unimplemented!()
- L24：unimplemented!()
- L31：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/half_res.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/half_res.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/half_res.rs)

SHA256：`fe1995025aa4ba2bbf9f180ee53f2d916ae6910a77760c71b589a1b07551a8c0`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `extract_half_res_depth_normals` | 7 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：pub fn extract_half_res_depth_normals(world: &mut crate::typed_gap::Gap, _recorder: &mut crate::typed_gap::Gap) {
- L9：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs)

SHA256：`c3b514a535c5c096c7f3a838a8fd7b1c4b398fd9661cc9609b232be07f0707d0`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `highlight_active_terrain_stroke` | 11 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：use crate::typed_gap::Gap;
- L13：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/leaf_particles.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/leaf_particles.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/leaf_particles.rs)

SHA256：`4c8e8efb316018191be9d4b096343e20eca9ccff7befa0ebd9305b47c1cfc828`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `leaf_particles_compute` | 13 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：use crate::typed_gap::Gap;
- L15：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/mod.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/mod.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/mod.rs)

SHA256：`5f659e301d750f3bdc3a7b687fe05febe302a4548175ac792046db99365aad51`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs)

SHA256：`e50eb69bd87f8787cc456cffa7ddb5445f3b7e474461096aeefb34f186979c78`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `mouse_depth_compute` | 9 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：use crate::typed_gap::Gap;
- L9：pub fn mouse_depth_compute(world: &mut crate::typed_gap::Gap, _window: &Gap, _cursor: &Gap) {
- L11：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/mouse_read_terrain_stroke_index.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/mouse_read_terrain_stroke_index.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/mouse_read_terrain_stroke_index.rs)

SHA256：`316f7e3c74a8ec73d3da03a219f3dcb0acf610b39581eb9502177e7601aa87ff`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `mouse_read_terrain_stroke_index` | 11 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：use crate::typed_gap::Gap;
- L13：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/path_pebbles.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/path_pebbles.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/path_pebbles.rs)

SHA256：`d66ea034c043f796a06a1e1dd5368156890b25370e323c8e7064d2e61f8eef4e`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `path_pebbles_compute` | 11 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：use crate::typed_gap::Gap;
- L13：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/prefix_scan.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/prefix_scan.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/prefix_scan.rs)

SHA256：`6733424b9a3e086d499e86f3ffa7e7bd924e2b1365d21f426dba80fb55d30dd9`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `inclusive_prefix_scan_u32_1m` | 6 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：pub fn inclusive_prefix_scan_u32_1m(world: &mut crate::typed_gap::Gap, _src: &crate::typed_gap::Gap, _count: u32) {
- L8：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/primary_rt_debug.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/primary_rt_debug.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/primary_rt_debug.rs)

SHA256：`20e1edcba750e9b912ee688d43653316d79175e3cead32c2dd0d2dc168670628`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `primary_rt_debug` | 6 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：pub fn primary_rt_debug(world: &mut crate::typed_gap::Gap, _frame: &crate::consume::core::FrameConstants) {
- L8：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs)

SHA256：`9f78662e6b3493991cd0ef34537b269a40ac1e15d7414e752ee5258fd8c9aa68`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `rasterize_buildings` | 12 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：use crate::typed_gap::Gap;
- L14：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs)

SHA256：`94fa5c6062705fc6b9639df5f03b6dd3814d1396545dc9ed79a8adc6fba0be91`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `source_hash` | 33 | 未由扫描证明；以证据限定范围为准 |
| `as_compute_pipeline` | 36 | 未由扫描证明；以证据限定范围为准 |
| `as_shader_def_value` | 50 | 未由扫描证明；以证据限定范围为准 |
| `hash` | 54 | 未由扫描证明；以证据限定范围为准 |
| `rasterize_terrain_stroke` | 60 | 未由扫描证明；以证据限定范围为准 |
| `gauss_blur` | 73 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L28：use crate::typed_gap::Gap;
- L67：unimplemented!(
### reconstruction/country-core/src/systems/render_loop/compute_passes/ray_trace_gi.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/ray_trace_gi.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/ray_trace_gi.rs)

SHA256：`038c2cb4720dcec1f282e99d89e48f394678e742fa3c45e03d974ef726e09e7d`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `ray_trace_gi` | 12 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L10：use crate::typed_gap::Gap;
- L14：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs)

SHA256：`6a0e44eafb9e6914afa8ff86ded952015783358fa13c83ac590b56bd1d6afc07`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `read_bush_positions_for_cpu` | 10 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：use crate::typed_gap::Gap;
- L12：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/read_terrain_data.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/read_terrain_data.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/read_terrain_data.rs)

SHA256：`8ec1a3e4540364c8a78110e6ffe8782ee811013d56be4bd8c1b92314849bc047`；缺口注释标记数：6。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `pack_water_mask_bits` | 22 | 未由扫描证明；以证据限定范围为准 |
| `read_terrain_data` | 30 | 未由扫描证明；以证据限定范围为准 |
| `read_terrain_height` | 38 | 未由扫描证明；以证据限定范围为准 |
| `read_terrain_rock_mask` | 43 | 未由扫描证明；以证据限定范围为准 |
| `read_water_mask` | 48 | 未由扫描证明；以证据限定范围为准 |
| `read_garden_mask` | 53 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L18：use crate::typed_gap::Gap;
- L35：unimplemented!()
- L40：unimplemented!()
- L45：unimplemented!()
- L50：unimplemented!()
- L55：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/reprojection.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/reprojection.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/reprojection.rs)

SHA256：`754d6727a65147b051165c33bd1be5a419263368534dd7f233aa2d8274f7acc6`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `calculate_reprojection_map` | 6 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：pub fn calculate_reprojection_map(world: &mut crate::typed_gap::Gap, _frame: &crate::consume::core::FrameConstants) {
- L8：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/reset_indirect_args.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/reset_indirect_args.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/reset_indirect_args.rs)

SHA256：`35f73c8e0671e6abeec139c330f76b69cdf3c4c7b928fc00cab64b1bbb667531`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `reset_indirect_args` | 7 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：pub fn reset_indirect_args(world: &mut crate::typed_gap::Gap, _args: &[crate::typed_gap::Gap]) {
- L9：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/restir_di.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/restir_di.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/restir_di.rs)

SHA256：`c6997603bb382079836b78de9856222d22ac15864a20db8dab01d54cdac052fd`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `restir_sample_direct_lights` | 7 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：pub fn restir_sample_direct_lights(world: &mut crate::typed_gap::Gap, _light_count: u32) {
- L9：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/roof_tiles_gravity.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/roof_tiles_gravity.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/roof_tiles_gravity.rs)

SHA256：`e1a28ce318ab1246377923294fda4d83221698a9c47a8e1cbd756c200c39f070`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `roof_tiles_gravity_compute` | 13 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：use crate::typed_gap::Gap;
- L15：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/shader_refactoring.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/shader_refactoring.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/shader_refactoring.rs)

SHA256：`cf322943e19386609c3f604c3f0e979805967b8f67f7107251c040e48806c73e`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `render_shader_refactoring` | 7 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：pub fn render_shader_refactoring(world: &mut crate::typed_gap::Gap, _state: &crate::typed_gap::Gap) {
- L9：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/spawn_boats.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/spawn_boats.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/spawn_boats.rs)

SHA256：`180a934ac44828278d22adf4d0f6d52d20c07ee1cdf4937fef723cb6aeb2b309`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `rg_buffer_hash` | 22 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L15：use crate::typed_gap::Gap;
- L21：/// typed gap：原值 = `utils::name_hash("…BoatSsbo")`。
### reconstruction/country-core/src/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs)

SHA256：`c307d940d2339f10b0df27582f9c0bae13ef183487ca3c947b4607e9f9b6d56e`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `spawn_vegetation_around_buildings` | 7 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：pub fn spawn_vegetation_around_buildings(world: &mut crate::typed_gap::Gap, _buildings: &crate::typed_gap::Gap) {
- L9：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/ssgi.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/ssgi.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/ssgi.rs)

SHA256：`074fdf7705efc80414a96927cc42b86ace6e40b0be9ae583eabe7984a9391664`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `calculate_ssgi` | 6 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：pub fn calculate_ssgi(world: &mut crate::typed_gap::Gap, _frame: &crate::consume::core::FrameConstants) {
- L8：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/stairs.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/stairs.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/stairs.rs)

SHA256：`6385669494ae1e9759322555a8cc8175a57c95dd929e1b66ccd00a6ddd9ccb1a`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `stairs` | 12 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：use crate::typed_gap::Gap;
- L14：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/taa.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/taa.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/taa.rs)

SHA256：`c902936574793cbe0083a5d4d96f8a8458ad4c8a545738ccd44481e76ffdda0c`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `source_hash` | 18 | 未由扫描证明；以证据限定范围为准 |
| `as_compute_pipeline` | 21 | 未由扫描证明；以证据限定范围为准 |
| `calculate_taa` | 31 | 未由扫描证明；以证据限定范围为准 |
| `calculate_tgsr` | 38 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L12：use crate::typed_gap::Gap;
- L33：unimplemented!()
- L40：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/terrain_lighting_texture.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/terrain_lighting_texture.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/terrain_lighting_texture.rs)

SHA256：`289a6cdafa2a0bddb6bc78b1aa6c2b71df76a3a61faff604605e9437068ffcd1`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `calc_terrain_lighting_texture` | 22 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L19：use crate::typed_gap::Gap;
- L24：unimplemented!("recover calc_terrain_lighting_texture: eye_position uniform + preset 8/9/10/0x11 + dispatch + generate_mips")
### reconstruction/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs)

SHA256：`21bdcf04d2b3b9dc7685f091c786a7c8ad8dca4b3c01aa786fd6a9e205c4e7c7`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `upload_world_raster` | 19 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L9：use crate::typed_gap::Gap;
- L21：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/compute_passes/visibility_queries.rs

[reconstruction/country-core/src/systems/render_loop/compute_passes/visibility_queries.rs](../../reconstruction/country-core/src/systems/render_loop/compute_passes/visibility_queries.rs)

SHA256：`4403a5f924f0fbe77e462e4f7d62173b2204258637f7caaab37dc91d457a6443`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `run_visibility_queries` | 8 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：use crate::typed_gap::Gap;
- L8：pub fn run_visibility_queries(world: &mut crate::typed_gap::Gap, _view: &Gap) {
- L10：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs)

SHA256：`1f6e2168d803368b07bccf64ce2a4da4e55496c5fc1dca6e742c5113ccd914b0`；缺口注释标记数：8。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 17 | 未由扫描证明；以证据限定范围为准 |
| `bind_common` | 23 | 未由扫描证明；以证据限定范围为准 |
| `bind_roof_color_customization` | 27 | 未由扫描证明；以证据限定范围为准 |
| `bind_wall_color_customization` | 31 | 未由扫描证明；以证据限定范围为准 |
| `bind_window_color_customization_textures` | 34 | 未由扫描证明；以证据限定范围为准 |
| `bind_brick_erase_highlight` | 37 | 未由扫描证明；以证据限定范围为准 |
| `bind_stone_floor_textures` | 41 | 未由扫描证明；以证据限定范围为准 |
| `bind_wooden_floor_textures` | 44 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L11：use crate::typed_gap::Gap;
- L24：unimplemented!()
- L29：unimplemented!()
- L32：unimplemented!()
- L35：unimplemented!()
- L39：unimplemented!()
- L42：unimplemented!()
- L45：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs)

SHA256：`6889d0e71e3c22fb9018fd1ef3c7b2b842a8dc3205197faac966deebe166dc30`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `custom_cursor_pass` | 12 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：use crate::typed_gap::Gap;
- L14：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/deferred_raster.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/deferred_raster.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/deferred_raster.rs)

SHA256：`ff99ce33966bf19414b6cd04b2cacffb852a0da0f5de455504a543ff75e05753`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `deferred_raster` | 18 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L12：use crate::typed_gap::Gap;
- L28：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_lists.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_lists.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_lists.rs)

SHA256：`d375b05a23cc7acef6fb6d59f1608b75b51aa5f888f4ca43832b5d9789b4c1be`；缺口注释标记数：9。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 21 | 未由扫描证明；以证据限定范围为准 |
| `new_main_with_velocity` | 25 | 未由扫描证明；以证据限定范围为准 |
| `new_main_pass` | 29 | 未由扫描证明；以证据限定范围为准 |
| `new_late_forward` | 33 | 未由扫描证明；以证据限定范围为准 |
| `new_velocity_only` | 37 | 未由扫描证明；以证据限定范围为准 |
| `default` | 43 | 未由扫描证明；以证据限定范围为准 |
| `new` | 54 | 未由扫描证明；以证据限定范围为准 |
| `sort` | 59 | 未由扫描证明；以证据限定范围为准 |
| `build_draw_lists` | 69 | 未由扫描证明；以证据限定范围为准 |
| `apply_topology` | 75 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L11：//! `DrawLists::sort` = `sort_by_key` (O). Field layouts U (typed gap).
- L14：use crate::typed_gap::Gap;
- L22：unimplemented!()
- L26：unimplemented!()
- L30：unimplemented!()
- L34：unimplemented!()
- L38：unimplemented!()
- L71：unimplemented!()
- L76：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs)

SHA256：`034871b41d03109d7e13ec5223924f2ddcf3422bb3910213f5740061a8ba4e79`；缺口注释标记数：5。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `build` | 35 | 未由扫描证明；以证据限定范围为准 |
| `startup_billboard_forest` | 46 | 未由扫描证明；以证据限定范围为准 |
| `closure_0` | 59 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L27：use crate::typed_gap::Gap;
- L37：unimplemented!()
- L45：/// 参数为 bevy 系统参数（O 候选接口）；本束以 typed gap 承载。
- L54：unimplemented!()
- L60：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs)

SHA256：`334cb9eaf474f8875835bcfded22863ae50970543ca4229adc64760b8fe36254`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `tree_trunk` | 45 | 未由扫描证明；以证据限定范围为准 |
| `tree_canopy` | 52 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L32：//! typed gap：各 TextureFromDisk 名字哈希对应的纹理资源名（除 water_noise_texture
- L41：use crate::typed_gap::Gap;
- L47：unimplemented!()
- L54：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/clouds.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/clouds.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/clouds.rs)

SHA256：`cd0838771eae137cf2f2eb98509261985136d3dabca85a325929986509ab0407`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `build` | 32 | 未由扫描证明；以证据限定范围为准 |
| `startup_clouds` | 40 | 未由扫描证明；以证据限定范围为准 |
| `closure_0` | 52 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L25：use crate::typed_gap::Gap;
- L33：unimplemented!()
- L48：unimplemented!()
- L53：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs)

SHA256：`605f1df9e52f33c19a72454e915de2523b5d1c49e872ae035a655731fc5504dc`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `bind_terrain_heightmap` | 26 | 未由扫描证明；以证据限定范围为准 |
| `bind_terrain_lighting` | 33 | 未由扫描证明；以证据限定范围为准 |
| `bind_water_object_push` | 43 | 未由扫描证明；以证据限定范围为准 |
| `bind_garden_exclusion` | 50 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L28：unimplemented!()
- L35：unimplemented!()
- L45：unimplemented!()
- L52：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs)

SHA256：`a9ee0e2ce4f84daf910235bb0da3e90836d19559de3726fec7d137df09e1f224`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `indirect_draw_bricks` | 42 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L27：//! typed gap：`0x8226cc786e679355`/`0x850dbd51eafef756` 资源 type-id 语义 U；
- L37：use crate::typed_gap::Gap;
- L50：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/floor_shader.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/floor_shader.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/floor_shader.rs)

SHA256：`413ea01de79fc9339d1fcb4a1a01130c2d194c5681a30e41720250a19dcdb9b7`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `draw_floor_shader` | 29 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L18：//! typed gap：preset 槽/磁盘纹理名语义 U（9/10/6/0x13/0x15/7/0xc/0xe/0x14 为
- L31：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs)

SHA256：`b1734b023800c89636fa55d8e20fd7203c53ddf2455682290ce2cf655ae8dacd`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `draw_common` | 27 | 未由扫描证明；以证据限定范围为准 |
| `draw_grass_shader` | 34 | 未由扫描证明；以证据限定范围为准 |
| `draw_grass_lod1_shader` | 44 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L16：//! typed gap：preset 9/10/8 槽语义 U（9/10=地面/水纹理族，I）。
- L29：unimplemented!()
- L39：unimplemented!()
- L49：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs)

SHA256：`f448355623920a3da5a1f8602ebe8a9b53b7789ea209c6e869fe46553e170df1`；缺口注释标记数：7。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `draw_instanced_butterfly_shader` | 43 | 未由扫描证明；以证据限定范围为准 |
| `draw_instanced_firefly_shader` | 52 | 未由扫描证明；以证据限定范围为准 |
| `draw_instanced_firefly_glow_shader` | 62 | 未由扫描证明；以证据限定范围为准 |
| `draw_duck_shader` | 75 | 未由扫描证明；以证据限定范围为准 |
| `draw_wren_shader` | 81 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L29：//! typed gap：`0xa2127765959c8b85` 资源语义 U；duck 三组件/Mat4 来源 U；
- L40：use crate::typed_gap::Gap;
- L48：unimplemented!()
- L58：unimplemented!()
- L70：unimplemented!()
- L77：unimplemented!()
- L83：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs)

SHA256：`4e4733f3a305580743d8c3901f381b5dbd9b8b1b3f020ab321ed36cd8b95c934`；缺口注释标记数：6。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `draw_instanced_ivy_branch_shader` | 40 | 未由扫描证明；以证据限定范围为准 |
| `draw_instanced_ivy_leaf_shader` | 49 | 未由扫描证明；以证据限定范围为准 |
| `draw_instanced_ivy_flower_shader` | 54 | 未由扫描证明；以证据限定范围为准 |
| `ivy_leaf_shader` | 63 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L27：//! typed gap：`0xdf356893cc4e98a8` 资源语义 U；FrozenMap 断言名 U；
- L36：use crate::typed_gap::Gap;
- L45：unimplemented!()
- L50：unimplemented!()
- L58：unimplemented!()
- L70：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/leaf_particle_draw.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/leaf_particle_draw.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/leaf_particle_draw.rs)

SHA256：`a76e9d11d32c4f34a6bd48ef5b88f8a727f8654a1ab723e0b1e29e5ded994927`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `leaf_particle_draw` | 24 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L13：//! b903…/f0ad…）。typed gap：`0x3dce36237fb50dff` 资源语义 U；两磁盘纹理名 U。
- L20：use crate::typed_gap::Gap;
- L30：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/mod.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/mod.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/mod.rs)

SHA256：`d05f71cedeaddc31e9b70902e6f21031f2e6aa5b08c266b5ca7373b249650fc1`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/path_pebble_shader.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/path_pebble_shader.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/path_pebble_shader.rs)

SHA256：`6b61e8ba70e86398d8427ec64b6695ea7d64a3e7d36f4d2120318f7d30cabe4e`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `path_pebble_draw` | 23 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L13：//! b903…/f0ad…）。typed gap：`0x84b51135b59095ba` 纹理名 U（O：与 floor 共用）。
- L25：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs)

SHA256：`e85fb40a7dc5d0e7b74b311cab04bbfcc96c6a7f99d363eb1c1431ef7c0fdf91`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `rocky_terrain_shader` | 27 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L16：//! typed gap：`0x84a037e697de0324` 纹理名 U；`0xfa44538b95ae76c6` 资源语义 U。
- L23：use crate::typed_gap::Gap;
- L34：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs)

SHA256：`c1d5041c132c811918ef1168ff0726c2af798287873726a04c9657f328daff72`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `stairs_draw` | 29 | 未由扫描证明；以证据限定范围为准 |
| `closure_0` | 39 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L18：//! typed gap：两 map 键/哨兵语义 U；`StairsTransformSsbo` 记录布局 U。
- L25：use crate::typed_gap::Gap;
- L35：unimplemented!()
- L40：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/stars.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/stars.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/stars.rs)

SHA256：`c32647fdb4537459feccdd54fae877c45492a656e5a8498d57bdf9a5172c95dc`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `build` | 30 | 未由扫描证明；以证据限定范围为准 |
| `startup_stars` | 36 | 未由扫描证明；以证据限定范围为准 |
| `closure_0` | 48 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L23：use crate::typed_gap::Gap;
- L31：unimplemented!()
- L44：unimplemented!()
- L49：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs)

SHA256：`562f542b3020fab631067aff9b2dea39f00d7dbd799a00cd119eeb8b070cce90`；缺口注释标记数：5。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `water_planes_draw` | 33 | 未由扫描证明；以证据限定范围为准 |
| `lilypad_draw` | 42 | 未由扫描证明；以证据限定范围为准 |
| `lilypad_flower_draw` | 47 | 未由扫描证明；以证据限定范围为准 |
| `reed_draw` | 52 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L28：use crate::typed_gap::Gap;
- L37：unimplemented!()
- L43：unimplemented!()
- L48：unimplemented!()
- L53：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/feedback_report_bg.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/feedback_report_bg.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/feedback_report_bg.rs)

SHA256：`6331a795f478a585d39b22208bc6e37bf48580ca1c149c59fdea69eb97fe9c16`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `render_feedback_report_bg` | 6 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：pub fn render_feedback_report_bg(world: &mut crate::typed_gap::Gap, _bg: &crate::typed_gap::Gap) {
- L8：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/final_post.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/final_post.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/final_post.rs)

SHA256：`7f49b1d1bc72d751c04c6879ef55fa73cf2c7899c5e7163a69a444eee1721fb9`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `final_post` | 18 | 未由扫描证明；以证据限定范围为准 |
| `resolve_to_swapchain` | 24 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L12：use crate::typed_gap::Gap;
- L20：unimplemented!()
- L26：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/foliage_exclusion_mask.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/foliage_exclusion_mask.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/foliage_exclusion_mask.rs)

SHA256：`b10f66f4ddbf815e327682477caf95c6e2d807b10421d3f4d81addb8853f6ff6`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `rasterize_foliage_exclusion_mask` | 6 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：pub fn rasterize_foliage_exclusion_mask(world: &mut crate::typed_gap::Gap, _foliage: &crate::typed_gap::Gap) {
- L8：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/logo_overlay.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/logo_overlay.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/logo_overlay.rs)

SHA256：`00405a112ad39fed6e43455c596d1559d04e6e4c7c3da93552f6bfec7b6f7ad9`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `render_logo_overlay` | 11 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：use crate::typed_gap::Gap;
- L13：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/main_pass.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/main_pass.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/main_pass.rs)

SHA256：`5a3e1aa802bb3d9de463ebeb475fe3761ad215cd1d0f61fc1ab2023d618a6fcc`；缺口注释标记数：5。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `with_stencil_write` | 16 | 未由扫描证明；以证据限定范围为准 |
| `main_pass` | 22 | 未由扫描证明；以证据限定范围为准 |
| `main_pass_oc2` | 28 | 未由扫描证明；以证据限定范围为准 |
| `late_forward_and_deferred_pass` | 34 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L13：use crate::typed_gap::Gap;
- L18：unimplemented!()
- L24：unimplemented!()
- L30：unimplemented!()
- L36：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/material_id.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/material_id.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/material_id.rs)

SHA256：`de7fbeb8b13396c3acf640ca3afb6cf22f7d9dc7ae591110a444513403ef5ae3`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|

缺口注释摘录（最多30条）：

- L3：use crate::typed_gap::Gap;
### reconstruction/country-core/src/systems/render_loop/draw_passes/mod.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/mod.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/mod.rs)

SHA256：`3eb9b9ab5b90f9fa77f3d7d92ab0622db627d4241f26419c567a51a061d88b00`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/country-core/src/systems/render_loop/draw_passes/object_grass_push.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/object_grass_push.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/object_grass_push.rs)

SHA256：`9e34117ae039bd070d62d2869147ee807ecb345b59ce1612b483618bfd85cbc0`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `object_grass_push_pass` | 12 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L10：use crate::typed_gap::Gap;
- L14：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/shadow_pass.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/shadow_pass.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/shadow_pass.rs)

SHA256：`651d17e5ccb49daa797d996f2b4aca62a02cb87b433db098cec78baf58237e61`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `shadow_pass` | 11 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：use crate::typed_gap::Gap;
- L13：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs)

SHA256：`9101f2f4ad6f4e564f57cf2e728f1213be4632300e138859e9703dca1ff49967`；缺口注释标记数：6。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `select_water_surface` | 35 | 未由扫描证明；以证据限定范围为准 |
| `refraction_copy_pass` | 45 | 未由扫描证明；以证据限定范围为准 |
| `water_surface_pass` | 54 | 未由扫描证明；以证据限定范围为准 |
| `ice_surface_pass` | 62 | 未由扫描证明；以证据限定范围为准 |
| `pre_taa_transparency_pass` | 68 | 未由扫描证明；以证据限定范围为准 |
| `post_taa_transparency_pass` | 74 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L24：use crate::typed_gap::Gap;
- L47：unimplemented!()
- L57：unimplemented!()
- L64：unimplemented!()
- L70：unimplemented!()
- L76：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs)

SHA256：`bd5c88c99690cbb98862a01da647a6a6b116b6894957c5886248b4ae219a6aaa`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `ui_2d_pass` | 15 | 未由扫描证明；以证据限定范围为准 |
| `mei_pass` | 20 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L10：use crate::typed_gap::Gap;
- L17：unimplemented!()
- L22：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/ui_pass.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/ui_pass.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/ui_pass.rs)

SHA256：`a0a9c87d00dc51fd7caaf99d9e3fafcafade95339d180bf8481b736665354a1f`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `ui_pass` | 7 | 未由扫描证明；以证据限定范围为准 |
| `loading_screen_pass` | 12 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：pub fn ui_pass(world: &mut crate::typed_gap::Gap, _lists: &crate::typed_gap::Gap) {
- L9：unimplemented!()
- L12：pub fn loading_screen_pass(world: &mut crate::typed_gap::Gap, _lists: &crate::typed_gap::Gap) {
- L14：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/draw_passes/wireframe_pass.rs

[reconstruction/country-core/src/systems/render_loop/draw_passes/wireframe_pass.rs](../../reconstruction/country-core/src/systems/render_loop/draw_passes/wireframe_pass.rs)

SHA256：`3bc3ffd04acf7744c6f2fac309fd1d30d57f6d8759f5db4fa49d3c46d64ad359`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `wireframe_pass` | 6 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：pub fn wireframe_pass(world: &mut crate::typed_gap::Gap, _lists: &crate::typed_gap::Gap) {
- L8：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/indirect_params.rs

[reconstruction/country-core/src/systems/render_loop/indirect_params.rs](../../reconstruction/country-core/src/systems/render_loop/indirect_params.rs)

SHA256：`9c206069aded959a753ae2ef85bbbd9609a0e3a8a86e1c12eae60178589b4d50`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|

缺口注释摘录（最多30条）：

- L10：use crate::typed_gap::Gap;
### reconstruction/country-core/src/systems/render_loop/lut_gallery.rs

[reconstruction/country-core/src/systems/render_loop/lut_gallery.rs](../../reconstruction/country-core/src/systems/render_loop/lut_gallery.rs)

SHA256：`513bb92b8bb3c01b03c05bf3034c86aecd0e48287aa9f1a805754d044cd91e7c`；缺口注释标记数：5。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `build` | 24 | 未由扫描证明；以证据限定范围为准 |
| `startup` | 34 | 未由扫描证明；以证据限定范围为准 |
| `render_photomode_lut_gallery` | 42 | 未由扫描证明；以证据限定范围为准 |
| `lut_cell_closure` | 61 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L14：use crate::typed_gap::Gap;
- L28：unimplemented!()
- L35：unimplemented!()
- L57：unimplemented!()
- L62：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/menu_background.rs

[reconstruction/country-core/src/systems/render_loop/menu_background.rs](../../reconstruction/country-core/src/systems/render_loop/menu_background.rs)

SHA256：`e6f3e2b6df4285f09ace98b6cbf7ce0ee7ed505fd1c1fc4142aa325ce377eb88`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `build` | 40 | 未由扫描证明；以证据限定范围为准 |
| `default` | 48 | 未由扫描证明；以证据限定范围为准 |
| `render_menu_bg` | 63 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L14：use crate::typed_gap::Gap;
- L30：/// 其余 typed gap。
- L42：unimplemented!()
- L70：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/mod.rs

[reconstruction/country-core/src/systems/render_loop/mod.rs](../../reconstruction/country-core/src/systems/render_loop/mod.rs)

SHA256：`4dd4d5f319f8ec4682172ad52f0c79ed67967e6e1b810b6158d58ef76cee4bd9`；缺口注释标记数：6。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `clear_buffer` | 36 | 未由扫描证明；以证据限定范围为准 |
| `clear_rg_buffer` | 43 | 未由扫描证明；以证据限定范围为准 |
| `clear_rg_color_image` | 48 | 未由扫描证明；以证据限定范围为准 |
| `rg_buffer_copy_from_slice` | 53 | 未由扫描证明；以证据限定范围为准 |
| `finish` | 58 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L29：use crate::typed_gap::Gap;
- L38：unimplemented!()
- L44：unimplemented!()
- L49：unimplemented!()
- L54：unimplemented!()
- L59：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/painterly.rs

[reconstruction/country-core/src/systems/render_loop/painterly.rs](../../reconstruction/country-core/src/systems/render_loop/painterly.rs)

SHA256：`7ef2272cda0017d73a07316aa9c452570b46b727a21081dfa677b88a3a8b935a`；缺口注释标记数：8。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `has_target` | 38 | 未由扫描证明；以证据限定范围为准 |
| `build` | 49 | 未由扫描证明；以证据限定范围为准 |
| `startup_painterly` | 56 | 未由扫描证明；以证据限定范围为准 |
| `painterly_pass` | 75 | 未由扫描证明；以证据限定范围为准 |
| `raster_strokes` | 81 | 未由扫描证明；以证据限定范围为准 |
| `painterly_dispatch_closure` | 86 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L17：use crate::typed_gap::Gap;
- L31：/// +0x18.. typed gap。
- L39：unimplemented!()
- L50：unimplemented!()
- L57：unimplemented!()
- L77：unimplemented!()
- L82：unimplemented!()
- L87：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/resources/critters.rs

[reconstruction/country-core/src/systems/render_loop/resources/critters.rs](../../reconstruction/country-core/src/systems/render_loop/resources/critters.rs)

SHA256：`7063539e342844ddaeffedecb8f9a77addd9eb11d52f5979d775d19c04db48be`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|

缺口注释摘录（最多30条）：

- L9：use crate::typed_gap::Gap;
### reconstruction/country-core/src/systems/render_loop/resources/mod.rs

[reconstruction/country-core/src/systems/render_loop/resources/mod.rs](../../reconstruction/country-core/src/systems/render_loop/resources/mod.rs)

SHA256：`269a23fffc8481a2cad42632a0e817a6fe9c33ab7f1d7118de5029291eb4f536`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/country-core/src/systems/render_loop/session_selection/mod.rs

[reconstruction/country-core/src/systems/render_loop/session_selection/mod.rs](../../reconstruction/country-core/src/systems/render_loop/session_selection/mod.rs)

SHA256：`bf8b7e36abc3489dbe33932c7f4f89105cd2efff57aaab949c1be7b4998ef971`；缺口注释标记数：8。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `build` | 35 | 未由扫描证明；以证据限定范围为准 |
| `thumbnails_startup` | 43 | 未由扫描证明；以证据限定范围为准 |
| `render_session_selection_previews` | 56 | 未由扫描证明；以证据限定范围为准 |
| `extract_thumbnails_to_display` | 70 | 未由扫描证明；以证据限定范围为准 |
| `find_backed_thumbnail_range` | 75 | 未由扫描证明；以证据限定范围为准 |
| `assign_pool_indices` | 80 | 未由扫描证明；以证据限定范围为准 |
| `refresh_screenshot` | 85 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L19：use crate::typed_gap::Gap;
- L36：unimplemented!()
- L51：unimplemented!()
- L66：unimplemented!()
- L71：unimplemented!()
- L76：unimplemented!()
- L81：unimplemented!()
- L86：unimplemented!()
### reconstruction/country-core/src/systems/render_loop/vermeer.rs

[reconstruction/country-core/src/systems/render_loop/vermeer.rs](../../reconstruction/country-core/src/systems/render_loop/vermeer.rs)

SHA256：`036530b49df54e927d2b2d41945f4e3a90bb1bb21f600e1fa6e4728e70c1f3aa`；缺口注释标记数：20。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `activate` | 57 | 未由扫描证明；以证据限定范围为准 |
| `deactivate` | 65 | 未由扫描证明；以证据限定范围为准 |
| `into_disable_hold` | 72 | 未由扫描证明；以证据限定范围为准 |
| `is_active` | 79 | 未由扫描证明；以证据限定范围为准 |
| `should_freeze_camera` | 85 | 未由扫描证明；以证据限定范围为准 |
| `source_hash` | 99 | 未由扫描证明；以证据限定范围为准 |
| `as_compute_pipeline` | 103 | 未由扫描证明；以证据限定范围为准 |
| `build` | 117 | 未由扫描证明；以证据限定范围为准 |
| `startup_vermeer` | 125 | 未由扫描证明；以证据限定范围为准 |
| `vermeer_pass` | 133 | 未由扫描证明；以证据限定范围为准 |
| `vermeer_render_from_target` | 148 | 未由扫描证明；以证据限定范围为准 |
| `max_stroke_count` | 173 | 未由扫描证明；以证据限定范围为准 |
| `raster_strokes` | 178 | 未由扫描证明；以证据限定范围为准 |
| `raster_combine` | 183 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L24：use crate::typed_gap::Gap;
- L26：/// Vermeer 揭示状态资源（O 名称）。字段偏移为机器码观测（O）；未观测区间 typed gap。
- L32：/// +0x08..+0x3d typed gap（`into_disable_hold` 复制 0x58B 含此区间）。
- L36：/// +0x3f typed gap。
- L50：/// +0x56..+0x57 typed gap。
- L61：unimplemented!()
- L67：unimplemented!()
- L74：unimplemented!()
- L80：unimplemented!()
- L86：unimplemented!()
- L95：/// 结构体字段 U（typed gap）。
- L101：unimplemented!()
- L105：unimplemented!()
- L118：unimplemented!()
- L126：unimplemented!()
- L143：unimplemented!()
- L168：unimplemented!()
- L174：unimplemented!()
- L179：unimplemented!()
- L184：unimplemented!()
