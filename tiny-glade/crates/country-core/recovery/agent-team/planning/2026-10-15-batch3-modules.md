# 第三批模块分派建议(2026-10-15,coordinator)— 完整 country-core 收尾

> 背景:本批 9 优先级模块已 review_passed(限定),卡1-7(验收层面开放项)推进中。本文件为**第三批**
> 规划:盘点 inventory 中 9 模块+卡1-7 覆盖之外的原文件,按 README 依赖顺序(order 2-8)分组,
> 每模块/卡片给 组/角色→依赖→验收门槛;标注宿主接线项(需主会话确权)。
> 数据来源:inventory.json(539 条目)+ 当前 src 逐文件在位比对 + contracts.json(0/25 大模块无合同)。

## 盘点事实

- 9 优先级 + 卡1-7 之外,仍有 **258 个原文件无正式文件在位**(全部无 contract,未核验)。
- 大块(≥20 索引函数、完全缺失、按函数数降序):collision/world.rs(170)、walls/decorator.rs(131)、
  walls/decorator_storage.rs(114)、history/wall_compound_edit.rs(79)、walls/decorator_subtype.rs(49)、
  history/color_edit.rs(48)、audio_resources.rs(47)、history/decorator_edit.rs(45)、
  wall/valid_enclosure(45)、camera_rig.rs(44)、ui/widget_params.rs(35)、walls/material_stats.rs(34)、
  rt/mod.rs(31)、paths/draw_path.rs(27)、history/tree_edit.rs(25)、terrain_edit.rs(24)、
  steam/mod.rs(23)、roof_edit.rs(22)、walls/wall_attachment.rs(22)、steam.rs(22)、
  main_camera.rs(22)、ui/signifiers/mod.rs(22)、wall/eraser/archivist.rs(22)、clutter_meshes.rs(21)、
  collision/intersect_meshes.rs(21)。
- 既有实现但无 contract(需评审闭合):world_raster.rs/systems/world_raster.rs(状态机)、
  camera_rig.rs/main_camera.rs(present_requires_semantic_review,部分实现)、startup/ 8 文件、
  render/render_loop 34 文件。

## 第三批卡片(按 README 依赖顺序)

### 卡 A walls/roofs/stairs/collision 收尾(order 2-3)— 最大块,拆 5 子卡
- **A1 walls/decorator 族**(interaction/restorer,或拆 eco):decorator.rs(131)、decorator_storage.rs(114,
  衔接卡5 DecoratorStorage 实体)、decorator_subtype.rs(49)、material_stats.rs(34)、
  wall_attachment.rs(22)、cached_decorator_transforms.rs(11)、decorator_collision/outline/dormers/
  plaster/curved_halftimber/wooden_gates/wall_stone_columns/arches/in_progress_wall/elevation_support_stats。
  依赖:PublicWalls/InnerWalls(已具备编辑/发布/回放链)。验收:装饰器实体/存储/材质统计/附着真实闭合;
  卡5 开放项(DecoratorStorage 实体机制)并入此卡统一验收。
- **A2 walls systems 收尾**(interaction/restorer):valid_enclosure/mod.rs(45)、wall_splitter、merge_walls、
  finish_wall、draw_wall、ui_continue_wall、ui_draw_wall、structure_outline(17)、set_wall_style、
  deferred_wall_history(5)、upload_instanced_wall_ssbo(1)、wall_state/{spatial_hash_update,wall_roof_hashmap,
  debug_viz_threshold_curve}、adjust_wall_height 族、shapes 族(calc_shape_height/circle/rectangle/shape_placement)、
  eraser 族(wall_eraser 10/archivist 22/ui_bulldozer 14/animate_wall_bulldoze 7)、snapping 族(9 文件)。
  依赖:PublicWalls/墙历史/AppMode。验收:墙编辑/形状/吸附/推土全链闭合。
- **A3 roofs 收尾**(interaction/restorer):roof_noise(8)、roof_ridge(8)、roof_tiles(2)、roof_visual_shape(6)、
  dormer_tiles(1)、roof_buffers。依赖:Roof 资源(已具备创建/删除/类型切换/回放)。验收:屋顶噪声/脊/瓦/
  视觉形状资源闭合。
