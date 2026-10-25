# RESTORE-WALLS-R4R5 — valid_enclosure + snapping + eraser + shapes

> restorer-walls，2026-10-16。波 1 = valid_enclosure(45 函数)；波 2 = snapping/；
> 波 3 = eraser/ + shapes/。工作流：章节/算法证据 → PDB 骨架函数清单 → Rust 实现。
> 编译：`cargo check -p country-core` 通过；`cargo test -p country-core --lib`
> **1709 过 / 1 败**（唯一败项 = 基线既有 `geometry::sphere::uv_formula_matches_the_closure`，
> 非本模块）。本模块 22 测试全过。

## 0. 现状核对

- 波目标文件在任务前均 `no_formal_mapping`（inventory），`src/` 无
  `valid_enclosure/`、`snapping/`、`eraser/`、`shapes/`——全部新建。
- 依赖缺口：`InProgressCurve`（resources/in_progress_curve.rs，原文件 3 业务函数，
  snapping/eraser 消费）与 `WallSpatialHash::query_with_aabb_clamped`（0x140aa5a10）
  此前不存在，本批补齐。`WallId` 双域（`resources::walls` 与 `recovered_building::wall_state`）
  在 eraser 桥接处显式转换。

## 1. 波 1 valid_enclosure（45 函数：20 业务 / 2 内联 / 23 派生胶水）

`src/systems/wall/valid_enclosure/mod.rs`（全部证据 = 章节 valid_enclosure/mod.rs.md + DIGEST §2.6）。

| 原函数 | VA | Rust 位置 | 证据 |
|---|---|---|---|
| ShapeParameters::center | 0x1411ff2a0 | `ShapeParameters::center` | O，`payload[tag*8]` 两 f32 |
| ShapeParameters::as_curve2 | 0x1411ff2b0 | `Shape::as_curve2` | O，Circle2d::as_curve2(0.2)/Rectangle2d::as_curve2 |
| ShapeParameters::as_curve3 | 0x1411ff2f0 | `Shape::as_curve3` | O，Circle2d::as_curve3(0.2,z)/Rectangle2d::as_curve3(z) |
| ShapeParameters::as_circle | 0x1411ff6f0 | `ShapeParameters::as_circle` | O，非圆 fmt panic |
| ShapeParameters::as_rectangle | 0x1411ff740 | `ShapeParameters::as_rectangle` | O，非矩形 fmt panic |
| ShapeParameters::contains_point | 0x140ad2950 | `Shape::contains_point` | O，圆/矩形叶转发 |
| ShapeParameters::contains_shape | 0x1411ff330 | `Shape::contains_shape` | O，矩形含圆采样 {0,1/6,1/3,1.0,2/3,5/6}、含矩形 4 角 |
| ShapeParameters::get_normal_at_u | 0x1411ff600 | `Shape::get_normal_at_u` | O，位 0 判别双路 |
| ShapeParameters::get_pos_at_u | 0x1411ff620 | `Shape::get_pos_at_u` | O，同上 |
| ShapeParameters::get_closest_u_from_pos | 0x1411ff640 | `Shape::get_closest_u_from_pos` | O，矩形最近边 u（0x140c99530 重构） |
| ShapeParameters::perimeter | 0x1411ff660 | `Shape::perimeter` | O，2πr / 2(hw+hh) |
| ShapeParameters::expand | 0x1411ff680 | `ShapeParameters::expand` | O，圆 r+=a 且 +0x14/+0x18 车道 +=2a；矩形 hw/hh=2a 覆写（语义 U） |
| impl$0::from（内联 46/290） | — | `ShapeParameters::from_shape/from_snapshot` | 字节↔类型转换 |
| 派生 serde 23 项 | — | `ShapeParameters` 手写 Serialize/Deserialize（{tag,payload}）+ `IsEnclosed` 派生 | serde 名 IsEnclosedNoYesPremadeShapeIsEnclosedV2（serialize 0x1416eb640 变体 0/1="No"/"Yes"） |
| `IsEnclosed` enum | — | `IsEnclosed {No, Yes, PremadeShape(ShapeParameters)}` | O，serialize decomp |

**待取证**：① `contains_shape` 自圈分支叶调用 `get_pos_at_u(0)` 与章节公式"other.center"的出入；
② ShapeParameters JSON 精确 wire 格式（本章 serde 标记 U）；③ expand 矩形覆写语义（调用方在 render 预览域）。

## 2. 波 2 snapping（8 原文件）

