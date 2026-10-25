# rg 第三批工作包 4：59 个 pass 调用点的 RenderPassApi / 设备命令清单

方法：`/tmp/w3_*.py`（objdump 反汇编 `tiny-glade.rne`，只读）。原版 pass 的回调体**不是**每个 pass 一个函数：
所有 pass 都通过 `PassBuilder::render<PassResources, closure>`（`rg::pass_builder`）把闭包装箱，闭包类型只有下面 6 种共享回调体，
各 pass 的差别在闭包环境（绑定、pipeline、参数），不在命令序列。所以"某 pass 调用了哪些 RenderPassApi 方法"等价于
"该 pass 的构图函数（含 `country_core::systems::*` / `country_core::render::*` 辅助，深度≤5）装箱了哪几种共享回调体"。

## 共享回调体（逐体反汇编，vtable 第 4 槽 → `recorder::render` → `Box::call_once` → 闭包）

槽偏移以 `GraphicsDevice` 指针为基址，= `0x78 + 8×ash::DeviceFnV1_*字段序号`（`tests/rg_wave3.rs` 用 `offset_of!(ash::DeviceFnV1_0, ..)` 验证，全部相等）。

| 回调体 | 装箱点（PassBuilder::render 实例） | 体 | RenderPassApi 方法（有符号） | 内联进体的设备命令槽 | 其他设备调用 |
|---|---|---|---|---|---|
| **D 计算分发** `compute_pass_builder::dispatch_impl::closure_env$0` | 0x1409dd4d0 | vtable 0x142ac3b48 → 0x1415fc050 → 0x14169af70 | `push_constants`(0x1416f7a50, 槽 0x410) | `cmd_dispatch` 0x360、`cmd_dispatch_indirect` 0x368（= `dispatch_groups_3d`/`dispatch_threads_3d`/`dispatch_indirect`，原版无独立符号，内联） | `GraphicsDevice::bind_pipeline`(0x140bee130)、`bind_descriptor_set`×2 实例(0x1416a6010/0x1416a5380；`update_descriptor_sets` 0x268 + `cmd_bind_descriptor_sets` 0x328)、`issue_full_pipeline_barrier`(0x140bee090，`cmd_pipeline_barrier` 0x3e0)、`[vtable+0x18]` 一次动态闭包调用（用户绑定回调） |
| **R 光栅绘制** `draw_stream::RasterPass::render::closure_env$0` | 0x1409dcd10（及 0x140868370 `RasterPass::render` 内） | vtable 0x142ac3a90 → 0x140b649e0 → 0x141697ab0 | `begin_render_pass`(0x1416f6e90, 0x418)、`set_default_view_and_scissor`(0x1416f7870, 0x2e0+0x2e8)、`set_stencil_ref`(0x1416f7920, 0x320)、`set_line_width`(0x1416f7960, 0x2f0，受 `[device+0xad9]` 门控)、`bind_vertex_buffer_by_binding_point`(0x1416f7980, 0x338)、`bind_index_buffer`(0x1416f79f0, 0x330)、`push_constants`(0x410)、`end_render_pass`(0x1416f7850, 0x428) | `cmd_draw` 0x340、`cmd_draw_indexed` 0x348、`cmd_draw_indirect` 0x350、`cmd_draw_indexed_indirect` 0x358、`cmd_draw_indirect_count` 0x4b8、`cmd_draw_indexed_indirect_count` 0x4c0（按 0x80 字节绘制记录的种类分派，种类由 pass 的 `DrawBuilder::draw*` 变体决定） | `bind_pipeline`、`bind_descriptor_set`×3 实例（0x1416a5910/0x1416a5380 + `bind_persistent_descriptor_set` 0x140bee150）、`issue_full_pipeline_barrier`、`begin_label`/`end_label`（0x898/0x8a0）、首次创建 raster pipeline / framebuffer（`create_raster_pipeline`、`create_framebuffer` 0x270） |
| **F 清缓冲** `clear_rg_buffer::closure_env$0` | 内联于 `clear_rg_buffer`(0x1409ddbb0)；体 0x140904990 | — | `fill_buffer`(0x1416f7c30, `cmd_fill_buffer` 0x3a0) | — | — |
| **C 清颜色图像** `clear_rg_color_image::closure_env$0` | `clear_rg_color_image`(0x1409dde90)；体 0x140904a30 | — | `clear_color_image`(0x1416f7d80, `cmd_clear_color_image` 0x3a8) | — | — |
| **P 缓冲拷贝** `rg_buffer_copy_from_slice<u32>::closure_env$0` | 0x1409dd240；体 0x140904ac0 → 0x14169a650 | — | `copy_buffer`(0x1416f7ac0, `cmd_copy_buffer` 0x370) | — | — |
| **Z 重置间接参数** `reset_indirect_args::closure_env$0` | 0x1409dcfd0；体 0x140a880a0 → 0x14169a830 | — | `copy_buffer`(0x370) | — | — |

