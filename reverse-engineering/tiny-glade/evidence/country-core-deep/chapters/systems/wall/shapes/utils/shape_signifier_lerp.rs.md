# systems/wall/shapes/utils/shape_signifier_lerp.rs

> 文件作用：形状 signifier 插值：ShapeVal{interpolate,get_radius,get_rectangle_dims}、ShapeSignifierLerp::lerp_radius——circle/rectangle 值间插值（glam 复用）。

函数总计 4：业务 1 / 内联无独立符号 3 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeVal::interpolate` | 0x1409ef010 | 28 | ShapeSignifierLerp 族（circle/rectangle 插值） | O | wall\decomp\decomp-0x1409ef010.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeSignifierLerp::lerp_radius`（行 15）— 吸附/数值
- `country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeVal::get_radius`（行 80）— 查询/计算
- `country_core::systems::wall::shapes::utils::shape_signifier_lerp::ShapeVal::get_rectangle_dims`（行 87）— 查询/计算
