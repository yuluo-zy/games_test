# 合同：components 残族 + geometry + utils 残量

覆盖 `components/{mod,transient_mesh}.rs`（beyond wp6 drawable）+ 残量
`geometry/{plane,cube,sphere,instanced_wall}.rs` + `utils/*` 游戏自有工具。
Rust：`src/components/{mod,transient_mesh,geometry,utils}.rs`

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。

## components/mod.rs（45 标记/参数组件，O 命名）

`ArrowGizmo` / `AutoClutter` / `AutoClutterConvertableToDecoClutter` /
`BackgroundTerrain` / `BalconyDoor` / `BrickColor` / `ClothlineCloth` /
`ConvertedPreplacedTree` / `DashedLineLength` / `DontReceiveContactShadows` /
`FlatLighting` / `FollowMouse` / `GothicWindowBrickDeformation` /
`HistoryButton` / `KeepOnLevelUnload` / `LanternLightParams` /
`MeshFollowTerrain` / `NormalMap` / `OverwriteRoughness` / `RoofFlagCloth` /
`SemiTransparent` / `SittableForBirds` / `Smoke` / `SnappingToEndPreview` /
`SnappingToSegmentPreview` / `SplashParticle` / `StairsStep` / `TestCube` /
`TreeAmbience` / `TreeAnimation` / `TreeComponent` / `TreeMarkedForDeletion` /
`TreeMoveBend` / `TreeStorageIndex`（u64，serde/hash 证据，`next` 内联）/
`Ui2DFollowMouse` / `Ui2dExtent` / `Ui2dImage`（`new` 内联于 UI/工具栏）/
`Ui2dWsExtent` / `UiDrawOrder` / `UiInactive` / `UiInteractionState` /
`UiMouseInteractable` / `UiNav` / `WallFlagCloth` / `WoodenGate`。
多数经 `impl$*::register_required_components`（0x140844f20..0x1408579f0 族）注册；
字段均为 U（typed gap）。

## components/transient_mesh.rs

- `TransientMeshRegistry`（O：`Res`；`Mutex<Vec<u64>>` 待删句柄队列）：
  `deferred_delete_mesh`（0x140aa8070，锁入队）/ `drain_meshes_to_delete<F>`
  （0x1412e9e70，排空逐条调闭包）。
- `TransientMesh` 组件（O：`Arc<TransientMeshRegistry>` + 网格句柄；
  `new_with_registry` 0x140aa8200）；`Drop`（`impl$2::drop` 0x140899f90）登记删除。
- 消费：backend `delete_dropped_transient_meshes`。

## geometry

- `Plane`（`impl$1::from` 0x140a06370）、`Cube`/`Box`（`Box::new` 内联于
  debug/startup_ivy；`impl$5::from` 0x140a05530）、`Sphere`（`Sphere::new`
  0x140b4af60 / `impl$2::try_from` 0x140a06b00）—— 字段 U。
- `instanced_wall`（O）：
  - `BrickSourceId`（`from_wall_id` 0x140858750 /
    `with_transient_stair_edge_id` 0x140858780）。
  - `InstancedWall`（`new` 0x140858800 / `update` 0x140858830 整体换砖表 /
    `upload_to_storage_buffer` 0x140858880：锁 writer、`clear`、逐砖 `push`
    （0x60B stride，`BrickTransform` 96B）、`gpu_flush`）。
  - `BrickTransformSsbo` / `BrickDrawListSsbo` / `BrickPrev|NextVisibilityBitmaskSsbo`
    （O：SparseInstanceBuffer / AssetSsboLibrary / ComputePass 读写）。
  - `WallBrickInstancedDrawProxy` 默认 AABB `0xc2c80000`=−100.0（O `impl$3::default`
    0x1408587c0）；绘制位标志 `impl$18::fmt::impl$0::*`（DISCARD_ABOVE /
    FORCE_LOD0 / LESS_SNOWY / NO_ERASER_HIGHLIGHT / ROOF_STITCH /
    SLIGHT_OVERLAP / STEPS）。

## utils（游戏自有 CPU 工具）

- `DirtyTiles`（O：512B=64×64 位图；`new_clear`/`clear` 0x140960ac0/0x960c20、
  `extend` 0x140960c30 逐 qword OR、`cell_from_world_space` 0x140960b20（<0x40
  越界）、`rasterize/intersects_world_space_rect` 0x140960ae0/0x960af0、
  `rasterize_world_space_stroke` 0x140ade620 经 `conservative_line_raster::
  conservative_plot_line`（复用本地 crate）、`iter_world_space_rect_cells`
  0x140ade8e0、`iter_tiles` 0x140adeea0、`retain` 0x140adfcb0、`set`/`is_set`
  内联）。消费：wall_state publish / terrain / autoclutter lazy grid / stairs。
- `debug_viz`：`DebugLine`（new 0x140b04e70 / color 0xb04eb0 / dashed 0xb04f10 /
  line_width 0xb04f40 / set_ui_pass 0xb04ee0 / show 0xb04f70）、`DebugSphere`
  （new 0x140b02490 默认半径 0.1 / color / radius / solid / show 0xb02520）、
  `DebugRectangle3d`（new 0x140b02e10 默认线宽 1.0）、`DebugRectangle2d`（show
  0x140b04640 经 `as_curve3`）；`show` 族 push 到 `append_only_vec::AppendOnlyVec`
  （复用）队列。
- `load_json`：`read_attribute_from_json`（0x14083e210）、`load_json_as_mesh`
  （0x140b0eeb0..0x140b11f50 泛型族；meshwatch 重载路径）。
- `geo`：`rewire_enclosed_curve`（0x140a6ba50）、`mesh_from_pts`（0x140a6c120）、
  `update_curve_mesh`（0x14157eac0 泛型族）。
- `audio_utils`：`AudioLoop`（current 0x140a593f0 / stop 0x140a59430 /
  start_if_not_playing 0x140a596b0 / is_playing 内联）、`AudioLoopDesc`（new 内联
  于 14 个 audio 初始化系统）。
- `project_cursor_onto_ray`：`ProportionalScreenSpace`（from_window_size
  0x140ac8420 / transform_screen_space 0x140ac8450）、
  `ProportionalScreenSpaceCoord`（from_screen_space 内联 / project_onto_ray
  0x140ac84a0）。
- `child_wall`：`ChildWall`（new 内联于 wall_splitter / slice 0x140a24000）。
- `mod.rs`：`get_res` / `get_res_mut` / `get_last_event`（0x1402b4910 泛型族）/
  `drain_events`（0x1402b47f0）/ `world_space_to_screen_space`（0x140ac8780）/
  `is_inside_map_bounds`。

## 未知（U）

- 45 个标记组件字段；`BrickTransform` 96B 内字段/位布局；`DirtyTiles` 单元→世界
  换算系数（世界尺寸/64）；debug_viz 队列消费与绘制 pass 接线；
  `load_json_as_mesh` JSON schema；`ChildWall::slice` 切分规则；`AudioLoop`
  handle 语义。
