# systems/wall/eraser/wall_eraser.rs

> 文件作用：徒手墙擦除系统：拖笔选段到 WallEraserArchivist::affected_walls/has_anything_been_affected 到 InnerWalls::get_unwrapped/update_wall_state(_curve/_enclosure) + new_wall；determine_freehand_height_ty、wall_height_based_on_length、adjust_wall_height_if_erased_flat_roof_w_no_rims（PublicWalls::roof_entity ×4）；ChildWall::new 子墙。

函数总计 10：业务 1 / 内联无独立符号 3 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::eraser::wall_eraser::wall_eraser` | 0x140accab0 | 0 | wall_eraser（拖笔选段 + archivist + ChildWall::new + 平屋顶高度补偿） | O | wall\decomp\decomp-0x140accab0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims`（行 549）— 待推断
- `country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty`（行 571）— 待推断
- `country_core::systems::wall::eraser::wall_eraser::wall_height_based_on_length`（行 599）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 6 项