结论：`RenderPassApi` 在原版里**只有 11 个命令类有符号函数**（begin_render_pass、end_render_pass、set_default_view_and_scissor、set_stencil_ref、set_line_width、bind_vertex_buffer_by_binding_point、bind_index_buffer、push_constants、copy_buffer、fill_buffer、clear_color_image；PDB 里 0x1416f6e90..0x1416f7d80 区间）；draw*/dispatch* 没有独立符号，是内联。`pass_api.rs` 已有全部这些方法，**没有缺方法**；
`draw_call_count`/`begin_attachments`（候选扩展）原版没有，保留。新增 `pass_api::original_slots`（槽常量 + `slot(&Command)`）并带测试。

## 59 个调用点

列说明：闭包种类 = 上面 D/R/F/C/P/Z；"构图调用"= 静态直接调用图（深度≤5，只数在 pass 函数及其 `country_core::systems/render` 辅助内的 `E8` 调用）里出现的装箱/分发入口计数。
`dispatch_*` 计数是 `ComputePass::dispatch_2d/dispatch_with/dispatch_indirect_raw/dispatch_indirect_with/dispatch_groups`（它们的体各自再装箱一个 D）。
绘制种类（R 内部分派）无法按 pass 精确到某个 `vkCmdDraw*`，因为它由运行时的绘制记录种类决定，只列 `DrawBuilder::draw*` 变体名。

