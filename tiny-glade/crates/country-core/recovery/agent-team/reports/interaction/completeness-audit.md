# 库内完整性审计 — interaction 组(逐函数功能完整性核验)

审查员: reviewer-interaction | 日期: 2026-10-16
基线: 全 lib 编译 1709/1(唯一败 geometry/sphere 既有精度);inventory.json 逐模块原函数清单。

## 判定口径

- **implemented_verified**:Rust 实现经机器码/反编译/实测字节核验,行为一致。
- **implemented_pending_evidence**:实现主体存在,关键子项待取证(诚实登记)。
- **glue_legitimate**:derive/serde/Bevy 注册/转发器/SystemParam 胶水,直接复用原依赖。
- **missing_in_library**:原函数无对应 Rust 实现(登记待续)。

## 假实现扫描(协调器 4 项 + 全库 grep)

全库 `grep "panic!(\"TODO|not yet implemented"` 仅 4 处,逐一裁决:

| 位置 | 内容 | 裁决 |
|---|---|---|
| `render/pass_builder/compute_pass_builder.rs:689` | `panic!("TODO?")`(External 分支带视图) | **合法文档化守卫**——doc:637 契约"视图必须为 None(否则 panic!)";External-with-view 为记录到的未处理路径,非冒充完成 |
| `render_loop/draw_passes/draw_lists.rs:408/413` | `todo_vulkan_1294/1304` = `panic!("not yet implemented: TODO(vulkan)")` | **合法**——closure_env$58/59 对应**原版自身** `todo!("TODO(vulkan)")`(normal_debug_shader / ivy_debug_lines_shader 调试着色器闭包),忠实复现原 todo |
| `render_loop/session_selection/mod.rs:507` | `panic!("TODO")`(carousel.active 必须非 0) | **合法**——doc:142 "O:源码行 135 panic!("TODO")",原版自身 panic,忠实复现 |
| `render_loop/compute_passes/upload_world_raster.rs:127` | `format_suffix` 其它格式 `panic!("not yet implemented: Format(..)")` | **合法文档化路径**——doc:9 原版 format 0x4c→"r16f"/0x61→"rgba16f"/其它→todo!,**先于任何状态修改**;有 `#[should_panic]` 测试 |

**结论:无假实现冒充。** 4 处 panic 均为忠实复现原版 todo/panic 或带测试的文档化守卫。

## 逐模块对账与库内完整性

### systems/backend/input(9 业务)
| 原函数 | Rust | 裁决 |
|---|---|---|
| calculate_input_delta@1416feeb0 | calculate_input_delta.rs | **implemented_verified**(Shape A/B 逐槽 fresh 机器码) |
| translate_input_to_cursor_moved@140b17a80 | translate_input_to_cursor_moved.rs | **implemented_verified**(PDB 签名+调用者+机器码,捕获 tag0/2) |
| input_record_replay@1409423d0 / advance@140942280 / reset@140942b00 | input_record_replay.rs / reset_input_state.rs | **implemented_pending_evidence**(状态机/panic/clamp 对;帧 serde/try_load/seek 待取证) |
| control_input_replay@140941a30 | 同 | **implemented_pending_evidence**(Sync/Load 结构;同步点 seek 待取证) |
| translate_input@1417cd0a0 | translate_input.rs | **implemented_pending_evidence**(结构+绑定表;动作↔槽映射/Steam 待取证) |
| record_and_replay_input_for_trailers@14083c620 / do_input_replay_ui@14083cdd0 | input_replay_for_trailers.rs | **implemented_pending_evidence**(键门/串/面板结构;文件 serde 待取证) |
| update_cursor_position(inline) | update_cursor_position.rs | **implemented_pending_evidence**(I 推断) |

**库内完整性**:主体函数对得上,动作↔槽映射、Steam、帧 serde、同步点 seek 为登记待取证。
**遗留**:库内可闭合=绑定映射/seek/帧 serde;外部依赖=systems::steam 全链、录制文件与
save/session 格式协调、FMOD。