`src/systems/wall/snapping/`。

| 原文件/函数 | VA | Rust 位置 | 证据 |
|---|---|---|---|
| mod: SnappingMode::new | 0x140ade610 | `SnappingMode::new`（mode=0,+0x20=0；snapped 不初始化） | O |
| mod: is_active / position_to_snap_to | 内联 67/74 | `SnappingMode::{is_active, position_to_snap_to}` | I，mode 判别 + 共享 +4..+0xc 区 |
| snapping_init | 0x140ada600 | `snapping_init` | O，重置→debug 门(+0xfd)→无进行中墙→曲线长≤1.65 早退→首段投影步行(1.5, eps 1e-20)→mode=1 + snapped |
| snapping_maintain: check_for_disable_snapping | 0x140a6a560 | `check_for_disable_snapping` | O，mode∈{2,3} 写 DisableSnapping{flag}，flag=1 同时清 mode |
| snapping_maintain: snapping_maintain | 0x140a6ad00 | `snapping_maintain`（发布到 +0x20 声音去重字节） | O，跳表分发打包 |
| snapping_maintain: snapping_signifiers | 0x140a6a650 | `snapping_signifiers`（mode2 端预览 / mode3 段预览→UiSignifierStream） | O 结构；signifier 流实体结构 待接线 |
| snapping_to_ends (+find_closest_end_to_snap_to) | 0x140a6ad80 | `snapping_to_ends` | O，Freehand 墙、自身墙用首原始点、1e8 哨兵、1.5 阈值、+0x29e 字节 |
| snapping_to_segments (+closest_segment_to_snap_to) | 0x140a6b5e0 | `snapping_to_segments` | O，±8.0 AABB 查询 + 1.5 筛选 + 地形采样写 height |
| snapping_audio | 0x140a6a480 | `snapping_audio` | O，mode2→kind1 / mode3→kind2，+0x20 去重；xvy swizzle=(snap.y,snap.x,snap.z) |
| telegraph_walls_are_merged | 0x14086ad70 | `telegraph_walls_are_merged` | O 结构；+0x284 偏移字段名/成对关系 待取证，kind-5 行→UiSignifierStream |

**新依赖**：`WallSpatialHash::query_with_aabb_clamped`（systems/wall/wall_state/wall_spatial_hash.rs，
0x140aa5a10 语义）；`resources/in_progress_curve.rs`（InProgressCurve::new/try_add_raw_input_points/
first/last/raw_input_curve_length_ws/is_raw_input_curve_valid 按章节；compute_final_curve 16KB 重采样 待取证）。
**代理资源**：`SnappingDebugDisabled`、`DisableSnappingInput`（B flag 资源本体待取证）。

## 3. 波 3 eraser（4 原文件）

`src/systems/wall/eraser/`。

| 原文件/函数 | VA | Rust 位置 | 证据 |
|---|---|---|---|
| wall_eraser | 0x140accab0 | `wall_eraser`（stroke→切片→enclosure tag2→wall_height_based_on_length 0.75..14→平屋顶 -0.79→archivist 缓存） | O 结构；ChildWall::new 分裂 待取证 |
| archivist: new / affected_walls / has_anything_been_affected | 0x14092e6e0 / 39 / 43 | `WallEraserArchivist::{new, affected_walls, has_anything_been_affected}` | O |
| archivist: cache_if_original_wall | 0x14092ea50 | `WallEraserArchivist::cache_if_original_wall` | O 结构 |
| archivist: record_rewiring_event / shrink_wall | 0x14092e8e0 / 0x14092eff0 | `record_rewiring_event` / `shrink_wall` | O |
| archivist: add_split_event / add_fully_erased_event | 0x14092f280 / 0x14092f630 | 合并于 `shrink_wall`/take | I |
| archivist: finish | 0x14092f990 | `finish_wall_eraser`（record_wall_bulldoze→RoofDelete FIRST + WallCompound→RecordRoofHistory/RecordWallHistory） | O |
| ui_bulldozer: wall_eraser_bulldoze_condition | 0x1409de170 | `is_bulldozer_layer`（AppMode::Eraser(Bulldoze)） | O，机器码 tag==9 && +4==3 |
| ui_bulldozer: wall_eraser_bulldoze | 0x1409de190 | `wall_eraser_bulldoze`（命令→AnimateWallBulldozeBuffer，WallId 去重） | O 结构；光线拾取/跳表分派 待接线 |
| ui_bulldozer: hammer_cursor_icon | 0x1409e35a0 | `hammer_cursor_icon`（光标图标谓词） | I |
| animate_wall_bulldoze | 0x140ac27e0 | `animate_wall_bulldoze`（phase+=dt/0.2→CubicInOut→切片→phase==1 移除 authority→retain_finished） | O；复用 recovered_building::wall_bulldoze::WallBulldozeCommand |

