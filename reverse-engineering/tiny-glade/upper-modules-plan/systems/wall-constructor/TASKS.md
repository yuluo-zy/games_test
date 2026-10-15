# systems/wall-constructor 任务清单

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/systems/wall-constructor` |
| package | `system-wall-constructor` |
| 规模 | 53 文件 / 105 本 ns 函数 / 23 闭包 / **73 业务函数** |
| 难度 | 高（几何族多、互相耦合） |
| 批次 | 3（主线第一步） |
| 上游依赖 | `country-core`（墙权威/几何/hole）、`tabloid`、`rhapsody`、`fastrand`、`perchance`、`smallvec`、`utils` |
| 复现工程现状 | MODULE_PROGRESS：**部分接线**；`construct_walls` / `WallConstructor` / noise/seed/authority walker → 96B 砖输出已恢复并接 World（普通 85 / 其他 304 限定入口 + 连续 World 链）；证据最厚（`evidence/building-*` 约 30 目录：constructor/structures/segmentation/holes/gates/fences/lintels/platform-stairs/arches/…）；core 侧 `recovered_building/wall_*.rs` 约 25 文件 |

## 已有基础（复用，不重复恢复）

- `construct_walls` / `WallConstructor::from_curve` → 96B 砖记录（含 noise/seed/authority walker）
- 分段：`calculate_wall_path_segmentation` / `sample_wall_path_intersections` / `wall_path_segment`
- `resolve_hole_overlap` / `trim_rows` / `split_into_random_rows`
- 平台楼梯、拱（regular/freeform）、门（120B ConstructGateCmd）、栅栏、过梁的部分链

## 任务表（按 B src 层级）

### 主干

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-WALL-01 | `src/wall_constructor.rs` | `WallConstructor::from_curve`：曲线→构造器（已有基础，补全族分支）。 | core 曲线/权威 | 部分接线 | 原入口对照 | 部分 |
| UM-WALL-02 | `src/construct_walls.rs` | `construct_walls`：墙主体构造（已有基础，补全未覆盖族）。 | walker/分段 | 部分接线 | 原入口对照 + 96B 记录一致 | 部分 |
| UM-WALL-03 | `src/arch_walker.rs` | `ArchWalker::new` 与 `walk<...>` 多单态：沿墙行走器。 | 分段/曲线 | 部分 | 原指令样例 + 各单态实例一致 | 部分 |
| UM-WALL-04 | `src/init_wall_height_and_arch_segment_height.rs` | `init_wall_height_and_arch_segment_height`：墙高/拱段高初始化。 | core 墙高权威 | 无 | 原指令样例 | 未开始 |

### 分段

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-WALL-05 | `src/segmentation/calculate_wall_path_segmentation.rs` | `calculate_path_wall_segmentation`、`calc_path_wall_segmentation`、`process_segment`、`add_splits_in_empty_range`、`score_split_candidate`：路径分段评分。 | 曲线/几何 | 部分接线 | 原指令样例 + 评分规则一致 | 部分 |
| UM-WALL-06 | `src/segmentation/wall_path_segment.rs` | `calculate_wall_path_segments_impl`。 | 分段 | 部分接线 | 原指令样例 | 部分 |
| UM-WALL-07 | `src/segmentation/sample_wall_path_intersections.rs` | `sample_wall_path_intersections`。 | 曲线相交 | 部分接线 | 原指令样例 | 部分 |
| UM-WALL-08 | `src/segmentation/split_wall_path_segment_into_platform_stairs.rs` | `split_wall_path_segment_into_platform_stairs`。 | 平台楼梯 | 部分接线 | 原指令样例 | 部分 |

### 拱 / 过梁 / 支撑梁

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-WALL-09 | `src/construct_arches/mod.rs` | `construct_arches`（regular/freeform）。 | 拱 profile | 部分接线 | 原入口对照 + 拱输出一致 | 部分 |
| UM-WALL-10 | `src/construct_arches/profile.rs` | `create_stone_arch_profile`、`ArchFunction::remap_t`。 | — | 部分 | 原指令样例 | 部分 |
| UM-WALL-11 | `src/construct_lintels/mod.rs` | `construct_lintels`。 | 过梁 | 部分接线 | 原入口对照 | 部分 |
| UM-WALL-12 | `src/construct_lintels/place_support_beams.rs` | `SupportBeamGrower::propose`：支撑梁放置。 | 过梁 | 部分接线 | 原指令样例 | 部分 |

### 立面支撑（柱 / 托架 / 塔楼）

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-WALL-13 | `src/construct_elevation_supports/mod.rs` | `detect_overhang_support_segments`、`overhang_subscribe_to_tabloids`、`construct_circle_supports`、`construct_rectangle_columns`、`construct_rectangle_brackets`。 | tabloid | 部分分析 | 原指令样例 | 部分 |
| UM-WALL-14 | `src/construct_elevation_supports/util_pillar_construction.rs` | `construct_brick_columns`、`construct_wooden_pillar_w_stone_base`。 | 砖列 | 部分分析 | 原指令样例 | 部分 |
| UM-WALL-15 | `src/construct_elevation_supports/circle_pillars.rs` | `construct_circle_pillars`。 | 圆墙 | 部分分析 | 原指令样例 | 部分 |
| UM-WALL-16 | `src/construct_elevation_supports/circle_turret.rs` | `construct_turret`。 | 圆墙 | 部分分析 | 原指令样例 | 部分 |
| UM-WALL-17 | `src/construct_elevation_supports/circle_bottom_layer.rs` | `construct_circle_bottom_layer`。 | 圆墙 | 部分分析 | 原指令样例 | 部分 |
| UM-WALL-18 | `src/construct_elevation_supports/circle_categorize.rs` | `circle_categorize`。 | 圆墙 | 部分分析 | 原指令样例 | 部分 |
| UM-WALL-19 | `src/construct_elevation_supports/rectangle_pillars.rs` | `construct_rectangle_pillars`、`find_stone_column_start_y`。 | 矩形墙 | 部分分析 | 原指令样例 | 部分 |
| UM-WALL-20 | `src/construct_elevation_supports/rectangle_large_brackets.rs` | `construct_rectangle_large_brackets`。 | 矩形墙 | 部分分析 | 原指令样例 | 部分 |
| UM-WALL-21 | `src/construct_elevation_supports/rectangle_small_brackets.rs` | `construct_rectangle_small_brackets`。 | 矩形墙 | 部分分析 | 原指令样例 | 部分 |
| UM-WALL-22 | `src/construct_elevation_supports/rectangle_bottom_layer.rs` | `construct_rect_bottom_layer`。 | 矩形墙 | 部分分析 | 原指令样例 | 部分 |
| UM-WALL-23 | `src/construct_elevation_supports/find_overlap_segments.rs` | `find_overhang_support_segments`、`OverhangSupportFinderCache::new`。 | 几何 | 部分分析 | 原指令样例 | 部分 |
| UM-WALL-24 | `src/construct_elevation_supports/maintain_stone_column_colliders.rs` | `maintain_stone_column_colliders`。 | rapier/碰撞 | 部分分析 | 原入口对照 | 部分 |
| UM-WALL-25 | `src/construct_elevation_supports/util_cross_brick_pillar.rs` | 交叉砖柱（无独立符号，需定位内联宿主）。 | 砖列 | 无 | 原指令样例 | 未开始 |
| UM-WALL-26 | `src/place_pillars.rs` | `place_pillars`、`PillarProposals::add_proposal`。 | 支撑 | 无 | 原指令样例 | 未开始 |

### 门 / 栅栏 / 地板 / 楼梯

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-WALL-27 | `src/construct_gates/systems.rs` | `construct_gates`（120B ConstructGateCmd → CPU 木门/支撑/post fallback）。 | 门几何 | 部分候选 | 原入口对照 | 部分 |
| UM-WALL-28 | `src/construct_gates/mod.rs` | `construct_wooden_gate_doors`、`DoorMeshInProgress::add_quad`。 | 门几何 | 部分候选 | 原指令样例 | 部分 |
| UM-WALL-29 | `src/construct_fences/mod.rs` | `construct_fences`。 | 栅栏 | 部分候选 | 原入口对照 | 部分 |
| UM-WALL-30 | `src/construct_fences/process_segment.rs` | `split_curve_based_on_angle`。 | 曲线 | 部分候选 | 原指令样例 | 部分 |
| UM-WALL-31 | `src/construct_floors.rs` | `construct_floors`。 | 地板 | 无 | 原入口对照 | 未开始 |
| UM-WALL-32 | `src/construct_platform_stairs/mod.rs` | `construct_platform_stairs`（80B 装饰源→cells/碰撞/96B 记录/holes/stats）。 | 平台楼梯 | 部分接线 | 原入口对照 | 部分 |
| UM-WALL-33 | `src/construct_playermade_stairs/mod.rs` | `construct_playermade_stairs_arches_n_walls<true>`、`preprocess_curve`。 | 楼梯 | 部分 | 原指令样例 | 部分 |
| UM-WALL-34 | `src/construct_playermade_stairs/brackets.rs` | `construct_playermade_stair_brackets`。 | 楼梯 | 部分 | 原指令样例 | 部分 |
| UM-WALL-35 | `src/construct_playermade_stairs/magic_support.rs` | `playermade_stair_floating_bricks`。 | 楼梯 | 部分 | 原指令样例 | 部分 |
| UM-WALL-36 | `src/construct_playermade_stairs/pillars_visual.rs` | `construct_playermade_stair_pillars_visuals`。 | 楼梯/视觉 | 部分 | 原指令样例 | 部分 |
| UM-WALL-37 | `src/construct_playermade_stairs/pillars_process_proposals.rs` | 柱提案处理（无独立符号，需定位内联宿主）。 | 楼梯 | 无 | 原指令样例 | 未开始 |

### 半木 / 抹灰

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-WALL-38 | `src/curved_halftimber/mod.rs` | `curved_halftimber_maintain_entities`。 | 半木 | 无 | 原指令样例 | 未开始 |
| UM-WALL-39 | `src/curved_halftimber/wooden_floor_borders.rs` | `wooden_floor_borders`。 | 半木 | 无 | 原指令样例 | 未开始 |
| UM-WALL-40 | `src/plaster_systems.rs` | `create_plaster`、`maintain_plaster`、`split_wall_into_sides`。 | 抹灰 | 无 | 原指令样例 | 未开始 |

### UI 控制

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-WALL-41 | `src/ui_arch_controls.rs` | `ui_arch_controls`、`do_ui_arch_controls`。 | toolbar UI | 无 | 原指令样例 | 未开始 |
| UM-WALL-42 | `src/ui_lintel_controls.rs` | `ui_lintel_controls`。 | toolbar UI | 无 | 原指令样例 | 未开始 |

### utils

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-WALL-43 | `src/utils/half_timber.rs` | `half_timbering`、`overlapping_windows`、`roof_outline`、`get_window_aabb`、`check_for_overlap_with_all_windows`。 | 几何 | 无 | 原指令样例 | 未开始 |
| UM-WALL-44 | `src/utils/wall_corners.rs` | `add_wall_corners`。 | 墙角 | 无 | 原指令样例 | 未开始 |
| UM-WALL-45 | `src/utils/wall_skirts.rs` | `add_skirts`。 | 墙裙 | 无 | 原指令样例 | 未开始 |
| UM-WALL-46 | `src/utils/resolve_hole_overlap.rs` | `resolve_hole_overlap`。 | hole | 部分候选 | 原指令样例 | 部分 |
| UM-WALL-47 | `src/utils/trim_rows.rs` | `trim_rows`、`above_to_below`、`below_to_above`。 | 砖行 | 部分候选 | 原指令样例 | 部分 |
| UM-WALL-48 | `src/utils/split_into_random_rows.rs` | `split_into_random_rows`。 | 砖行/RNG | 部分候选 | 原指令样例 + RNG 序列一致 | 部分 |
| UM-WALL-49 | `src/utils/mod.rs` | `perturb_row_ends`、`split_horizontally`。 | 砖行 | 无 | 原指令样例 | 未开始 |
| UM-WALL-50 | `src/utils/sine_wave.rs` | 正弦波工具（无独立符号，需定位内联宿主）。 | — | 无 | 原指令样例 | 未开始 |
| UM-WALL-51 | `src/utils/stairs_signifiers.rs` | `stairs_signifiers_new`。 | UI | 无 | 原指令样例 | 未开始 |

### 插件 / GPU 辅助

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-WALL-52 | `src/lib.rs` | `gpu_clear_wall_gate_pieces`、`gpu_flush_plank_buffers`、`debug_force_update_arches` + 插件注册。 | rhapsody | 仅索引 | 原入口对照 + 注册顺序 | 未开始 |

## 卡点

- 全部任务消费 core 墙权威/几何/hole；core 侧未闭合前用冻结契约。
- `maintain_stone_column_colliders` / 平台碰撞依赖 rapier（与 decorator 共用物理后端）。
- GPU 辅助（gpu_clear_wall_gate_pieces / gpu_flush_plank_buffers）依赖 rhapsody，GPU 消费后置。
- 冻结后新增源码不继承旧报告（MODULE_PROGRESS 已登记）；每个 family 独立验收，不互相继承。

## 验收标准

- 每 family ≥ 原入口对照或原指令样例；96B 砖记录字段/顺序与原一致；RNG 序列逐位一致。
- 复用已有 construct_walls/分段/拱/门/栅栏基础，不重新推导。
