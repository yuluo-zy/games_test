# WP17 — 领域编辑历史资源类型（edit-history types）业务逻辑合同

B 版本（EXE sha256 `cf048e27…`，PDB sha256 `2d9686b8…`）。本文件记录
`crates/country-core/src/resources/history/` 的 10 个编辑记录容器 +
`replay_cmd_buffer.rs` 的 **Observed / Inferred / Unknown** 证据。
字段结构优先采用 `reconstruction/history/observed-history-schema.json`
（TG-HISTORY-OBSERVED-20261010，2,120 edit_chain 条目/2,774 命令/13 外层 tag）
观察模型；运行时类型用 PDB 骨架函数符号 + Ghidra 反编译核验。`roof_edit.rs`
不在 WP17 范围（屋顶域其它批次）。实现见 `src/resources/history/`。

## 通用说明
- 外层 EditType tag（O：观察 JSON 13 类）映射到 `EditType` 枚举判别
  （wp13 已实现 `EditType` + `from_observed_tag`）。
- 运行时 `EditType` 判别（O：`EditType::new_height_changed` 0x140a83fb0
  写 `*self = 7`）：墙高度变更 EditType 判别==7。完整判别表 U。
- MultiEdit 辅助容器（decorators/stairs，观察 JSON 的 `decorators.diffs` /
  `stairs`）负载 = `Arc<DecoratorHistoryDiff>` / `Arc<StairsHistoryDiff>`，
  经 replay_edit → ReplayBucket 附加（见编排合同）。

## 1. wall_history.rs（trait impl：EventHandlers）

### O
- `impl$0::ev_new_wall` 0x140a83300：新墙历史事件 → 构造 `EditType`。
- `impl$0::ev_height_changed` 0x140a83f80：`EditType::new_height_changed()`
  后 `History::add()`。宿主：`ui_adjust_wall_height_mode::
  history_record_adjust_wall_height` 0x140ac79e0。
- `EditType::new_height_changed` 0x140a83fb0：分配 `WallHistoryCompoundEdit`
  （0x108 字节负载拷贝），`Arc` refcount=`0x8000000000000002`；
  `from_iter<ConcurrentEdits, Map<IntoIter<Vec<Arc<WallHistoryEdit>>>,
  impl_0::new::closure_env_0>>`；写 `EditType` 判别 7 + 负载字段。

### U
- ev_new_wall / ev_height_changed 的完整参数（WallId + 高度值）。

## 2. wall_compound_edit.rs

### O
- `WallHistoryEdit`（enum2$）：墙历史编辑操作（观察 JSON 内层操作
  CreateWall/WallChanged/DeleteWall/AdjustHeight/MorphPremadeShape/
  ChangeStyle/ChangeElevation/ChangeArchHeight/MorphFreehandWall/
  ChangeIsEnclosedState/ChangeFreehandHeightType/ChangeArchType）。
- `ConcurrentEdits`：`Vec<Vec<Arc<WallHistoryEdit>>>`（观察 `[[{Operation}...],...]`
  组数组；`impl$0::new` 从 `Vec<Vec<Arc<WallHistoryEdit>>>` map 构建）。
- `WallHistoryCompoundEdit`：`new`（0x108B 负载，`Vec<ConcurrentEdits>` +
  Arc）；`impl$5::fmt` 0x140a87970；`impl$6/7/8` clone/fmt；serde
  （serialize/deserialize 手写 + visit_str/visit_enum 全套）。
- `CompoundEditBuilder`：`new_edit_sequence` 0x140a82b20（17 宿主点）、
  `next` 0x140a82b40、`build` 0x140a82d50（15 宿主点，返回
  `Option<WallHistoryCompoundEdit>`）、`next_empty`、`with_concurrent`。
- `panic_deprecated_operation` 0x140a83120：弃用操作 panic。
- wp12 已在共享 crate 的 `resources::history::wall_compound_edit` facade
  记录 `wall_deleted` 0x140a876a0（Delete(snapshot) 组）。

### U
- WallHistoryEdit 各判别负载字段（观察 schema 的 wall_index/curve/
  height/elevation/color_id/before/after 等为候选，非封闭）。
- CompoundEditBuilder 内部状态字段。

## 3. path_edit.rs

### O
- `RasterEdit`（观察 JSON Path/Cobblestone1..3 命令负载）：`points` /
  `brush_size` / `is_additive` / `blend_mode`（Path 234 例全有前三者，
  228 例含 blend_mode；Cobblestone* 同构）。3D 点 Vec。
- `impl$1::from` 0x140a976b0：某类型 → RasterEdit（生成端）。
- serde（手写 serialize/deserialize + visit_str/visit_map/visit_seq）。
- 消费端：replay_raster 的 `RasterEdit` Arc 负载（判别 0/1/2）；
  `raster_stroke::iter_points_xz` 闭包把 Vec3 点投影到 xz。

### U
- `RasterEdit` 内部是否含 filter/原始曲线（Water/Garden 的
  WaterRasterEdit/GardenRasterEdit 归属——观察中 Water/Garden 命令
  字段与 Path 同构，判别可能仍为 RasterEdit 系）。

## 4. terrain_edit.rs

