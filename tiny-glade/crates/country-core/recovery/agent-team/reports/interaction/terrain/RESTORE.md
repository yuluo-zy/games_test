# RESTORE — systems/terrain（工具 UI 与历史生产）

实现者: restorer-interaction
日期: 2026-10-15
范围: 新增 5 个缺失系统文件 + AppMode 裁决落地;既有 3 个 CPU 系统经 reviewer 核对 OK 不动。

## 原函数 → Rust 位置 → 证据

| 原函数 | VA | Rust 位置 | 证据 | 状态 |
|---|---|---|---|---|
| `deferred_terrain_history::history_record_terrain` | 0x140ac9770 | `deferred_terrain_history.rs::history_record_terrain` | decomp-0x140ac9770 | O 状态机;TerrainEdit 字段 U |
| `terrain_hover_idx::terrain_hover_idx` | 0x140aff720 | `terrain_hover_idx.rs::terrain_hover_idx` | ALGORITHMS + re-analyst | O 门;SSBO 布局待取证 |
| `ui_draw_terrain::ui_draw_terrain` | 0x140a9fdc0 | `ui_draw_terrain.rs::ui_draw_terrain` | decomp-0x140a9fdc0 | O 门 + new_stroke |
| `ui_edit_terrain::ui_edit_terrain` | 0x140aef1f0 | `ui_edit_terrain.rs::ui_edit_terrain` | decomp-0x140aef1f0 + re-analyst | O 门 + 脏掩码;widget 待取证 |
| `ui_terrain_focus_mode::ui_terrain_focus_mode` | 0x1409a4fb0 | `ui_terrain_focus_mode.rs::ui_terrain_focus_mode` | decomp(171KB)+ALGORITHMS | O 门;主体待取证 |

既有（reviewer 已核 OK，不动）: `terrain_stroke_changed`@0x140a03ba0、
`edit_terrain_stroke`@0x14094e000、`delayed_terrain_changed_msg`@0x140aabdd0。

## 新增实现与依赖复用边界

- 新资源（systems/terrain 内）: `PendingTerrainHistory`、`HoveredTerrainStrokeId`、
  `MouseTerrainHoverReadback`、`TerrainWaterSample`、`MouseActivityTimer`、
  `TerrainStrokeParamsCache`、`TerrainHoverInput`、`TerrainFocusModeState`。
- 复用: `TerrainEditingData`（tg_terrain）、`StrokeId`、`EditTerrainStrokeCmd`/
  `SetTerrainStrokePtsCmd`/`EvTerrainStrokeChanged`/`PlayTerrainSound`、`History`、
  `AppMode`（native_tag）、`CursorTerrainRaycast`、`CursorPositionSS`、`GameTime`。
- **跨模块最小改动**: `resources/history/mod.rs` 增加 `EditType::Terrain(Arc<TerrainEdit>)`
  （deferred_terrain_history 经 FUN_140a84ec0 入历史需要;tag 待取证,占位 14）。

## AppMode 裁决落地（coordinator 转发 re-analyst）

- terrain_hover_idx 门: `disc < 0xb && (0x421 >> disc) & 1`，bits {0,5,10} =
  All/Garden/Terrain（修正早前误记 {0,5,9}）。
- ui_draw_terrain / ui_edit_terrain 门: `disc == 10` = AppMode::Terrain。

## 仍缺失/待取证

- `TerrainEdit` 0x38 B 记录字段语义（{tag 对, pending 4 qword, 旧 state}）；EditType::Terrain
  native_tag 位序。
- `MouseTerrainRaycastSsbo`/`WaterMaskSsbo` 布局（Gap，宿主回读适配）。
- ui_edit_terrain 的 widget 注册（`add<String>` 0x1409231f0）、16 位脏掩码位图完整语义、
  130.0×2/0x100 常量、panic@0x142ae3acf（行 370）越界。
- ui_draw_terrain 的 UI 画布注册（名字 hash、UiState::add）。
- ui_terrain_focus_mode 状态机主体（PublicWalls 曲线采样/OutlineWallCmd，wall 模块）。
- `EditCommand` 字段序（原事件 `{falloff,height,position,id}` id@+0x1c vs tg_terrain）待取证。

## 编译状态

`cargo check -p country-core --lib` 0 error。测试二进制受其他 worker 在途错误阻塞
（ivy_grower EventWriter::new、save seek 导入）。

## 二轮 FIX 追加（reviewer-interaction REVIEW2，2026-10-15）

1. **ui_edit_terrain 点集死代码修复**: `DIRTY_MASK_POINTS` 从未置位 → `SetTerrainStrokePtsCmd`
   永不发出。已补点集指纹（len ^ 各点 f32 位型）到 `TerrainStrokeParamsCache.points_stamp`，
   点变化时置位并发出点集命令。

## 三.RE-ANALYSIS2 证据落地（re-analyst-interaction 终轮）

