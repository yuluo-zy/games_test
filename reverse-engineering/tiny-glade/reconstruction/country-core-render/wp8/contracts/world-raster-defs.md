# WP8 契约 3/4 — `resources::world_raster_defs`（共享 CPU 世界光栅定义）

版本 **B**。数据源：skeleton bundle `resources/world_raster.rs.{pdb,structures,enriched,dependencies}`
（注意：原源码文件为 `world_raster.rs`，`world_raster_defs` 是其内子模块；skeleton 无独立
`world_raster_defs.rs` bundle——已按 `ls` 确认）。WP6 已交付 `reconstruction/country-core/src/resources/
{world_raster,world_raster_defs}.rs`（拆分布局），本契约复验并扩展。反编译：`upload_raster_staging_to_texture<PathRaster>`@0x1403d8bd0。

标注：**O**=PDB/反编译直接支持；**I**=调用链/组合；**U**=未恢复（typed gap）。

## 1. 命名实体（world_raster.rs bundle，PDB）

| 实体 | 证据 |
|---|---|
| `WorldRaster<N>` | 泛型世界光栅（O；实例 `WorldRaster<512>`） |
| `RasterBufferRef<N>` | 光栅缓冲引用（O；`bilinear_sample_at_world_xz<RasterBufferRef<512>>`，512×512） |
| `DirtyTilePublisher` | 脏瓦片发布枚举（O 名；`unique`/`shared`/`secondary` 方法；`drop_in_place<Option<DirtyTilePublisher>>`） |
| `StrokeBlendMode` | 笔触混合模式枚举（O 名；serde serialize/fmt） |
| `UploadWorldRasterBuffersToStagingCmd` | 上传命令（O 名） |
| `WorldRaster::ws_to_px / prepare_tile / replace_composite / clear_composite / as_float_grid / last_staged` | CPU 侧方法（O 名） |
| `DirtyTilePublisher::unique/shared/secondary` | 发布器构造（O 名，行 23/29/47？——world_raster 内） |
| `AllWorldRasterRead::read` / `AllWorldRasterWrite` | 全光栅 SystemParam（O：`FunctionSystem<void(*)(AllWorldRasterWrite)>` 等） |

world_raster_defs 子模块（O 名，均在 world_raster.rs bundle 中）：

| 实体 | 证据 |
|---|---|
| `Cobblestone1Raster::new` / `Cobblestone2Raster::new` / `Cobblestone3Raster::new` | 命名光栅构造（O） |
| `PathRaster::new` / `GardenRaster::new`（+`WaterRaster`） | 命名光栅构造（O） |
| `PathRasterStagingSsbo` / `WaterRasterStagingSsbo` / `GardenRasterStagingSsbo` / `Cobblestone*RasterStagingSsbo` | 暂存 SSBO 标记（O：upload/replay 泛型参数） |
| `_::impl$0::{init_state, get_param, validate_param}` | SystemParam 实现（O：`AllWorldRasterRead`/`AllWorldRasterWrite`） |

## 2. 每个命名光栅的行为（O）

- 均为 `Res<…Raster>` 资源，Deref/DerefMut 到 `RasterBufferRef<512>`（WP6 已恢复）。
- `WaterRaster`：`Res<WaterRaster>`（O：`nani_update_preplaced_flower` ECS 签名）；水位 ≤0.01 判定（WP6）。
- 每帧 W1 上传：`upload_raster_staging_to_texture<T>`（O，@0x1403d8bd0 起六个实例）：
  - 读资源 + 全局调试开关（`TweakerView+0x29a`，O）。
  - 按格式枚举选 shader：`"r16f/upload_cpu_raster.cs.hlsl"`（fmt 0x4c=76）或
    `"rgba16fr16f/upload_cpu_raster.cs.hlsl"`（fmt 0x61=97）（O，panic `not yet implemented` 分支）。
  - `ComputePass::new<String>` + `read_storage_buffer<T::StagingSsbo>` + `bind_texture_to_compute_pass<TexturePreset>`(preset 2) + `write_uniform<u32>` + `dispatch_2d`（O）。
- 历史回放：`systems::history::replay_raster::replay_raster<StagingSsbo, Raster>`（O：`WorldRaster<512>` 的
  `drop_in_place`/`replay_raster<Cobblestone2RasterStagingSsbo,…>` 等过程存在）。

## 3. 与 terrain/buildings 的共享（I）

- `terrain_editing`/`paths`/`water`/`garden` 都向各自光栅暂存 SSBO 写笔触，W1 统一上传（I：frame-wiring W1）。
- `rasterize_terrain_stroke` 只改地形高度（TerrainHeightsData），不落世界光栅（I）。
- `AllWorldRasterRead`/`AllWorldRasterWrite` 供 `draw_terrain_stroke`/`upload_world_raster` 等系统批量访问（O 名）。

## 4. O/I/U 摘要

- **O**：命名光栅实体 + `new`；`RasterBufferRef<512>`/`WorldRaster<512>`；上传 pass 的 shader 选择与声明；
  staging SSBO 家族；`AllWorldRasterRead`。
- **I**：上传 pass 的 dispatch 规模（dispatch_2d）；`WorldRaster<N>` 内部含 float grid + dirty 发布器。
- **U**：`WorldRaster<N>` 字段布局（含 `as_float_grid` 目标）、`DirtyTilePublisher`/`StrokeBlendMode` 判别式、
  `prepare_tile`/`replace_composite` 内部算法、staging SSBO 记录格式。→ typed gap。

WP6 交付的 `world_raster.rs`/`world_raster_defs.rs` 与本契约一致；本 WP8 在 `world_raster_defs.rs` 中补充
`new` 构造、`AllWorldRasterRead` 骨架与 staging SSBO 标记（见 `src/resources/world_raster_defs.rs`）。
