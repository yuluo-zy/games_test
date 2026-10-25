# 逐函数逻辑判等总账(2026-10-17,全 14 verifier 落盘)

> 用户硬标准:结构/签名≠实现,函数内部逻辑必须逐函数 fresh 反编译判等。
> 每模块 LOGIC.md 在 reports/<组>/<模块>/LOGIC.md;协议 planning/2026-10-16-logic-verify-protocol.md。

## 总账

| 模块 | 判等 | 未实现-需重做 | 未实现(无 body) |
|---|---|---|---|
| collision | 4 | 9 | ~66 业务+intersect_meshes 21+visualize 6+rt 13 |
| input | 20 | 8 | 0 |
| ivy | 13 | 9 | — |
| walls | ~26 | 11 | ~240 待判等+未实现登记 |
| terrain | 9 | 7 | 注册/消费端 |
| ui_clutter | 1 | 5 | — |
| world_raster/paths | 26 | 1 | draw_path/record_path_history 等 |
| camera/time/ui_mode | 29 | 21 | 3(focus/lens/mode) |
| session+music+camera_rig | ~34 | 10 | — |
| audio | ~31 | 21 | — |
| garden+sheep | 2 | 2(+3 分歧) | — |
| save+meta+startup | 31 | 19 | — |
| roofs+stairs | 53 | 17 | wall_platform 5 |
| render_loop | 34 | 0 | 3 批(闭包/nani/ice 待续) |
| **合计** | **~313** | **~155 需重做** | ~300+ 未实现登记 |

## 真实现模块(判等占比高)

- **world_raster/paths**(26/1)、**render_loop**(34/0)、**music_player**(8/0)、**roofs+stairs**(53/17 主链判等)、
  **EditType 判别与 History 游标/快照/录制链**、**calculate_input_delta 逐槽**、**draw_garden**。

## 未实现-需重做的主要落点(按模块,详见各 LOGIC.md)

### 系统性模式(跨模块反复)
- **panic-vs-return 语义反**(ivy wall 门、input Load 失败、history History::add 校验)
- **门控极性反**(input Steam 门、save commander_update case0、ui_draw 画布门)
- **数据流/载荷假**:edit 载荷平面近似(ColorEdit/RasterEdit/TreeEditData/DecoratorHistoryEdit)、
  AutoSaveState 布局、Sheep 组件绑定错位、RoofNoise hash 链编造
- **事件/命令自造**:PlayTerrainSound 事件系、PlayClutterSoundCmd 布局、HistoryRecordingFailed 分支、
  execute_save_cmd unsaved 死链、fence 释放被丢
- **调用链错**:collision raycast 算法(无 toi 比较/mask u32)、intersects_any Qbvh、render 委托、
  CameraMatrices 维护、snapping 全状态发布
- **返回类型/结构不符**:RaycastWorldResult ~0x40B vs Option、session_file_list PathBuf、
  replay_stairs HashMap<StairGraphEdge,()> vs <u32,u32>

### 每模块关键需重做函数(摘)
- **ivy**:into_world_space/grower/spawner/pruner/is_point_still_valid/send_to_gpu×2/intersect
- **input**:translate_input/control_input_replay/drop_oldest_sync_point/input_record_replay/
  reset_input_state/record_and_replay_for_trailers/do_input_replay_ui
- **walls**:on_wall_split/merge_with/on_walls_merged/restore-take_snapshot/contains_shape/expand/
  snapping_maintain/snapping_to_segments/query_with_aabb_clamped/CompoundEditBuilder
- **terrain/ui_clutter**:edit_terrain_stroke/terrain_stroke_changed/terrain_hover_idx/ui_draw_terrain/
  ui_edit_terrain/ui_terrain_focus_mode/sample_at_world_xz/rmb_autoclutter/ui_convert/debug_spawn/
  decoclutter_show_pins/ui_focus_decoclutter
- **camera/ui**:load_parameters/eye_position/eye_forward/xz_projective/current_dyn/time_of_day 主系统/
  widget_params 方向/signifier 环形流+create+animation/brush_preview/slider3d
- **session/music/camera_rig**:session_file_list/delete_session/build/NextSession::create/write_to_disk/
  load_parameters/current_dyn/music_audio_system
- **audio**:wall_audio_ui 全 kind/AudioInstance Drop/stop_on_drop/sheep kind 分派/building 7 loop/
  water ice/tree 驱动/trigger/terrain set_parameter/terrain_stairs/multiple 空壳
- **save/meta**:commander_update case0 门控/load_game fence 释放+clamp/save_thread Shutdown/
  flush_deferred sync/save_game_system 主体/execute_save_cmd/AutoSaveState 布局/VA 错锚
- **roofs/stairs**:roof_noise hash/roof_shape_at_intersection_y/replay_stairs/
  StairsRemovedSupports/structural_graph clear 错锚/MagicSupports/disconnect 单端点
- **history**:History::add Wall 校验/edit 载荷 5 个/replay_stairs 0.05 下限/finish_replay/replay_mode/
  replay_wall 17 操作跳表/history_process 第二事件流

## 下一步:按真实控制流重建轮

1. 每模块按 LOGIC.md 分歧点,以 fresh 反编译重建真实控制流,逐语句实现(不采信旧 RESTORE)。
2. 重建后逐函数再判等(同协议),直到判等。
3. 已判等函数不动;未实现(无 body)单独分批。
