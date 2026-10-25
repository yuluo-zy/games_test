# systems/wall/shapes/rectangle/ui_create_rectangle.rs

> 文件作用：矩形绘制 UI：rectangle_dims_from_cursor_pos + get_rectangle_orientation 到 PreviewRectangle。

函数总计 10：业务 2 / 内联无独立符号 2 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ui_create_rectangle` | 0x14155d410 | 0 | ui_create_rectangle（dims_from_cursor_pos + orientation） | O | ui-input\decomp\decomp-0x14155d410.txt, wall\decomp\decomp-0x14155d410.txt |
| `country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_signifier` | 0x14155e460 | 347 | rectangle_signifier（构建 cursor_rectangle_ 预览 signifier：FUN_141559ce0 矩形预览 + signifier 流） | O |  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle::new`（行 62）— 构造
- `country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_dims_from_cursor_pos`（行 332）— UI 交互

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 5 项

## 实现方式（算法级）

**ui_create_rectangle（O，0x14155d410）**：12+ 参系统。绘制矩形预览并提交 CreateRectangleCmd。处理链：
1. **门控**：形状选择资源须为 `(6, 2)`（矩形），否则清 PreviewRectangle 返回（同 ui_create_circle 模式）。
2. **dims_from_cursor_pos**：`func_0x000140c99520 ×5`（矩形 dims 计算：光标首击锚点 → 矩形中心 = 锚点；`hw = |cursor−center|` 投影到旋转轴）。**旋转**由 `get_rectangle_orientation`（I）计算，常量 abs 掩码 0x7fffffff @0x14292e980/14292e984、1.0 @0x142925904。
3. **写 PreviewRectangle**：{+0x10 center, +0x18 dims, +0x20 rotation}（与 PreviewCircle 同布局族）。
4. **ease**：`func_0x000140c904a0 ×5`（维度平滑追赶）。
5. **signifier**：`FUN_14155e460 ×2`（**rectangle_signifier**，前缀 cursor_rectangle_）→ `FUN_14146c880 ×2`（signifier 流）；`FUN_141559af0 ×3`（矩形预览网格）。
6. **提交**：释放 → push CreateRectangleCmd 事件行（0x70B，字段见 create_rectangle）。

**rectangle_signifier（O，0x14155e460）**：见 rectangle/mod.rs.md 算法节（FUN_141559ce0 闭包 + 0x28B 记录 + cursor_rectangle_ 前缀）。
**常量**：1.0 @0x142925904、abs 掩码 @0x14292e980/984。
**跨模块**：`FUN_141286860`（矩形预览 mesh 相关）；`func_0x000140c99520` = utils/geometry 矩形 dims 计算。
