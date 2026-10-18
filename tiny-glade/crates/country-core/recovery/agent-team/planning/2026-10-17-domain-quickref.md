# 域速查索引(2026-10-17,来自各模块 LOGIC/REBUILD 浓缩)

> **状态更新(2026-10-17 晚)**:本表多数"需重做"项已在密集波次中 closed;最新进度与剩余登记
> 见 `2026-10-17-final-progress-v2.md`。剩余项均为跨域依赖倒置/宿主接线/渲染设备面。
> **上层 13 模块**(glade-border/forest/tree-tool/critters/water/timberframe/roof/color/
> decorator/clutter/onboarding/wall-constructor/toolbar)速查见文末 **U1-U13**;进度汇总见
> `2026-10-17-upper-final-progress.md`。
> 用法:定位目标函数先查本表一行;详细分歧点仍看对应 LOGIC/REBUILD。
> 计数口径:判等 / 需重做 / 无body登记 / 待判等。

## walls(LOGIC:reports/eco/walls/LOGIC.md;需重做 11 + 跨域阻塞 3 + 待判等 ~240)
- 需重做:on_wall_split 0x140a1f590 / merge_with 0x14155b170 / on_walls_merged 0x140a1ea40 / restore_snapshot·take_snapshot 0x140a20d70 / contains_shape 0x1411ff330 / expand 0x1411ff680 / snapping_maintain 0x140a6ad00 / snapping_to_segments 0x140a6b5e0 / query_with_aabb_clamped 0x140aa5a10 / CompoundEditBuilder 0x140a82b20·2b40·2d50
- 跨域阻塞:eraser finish 0x14092f990(需 history WallHistoryEdit 0xf8B)/ restore_decorators 0x14155ba30(需 collision remove_collider)/ get_building_and_roof_colors 0x140a94880(需颜色库)
- 待判等 ~240:大 UI 函数(ui_rotate_circle 0x1409ac910 / create_circle 0x14097e560 / ui_create_circle 0x14165bed0 / ui_edit_circle 0x14165d460 / show_full_circle_signifier 0x141660880)
- 已判等(本轮):create_rectangle ridge(0x140a93790)/ unleash 0x1408337c0 / DecoratorOpEventKind::id 0x140833570

## collision(LOGIC:reports/interaction/collision/LOGIC.md;需重做 10 + 无body ~85 + hardpoint 2)
- 需重做:Raycasting::raycast 0x1408cc2e0(转发错)/ intersects_any 0x1408cc320 / spherecast 0x1408cc390 / mesh_intersect_collider_diff_publisher 0x1408cc670 / legacy_raycast 0x1408cc680 / RaycastWorld::raycast 0x1408cc870 / new_convex 0x1408cec10 / RaycastWorld::new / SubworldIndex::from_usize_unchecked 0x14067dd80 / impl$0::build 0x140b685e0
- 无body:world.rs ~66 + intersect_meshes 21(业务4)+ visualize 6(业务2)+ rt 13
- hardpoint:update_rapier_colliders 0x1408d0490(4630 指令行为骨架)/ find_mesh_mesh_intersections 0x140a97b70(跨域)
- rt 设备面:update_rt_world 0x1408bc650 / upload_rt_world 0x1408bcff0 / build_gpu_bvh 0x140b0da50 / bvh_builder_worker 0x140b24d50

## audio(LOGIC:reports/eco/audio/LOGIC.md;需重做 21)
- wall_audio_ui 0x1408b7690(kind0/9/0x11/0x12+事件布局) / AudioInstance::drop+stop_on_drop 0x14089a280·0x140a57540 / AudioCoreInner update·set_parameter·set_vca_volume 0x140a53a00·db0·4810 / Bank::load_sample_data 0x140a53870 / water_audio_system 0x140873630 + init 0x140873140 / tree_audio_system 0x1414638e0 / trigger_audio_system 0x1409eafa0 / sheep_audio_system 0x1409d5ea0(0x38+ 分派)/ terrain_audio_system 0x1414618c0 / path_audio_system 0x1409d0f80 / firstperson_steps 0x140a889b0 / terrain_stairs 0x141462dc0 / building_audio 0x1417d2ad0 / roof_audio 0x140aa3860 / water_ambience 0x1409ecea0 / signifier 0x1409d6b90 / wall_audio_path 0x140a89bb0 / wall_audio_orchestrator 0x141234bf0 / timberframe 0x140a67b90 / decorator 0x14145dc20
- 判等:music_player 8 + AudioInstance 判别族 + clutter 39 表

## save+meta(LOGIC:reports/state/save/LOGIC.md;需重做 19)
- commander_update 0x14091f180(case0 门控反) / load_game 0x14146d030(fence 释放丢) / save_thread 0x141720680(Shutdown 无 flush) / execute_save_cmd 0x141721aa0(unsaved 死链) / flush_deferred_and_sync 0x14171f7f0 / save_game_system 0x140a3f470(只节拍器) / close_file 0x140a43840 / create_backup_of 0x141e59860 / last_error_state 0x140a439a0 / capture_game_snapshot_save_data 0x1417a0430 / capture_game_snapshot 0x1417a1d00(无body) / try_chunk 0x1417a6d70 / validate_and_load_closest_snapshot_data 0x1417a6e50 / init_game_state_from_snapshot 0x1417a9c50 / parse_saved_game_data 0x141470d00 / add_game_startup_systems 0x140941110 / copy_dir_all 0x141740c00 / load_glade_audio 0x1408be360(无body) / save_game_to_file(部分)

