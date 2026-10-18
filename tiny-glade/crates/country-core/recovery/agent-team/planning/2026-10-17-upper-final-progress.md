# 上层 13 模块最终进度（2026-10-17 晚）

> 角色：coordinator-quickref（只读汇总，未改 src、未重读二进制）。
> 来源：`reports/{wave2,wave2b,wave3,gap}/**` + `reports/integration/{coregap-1,coregap-2,purelogic,external-dep}/**` + `reports/host-wiring/**` + `planning/2026-10-17-unimplemented-ledger.md`。
> 目录中不存在 `reports/consume`、`reports/remaining`；`reports/unlock/nani` 仅有原始证据（无 RESTORE.md），NaniBundle 仍按"在途"计。
> 口径：**判等**=逐语句等价且有独立复核；**重建**=控制流/写路径已落库，但含已登记 I/U 分歧；**部分**=判等面与未实现面并存；**未实现**=诚实空体/近似体并写明解锁条件。
> 计数：族条目按组计（如 decorator ~146 业务函数、toolbar widgets/taffy 族、wall-constructor 各族）；按函数面展开的估算另列。

## 一、总览

| 指标 | 数值 |
|---|---|
| 模块数 | 13 |
| 速查表条目（行） | 137 |
| 判等 | 34（24.8%） |
| 重建 | 25（18.2%） |
| 部分（判等面 + 未实现面） | 3（2.2%） |
| 未实现 | 73（53.3%） |
| 本地等效 / DIVERGE | 1 / 1 |
| 判等 + 重建 + 部分 | 62（45.3%） |
| 剩余登记（未实现 + 部分） | 76 行 |
| 剩余按函数面展开（估算） | 约 305 函数（decorator ~146、wall-constructor ~69、toolbar ~90；其余模块 0-10） |

> 结论：13 模块的**已落库业务面**集中在 onboarding（22 重建 + 4 判等）、tree-tool（4 重建）、timberframe（6 判等 + 2 重建）、critters（7 判等 + 2 重建）、clutter（4 判等 + 1 重建）。剩余量高度集中在 decorator、wall-constructor、toolbar 三个大模块，且绝大多数阻塞点是**跨域类型/渲染边界/宿主接线**，不是域内逻辑未解。

## 二、逐模块进度

图例：`█` 判等，`▓` 重建/部分，`░` 未实现；百分比按行级条目。

