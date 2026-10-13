# 墙高调整契约 — adjust_wall_height/（5 单元）

## 1. adjust_wall_height_impl.rs（0x140ac1f20）

**签名（O）**：`EventReader<AdjustWallHeightCmd>, ResMut<InnerWalls>,
EventWriter<RequestWallMaterialAudio>, EventWriter<…>, Res<TerrainHeightsData<…>>`。
**业务（O，反编译 2235B）**：
1. `AdjustWallHeightCmd` 0x30B 行：ty u32@+0x10、height f32@+0x14、WallId@+0x18、
   follow-terrain flag。
2. `InnerWalls::get`；`clamp_wall_height` 返回夹取值，缺省回退命令 height；与当前高度
   相等 → 跳过。
3. follow-terrain：`TerrainHeightsData::sample_at_world_xz`（`Curve::get_coord_at_u` +
   `get_pos_at_u`）。
4. 预置形状（曲线点数 `< 2` 且 height-ty 默认）：`update_wall_state_height` + 0x28B 行
   {WallId, pos+height, u32, clamp01(inverse_lerp), 3}；否则 0x20B
   `RequestWallMaterialAudio` 行。
**typed gap**：0x28B 行第三字段/0x20B 行尾部语义 U。

## 2. ui_adjust_wall_height_mode.rs（0x140ac6d60 / 0x140ac7400 / 0x140ac79e0 / 0x140ac7f30）

- `clamp_wall_height`（0x140ac7f30，O 反编译 422B + S02 `reconstruction/wall/height_rules.rs`）：
  `PublicWalls` 查墙 → roof height_01；min = 1.6/0.7/0.3/0.5（roof≤0.1 / roof / special / 无 roof），
  max = 2.5（special）/ 14.0；MAXSS/MINSS 有序语义。
- `history_record_adjust_wall_height`（0x140ac79e0）：`wall_history::ev_height_changed` 历史。
- `get_closest_point_u_to_cursor`（0x140ac7400）：`points_with_u/segments_with_u` + 投影。
- `ui_adjust_wall_height_mode`（0x140ac6d60）：模式进出、`CursorPositionSS`/`MainCamera`
  光线 → `ProportionalScreenSpace::from_window_size`；TestCube 标记；decorator terrain cull。

## 3. ui_adjust_wall_height.rs（0x140b2fc00）

**签名（O）**：`Res<AppMode>, Res<PublicWalls>, Query<&Roof>, Res<UiState>,
EventWriter<…>, ResMut<UiWallHeight>, ResMut<UiSignifierStream>`。
**业务（O，调用链）**：未拖动时选中预置墙 → `UiWallHeight::new` 开高度控件；
roof flat/no-rim/正在动画分支。

## 4. adjust_wall_ui_mode.rs（0x1409566d0..0x1409576d0）

`UiWallHeight::{widget, signifier, new}`、`adjust_wall_height_curve`（0x140957390 系）：
widget/signifier 层（`UiState::intersecting`、`SignifierEntityBuilder`/`UiSignifierStream`）。
**typed gap**：曲线 UI 布局 U。

## 5. signifier_animation.rs（0x14085bb50）

**签名（O）**：`Query<(&mut WallHeightSignifier, &SignifierAnimation, &Handle<Mesh>,
Option<&SignifierMarkedForDeletion>)>, Res<PublicWalls>, ResMut<UiSignifierStream>, …`。
**业务（O，调用链）**：平屋顶无 rim（`Roof::is_flat_w_no_rim`）→ `adjust_wall_height_curve`
重算 + `Curve::crashy_new_from_points`；否则 `SignifierAnimation::t` 驱动。
**GPU 效果**：高度 signifier 动画（UI pass）。
