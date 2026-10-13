# 合同：systems 杂项（backend / collision / debug / paths / steam / ivy）

覆盖 `systems/backend/*`、`systems/collision/*`、`systems/debug/*`、
`systems/paths/*`、`systems/steam/*`、`systems/ivy/*`。
Rust：`src/systems/…`（与骨架同名文件）。

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。

## backend（12 文件）

- **插件**：`add_backend_systems`（0x140942c80）/ `insert_backend_resources`
  （0x140942f80，注册 GameFrameIndex/Toolbar/camera params/GrowGrass Timer/
  MeshWatch/ConfigWatch/TextureWatch）/ `add_backend_events`（0x140943660，
  `TimedFrameEvent`）。
- **configwatch / meshwatch / texture_watch**（0x14085b1c0 / 0x14085b4b0 /
  0x140aab370）：每帧排空 `*Watch` 的变更字节队列 → 重载 RenderConfig /
  `AssetMeshLibrary`（`load_json_as_mesh` 路径）/ `AssetTextureLibrary`。
- **vao_update**：`build_missing_vaos`（0x1408646b0）遍历无 VAO 网格 →
  `AssetVAOLibrary::add`，成功后插 `Handle<Vao>` 组件（0x28B 命令）；空网格日志
  "Mesh is empty. Skip building VAO"。`rebuild_vaos`（0x140864bf0）按绑定差异
  `rebuild_vao`。
- **grow_grass**（0x14172fe80 + `platform::is_grown`）：启动后 Timer 延迟生长。
- **transient_mesh**：`delete_dropped_transient_meshes` 消费
  `TransientMeshRegistry::drain_meshes_to_delete`（0x1412e9e70）释放 GPU 网格。
- **icc**：`inner::update_window_icc_profile`（0x14093d830）显示器 ICC 变更 →
  读文件 → `cmsOpenProfileFromMemTHR`（lcms2 复用）→ `generate_icc_lut` →
  GPU 上传；`inner::impl$0::build`（0x140a58440）注册系统/事件。
- **prepare_frame_constants**（0x140af40b0）：帧序号推进写 `& 3`（+0x108），
  TAA jitter 按 `USE_TAA_JITTER` 全局采样 `%table_len`，
  `ViewportConstantBuilder::build` + `ViewConstants::set_pixel_offset`。
- **flush_screen_log**：`flush_error_log_to_screen`（0x1408c0d90）/
  `log_error_to_screen`（0x1408c10a0）/ `log_warning_to_screen`（0x1408c1310）。
- **game_frame_index**：全内联（`GameFrameIndex` 每帧 +1）。
- **panic_reporter_fetch_active_session**：全内联，向崩溃报告器供活动 session。

## collision（3 文件）

- **world.rs（rapier 胶水）**：`RaycastWorld`/`Rapier`/`RapierSubWorld`/
  `RaycastMesh(+Data/Key)`/`RaycastGeoSource(+Flag)`/`SubworldIndex`（16 常量：
  Arches/AutoClutter/Column/DecoratorCollision/DecoratorInteraction/Fence/Floor/
  Roof/Stairs/StairInteraction/StairSegment/StairSupport/
  SupportedStairStructural/Terrain/Tree/UnsupportedStairStructural）/
  `SubworldColliderHandle`/`ColliderRegistry`/`ColliderMeshInstance`/
  `MeshInstanceSources`/`OffsetColliderTransform`/`RaycastWorldResult2`/
  `Raycasting`/`RotatedBox`/`ColliderDelta`/`ColliderDiff`。
  - `RaycastMeshData::new_{convex,indexed,unindexed,cuboid,sphere,compound}`
    （0x1408cec10/0x8cee40/0x8cf7d0/0x8cfff0/0x8d0120/0x8d0230）：顶点/法线/
    索引 + 去重哈希（`utils::calculate_hash`）+ parry `SharedShape`。
  - 查询：`raycast`（0x1408cc870）/`raycast_simple`（0x1408ccc20）/
    `raycast_simple_exclude_wall_id`（0x1408ccc50）/`raycast_intersections_with_ray`
    （0x1408cc8a0）/`raycast_w_terrain`（0x1408cd430）/`intersects_any`
    （0x1408cd5b0）/`spherecast`（0x1408ccfc0，子世界位掩码驱动 cast_shape）/
    `legacy_*`（0x1408cc680/0x8cc770/0x8ccc90）。
  - 更新：`update_rapier_colliders`（0x1408d0490，含 `construct_arch_geo`
    0x1408d7ed0/`floor_mesh`）、`flush_rapier_colliders`（0x1408d8360）、
    `on_level_reload`（0x1408d8710）、`dispose_collider_mesh_instance`
    （0x1408d9590）、`upsert/remove_collider<T>`（WallId/DecoratorId/
    StairGraphEdge 泛型族）、`iter_shape_overlaps`（0x254890..0x129bad0，
    parry QBVH `intersections_with_shape`）、`shape_overlaps_with<Rectangle3d>`
    （0x2612d10）、`decorator_overlaps_with_others`（0x129be00）、
    `subscribe_to_changes`（0x1408cdc20）。
