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
