# WP5 契约 1/3 — render_loop 帧接线（frame wiring）

版本 **B**（`tiny-glade.exe` sha256 cf048e27…，PDB `tiny_glade.pdb` 2d9686b8…）。
本契约汇总 `tiny_glade::render_loop::{render, render_world}`（L0 帧编排，属 **tiny_glade** crate，WP5 只消费不实现）
与 `country_core::systems::render_loop`（WP5 实现）之间的接线。数据源：
`evidence/render-architecture/0x1404ec110.c`（render_world 反编译，本包复读确认）、
`0x1404e8010.c`（render）、`evidence/render-passes/cpu-frame/sequence.md`、`resolved-calls.json`、`public-symbols.json`（地址→PDB 名）。

标注：**O**=反编译/PDB 直接支持；**I**=调用链/名称组合；**U**=未恢复。

## 1. 帧外层 `render`（O，地址 0x1404e8010）

| 步 | 动作 | 归属 | 证据 |
|---|---|---|---|
| R1 | ECS 资源获取（`Components::get_resource_id` ×2）+ settings 模式字节 +0xac | bevy_ecs / tiny_glade | sequence.md |
| R2 | `DrawLists::new` → `build_draw_lists` → `DrawLists::sort` | **country_core::systems::render_loop::draw_passes::draw_lists**（0x1409ba340/540/4b0） | O |
| R3 | `Swapchain::format/extent` | rhapsody::raw::swapchain | 仅当 swapchain 存在 |
| R4 / R4' | `RenderGraphState::compile`：模式字节==0 → 0x1401647b0；!=0 → 0x140161cd0（含 `render_world` 闭包，:835）→ `ResourceInfo::calculate`（:1736） | rhapsody::rg::state | O |
| R5–R7 | `GraphExecutor::begin_frame` / profiler retrieve / `PassResources::new_from`（0x140909310） | rhapsody / country_core::render::pass_builder::pass_resources | O |
| R8 | `GraphExecutionRecorder::execute_compiled`（0x1400fa190）：BatchCompiler → flush_batch → submit_pending_cbs → 逐 pass record_batch → into_exports → ExecutedRenderGraph::new | rhapsody::rg::recorder | O |
| R9 | `ExecutedRenderGraph::retire` / `ExportedRenderGraphResources::take_resource` / `ImportExportToRenderGraph::from_owned_rg_resource` | rhapsody::rg::import_export | O |
| R10–R11 | `validate_swapchain_and_acquire_image`（acquire 在**主图 execute 之后**、最终图之前）；返回字节==7 → 本帧跳过 present | rhapsody::raw::swapchain | O |
| R12–R14 | 最终图 `execute`（acquired 图像；UI/后处理）→ `into_present_source` | rhapsody | O |
| R15–R16 | `end_frame` → `Swapchain::present` | rhapsody | O |
| R17–R20 | screenshot / `RenderStats::snapshot_and_reset` / `refresh_memory_budget` / `wait_gpu_queue_trim` | tiny_glade / rhapsody::raw | O |

## 2. 世界图 `render_world`（O，地址 0x1404ec110）——WP5 pass 的接线帧

参数 `(param_2=world, param_3=world?ecs, param_5=settings/params, param_6=config_bytes, param_7=view_handle[9], param_8=…handles)`。
判别式 `cStack_154 != '\x02'` 保护整段（==2 → unwrap_failed panic，I：错误标签）。

