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

## 实现方式（算法级）

**ShapeSignifierLerp::lerp_radius（O，0x1409ef010）**：`(out, prev_shape, cur_shape, t)`，输出 12B 记录 `{tag u32@+0, val f32@+4, val2 f32@+8}`。处理链：

1. **判别**：`prev.tag bit0`（`*param_2 & 1`）。prev 为 circle（bit0=0）：
   - cur 为 rect（bit0=1）→ **panic**（`FUN_1428d9430(&UNK_142ac5368, 0x28, &UNK_142ac53e0)`，shape_signifier_lerp.rs:40）；
   - cur 为 circle：`ease = func_0x142923ec0(rate·t)`（rate = _UNK_14295c698）；`fVar9 = (1-ease)·(cur_r − prev_r)`；`fVar8 = min(t·2, |cur_r−prev_r|)`；`fVar10 = sign·min(...)` 混合（abs 掩码 _UNK_14292e980=0x7fffffff）；`result = prev_r + min_select(fVar9, fVar10)`；写 `{tag@+0=0, r@+4}`。
2. prev 为 rect（bit0=1）：
   - cur 为 circle → panic（同上）；
   - cur 为 rect：按轴逐维同式 lerp（`_UNK_14292e980/14292e984` 逐轴 abs），输出 `{tag@+0=1, x@+4, y@+8}`。
3. **语义**：拖拽中 signifier 半径/维度的指数式平滑追赶（ease = `func_0x142923ec0(rate·t)`，t∈[0,1]），并钳制增量 ≤ |目标−当前|。

**内联函数语义（I）**：`get_radius` 取 circle payload+0xc；`get_rectangle_dims` 取 rect payload +0x14/+0x18（hw/hh）。
**常量**：rate _UNK_14295c698（U 位型）、abs 掩码 0x7fffffff @0x14292e980、1.0 @0x142925904。
**边界**：跨形状 lerp → panic 行 40。