## roofs+stairs(LOGIC:reports/state/roofs-stairs/LOGIC.md;需重做 ~15)
- roof_noise::noise 0x140a152c0(hash 链+finalizer) / apply_roof_noise 0x1412348f0 / get_height_01_unclamped_from_ws 0x1408e4790 / get_roof_shape_from_pos_ws_clamped 0x1408e4750 / roof_shape_at_intersection_y 0x1408e3820 / roof_uv×2 0x1408e4040·4090 / roof_ridge get_inner(错标) / structural_graph clear(错锚=MagicSupports) 0x140869a30 / disconnect_node_by_collider 0x14086a4c0 / MagicSupports::clear 0x140aa8210 / reset_edge 0x140aa8a60 / force_rebuild / StairsRemovedSupports(边键模型)/ replay_stairs 0x140af75b0(双路径 undo 反)
- 共享域:wall_platform_stairs 5

## camera/ui/time(LOGIC:reports/interaction/camera_rig/LOGIC.md + ui_mode/LOGIC.md + time_weather/LOGIC.md;需重做 19)
- camera_rig:load_parameters 0x140911520(轨道四元数+ActiveRotation)/ current_dyn 0x1409149d0 / current_dyn_mut·current_body_offset
- ui_mode 12:WidgetParams::new 0x1409a8e10 / no_history_recording 0x1409a8f10 / allow_negative_distance 0x1409a8f40 / UiFocus::as_mei 0x1409a9130 / Slider3dState::project_cursor_onto_line 0x140a9bd40 / signifier_animation 0x140a69c30 / UiSignifierStream::write 0x140b2cf90 / signifier_create 0x140b2cfa0 / signfier_mark_for_delete 0x140929160 / brush_preview 0x140865050 / brush_size
- time_of_day 4:主系统 0x140a73460 / advance_playback / advance_tod_hour / suncalc_pos_to_vec

## terrain+ui_clutter(LOGIC:reports/interaction/terrain/LOGIC.md + ui_clutter/LOGIC.md;需重做 12)
- terrain:edit_terrain_stroke 0x14094e000(PlayTerrainSound 系编造) / terrain_stroke_changed 0x140a03ba0 / terrain_hover_idx 0x140aff720 / ui_draw_terrain 0x140a9fdc0(画布门反) / ui_edit_terrain 0x140aef1f0(主体缺) / ui_terrain_focus_mode 0x1409a4fb0(仅骨架) / terrain_rock_mask 0x1409ea800
- ui_clutter:handle_rmb 0x140976ca0(旋转公式) / ui_convert 0x14099ca60 / debug_spawn 0x1409a1f50 / decoclutter_show_pins 0x1412357c0 / ui_focus_decoclutter 0x1409a92c0

## history(LOGIC:reports/state/history/LOGIC.md;需重做 14 + 无body 11)
- 需重做:History::add 0x140a84ec0(tag-7 墙校验)/ closest_resolved_snapshot 0x140a84a50 / finish_replay 0x1408b36a0 / replay_mode 0x140a7c550(0.2s) / replay_stairs 0x140af75b0(0.05 vs 0.1)/ replay_wall 0x140951b80+process 0x140abfb30(17 操作)/ history_process 0x140af5b30(第二事件流)/ replay_edit 0x140af6170(Color/Decorator)/ ColorEdit 0x14003c5d0 / DecoratorHistoryEdit 0x1404f46e0 / RasterEdit 0x1404896f0·0x1416e8510 / TreeEditData 0x14003d4e0 / StairsHistoryEdit 0x140259af0·0x140a18ee0 / DecoratorHistoryState::from 0x140b3f930
- 无body:add_decorator_diff 0x140a854a0 / send_state_diff_events 0x1417bdae0 / convert_before_after_states_to_changes 0x140b3fab0 / missing_prefab 0x1409e6210 / is_default_chimney_smoke 0x1408c5680 / replay_pop_bucket 0x140b38250 / get_param 0x140ae5830 / debug×2 / history_finish_recording / signifier_replay_active×2

## UI域余量(LOGIC:reports/interaction/ui_mode/LOGIC-UI-REMAINDER.md;无body 21)
- grading_lut 5:flattened(已实现)/ load_unwrapped 0x140b00bc0 / GradingLuts::load 0x140b01df0 / lut_name_to_asset_name·asset_name
- prefab 16:PrefabLibrary{add 0x1408e2540,get,has,iter}+register + spawn_prefabs 0x140840610 / create_prefab_rt 0x140841550 / create_prefab_gpu_instances 0x140841b00 / update_prefabs 0x140841d60 / despawn_prefabs 0x140842d60 / despawn_prefab_contents 0x140843390 / hot_reload_prefabs 0x140843740(缺 asset_libraries PrefabLibrary 类型)

## session/music(LOGIC:reports/state/session/LOGIC.md + state/music/LOGIC.md;需重做 6)
- session_file_list 0x140b46300 / write_to_disk 0x140b463c0(version 2)/ NextSession::create 0x140b46960 / delete_session 0x140b46d60(自造三段)/ SessionFlowPlugin::build 0x140b47840 / music_audio_system 0x140a72810(photomode start 反)

## ivy(LOGIC:reports/eco/ivy/LOGIC.md;需重做 9)
- into_world_space 0x140b199e0(Y 分量错) / ivy_grower 0x14094a910 / ivy_spawner 0x14094cf90(门来源) / ivy_pruner 0x140997410 / is_point_still_valid 0x140998dc0 / intersect_ivy_growth 0x14094cc60(死代码) / send_to_gpu_leaf 0x1408c1580 / send_to_gpu_segments 0x140a9b4e0(SsboId 不命中)

