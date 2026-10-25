# 擦除（eraser）契约 — eraser/（4 单元）

## 1. animate_wall_bulldoze.rs（0x140ac27e0）

**签名（O）**：`ResMut<AnimateWallBulldozeBuffer>, ResMut<InnerWalls>,
EventWriter<RequestWallMaterialAudio>, Res<Time>`。
**业务（O；复用 `reconstruction/building/wall_bulldoze.rs`，`B-bulldoze-verification.json`
128 场景/768 全原 140ac27e0 入口-RET 通过）**：
- 命令行 152B：Source112、WallId+0x70、low/high+0x78/+0x7c、anchor Vec3+0x80、
  fractional phase+0x8c、finished+0x90。
- 每帧 `phase = clamp01(phase + GameTime+0x58 / 0.2)`（先推进后应用，CubicInOut）；
  缺 authority → 标 finished；phase==1 → 移除 authority（无音频）。
- 增长阶段写原 Source 曲线切片 + enclosure tag2 + style=(原 style==1)；
  source54..69 保持快照；每段一个 updated+60；完成一个 removed+90 无音频。
**GPU 效果**：无（CPU 权威移除 → publish 后段）。

## 2. ui_bulldozer.rs（0x1409de170 / 0x1409de190 20909B）

**签名（O）**：`wall_eraser_bulldoze_condition(Res<AppMode>) -> bool`（0x1409de170）；
`wall_eraser_bulldoze(Res<InnerWalls>, ResMut<AnimateWallBulldozeBuffer>,
Query<(&Roof, &RoofTriggerVolume)>, DecoratorArchivalResources, Res<UiState>, …)`。
**业务（O，调用链 + `evidence/building-wall-history` 选段 1409e1a11→1409df738 128 段验证）**：
- `handle_decorators` 排空墙属 decorator + terrain/stair cull（`DecoratorArchivist`
  `decorator_cull_allow_terrain/stairs`）。
- `can_reconnect_stairs_if_midpoint_deleted`、`StairsState::disconnect_edge`。
- 命中墙 → `AnimateWallBulldozeCmd::new` 入 buffer（RoofDelete FIRST + WallCompound
  StateChange(slice, op1)+Delete(slice) 双组次序 O）。
**typed gap**：命中/选区/锚点上游仍为解析调用者输入。

## 3. wall_eraser.rs（0x140accab0 20675B）

**签名（O）**：`Res<AppMode>, Res<AnimateWallBulldozeBuffer>, Res<CursorTerrainRaycast>,
DecoratorArchivalResources, Res<UiState>, ResMut<InnerWalls>, ResMut<WallEraserArchivist>, …`。
**业务（O，调用链）**：
- `determine_freehand_height_ty`、`wall_height_based_on_length`、
  `adjust_wall_height_if_erased_flat_roof_w_no_rims`（平屋顶无 rim 高度补偿）。
- 拖笔选段 → `WallEraserArchivist::affected_walls/has_anything_been_affected` →
  `InnerWalls::get_unwrapped/update_wall_state(_curve/_enclosure)` + `new_wall`。
- `ChildWall::new`（`utils::child_wall`）。
**typed gap**：eraser 段重连/子墙归属细节 U。

## 4. archivist.rs（0x14092e6e0..0x14092f990）

非 ECS 数据结构：`WallEraserArchivist::{new, record_rewiring_event, cache_if_original_wall,
shrink_wall, add_split_event, add_fully_erased_event, finish (21197B), record_erase_original_wall,
finish_erase_event, finish_shrink_or_split_event, affected_walls, has_anything_been_affected}` +
`ChildWalls` + `WallHistoryEdits::compile_history_edit`。
**业务（O，调用链）**：擦除期间累积 original/split/rewire 事件，`finish` 经
`CompoundEditBuilder::build` 编译为 WallHistoryCompoundEdit（O：`finish` 调 build/new_edit_sequence/
next_empty/with_concurrent）。
**typed gap**：archivist 内部缓冲字段 U。