- **intersect_meshes.rs**：`find_mesh_mesh_intersections`（0x140a97b70）、
  `intersect_mesh_mesh`（0x140a9afa0，parry `traverse_bvtt`）、
  `should_intersect_collider_meshes`（0x140a9b3c0）、
  `merge_segments_into_polylines`（0x1408f2850，交点段→折线）+
  `polyline::{Polyline,PolylineMergeScratch,PolylinePoint,VertexId}`；
  `MeshIntersectionArtifacts`（petgraph GraphMap 边权）。
- **visualize_colliders.rs**：`visualize_colliders`（0x141326370）/
  `visualize_shape`（0x141326930）—— parry 形状→线框 debug_viz。

## debug（3 文件）

- `human_scale_ref`（0x1408b2b30）/ `sheep_controls`（0x1416f11a0 +
  `SheepControl::new` 0x1416f0ea0 + `apply_sheep_snapshot` 0x1416f24f0 +
  `PickingMode`）/ `lazy_grid_debug`（全内联）。

## paths（4 文件）

- `draw_path`（0x140ae4d00）：UI 拖拽 → `DrawState::update`（0x140ae4bd0 状态机）
  → 按变体（path/cobblestone1..3/eraser）对 `WorldRaster<512>` 做
  `world_raster::draw_raster`，同时 `erase_other_path_variants` 擦其它变体。
  `PathRastersResources::as_float_grid`（0x140ae4c20）：4 路（CPU +8/+0x10 vs
  GPU 暂存 +0x20/+0x28，按 +0x298 标志）f32 采样视图。
- `record_path_history`（0x140b1c270）+ 4 命令实体；`path_brush_preview_size`
  （0x140b1c010）+ `SetSizeBasedOnAppModePath`；`impl$0::build`（0x140b68e40）。

## steam（2 文件）

- `AppSteamIntegration`（`try_new` 0x140ae94b0 / `client` 0x67dd80 /
  `user_id` 0x140ae96f0）、`SteamClientResource`（
  `show_floating_gamepad_text_input` 0x140ae9390 /
  `is_floating_gamepad_text_input_open` 0x140ae9470 /
  `input_show_binding_panel` 0x140ae9480）、`SteamOverlayOpenResources`、
  `PrevSteamInputState`、`SteamScreenshotRequest(s)`。
- `startup_steam`（0x140a9bbf0）+ `check_get_{action_set,digital_action,
  analog_action}_handle`（`SteamMainActionSet/SteamMenuActionSet::try_load` 内联）。
- `add_steam_integration`（0x140945320）、`update_steam_inputs`（0x140ae9740）
  把 Steam 数字/模拟动作映射到游戏输入资源。

## ivy（9 文件）

- `ivy_grower`（0x14094a910）+ `get_growth_point_proposal` /
  `get_ivy_x_direction_relative_to_wall_direction` / `check_for_wall_jump` /
  `get_wall_double_sided_normal`（0x14094caa0）/ `closest_segment_index`
  （0x14094c800）/ `intersect_ivy_growth_w_wall_segment`（0x14094cc60）——
  沿墙生长、高度上限 `PublicWallState::max_ivy_grow_height`。
- `ivy_spawner`（0x14094cf90）+ `try_random_ivy_root` /
  `try_ivy_root_next_to_cursor` / `is_point_inside_wall_segment`。
- `ivy_pruner`（0x140997410）/ `is_point_still_valid`（0x140998dc0）/
  `ivy_pruner_cmd`（0x140999330）/ `remove_ivy_at_location`（0x140999860）+
  `RemoveIvyCmd`。
- `ivy_leaf_spawner`（0x140996080 + `IvySegmentDoneGrowingMsg`）、
  `ivy_leaf_animation`（0x140b0e380 + `IvyLeafIsGrowing`）、
  `ivy_branch_thickness_animation`（0x140995a80）、
  `ivy_send_to_gpu_leaf`（0x1408c1580 + `IvyLeafSsbo`/`IvyFlowerSsbo`）、
  `ivy_send_to_gpu_segments`（0x140a9b4e0 + `IvySegmentSsbo`）。
- 插件 `add_ivy_systems`（0x140944070）/`insert_ivy_resources`（0x1409440c0）/
  `add_ivy_events`（0x140944280）。

## 未知（U）

- backend：各 watch 重载目标字段映射、ICC LUT 布局、FrameConstants 字段投影、
  grow_grass 视觉过渡、日志队列位置。
- collision：`RaycastMeshData` 0x0..0x70 精确字段、rapier user-data 编码、
  `SubworldIndex` 位标志到 pass 的映射、相交伪影数据结构。
- paths：`DrawState` 状态转移表、`as_float_grid` 采样细节、brush size 1.5 分支。
- steam：动作集→游戏输入的映射表、overlay 回调细节。
- ivy：生长步长/约束数值、叶/段 SSBO 打包、pruner 半径。
