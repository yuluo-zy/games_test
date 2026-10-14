# systems/save/snapshot.rs

> 文件作用：快照捕获/加载：capture_game_snapshot 巨型系统（184KB）串化全 Query 为 JSON 文档（节序 Version→Walls→Roofs→Decorators→WallColors→GateColors→Terrain(flate2 deflate)→Trees→Raster）；capture_game_snapshot_save_data 构建 0x400B 结构并返回 Arc；validate_and_load_closest_snapshot_data 解析快照 JSON（parsing 全串+Version mismatch）；init_game_state_from_snapshot 应用数据。

函数总计 21：业务 6 / 内联无独立符号 3 / 派生胶水 12

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::systems::save::snapshot::capture_game_snapshot_save_data`` | 0x1417a0430 | 86 | 快照 | O：capture_game_snapshot_save_data = 从世界数据构建快照结构（~0x400B），返回 Arc（alloc 0x18 {1,1}+LOCK inc） | session-save\decomp\decomp-0x1417a0430.txt  |
| ``country_core::systems::save::snapshot::capture_game_snapshot`` | 0x1417a1d00 | 162 | 系统 | O：capture_game_snapshot 系统体 = 巨型串化（StairsSnapshot{edges,checksum}/TerrainEditingData{strokes,next_stroke_id,next_stroke_index}/DecoratorStorageSnapshot{states} 等） | session-save\decomp\decomp-0x1417a1d00.txt  |
| ``country_core::systems::save::snapshot::_::impl$0::init_state`` | 0x1417ab290 | 250 | 构造/注册/生成 | O：init_state（v1 名 get_param 同址）= Bevy SystemParam 胶水（20+ param 函数链，非业务） | session-save\decomp\decomp-0x1417ab290.txt  |
| ``country_core::systems::save::snapshot::try_chunk`` | 0x1417a6d70 | 286 | 待推断 | O：try_chunk = Result<Option<chunk>>（分块/文件读取） | round3\decomp-0x1417a6d70.txt  |
| ``country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data`` | 0x1417a6e50 | 296 | 谓词/校验 | O：validate_and_load_closest_snapshot_data（v1 名 validate_param 错位）= 快照 JSON 解析器：parsing Version/Walls/Roofs/Decorators/WallColors/GateColors/Terrain(decompressing terrain data)/Trees/Raster；Version mismatch; expected , got ；Ignoring unknown raster data in snapshot | session-save\decomp\decomp-0x1417a6e50.txt  |
| ``country_core::systems::save::snapshot::init_game_state_from_snapshot`` | 0x1417a9c50 | 470 | 构造/注册/生成 | O：init_game_state_from_snapshot（v1 名 try_load_snapshot_data）= 应用解析数据到世界（被 load_game 调用） | session-save\decomp\decomp-0x1417a9c50.txt  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::save::snapshot::_::impl$0::get_param`（行 250）— 查询/计算
- `country_core::systems::save::snapshot::_::impl$0::validate_param`（行 250）— 谓词/校验
- `country_core::systems::save::snapshot::try_load_snapshot_data`（行 327）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 12 项