## input(LOGIC:reports/interaction/input/LOGIC.md;需重做 8)
- translate_input 0x1417cd0a0(Steam 门反,最严重) / control_input_replay 0x140941a30 / drop_oldest_sync_point 0x140949660 / reset_input_state 0x140942b00 / record_and_replay 0x14083c620 / do_input_replay_ui 0x14083cdd0 / input_record_replay 0x1409423d0 / update_cursor_position

## world_raster(LOGIC:reports/interaction/world_raster/LOGIC.md;需重做 1 + 无body 4)
- as_float_grid 0x140ae4c20(签名不符) / 无body:draw_path 0x140ae4d00 / record_path_history 0x140b1c270 / path_brush_preview_size 0x140b1c010 / build 0x140b68e40

## garden+sheep(LOGIC:reports/eco/garden/LOGIC.md + eco/sheep/LOGIC.md;需重做 2)
- startup_garden_brush 0x1415f87c0(shader+spawn 缺) / Sheep 结构布局矛盾(goal 与 move_state 槽错位)

## render_loop(LOGIC:reports/interaction/render_bridge/LOGIC.md;需重做 0)
- 判等 34 + env 闭包 55 + nani 20 + ice 4;仅登记 RoofTileSsbo 存储形态(归 roof/device 域)

## steam(LOGIC:reports/interaction/steam/LOGIC.md;剩余 0)
- 重建 22 业务,27 胶水;无剩余

---

# 上层 13 模块速查（2026-10-17 晚，coordinator-quickref 汇总）

> 范围：`reports/{wave2,wave2b,wave3,gap}/**` + `reports/integration/{coregap-1,coregap-2,purelogic,external-dep}/**`。
> 目录中不存在 `reports/consume`、`reports/remaining`；对应轮次以 wave2b/wave3/gap 及 integration 报告为准。
> `reports/unlock/nani` 仅有原始证据（无 RESTORE.md），NaniBundle 落库仍按"在途"计。
> 口径：判等=逐语句等价且已复核；重建=控制流/写路径已落库但含已登记 I/U 分歧；未实现=诚实空体/近似体并写明解锁条件。
> 条数：137 行（族条目按组计；按函数面展开约 305 函数，见 `2026-10-17-upper-final-progress.md`）。

## U1. eco/glade-border（3 条：判等 1 / 重建 1 / 未实现 1）

| 函数 | VA | verdict | 关键分歧 / 解锁条件 |
|---|---|---|---|
| display_glade_border | 0x141be0ad0 | 未实现-需重做 | 分歧① `DebugLine::show` 0x140b04f70 未落库（折线 drop）；分歧② `UiSignifierStream::write` 0x140b2cf90 仅收 UiAppendSignifier，SignifierFactory 40B/closure vtable 0x142efa150 不可承载。解锁：core 补 show 系列 + stream 承接/唤醒 |
| closure$1 | 0x141be1080 | 重建 | builder 链 + 2×insert（DrawTopology Alt(2.0)、DashedLineLength=env.len×0.7）判等；唤醒未接线（dead_code） |
| feedback_input_out_of_border（宿主内联） | 0x141be2280 | 判等 | PlayUiSound 变体#4 `{kind=4@+0,id@+0x18}` stride 0x20；上升沿去抖 cache@+0x158 判等 |

## U2. eco/forest（5 条：判等 1 / 重建 1 / 未实现 2 / 本地等效 1）

| 函数 | VA | verdict | 关键分歧 / 解锁条件 |
|---|---|---|---|
| manage_tree_prefabs | 0x141bf16c0 | 重建（门控判等）+ 未实现 | +0x18 处理字节门控、墙圆查询恒运行、±65 高度门判等；构造形状源 ShapeParameters 0x28B / 点-矩形 dist 分派未实现 |
| handplaced_tree_animation | 0x142279500 | 未实现 | TreeAnimation stride 0x10 的 +4 进度位；PrefabInstanceData 0x30 写面 |
| forest_tree_animation | 0x14227a0d0 | 判等 + 2 缺口 | +0x44 有树标志；DespawnPrefabCmd 载荷形状 |
| get_gothic_hat_bones | 0x1407eb500 | 未实现 | 未 fresh 复核；骨架主体待取证 |
| Aabb3::intersect_segment 3D | — | 本地等效（标注） | utils 3D 版待补 |

## U3. eco/tree-tool（9 条：重建 4 / 未实现 5）

| 函数 | VA | verdict | 关键分歧 / 解锁条件 |
|---|---|---|---|
| process_tree_cmds | 0x142267430 | 重建 | Move/Delete 判等（Delete 含 `Entities::contains` 门控）；Insert 分支 T2 未实现（InsertTreeCmd 0x48/SpawnPrefabCmd 0x50/TreeMoveBend/TreeAmbience）；T1 PlayTreeSoundCmd 缺 in_water@+9/inverse_lerp@+0xc |
| replay_tree | 0x141bc34f0 | 重建 | 时序 bit0 极性/Move/Delete/Place/undo 判等；T2 Insert 0x48 缺 height/variant/rotation/flags；T3 Move undo 旧位置 edit+0x14/+0x1c 不可表达 |
| ui_move_tree | 0x14226b1a0 | 重建 | MoveTreeCmd 0x20 载荷判等；slider3d case1/2/3 近似（分歧 15/16）；补登记门禁 mode∈{0x10,0}；候选源 `storage.iter()` 代理 |
| convert_preplaced_trees | 0x14226be70 | 重建 | Wait flag 判等；墙查询闭包方向判等（0x1425defd0 命中墙→清位→删除）；`insert_starting_edit` 未实现——core `ledger_mut()` 已在 gap-core-events 落库，解锁=tree-tool 侧改为 `&mut ObservedHistoryLedger` |
| hide_trees | 0x14226a260 | 未实现 | HideTreesinAreaCmd/NextToWallCmd 缺 shape@+0 bit0 + @+0x24 门控；两命令 stride 0x18/0x30；树级 `Decorators::iter_removed_ids` 资源 |
| ui_place_tree | 0x142269170 | 未实现 | raycast + `geo_src_to_edit_subject` 早退分支 |
| place_tree | 0x142269970 | 未实现 | InsertTreeCmd 0x48 全字段 + 会话门控接线 |
| ui_delete_tree | 0x141bd2510 | 未实现 | PathDrawEvent flag@+0x18 / 中心半径源 |
| tree_move_bend | 0x1422689c0 | 未实现 | PrefabInstanceData +0x24/+0x28 step 写面 |