| W# | pass | PDB 名 / 地址 | 分支条件（O） | WP5 模块 |
|---|---|---|---|---|
| W1 | upload_world_raster ×6 | `upload_raster_staging_to_texture` 6 单态：0x1403d8bd0/0x1403da050/0x1403da570/0x1403d90f0/0x1403d9610/0x1403d9b30，参数 1/2/3/4/0x13/0x14 | 无 | compute_passes::upload_world_raster |
| W2 | water_rasterize_from_cpu | 0x140a12680 | 无 | compute_passes::compute_water |
| W3 | rasterize_terrain_stroke | 0x1408c7120 | 无 | compute_passes::rasterize_terrain_stroke |
| W4 | displace_rocky_terrain | 0x140a3ee80 | 无 | compute_passes::displace_rocky_terrain |
| W5 | read_terrain_data | 0x14098c680 | 无 | compute_passes::read_terrain_data |
| W6 | mouse_read_terrain_stroke_index | 0x1409768b0 | 无 | compute_passes::mouse_read_terrain_stroke_index |
| W7 | rasterize_foliage_exclusion_mask | 0x1409cb7b0 | 无 | draw_passes::foliage_exclusion_mask |
| W8 | highlight_active_terrain_stroke | 0x140a58450 | 无 | compute_passes::highlight_active_terrain_stroke |
| W9 | object_grass_push_pass | 0x1409daaa0 | 无 | draw_passes::object_grass_push |
| W10 | compute_path_masks | 0x1409014e0 | 无 | compute_passes::compute_path_masks |
| W11 | rasterize_buildings | 0x140a32770 | 无 | compute_passes::rasterize_buildings |
| W12 | garden_rasterize_from_cpu | 0x140a22140 | 无 | compute_passes::compute_garden |
| W13 | water_spawn_vegetation | 0x140a13970 | 无 | compute_passes::compute_water |
| W14 | spawn_vegetation_around_buildings | 0x1408e4cc0 | 无 | compute_passes::spawn_vegetation_around_buildings |
| W15 | read_bush_positions_for_cpu | 0x140866540 | 无 | compute_passes::read_bush_positions_for_cpu |
| W16 | garden_spawn_vegetation | 0x140a22b80 | 无 | compute_passes::compute_garden |
| W17 | path_pebbles_compute | 0x140958f70 | 无 | compute_passes::path_pebbles |
| W18 | leaf_particles_compute | 0x1408f4d00 | 无 | compute_passes::leaf_particles |
| W19 | roof_tiles_gravity_compute | 0x140a29b10 | 无 | compute_passes::roof_tiles_gravity |
| W20 | stairs | 0x140a68a50 | 无 | compute_passes::stairs |
| W21 | generate_wall_draw_lists | 0x14098aaf0 | 无 | compute_passes::gpu_generate_draw_lists |
| W22 | gpu_generate_draw_lists 第 1 轮 | 0x140987dd0，第三参 0 | 无 | compute_passes::gpu_generate_draw_lists |
| W23 | shadow_pass | 0x1409cc270 | 无 | draw_passes::shadow_pass |
| W24 | calc_terrain_lighting_texture | 0x140a51440 | 无 | compute_passes::terrain_lighting_texture |
| — | create_raw_resource ×2 | 0x140be94d0，名字 0x429ebfc8/0x429ebfcf（main 深度 + HZB 纹理） | 无 | rhapsody::rg::graph（消费） |
| W25 | main_pass | 0x140833bc0 | 无 | draw_passes::main_pass |
| W26 | generate_hzb | 0x1409286f0 | 无 | compute_passes::generate_hzb |
| W27 | generate_wall_draw_lists_pass2 | 0x14098b2a0 | **\*0x14362c0d8 == 0** | compute_passes::gpu_generate_draw_lists |
| W28 | gpu_generate_draw_lists 第 2 轮 | 0x140987dd0，第三参非 0 | 同上 | compute_passes::gpu_generate_draw_lists |
| W29 | grass_compute | 0x140a60280 | 无条件执行 | compute_passes::grass |
| W30 | main_pass_oc2 | 0x140835a40 | 无 | draw_passes::main_pass |
| W31 | run_visibility_queries | 0x140a2f2e0 | 无 | compute_passes::visibility_queries |
| W32 | calculate_dof_autofocus | 0x140a17880 | 无 | compute_passes::dof_autofocus |
| W33 | mouse_depth_compute | 0x140a01620 | 无 | compute_passes::mouse_depth_value |
| W34 | late_forward_and_deferred_pass | 0x1408373b0 | 无 | draw_passes::main_pass |
| W35 | extract_half_res_depth_normals | 0x1408f6180 | 无 | compute_passes::half_res |
| W36 | calculate_reprojection_map | 0x140a0d190 | 无 | compute_passes::reprojection |
| W37 | calculate_ssgi | 0x1408743b0 | 无 | compute_passes::ssgi |
| — | GI history temporal pair | 0x1404f7450 `get_or_create_temporal_pair`；extent 按 param_6[0xab] 字节 | 无条件（先建后跑 GI） | rhapsody（消费） |
| W38 | ray_trace_gi | 0x140a01cf0 | 无 | compute_passes::ray_trace_gi |
| W39 | eval_gi_screen_radiance | 0x140a44cd0 | 无 | compute_passes::eval_gi_screen_radiance |
| W40 | bin_lights | 0x1409e7770 | 无 | compute_passes::bin_lights |
| W41 | restir_sample_direct_lights | 0x140a6fec0 | 无 | compute_passes::restir_di |
| W42 | apply_screen_shadows | 0x140a39e70 | 无 | compute_passes::apply_screen_shadows |
| W43 | deferred_raster | 0x1409d20b0 | 无 | draw_passes::deferred_raster |
| W44 | refraction_copy_pass | 0x1409bf320 | **\*0x14362c098 == 0** | draw_passes::transparency_pass |
| W45 | water_surface_pass / ice_surface_pass | 0x1409bf800 / 0x1409c08d0 | is_winter（0x140ae8320）间接派发 | draw_passes::transparency_pass |
| W46 | pre_taa_transparency_pass | 0x1409c3a80 | 无 | draw_passes::transparency_pass |
| W47 | compute_shadow_mask_mean | 0x140a3a6f0 | 无 | compute_passes::apply_screen_shadows |
| W48 | apply_volumetrics | 0x1408a44c0 | 无 | compute_passes::apply_volumetrics |
| W49 | dof_prepare | 0x140a5a020 | 无 | compute_passes::dof |
| W50 | generate_mips | 0x1400fb890 | **param_6[0xa9 + (res_type==0x13?1:0)] != 0** | compute_passes::generate_mips |
| W51 | calc_dof | 0x140a5a570 | 同上 | compute_passes::dof |
| W52 | calculate_taa / calculate_tgsr | 0x140a3b2f0 / 0x140a3ba60 | **param_6[0xab]==0→TAA，!=0→TGSR** | compute_passes::taa |
| W53 | post_taa_transparency_pass | 0x1409c40b0 | 无 | draw_passes::transparency_pass |
| W54 | render_shader_refactoring | 0x14098f9f0 | callees 仅 RwLock+unwrap；作用 U | compute_passes::shader_refactoring |

