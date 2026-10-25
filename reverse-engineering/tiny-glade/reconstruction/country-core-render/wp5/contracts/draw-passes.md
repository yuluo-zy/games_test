# WP5 契约 3/3 — draw_passes（CPU 侧帧 draw pass 组合）

版本 **B**。数据源：skeleton bundle（`draw_passes/*.rs.{pdb,structures,enriched,dependencies}`）、
`0x1404ec110.c`、`public-symbols.json`、`pass-declaration.md`（DrawBuilder/RasterPass API）、`shader-uri-list.json`。

标注：**O**=反编译/PDB 直接支持；**I**=调用链组合；**U**=未恢复。

通用 wiring：raster pass 以 `RasterPass::new`（`new_impl` 0x140867970，≤5 ColorAttachment + 深度标志）构造 RenderPass；
`DrawPacket::new`（0x140867850，默认 shader + 两个 hash map + blend 1.0）取 DrawBuilder；
`DrawListItem::to_draw_packet` 把 CPU draw-list 项转 packet；`CommonDrawParams::new`（common_draw_params.rs:26）绑定帧 uniforms；
`RasterPipelineOpts::set_*` 配置管线；`impl$2::read_texture/uniform` 绑定纹理/uniform；`PassBuilder::render` 注册。
draw 模式判别 1/3/4/5/7/8（DrawBuilder，wp1 §4）。

## 1. `draw_lists.rs`（R2：DrawLists::new/build/sort）

地址：`DrawLists::new` 0x1409ba340、`build_draw_lists` 0x1409ba540、`DrawLists::sort` 0x1409ba4b0。

| 类型/函数 | 行 | 语义 |
|---|---|---|
| `DrawLists` | — | CPU 绘制列表容器（O 名；字段 U） |
| `DrawData` | — | 单个 pass 的绘制数据（O 名；字段 U） |
| `DrawData::new_main_pass` | 189 | 主 pass 项构造（~71 个单态实例，按材质 shader 分派） |
| `DrawData::new_main_with_velocity` | 212 | 含速度的主 pass 项（5 实例） |
| `DrawData::new` | 181 | 通用项（3 实例，`impl$7::default`） |
| `DrawData::new_velocity_only` | 227 | 仅速度项（1 实例） |
| `DrawData::new_late_forward` | 204 | 延迟光照项（1 实例） |
| `build_draw_lists` | — | puffin scope + `World::query` 遍历实体 + 按材质/组件分派到 `DrawData::new*`；`puffin::clean_function_name` |
| `DrawLists::sort` | — | `sort_by_key` 排序（`impl$190::next`） |
| `apply_topology` | 1452 | `RasterPipelineOpts::set_topology` |

`DrawData::new*` 闭包族（closure$0..closure$60，O 行号区间 297–1313）：按材质族绑定
- `bind_common`（`CommonDrawParams::bind_common`：bind_shadowmap + `Transform::compute_matrix` uniform）
- `bind_terrain_heightmap`（impl$0，读地形高度）
- `bind_roof_color_customization` / `bind_window_color_customization_textures` / `bind_wall_color_customization` / `bind_brick_erase_highlight` / `bind_stone_floor_textures` / `bind_wooden_floor_textures`
- `DrawBuilder::draw_instanced` / `draw_indexed_indirect` / `draw_multi_instanced`
- `RasterPipelineOpts::set_cull_mode` / `set_blend_mode`
- `SessionMeta::is_halloween_any` / `ActiveSession::is_autumn` / `MainCamera::mode` 条件（I：节日/季节变体）
- 具体 draw shader 在 `draw_passes/draw_shaders/*`（billboard_forest/clearing_shaders/clouds/draw_bricks/floor_shader/grass_shader/instanced_critters/instanced_ivy/instanced_roof/leaf_particle_draw/path_pebble_shader/rocky_terrain_draw/stairs_draw/stars/water_planes_draw —— WP5 记录为命名边界，具体 body 属 draw_shaders 域）

## 2. `main_pass.rs`（W25/W30/W34）

| 函数 | 语义 |
|---|---|
| `with_stencil_write` | 22；`RasterPipelineOpts::set_stencil_test_enable` + `set_stencil_pass_op`（主 pass 写 stencil） |
| `main_pass`（W25，0x140833bc0） | puffin scope + `RasterPass::new` + `CommonDrawParams::new` + 遍历 `DrawListItem` + `to_draw_packet` + `DrawBuilder::stencil_ref` + `with_stencil_write` |
| `main_pass_oc2`（W30，0x140835a40） | 同 main_pass（HZB 后二次遮挡判定，无 `ShaderRefactoringState`） |
| `late_forward_and_deferred_pass`（W34，0x1408373b0） | `impl$0::is_empty` 守卫 + 同上（nani_* + deferred 光照组合） |

