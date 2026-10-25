# systems/ui_raise_shape.rs

> 文件作用：抬升形状：widget 名 'outline_wall_bottom_'/'arrow_raise_shape_'/'raise_wall_elevation_'；impl_raise_shape 用 ProportionalScreenSpaceCoord::project_onto_ray 算抬升量 + History::add 记录。

函数总计 8：业务 2 / 内联无独立符号 0 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_raise_shape::ui_raise_shape` | 0x140952e70 | 37 | 系统:周期性处理（ui_raise_shape） | O：8.7KB 主系统（wp20 + 本域复核） | ui-input\decomp\decomp-0x140952e70.txt |
| `country_core::systems::ui_raise_shape::impl_raise_shape` | 0x140955650 | 238 | 抬升实现 | O：impl_raise_shape（project_onto_ray + 高度 delta + History::add；wp20 既有） | wp20/analysis/0x140955650_impl_raise_shape.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 6 项

## 实现方式（算法级）

- `ui_raise_shape` @0x140952e70（O，8.7KB，wp20 + 本域）：抬升形状 gizmo。
  - widget 名 'outline_wall_bottom_' / 'arrow_raise_shape_' / 'raise_wall_elevation_'（@0x142ab4548/45f8/46c0，串区 UNK_142ab44xx/45xx/46xx）。
  - `impl_raise_shape` @0x140955650（O）：`ProportionalScreenSpaceCoord::project_onto_ray` 算抬升量 + 高度 delta + `History::add` 记录（wp20 既有）。
  - panic 分支（O）：UNK_142ada1（public_walls.rs 断言）与 `FUN_1428d9430`×4（越界/断言 panic）——非法墙数据时先断言后恢复。
