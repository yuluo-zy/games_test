# WP21 — country-core 杂项资源余量（Tiny Glade B）

本包恢复 country-core 最后未覆盖的 **resources 杂项**：walls 余量 9、stairs 余量 7、
ivy/rt 余量（骨架子路径粒度）、misc resources 26、render/nani.rs、
systems/save/save_backup_metronome、systems/garden/{draw_garden,record_garden_history}。
方法按项目流程：**业务逻辑分析（PDB enriched/structures + Ghidra 只读反编译 + 调用链）→
整理合同（O/I/U + typed gap）→ 骨架对齐 Rust**；机器码差分仅作数值/边界裁决，不构成
实现手段。

版本 **B**（EXE cf048e27…，PDB 2d9686b8…）。

## 交付

| 路径 | 内容 |
|---|---|
| `contracts/walls-residual.md` | walls 余量 9（arches/dormers/wall_holes/wall_stone_columns/wooden_gates/decorator_collision/decorator_outline/decorator_subtype/elevation_support_stats） |
| `contracts/stairs.md` | stairs 余量 7（stairs_state/stairs_assembly_params/stairs_collider/structural_graph/stairs_removed_supports/invalidate + stairs_walls） |
| `contracts/ivy-rt.md` | ivy 余量 4 + rt 余量 3（扩展 wp16 合并文件到骨架子路径） |
| `contracts/misc-resources.md` | misc resources 26 + render/nani + save_backup_metronome + garden 系统 |
| `src/` | 骨架对齐 Rust（65 个 .rs） |
| `Cargo.toml` + `Cargo.lock` | 独立 offline gate（edition 2024，rust 1.86） |
| `evidence/` | Ghidra 只读反编译原始返回（g1..g9 result.json + 可读 txt） |
| `evidence.json` | 源 SHA、关键地址、先验复用、编译门禁 |

## 覆盖（按族；`*` = 新增，`~` = 扩展既有）

- walls 余量 9：`arches` `dormers` `wall_holes` `wall_stone_columns` `wooden_gates`
  `decorator_collision` `decorator_outline` `decorator_subtype` `elevation_support_stats`。
  `wall_holes`/`decorator_subtype`/`wooden_gates` 复用 building-holes / building-gates /
  building-decorator-info 先验，未重推导。
- stairs 余量 7：`stairs_state`（petgraph GraphMap 图 + 快照/diff/备份）、
  `stairs_assembly_params`、`stairs_collider`、`structural_graph`（碰撞支撑图）、
  `stairs_removed_supports`、`invalidate`（系统面）、`stairs_walls`（~WP12 facade）。
- ivy 余量 4：`ivy_storage`/`ivy_leaf_storage`/`ivy_direction_proposer`/`wall_coord`
  （~共享 crate 合并 ivy.rs → 骨架子路径；新恢复 IvyLeaf 104B 布局）。
- rt 余量 3：`rt/mod`/`rt/bvh`/`rt/bvh_builder`（~共享 crate 合并 rt.rs → 子路径）。
- misc resources 26：`clutter_meshes` `color_icon` `daily` `feedback_report_resources`
  `garden_mask` `glade_color_customization` `glade_settings` `gothic_window_profile`
  `image_formation` `mouse_capture` `photo_mode` `placement_data` `prefab`（~wp13）
  `preplaced_autoclutter` `raster_stroke` `render/spatial_resolve_offsets`
  `shader_refactoring_state` `shape_intersections` `shutdown` `sparkle` `steam`
  `texture_presets` `time_of_day` `toolbar` `ui_settings` `weather`。
- `render/nani.rs`（mod 胶水）、`systems/save/save_backup_metronome`、
  `systems/garden/{draw_garden,record_garden_history}`。

## 门禁

```sh
cargo check --offline    # 0 errors / 0 warnings（65 源文件）
```

只证明骨架对齐表面闭合；全部 `implemented_unverified`，无 runtime/GPU 验收。合同是
当前可审阅依据，不因编译通过声称原作等价。

## 关键恢复证据（节选）

- **WallHoleStorage** 0x68B（phase/raw/finalized/指纹/previous/长度/闭合）+ `finalize`
  0x140f87300（Aabb2、maxX 排序、跨 seam ±padding 副本 origin 0/1）。
- **StairsState**（petgraph GraphMap 权威图）+ endpoint 状态机 + make_backup 0x90B
  blob + extract_diff/apply_diff（restore_snapshot）。
- **IvyLeaf** 104B（`IvyLeafStorage::add` 0x1408fe280 全字段写）。
- **DormerHoleSsbo** 48B 构造器（gothic/cottage/dummy）。
- **ClutterMeshTy** 56 变体资产名匹配 + `clutter_mesh_ty_data` 数据表。
- **gothic_window_profile** 拱/门剖面曲线常量（size 1/2/3 逐 f32 转录）。
- **PlacementDataSsbo::to_affine3a**（绕 Y 旋转×scale）。
- **TodSequence::snapshot_at_tod** 两段插值。
- **Toolbar::new** 11 类模式表（demo 精简）。
- **SpatialResolveOffsetsSsbo::default_value** 2KiB 偏移表（结构 O，全表转录 U）。

## 集成说明

- 集成 owner 用整合 crate 真实类型替换 `src/boundary.rs`（WallId/DecoratorId/Aabb2/
  ColliderHandle/RaycastMeshData/StairsVisualState）；未知字段一律 `typed_gap::Gap`。
- steamworks / bracket-noise / obvhs / parry3d / perchance / rhapsody 为复用边界
  （缓存无者 typed gap）；petgraph 0.8.3、indexmap 2.7.1、foldhash 0.1.4 已按原锁
  基线声明并用于 stairs 图容器。
- 系统级接线（invalidate_dirty_stair_artifacts、mouse_capture、draw_garden、
  record_garden_history、save_backup_metronome）为 O 语义 + 骨架签名；完整 ECS
  参数由集成 owner 接真实组件/资源图。