| # | pass | 地址 | 闭包种类 | 构图调用（计数） | 推导的 RenderPassApi 方法 / 设备命令 |
|---|---|---|---|---|---|
| 1 | upload_world_raster#0 | 0x1403d8bd0 | D | dispatch_2d×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 2 | upload_world_raster#1 | 0x1403da050 | D | dispatch_2d×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 3 | upload_world_raster#2 | 0x1403da570 | D | dispatch_2d×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 4 | upload_world_raster#3 | 0x1403d90f0 | D | dispatch_2d×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 5 | upload_world_raster#4 | 0x1403d9610 | D | dispatch_2d×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 6 | upload_world_raster#5 | 0x1403d9b30 | D | dispatch_2d×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 7 | water_rasterize_from_cpu | 0x140a12680 | D | dispatch_closure×4 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 8 | rasterize_terrain_stroke | 0x1408c7120 | D | dispatch_with×1, dispatch_closure×3 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 9 | displace_rocky_terrain | 0x140a3ee80 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 10 | read_terrain_data | 0x14098c680 | DR | draw×1, draw_no_index_buffer×1, RasterPass::render×1, dispatch_closure×4 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) / R: begin_render_pass, set_default_view_and_scissor, set_stencil_ref, set_line_width, bind_vertex_buffer_by_binding_point, bind_index_buffer, push_constants, draw*(0x340..0x358,0x4b8,0x4c0), end_render_pass |
| 11 | mouse_read_terrain_stroke_index | 0x1409768b0 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 12 | rasterize_foliage_exclusion_mask | 0x1409cb7b0 | R | RasterPass::render×2, draw_no_vao×1 | R: begin_render_pass, set_default_view_and_scissor, set_stencil_ref, set_line_width, bind_vertex_buffer_by_binding_point, bind_index_buffer, push_constants, draw*(0x340..0x358,0x4b8,0x4c0), end_render_pass |
| 13 | highlight_active_terrain_stroke | 0x140a58450 | D | dispatch_closure×3 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 14 | object_grass_push_pass | 0x1409daaa0 | R | draw_no_vao×1, RasterPass::render×1 | R: begin_render_pass, set_default_view_and_scissor, set_stencil_ref, set_line_width, bind_vertex_buffer_by_binding_point, bind_index_buffer, push_constants, draw*(0x340..0x358,0x4b8,0x4c0), end_render_pass |
| 15 | compute_path_masks | 0x1409014e0 | D | dispatch_closure×7 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 16 | rasterize_buildings | 0x140a32770 | D | dispatch_closure×4 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 17 | garden_rasterize_from_cpu | 0x140a22140 | D | dispatch_closure×2 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 18 | water_spawn_vegetation | 0x140a13970 | D | dispatch_closure×2 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 19 | spawn_vegetation_around_buildings | 0x1408e4cc0 | D | dispatch_closure×2 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 20 | read_bush_positions_for_cpu | 0x140866540 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 21 | garden_spawn_vegetation | 0x140a22b80 | D | dispatch_closure×4 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 22 | path_pebbles_compute | 0x140958f70 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 23 | leaf_particles_compute | 0x1408f4d00 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 24 | roof_tiles_gravity_compute | 0x140a29b10 | D | dispatch_with×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 25 | stairs | 0x140a68a50 | D | dispatch_closure×2 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 26 | generate_wall_draw_lists | 0x14098aaf0 | DF | clear_rg_buffer×1, dispatch_with×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) / F: fill_buffer |
| 27 | gpu_generate_draw_lists#1 | 0x140987dd0 | DF | dispatch_closure×10, clear_rg_buffer×2, dispatch_with×5, dispatch_indirect_raw×2, dispatch_indirect_with×7 | D: push_constants, dispatch_groups_3d/threads_3d(0x360); dispatch_indirect(0x368) / F: fill_buffer |
| 28 | shadow_pass | 0x1409cc270 | R | RasterPass::render×1 | R: begin_render_pass, set_default_view_and_scissor, set_stencil_ref, set_line_width, bind_vertex_buffer_by_binding_point, bind_index_buffer, push_constants, draw*(0x340..0x358,0x4b8,0x4c0), end_render_pass |
| 29 | calc_terrain_lighting_texture | 0x140a51440 | D | dispatch_closure×2 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 30 | main_pass | 0x140833bc0 | R | RasterPass::render×6 | R: begin_render_pass, set_default_view_and_scissor, set_stencil_ref, set_line_width, bind_vertex_buffer_by_binding_point, bind_index_buffer, push_constants, draw*(0x340..0x358,0x4b8,0x4c0), end_render_pass |
| 31 | generate_hzb | 0x1409286f0 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 32 | generate_wall_draw_lists_pass2 | 0x14098b2a0 | DF | dispatch_closure×1, clear_rg_buffer×1, dispatch_with×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) / F: fill_buffer |
| 33 | gpu_generate_draw_lists#2 | 0x140987dd0 | DF | dispatch_closure×10, clear_rg_buffer×2, dispatch_with×5, dispatch_indirect_raw×2, dispatch_indirect_with×7 | D: push_constants, dispatch_groups_3d/threads_3d(0x360); dispatch_indirect(0x368) / F: fill_buffer |
| 34 | grass_compute | 0x140a60280 | DF | dispatch_closure×9, clear_rg_buffer×1, dispatch_indirect_raw×2 | D: push_constants, dispatch_groups_3d/threads_3d(0x360); dispatch_indirect(0x368) / F: fill_buffer |
| 35 | main_pass_oc2 | 0x140835a40 | R | RasterPass::render×5 | R: begin_render_pass, set_default_view_and_scissor, set_stencil_ref, set_line_width, bind_vertex_buffer_by_binding_point, bind_index_buffer, push_constants, draw*(0x340..0x358,0x4b8,0x4c0), end_render_pass |
| 36 | run_visibility_queries | 0x140a2f2e0 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 37 | calculate_dof_autofocus | 0x140a17880 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 38 | mouse_depth_compute | 0x140a01620 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 39 | late_forward_and_deferred_pass | 0x1408373b0 | R | RasterPass::render×2 | R: begin_render_pass, set_default_view_and_scissor, set_stencil_ref, set_line_width, bind_vertex_buffer_by_binding_point, bind_index_buffer, push_constants, draw*(0x340..0x358,0x4b8,0x4c0), end_render_pass |
| 40 | extract_half_res_depth_normals | 0x1408f6180 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 41 | calculate_reprojection_map | 0x140a0d190 | D | dispatch_closure×2 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 42 | calculate_ssgi | 0x1408743b0 | D | dispatch_closure×3 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 43 | ray_trace_gi | 0x140a01cf0 | D | dispatch_closure×2 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 44 | eval_gi_screen_radiance | 0x140a44cd0 | D | dispatch_closure×3 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 45 | bin_lights | 0x1409e7770 | DF | clear_rg_buffer×1, dispatch_closure×5 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) / F: fill_buffer |
| 46 | restir_sample_direct_lights | 0x140a6fec0 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 47 | apply_screen_shadows | 0x140a39e70 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 48 | deferred_raster | 0x1409d20b0 | R | draw_no_vao×1, RasterPass::render×1 | R: begin_render_pass, set_default_view_and_scissor, set_stencil_ref, set_line_width, bind_vertex_buffer_by_binding_point, bind_index_buffer, push_constants, draw*(0x340..0x358,0x4b8,0x4c0), end_render_pass |
| 49 | refraction_copy_pass | 0x1409bf320 | R | RasterPass::render×1 | R: begin_render_pass, set_default_view_and_scissor, set_stencil_ref, set_line_width, bind_vertex_buffer_by_binding_point, bind_index_buffer, push_constants, draw*(0x340..0x358,0x4b8,0x4c0), end_render_pass |
| 50 | water_surface_pass OR ice_surface_pass | 0x1409bf800 / 0x1409c08d0 | DRFCP | clear_rg_buffer×1, RasterPass::render×2, dispatch_closure×3, dispatch_indirect_raw×3, rg_buffer_copy_from_slice×2, clear_rg_color_image×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360); dispatch_indirect(0x368) / R: begin_render_pass, set_default_view_and_scissor, set_stencil_ref, set_line_width, bind_vertex_buffer_by_binding_point, bind_index_buffer, push_constants, draw*(0x340..0x358,0x4b8,0x4c0), end_render_pass / F: fill_buffer / C: clear_color_image / P: copy_buffer |
| 51 | pre_taa_transparency_pass | 0x1409c3a80 | R | RasterPass::render×1 | R: begin_render_pass, set_default_view_and_scissor, set_stencil_ref, set_line_width, bind_vertex_buffer_by_binding_point, bind_index_buffer, push_constants, draw*(0x340..0x358,0x4b8,0x4c0), end_render_pass |
| 52 | compute_shadow_mask_mean | 0x140a3a6f0 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 53 | apply_volumetrics | 0x1408a44c0 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 54 | dof_prepare | 0x140a5a020 | D | dispatch_closure×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 55 | generate_mips | 0x1400fb890 | D | dispatch_groups×1 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 56 | calc_dof | 0x140a5a570 | D | dispatch_closure×3, dispatch_indirect_raw×5 | D: push_constants, dispatch_groups_3d/threads_3d(0x360); dispatch_indirect(0x368) |
| 57 | calculate_taa OR calculate_tgsr | 0x140a3b2f0 / 0x140a3ba60 | D | dispatch_closure×4 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |
| 58 | post_taa_transparency_pass | 0x1409c40b0 | R | RasterPass::render×1 | R: begin_render_pass, set_default_view_and_scissor, set_stencil_ref, set_line_width, bind_vertex_buffer_by_binding_point, bind_index_buffer, push_constants, draw*(0x340..0x358,0x4b8,0x4c0), end_render_pass |
| 59 | render_shader_refactoring | 0x14098f9f0 | D | dispatch_closure×3 | D: push_constants, dispatch_groups_3d/threads_3d(0x360) |

## 读不出 / 需要留意

* 静态直接调用图，**不含尾调用 `jmp`、动态分派（`dyn FnOnce`）里再装箱的闭包**；有 `—` 的 pass 不代表没有命令，只代表没找到装箱点。
* `D` 里 `cmd_dispatch` 与 `cmd_dispatch_indirect` 都在同一个体内，按闭包环境里的参数选一条；出现 `dispatch_indirect_*` 的 pass 才标注 `dispatch_indirect`。
* R 的绘制种类（`cmd_draw`/`draw_indexed`/`indirect`/`indirect_count`）由绘制记录种类分派，没有逐 pass 逐记录核对。
* `generate_mips`(0x1400fb890) 在 rg 域内（`rg::generate_mips`），只数到 `dispatch_groups` 一个入口。
* 加载图/展示图闭包内的入口（`ui_pass`、`final_post` 等）本表不含。