- **A4 stairs 收尾**(interaction/restorer):stairs/structural_graph(8)、visual(11)、stairs_collider(1)、
  stairs_removed_supports(9)、invalidate(3)、stairs_walls(5)。依赖:楼梯权威图(已具备)。验收:楼梯快照/
  差分/碰撞闭合。
- **A5 collision/rt**(interaction/restorer + re-analyst):collision/world.rs(170,最大单文件)、
  intersect_meshes(21)、visualize_colliders(6)、rt/{mod,bvh,bvh_builder}(31+5+6)、shape_intersections(1)。
  依赖:墙/屋顶/楼梯几何。验收:碰撞世界/相交/BVH 与几何附着闭合。

### 卡 B world_raster 状态机(order 4)—(interaction/restorer)
- world_raster.rs + resources/world_raster.rs 已在位但无 contract:核对开始/续画/提交/历史提取状态机
  (README 断点已部分实现),补 `resources/terrain_stroke.rs`(10)、`rocky_terrain.rs`(10)、
  `terrain_rock_mask.rs`(1)、`garden_mask.rs`、`water_mask` 消费对账。
  依赖:地形编辑/延迟回读(已闭合)。验收:栅格状态机与历史/水/草消费全链闭合。

### 卡 C history 收尾(order 5)—(state/restorer)
- resources/history/:color_edit(48)、decorator_edit(45)、path_edit(13)、roof_edit(22)、
  stairs_history_edit(12)、terrain_edit(24)、tree_edit(25)、wall_compound_edit(79)、wall_history(3);
  systems/history/:history_process(17)、replay_* 族(replay_garden/path/pop_bucket/stairs/terrain/water/
  mode)、finish_replay(2)、history_recording(2)、mod(3)、signifier_replay_active(3)。
  依赖:History 权威链(已具备编辑/undo/redo/replay)。验收:各 edit 类型 serde/apply/undo/redo 全闭合。
  依赖卡 A(编辑域)→卡 C(历史域)。

### 卡 D camera/time/weather(order 6)—(interaction/restorer + re-analyst)
- camera_rig.rs(44,present partial)+ camera_rig/{butterfly 4,free 9,normal 11,walk 9,camera_body 1}、
  main_camera.rs(22,partial)、look_at_camera(3)、arrow_gizmo_animation(1)、mesh_follow_terrain(2)。
- time_of_day(3)、time_update(2)、weather.rs(1)、daily.rs(10)。
  依赖:AppMode/输入(已闭合)。验收:相机模式状态机/相机 rig 参数缓存(供 save Camera 节)、
  时间/天气更新闭环。**注意**:camera_rig CameraRigParametersCache 已派 re-analyst-state 挖证(卡0),
  与此卡对接。

### 卡 E input/UI/mode(order 6)—(interaction/restorer)
- systems/ui/:debug_visualize_volumes(11)、hover_intersection(14)、slider3d(17)、widget_params(35)、
  signifiers/{mod 22,animation 2,signifier_create 4,signifier_mark_for_delete 3}、ui_update(5);
  ui_* 顶层:ui_2d_follow_mouse、ui_camera_mode_gizmos(3)、ui_edit_focus(5)、ui_edit_freehand_walls(4)、
  ui_focus_mode_shape_height(4)、ui_move_freehand_walls(2)、ui_move_shapes(8)、ui_raise_shape(8)、
  mod_key(4)、clear_canvas(2)、brush_preview(6)、drawing_input_smoother(4)、placement_data(11)。
  依赖:AppMode/输入全链(已闭合)、模式管理器。验收:UI 模式/画笔/光标/悬停/聚焦/标号可见链闭合。

### 卡 F audio 域(order 7)—(eco/restorer,衔接音乐)
- systems/audio/ 34 文件(158 函数:building/clutter/decorator/forest/garden/ivy/menu/path/roof/sheep/
  terrain/timberframe/tree/trigger/wall_*/water/weather 等)+ audio_resources.rs(47,GladeAudio 曲目生产者,
  衔接卡 music native FMOD)、audio_settings.rs(7)、gpu_lut.rs(11)。
  依赖:各域权威状态(墙/屋顶/生态/地形,卡 A-D)、MusicPlayer(已闭合)。验收:触发表/事件→AudioCore/
  FMOD 全链闭合。