| 模块 | 条目 | 判等 | 重建 | 部分 | 未实现 | 其它 | 判等率 | 状态条 | 还原进行中 / 关键剩余 |
|---|---:|---:|---:|---:|---:|---|---:|---|---|
| U1 eco/glade-border | 3 | 1 | 1 | 0 | 1 | 0 | 33% | █▓░ | display_glade_border 缺 DebugLine::show + stream 承接/唤醒 |
| U2 eco/forest | 5 | 1 | 1 | 0 | 2 | 1 本地等效 | 25% | █▓░░ | manage 构造形状源 ShapeParameters；handplaced 布局；gothic_hat_bones 未取证 |
| U3 eco/tree-tool | 9 | 0 | 4 | 0 | 5 | 0 | 0% | ▓▓▓▓░░░░░ | 4 重建（Insert T2 / Move undo T3 / slider3d 分歧 15/16）；5 未实现 |
| U4 eco/critters | 18 | 7 | 2 | 2 | 6 | 1 DIVERGE | 39% | ███████▓▓▓░░░░░░ | wren/butterfly 部分；ui_pet 核心重建；6 轨迹/鸟点/鱼未实现 |
| U5 state/water | 5 | 1 | 0 | 0 | 4 | 0 | 20% | █░░░░ | draw_water 注册判等；closure$0/signifier 布局/WaterAudioGrid 待 core |
| U6 state/timberframe | 9 | 6 | 2 | 0 | 1 | 0 | 67% | ██████▓▓░ | rect/circle 重建+写路径；GAP=RenderInstance/主环朝向/bit0；unload 阻塞 |
| U7 state/roof | 8 | 1 | 2 | 0 | 5 | 0 | 13% | █▓▓░░░░░ | simulate 判等、highlight/gpu_flush 重建；trigger volumes/ui_edit/ridge 族未实现 |
| U8 state/color | 7 | 0 | 3 | 0 | 4 | 0 | 0% | ▓▓▓░░░░ | show_color_options/色 SSBO/烟值重建；color_menu/roof_customization/图标族/replay 未实现 |
| U9 state/decorator | 14 | 2 | 6 | 0 | 6 | 0 | 14% | ██▓▓▓▓▓▓░░░░░░ | 6 部分/重建；~146 业务函数 + replay/visual/apply 族未实现 |
| U10 interaction/clutter | 13 | 4 | 1 | 0 | 8 | 0 | 31% | ████▓░░░░░░░░ | on_windows/load/add_region/dirty_walls 判等；populate 重建；8 族未实现 |
| U11 interaction/onboarding | 11 行 / 32 函数 | 4 | 1 行（22 函数） | 0 | 6 | 0 | 36% 行 / 12.5% 函数 | ████▓░░░░░░ | 22 重建结构/flags 已核；6 个 mei/相机/geo_src 函数未实现 |
| U12 interaction/wall-constructor | 16 | 3 | 0 | 0 | 13 | 0 | 19% 行 / ~4% 函数面 | ███░░░░░░░░░░░░░ | BrickRowSine/merge/resolve case0 判等；13 族未实现（函数面 ~69） |
| U13 interaction/toolbar | 19 | 4 | 2 | 1 | 12 | 0 | 21% 行 / ~6% 函数面 | ████▓▓░░░░░░░░░░░ | place_childen/alignment 重建；add_toolbar_item 部分；12 族未实现（函数面 ~90） |

> 判等率按行级：overall 34/137=24.8%；U12/U13 行级比例偏高是因为族条目按组计，函数面比例更低（已并列）。
> onboarding 行级 4/11，但按函数面 4/32=12.5%，判等+重建 = 26/32 = 81%。

## 三、按模块的还原进行中清单

### U1 glade-border
- 进行中：closure$1（builder 链 + 2×insert 判等；唤醒未接线）。
- 判等：feedback_input_out_of_border。
- 剩余：display_glade_border（2 分歧：DebugLine::show、UiSignifierStream 承接 SignifierFactory/closure）。

### U2 forest
- 进行中：manage_tree_prefabs（门控判等；构造形状源未实现）。
- 判等：forest_tree_animation（+0x44 标志、DespawnPrefabCmd 载荷 2 缺口）。
- 剩余：handplaced_tree_animation、get_gothic_hat_bones；Aabb3 3D 为本地等效标注。

### U3 tree-tool
- 重建 4：process_tree_cmds（Move/Delete 判等，Insert T2）、replay_tree（时序/undo 判等，T2/T3）、ui_move_tree（载荷判等，slider3d 近似）、convert_preplaced_trees（Wait flag/墙方向判等，insert_starting_edit 待接线）。
- 剩余 5：hide_trees、ui_place_tree、place_tree、ui_delete_tree、tree_move_bend。
- 已解锁：`HistoryTransactions::ledger_mut()` 已在 gap-core-events 落库；tree-tool 侧转换只需改签名接线。

### U4 critters
- 判等 7：move_critters、critters_avoid_collisions、sheep_walk_animation、sheep_pet_animation、set_butterfly_velocities、DuckSpawnLocations::new、FishSpawnLocations::new。
- 部分 2：update_wrens（REVIEW 需修 2 已修）、update_butterfly_states（state0/2 U）。
- 重建核心 2：ui_pet_sheep、ui_pet_ducks（UI 交互 API 缺）。
- 剩余 6：despawn_fish、plan_duck_trajectories、move_ducks_along_trajectories、update_decorator_bird_spots、update_roof_bird_spots、ducks_react_to_being_inside_buildings；CritterGoal DIVERGE。

