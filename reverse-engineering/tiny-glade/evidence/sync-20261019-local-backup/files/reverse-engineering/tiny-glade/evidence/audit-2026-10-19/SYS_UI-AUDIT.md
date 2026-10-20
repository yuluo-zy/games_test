# country-core systems（UI/交互/相机/输入/碰撞/时间天气）逐文件还原度复核

- 日期：2026-10-19
- 性质：只读审计，未修改任何源码。
- 工作目录：`D:/game/tiny-glade/crates/country-core`
- B 骨架：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src`
- 机械统计：`D:/game/reverse-engineering/tiny-glade/evidence/audit-2026-10-19/fn_audit.json`
- 基线文档：`recovery/STUBS-2026-10-19.md`（仅作参考，本报告以源码/调用点和 fn_audit 为准）

## 0. 口径与范围说明

- 按“排除 `render_loop/ wall/ save/ history/ audio/ ivy/ garden/ terrain/` 目录”的口径，`src/systems` 下剩 **65 个 .rs**。其中包含顶层的 `systems/history.rs` 与 `systems/ivy.rs`（它们是文件而非被排除目录）；若把这两个顶层文件也排除则是 63 个。本表保留并单列为“边界文件”，避免与任务给定的 65 个口径冲突。
- 规模：65 文件共 **20,118 行**，**780 个 `fn`**，其中 **238 个 `#[test]`**（另有测试辅助函数）。没有任何 `todo!`/`unimplemented!` 宏；空壳以“语义空壳/缺文件/缺接线”为主，而不是占位宏。
- `fn_audit.json` 的 `file_cov` 是**骨架文件口径**，与当前 crate 的拆分/改名不完全同路径（例如骨架 `camera_rig.rs/normal.rs/free.rs/walk.rs`，当前为 `camera_rig/{mod,main_camera,normal_body,free_camera}.rs`）。本报告的 no_trace 按“模块映射”处理，映射不确定处标“需人工”。
- `in_file` 只代表“该名字/地址能在原文件对上”，**不代表语义已等价**。真正的空壳按源码正文判断（见 §2）。
- 范围内骨架函数记账（75 个骨架文件键、345 条 fn 记录）：
  - `in_file` 156（45.2%）
  - `moved` 51（14.8%）
  - `name_only` 40（11.6%）
  - `va_only` 3（0.9%）
  - `no_trace` 95（27.5%）；其中 51 条是 `closure_env$N>`、serde/泛型 `Type>` 伪条目，**约 44 条是业务向缺口**。

---

## 1. 文件级状态表（65 个文件）

“性质”：声明=仅 `mod`/枚举/事件类型；数据合同=布局/字段/访问器为主；真实行为=实际系统逻辑；混合=类型/合同+行为+测试。
“函数”含 `#[cfg(test)]` 内的测试函数。“水分”只列可定位的空壳/语义缺陷，行号即证据。