render_world 返回结构（0x1404ec110.c:487-502）：把帧结果 handle（main/hzb/dof/reprojection 等）写回调用方 0xd qword + 4B 结构。该返回体是 WP5 各 pass 输出 handle 的汇聚。

## 3. 分支判定汇总（O，语义 U）

| 判定 | 位置 | 效果 |
|---|---|---|
| settings+0xac 模式字节 | render | 选 compile 实例 0x1401647b0（==0）↔ 0x140161cd0（!=0，含 render_world） |
| \*0x14362c0d8 == 0 | render_world W27/W28 | 第 2 轮 wall draw list + gpu_generate_draw_lists（第三参非 0） |
| \*0x14362c098 == 0 | render_world W44/W45 | refraction_copy_pass + 水/冰 surface |
| is_winter | render_world W45 | water_surface_pass ↔ ice_surface_pass |
| param_6[0xa9(+type)] != 0 | render_world W50/W51 | generate_mips + calc_dof |
| param_6[0xab] == 0/!=0 | render_world W52 + GI pair | TAA ↔ TGSR |
| acquire 返回字节==7 | render | 本帧不 present |

## 4. 生产/消费关系

- **生产者（WP5 输出 handle）**：main 深度、HZB 纹理（create_raw_resource 于 W25/W26 前）、main_pass 输出、DoF/TAA 帧结果（写入 render_world 返回体）。
- **消费者（WP5 读取的共享 handle）**：`frame_constants`（set1/b0）、`ColorLibrarySsbo`、`NaniMeshAtlasPartInfo`/`NaniDrawListInfo`/`NaniInstanceIndirectionSsbo`（WP3）、`DrawIndexedIndirectBuffer`/`DrawIndirectBuffer`/`DispatchIndirectBuffer`（WP2）、`rt_bl_bvh`/`rt_triangles`/`rt_triangle_meta`/`WallColorIdSsbo`（软件 BVH，WP4 bind_ray_tracing）、`MouseDepthValueSsbo`、`DofAutofocusSsbo`、`ShadowMaskHistogramSsbo`/`ShadowMaskMeanSsbo`、`LanternSsbo`/`SpatialResolveOffsetsSsbo`（deferred_raster）。
- **资源名→句柄**：`PassResources::new_from` 7 hash → 7 资源（wp1）；`AssetSsboLibrary::get_rg_buffer<T>`（wp4）。

## 5. typed gap（消费侧）

- 全局 \*0x14362c0d8 / \*0x14362c098 语义、param_6 字节语义、settings+0xac 模式字节含义：U。
- render_world 返回体逐字段映射：U（本包记录布局 O、语义 U）。
- W54 render_shader_refactoring 作用：U。