Shader URI：nani_* raster（uber/albedo_map/vertex_color/velocity_only…，`/nani_*.raster.hlsl`）+ `/deferred/*_light_sun_sky.raster`（W34）。
资源：main 深度（写）、反照率/速度（写）、`frame_constants`（读）。

## 3. `shadow_pass.rs`（W23）

`shadow_pass`（0x1409cc270）：puffin scope + `utils::get_res_mut` + `RasterPass::new` + `DepthAttachment::layer`（shadow atlas 层）+ `CommonDrawParams::new` + 遍历 DrawListItem + `to_draw_packet` + `RasterPipelineOpts::set_depth_compare_mode`。
Shader URI：`_mei_shadow.raster.*`（I：无同名文件，阴影图光栅）。`bind_common`+render（sequence.md W23）。

## 4. `transparency_pass.rs`（W44/W45/W46/W53）

| 函数 | 语义 |
|---|---|
| `refraction_copy_pass`（W44，0x1409bf320） | `RasterPass::new` + `CommonDrawParams::new` + 遍历 + `to_draw_packet` + `RasterPipelineOpts::set_depth_write`；条件 `\*0x14362c098==0` |
| `water_surface_pass`（W45 非冬季，0x1409bf800） | `RenderGraphRecorder::create` + `Handle::desc` + `Iterator::product`（extent）+ raster：`RasterPass::new`+`to_draw_packet`+`set_blend_mode`；compute：`bind_terrain_heightmap`+`bind_terrain_lighting`+`ComputePass::write_texture`+`dispatch_1d`；`dispatch_indirect_raw`+`bind_ray_tracing`（软件 BVH） |
| `ice_surface_pass`（W45 冬季，0x1409c08d0） | 同水 + `bind_shadowmap` + `Temporal::into_inner` + `read_texture_view` + `dispatch_2d`；`_water_ice_{deferred_rt,refl_march,refl_resolve,refr_resolve}.cs`+`_ice_plane`+`_water_ice_cracks.raster` |
| `pre_taa_transparency_pass`（W46，0x1409c3a80） | `RasterPass::new` + `to_draw_packet` + `set_depth_write` + `set_blend_mode`（透明 nani raster：window_glass/cloth…） |
| `post_taa_transparency_pass`（W53，0x1409c40b0） | `Vec::is_empty` 守卫 + 同上 |

Shader URI（O）：水：`/water/{deferred_rt,calc_deferred_rt_args}.cs.hlsl` + `_water_plane.raster`；冰：`/water/ice_{deferred_rt,refl_march,refl_resolve,refr_resolve}.cs.hlsl` + `_ice_plane` + `/water/ice_cracks.raster.hlsl`；折射：`_water_plane_refraction_copy`/`_waterfall_refraction_copy`。
资源：`rt_bl_bvh`/`rt_triangles`/`rt_triangle_meta`/`WallColorIdSsbo`（bind_ray_tracing）、`frame_constants`、透明颜色（写）。

## 5. `deferred_raster.rs`（W43）

`deferred_raster`（0x1409d20b0）：`Duration::as_secs_f32`（时间 uniform）+ `RasterPass::new` + `AssetSsboLibrary::get` + `InstanceBuffer::instance_count` + `DrawPacket::new` + `stencil_ref` + `set_blend_mode`/`set_depth_write`/`set_depth_test_enable`/`set_stencil_test_enable`/`set_stencil_compare_op`/`set_stencil_compare_mask`；`bind_shadowmap` + `bind_terrain_heightmap`。
Shader URI：`/deferred/{solid,grass,plant,canopy}_light_sun_sky.raster.hlsl`。draw_no_vao + bind_color_library + bind_ray_tracing（O，sequence.md）。读 `LanternSsbo`/`SpatialResolveOffsetsSsbo`、`ssgi_tex`/`screen_shadows_tex`/`rdi_reservoir_tex`/`light_indirection_grid`/`grid_light_indices`。
结构体：`DeferredRasterResources`（U 字段）。

## 6. `final_post.rs`（最终图）

| 函数 | 语义 |
|---|---|
| `final_post` | `ImageFormationConfigs::current` + `ColorGradingState::flattened` + `RenderGraphRecorder::create`；闭包绑定 LUT（`GradingLut::lut_name_to_asset_name`）+ read_texture + uniform |
| `resolve_to_swapchain` | `Handle::desc` 对比 + `get_resource`；`_blit_resolve_to_swapchain`/`_blit_resolve_msaa_to_swapchain`（MSAA 档）/`_blit_blit_stretch_decode_srgb` |
| `closure$0/closure$1` | 120/244；read_texture + uniform |