## 4. 波 3 shapes（~20 原文件）

`src/systems/wall/shapes/`。

| 原文件/函数 | VA | Rust 位置 | 证据 |
|---|---|---|---|
| mod: impl$0::add_wall_shape_ux | 0x140945f50 | `register_shape_resources` + registration.rs 8 系统注册 | O（哈希→系统映射 U） |
| calc_shape_height | 0x140ac80e0 | `calc_shape_height`（base rect 6/circle 8、-0.46、clamp[0.7,14]、写 preview+0x40、CalcShapeHeight{tag,cur,prev,delta}） | O |
| shape_placement: shape_placement_params | 0x14097f970 | `shape_placement_params`（光标位→AABB 查墙→1.5 吸附→法向） | O 结构 |
| shape_placement: snap_to_roof_ridge | 0x140980db0 | `snap_to_roof_ridge`（Roof::pivot_3d/RoofRidgeDims::calc 待取证近似） | 待取证 |
| shape_placement: new_unsnapped / closest_segment | 内联 70/456 | `ShapePlacement::new_unsnapped` | I |
| shape_placement_signifiers: preview_snapping_line | 0x1408dfc60 | `preview_snapping_line`（圆 +0.315/矩形 +0.63 扩张锚点） | O |
| shape_placement_signifiers: shape_placement_signifiers | 0x1408dec70 | `shape_placement_signifiers` | O 结构；outline spawn 待接线 |
| utils/shape_signifier_lerp: ShapeVal::interpolate | 0x1409ef010 | `lerp_shape`/`ShapeVal::{get_radius,get_rectangle_dims,interpolate}` | O；rate 常量 待取证 |
| rectangle/mod: get_rectangle_orientation | 内联 83 | `get_rectangle_orientation` | I |
| rectangle: create_rectangle | 0x140a93790 | `create_rectangle`（as_curve2→墙+enclosure→CreateRoofCmd(ridge_length_01)→历史） | O 结构；ridge 插值 待取证 |
| rectangle: edit_rectangle | 0x14097f620 | `edit_rectangle`（EditRectangleCmd→curve+enclosure） | O |
| rectangle: finish_edit_rectangle | 0x141464a60 | `finish_edit_rectangle`（复用 finish_rectangle_edit_decision 双判据 0.01；UiState80 变体 1/2） | O |
| rectangle: ui_create_rectangle (+PreviewRectangle::new, rectangle_dims_from_cursor_pos) | 0x14155d410 | `ui_create_rectangle` + `RectangleCanvasInput` | O 结构；dims 投影 0x140c99520 待取证 |
| rectangle: ui_edit_rectangle_dims (+do_ui/get_distance_to_edge) | 0x140ad6250 / 0x140ad6e00 | `ui_edit_rectangle_dims`/`do_ui_edit_rectangle_dims`（expand_along_edge） | O 结构 |
| rectangle: ui_edit_rectangle_orientation (+do_ui/box/skewed) | 0x140ad8000 / 0x140ad8be0 | `ui_edit_rectangle_orientation`/`do_ui_edit_rectangle_rotate`（8 阶 acos 多项式 + cross 符号） | O |
| rectangle: signifier_edit_dims_line (+register_required_components) | 0x1408dda10 / 0x1408e0e80 | `signifier_edit_dims` + `register_required_components` | O 结构 |
| rectangle/ui_modes: edit_dims widget/signifier | 0x14155fc70 / 0x14155eba0 | `ui_rectangle_dims_edit_signifier` / `ui_rectangle_dims_edit_widget` | I（widget 层 待接线） |
| rectangle/ui_modes: edit_orientation widget/signifier | 0x14083d3f0 / 0x14083da10 | `ui_rectangle_orientation_edit_widget` / `_signifier` | I（常量 1.5/0.3/0.25 已登记） |
| circle: create_circle | 0x14097e560 | `create_circle`（Circle2d::as_curve2(0.2)→墙+enclosure→CreateRoofCmd(profile 1.0)→RecordWallHistory::new_wall） | O 结构；roof_height_01 插值 待取证 |
| circle: ui_create_circle (+PreviewCircle::new, circle_signifier) | 0x14165bed0 / 0x14165cd40 | `ui_create_circle` + `CircleCanvasInput` + `circle_signifier` | O 结构；输入 action 0x1409236d0 待接线 |
| circle: ui_edit_circle (+finish_circle_edit/update_circle_radius/get_trigger_vol) | 0x14165d460 | `ui_edit_circle`（EditCircleCmd→curve+enclosure；FinishEditCircleCmd 0.01 阈值→StateChange 历史） | O 结构 |
| circle: ui_rotate_circle | 0x1409ac910 | `ui_rotate_circle`/`rotate_circle_wall`（旋转→重建曲线+enclosure） | O 结构；屏幕↔世界角 待接线 |
| circle/signifiers: edit_line signifier_edit_circle_line | 0x140a64a40 | `signifier_edit_circle_line`（0.23/0.75·90° 角偏移） | O |
| circle/signifiers: edit_arrow signifier_edit_circle_arrow (+clone_behavior) | 0x140b35120 / 0x140002370 | `signifier_edit_circle_arrow`（Freehand 无箭头）/`clone_behavior` | O |

