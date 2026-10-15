# systems/clutter 任务清单

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/systems/clutter` |
| package | `system-clutter` |
| 规模 | 20 文件 / 38 本 ns 函数 / 6 闭包 / **21 业务函数** |
| 难度 | 中 |
| 批次 | 2 |
| 上游依赖 | `country-core`（墙/屋顶 hole、区域）、`tabloid`、`rhapsody`、`fastrand`、`glam`、`utils` |
| 复现工程现状 | MODULE_PROGRESS：**部分候选实现**（9 恢复文件）；autoclutter 区域/筛选/放置/实体管理、merlons/stitches 已接可编译 crate；`clutter+audio` 5/5 测试用 glue 占位，未通过完整原作行为验证；core 侧 `recovered_systems/clutter.rs` |

## 任务表（按 B src 层级）

### autoclutter 核心状态与放置

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-CLUTTER-01 | `src/autoclutter/mod.rs` | 恢复 `AutoClutterState::{add_region, mark_dirty, remove_region_from_lazy_grid, flush_candidates_pending_removal}`：autoclutter 权威状态（区域/lazy grid/候选队列）。 | core 区域 | 候选 | 原指令样例 + 状态机与原一致 | 部分 |
| UM-CLUTTER-02 | `src/autoclutter/populate_autoclutter_regions.rs` | 恢复 `populate_autoclutter_regions`、`autoclutter_subscribe_to_tabloids`：区域填充 + tabloid 订阅。 | tabloid | 候选 | 原入口对照 | 部分 |
| UM-CLUTTER-03 | `src/autoclutter/process_autoclutter_candidates.rs` | 恢复 `process_autoclutter_candidates`：候选筛选/放置决策。 | 区域/筛选规则 | 候选 | 原指令样例 + 筛选规则与原一致 | 部分 |
| UM-CLUTTER-04 | `src/autoclutter/manage_autoclutter_entities.rs` | 恢复 `manage_autoclutter_entities`：实体生命周期管理。 | 实体系统 | 候选 | 原指令样例 | 部分 |
| UM-CLUTTER-05 | `src/autoclutter/read_bush_positions_from_gpu.rs` | 恢复 `read_bush_positions_from_gpu`：从 GPU 读回灌木位置（GPU 回读链）。 | rhapsody GPU | 无 | 原入口对照 + 回读格式一致；GPU 后置登记 | 未开始 |
| UM-CLUTTER-06 | `src/autoclutter/load_clutter_meshes.rs` | 恢复 `load_clutter_meshes`：clutter 网格加载。 | asset | 无 | 原入口对照 | 未开始 |
| UM-CLUTTER-07 | `src/autoclutter/autoclutter_animation.rs` | 恢复 `autoclutter_animation`：clutter 动画。 | 时间/风 | 无 | 原指令样例 | 未开始 |

### autoclutter 各放置族

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-CLUTTER-08 | `src/autoclutter/add_autoclutter_around_gates.rs` | 恢复 `add_autoclutter_around_gates`：门周边自动 clutter。 | 门几何 | 无 | 原指令样例 | 未开始 |
| UM-CLUTTER-09 | `src/autoclutter/add_autoclutter_around_windows.rs` | 恢复 `add_autoclutter_around_windows`。 | 窗几何 | 无 | 原指令样例 | 未开始 |
| UM-CLUTTER-10 | `src/autoclutter/add_autoclutter_on_windows.rs` | 恢复 `add_autoclutter_on_windows`（窗台花箱等）。 | 窗几何 | 无 | 原指令样例 | 未开始 |
| UM-CLUTTER-11 | `src/autoclutter/add_birdnests.rs` | 恢复 `add_birdnests`。 | 屋顶/墙 | 无 | 原指令样例 | 未开始 |
| UM-CLUTTER-12 | `src/autoclutter/add_lantern_water_setdresssing.rs` | 恢复 `add_lantern_water_setdresssing`：灯笼/水边装饰。 | 水/装饰 | 无 | 原指令样例 | 未开始 |
| UM-CLUTTER-13 | `src/autoclutter/add_preplaced_autoclutter.rs` | 恢复 `add_preplaced_autoclutter`：预置 clutter。 | — | 无 | 原指令样例 | 未开始 |
| UM-CLUTTER-14 | `src/autoclutter/add_tree_vegetation_setdressing.rs` | 恢复 `add_tree_setdressing`：树/植被点缀。 | 树/地形 | 无 | 原指令样例 | 未开始 |

### 屋顶雉堞 / 形状接缝

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-CLUTTER-15 | `src/inter_roof_merlons/construct_inter_roof_merlons.rs` | 恢复 `construct_inter_roof_merlons`：屋顶间雉堞构造。 | 屋顶几何 | 候选 | 原指令样例 | 部分 |
| UM-CLUTTER-16 | `src/inter_roof_merlons/detect_inter_roof_merlons.rs` | 恢复 `detect_inter_roof_merlons`：雉堞检测。 | 屋顶几何 | 候选 | 原指令样例 | 部分 |
| UM-CLUTTER-17 | `src/inter_roof_merlons/inter_roof_merlons_audio.rs` | 恢复 `request_inter_roof_merlon_audio`（音频跨切面）。 | 音频胶水 | 无 | 原指令样例 | 未开始 |
| UM-CLUTTER-18 | `src/inter_shape_stitches/detect_intra_shape_corners.rs` | 恢复 `detect_intra_shape_corners`、`add_hole_at_shape_intersection`、`WallAndRoofChanges::dirty_walls`。 | 墙/屋顶 hole | 候选 | 原指令样例 | 部分 |
| UM-CLUTTER-19 | `src/inter_shape_stitches/stitch_bricks.rs` | 恢复 `spawn_stitches`、`spawn_stitch_bricks`：形状接缝砖。 | 墙几何 | 候选 | 原指令样例 | 部分 |

### 插件

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-CLUTTER-20 | `src/lib.rs` | 恢复插件注册 `impl Plugin::build`。 | 全部上面 | 仅索引 | 与原注册顺序对照 | 未开始 |

## 卡点

- **`BrickTransformSsbo` / `StairsFloatingBrickParams` 字段与真实资源 producer** 未定（MODULE_PROGRESS 登记）；GPU 侧字段需从原侧定位。
- 现有 `clutter+audio` 测试用 glue 占位，不能计完整行为验收。
- autoclutter 依赖 tabloid 通道（DirtyTiles/ColliderDiff），其 State/StateDelta 实现在 core；需对齐接口。

## 验收标准

- 每函数 ≥ 原指令样例；放置规则（位置/朝向/密度/筛选）与原一致；GPU 回读格式有原侧证据。
