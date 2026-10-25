# country-core 还原进度终版图 v2(2026-10-17 晚,并发修复后密集波次收口)

> 本轮(WallHistoryEdit 解锁后)完成:walls 判等清空 + 全程重建、collision 收口、camera/ui/time 闭合、
> audio/save/history/terrain/roofs+stairs/ivy/input/session/world_raster/UI域余量全部推进。
> 基线:全库 **2024 测试 / 0 失败**(v1 时为 1842);native-physics 下 collision 20/0。
> 口径:进度 = (判等 + 重建闭合 + seam 落地) / 业务函数面;剩余多数为**跨域依赖倒置/宿主接线**登记。

```
模块                进度                                 本轮增量
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
input              ███████████████████▉  99%   8 需重做全修(Steam 门/回放 seek)
steam              ████████████████████ 100%   —
render_loop        ███████████████████▉  99%   Env36/设备面;RoofTileSsbo 登记
world_raster/paths ███████████████████▉  99%   as_float_grid 修正+draw_path/record/preview/build
music              ████████████████████ 100%   —
ivy                ███████████████████▉  98%   9 修正(grower/spawner/pruner/GPU)
garden+sheep       ███████████████████▉  98%   2 修正(布局/绑定/CmdCheckForSheep)
session            ███████████████████▉  98%   5 修(file_list/version2/create/delete/build)
camera_rig         ████████████████████ 100%   4 判等(load_parameters 轨道四元数)
ui_mode            ███████████████████▉  98%   12 重建(WidgetParams/signifier/brush_preview)
time_of_day        ███████████████████▉  98%   4 重建(13 参全链/suncalc)
history            ██████████████████▉   96%   replay_wall 17 操作/replay_edit/replay_mode/pop_bucket/edit 载荷
save+meta          ██████████████████▉   97%   19 重建(commander/load_game/save_thread/快照链)
roofs+stairs       ██████████████████▉   96%   13/15(noise oracle 验证/replay undo 源)
terrain+clutter    ██████████████████▉   95%   12 重建+删编造事件(PlayTerrainSound/InstaCreate*)
audio              ██████████████████▉   95%   21 重建(wall_audio_ui/Drop/各系统)
walls              █████████████████▉    88%   判等全清(39+130+48);重建 8+49+9+1;eraser/restore_decorators/颜色库
collision          █████████████████▉    86%   world.rs 9 判等/rt 设备面/find_mesh 装配/三角相交
UI域余量           ██████████████████▉   93%   prefab 7+4 胶水/grading 5
```

## 本轮 closed 关键函数

- **walls**:on_wall_split/merge_with/on_walls_merged/contains_shape/expand/snapping_*/CompoundEditBuilder、
  create_circle/ui_create_circle/snapping_to_ends/telegraph/shape_placement/preview_snapping_line、
  load_state/new_wall_from_curve/max_y_gable/snap_to_roof_ridge/archivist 8、create_rectangle 继承+
  get_building_and_roof_colors、restore_decorators 5 段、eraser finish 3-map
- **collision**:world.rs 9 需重做全判等;rt 设备面布局;find_mesh 域内装配+三角相交;
  BirdNestCandidateLocation(0x40);ColliderDiff/remove_collider;raycast 句柄区
- **history**:WallHistoryEdit 0xf8B 17 变体;replay_wall 17 操作;replay_edit Color/Decorator;
  replay_mode(0.2s);finish_replay;replay_pop_bucket;ColorEdit 14 键;send_state_diff_events/add_decorator_diff
- **save**:19 项(commander case0 极性/load_game fence/save_thread Shutdown/execute_save_cmd/
  快照捕获/write_to_disk version2)
- **audio**:21 项(wall_audio_ui 跳表/AudioInstance Drop state 门/water ice/tree 3 loop/trigger/羊 0x38+)
- **roofs+stairs**:noise(finalizer 链,RNE 0.5487866)/intersection y==floor/replay undo 源/StairsRemovedSupports
- **camera/ui/time**:camera_rig 4、ui_mode 12、time_of_day 4
- **terrain/ui_clutter**:12(删 PlayTerrainSound/InstaCreate*/16 位脏掩码编造;补 stairs 桶/哨兵)
- **ivy**:9(into_world_space Y/spawner 门来源/pruner 7 参/GPU IvyFlower)
- **input/session/world_raster**:8+5+5

## 剩余登记(全部为跨域/宿主依赖,非域内逻辑缺口)

### A. 依赖倒置(下游模块宿主面未恢复)
- walls 24:edit_rectangle/ui_create_rectangle→PreferredNodeAlignment(system_decorator)+AlignKind 24B;
  finish_edit_rectangle→history MorphPremadeShape 载荷;wall_roof_hashmap→AutoClutterObject(system-clutter);
  6 UI 系统→UiWallHeight/UiState80 宿主+ui_state Display;history_finish_recording(history 域)
- walls 17:14 build 闭包(render-host builder/vtable)+ create_rectangle 第二条 Color history(Color owner)

### B. 渲染/设备/第三方
- collision:update_rapier_colliders 语句级(decompile-hardpoint 不可逆;13 段结构判等已兜底);
  BrickTransformSsbo 铺砖、obvhs 未入 workspace、barycentric/法线修正、registry free-list
- roofs:roof_uv×2(GlobalId 两级表+arena)、MagicSupports/reset_edge(sparse writer)
- render:RoofTileSsbo 存储形态
- UI:7 项 seam(RtMesh 矩阵/RT Resource/atlas cull/PrefabWatcher/PrefabPart/GradingLuts 纹理)

### C. 宿主接线(验收层)
- 全局调度(PreStartup/OnEnter/OnExit)、G1-G6 资产/音频组、完整快照往返、init 全链
- input:工具栏 modes 裸指针克隆/immediately_finish_interpolating/Replay sheep 回调/update_cursor_position 内联
- save:域 Query/宿主注入/28 系统 SubApp 槽位/AutoSaveState +0x28/+0x120
- audio:signifier 21 参数/6 事件流、building/decorator 表布局、firstperson SsboId11

## 台账指针

- 总账: planning/2026-10-17-logic-master-ledger.md
- SOP: planning/2026-10-17-recovery-sop.md
- 域速查: planning/2026-10-17-domain-quickref.md
- 各域本轮报告:reports/<组>/<模块>/*-REBUILD*.md
