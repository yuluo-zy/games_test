# WP20 Contract — `ui_*` systems + misc systems（剩余 UI 与杂系统）

Source: `skeleton/B/crates/country-core/src/systems/{ui_2d_follow_mouse,ui_camera_mode_gizmos,
ui_edit_focus,ui_edit_freehand_walls,ui_focus_mode_shape_height,ui_move_freehand_walls,
ui_move_shapes,ui_raise_shape,ui_clutter/*,arrow_gizmo_animation,brush_preview,clear_canvas,
despawn,mesh_follow_terrain,mod_key,utils,utils_intersect}.rs`
Evidence: Ghidra decompiles in `analysis/b5_ui/`,`b6_clutter/`,`b7_misc/`; PDB bundles 同名。
与 wp15 `systems::ui`（UiState/UiWidgetContext/WidgetId/signifiers）、wp16 backend、wp6
Transform 对接，不重复定义。

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。

## ui_* systems

- **ui_2d_follow_mouse.rs** @0x14092a100（O）：puffin scope；Query 两分支（标志 @+0x108），
  每个实体取源索引 `@+0x94`（stride 0xa0）/`@+0x10`（stride 0x48），把游标位置 Vec2 写进
  实体局部数组（索引来自全局表 `@+0x110`）——2D UI 标记跟随鼠标。
- **ui_camera_mode_gizmos.rs** @0x1409527a0 + closure$0 @0x140952b60（O）：模式
  （Timelapse 等）且相机模式字节 @+0x4d0 非 Free 时：窗口过小 → `UiState::add` 注册 widget；
  `AppendOnlyVec::push<UiAppendSignifier>` 推 gizmo 指示符 `{tag,size,pos,fov}`。
- **ui_edit_focus.rs**（O）：`ui_edit_focus` @0x1409771f0（3411B）——编辑焦点跟随游标/地形；
  `geo_src_to_edit_subject` @0x140977f50、`geo_src_to_edit_subject_w_deco_clutter` @0x140978000
  ——几何源→`EditSubject`（含 deco/clutter 变体）；`TerrainFocusModeOrigin`（O 名，焦点原点）。
- **ui_edit_freehand_walls.rs** @0x140ac98e0（O，12129B）：自由手绘墙编辑（`EditFreehandWallCache`
  O 名，笔划缓存 → 墙生成）。
- **ui_focus_mode_shape_height.rs** @0x140a623e0（O，9215B）：焦点模式形状高度——按游标射线
  投影调整所选形状高度；`LerpedGizmoPosition`（O 名）。
- **ui_move_freehand_walls.rs** @0x1409781d0（O，7816B）：移动自由墙（`MoveWallCache` O 名）。
- **ui_move_shapes.rs**（O）：`ui_move_shapes` @0x14097a060（12822B）、
  `move_gizmo_and_trigger_vol` @0x14097d280（2618B，`MoveShapeCache` + 触发 volume 更新）。
- **ui_raise_shape.rs**（O）：`ui_raise_shape` @0x140952e70（8742B）、`impl_raise_shape`
  @0x140955650（4211B）——`utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::
  project_onto_ray` 计算抬升量 + `resources::history::History::add`；`RaiseShape`、
  `HandleRaiseShapeUiInteraction`（O 名）。
- **ui_clutter/**：
  - `debug_spawn_all_clutter` @0x1409a1f50（O）：消费 `DebugSpawnClutterCmd`/`DebugSpawnAllClutterCmd`
    批量生成 clutter。
  - `decoclutter_show_pins` @0x1412357c0（O，6719B）：显示/隐藏已固定 deco-clutter 图钉
    （`DecoClutterLock`、`ShowPinnedDecoClutter` O 名）。
  - `defered_autoclutter_history` @0x1409aa3e0（O）：`history_record_auto_clutter` 延迟
    记录（`RecordAutoClutterHistory` O 名）。
  - `rmb_autoclutter` @0x140976ca0（O）：`handle_rmb_autoclutter` 右键移除 auto-clutter。
  - `ui_autoclutter` @0x14099ca60（O，8136B）：`ui_convert_autoclutter_to_decoclutter` 右键
    转 deco-clutter（`InstaCreateDeco`/`InstaCreateStairPoint`/`StartPlaceDecoClutterInteraction`/
    `EvRightClickedAutoClutter` O 名）。
  - `ui_focus_decoclutter` @0x1409a92c0（O，11278B）：焦点模式 deco-clutter 移动/旋转
    （`RequestMoveClutter`/`RequestRotateDecorator`/`LastClonedClutter` O 名，4 个 closure）。

## misc systems

- **arrow_gizmo_animation.rs** @0x140a92f80（O）：puffin scope；`EnableArrowGizmoStretch`
  组件驱动的箭头 gizmo 拉伸动画。
- **brush_preview.rs** @0x140865050（O，5355B）：`BrushPreview`/`OverwriteBrushPreview`
  （O 名）——笔刷预览网格跟随游标/形状。`register_required_components` @0x140866800。
- **clear_canvas.rs** @0x140a7abd0（O）：清空画布——`InnerWalls::remove_all` + 7 次
  `AssetTextureLibrary::clear_preset_texture` + 各 `WorldRaster<512>::clear_all`。
- **despawn.rs** @0x1408b3510（O）：`despawn_glade_entities`——按标志 @+0x108 两分支遍历
  实体 id 数组（stride 0xa0/0x48）→ `EntityCommands::despawn`。
- **mesh_follow_terrain.rs** @0x140a68430（O）：`mesh_follow_terrain`——网格跟随地形高度
  （"MeshFollowTerrain got an invalid mesh handle" panic）。
- **mod_key.rs**（O/I）：`Modifiers` 组件 + `ToggleSnapping`（O 名）；`disable_snapping`/
  `is_disable_snapping_just_pressed` 折叠单行（与 `ActiveInputMode::is_controller`
  @0x1408c5680 折叠）；`update_mod_key_state` PDB rva @0x140AF9000 落在 `Toolbar::new`
  （ICF/折叠，U 实际地址）——读 ctrl/shift 键态写 `Modifiers`。
- **utils.rs** @0x140945ed0（O）：`add_stages`——`enum_iterator::All<GameSystemSet>` →
  `configure_sets(Update)`（各 GameSystemSet 变体）。
- **utils_intersect.rs** @0x14175ada0（O）：`line_x_shape_overlap`——线段×圆
  （`Circle2d::intersect_ray`）/×矩形（`Rectangle2d::orientation_mat2`、`Aabb2::from_center_dims`
  /`intersect_ray`），t 值 fmod wrap + `utils::lerp`。
- **collision/**：全量属 wp16 恢复范围（`world.rs`/`intersect_meshes.rs`/`visualize_colliders.rs`），
  本包不重复，仅登记引用。

## Unknown (U / typed gaps)
- 各大 UI 系统内部 UI 交互状态机、widget 布局、`MoveShapeCache`/`MoveWallCache`/
  `EditFreehandWallCache`/`LerpedGizmoPosition`/`TerrainFocusModeOrigin` 字段；`Modifiers`
  布局；`EditSubject`/`EraserLayer`/`CobblestoneVariant` 变体集合；`Toolbar`（0x140af9000
  = `Toolbar::new`，11 类工具集 + demo 裁剪，O 但属 toolbar 资源域）。

Reused deps：bevy_ecs、egui、append-only-vec、puffin、enum-iterator。
