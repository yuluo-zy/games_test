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

## 实现方式（算法级）

- `ui_convert_autoclutter_to_decoclutter` @0x14099ca60（O，8.1KB，wp20 + 本域）：右键把 auto-clutter 转 deco-clutter。
  - 事件族 InstaCreateDeco / InstaCreateStairPoint / StartPlaceDecoClutterInteraction / EvRightClickedAutoClutter（O：wp20 合同）。
  - 变换计算（O）：`FUN_1408b83d0`×4 + 常量区 UNK_1429cd1/1429258（旋转/变换），生成放置数据并推 0x48B 事件；panic：`FUN_1428d9940`（path/越界，O）。
  - 内联：`get_clutter_rot_in_roof_space`（行 403，I：屋顶空间旋转，无独立符号）；`maintain_autoclutter_parent_hashmap`（行 410，I：维护 clutter→parent 映射）。