### O
- `TerrainEdit`（enum2$；判别 3 的 ReplayCmd 负载 `Arc<Option<TerrainEdit>>`）：
  - `TerrainEdit::duration`（enumerate+fold 曲线点距累加，`duration::closure`
    内联；replay_terrain forward 用）。
  - `revert` 0x140858d30（12111B）：撤销（恢复 stroke 原状态 → EditTerrainStrokeCmd）。
  - `reapply` 0x1408592d0（5855B）：重放（应用 stroke → EditTerrainStrokeCmd）。
  - `update_terrain_curve` 0x140859740（14689B）：更新地形曲线。
  - `impl$0::fmt` 0x140858c80。
- 观察 JSON：NewTerrainStroke(index/height_ws/stroke_pts)、
  EditTerrainStroke(index/prev_height_ws/new_height_ws/prev_falloff/new_falloff)、
  MoveTerrainStroke(id/original_stroke_pts/move_vec)。

### U
- TerrainEdit 各判别负载（index/id/height/falloff/stroke 的精确字段）。

## 5. tree_edit.rs

### O
- `TreeEdit`（判别 8 负载 `Arc<Option<TreeEdit>>`）：serde 手写全套；
  `missing_prefab` 0x1409e6210（无 prefab 时处理）。
- `TreeCanopyMaxCullDistance::get_dist`（树冠最大剔除距离）。
- 观察 JSON：Tree Place(index/tree{id,position,scale,rotation_degrees,
  prefab_name,is_preplaced})、Tree Move(index/from/to)。

### U
- TreeEdit 判别负载字段；get_dist 数值。

## 6. color_edit.rs

### O
- `ColorEdit`（观察 JSON Color 命令嵌套 tag）：
  Wall/Roof/Halftimber/Decorator/StairRailing/FlatRoof/StairLadder/
  Brightness。字段：wall_id/roof_id/from/to/decorator_id/
  brightness（观察）。
- serde 手写全套（visit_enum 32 子 visitor）。

### U
- ColorEdit 判别与字段精确类型。

## 7. decorator_edit.rs

### O
- `DecoratorHistoryEdit`（判别 6/7 负载 `Arc<Option<DecoratorHistoryEdit>>`）：
  serde 手写全套（8 个 serialize 实例化点）。
- `DecoratorHistoryDiff`（观察 JSON `decorators.diffs` 批次：
  `[target, {before:{id:record}, after:{id:record}}]`；target
  {Wall:数字}/"Terrain"/"Stairs"；427 批/523 记录变化）。replay_edit /
  replay_pop_bucket 消费的 `Arc<DecoratorHistoryDiff>`。
- `DecoratorStorageHistoryDiff`（装饰存储 diff；decorator_archivist
  extract_diff / drop_inner_table<(DecoratorStorageAddr,
  DecoratorStorageHistoryDiff)>）。
- `send_state_diff_events<closure_env$0>` 0x14017bdae0：把 diff 状态
  变更事件发给 `replay_pop_bucket::closure_env$0`（`restore_decorators`）。
- `convert_before_after_states_to_changes` 0x140b31220（before/after
  状态 → 变更集）。
- `impl$1::from` 0x140b31240；`is_default_chimney_smoke` 0x1408c5680、
  `is_default_stair_width` 0x140b3d590、`is_default_lantern_brightness`
  （缺省值判定）。
- 观察 JSON 装饰记录字段：dst(Wall anchor/coord, Roof mesh_uv,
  Terrain pos_xz/yaw_radians, Stair graph_edge/uv/yaw_radians)、
  rank/seed/ty/color_id/allow_connection/chimney_h_offset/h_offset/clv/
  stair_width/railing/ladder_color。

### U
- DecoratorHistoryEdit 判别负载；diffs 目标键与 id 的精确类型。

## 8. stairs_history_edit.rs

### O
- `StairsHistoryEdit`（判别 10 附近负载）与 `StairsHistoryDiff`
  （观察 JSON `stairs` 字段：before/after 为整数边对数组，
  hash_before/hash_after）：serde 手写全套。
- `StairsHistoryDiff` 由 replay_edit → ReplayBucket 附加、replay_pop_bucket
  的 `StairsState::restore_backup`/`apply_diff` 消费。
- `is_stairs_history_diff_empty` 0x140a842a0（wp13 已实现）：`+0x30` 非空。

### U
- 边对/hash 的精确类型与算法。

## 9. auto_clutter_edit.rs

### O
- `AutoClutterEdit`（判别 9 附近负载）：serde 手写全套
  （serialize 0x140a18320/deserialize 0x140a18500）。
- 消费：replay_auto_clutter 恢复/删除自动杂物（WallId HashMap +
  DeletedAutoClutter）。

### U
- AutoClutterEdit 负载字段（自动杂物记录）。

## 10. replay_cmd_buffer.rs
见 `replay-orchestration.md`（ReplayCmd/ReplayBucket/ReplayBuffer）。

## 跨类型记录
- 观察 schema 为开放模型（非封闭枚举）；U 一律 typed gap，不臆造字段。
- serde 均按手写 `serialize_with`/`deserialize_with` visitor 推断
  （PDB 显示 visitor 族），字段序按观察 JSON（大整数保真）。
