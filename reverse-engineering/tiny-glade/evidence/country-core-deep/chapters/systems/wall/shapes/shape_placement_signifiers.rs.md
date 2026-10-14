# systems/wall/shapes/shape_placement_signifiers.rs

> 文件作用：形状放置 signifier：吸附时 preview_snapping_line（ShapeParameters::expand + Curve::get_tangent_at_u/try_slice）+ 形状 outline；断言 !u.is_nan()/!u.is_infinite()。

函数总计 6：业务 2 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line` | 0x1408dfc60 | 0 | shape_placement_signifiers（!u.is_nan()/!u.is_infinite() 断言） | O | wall\decomp\decomp-0x1408dfc60.txt |
| `country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers` | 0x1408dec70 | 0 | shape_placement_signifiers（吸附线 + outline） | O | wall\decomp\decomp-0x1408dec70.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