### U5 water
- 判等：draw_water 注册（add_water_cpu_systems + register_water_events）。
- 剩余：draw_water::closure$0、橡皮 signifier 记录布局、update_water_audio_grid max 参数、WaterAudioGrid 语义。

### U6 timberframe
- 判等 6：rect/circle 的相位与计数面、split_height_into_skirts、rotation_from_normal、orientation_mat3、beam_count_audio 主体。
- 重建 2：construct_timberframe_rectangle、construct_timberframe_circle（梁写路径落库）。
- GAP：RenderInstance 字段序、ResMut 库面、bit0 代理、主环 Quat::IDENTITY、PlayTimberframeSound size/id、GameClock 布局、timberframe_on_glade_unload（bevy_state）。

### U7 roof
- 判等：simulate_roof_tile_for_audio。
- 重建 2：highlight_roof（数据流判等）、gpu_flush_buffers（结构落位，gpu_flush 待 SparseSsbo）。
- 剩余：calculate_roof_trigger_volumes、ui_edit_roof/do_ui_edit_roof、ridge_dir/ridge_length/focused 族、switch_roof_type mesh ops、RoofSound/RoofTileSimulation/GameClock 边界。

### U8 color
- 重建 3：show_color_options（case 0/1 已按 REVIEW 修正）、init_color_ids_buffer + roof color write（SSBO 落库）、set_flag_connection_and_chimney_smoke（ChimneyExtra 已落）。
- 剩余 4：color_menu、set_roof_customization、图标/动画/径向菜单族、replay_color。

### U9 decorator
- 判等 2：gothic_half_dormer_radius、clear_out_* 注册面。
- 重建/部分 6：maintain_dormer_tiles（游标写序已修）、window_lights、chimney_smoke_anim_thickness、replay_decorator、visual build、create_or_restore_decorator_backup、calc_gothic_half_dormer_profile_ws。
- 剩余 6：impl$5、apply_decorator_op、decorator_record_history、get_gothic_hat_bones、~146 业务函数、以及上述部分项的 U 面。

### U10 clutter
- 判等 4：add_autoclutter_on_windows、load_clutter_meshes（结构）、add_region/mark_dirty/flush_candidates、dirty_walls。
- 重建 1：populate_autoclutter_regions（分派+槽表 + 5 项 U）。
- 剩余 8（5 组）：manage_autoclutter_entities、read_bush_positions_from_gpu、animation/放置族、merlon/stitch 族、candidates/subscribe/tick/reset 族。

### U11 onboarding
- 判等 4：tutorial_completion、hints_startup、hint_camera_enable_condition、hint_camera_progress_update。
- 重建 22：见速查表 U11（结构/flags 逐字节核验；hint_roof_shape 完成路径已按 REVIEW 修正）。
- 剩余 6：hint_orchestrator、hint_draw、render_hints、hint_position_update、hint_focused_edit_hint、hint_clutter_focused_edit_hint。

### U12 wall-constructor
- 判等 3：BrickRowSine（random_f/deform）、merge_nearby_proposals、resolve_hole_overlap case0（+残留定向用例）。
- 剩余 13 族：half_timbering、arches_n_walls\<true\>、pillars 宿主 4-6、ArchWalker×4、支撑梁 5、segmentation 3、立面支撑整族、门/栅栏/地板/楼梯族、半木/抹灰族、utils 近似体族、gpu_*、maintain_stone_column_colliders、DoorMeshInProgress::add_quad。

### U13 toolbar
- 判等 4：toolbar_button_keep_focus、switch_toolbar_category_button、tod_v2_button、ui_animation_state_update。
- 重建 2：place_childen_impl（核心）、toolbar_alignment（分派结构）。
- 部分 1：add_toolbar_item（控制流判等，kind 载荷未实现）。
- 剩余 12 族：switch_app_mode_button、toolbar_auto_hide、ui_sounds、ui_slider、render_taffy_ui_to_widgets、mei_pre/post_update、widgets/taffy 族、photomode、menu/load 族、text 族、26 组件、Plugin build。

