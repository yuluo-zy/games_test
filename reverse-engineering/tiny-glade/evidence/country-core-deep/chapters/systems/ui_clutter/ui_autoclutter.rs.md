# systems/ui_clutter/ui_autoclutter.rs

> 文件作用：右键把 auto-clutter 转 deco-clutter（InstaCreateDeco/InstaCreateStairPoint/StartPlaceDecoClutterInteraction/EvRightClickedAutoClutter；wp20 合同）。

函数总计 7：业务 1 / 内联无独立符号 2 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter` | 0x14099ca60 | 89 | 系统:输入处理（ui_convert_autoclutter_to_decoclutter） | O：8.1KB（wp20 + 本域复核） | ui-input\decomp\decomp-0x14099ca60.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::ui_clutter::ui_autoclutter::get_clutter_rot_in_roof_space`（行 403）— 查询/计算
- `country_core::systems::ui_clutter::ui_autoclutter::maintain_autoclutter_parent_hashmap`（行 410）— 系统:周期性处理

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
