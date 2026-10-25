# 墙状态发布/空间哈希契约 — wall_state/（6 单元）

## 1. publish_public_wall_state.rs（0x140a8b530，18515B）

**签名（O）**：`ResMut<InnerWalls>, ResMut<PublicWalls>, Res<WallSpatialHash>,
Res<TerrainHeightsData<…>>, EventWriter<OnWallChanged>, EventWriter<OnWallDeleted>, …`。
**业务（O，调用链 + 反编译入口）**：
- `gather_lazy_update_sources`（closure$0$0/closure$0）收集需重发布内墙；
- `process`（closure$0..$4）：重建 public 记录（Source 曲线点、shape、
  `WallStyle::is_halftimbered`、`material_stats`、`SparseInstanceBuffer::allocate`、
  WallSpatialHash 段成员维护）；`AnimateWallBulldozeBuffer::contains` 保留动画中墙。
- 发 `OnWallChanged`/`OnWallDeleted`（被 calc_wall_bend/delete_wall/spatial_hash_update 消费）。
**GPU 效果**：public 状态 → instanced wall SSBO/draw-list（W21/W27）。
**typed gap**：inner→public 复制字段映射、material_stats 布局 U。

## 2. spatial_hash_update.rs（0x140a8fd90）

**签名（O）**：`EventReader<OnWallChanged>, EventReader<OnWallDeleted>, Res<PublicWalls>,
ResMut<WallSpatialHash>`。
**业务（O，调用链）**：`OnWallChanged` → 曲线逐点 `get_pos_at_coord/sample_at_coord` 建段 →
`add_segment`；`OnWallDeleted` → `remove_wall_id`。

## 3. wall_spatial_hash.rs（0x1409ef210..0x140aa6600）

数据结构（PDB 过程 O）：`WallSpatialHash::{new, bounds_ws, dims, cell_size, cell_center,
cell_index_to_grid_coord, grid_coord_to_cell_index, world_space_to_grid_coord(_unclamped),
get_cells_from_aabb/circle/rectangle2d/segment, add_segment (0x140aa5dd0), remove_wall_id
(0x140aa5360), query(_with_segment/_with_circle/_with_aabb_clamped/_with_rectangle2d/
_with_circle_callback)}`、`WallSegment::terrain_height_closest_to_ws_pos`、`GridCell`。
**结构（O）**：固定 64×64 格（walls-audit）；每格持 (wall_id, segment) 成员；
`add_segment` 走 `get_cells_from_*` 闭包填格。
**GPU 效果**：无（CPU 空间索引 → 墙相交/吸附/支撑消费者）。
**typed gap**：bounds/cell_size 常量来源、段 u/terrain/normal metadata U。

## 4. wall_roof_hashmap.rs（0x140a94cd0）

**签名（O）**：`Res<AppMode>, ResMut<PublicWalls>, ResMut<WallEntities>,
Query<(Entity, &Roof, Option<…>)>`。
**业务（O，调用链）**：维护 `PublicWalls::roof_entity` 与 `WallEntities::wall_entity`；
预置形状无 roof → 清除。
**typed gap**：wall↔roof 关联来源 U（WP11 状态）。

## 5. spatial_hash_debug.rs（0x1408ec0e0）

**签名（O）**：`Res<WallSpatialHash>, Res<MouseDepthValue>, ResMut<UiSignifierStream>,
Res<DebugGui>`。
**业务（O，调用链）**：调试 GUI 打开时画每格 `DebugRectangle2d` + 成员线。

## 6. debug_viz_threshold_curve.rs（0x140a8b0c0）

**签名（O）**：`Res<DebugGui>, Res<PublicWalls>, ResMut<UiSignifierStream>`。
**业务（O，调用链）**：逐 public 墙沿发布曲线画 `DebugSphere`。