## 四、跨模块解锁线（按影响面排序）

1. **NaniBundle / DrawableMeshBundle 结构体落库**（`reports/unlock/nani` 证据已备，未落库）
   解锁：decorator `WindowVisualEntityBuilder::build`、roof `switch_roof_type` mesh ops、toolbar spawn 族、signifier build。
2. **RoofTileSsbo `SparseSsbo`（CHUNK_COUNT=0x2000）+ rhapsody 回调注册表**
   解锁：decorator `maintain_dormer_tiles` GPU 退役、roof `gpu_flush_buffers` 的 gpu_flush 5 行、render 存储形态。
3. **DebugLine::show 0x140b04f70 + UiSignifierStream closure 槽位/唤醒（vtable 0x142efa150）**
   解锁：glade-border `display_glade_border`、water `draw_water::closure$0`、collision visualize 族。
4. **InsertTreeCmd 0x48 + TreeEdit 旧位置 edit+0x14/+0x1c**
   解锁：tree-tool `process_tree_cmds` Insert、`replay_tree` T2/T3、`place_tree`；history 组持有 tree_edit.rs。
5. **PublicWalls 密集窗/屋顶桶表（stride 0xa0/0x48/+0xf8/+0x110）**
   解锁：decorator `window_lights`/`chimney_smoke` 收集主循环、color `set_roof_customization`、onboarding 墙高查询。
6. **ReplayCommand Decorator 变体 + ReplayBucket Arc\<[DecoratorId]\> cull 表 + 0x30B 内嵌时序**
   解锁：decorator `replay_decorator`、color `replay_color`。
7. **geo_src_to_edit_subject + legacy_raycast 命中码**
   解锁：onboarding `hint_focused_edit_hint`/`hint_clutter_focused_edit_hint`、tree-tool `ui_place_tree`。
8. **mei DSL + render_taffy_ui_to_widgets 13 参接线**
   解锁：onboarding `hint_draw`/`render_hints`、toolbar `render_taffy_ui_to_widgets`/widgets 族/mei_pre-post。
9. **DisplayHintRequest 完整布局（placement@+0 / id@+0x49 / flags@+0x48）+ 事件载荷字节**
   解锁：onboarding 剩余 hint 展示/完成分支的逐语句判等（现为 seam 0）。
10. **共享 UI 类型（UiNav/HistoryButton 布局、26 组件）+ taffy setter closure_env_0..13**
    解锁：toolbar toolbar_alignment setter、add_toolbar_item kind 载荷、26 register_required_components。

## 五、结论与口径说明

- 上层 13 模块**域内可判等/可重建的部分已基本落库**：onboarding 81%、timberframe 89%、critters 61%（判等+重建+部分）、tree-tool 44%、clutter 38%、decorator 57%（含部分）、color 43%、roof 38%；整体行级 45%。
- 剩余 76 行登记中，**无一项是"逻辑未解"**：均标有缺类型、缺渲染边界、缺宿主接线或截断跳表取证路径。其中 decorator/wall-constructor/toolbar 的成族条目展开约 305 函数，是后续专项批次的主体。
- 本文件仅汇总已落盘报告，未重跑 `cargo check`/测试；各报告记录的编译窗口状态为：tree/glade-border、timberframe/roof、onboarding、toolbar、critters/clutter/decorator、utils/toolbar 均在时间窗内 0 error（并发 rhapsody WIP 阻断已由归属组闭合）。
- 与核心模块进度（`2026-10-17-final-progress-v2.md` 的 88-100%）口径不同：该文件覆盖 country-core 基础域；本文件覆盖 country-core 之上的 13 个上层业务模块，故整体判等率显著更低属预期。
