# 形状（shapes）契约 — shapes/（21 单元）

## 1. shapes/mod.rs — add_wall_shape_ux（0x140945f50）

`fn add_wall_shape_ux(app: &mut App)`：注册 `CalcShapeHeight`/`PreviewCircle`/`PreviewRectangle`/
`ShapePlacement` 资源（O，调用链：4 个 default/new）。UI 装配，非 ECS 系统。

## 2. calc_shape_height.rs（0x140ac80e0）

**签名（O）**：`Res<AppMode>, Res<PreviewRectangle>, Res<PreviewCircle>,
Res<TerrainHeightsData<…>>`。
**业务（O，反编译 827B）**：
1. shape `{tag=6, ty}`；ty 1=circle 2=rectangle，否则预览高度 0 返回。
2. `Rectangle2d::as_curve2`/`Circle2d::as_curve2` → `crashy_resample_remap_per_segment`
   （地形采样闭包）→ 高度和 + 最大值。
3. base：rect 6.0 / circle 8.0；height-ty 2 → +地形；否则 `avg + base - 0.46`。
4. `final - (max - 0.46)` → clamp [0.7, 14.0]；写预览 +0x40；`time_continuous_lerp` 动画。
**GPU 效果**：预览高度驱动 shape signifier。

## 3. shape_placement.rs（0x14097f970 / 0x140980db0）

**签名（O）**：`shape_placement_params(Res<AppMode>, Res<CursorTerrainRaycast>,
Res<PublicWalls>, Res<WallSpatialHash>, ResMut<ShapePlacement>, …)`。
**业务（O，调用链）**：光标地形位 → `ShapePlacement::new_unsnapped`；
`closest_segment_to_snap_to`（WallSpatialHash 段查询 + `distance_segment_point`）；
`snap_to_roof_ridge`（`Roof::pivot_3d`/`RoofRidgeDims::calc` 屋顶脊线对齐）。
**GPU 效果**：placement → 形状预览/吸附线 signifier。

## 4. shape_placement_signifiers.rs（0x1408dec70 / 0x1408dfc60）

**签名（O）**：`shape_placement_signifiers(Res<AppMode>, Res<ShapePlacement>,
Res<PublicWalls>, ResMut<UiSignifierStream>, …)`。
**业务（O，调用链）**：吸附时画 `preview_snapping_line`（`ShapeParameters::expand`、
`Curve::get_tangent_at_u`/`try_slice`）+ 形状 outline signifier。

## 5. utils/shape_signifier_lerp.rs（0x1409ef010）

`ShapeVal::{interpolate, get_radius, get_rectangle_dims}`、`ShapeSignifierLerp::lerp_radius`：
circle/rectangle 值间插值。数学复用 glam。

## 6. circle/（create_circle 0x14097e560；ui_create_circle 0x14165bed0；ui_edit_circle
0x14165d460；ui_rotate_circle 0x1409ac910；signifiers/edit_arrow 0x140b35120；
signifiers/edit_line 0x140a64a40）

- **create_circle**（O，反编译 4275B + B-authority-create/14097e560.asm）：
  `CreateCircleCmd` → 圆形曲线（`Circle2d::as_curve2`→`crashy_new_from_points`）、
  enclosure=circle、style/elevation、`RoofShapeParams::from_wall_shape` +
  `roof_height_01_from_wall_shape` → `CreateRoofCmd`、历史。
- **ui_create_circle**：`CameraCursorRay::intersect_with_xz_plane` 拖拽半径 → PreviewCircle；
  `circle_signifier`（custom shader）。
- **ui_edit_circle**：`update_circle_radius`（拖触发体 `WallTriggerVolume::is_valid/
  edit_shape_default`）→ `update_wall_state_curve`/`update_wall_state_enclosure`；
  `finish_circle_edit` 历史。
- **ui_rotate_circle**：`world_space_to_screen_space` + DebugLine 旋转 → 更新曲线/enclosure；
  history 复合编辑。
- **edit_arrow/edit_line**：Query 驱动箭头/半径线 signifier 位姿。

## 7. rectangle/（create_rectangle 0x140a93790；edit_rectangle 0x14097f620；
finish_edit_rectangle 0x141464a60；ui_create_rectangle 0x14155d410；ui_edit_rectangle_dims
0x140ad6250/0x140ad6e00；ui_edit_rectangle_orientation 0x140ad8000/0x140ad8be0；
signifier_edit_dims_line 0x1408dda10；ui_modes/edit_dims 0x14155eba0；ui_modes/edit_orientation
0x14083d3f0）

- **create_rectangle**（O，反编译 4322B）：`CreateRectangleCmd` → 矩形曲线
  （`Rectangle2d::as_curve2` + `smoothstep` 角）+ `ridge_length_01_from_rectangle_ratio` +
  `roof_height_01_from_wall_shape` → `CreateRoofCmd`、历史。
- **edit_rectangle**（O，反编译 834B）：`EditRectangleCmd` → `update_wall_state_curve` +
  `update_wall_state_enclosure`。
- **finish_edit_rectangle**（O；复用 `reconstruction/building/wall_rectangle_edit.rs`，
  `B-rectangle-finish-verification.json` 192 全原调用通过）：UiState80 快照先发；
  维度 state1/朝向 state2；改变判据 `abs(acosf(prev_axis·cur_axis))*57.29578 > 0.01` 或
  维度差 `> 0.01`（有序 x86 float）；next enclosure 取真实 Source；记
  ConcurrentEdits + niche8000000000000004；无变化 → DecoratorArchivist 恢复。
- **ui_create_rectangle**：`rectangle_dims_from_cursor_pos` + `get_rectangle_orientation`。
- **ui_edit_rectangle_dims**：`get_distance_to_edge` 拖边 → 新 dims → `EditRectangleCmd`；
  `decorator_cull_allow_any`。
- **ui_edit_rectangle_orientation**：`box_trigger_volume`/`skewed_box_trigger_vol` 旋转 →
  `EditRectangleCmd`/`FinishEditRectangleCmd`；StairsPillarProposals 变体。
- **ui_modes/edit_dims·edit_orientation**：`UiRectangleDimsEdit`/`UiRectangleOrientationEdit`
  widget/signifier/trigger_volume（widget 层，调用链 O）。
**typed gap**：各 widget/触发体精确参数 U（UI 层）。