| # | 文件 | 行 | 性质 | 函数 | 空壳/水分（文件:行） | no_trace（本模块） | 链/备注 |
|---:|---|---:|---|---:|---|---:|---|
| 1 | systems/arrow_gizmo_animation.rs | 738 | 真实行为 | 64 | 无宏级空壳；56 个 `original_full_loop_N` 是测试 oracle，不是业务 | 0 | 业务函数仅 `arrow_gizmo_animation`；**未注册**，`EnableArrowGizmoStretch` 无生产消费者 |
| 2 | systems/backend/flush_screen_log.rs | 42 | 真实行为 | 3 | 无 | 0 | 日志投递；GUI 读 |
| 3 | systems/backend/gui.rs | 189 | 混合 | 7 | 无 | 0（骨架无此文件） | `configure_gui_consumers` 在 tiny-glade:423 接线 |
| 4 | systems/backend/input/button_input.rs | 37 | 数据合同 | 4 | Deref 4 个 2 行胶水（14-36） | 0（新增拆分） | `KeyButtonInput` 供 time_of_day 消费 |
| 5 | systems/backend/input/calculate_input_delta.rs | 134 | 真实行为 | 7 | 无 | 0 | `InputSystemSet::Resolve` 已注册 |
| 6 | systems/backend/input/input_record_replay.rs | 278 | 混合 | 13 | 无 | 0 | Prepare/Control/RecordReplay/Finish 已注册 |
| 7 | systems/backend/input/input_replay_for_trailers.rs | 71 | 真实行为 | 2 | 无 | 0 | Control；egui 门 |
| 8 | systems/backend/input/mod.rs | 172 | 混合 | 4 | 无 | 0（骨架 mod 无记录） | `register_input_events`/`add_input_cpu_consumers` 两端接线 |
| 9 | systems/backend/input/mod_key.rs | 159 | 混合 | 7 | `disable_snapping`/`is_disable...` 2 行访问器（16-21） | 0（骨架 `systems/mod_key.rs` name_only 2，属搬移） | Derive 已注册 |
| 10 | systems/backend/input/reset_input_state.rs | 73 | 真实行为 | 2 | 无 | 0 | Prepare 已注册 |
| 11 | systems/backend/input/translate_input.rs | 557 | 混合 | 13 | 无 | 0 | Translate 已注册；输入主表 |
| 12 | systems/backend/input/translate_input_to_cursor_moved.rs | 142 | 真实行为 | 7 | 无 | 0 | Translate 已注册 |
| 13 | systems/backend/input/update_cursor_position.rs | 207 | 混合 | 7 | `CursorPosition::new/as_vec2` 2 行（22-27） | 0（骨架同文件 in_file） | Derive 已注册 |
| 14 | systems/backend/mod.rs | 6 | 声明 | 0 | 仅 5 个 `pub mod`；**无插件装配** | 3（`add_backend_systems`/`insert_backend_resources`/`add_backend_events`） | 真缺失装配函数，host 手工散装 |
| 15 | systems/backend/photomode.rs | 111 | 真实行为 | 4 | 仅恢复 `update_photomode_visibility` 前缀；完整 `do_photomode` 未恢复（文件头 1-2） | 0（新增拆分） | `configure_photomode_visibility` 无调用点 |
| 16 | systems/backend/prepare_frame_constants.rs | 500 | 真实行为 | 7 | 无 | 0 | tiny-glade:484 接线 |
| 17 | systems/brush_preview.rs | 223 | 真实行为 | 8 | `register_required_components` 空体 173；bundle 只落库片段 20 | 0 | 系统未注册 |
| 18 | systems/camera_rig/free_camera.rs | 107 | 真实行为 | 8 | 3 个 3 行访问器（48-57） | 0（骨架 free.rs） | 无系统接线 |
| 19 | systems/camera_rig/main_camera.rs | 1005 | 混合 | 44 | `projection` 单位阵占位 402；`main_camera_update` 未实现 411；多个 2 行读写器 | 1 业务（`init_walk_camera`）+3 伪（Error>/Global>/Serializer>） | MainCamera 资源被插/读，但无更新系统 |
| 20 | systems/camera_rig/mod.rs | 201 | 混合 | 7 | 3 个 2 行访问器（47/119/124） | 0（归入 #19） | `init_walk_camera` 全 crate 无实现 |
| 21 | systems/camera_rig/normal_body.rs | 423 | 混合 | 23 | 4 个 tiny 访问器；无占位 | 0（骨架 normal.rs） | zoom/rotate/constrain 真实 |
| 22 | systems/collision.rs | 250 | 混合 | 11 | 插件 build 真实；初始化系统“此处不注册”187 | 0 | `CollisionPlugin::build` 无宿主调用 |
| 23 | systems/collision/intersect_meshes.rs | 697 | 混合 | 24 | 三角/折线已实现；砖铺放、Graph add_node/edge、鸟巢 spec_extend 未接线 98-110 | 0 | 无系统注册/调用点 |
| 24 | systems/collision/query_policy.rs | 92 | 混合 | 5 | `cast_ray` 是 trait，无具体后端装配（44-54） | 0（新增） | 可执行接口，非完整实现 |
| 25 | systems/collision/visualize_colliders.rs | 260 | 真实行为 | 8 | 文档仍写“未实现-登记”11-12，与正文 29/49 行实现不一致；host `show` 未接 39；结构占位 137 | 0 | 未注册 |
| 26 | systems/collision/world.rs | 3213 | 混合 | 142 | 注册表插入接线未落地 237-239；非 native-physics 查询壳 2204；`on_level_reload` 简化为整体 `new` 2414；文档宣称的 `update_rapier_colliders`/`construct_arch_geo` 实际不存在 | 0 | Rapier 查询核心真实；collider 生产者缺失 |
| 27 | systems/debug/mod.rs | 2 | 声明 | 0 | 仅 `pub mod sheep_controls` | 0 | — |
| 28 | systems/debug/sheep_controls.rs | 112 | 真实行为 | 3 | 无 | 0 | core startup 已接线 |
| 29 | systems/history.rs | 554 | 混合 | 17 | 边界文件（目录口径外）；`iter_edges_and_stubs` 空闭包是真实用法 379 | 边界，不计 | history 链已接线；调用 `flush_colliders` 389/445 |
| 30 | systems/ivy.rs | 215 | 混合 | 12 | 边界文件；无空壳 | 边界，不计 | `closest_segment_index` 真实 |
| 31 | systems/main_camera.rs | 31 | 声明 | 0 | 只有 `CameraRotationPivot` 枚举；`main_camera_update`/`paused_main_camera_update`/`reset_camera`/`camera_snapshot`/`build` 未落 | 0（骨架本文件） | 骨架的 5 个 name_only+1 moved 无对应实现 |
| 32 | systems/mode_manager.rs | 493 | 混合 | 19 | 无业务空壳；`esc_menu_blocks` 2 行是判据 | 13（11 closure+3 伪，见 §3） | 两端 startup 均已接线；核心 105 行真实 |
| 33 | systems/mode_manager/types.rs | 332 | 混合 | 6 | 数据合同+转换；`default` 2 行 | 0（骨架 mode_manager.rs 拆分） | `from_native_tag`/`native_tag` 真实 |
| 34 | systems/paths.rs | 934 | 混合 | 24 | `draw_path` 194 行、`record_path_history` 真实；无宏空壳 | 0（骨架 paths/*） | `PathsPlugin` 仅在测试调用；**未接线** |
| 35 | systems/prefab.rs | 1141 | 混合 | 31 | `_gap:u64` 是类型合同非空壳 238；ECS 事件 seam 有登记 | 0 | spawn/update/despawn/hot_reload **均未注册** |
| 36 | systems/steam/mod.rs | 446 | 混合 | 16 | `now_hint()` 固定 0 占位 403-406 | 0 | `add_steam_integration`/`update_steam_inputs` 无调用点 |
| 37 | systems/steam/startup_steam.rs | 41 | 真实行为 | 1 | 无 | 0 | 未接线 |
| 38 | systems/time_of_day.rs | 736 | 真实行为 | 24 | 无占位；`run_time_of_day` 147 行状态机 | 0 | `time_of_day::<S>` core startup、`time_of_day_audio_disabled` tiny-glade 均接线；输出被 prepare_frame_constants 消费 |
| 39 | systems/time_update.rs | 51 | 真实行为 | 1 | 无 | 0 | Clock 已注册 |
| 40 | systems/timelapse_replay.rs | 7 | 声明 | 0 | 只有事件类型；`start_timelapse`/`cleanup_timelapse` 未实现 | 2 | 真缺失 |
| 41 | systems/ui/mod.rs | 6 | 声明 | 0 | 只有 `pub mod resources; pub mod signifiers;`；**`add_ui` 未落** | 0（骨架 add_ui 为 moved，但当前不存在） | UI 插件装配缺失 |
| 42 | systems/ui/resources/mod.rs | 3 | 声明 | 0 | — | 0 | — |
| 43 | systems/ui/resources/slider3d.rs | 146 | 混合 | 6 | `into_drag_state` 2 行 41-43；`project_cursor_onto_line` 真实 | 4（closure_env$0/5/7/21） | 拖拽回调闭包缺；需人工判定业务 |
| 44 | systems/ui/resources/ui_state.rs | 382 | 混合 | 33 | 17 个 2 行访问器是**合法布局合同**，非水分；`clearing_canvas`/drag 真实 | 7（4 伪类型+`current_intersection`/`active_widget_state`/`is_dragging` 真缺） | 状态被 mode_manager/wall/terrain 消费；通用 hover/click 生产者缺失 |
| 45 | systems/ui/resources/widget_params.rs | 418 | 混合 | 28 | 9 个标志访问器为布局合同；`volume`/flags 真实 | 7（3 closure+`on_hover_start`/`on_click`/`on_drag_start`/`on_drag_release` 真缺） | widget 回调面缺失 |
| 46 | systems/ui/signifiers/animation.rs | 158 | 混合 | 5 | 登记 GAP：kind switch 与 translation 未写 100-101 | 0 | 动画 t/scale 真实；不完整 |
| 47 | systems/ui/signifiers/mod.rs | 44 | 真实行为 | 2 | 装配函数真实；依赖 UiState 配置“待取证”8 | 3（`minimum_lifespan`/`animation_easing`/`parent_to_widget`） | `add_signifier_systems` 仅测试调用 |
| 48 | systems/ui/signifiers/signifier_create.rs | 336 | 混合 | 19 | Raw 记录只 `spawn(SignifierId)` 占位 218 | 0 | 依赖 Factory 闭包 |
| 49 | systems/ui/signifiers/signifier_factory.rs | 445 | 混合 | 21 | `build` 只插 `SignifierAnimation`/`SignifierId`；DrawableMeshBundle/UiPass/WireframePass/TransientMesh 等未落 412-426 | 0 | 显式 GAP，不冒充 |
| 50 | systems/ui/signifiers/signifier_mark_for_delete.rs | 48 | 真实行为 | 2 | 无 | 0 | 已实现 |
| 51 | systems/ui/signifiers/tests.rs | 136 | 测试 | 6 | — | 0 | 仅测试 |
| 52 | systems/ui_clutter/autoclutter_state.rs | 206 | 混合 | 5 | `gap:[u8;0x37]`/`key` 是布局合同 | 0（新增） | 资源已注册 |
| 53 | systems/ui_clutter/debug_spawn_clutter.rs | 119 | 真实行为 | 2 | 无 | 0 | 已注册 |
| 54 | systems/ui_clutter/decoclutter_show_pins.rs | 334 | 混合 | 6 | 字段语义 U 待取证 100 | 0 | 已注册 |
| 55 | systems/ui_clutter/defered_autoclutter_history.rs | 30 | 真实行为 | 1 | 无 | 0 | 已注册 |
| 56 | systems/ui_clutter/edit_autoclutter.rs | 37 | 真实行为 | 1 | 无 | 0（新增） | 已注册 |
| 57 | systems/ui_clutter/mod.rs | 71 | 数据合同 | 1 | 0x58B 字段名待取证 37 | 0 | — |
| 58 | systems/ui_clutter/registration.rs | 82 | 真实行为 | 2 | 无 | 0（新增） | 被 history 装配调用（history.rs:488/538） |
| 59 | systems/ui_clutter/rmb_autoclutter.rs | 166 | 真实行为 | 3 | 无 | 0 | 已注册 |
| 60 | systems/ui_clutter/tests.rs | 419 | 测试 | 8 | — | 0 | 仅测试 |
| 61 | systems/ui_clutter/ui_autoclutter.rs | 277 | 混合 | 9 | 转换真实；放置消费仍属上层缺口（测试注释 246） | 1（closure$1） | 已注册 |
| 62 | systems/ui_clutter/ui_focus_decoclutter.rs | 140 | 真实行为 | 1 | 无 | 4（closure$0..3） | 已注册 |
| 63 | systems/ui_edit_focus.rs | 774 | 真实行为 | 25 | 无宏空壳；GeoSource→EditSubject 逐指令还原 | 0 | 仅 `ui_edit_focus_terrain` 经 terrain/mod.rs:144 接线；wall/clutter/移动/升起等 sibling 全缺 |
| 64 | systems/weather_system.rs | 17 | 真实行为 | 1 | 无 | 0 | tiny-glade:474 接线 |
| 65 | systems/world_raster.rs | 342 | 真实行为 | 8 | 无占位；record/upload/draw 状态机真实 | 7（全部 closure_env/类型伪条目） | upload 被 scene_compute 消费；`draw_raster` 调用方未注册 |

性质汇总：声明 6、数据合同 2、测试 2、真实行为约 25、混合约 30。

---

## 2. 空壳与“水分”清单（带证据）

### 2.1 宏级/字面空壳
- `src/systems/brush_preview.rs:173` `pub fn register_required_components(_world: &mut World) {}`（derive 注册胶水；非业务空壳）。
- `src/systems/collision/world.rs:1206` `flush_colliders` 在 `not(native-physics)` 下为空体（feature 壳）；实际 host 启用 `native-physics`（tiny-glade Cargo.toml:35），所以走 1191 的真实实现。

### 2.2 语义占位/未接线（比空体更实质）
- `camera_rig/main_camera.rs:402` `projection` 用单位阵占位；`:411` 注释明确“`main_camera_update` 未实现，setter 供宿主接线”。
- `camera_rig/main_camera.rs:418-431` 用 Normal body 位置/yaw/pitch 重建 `inverse_view`，是替代宿主逻辑，不是原 `main_camera_update` 全分支。
- `collision/world.rs:237-239` `upsert_collider_entry` 注册表插入接线“尚未落地”；`:2414` `on_level_reload` 简化为 `*self = new()`。
- `collision/intersect_meshes.rs:98-110` 折线→砖铺放、Graph `add_node/add_edge`、鸟巢候选 `spec_extend` 均登记未接线。
- `collision/visualize_colliders.rs:39` 原 `DebugLine::show`/`DebugSphere::show` 宿主 seam；`:137` 子 compound/trimesh 线框仍是结构占位。
- `steam/mod.rs:403-406` `now_hint()` 固定返回 `Duration::from_millis(0)`。
- `ui/signifiers/animation.rs:100-101` kind switch 与 translation 写入未恢复。
- `ui/signifiers/signifier_create.rs:218` Raw 记录仅 spawn 占位实体（无闭包/网格）。
- `ui/signifiers/signifier_factory.rs:412-426` DrawableMeshBundle/UiPass/WireframePass/DontCastShadow/MinimumLifespan/ParentToVolumeOrigin/TransientMesh 未落库。
- `ui/signifiers/mod.rs:8` scale/alpha 目标 switch 与 config_hash→widget 匹配“待取证”。
- `ui_clutter/decoclutter_show_pins.rs:100`、`ui_clutter/mod.rs:37` 字段语义/字段名待取证（布局本身已锚定）。
- `brush_preview.rs:20` 原生 0x68B bundle 只落库一部分。
- `collision/query_policy.rs:44-54` `cast_ray` 是 trait 接口，无具体 subworld 后端实现。
- `main_camera.rs`（31 行）仅枚举，非空壳但说明原 `main_camera` 模块绝大多数函数未在此模块落地。

### 2.3 合法但容易被误判为“水分”的
- `ui_state.rs` 的 17 个、`widget_params.rs` 的 9 个 2 行访问器：是按原生字节偏移实现的布局合同，不是假实现；缺的是生产者/回调，不是访问器。
- `collision/world.rs` 的 37 个 body≤2 的 `miss/is_hit/slot/...`：枚举/句柄投影与 trait 胶水，属于数据合同。
- `mode_manager.rs` 的 13 个 no_trace：11 个 `closure_env$N>` + `Error>/StrRead>/Global>`，是闭包/serde 伪条目，不是业务函数。
- `arrow_gizmo_animation.rs` 的 56 个 `original_full_loop_N`：测试 oracle，逐行复刻原循环以做等价校验；真正业务只有 1 个系统函数。

---

## 3. no_trace 对照（改名/搬移/真缺失）

### 3.1 `systems/ui/**` 的 30 个 no_trace（与任务数字一致）

| 骨架文件 | no_trace 函数 | 判定 | 证据 |
|---|---|---|---|
| ui/ui_update.rs | `ui_update`, `get_ui_volume`（2） | **真缺失（整文件不存在）** | crate 内全符号 0 命中 |
| ui/hover_intersection.rs | `hover_intersection`（1） | **真缺失（整文件不存在）** | 同上 |
| ui/debug_visualize_volumes.rs | `debug_visualize_roof_uvs`, `debug_visualize_volumes`, `closure$0/$3/$4/$5`（6） | **真缺失（整文件不存在）** | 同上 |
| ui/resources/slider3d.rs | `closure_env$0/$5/$7/$21`（4） | **真缺失/需人工**：拖拽回调闭包；核心 `project_cursor_onto_line` 已 in_file | 符号 0 命中 |
| ui/resources/ui_state.rs | `String>`, `UiRectangleDimsEdit>`, `UiRectangleOrientationEdit>`, `DecoratorHandleName>`（4） | **伪条目**（泛型/类型名） | 非可调用函数 |
| ui/resources/ui_state.rs | `current_intersection`, `active_widget_state`, `is_dragging`（3） | **真缺失方法** | 全 crate 0 命中 |
| ui/resources/widget_params.rs | `closure_env$0/$2/$3`（3） | **真缺失/需人工**（回调闭包） | 全 crate 0 命中 |
| ui/resources/widget_params.rs | `on_hover_start`, `on_click`, `on_drag_start`, `on_drag_release`（4） | **真缺失 widget 回调** | 全 crate 0 命中 |
| ui/signifiers/mod.rs | `minimum_lifespan`, `animation_easing`, `parent_to_widget`（3） | **真缺失方法**（对应组件类型已存在） | 全 crate 0 命中 |

- 30 个 no_trace 中：**真缺失的可调用面 = 2+1+6+3+4+3 = 19**；4 个 slider 闭包 + 3 个 widget 闭包共 7 个属“需人工判定是否业务”；4 个是泛型/serde 伪条目。
- 除 no_trace 外，`add_ui` 在骨架里是 `moved`，但当前 crate **不存在** `add_ui`（`ui/mod.rs` 仅 6 行），属于“moved 但未落”的缺口。

### 3.2 camera_rig
- `camera_rig.rs`：`init_walk_camera`（业务 no_trace，**真缺失**，全 crate 仅注释）；`Error>/Global>/Serializer>` 为 derive/serde 伪条目。
- `camera_rig/walk.rs`：`teleport` no_trace（**真缺失**，文件整体不存在）；其余 5 个 name_only（`new`/`rotation`/`rotate_yaw_pitch`/`constrain_to_be_above_terrain`/`translate_screen_space`）+ `update` moved 也无对应文件。
- `camera_rig/butterfly.rs`：无 no_trace，但**整个 Butterfly 相机文件缺失**。
- `camera_rig/free.rs` → `free_camera.rs`（改名）；`normal.rs` → `normal_body.rs`（改名）；`camera_rig.rs` → `camera_rig/mod.rs`+`main_camera.rs`（拆分/搬移）。
- `systems/main_camera.rs`（骨架）没有 no_trace，但其 5 个 `name_only`（`build`/`reset_camera`/`paused_main_camera_update`/`camera_snapshot`/`fmt`）和 `moved` 的 `main_camera_update` 在当前 crate 无实现；`in_file` 的 `next` 也未找到，只有 `CameraMode::previous` 在 `camera_rig/main_camera.rs:71` 落地 → **不是 no_trace，却同样是真缺失**。
- 结论：Normal/Free 姿态数学大体恢复；**Walk/Butterfly 模式、主相机模式跳表更新、暂停更新、reset/snapshot 缺**。

### 3.3 backend/input
- 6 个骨架输入文件 **no_trace 均为 0，全部 `in_file`**：`calculate_input_delta`、`input_record_replay`(×3)、`input_replay_for_trailers`(×2)、`reset_input_state`、`translate_input`、`translate_input_to_cursor_moved`。
- 当前 crate 额外的 `mod_key.rs`、`update_cursor_position.rs`、`button_input.rs`、`mod.rs` 是拆分/搬移（骨架 `systems/mod_key.rs` 两个 name_only 已落到 `backend/input/mod_key.rs`）。
- 因此输入函数面“可追溯性最好”；但动作槽位消费者不完整（见 §5）。

### 3.4 collision
- `collision.rs`、`world.rs`、`intersect_meshes.rs`、`visualize_colliders.rs` **no_trace 全为 0**。
- `world.rs`：49 in_file + 4 moved（Bevy hook 胶水 `apply/init_state/new_archetype/queue`）+1 name_only。
- 但两类“非 no_trace 缺口”必须记录：
  1. `world.rs` 模块文档 29-33 行宣称已新增/仍登记的函数中，`update_rapier_colliders`、`construct_arch_geo` **在当前源码里完全不存在**（`grep "fn update_rapier_colliders|fn construct_arch_geo"` 无结果），属文档过度声明。
  2. collider 生命周期生产者（谁的代码调用 `upsert_collider`）在全 crate 无调用点；只有 `history.rs:389/445` 调 `flush_colliders`。查询核心虽有，但世界里没有 collider → 端到端射线仍会 miss。

### 3.5 mode_manager
- 20 条记录：in_file 5（`is_tool_selected`、`mode_manager`、`send_mode_just_changed_event`、2 个 closure_env）、name_only 1（`fmt`）、va_only 1（`closure_env$9`）、no_trace 13。
- 13 个 no_trace = `Error>`、`StrRead>`、`Global>` + 10 个 `closure_env$N>`（0/4/5/6/7/8/10/11/12/13）。**全部是 derive/serde/闭包伪条目，无业务函数缺口**。
- 核心 105 行状态机真实，且 country-core startup 与 tiny-glade run_game 两端都接线。

### 3.6 time_of_day
- 2/2 `in_file`（`time_of_day`、`remap_hour_to_fmod_hour`），**0 no_trace**。
- 源码实际有 24 个函数：`advance_angle/hour/playback/tod_hour`、内联 `suncalc`、`write_derived_atmosphere`、147 行 `run_time_of_day`。函数级闭合最好的一条链。

---

## 4. 抽读结论（12 个最大/重点文件，逐条）

1. **collision/world.rs（3213 行，142 fn）**：真实实现，Rapier 查询、`RaycastMeshData` 布局、`rapier_hit_to_raycast_world_result`、dirty tile、`upsert/remove/dispose` 都存在；但 `on_level_reload` 2414 是整体重置而非原增量重建；`upsert_collider_entry` 注册“接线未落地”237；`update_rapier_colliders`/`construct_arch_geo` 在文档 29-33 被标为已处理/仍登记，源码实际不存在。**判定：查询引擎真实，生命周期装配不闭合。**
2. **prefab.rs（1141 行）**：`spawn_prefabs` 46 行、`build_prefab_contents` 61 行、`update_prefabs_instance` 42 行、`hot_reload_prefabs` 51 行均为真实逻辑，RT 矩阵与 GPU/RT 双路径有测试；但全 crate 无 `spawn_prefabs`/`update_prefabs`/`despawn_prefabs` 调用点，只有资源加载（gameplay_startup）→ **库内实现、宿主未消费**。
3. **camera_rig/main_camera.rs（1005 行）**：偏移/布局、`previous` 位移表、FOV 表、`store/load_parameters` 是逐指令恢复；`camera_matrices()` 402 的 projection 是单位阵占位，`main_camera_update` 411 明确未实现；没有 `set_camera_matrices`/`rebuild_camera_matrices_from_normal_body` 的外部调用者 → **相机每帧更新链断开**。
4. **paths.rs（934 行）**：`draw_path` 194 行、`DrawState::update`、边框/笔刷/eraser 分支、`record_path_history` 都是真实逻辑且有等价测试；`PathsPlugin::build` 仅在 4 个测试里调用，`draw_path` 未进任何宿主 Schedule → **笔刷逻辑真实、生产接线缺**。
5. **ui_edit_focus.rs（774 行）**：`geo_src_to_edit_subject` 及 deco-clutter 分支按 radare2 原指令恢复，`ui_edit_focus_terrain` 用真实 `UiState`/`RaycastWorld`/光标资源且经 terrain/mod.rs:144 注册；但 `ui_edit_freehand_walls`/`ui_move_shapes`/`ui_raise_shape` 等 sibling 文件全缺 → **只闭了 terrain focus 一个消费者**。
6. **arrow_gizmo_animation.rs（738 行）**：业务函数 27-66 真实（共享 EventReader 游标、axis guard、decay/increase 常量逐位）；64 个函数里 56 个 `original_full_loop_N` 是测试 oracle；`EnableArrowGizmoStretch` 只有测试发送，无生产者，系统也未注册。
7. **mode_manager.rs（493 行）**：`mode_manager` 105 行完整呈现 Replay 早退、undo/redo 优先级、loading restore、类别槽位顺序、photomode/TOD 快捷键；两端宿主都注册。**这是范围内闭合度最高的行为链之一。**
8. **time_of_day.rs（736 行）**：`run_time_of_day` 147 行按原始块顺序（音频门→angle→hour→播放→灯光/大气）实现，`suncalc` 以 628B 等价内联替代；core startup 注册 `time_of_day::<S>`，tiny-glade 用 `time_of_day_audio_disabled`，`DerivedAtmosphere`/`DirectionalLight` 被 tiny-glade 与 prepare_frame_constants 消费。
9. **backend/input/translate_input.rs（557 行）**：105 行主函数真实，含 Steam/Deck 模式跟踪、EscMenu 门、滚轮权重 8/1、鼠标拖拽阈值 3、demo 硬编码键、history 按钮 last-only；`add_input_cpu_consumers` 已把它放进 Translate 且设置完整集合链 → **输入生产链闭合**。
10. **ui/resources/ui_state.rs + widget_params.rs**：两者是“原生字节布局的可执行合同”；`add_with_id`、`clearing_canvas`、`dragged/clicked/hovered` 真实；但 `current_intersection`/`active_widget_state`/`is_dragging` 与 4 个 widget 回调全缺，且 crate 内几乎没有非测试代码写 `replace_interaction`（只有 wall/terrain 零星写 `intersection_tag`）→ **UI 交互状态有消费者、无通用生产者**。
11. **ui/signifiers/signifier_factory.rs + signifier_create.rs**：Stream/Factory 记录与唤醒机制真实，但 `build` 只插入 2 个组件，原版的网格/渲染/生命周期组件包全缺；Raw 路径只 spawn 空实体；系统只在 tests 注册 → **指示符视觉链未闭合**。
12. **steam/mod.rs（446 行）**：`update_steam_inputs` 108 行与 22 个输入码映射真实；`now_hint` 固定 0 是时间语义占位；`add_steam_integration`/`update_steam_inputs` 无调用点，`startup_steam` 也未注册 → **Steam 输入与 overlay 链未接线**。
13. **collision/intersect_meshes.rs（697 行）**：三角对相交、折线合并、图剪枝/去重是真实实现；跨域砖铺放/图边/鸟巢候选只登记未接线 → mesh-intersection 产物无法进入建筑/装饰下游。
14. **world_raster.rs（342 行）**：`record_raster_history`、`upload_world_raster_buffers_to_staging`、`draw_raster_with_staging` 真实；upload 被 tiny-glade `scene_compute.rs` 消费；但 `draw_raster` 的调用方（路径绘制系统）未注册，路径 raster 生产链断在 paths.rs。

---

## 5. 各链闭合情况（生产者/消费者/时序）

| 链 | 已闭合部分 | 缺口（生产者/消费者/时序） | 判定 |
|---|---|---|---|
| 输入 | 事件注册、Translate/Resolve/RecordReplay/Derive/Finish 全集合链、`translate_input`、光标、modifier、录制回放、reset | 部分动作槽无消费者（因 brush_preview/arrow/UI 工具系统未注册） | **生产链闭合，消费面不全** |
| 模式管理 | `mode_manager` + `send_mode_just_changed_event` + configure，两端接线；与输入 Control、history Replay 时序有显式 set | 无（除 UI esc 探针依赖 UiState 真值） | **闭合度高** |
| 时间/天气 | `time_of_day`（含 audio-disabled 变体）→ `DerivedAtmosphere/DirectionalLight` → prepare_frame_constants/tiny-glade；`weather_system`；`time_update` | 无 | **闭合** |
| GUI/屏幕日志 | `show_screen_log`/`show_debug_controls`/`publish_pending_egui_context` 经 configure_gui_consumers 接线 | photomode visibility 前缀未接 | **基本闭合** |
| UI widget/交互 | UiState 布局合同、clearing_canvas、mode_manager 消费 hover/widget/tag；terrain focus 消费 | `ui_update`/`hover_intersection` 通用 hover/click/volume 生产者缺失；widget 回调 4 个缺；`add_ui` 缺失；多数工具系统未注册 | **未闭合（缺生产者）** |
| UI signifiers | 流/动画 t/删除链 | `build` 组件包、mesh/shader 解析、`add_signifier_systems` 未注册；kind switch/translation 缺失 | **未闭合** |
| UI clutter | 资源注册 + 6 系统经 history 注册；rmb→convert→focus 链 | 放置消费上层缺口；部分字段语义 U | **基本闭合（局部待取证）** |
| 相机 | Normal/Free 姿态数学、参数存取、MainCamera 资源读写 | `main_camera_update`/paused/reset/snapshot、Walk/Butterfly、系统注册全部缺；projection 占位 | **未闭合** |
| 碰撞 | Rapier 查询/形状构造/射线/球扫/重叠、mesh 三角相交、插件资源合同 | `CollisionPlugin::build` 无调用；`upsert_collider` 无生产者；`MeshIntersectionGraph` 未插入；砖铺放/鸟巢未接线；`update_rapier_colliders`/`construct_arch_geo` 缺 | **查询核心闭合、生命周期未闭合** |
| 路径/光栅 | `draw_path`、`record_path_history`、raster 上传/历史 | `PathsPlugin` 未调用；`draw_path`/garden 之外的 raster 绘制未注册 | **库内闭合、接线缺** |
| 预制体 | spawn/update/despawn/hot reload 核心 | 全部系统未注册；事件/命令 seam 由宿主负责 | **库内闭合、接线缺** |
| Steam | 22 码映射、update 处理、资源类型 | `add_steam_integration`/`startup_steam`/`update_steam_inputs` 未注册；时间源 0 占位 | **未接线** |
| Arrow gizmo | 单系统逻辑真实 | 系统未注册、Enable 事件无生产者 | **未接线** |

---

## 6. 仍未复现清单（≥12 条，按证据）

1. **`systems/ui/ui_update.rs` 整文件缺失**：`ui_update`、`get_ui_volume` 均 no_trace，crate 全符号 0 命中。
2. **`systems/ui/hover_intersection.rs` 整文件缺失**：`hover_intersection` no_trace，0 命中。
3. **`systems/ui/debug_visualize_volumes.rs` 整文件缺失**：2 个业务函数 + 4 个 closure no_trace，0 命中。
4. **UI 工具交互文件全缺**：`ui_2d_follow_mouse.rs`、`ui_camera_mode_gizmos.rs`、`ui_edit_freehand_walls.rs`、`ui_focus_mode_shape_height.rs`、`ui_move_freehand_walls.rs`、`ui_move_shapes.rs`、`ui_raise_shape.rs`；合计 17 个 no_trace（含 `move_gizmo_and_trigger_vol`、`impl_raise_shape`）。
5. **UiState 缺方法**：`current_intersection`、`active_widget_state`、`is_dragging`（骨架 `ui_state.rs` no_trace；当前源码 0 命中）。
6. **WidgetParams 缺回调**：`on_hover_start`、`on_click`、`on_drag_start`、`on_drag_release`（均 no_trace，0 命中）。
7. **Signifier 缺方法/行为**：`minimum_lifespan`、`animation_easing`、`parent_to_widget`（no_trace）；`animation.rs:100` kind switch/translation；`factory.rs:412-426` 组件包；`create.rs:218` Raw 占位实体。
8. **相机主更新链缺**：`main_camera_update`（moved 但无实现）、`paused_main_camera_update`、`reset_camera`、`camera_snapshot`、`build`；`main_camera.rs:402` projection 占位。
9. **Walk/Butterfly 相机缺**：`camera_rig/walk.rs` 整文件（`teleport` no_trace 等）、`camera_rig/butterfly.rs` 整文件；`init_walk_camera` no_trace。
10. **碰撞生命周期装配缺**：`CollisionPlugin::build` 无宿主调用；`MeshIntersectionGraph` 未插入；`upsert_collider` 无生产者；`history.rs:389/445` 只 flush 空世界。
11. **碰撞文档过度声明**：`world.rs:24-33` 列出的 `update_rapier_colliders`、`construct_arch_geo` 实际无函数体；`on_level_reload` 2414 简化为整体重置。
12. **mesh 相交下游断点**：`intersect_meshes.rs:98-110` 砖铺放/Graph add_node/add_edge/鸟巢 spec_extend 未接线；`visualize_colliders.rs:39/137` host show 与子形状线框仍是 seam/占位。
13. **Steam 接线缺**：`add_steam_integration`/`startup_steam`/`update_steam_inputs` 全无调用点；`mod.rs:403-406` 时间源 0 占位。
14. **Prefab 系统未消费**：`spawn_prefabs`/`update_prefabs`/`despawn_prefabs`/`hot_reload_prefabs` 全 crate 无调用点（只有资源加载）。
15. **Paths 插件未消费**：`PathsPlugin::build` 仅测试调用；`draw_path`/`record_path_history` 未进 Schedule。
16. **Arrow gizmo 未消费**：系统未注册；`EnableArrowGizmoStretch` 只有测试发送。
17. **Photomode 主体未恢复**：`photomode.rs` 仅前缀 `update_photomode_visibility`；`configure_photomode_visibility` 无调用点；完整 `do_photomode` 未落。
18. **Timelapse 缺函数**：`start_timelapse`、`cleanup_timelapse` no_trace；当前 `timelapse_replay.rs` 仅事件类型。
19. **backend 装配函数缺**：`add_backend_systems`、`insert_backend_resources`、`add_backend_events` no_trace，当前 `backend/mod.rs` 仅声明模块。
20. **其他整文件缺失**：`look_at_camera.rs`、`mesh_follow_terrain.rs`、`clear_canvas.rs`、`despawn.rs`、`utils_intersect.rs`（`line_x_shape_overlap`）、`debug/human_scale_ref.rs`、`debug/lazy_grid_debug.rs`、`backend/grow_grass.rs`、`backend/icc.rs`（`update_window_icc_profile`）、`backend/game_frame_index.rs`、`backend/panic_reporter_fetch_active_session.rs`；这些符号在全 crate 0 命中。
21. **简化/替代实现需人工确认**：`camera_rig/main_camera.rs:418-431` 的 `inverse_view` 重建、`collision/world.rs:2414` 的 `on_level_reload`、`steam` 的 0 时间源、`arrow_gizmo` 的测试 oracle 不等于原实现。

### 已搬移/改名、不是真缺失（避免误报）
- `backend/configwatch.rs`、`meshwatch.rs`、`texture_watch.rs`、`transient_mesh.rs` → `asset_libraries/` 同名文件（搬移）。
- `backend/vao_update.rs` → `asset_libraries/vao_library.rs` 的 `rebuild_vao`（搬移/改名；`build_missing_vaos` 名未保留，需人工确认等价）。
- `systems/mod_key.rs` → `backend/input/mod_key.rs`（搬移）。
- `camera_rig.rs` → `camera_rig/mod.rs` + `main_camera.rs`；`normal.rs`→`normal_body.rs`；`free.rs`→`free_camera.rs`（拆分/改名）。
- `utils.rs` 的 `add_stages` → tiny-glade `run_game.rs:50/249` 以 0..44 手工链替代（搬移到宿主，但仍非同名函数）。
- `systems/mod.rs` → 当前 crate 用 `lib.rs:37-64` 直接声明 `systems` 子模块（结构搬移）。

---

## 7. 完成度估计

### 7.1 量化依据
- 文件面：骨架同口径 83 文件，当前 65 文件；其中约 30 个骨架文件无对应（约 10 个属搬移/改名，约 20 个真缺失），另有约 12 个当前新增拆分文件。
- 函数面（骨架 345 条）：in_file 45.2%；任意线索（in_file+moved+name_only+va_only）72.5%；no_trace 27.5%，其中业务向约 44 条。
- 正文规模：20,118 行，780 fn；238 个 `#[test]`（另有大量测试辅助函数）说明关键纯函数有等价校验，但不代表宿主链闭合。
- 接线面：输入、模式、时间天气、GUI、sheep、terrain-focus、ui_clutter 已进 Schedule；相机、碰撞生命周期、prefab、paths、arrow、signifier、steam、photomode、timelapse 未进。

### 7.2 分链完成度（源码语义 / 端到端接线）
| 链 | 源码语义 | 端到端 | 依据 | 置信度 |
|---|---:|---:|---|---|
| 输入 | 85% | 80% | 6/6 骨架文件 in_file；完整 InputSystemSet 链已注册；少量动作槽无消费者 | 高 |
| 模式管理 | 90% | 90% | 20 条记录中业务全覆盖；两端接线；13 no_trace 全伪条目 | 高 |
| 时间/天气/时钟 | 90% | 90% | 0 no_trace；147 行状态机；输出被渲染常量消费 | 高 |
| GUI/日志/sheep | 85% | 85% | 行为+接线；photomode 仅前缀 | 高 |
| 碰撞 | 70% | 45% | 查询/mesh 核心真实；生命周期/插件/下游砖铺放缺；文档过度声明 | 中 |
| 相机 | 45% | 20% | Normal/Free 数学在；主更新/Walk/Butterfly/reset/snapshot 缺；无 Schedule | 中高 |
| UI widget/交互 | 30% | 15% | 10 个工具文件缺；30 no_trace 中 19 真缺；无通用 hover/click 生产者；add_ui 缺 | 中 |
| UI signifiers | 45% | 15% | 流/动画/删除在；组件包/网格解析/系统注册缺；GAP 明确 | 中高 |
| UI clutter | 75% | 70% | 转换/历史/显示真实且经 history 注册；局部字段语义待取证 | 中 |
| Paths/world_raster | 80% | 45% | 绘制/历史/上传真实；PathsPlugin 未调用 | 中 |
| Prefab | 75% | 30% | 核心实现真实；系统全未注册 | 中 |
| Steam | 60% | 15% | update 真实；插件/输入系统未接；时间 0 占位 | 中 |
| Arrow gizmo | 85% | 5% | 单系统真实；未注册、无事件生产者 | 中高 |

### 7.3 总体
- **本范围源码/语义完成度：约 62%（置信度：中低）**。依据：任意线索 72.5%，扣除 no_trace 业务缺口、缺文件与简化实现。
- **端到端接线完成度：约 45%（置信度：中）**。依据：§5 的 Schedule 注册检查。
- **综合估计：约 55%（置信度：中低）**。UI 工具层、相机主更新、碰撞生命周期是最大拖累；输入/模式/时间天气已接近闭合。
- 需要人工复核的项：slider3d/widget 闭包是否业务、`vao_update` 与 `rebuild_vao` 等价性、`main_camera.rs` 的 `next/previous` 实际落点、`collision/world.rs` 文档与源码的函数清单矛盾、`history.rs`/`ivy.rs` 是否应计入本范围。