### 卡 G startup/系统注册(order 8)—(state/restorer + re-analyst,部分宿主)
- startup/ 27 缺失文件(clearing/debug/glade_themes/init_color_library/load_audio/load_glade_grading/
  load_glade_tod_and_weather/load_mesh_atlases/load_prefabs/load_shaders/panic_reporter_startup/
  spawn_brush_previews/spawn_human_ref/spawn_preplaced_trees/startup_blit/startup_critters/
  startup_deferred_raster/startup_grass/startup_meadow/startup_nani/startup_roof/startup_sheep/
  startup_sky_and_sun/startup_time_of_day/startup_walls/startup_water)+ 8 在位文件核验 + lib.rs/mod.rs
  系统注册顺序(原初始化/调度顺序,round3 startup 章节)。
  依赖:卡 A-F(所有域资源/系统)。验收:原 init 顺序核对、资源插入/系统注册全链闭合。

### 卡 H render 消费桥(order 8)—(宿主接线,需主会话确权)
- ivy GPU 提交(GraphicsDevice)、terrain 延迟回读 GPU 执行、garden pin 渲染、wall upload_instanced_ssbo、
  render_loop→render 桥、backend(configwatch/game_frame_index/icc/meshwatch/texture_watch/vao_update/
  prepare_frame_constants/transient_mesh)、render/ 40 文件(34 在位)。
  依赖:卡 A-G + GraphicsDevice(render 域/tiny-glade 宿主)。验收:GPU 消费桥闭合;属宿主接线。

## 共享基础缺口评估(2026-10-15)

- **⚠️ SsboId::of 对账(NameHash secret,库内修)**:`utils/src/name_hash.rs` 与 `country-core/src/name_hash.rs`
  均用 `xxh3_64(name)`(默认 secret,seed 0);但 `tools/country-core-audit/verify_ssbo_id.py` 从原 EXE
  **0x142b2ebb0 读 192B 自定义 secret** 复算 Cobblestone3RasterStagingSsbo/NaniNextVisibilityBitmaskSsbo
  均 OK,`tests/scratch_ssbo.rs` 用该 secret 对 ivy SsboIds(0x9e3cfa7f2993f406 等)。即原版 SsboId 用
  游戏 secret 的 xxh3_64_with_secret,当前 NameHash::compute 用默认 secret → **对账缺口**。处置:
  re-analyst 定点反编译 0x140c9cd80 裁决 NameHash::compute 是否传 0x142b2ebb0 secret;是则改
  `utils` 库内实现(scope_shared_foundation 库内实现原则),country-core 同步;卡6/卡 A1 SsboId 对账
  以此为准。
- **tabloid / tg-history 接线点**:country-core 已真实消费 tabloid(Publisher/Subscriber/State/StateDelta,
  world_raster/terrain_height/dirty_tiles/wall_state/registration)与 tg_history(RoofVisualEdit,
  building_history_transactions),编译通过、接线健康;**本批无需回补**。共享层自身缺口按
  FOUNDATION_BUSINESS_FUNCTIONS.md 独立任务,不在本批。

## 宿主接线清单(需主会话确权)

| 项 | 内容 | 归属 |
|---|---|---|
| render GPU 桥 | ivy/garden/wall/terrain GPU 提交、render_loop→render、GraphicsDevice | tiny-glade/渲染宿主 |
| Steam 全链 | steam/mod + startup_steam + resources/steam(22)SDK 集成 | steam 宿主/crate |
| 全局调度注册 | lib.rs 系统注册顺序、AppMode/toolbar 判别映射 | tiny-glade app_systems |
| 库外消费对账 | DecoratorStorage 实体(system_clutter)、save/load 经 run_game | 宿主 |

## 依赖顺序与验收口径

```
卡A(walls/roofs/stairs/collision)→卡C(history)→卡D(camera/time/weather)+卡E(input/UI/mode)→卡F(audio)→卡G(startup)
卡B(world_raster)与卡A 并行(都靠地形/墙权威)
卡H(render 桥)依赖全部 + 宿主确权
```
- **验收口径**:每卡按原函数清单逐一闭合 + fresh 反编译 + 真实消费者接线;不把编译/局部测试当整模块完成;
  宿主项(卡H/Steam/全局调度)需主会话确权范围后再派,期间不伪造 GPU/调度成功。
- **优先建议**:卡A1(decorator 族,衔接卡5)、卡D(camera_rig,衔接卡0 域类型)先行;卡H 与
  re-analyst 的 NameHash secret 裁决并行(涉及 utils 库内修)。
