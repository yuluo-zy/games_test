# systems/wall/shapes/rectangle/mod.rs

> 文件作用：rectangle 形状子模块根。

函数总计 3：业务 0 / 内联无独立符号 1 / 派生胶水 2

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::rectangle::get_rectangle_orientation`（行 83）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure` 等 2 项

## 实现方式（算法级）

**get_rectangle_orientation（I，行 83）**：由矩形 dims（{hw,hh}）与朝向（cos/sin）构造矩形旋转参数；被 ui_create_rectangle 的 `rectangle_dims_from_cursor_pos` 内联使用（U 独立反编译）。

**rectangle_signifier（O，0x14155e460，本模块从属**，源码登记于 ui_create_rectangle.rs 行 347）**：`FUN_141559ce0`（矩形预览生成器，闭包）+ `FUN_140d79f90`（spawn）→ 分配 0x28B 记录 `{1, 1, center.x, center.y, dims.x, dims.y, flag}`；signifier 前缀 = `cursor_rectangle_`（UNK_142c6e320 胖指针验证）。与 circle_signifier（0x14165cd40，前缀 cursor_circle_）严格同构，仅预览闭包（141559ce0 vs 14165bd40）与参数（dims vs radius）不同。

**模块常量**：矩形 ShapeParameters 布局 = `{tag u32@+0, cos f32@+4, sin f32@+8, cx f32@+0xc, cy f32@+0x10, hw f32@+0x14, hh f32@+0x18}`（28B 记录），见 valid_enclosure/mod.rs。