## U4. eco/critters（18 条：判等 7 / 重建 2 / 部分 2 / 未实现 6 / DIVERGE 1）

| 函数 | VA | verdict | 关键分歧 / 解锁条件 |
|---|---|---|---|
| move_critters（命中码） | 0x142318fa0 | 判等 | 命中码 2=岩石/3=跳墙；补登记 spherecast 外层门 `(local_fc||local_100)` 未建模、loc==2 AL 标志 |
| critters_avoid_collisions | 0x1423179a0 | 判等 | hit==2||球距>=1.0 保留；<0.4&&water8→DespawnDuckCmd；墙 relate 循环语义已登记 |
| update_wrens | 0x14231e390 | 重建（REVIEW 需修 2 已修） | 5 handler 结构判等；h3 移动精确式 `pos.y+=vx*dt*(dvy*dir_y+dir_z)`、anim_val=progress×7 已修；距离查询 0x140a12650/音频 call/panic 路径 U |
| sheep_walk_animation | 0x14231a630 | 判等 | 跳表 0x14304b0f8 两 handler 逐语句；accum fmod 1.25/×24/mod 30 |
| sheep_pet_animation | 0x14231ac20 | 判等 | 跳表 0x14304b178 五 handler（0.3/1.0/0.2） |
| update_butterfly_states | 0x141d84d40 | 部分判等 | 主循环/step_state/state1/3 结构判等；state0/2 `[+4]`/xmm14 U、水/墙资源缺；state3 NaN 次差已登记 |
| set_butterfly_velocities_towards_goals | 0x141d867b0 | 判等 | vel 表 4 handler 逐语句（常量 from_bits） |
| DuckSpawnLocations::new | 0x140f8ee90 | 实现/判等 | 网格 `(i/9)*52-(1-i/9)*52`；timer=25.0 |
| FishSpawnLocations::new | 0x140f909f0 | 判等 | 公式/行列序一致；used 字节 DIVERGE 登记 |
| despawn_fish | 0x140f90c10 | 未实现 | 真实=资源重置；缺 `FishBoids(Resource)` + `OnExit<AppLoadingState>` |
| plan_duck_trajectories | 0x14230ff00 | 未实现 | dev 热键 A/B + legacy_raycast 插值 + Frenet + Trajectory 4-f32 + spawn 10 组件 bundle；转 re-analysis |
| move_ducks_along_trajectories | 0x142312200 | 未实现 | Frenet 帧；轨迹状态二进制 ∈{0,2} vs 现 {1,3}；音频链 |
| update_decorator_bird_spots | 0x14231c7d0 | 未实现 | create_spots 布局/数量、双列表滑动窗口缓冲、装饰物注册表；转 re-analysis |
| update_roof_bird_spots | 0x14231d6c0 | 未实现 | random_splits 分段语义、RoofRidgeDims/噪声/选边；转 re-analysis |
| ui_pet_sheep | 0x142324f80 | 重建核心 + UI 未实现 | 缺 `UiWidgetContext::intersecting`/SignifierEntityBuilder/slider3d::into_drag_state |
| ui_pet_ducks | 0x142322e30 | 重建核心 + UI 未实现 | 同上 |
| ducks_react_to_being_inside_buildings | 0x14231b140 | 未实现 | 缺 PublicWalls::get/get_wall_shape/Roof::height/GladeBorder::is_pos_inside |
| CritterGoal 字节序 / sample_at_world_xz→u32 | — | DIVERGE 桥接 | 跨组类型变更，集成 owner 裁决 |

## U5. state/water（5 条：判等 1 / 未实现/登记 4）

| 函数/条目 | VA | verdict | 关键分歧 / 解锁条件 |
|---|---|---|---|
| draw_water 注册 | — | 判等/已验证 | add_water_cpu_systems + register_water_events 自足 |
| draw_water::closure$0 | 0x141ba9700 | 未落库 | SignifierEntityBuilder 链 + "brush_circle_scribble" |
| 橡皮 signifier 记录布局 | — | 待 core 复核 | SignifierFactory 40B vs UiAppendSignifier；gap-core-events 已补 `write_raw/write_factory` 承接面 |
| update_water_audio_grid intersects_any max=1.0 | — | 语义差（已登记） | core API 增 max 参数后回填 |
| WaterAudioGrid 语义投影 | — | 契约差 | 24B 单元 {x,z,窗@+8,计数@+0x10} vs 语义模型；音频域裁决 |

## U6. state/timberframe（9 条：判等 6 / 重建 2 / GAP 1）