### systems/terrain(12 业务)
| 原函数 | Rust | 裁决 |
|---|---|---|
| terrain_stroke_changed@140a03ba0 / edit_terrain_stroke@14094e000 / delayed_terrain_changed_msg@140aabdd0 + on_terrain_modified@140aabdc0 | draw/edit/delayed | **implemented_verified**(fresh 反编译 + 常量 20.0/1.0/0.001) |
| history_record_terrain@140ac9770 / terrain_hover_idx@140aff720 | deferred_terrain_history.rs / terrain_hover_idx.rs | **implemented_verified**(哨兵状态机 / 0x421 门+哨兵+水位+距离) |
| ui_draw_terrain@140a9fdc0 / ui_edit_terrain@140aef1f0 | ui_draw / ui_edit | **implemented_pending_evidence**(门+生产面+点集指纹;widget/消费面待取证) |
| ui_terrain_focus_mode@1409a4fb0 | ui_terrain_focus_mode.rs | **implemented_pending_evidence**(门+raycast 距离门骨架;switch 表 U) |
| add_terrain_systems@1409455f0 / insert_terrain_resources@1409457c0 / add_terrain_events@140945a20 | mod.rs 适配 | **implemented_pending_evidence**(register_terrain_events 6 事件;完整注册待域系统并入) |

**库内完整性**:3 CPU 系统 + 历史 + hover + 回读链 verified;UI 消费面/focus-mode 主体待取证。
**遗留**:库内可闭合=EditType::Terrain 真 tag、focus-mode switch 表、ui_draw 消费面、脏掩码
位序、EditCommand 字段序;外部依赖=MaskSSBO 资源已闭合(卡7/卡B)、宿主 UI 画布 widget。

### systems/ui_clutter(8 业务)
| 原函数 | Rust | 裁决 |
|---|---|---|
| history_record_auto_clutter@1409a9160 | defered_autoclutter_history.rs | **implemented_verified**(decomp 核对) |
| handle_rmb_autoclutter@140976ca0 | rmb_autoclutter.rs | **implemented_pending_evidence**(删除+无条件历史结构;SIMD 查找/旋转公式已按 RE-ANALYSIS2;实体位集待取证) |
| ui_convert_autoclutter_to_decoclutter@14099ca60 | ui_autoclutter.rs | **implemented_pending_evidence**(0x48B 布局登记;quat 输入待接线,旋转置 0 诚实) |
| debug_spawn_all_clutter@1409a1f50 | debug_spawn_clutter.rs | **implemented_pending_evidence**(螺旋+实测类型表/角度;消费端待接) |
| decoclutter_show_pins@1412357c0(+register/clone) | decoclutter_show_pins.rs | **implemented_pending_evidence**(遍历→事件结构;实体切换机制缺) |
| ui_focus_decoclutter@1409a92c0 | ui_focus_decoclutter.rs | **implemented_pending_evidence**(门存根;状态机主体缺) |

**库内完整性**:删除/历史链 verified;拾取/转换/spawn/pins/focus 为结构+事件面。
**遗留**:外部依赖=DecoratorStorage 实体机制(宿主)、图钉实体切换、ui_focus 状态机主体。

### systems/camera_rig(47 业务)+ systems/time_of_day(2 业务)
**camera_rig**:load_parameters@140911520 / store_parameters@140911710 / CameraModeStates::
current_dyn@1409149d0 / mode/focus/lens / target_fov / immediately_set / eye_position/eye_forward/
xz_projective_rotation / serde_default — **implemented_verified**。zoom@140911280 /
init_walk@14090e510 / match_free@14090e6f0 / new@14090ec70 / immediately_finish_interpolating@
140911b50 / body 更新族(update/rotate_yaw_pitch/constrain/translate 转发) — **missing_in_library**
(登记后置)。
**time**:time_of_day@140a73460(纯算法+TimeOfDayState)/ remap@140a74790 — **implemented_verified**;
suncalc::get_position/光照/音频 — **missing_in_library**(外部依赖: suncalc crate、FMOD、光照资源)。
**遗留**:外部依赖=宿主渲染循环、suncalc、FMOD、相机 body 更新算法(大件后置)。

