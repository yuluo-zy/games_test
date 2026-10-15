# systems/world_raster.rs

> 文件作用：**世界栅格系统：record_raster_history（栅格历史记录）、upload_world_raster_buffers_to_staging（脏流门 + 6 光栅发布到 staging SSBO）、draw_raster<T> 泛型状态机（10 实例：Path/Cobblestone1-3/Garden/Water，start_new→add_point→resample_and_smooth→rasterize_stroke→copy_to_staging 链）。**

函数总计 19：业务 18 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$3,country_core::systems::paths::draw_path::draw_path::closure_env$4,country_core::systems::paths::draw_path::draw_path::closure_env$5>` | 0x140abba80 | 0 | draw_raster<Cobblestone1RasterStagingSsbo> 实例：同泛型状态机（erase_other_path_variants 闭包变体） | O | terrain-water\decomp\decomp-0x140ab2340.txt
| `country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$0<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$1<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> >` | 0x140ab2340 | 0 | draw_raster<Cobblestone1RasterStagingSsbo> 实例（erase_other_path_variants 闭包变体：擦除其它路径变体） | O | terrain-water\decomp\decomp-0x140ab2340.txt
| `country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$6,country_core::systems::paths::draw_path::draw_path::closure_env$7,country_core::systems::paths::draw_path::draw_path::closure_env$8>` | 0x140ab0f10 | 0 | draw_raster<Cobblestone2RasterStagingSsbo> 实例（draw_path closure_env$6-8） | O | terrain-water\decomp\decomp-0x140ab0f10.txt
| `country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$3<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$4<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$5<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster> >` | 0x140ab6de0 | 0 | draw_raster<Cobblestone2RasterStagingSsbo> 实例（erase_other_path_variants 闭包变体） | O | terrain-water\decomp\decomp-0x140ab6de0.txt
| `country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$9,country_core::systems::paths::draw_path::draw_path::closure_env$10,country_core::systems::paths::draw_path::draw_path::closure_env$11>` | 0x140ab9340 | 0 | draw_raster<Cobblestone3RasterStagingSsbo> 实例（draw_path closure_env$9-11） | O | terrain-water\decomp\decomp-0x140ab9340.txt
| `country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> >` | 0x140ab8090 | 0 | draw_raster<Cobblestone3RasterStagingSsbo> 实例（erase_other_path_variants 闭包变体） | O | terrain-water\decomp\decomp-0x140ab8090.txt
| `country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2>` | 0x140ab59e0 | 0 | draw_raster<GardenRasterStagingSsbo> 实例（draw_garden 闭包） | O | terrain-water\decomp\decomp-0x140ab59e0.txt
| `country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster> >` | 0x140ab35f0 | 0 | draw_raster<PathRasterStagingSsbo> 实例（draw_path 闭包） | O | terrain-water\decomp\decomp-0x140ab4750.txt
| `country_core::systems::world_raster::_::impl$0::apply` | 0x14003bb80 | 65 | SystemParam apply（ICF=裸 ret） | O | terrain-water/DIGEST.md
| `country_core::systems::world_raster::_::impl$0::get_param` | 0x141507ce0 | 65 | RasterDrawResources/RasterHistoryRecordResources SystemParam get_param（另一实例 0x1401507ce0） | I | terrain-water\decomp\decomp-0x141507ce0.txt
| `country_core::systems::world_raster::_::impl$0::init_state` | 0x140958040 | 65 | SystemParam init_state（6 栅格资源注册，两实例 0x140958040/0x1409580e0） | O | terrain-water\decomp\decomp-0x140958040.txt
| `country_core::systems::world_raster::_::impl$0::new_archetype` | 0x14003bb80 | 65 | SystemParam new_archetype（ICF=裸 ret） | O | terrain-water/DIGEST.md
| `country_core::systems::world_raster::_::impl$0::queue` | 0x14003bb80 | 65 | SystemParam queue（ICF=裸 ret） | O | terrain-water/DIGEST.md
| `country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2>` | 0x140aba770 | 75 | draw_raster<PathRasterStagingSsbo> 实例（draw_path closure_env$0-2） | O | terrain-water\decomp\decomp-0x140aba770.txt
| `country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$12,country_core::systems::paths::draw_path::draw_path::closure_env$13,country_core::systems::paths::draw_path::draw_path::closure_env$14>` | 0x140ab4750 | 75 | draw_raster<PathRasterStagingSsbo> 实例（draw_path closure_env$12-14） | O | terrain-water\decomp\decomp-0x140ab4750.txt
| `country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3>` | 0x142260ae0 | 75 | draw_raster<WaterRasterStagingSsbo> 实例：状态 0 start_new；状态 1 last 距>0.0005→add_point+resample_and_smooth+DirtyTiles::clear+rasterize_world_space_stroke+extend×2+交换双位图+copy_composite_to_scratch+rasterize_stroke+SSBO map+copy_to_staging；其余 commit_scratch_to_composite 发布；按 +0x64 发 WaterDraw/WaterErase 事件 | O | terrain-water\decomp\decomp-0x142260ae0.txt
| `country_core::systems::world_raster::record_raster_history` | 0x140957990 | 222 | 按 variant·0x470 槽位取 12B 元素笔画 Vec → Arc 克隆 → History::add → 重置 Vec(cap 4) | O | terrain-water\decomp\decomp-0x140957990.txt
| `country_core::systems::world_raster::upload_world_raster_buffers_to_staging` | 0x140957bf0 | 241 | 脏输入流非空 → AllWorldRasterWrite::upload_to_staging（6 光栅 composite→staging SSBO 发布） | O | terrain-water\decomp\decomp-0x140957bf0.txt

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::world_raster::_::impl$0::validate_param`（行 65）— 谓词/校验

## 实现方式（算法级）

### draw_raster<T> 状态机（10 实例；水 0x142260ae0 精读）

```
state==0 → start_new(stroke, cursor, mode, brush)
state==1：
  last()（空→panic）→ dist(last,point) ≤ 0.0005（0x1429cd060）早退
  add_point → resample_and_smooth
  DirtyTiles::clear(stroke.bitmap@+0x270)
  rasterize_world_space_stroke(bitmap, iter_points_xz(smoothed@+0x48), first, brush@+0x60)
  DirtyTiles::extend(worldraster.bitmap@+0x98, stroke.bitmap)（×2）
  swap(stroke.bitmap 0x70.. 与 0x270.. 双 512B 半)
  copy_composite_to_scratch(worldraster)
  rasterize_stroke(worldraster, iter_points_xz(smoothed), ..., mode, brush)
  map StagingSsbo → copy_to_staging(selector=0)
其余状态 → commit_scratch_to_composite 发布
按 +0x64 模式发 Draw/Erase 事件（WaterDraw/WaterErase 等）
```

### 输入源 / 槽位

- 点：CursorTerrainRaycast::position（func_0x000140a12650）；平滑后曲线列 Vec3@+0x48 是 rasterize_stroke 唯一输入（iter_points_xz 0x140a97860）。
- ActiveRasterStroke 槽位：index_mut 0x868cf0，imul 0x470；记录_raster_history 同槽位（record_raster_history 0x957990 = 12B 元素 Vec 克隆 → History::add → 重置 cap 4）。
- upload_world_raster_buffers_to_staging（0x957bf0）：脏输入流（EventReader 双缓冲差）非空 → upload_to_staging（6 光栅 composite→staging SSBO）；log/panic module path country_core::systems::world_raster。
