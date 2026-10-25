# systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs

> 文件作用：矩形朝向编辑 UI：box_trigger_volume/skewed_box_trigger_vol 旋转到 EditRectangleCmd/FinishEditRectangleCmd；StairsPillarProposals 变体。

函数总计 6：业务 4 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate` | 0x140ad8000 | 49 | ui_edit_rectangle_orientation（触发体旋转） | O | wall\decomp\decomp-0x140ad8000.txt |
| `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::do_ui_edit_rectangle_rotate` | 0x140ad8be0 | 183 | ui_edit_rectangle_orientation（变体） | O | ui-input\decomp\decomp-0x140ad8be0.txt, wall\decomp\decomp-0x140ad8be0.txt |
| `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::skewed_box_trigger_vol` | 0x140ad9f10 | 352 | UI 交互 | I | ui-input\decomp\decomp-0x140ad9f10.txt, wall\decomp\decomp-0x140ad9f10.txt |
| `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume` | 0x140ada0e0 | 386 | UI 交互 | I | ui-input\decomp\decomp-0x140ada0e0.txt, wall\decomp\decomp-0x140ada0e0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure` 等 2 项

## 实现方式（算法级）

**ui_edit_rectangle_rotate（O，0x140ad8000）**：系统外壳。事件读取（FUN_140c170f0）+ `func_0x000140c96d40`（矩形旋转参数）+ `FUN_140c98ab0`（矩形方向）；状态串 = `...ui_edit_rectangle_orientation::ui_edit_rectangle_orientation::f`（UNK_142ae1223）。委托 **do_ui_edit_rectangle_rotate**（0x140ad8be0）。

**do_ui_edit_rectangle_rotate（O，0x140ad8be0）**：**触发体旋转 → 新朝向（核心）**。处理链：
1. 取两方向向量（当前朝向 dir_cur 与光标方向 dir_cursor）；
2. **cosθ = (dir_cur·dir_cursor)/√(|cur|²·|cursor|²)**；
3. **acos 快速近似**：`acos(|c|) ≈ ((c0·x + c1)·x + … + c7)·√(1 − |c|)`（**8 阶多项式**，系数 0x142ae0d44..142ae0d60）；`|c| = cosθ & 0x7fffffff`（abs）；
4. **角度符号**：cross = dir_cur×dir_cursor（`fVar56·fVar63 − fVar57·fVar62`）定象限；`result = π − acos`（π = 0x40490fdb@0x1429edba4）按符号取；
5. NaN 防护（0x7fc00000 = NaN 位型 @0x142a8d5c4）；量化为 0.25 步长常量 @0x142925950（I：朝向吸附/量化）；
6. 发 **EditRectangleCmd/FinishEditRectangleCmd**（变体标记）+ 旋转后的矩形参数；触发体 = `box_trigger_volume`（0x140ada0e0）/`skewed_box_trigger_vol`（0x140ad9f10）。

**box_trigger_volume（I，0x140ada0e0）/ skewed_box_trigger_vol（I，0x140ad9f10）**：构造触发体（旋转 box），供拖拽命中测试（ui-input 域另有 decomp）。
**常量表**：acos 多项式 8 系数 0x142ae0d44..0x142ae0d60（值 = 0x3bda90c5、0x3cd90a90、0x3c8bfc66、0x3dfd10f8、0xbd4d8392、0x3db63a9e、0xbe5bbfca、0x3fc90fda —— 经典 acos 多项式系数）、π 3.1415927 @0x1429edba4、NaN @0x142a8d5c4、abs 掩码 0x7fffffff @0x14292e980、量化 0.25 @0x142925950、1.0 @0x1429258e0。
**跨模块**：`func_0x142923fd0/142923e80` = libm sin/cos 包装（朝向向量）；`FUN_140c99600` = 矩形角。
