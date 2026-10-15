# systems/wall/shapes/circle/signifiers/edit_line.rs

> 文件作用：圆形编辑半径线 signifier：Query 驱动半径线位姿。

函数总计 4：业务 2 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::circle::signifiers::edit_line::impl$0::register_required_components` | 0x140a66c90 | 25 | 构造/注册/生成 | I | wall\decomp\decomp-0x140a66c90.txt |
| `country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line` | 0x140a64a40 | 40 | circle edit_line signifier | O | wall\decomp\decomp-0x140a64a40.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项

## 实现方式（算法级）

**signifier_edit_circle_line（O，0x140a64a40）**：4 参系统。绘制圆形编辑半径线（圆心→圆上一点）。处理链：
1. `FUN_140c975a0 ×2`（**Circle2d::get_pos_at_u**：圆心/端点）+ `func_0x000140c97540 ×2`（圆上点采样）。
2. 端点计算用常量：**0.23**（0x3e6b851f@0x142a89600）与 **0.75**（0x3f400000@0x1429edb90）、**90.0**（0x42b40000@0x1429edb90+4）——半径线端点沿圆的角度/比例偏移。
3. `FUN_140d8c910 ×2`（曲线/线绘制）→ 生成 signifier 流记录；曲线点 Vec（8B 元素 cap-first）拷贝/释放经 `func_0x000140613c10/140613c20`、`func_0x000142923c00`（memmove）。

**register_required_components（I，0x140a66c90）**：组件注册表切片返回。
**常量**：0.23 @0x142a89600、0.75/90.0 @0x1429edb90。
**跨模块**：`FUN_140c975a0/c97540` = utils/geometry 圆几何（central 关联）。