| 函数/条目 | VA | verdict | 关键分歧 / 解锁条件 |
|---|---|---|---|
| construct_timberframe_rectangle | 0x141a41ba0 | 重建+写路径（判等） | 相位/`n=⌊d/1.51⌋`/截面 0.2·0.18/裙层两环/屋脊 min+perimeter>0.5+1/BeamCount 写回判等；ResMut 库面、RenderInstance 字段序 I、bit0 代理、rotation_from_normal→beam_rotation I |
| construct_timberframe_circle | 0x1421b67a0 | 重建+写路径（判等） | count=max(6,⌊p/1.5⌋)、u=i/(count-1)、屋顶环高度判等；主环 Quat::IDENTITY 占位 GAP；不 clear 忠实 |
| split_height_into_skirts | 0x140b34f80 | 判等 | height≥5.9→[4.5,8.0,11.5…] cap 4 |
| utils::rotation_from_normal | 0x140c90a40 | 判等 | 4 case 半 sqrt；gap-utils-toolbar 已落库，beam_rotation 可换回 |
| Rectangle2d::orientation_mat3 | 0x140c99570 | 判等 | Mat3 9 store 逐字段 |
| timberframe_beam_count_change_audio | 0x1421b6610 | 判等主体 + GAP | 缺 get_building_size_01 0x1417d3f10 + PlayTimberframeSound size/id |
| ConstructTimberframeCmd bit0/+0x1c | — | 未判等代理 | 事件首字节 bit0（rect=1/circle=0）与 +0x1c 契约未暴露 |
| GameClock frame_count 布局 | — | GAP | 字段@0x28 vs 二进制读 +0x2c |
| timberframe_on_glade_unload | — | 阻断 | `OnExit<AppLoadingState>` bevy_state 未入 workspace |

## U7. state/roof（8 条：判等 1 / 重建 2 / 未实现 5）

| 函数/条目 | VA | verdict | 关键分歧 / 解锁条件 |
|---|---|---|---|
| simulate_roof_tile_for_audio | 0x1421a7090 | 判等 | angle' 三处修正属实（panic 判据/角度钳制/累加门用最终角） |
| highlight_roof | 0x141a284b0 / 0x141a31280 | 重建 | tag=0 清位 + 事件 stride 0x18 {wall@+8,color@+0x10} + to_rgb→EditingHighlight.color 判等；8B pad 覆盖 I |
| gpu_flush_buffers | 0x140de50a0 | 重建结构 + gpu_flush GAP | RoofTileSsbo 仍 TypedStorageBuffer（需 SparseSsbo CHUNK_COUNT=0x2000）；GraphicsDeviceResource +8/+0x10/+0x150 槽未建模 |
| calculate_roof_trigger_volumes | 0x1421a3220 | 未实现（诚实） | 细分 `(is_flat^1)*5+4` 已登记；1580 行几何/剖面/檐口/UV/噪声未逐语句 |
| ui_edit_roof / do_ui_edit_roof | 0x141a18dd0 / 0x141a19a90 | 未实现 | WidgetParams/slider3d/box_trigger_volume + HighlightRoofCmd |
| ui_edit_roof_ridge_dir + ridge_length + focused 族 | 0x141a386b0/0x141a39490/0x1421a8210/0x1421a8c50/0x1421aa170/0x1421ad440 | 未实现 | TweakerView/project_cursor_onto_ray/CameraMatrices::eye_right/xz_rotation_bad/RoofRidgeDims/SignifierFactory |
| switch_roof_type | 0x1421ac4f0 | 部分重建 | 非 mesh 部分判等；mesh ops 缺 NaniBundle/DrawableMeshBundle/TransientMeshRegistry/Handle\<Vao\> |
| RoofSound/RoofTileSimulation stride 0x18/GameClock.delta | — | 边界登记 | 载荷字段布局/数组模型/+0x24 vs +0x28 |

## U8. state/color（7 条：重建 3 / 未实现 4）

| 函数/条目 | VA | verdict | 关键分歧 / 解锁条件 |
|---|---|---|---|
| show_color_options | 0x14229a7f0 | 重建（case 0/1 已按 REVIEW 修正） | 事件遍历/两段跳表 0x14303ca70 40 项/创建/spawn/事件写面判等；case 0 主 13 kind-3 swatch + plaster(peel low/medium/high)/halftimber/skirt/is_flat 条件块；case 2..0x26、平顶 else、demo、事件写面 GAP |
| color_menu | 0x142291230 | 未实现（登记准确） | CustomizationIcon 0xa0 + `Events<(&str,SettingsMenuState)>` + History Color + ~12 参 ABI typed-gap |
| init_color_ids_buffer + write_roof_color_id_to_gpu | 0x142290750 / 0x142298520 | 重建 + SSBO 落库 | coregap-2 四色 SSBO 已落；写路径接线待 |
| set_flag_connection_and_chimney_smoke | 0x142298970 | 重建 + 未实现 | ChimneyExtra +4 已落 coregap-2；烟值写面待接 |
| set_roof_customization | 0x142299dd0 | 未实现 | PublicWalls 稀疏位集/密集表 stride 0x14/0xa0/0x48；内嵌 InnerWalls/RoofShape 未落库 |
| color_button_animations / delete_color_icon_entities / hide_color_menu / autoclose_color_menu / ColorRadialMenu::create / color_cursor_icon / ui_color_* | 0x1422979b0/0x142297ff0/0x1422adcf0/0x142298d30/0x1425f27c0/0x141c9b510/0x141c9ab70/0x1422afe70/0x1422b0e00 | 未实现 | CustomizationIcon/Ui2dExtent/Ui2dImage/UiInactive/RaycastWorldResult+0x1d8/ActionSetStates |
| replay_color | 0x141c7f270 | 未实现 | ReplayBuffer/ReplayCommand/DecoratorHistoryEdit 原始字节布局 |

