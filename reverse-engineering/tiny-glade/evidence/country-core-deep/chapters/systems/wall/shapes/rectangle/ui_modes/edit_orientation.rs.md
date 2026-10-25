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

## 实现方式（算法级）

**UiRectangleOrientationEdit::widget（O，0x14083d3f0）**：朝向编辑 widget。处理链：
1. `FUN_140c99600 ×2`（矩形角）→ 旋转 box 触发体四角；
2. **朝向角度**：常量 **1.5**（0x3fc00000@0x1429258d0）、**0.3**（0x3e99999a@0x1429258d8）、**1.0**（0x142925870/1429258e0）、**0.25**（0x3e800000@0x142925950）、abs 掩码（0x1429258f0）——触发体旋转灵敏度/量化阈值（I：0.25 = 90°/4 步进或灵敏度）；
3. `FUN_141469e00`（2D→3D 投影）+ `FUN_1413047b0`（曲线/网格）——把朝向可视化到 3D；
4. 名字串族 UNK_142a93920/38、UNK_142c63658、UNK_142a93358（"orientation_edit_" 前缀）。

**UiRectangleOrientationEdit::signifier（I，0x14083da10）**：朝向 signifier 生成（ui-input 域另有 decomp）。
**内联**：`new`（行 46）构造；`to_widget_name_hash`（行 39）名字哈希。
**常量**：1.5 @0x1429258d0、0.3 @0x1429258d8、0.25 @0x142925950。
