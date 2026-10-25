# systems/wall/shapes/rectangle/signifier_edit_dims_line.rs

> 文件作用：矩形尺寸编辑线 signifier（rectangle_edit_dims_line_ 前缀）。

函数总计 4：业务 2 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::impl$0::register_required_components` | 0x1408e0e80 | 25 | 构造/注册/生成 | I | wall\decomp\decomp-0x1408e0e80.txt |
| `country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims` | 0x1408dda10 | 34 | signifier_edit_dims_line | O | wall\decomp\decomp-0x1408dda10.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项

## 实现方式（算法级）

**signifier_edit_dims（O，0x1408dda10）**：矩形尺寸编辑线 signifier（`rectangle_edit_dims_line_` 前缀族）。处理链：
1. 读矩形 corner/参数（`FUN_140c99600` 域）+ 曲线绘制（`FUN_140d8c910 ×2`）。
2. 曲线点 Vec（8B 元素 cap-first）分配/拷贝/释放（`func_0x000140613c10/140613c20`、`func_0x000142923c00` memmove）；`.unwrap()` panic（`FUN_1428d8fe3` + UNK_142c38b78）。
3. 用常量 **0.01**（@0x142960674）与 **1.0**（@0x142925904）——编辑线端点/状态阈值。
4. 模块路径 = `crates/country-core/src/systems/wall/shapes/rectangle/signifier_edit_dims_line.rs`（UNK_142aa4d38/142aa4d50 胖指针）；断言族 = CurveU（`!u.is_nan()` / `(0.0..=1.0).contains(&t)` @0x142aa53d0，curve_u.rs）。

**register_required_components（I，0x1408e0e80）**：组件注册表切片返回。
