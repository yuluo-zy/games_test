# systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs

> 文件作用：矩形朝向编辑 widget/signifier/trigger_volume（UiRectangleOrientationEdit，widget 层）。

函数总计 6：业务 2 / 内联无独立符号 2 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget` | 0x14083d3f0 | 0 | ui_modes edit_orientation widget | O | ui-input\decomp\decomp-0x14083d3f0.txt, wall\decomp\decomp-0x14083d3f0.txt |
| `country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier` | 0x14083da10 | 153 | UI 交互 | I | ui-input\decomp\decomp-0x14083da10.txt, wall\decomp\decomp-0x14083da10.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$0::to_widget_name_hash`（行 39）— 访问器/转换
- `country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::new`（行 46）— 构造

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项