1. **ui_terrain_focus_mode 主体按行为级骨架落地**: 门 *(u32)*param_1==1；`raycast_simple`
   （0x1408ce180）命中 tag 0x11 分叉 + 3D 距离门（未命中 `_UNK_142925980`=f32::MAX）+
   0.5 阈值；Replay 哨兵 0x8000000000000002 写入；常量表（0.01 阈值、-10 速度、1.2/7.0、
   ±1 反射矩阵、CircularF32）登记。**卡点如实标注**: 167KB 内联体转移 switch 表 U。
   [`TerrainFocusRaycast`] 适配资源 + [`TerrainFocusTrigger`] 事件已挂入。

## 六.卡7 terrain 回读（coordinator next-batch，2026-10-15）

**read_terrain_data 在库内（render_loop/compute_passes/read_terrain_data.rs，已实现），
回读链验证**：
- `read_terrain_data`（0x14098c680）→ 光栅 "Visual terrain height" 预渲染（
  `VisualHeightRasterPlan`，terrain_shader/rocky_terrain 两 draw）→ "Read terrain height"/
  "Read terrain rock mask"/"Read water"/"Read garden" compute pass，各
  `write_storage_buffer<S::Height/VisualHeight/RockMask/WaterMask/WaterClutter/Garden>`
  → `ScheduledTerrainReadback::on_gpu_copy_issued`（cmpxchg 取走请求 + copy_issued 位）。
- 消费链：`delayed_terrain_changed_msg`（0x140aabdd0）经 `library.get::<TerrainHeightSsbo>()
  .mapped` + `TerrainHeightsData::copy_from_gpu` 拷贝双实例（authority + visual），
  两次拷贝间发 `OnTerrainChange`；与 read_terrain_data 的 GPU 拷贝衔接完整。
- **尺寸断言适配确认**：原版 BufferState 名字 = 完整类型路径（"…TerrainRockMaskSsbo"
  0x3f 字节 memcmp，见 render/nani/instance_updates.rs `TERRAIN_ROCK_MASK_SSBO_NAME`；
  TerrainHeightSsbo 全路径 0x3a、VisualTerrainHeightSsbo 0x40），crate 用
  `SsboLibrary::get::<T>` 的 `type_name::<T>()` 断言，行为等价（reviewer 已有此适配判定）。
- `TerrainHeightSsbo`/`VisualTerrainHeightSsbo` 经 `height_readback_ssbo!` 宏实现
  `SsboType`（NAME=stringify、BUFFER_LEN 0x20000、MAPPED_MEMORY_READBACK）✓。

**登记接线（阻塞项）**：生产 `ReadbackSsbos` 关联类型映射缺 4 个 mask SSBO 资源——
`TerrainRockMaskSsbo`（resources/terrain_rock_mask.rs GAP）、`WaterExcludeClutterMaskSsbo`
（water_exclude_clutter_mask.rs GAP）、`GardenMaskSsbo`（GAP）、`WaterMaskSsbo`（资源内
用 `pass_resource_access::SsboType`，缺 `asset_libraries::ssbo_library::SsboType` 实现）。
高度对（Height/VisualHeight）链已闭合；mask 四类待资源 GAP 闭合后补生产 impl。

## 七.卡B mask 资源闭合 + world_raster 登记（coordinator，2026-10-16）

**mask 四类 GAP 已建（卡7 ReadbackSsbos 阻塞项解除）**：
- `resources/terrain_rock_mask.rs`：`TerrainRockMaskSsbo`（SsboType，名字断言
  "…TerrainRockMaskSsbo" 0x3f 字节）+ `sample_at_world_xz`（0x1409ea800，世界→栅格采样）。
- `resources/garden_mask.rs`：`GardenMaskSsbo`；`resources/water_exclude_clutter_mask.rs`：
  `WaterExcludeClutterMaskSsbo`。
- `resources/water_mask.rs`：`WaterMaskSsbo` 补 `asset_libraries::ssbo_library::SsboType`
  + `StorageElement`（原只有 pass_resource_access::SsboType）。
- **生产 `TerrainReadbackSsbos`**（read_terrain_data.rs）：关联类型映射
  Height/VisualHeight/RockMask/WaterMask/WaterClutter/Garden 全六类。
- `resources/rocky_terrain.rs`：`RockyTerrainMesh::load`（0x1408bbf50，JSON 中间结构
  LoadedRockyTerrainVertex/Triangle → GPU 0x30B/0x20B → add_new_with_data）+ 顶点/三角形
  SSBO 类型（"Rocky Terrain Vertices/Triangles"）。

**world_raster**：`systems/world_raster.rs` 已含 record_raster_history + 上传消费（既有）；
开始/续画/提交/历史提取已接，本批复核 OK 未加深。

**登记待取证**：`RockyTerrainVertexSsbo`/`TriangleSsbo` 的 GPU 打包源布局（JSON 字段名）；
`EditType::Terrain` 真 tag（占位 14，待卡C 协调）；world_raster 完整状态机逐分支未重审。

## 留给 reviewer 的核对点

1. `terrain_hover_idx` 的 0x421 门（bits {0,5,10}）+ -1 哨兵/水位/距离门。
2. `deferred_terrain_history` 脏哨兵状态机（0x8000000000000002）+ 首次变化才 History::add。
3. `ui_edit_terrain` 脏掩码 → EditTerrainStrokeCmd/SetTerrainStrokePtsCmd 生成。
4. `ui_draw_terrain` mode==10 门 + new_stroke 生产。