## U9. state/decorator（14 条：判等 2 / 重建·部分 6 / 未实现 6）

| 函数/条目 | VA | verdict | 关键分歧 / 解锁条件 |
|---|---|---|---|
| maintain_dormer_tiles | 0x1407ce920 | 结构判等 + GPU 未实现 | 游标写序 FIX-E 已修（循环前预写 total-lenA'-lenB'，每瓦 +1，末值=total）；缺 rhapsody RawTable\<u64,Box\<dyn FnOnce\>\> + RoofTileSsbo SparseSsbo |
| window_lights | 0x1407b5980 | 判等（部分）+ 收集未实现 | 0.51/0.44/0.72、灯态 timer=2rng、retain 次序、帧末判等；缺 PublicWalls 密集窗桶 stride 0xa0/0x48；NaN 检查 minor |
| chimney_smoke_anim_thickness | 0x140818df0 | 公式判等 + 收集未实现 | t 窗口/目标 r*3+(1-r) 或 r*4+2(1-r)/符号/clamp01(old+sign*dt/target) 判等（ReVa+fresh 双证）；spot 收集/rng 种子/retain 谓词 U |
| replay_decorator | 0x14080c8d0 | 结构判等 + 未实现 | DURATION_FLOOR=0.05、Arc\<[DecoratorId]\> cull 表、tag-7 扫描、反序删除判等；缺 ReplayCommand Decorator 变体 + ReplayBucket cull 字段 + 0x30B 内嵌时序 |
| WindowVisualEntityBuilder::build\<Transient/Persistent\> | 0x1412ab270 / 0x1412a9b30 | 结构判等 + 未实现-需重做 | mesh 名 12 类 NaniBundle + 默认 DrawableMeshBundle、条件链 14 位、impl$4/5 判等；NaniBundle/DrawableMeshBundle 结构体未落库；build_transient→impl$5 autoclutter 表目标分歧 |
| impl$5::add_decorator_visual_entity | 0x140811c30 | 未实现 | autoclutter 表 `HashMap<u32,Vec<DisplayChildAutoClutterInfo>>`；类型未落库 |
| apply_decorator_op | 0x141276e30 | 未实现 | archive_current_decorators_in_storage/DecoratorLocator/build_op_event 0x88B |
| create_or_restore_decorator_backup | 0x1408238a0 | 部分（已注册） | 0x30B 三 map 反向拷贝；live 资源身份 U |
| decorator_record_history | 0x140817850 | 未实现 | 48B {1,1,kind,f1,f2,f3} + DecoratorHistoryEdit 存档模型 |
| ~146 业务函数（blueprint/cull_oob/merge/visual 族/stairs/ui_systems） | — | 未实现 | 按族分批 fresh 后重建 |
| calc_gothic_half_dormer_profile_ws | 0x1407eb160 | 重建 + 部分未实现 | profile 3 档表/半径表/max_y 截断/双 Curve 判等；wall-space A/B/C/D lerp + 法线 raw PublicWallState + 0..2 索引 U |
| gothic_half_dormer_radius（inline） | 0x1412ac9b0 | 判等 | circle && shape_kind==7 && rank∈1..3 → 表{0.25,0.3,0.29}，否则 0.38 |
| clear_out_visual_entities / clear_out_wall_holes（注册面） | 0x1407f9260 / 0x1407f98c0 | 判等 + 已注册 | add_decorator_systems\<S,C\> 4/5 接线 |
| get_gothic_hat_bones | 0x1407eb500 | 未实现 | 骨架主体待取证 |

## U10. interaction/clutter（13 条：判等 4 / 重建 1 / 未实现 8）

| 函数/条目 | VA | verdict | 关键分歧 / 解锁条件 |
|---|---|---|---|
| add_autoclutter_on_windows | 0x1422cd7d0 | 判等 | stride 0x50、跳表 6 项、反射 (2xz,2yz,2z²-\|p\|²)、写块 0x68 全判等；未初始化读 +0x08/+0x31/+0x33 登记 |
| populate_autoclutter_regions | 0x1422d6ed0 | 重建（分派+槽表）+ 5 U | kind 0/2/3/4 槽 9/25/25/37；mesh-10=1.5、select_mesh 严格 `acc>r`、count==0 panic；网格表→资产/kind1/default/记录 stride 0x48/kind4 权重 U |
| load_clutter_meshes | 0x1422c0760 | 结构判等 | 55×closure_0 + from_iter + insert_resource；路径表 I/U（strings 未取） |
| manage_autoclutter_entities | 0x1422d04e0 | 未实现 | 宿主查询 +0xf8..+0x120/四元数/ColorId/update_shutter |
| read_bush_positions_from_gpu | 0x140eebaa0 | 未实现 | GPU 回读 FrozenMap\<SsboId,Box\<BufferState\>\> |
| autoclutter_animation + add_autoclutter_around_gates/windows/add_preplaced/add_tree_setdressing | 0x1422ccef0/0x1422cc390/0x1422cefe0/0x141ce5900/0x1422d3800 | 未实现 | Easing 曲线/四元数/变更检测/门控序列 |
| construct_inter_roof_merlons / spawn_stitch_bricks | 0x1422d4870 / 0x141ce7190 | 未实现 | BrickTransformSsbo 字段 / SsboId 烘焙 |
| request_inter_roof_merlon_audio / detect_inter_roof_merlons / detect_intra_shape_corners / add_hole_at_shape_intersection / spawn_stitches | 0x1422d4230/0x141ce3a70/0x141cd1a90/0x141cd3ce0/0x141ce65b0 | 未实现 | indexmap Bucket kind 8/9 / intersect_shapes / WallSpaceCoord / 四元数 |
| process_autoclutter_candidates | 0x1422ce260 | 未实现 | closure_env_0 过滤谓词 / closure_env_1 排序键 / case5 key |
| autoclutter_subscribe_to_tabloids | 0x1422d6930 | 未实现 | RaycastWorld::subscribe_to_changes×8 / WorldRaster::subscribe\<512\>×3 |
| set_current_tick / reset_autoclutter_for_new_frame / remove_region_from_lazy_grid | — | 未实现 | PDB 系统签名 + GlobalId 删除键 |
| add_region / mark_dirty / flush_candidates_pending_removal | 0x140ee3fd0/0x140ee48c0/0x140ee4bf0 | 判等 | Arc\<[Vec3]\> payload；State/Region 0x40/Spawn 0x68/Candidate 0x98 |
| dirty_walls | 0x141cd16b0 | 判等 + 已注册 | detect_inter_roof_merlons 改读 changes.dirty_walls |

