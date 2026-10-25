# systems/wall/shapes/rectangle/ui_modes/edit_dims.rs

> 文件作用：矩形尺寸编辑 widget/signifier/trigger_volume（UiRectangleDimsEdit，widget 层）。

函数总计 7：业务 2 / 内联无独立符号 3 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::signifier` | 0x14155eba0 | 63 | ui_modes edit_dims widget | O | ui-input\decomp\decomp-0x14155eba0.txt, wall\decomp\decomp-0x14155eba0.txt |
| `country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::widget` | 0x14155fc70 | 168 | UI 交互 | I | ui-input\decomp\decomp-0x14155fc70.txt, wall\decomp\decomp-0x14155fc70.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::impl$0::to_widget_name_hash`（行 45）— 访问器/转换
- `country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::new`（行 52）— 构造
- `country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::trigger_volume`（行 194）— UI 交互

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项

## 实现方式（算法级）

**UiRectangleDimsEdit::signifier（O，0x14155eba0）**：维度编辑 signifier/widget。处理链：
1. `FUN_140c99600 ×4`（**矩形 get_pos_at_u/角**）——四边端点采样；
2. 曲线生成（`func_0x000140c9cde0`）→ signifier 流（`FUN_140d79f90/FUN_140d4ba00/FUN_14146c880`）；
3. 常量 **1.0 @0x1429258e0/1429258e4**（dim 归一）、`FUN_1415610f0 ×2`（矩形预览构造）；
4. 名字串族 UNK_142c6e400/510（"dims_edit_" 前缀）。

**UiRectangleDimsEdit::widget（I，0x14155fc70）**：widget 层 UI（`to_widget_name_hash` = 名字哈希；`trigger_volume` = 构造触发体）。
**内联**：`new`（行 52）构造；`to_widget_name_hash`（行 45）名字哈希；`trigger_volume`（行 194）触发体。
**边界**：`.unwrap()`/alloc panic（UNK_142c6e510 路径）。