## 5. 新增实现 / 依赖边界

- **新原文件**：`resources/in_progress_curve.rs`（3 业务函数，snapping/eraser 消费者必需，属"当前消费者必需合同"）。
- **修改既有文件**：
  - `systems/wall/mod.rs`：声明 4 新模块。
  - `systems/wall/registration.rs`：`register_wall_cpu_resources`/`add_wall_cpu_systems` 增 R4+R5 资源与系统
    （Edit 组 + Edit 后 Publish 前；Bulldoze 条件系统经 run_if 由宿主接）。
  - `systems/wall/wall_state/wall_spatial_hash.rs`：新增 `query_with_aabb_clamped`（0x140aa5a10）。
  - `resources/mod.rs`：声明 `in_progress_curve`。
  - `recovered_building/wall_state.rs`：新增 `float_0x284`/`byte_0x29e` 投影访问器（public_wall_state +0x284/+0x29e 字节）。
- **复用共享库**：utils::geometry{Circle2d,Rectangle2d}、utils::curve、utils::swizzlers::xvy、
  system_roof::surface::Curve2、recovered_building::{wall_bulldoze, wall_history_edit,
  wall_rectangle_edit, roof_creation, roof_shape}、history_recording::RecordWallHistory。
- **消费者链**：create_* → InnerWalls(created) → publish_public_wall_state；create_* → CreateRoofCmd →
  systems/roof/lifecycle::create_roof；create_*/finish_* → RecordWallHistory → systems/history。
  ui_create_* → Create*Cmd 事件（UI 生产者接线后闭合）。snapping_audio → PlayWallUiSound →
  systems/audio/wall_audio_ui。

## 6. 待取证 / 待接线（诚实分层）

- **待取证**：InProgressCurve::compute_final_curve 16KB 重采样；snap_to_roof_ridge 脊几何；
  create_circle/rectangle 的 roof_height_01/ridge_length_01 插值常量；ShapeParameters 精确 JSON wire；
  expand 矩形覆写语义；telegraph +0x284 偏移字段名；snapping +0x20 资源/2 参 flag 资源本体；
  shape_signifier_lerp rate 常量；preview/dims 投影 0x140c99520；WallBulldoze +0x29e 字节名。
- **待接线（库外 render/UI/输入域）**：signifier 流实体 spawn；hammer 光标图标；widget/trigger_volume 层；
  wall_circle/rectangle_canvas 输入 action（0x1409236d0）；射线拾取/跳表分派；Bulldoze 条件 run_if 宿主挂载；
  装饰恢复 extract_before_after_all；ChildWall 分裂。
- **未宣布完成**：本批为库内实现层面闭合；验收层面（上述消费端接线 + 全局调度）未闭合，
  不因 `cargo check` 通过标 review_passed。

## 7. 实测

- `cargo check -p country-core`：通过（余 6 warning 属他组在途：ON_LOAD_FENCE/SUBWORLD_SLOT_COUNT/LoadUpgradeFlags）。
- `cargo test -p country-core --lib`：**1709 过 / 1 败**；唯一败项 = 基线既有 sphere 测试，非本模块。
- 本模块 22 测试全过（valid_enclosure 5、snapping 8、eraser 1、shapes 8）。
- 他组在途故障未归因本模块：roof/stairs/replay_stairs 测试曾有瞬态失败（并发修复后消失）。