## U11. interaction/onboarding（32 函数 / 11 行：判等 4 / 重建 22 函数（表内 1 行）/ 未实现 6）

| 函数/条目 | VA | verdict | 关键分歧 / 解锁条件 |
|---|---|---|---|
| tutorial_completion | 0x14227c230 | 判等 | 18 偏移 0x1f4..0x205 逐项一致；每事件写 WriteSettingsFileCmd |
| hints_startup | 0x14227b450 | 判等 | 12 资源 insert 逐字节；count@+0/hint_done@+8/state@+9；CameraHintProgress 0.7/0.7/60 |
| hint_camera_enable_condition | 0x14227c780 | 判等 | 四分支逐语句；state=1 门 |
| hint_camera_progress_update | 0x14227bd30 | 判等 | tag 2/1/else；跨 1.0 写 TutorialHintComplete(0xa/0xb/0xc) |
| 22 重建族 | exponential_back_off 0x142284800 / reset_hints 0x141c1c8d0 / prompt_rollercoaster 0x14227bb70 / hint_camera_display 0x14227c910 / camera_pan 0x14227d000 / camera_rotate 0x14227d2f0 / camera_zoom 0x14227d680 / hint_highlight_shape 0x141c50120 / resolve_steam 0x14227e5e0 / make_focus_or_camera 0x141c3de00 / make_lbm_drag 0x142285270 / hint_focused_edit 0x14227da70 / hint_clutter_focused_edit 0x141c3d890 / hint_roof_shape 0x142284ed0 / hint_terrain_layering 0x142283cc0 / hint_terrain_focus 0x142283980 / hint_terrain_steepness 0x142283640 / hint_undo 0x142284220 / hint_unlink_windows 0x14227d760 / hint_deselect_tool 0x1422853c0 / hint_disable_snapping 0x14227e140 / hint_stairs 0x142285a20 | 重建 | 结构/flags 逐字节核验；分歧=事件载荷字节 seam 0、Steam index/键码占位、DisplayHintRequest placement@+0/id@+0x49 U、hint_roof_shape 完成路径已按 REVIEW 修正 |
| hint_orchestrator | 0x14227fc30 | 未实现 | closure$0 排序键 + DisplayHintRequest 0x50B 布局 |
| hint_draw + closure$2 | 0x141c51a60 / 0x14228c260 | 未实现 | mei/toolbar DSL 未落库 |
| render_hints + closure$1/2/4 | 0x141c52700 | 未实现 | mei DSL 未落库 |
| hint_position_update | 0x141c50510 | 未实现 | 跳表已解；相机矩阵/几何接线 |
| hint_focused_edit_hint | 0x14227dd60 | 未实现 | legacy_raycast 命中码 + geo_src 跳转表 |
| hint_clutter_focused_edit_hint | 0x141c3db80 | 未实现 | geo_src_w_deco_clutter 链 |

## U12. interaction/wall-constructor（16 条：判等 3 / 未实现 13）

