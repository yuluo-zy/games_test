# systems/wall/eraser/archivist.rs

> 文件作用：擦除归档器：WallEraserArchivist 累积 original/split/rewire 事件（cache_if_original_wall/shrink_wall/add_split_event/add_fully_erased_event），finish 经 CompoundEditBuilder::build 编译 WallHistoryCompoundEdit；断言 original_wall_state.curve.is_valid()/!before_wall.curve.is_empty()；ChildWalls/WallHistoryEdits。

函数总计 22：业务 7 / 内联无独立符号 9 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::new` | 0x14092e6e0 | 110 | WallEraserArchivist::new | O | wall\decomp\decomp-0x14092e6e0.txt |
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::record_rewiring_event` | 0x14092e8e0 | 118 | WallEraserArchivist::record_rewiring_event | O | wall\decomp\decomp-0x14092e8e0.txt |
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall` | 0x14092ea50 | 133 | WallEraserArchivist::cache_if_original_wall | O | wall\decomp\decomp-0x14092ea50.txt |
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::shrink_wall` | 0x14092eff0 | 167 | WallEraserArchivist::shrink_wall | O | wall\decomp\decomp-0x14092eff0.txt |
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_split_event` | 0x14092f280 | 187 | 累加/追加 | I | wall\decomp\decomp-0x14092f280.txt |
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_fully_erased_event` | 0x14092f630 | 224 | 累加/追加 | I | wall\decomp\decomp-0x14092f630.txt |
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish` | 0x14092f990 | 270 | WallEraserArchivist::finish（CompoundEditBuilder::build；!before_wall.curve.is_empty() 断言） | O | wall\decomp\decomp-0x14092f990.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::eraser::archivist::WallEraserArchivist::affected_walls`（行 39）— 待推断
- `country_core::systems::wall::eraser::archivist::WallEraserArchivist::has_anything_been_affected`（行 43）— 谓词/校验
- `country_core::systems::wall::eraser::archivist::ChildWalls::remove`（行 73）— 删除/清理
- `country_core::systems::wall::eraser::archivist::ChildWalls::extend`（行 82）— 累加/追加
- `country_core::systems::wall::eraser::archivist::ChildWalls::is_empty`（行 98）— 谓词/校验
- `country_core::systems::wall::eraser::archivist::WallEraserArchivist::record_erase_original_wall`（行 259）— 渲染/GPU
- `country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish_shrink_or_split_event`（行 353）— 生命周期控制
- `country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish_erase_event`（行 540）— 生命周期控制
- `country_core::systems::wall::eraser::archivist::WallHistoryEdits::compile_history_edit`（行 633）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default`、`eq` 等 6 项