### systems/ui_mode(widget_params/slider3d/AppMode/mode_manager/signifiers/brush_preview)
widget_params 族(0x1409a8e10 默认/0x140985f30 is_none/0x1409a9130 as_mei/…)+ slider3d
project_cursor_onto_line@140a9bd40 + AppMode is_tool_selected@140a86b40(native_tag 全部)—
**implemented_verified**(机器码 0x3cfe 验证)。signifier_create@140b2cfa0/UiSignifierStream
write@140b2cf90 + brush_preview@140865050 + mode_manager@140a86b60/send_mode_just_changed@
140a875e0/is_tool_selected 闭包族 — **implemented_pending_evidence**(环形流 VecDeque 近似、
brush 大小表推断、check_if_can_switch 闭包 U)。ui_update/get_ui_volume/hover_intersection/
debug_visualize_volumes/signifier_animation/mark_for_delete/despawn — **missing_in_library**。
**遗留**:外部依赖=signifier 实体 spawn、UiState 画布、宿主 UI 顶层系统;库内可闭合=
check_if_can_switch 闭包、signifier_animation。

### systems/wall(133 业务,57 文件)
construct_wall_bricks/adjust_wall_height/delete_wall/wall_highlight/calc_wall_bend/
history_recording/wall_state 族 — **implemented_pending_evidence**(既有链,一轮核 OK)。
valid_enclosure(45)/snapping/eraser/shapes/wall_compound_edit 完整族 — **missing_in_library**
(登记待续,诚实预算)。**遗留**:外部依赖=与墙组台账衔接;库内可闭合=卡A2 已核对+登记。

### systems/collision(82 业务)
SubworldColliderHandle::new@1408cc620 / RaycastMeshData::new_convex@1408cec10 /
RaycastWorld raycast@1408cc870/legacy@1408cc680 / Raycasting raycast@1408cc2e0+intersects_any@
1408cc320 — **implemented_verified**(R1+R1 fix)。spherecast@1408cc390 — **implemented_pending_
evidence**(桩,待 PounceLight fork API)。flush_colliders/upsert-remove 泛型族/dirty 瓦片
生命周期/intersect_meshes(21)/visualize_colliders(6) — **missing_in_library**。**遗留**:
外部依赖=rapier3d(native-physics 已接,双构建 0 error);库内可闭合=dirty 瓦片
flush/publish/QueryPipeline::update。

### render_bridge / world_raster
ivy GPU(read_storage_buffer IvyFlower/IvyLeaf/IvySegment)/garden pin(draw_raster)/terrain
回读(read_terrain_data+mask)/world raster(upload_world_raster)消费 + GPU 提交(nani+
add_new_with_data)+ startup(add_game_startup_systems@140941110→register_core_startup)—
**implemented_pending_evidence**(消费桥+合同;宿主顶层渲染序外部)。world_raster
record_raster_history@140957990/upload_world_raster_buffers_to_staging@140957bf0 已接 —
**implemented_pending_evidence**。**遗留**:外部依赖=宿主 run_game 顶层渲染循环、ivy/garden
实体→SSBO 48B 逐字段。

## 库内完整性总体判定

- **无假实现**:4 处 panic 均忠实复现原版 todo/panic 或带测试文档化守卫。
- **implemented_verified 覆盖**:input 2、terrain 5+回读链、ui_clutter 1、camera slice1/2、
  time 纯算法、ui_mode AppMode/widget/slider3d、collision R1 核心查询族。
- **implemented_pending_evidence 覆盖**:结构/事件面/骨架(translate_input 绑定、ui_draw 消费、
  rmb 转换、focus 门、signifiers 环形、world_raster)。
- **missing_in_library(登记待续)**:camera body 更新 20+、wall 45+79、collision 170 胶水
  intersect/visualize、ui 顶层系统、roof 视觉方法+ridge、EditType::Terrain 真 tag。
- 各模块均处于**实现层面部分闭合 + 验收层面开放**(消费者闭合/全局调度/宿主接线),与
  前几轮 REVIEW 结论一致;**无一模块可宣布"100% 完成"**。
