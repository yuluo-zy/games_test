# systems/decorator 任务清单

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/systems/decorator` |
| package | `system-decorator` |
| 规模 | 74 文件 / 236 本 ns 函数 / 58 闭包 / **164 业务函数**（上层最大游戏逻辑模块） |
| 难度 | 高 |
| 批次 | 3（wall-constructor 之后） |
| 上游依赖 | `country-core`（墙/屋顶几何、hole）、`rapier3d`（物理）、`tabloid`、`rhapsody`、`bevy_math`、`petgraph`、`smallvec`、`half`、`utils` |
| 复现工程现状 | MODULE_PROGRESS：**部分接线**（9 恢复文件）；墙面四族 asset/subtype/position→raw holes→wall brick；Info/storage 与部分 stairs 候选；placement 1668 / subtype 1326 等限定报告；`evidence/building-decorator-info/`、`building-placement/`；core 侧 `recovered_building/decorator_{info,placement,subtype}.rs` |

## 已有基础（复用，不重复恢复）

- 墙面四族 decorator 的 asset/subtype/position → raw holes → wall brick
- `decorator_info` / `decorator_placement` / `decorator_subtype`（core 侧）
- 部分 stairs 候选

## 任务表（按 B src 层级）

### 主干：蓝图 / 加载 / 子类型

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-DECOR-01 | `src/blueprint.rs` | 恢复 `DecoratorBlueprint::{set_colliders, set_interaction_mesh, add_door_stairs}`、`DecoratorBlueprints::{insert, get}`、`clear_blueprints`、`copy_blueprints_to_prev`、`validate_blueprints`、`show_max_wall_y`、`instantiate_blueprint_colliders`、`instantiate_blueprints`、`instantiate_decorator_colliders`。 | rapier、asset | 部分 | 原入口对照 + 蓝图/碰撞实例一致 | 部分 |
| UM-DECOR-02 | `src/load_decorators.rs` | 恢复 `load_decorators`、`load_collision_as_bbx`。 | asset/JSON | 部分 | 原入口对照 | 部分 |
| UM-DECOR-03 | `src/derive_decorator_subtype.rs` | 恢复 `derive_decorator_subtypes`、`derive_decorator_subtype`、`is_half_dormer`。 | — | 部分（subtype 1326 报告） | 原指令样例 | 部分 |
| UM-DECOR-04 | `src/apply_decorator_op.rs` | 恢复 `apply_decorator_op`：装饰操作应用（放置/移动/删除的统一入口）。 | 全部 | 部分 | 原入口对照 | 部分 |
| UM-DECOR-05 | `src/handle_external_decorator_cmds.rs` | 恢复 `handle_external_decorator_cmds`。 | 命令层 | 无 | 原入口对照 | 未开始 |
| UM-DECOR-06 | `src/backup.rs` | 恢复 `create_or_restore_decorator_backup`、`DecoratorBackup::clear`。 | — | 无 | 原指令样例 | 未开始 |
| UM-DECOR-07 | `src/restore_archived_decorators.rs` | 恢复 `restore_archived_decorators`。 | 归档 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-08 | `src/cache_decorator_transforms.rs` | 恢复 `cache_decorator_transforms`。 | — | 无 | 原指令样例 | 未开始 |
| UM-DECOR-09 | `src/decorator_visual_state.rs` | 恢复 `DecoratorVisualState::seal_persistent_entities`、`PersistentVisualEntities::entities`、`DecoratorPersistentVisualState::entities`、`add_decorator_visual_entity`。 | 实体系统 | 部分 | 原入口对照 | 部分 |
| UM-DECOR-10 | `src/decorator_resources.rs` | 恢复资源定义（无独立符号，需确认归属）。 | — | 无 | 类型合同对照 | 未开始 |
| UM-DECOR-11 | `src/collision_detection.rs` | 恢复碰撞检测（无独立符号，需定位内联宿主）。 | rapier | 无 | 原指令样例 | 未开始 |

### 越界裁剪 / 合并提案 / 跟随

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-DECOR-12 | `src/cull_oob_decorators.rs` | 恢复 `cull_oob_decorators`、`cull_stair_attached_decorators{,_on_invalid_stairs}`、`check_if_dormer_ridge_is_within_roof_bounds`、`is_decorator_location_valid`、`auto_allow_decorator_culling`、`should_snap_to_corner`。 | 屋顶/楼梯几何 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-13 | `src/merge_proposal_wall_decorators.rs` | 恢复 `find_best_wall_decorator_merge_position`、`propose_wall_decorator_merge_position_inner`。 | 墙几何 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-14 | `src/merge_proposal_roof_decorators.rs` | 恢复 `find_best_roof_decorator_merge_position`。 | 屋顶几何 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-15 | `src/move_decorators_following_anchors.rs` | 恢复 `move_decorators_following_anchors`。 | 锚点 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-16 | `src/remove_ivy_under_windows.rs` | 恢复 `remove_ivy_under_windows`。 | ivy | 无 | 原指令样例 | 未开始 |
| UM-DECOR-17 | `src/dormer_holes.rs` | 恢复 `upload_dormer_holes_to_gpu`。 | rhapsody | 无 | 原入口对照；GPU 后置登记 | 未开始 |
| UM-DECOR-18 | `src/chimney_smoke_anim_thickness.rs` | 恢复 `chimney_smoke_anim_thickness`。 | — | 无 | 原指令样例 | 未开始 |
| UM-DECOR-19 | `src/window_lights.rs` | 恢复 `window_lights`。 | 时间/天气 | 无 | 原指令样例 | 未开始 |

### 墙/形状变化响应

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-DECOR-20 | `src/decorator_on_wall_height_changed.rs` | 恢复 `decorator_on_wall_height_changed`。 | core 墙高权威 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-21 | `src/decorator_on_move_shape.rs` | 恢复 `decorator_on_move_shape`。 | core 形状 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-22 | `src/decorator_on_rectangle_edited.rs` | 恢复 `decorator_on_rectangle_edited`。 | 矩形墙 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-23 | `src/decorator_on_freehand_wall_edited.rs` | 恢复 `decorator_on_freehand_wall_edited`、`decorator_on_circle_edited`。 | 自由墙/圆墙 | 无 | 原指令样例 | 未开始 |

### 历史 / 回放

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-DECOR-24 | `src/deferred_decorator_history.rs` | 恢复 `decorator_record_history`。 | core 历史事务（NR2） | 无 | 原入口对照 | 未开始 |
| UM-DECOR-25 | `src/replay_decorator.rs` | 恢复 `replay_decorator`。 | core 历史回放 | 无 | 原入口对照 | 未开始 |

### 音频

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-DECOR-26 | `src/audio/op_confirm_audio.rs` | 恢复 `op_confirm_audio`、`place_sound`。 | 音频胶水 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-27 | `src/audio/op_preview_audio.rs` | 恢复 `op_preview_audio`。 | 音频胶水 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-28 | `src/audio/op_stairs_audio.rs` | 恢复 `op_stairs_audio`。 | 音频胶水 | 无 | 原指令样例 | 未开始 |

### decorator_visual/* 视觉族（按族一组）

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-DECOR-29 | `src/decorator_visual/deco_differential_state.rs` | 恢复 `WallAttachedDecoTracker::{add, remove}`（多单态）：墙面装饰差分状态。 | — | 无 | 原指令样例 | 未开始 |
| UM-DECOR-30 | `src/decorator_visual/maintain_visual_entities.rs` | 恢复 `clear_out_visual_entities`、`clear_out_wall_holes`。 | 实体系统 | 无 | 原入口对照 | 未开始 |
| UM-DECOR-31 | `src/decorator_visual/decorator_clutter.rs` | 恢复 `generate_decorator_clutter`。 | clutter | 无 | 原指令样例 | 未开始 |
| UM-DECOR-32 | `src/decorator_visual/arrow_slit.rs` | 恢复 `generate_arrow_slits`。 | 墙几何 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-33 | `src/decorator_visual/chimney_assemble_bricks.rs` | 恢复 `chimney_assemble_bricks`。 | 砖列 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-34 | `src/decorator_visual/chimney_roof.rs`、`chimney_wall.rs` | 恢复 `generate_roof_chimneys`、`generate_wall_chimneys`。 | 屋顶/墙几何 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-35 | `src/decorator_visual/cottage_wall_window.rs` | 恢复 `generate_cottage_wall_windows`、`flowerbed_xforms`、`add_lintels`。 | 墙/过梁 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-36 | `src/decorator_visual/cottage_corner_window.rs` | 恢复 `generate_cottage_corner_windows`。 | 墙角 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-37 | `src/decorator_visual/cottage_balcony_door.rs` | 恢复 `generate_cottage_balcony_doors`、`add_door_autoclutter`。 | 门/clutter | 无 | 原指令样例 | 未开始 |
| UM-DECOR-38 | `src/decorator_visual/gothic_wall_window.rs` | 恢复 `generate_gothic_wall_windows`、`get_gothic_hat_bones`、`calc_gothic_half_dormer_profile_ws`。 | 墙几何 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-39 | `src/decorator_visual/gothic_balcony_door.rs` | 恢复 `generate_gothic_balcony_doors`、`check_for_door_stairs`。 | 门/楼梯 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-40 | `src/decorator_visual/gothic_dormer.rs` | 恢复 `generate_gothic_dormers`、`gothic_dormer_profile_curve`。 | 屋顶几何 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-41 | `src/decorator_visual/cottage_dormers/*` | 恢复 `generate_cottage_dormers`、`is_out_of_roof_bounds`、`generate_dormer_roofs`、`maintain_dormer_tiles`、`get_collision_mesh`。 | 屋顶/碰撞 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-42 | `src/decorator_visual/door_stairs_assemble_bricks.rs` | 恢复 `door_stairs_assemble_bricks`。 | 砖列 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-43 | `src/decorator_visual/flag.rs`、`flag_connections.rs`、`generate_clothline.rs` | 恢复 `generate_flags`、`generate_flag_connections`、`greedily_connect`、`clothline`、`generate_clothline_visuals`。 | 几何 | 无 | 原指令样例 + 连接贪心规则一致 | 未开始 |
| UM-DECOR-44 | `src/decorator_visual/lantern.rs` | 恢复 `generate_lanterns`、`maintain_lantern_lights`。 | 光照 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-45 | `src/decorator_visual/trap_doors.rs` | 恢复 `generate_trap_doors`。 | 地板 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-46 | `src/decorator_visual/stairs.rs` | 恢复 `generate_playermade_stair_decorators`、`generate_playermade_stair_requests<false/true>`、`maintain_stair_graph`、`walk_shape`、`compute_stair_assembly_params`、`derive_stair_node_params`、`densely_sample_spline_segment`、`find_alignment_preferences`、`find_best_neighbor_alignments`、`determine_segment_type`。 | 楼梯几何/曲线 | 部分 | 原入口对照 + 楼梯图一致 | 部分 |
| UM-DECOR-47 | `src/decorator_visual/stairs_assemble.rs` | 恢复 `playermade_stairs_assemble<false/true>`、`construct_stair_steps`、`gen_stair_floors`、`construct_ladder`、`calculate_adjusted_ladder_positions`、`determine_ladder_support`、`create_stairs_collider`、`gen_stairs_collider_and_rt_geo`、`gen_stairs_structural_collider`、`create_structural_colliders_and_initial_graph`、`calculate_distortion_from_structural_graph`、`split_segment_into_supported_and_unsupported_subsegments`、`playermade_stairs_{interpolate_and_compare,assembly_propagate_dirty_flags,add_wall_holes}`、`stairs_subscribe_lazy`、`structural_graph_subscribe`。 | rapier/petgraph | 部分 | 原入口对照 + 结构图/碰撞一致 | 部分 |
| UM-DECOR-48 | `src/decorator_visual/stairs_railings_assemble.rs` | 恢复 `assemble_railings`、`railings_tips`、`construct_wooden_railing`、`add_corner_stone_railing_piece`。 | 楼梯几何 | 无 | 原指令样例 | 未开始 |

### ui_systems/*

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-DECOR-49 | `src/ui_systems/decorator_interaction_intent.rs` | 恢复 `calculate_decorator_interaction_intent`、`snap_balcony_door`。 | 输入层 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-50 | `src/ui_systems/ui_move_decorator.rs` | 恢复 `ui_move_decorator`、`find_stair_placement_candidate`。 | 输入层 | 无 | 原入口对照 | 未开始 |
| UM-DECOR-51 | `src/ui_systems/ui_place_decorator.rs` | 恢复 `ui_place_decorator`、`show_decorator_icon`、`decorator_cursor_icon`。 | 输入层/UI | 无 | 原指令样例 | 未开始 |
| UM-DECOR-52 | `src/ui_systems/calculate_decorator_{dst,grab_offset}.rs` | 恢复 `convert_raycast_hit_to_decorator_dst`、`try_get_grab_offset_ss_if_valid`、`raycast_with_ss_offset`。 | 射线检测 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-53 | `src/ui_systems/handle_splitting_{wall,roof}_decorators.rs` | 恢复 `display_decorator_wall_parts_and_get_hovered_part_id`、`handle_moving_out_wall_decorator_part`、`try_display_roof_decorator_parts_and_get_hovered_part_id`、`handle_moving_out_roof_decorator_part`。 | 几何 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-54 | `src/ui_systems/ui_change_{chimney,stair}_height.rs` | 恢复 `ui_change_chimney_height`、`show_signifiers`、`ui_change_stair_height`。 | 输入层 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-55 | `src/ui_systems/ui_decorator_affordance_dispatch.rs` | 恢复 `ui_dispatch_decorator_affordance`、`create_decorator_widget<...>`、`DecoratorAffordanceDispatcher::clear`。 | UI 框架 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-56 | `src/ui_systems/ui_stairs_misc.rs`、`ui_stairs_viz.rs`、`ui_focus_stairs.rs` | 恢复 `stairs_reset_for_new_frame`、`continue_stairs`、`visualize_ui_stairs`、`insert_point_ui`、`ui_focus_stairs`。 | 楼梯/UI | 无 | 原指令样例 | 未开始 |
| UM-DECOR-57 | `src/ui_systems/move_ops.rs` | 恢复 `calculate_move_dst`、`show_decorator_remove_signifier`。 | 输入层 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-58 | `src/ui_systems/ui_inactive.rs`、`ui_rotate_lantern.rs` | 恢复 `ui_decorator_inactive`、`ui_rotate_decorator`。 | 输入层 | 无 | 原指令样例 | 未开始 |

### utils / 插件

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-DECOR-59 | `src/utils/visual_entity_builder.rs` | 恢复 `WindowVisualEntityBuilder::build<...>`（两单态）。 | 实体系统 | 无 | 原指令样例 | 未开始 |
| UM-DECOR-60 | `src/lib.rs` | 恢复插件注册 `impl Plugin::build`。 | 全部上面 | 仅索引 | 与原注册顺序对照 | 未开始 |

## 卡点

- 依赖 core 墙/屋顶几何与 hole（NR1/NR2）；core 未闭合前用冻结契约。
- rapier 物理（蓝图碰撞、楼梯结构图）与 wall-constructor 的 `maintain_stone_column_colliders` 共用后端。
- `decorator_visual/*` 全族是主体工作量（约 29 个 GPU/visual 文件）；建议按族逐个 agent 并行。
- 历史任务依赖 core 历史事务（NR2）。

## 验收标准

- 主干 ≥ 原入口对照；每视觉族 ≥ 原指令样例；楼梯结构图/碰撞与原一致；RNG 序列逐位一致。
- 复用已有墙面四族与 info/placement/subtype，不重新推导。