Shader URI：`_final_post.raster`（色调映射/grading_lut/icc_lut/tony_mc_mapface_lut/shadow_mask_mean）。最终图属 tiny_glade::render_loop（R12–R14），country-core 仅提供 final_post 组装（O 名）。

## 7. `ui_2d_pass.rs`（最终图 UI）

`ui_2d_pass`：`WindowSize::as_vec2` + `World::query`（UI 实体）+ `sort_by` + `RasterPass::new` + `DrawPacket::new` + uniform + `read_texture_view`/`read_texture` + `set_depth_test_enable`/`set_blend_mode`。
`mei_pass`：`Handle::desc` + `RenderGraphRecorder::create` + 同上。
结构体：`ShapeStreamSsbo`、`TextPrimSsbo`（U）。
Shader URI：`/ui_2d.raster.hlsl`、`/egui.raster.hlsl`、`/text.raster.hlsl`、`/custom_cursor.raster.hlsl`（最终图 execute）。

## 8. `object_grass_push.rs`（W9）

`object_grass_push_pass`（0x1409daaa0）：puffin scope + `utils::get_res`/`get_res_mut` + `MainCamera::eye_position` + `CursorTerrainRaycast::position`（closure$1 把 eye/cursor → `Vec3::extend` Vec4） + `World::query` + `Iterator::chain` + `RasterPass::new` + `DrawPacket::new` + uniform + `set_depth_test_enable`/`set_blend_mode`。
Shader URI：`_grass_object_push.raster`（O，sequence.md）。

## 9. `common_draw_params.rs`（共享绑定）

| 函数 | 语义 |
|---|---|
| `CommonDrawParams::new` | 26；`utils::get_res` + `Duration::as_secs_f32`（帧时间 uniform） |
| `bind_common` | `DrawBuilder::bind_dummy_shadowmap` + `bind_shadowmap` + `Transform::compute_matrix` + uniform（全部 draw pass 共用） |
| `bind_roof_color_customization` | 84；read_texture（屋顶色） |
| `bind_wall_color_customization` | read_texture（墙色） |
| `bind_brick_erase_highlight` | `CursorTerrainRaycast::position` + uniform |
| `bind_stone_floor_textures` / `bind_wooden_floor_textures` | read_texture |
| `bind_window_color_customization_textures` | 89；read_texture |

## 10. 其余 draw_passes（UI/调试族，O 名）

| 模块 | 函数 | 语义 |
|---|---|---|
| `wireframe_pass.rs` | `wireframe_pass` | `RasterPass::new` + `CommonDrawParams::new` + `set_polygon_mode`/`set_depth_test_enable` |
| `material_id.rs` | — | `MaterialId` 类型（fmt） |
| `ui_pass.rs` | `ui_pass` / `loading_screen_pass` | `RasterPass::new` + `to_draw_packet` + `set_blend_mode` |
| `logo_overlay.rs` | `render_logo_overlay` | `RasterPass::new` + `DrawPacket::new` + read_texture + set_depth_test_enable/blend；`LogoOverlayPlugin` |
| `custom_cursor_pass.rs` | `custom_cursor_pass` | `WindowSize::as_vec2` + `RasterPass::new` + `DrawPacket::new` + uniform + read_texture；`CustomCursorSsbo` |
| `foliage_exclusion_mask.rs` | `rasterize_foliage_exclusion_mask`（W7） | `RasterPass::new` + `DrawPacket::new` + `set_cull_mode` + `bind_terrain_heightmap` |
| `feedback_report_bg.rs` | `render_feedback_report_bg` | `RasterPass::new` + `DrawPacket::new` + read_texture + set_depth_test_enable |

## 11. typed gap（draw 侧）

- `DrawLists`/`DrawData` 字段布局、`DrawListItem` 定义（wp1 draw_list.rs 边界）、draw_shaders 各 body：U。
- `DrawData::new*` 71 个单态实例的材质→shader 选择表：I/U（draw_shaders 域）。
- `CommonDrawParams` 结构体字段：U（new 绑定时间 uniform O）。
- `DeferredRasterResources`、`FinalPostResources`、`ShapeStreamSsbo`/`TextPrimSsbo` 字段：U。
- 每个 pass 绑定的确切 shader 变体（两 16-hex 变体对）：U（shader-variants 域）。