| 函数/条目 | VA | verdict | 关键分歧 / 解锁条件 |
|---|---|---|---|
| BrickRowSine::random_f / sine_wave_deform | 宿主 0x140769760 | 判等 | 12.9898/43758.547、frac=x-trunc、0.06 幅度、顶边门 0.01 |
| merge_nearby_proposals | 宿主 0x140178a70 步骤2 | 判等 | stride 0x2c、kind@0x28==2 终止、dist<0.01 合并、size_x max/size_z min |
| resolve_hole_overlap case0 split_x 守卫 | — | 判等 + 残留 | 定向冻结用例（split_x==1&&split_y==0）待补 |
| half_timbering | 0x141218e30 | 未实现 | pdg 前导已解；逐梁坐标/closure ~0x6860B 截断 |
| construct_playermade_stairs_arches_n_walls\<true\> | 0x140496800 | 未实现 | 97KB 全流程已解；9 closure/from_curve/raycast 装配未逐语句 |
| process_playermade_stair_pillars_proposals（宿主） | 0x140178a70 | 未实现 | 步骤2 合并判等；4-6 缺 StairColliderRegistry/RtWorld/RtMeshHandle/UpdateRtWorldEvent |
| ArchWalker::walk ×4 | 0x141232bb0/0x141233380/0x141231ea0/0x14048ab50 | 未实现 | travelled 推进/周期 u/尾砖；wall_space subtract+project_clamp |
| 支撑梁 5（place/add_diagonal/emit/propose/construct_platform_stairs） | 0x141242530/0x141243740/0x141243c20/0x141fe09c0/0x1407744d0 | 未实现 | 双 Grower/cost=len·(‖Δdir‖+.5)/rdp/96B 砖链 |
| segmentation 3 | 0x140750420/0x140751240/0x140751610 | 未实现 | score>0.4/floor(len/span)/2 单位弦 |
| 立面支撑整族（circle/rect/find_stone_column_start_y/pillars/pillar assemble） | 0x14124af60..0x141253fc0/0x141250720/0x140753670/0x14120d670 | 未实现 | 跳表已解；7 参 ABI/helper 语义 |
| 门/栅栏/地板/楼梯族 | 0x140756370/0x141210110/0x140765830/0x1412388f0/0x1407661e0/playermade_stairs 族 | 未实现 | G06 冻结链接线 + 未冻结分支 |
| 半木/抹灰族 | 0x1407706d0/0x141244310/0x14075d460/0x14075b810/0x14075dc60/0x1412269b0/0x141217be0 | 未实现 | half_timbering 卡点；其余逐语句重建 |
| utils 近似体族 | 0x141223d10/0x141215540/0x1412221f0/0x1412218a0/0x141221da0/0x141221f70/0x141fd1c30 | 未实现 | 角检测 ±10°/split_height_into_skirts/SIMD 掩码 |
| gpu_clear_wall_gate_pieces / gpu_flush_plank_buffers | 0x140773c00 / 0x140774010 | 未实现 | render 边界接线 |
| maintain_stone_column_colliders | 0x14078a6e0 | 未实现 | rapier3d 0.22 已入 workspace；country-core native-physics 后端暴露 |
| DoorMeshInProgress::add_quad | 0x141214ed0 | G06 覆盖差异 | 原 3 数组 vs country-core 4 数组布局 |

## U13. interaction/toolbar（19 条：判等 4 / 重建 2 / 部分 1 / 未实现 12）

| 函数/条目 | VA | verdict | 关键分歧 / 解锁条件 |
|---|---|---|---|
| place_childen_impl\<XY/YX\> | 0x140fe12a0 / 0x140fe19a0 | 重建核心 + 尾区未实现 | 签名裁决（ParentLayout/ChildLayoutInput stride 0x18）；padding/gap/available/sum/remaining/weight + justify 0..8 判等；尾区 idx>8/RON 路径 + `{cap,ptr,len}` 返回容器需 mei_post_update 裁决 |
| add_toolbar_item | 0x141e4dc40 | 判等（控制流）+ kind 载荷未实现 | 容器 4/5 递归、spawn 8 组件、UiShowIfParentIsInFocus 判等；0x60B/0x20B 记录/fn@0x140fdf9a0/0x140fdf500 语义 U |
| toolbar_alignment | 0x142344560 | 重建分派 + setter 未实现 | 20 项跳表 0x14304e6d8；closure_env_0..13 setter 目标 U；掩码 FIX-A 已补 `&3` |
| toolbar_button_keep_focus | 0x1423414b0 | 判等 | per-widget keep 字节 + 冷却 0.3/重置/步骤 3-4；WidgetByteState 已落库 |
| switch_app_mode_button | 0x142341fe0 | 未实现 | 载荷源 U（SwitchAppModeButton 16B {ptr,len}）；`(String,SettingsMenuState)` 事件面已由 gap-core-events 闭环 |
| switch_toolbar_category_button | 0x1423427f0 | 判等 | WindowFocusEvent push + per-widget +2 选中；ModeJustChanged 存在门/未选中门已补 |
| tod_v2_button | 0x142342f30 | 判等 | per-widget +2=visible 写面；`!=0xd` 过滤已登记 |
| ui_animation_state_update | 0x14233b3f0 | 判等 | 公式 + 字节判别；UiAnimationState 16B / UiInteractionState 4B 布局判等 |
| toolbar_auto_hide | 0x142343a10 | 未实现 | 常量/公式已取证；SystemParam 接线 U；截断区未取证 |
| ui_sounds | 0x14233a7f0 | 未实现 | 9 SystemParam 已证；~18 case 截断（Bad instruction data） |
| ui_slider | 0x1423486d0 | 未实现 | ~28 case 截断；MainCamera/BrushSizes |
| render_taffy_ui_to_widgets | 0x142338990 | 未实现 | Query init/WidgetParams 链/format_inner 已证；13 参接线 U |
| mei_pre_update / mei_post_update | 0x1406842e0 / 0x1406884c0 | 未实现 | 输入→handler 状态机 / render 两遍 + taffy 最小尺寸 |
| widgets 构建器/视图/tweaker 族 + taffy 族 | — | 未实现 | Label/MenuButton/Image/Pill/Frame/TweakerContent；taffy 0.3.13 已入 workspace |
| photomode 族 | 0x14234b010/0x1423486d0 等 | 未实现 | 滑块跳表截断 + MainCamera/BrushSizes |
| menu/load/corrupted/credits/report_bug + SingleFileServer | 0x14234d690 / 0x14286bf40 | 未实现 | tiny_http/image/open 已入 workspace；mei DSL |
| text 渲染族 | 0x141e9ce50 等 5 | 未实现 | tg-text TextRenderer 接线 + SSBO 名 |
| 26× register_required_components + 共享 UI 类型 | — | 部分落库 | UiAnimationState/UiInteractionState/WindowFocusEvent 已落；UiNav/HistoryButton 布局 U |
| ToolbarPlugin / Ui2dPlugin build | 0x14068d610 / 0x14068dc70 | 未实现 | 9 事件 + 系统集合；系统恢复后接线 |
