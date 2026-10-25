# systems/ui_clutter/decoclutter_show_pins.rs

> 文件作用：显示/隐藏已固定 deco-clutter 图钉（DecoClutterLock、ShowPinnedDecoClutter；wp20 合同）。

函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_clutter::decoclutter_show_pins::impl$1::clone_behavior` | 0x140002370 | 63 | clone_behavior | O：折叠单行（NotFound；Clone 行为标记） | ui-input\decomp\decomp-0x140002370.txt |
| `country_core::systems::ui_clutter::decoclutter_show_pins::impl$1::register_required_components` | 0x141237200 | 63 | register_required_components | O：bevy 组件注册（有 decomp） | ui-input\decomp\decomp-0x141237200.txt |
| `country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins` | 0x1412357c0 | 72 | 系统:周期性处理（decoclutter_show_pins） | O：6.7KB（wp20 + 本域复核） | ui-input\decomp\decomp-0x1412357c0.txt |

## 实现方式（算法级）

- `decoclutter_show_pins` @0x1412357c0（O，6.7KB，wp20 + 本域）：显示/隐藏已固定 deco-clutter 图钉（DecoClutterLock / ShowPinnedDecoClutter）。
  - 遍历 pinned clutter 集合，按 ShowPinnedDecoClutter 状态切换图钉实体可见性；局部 helper `FUN_141233f80`（图钉槽查找，I）。
  - 事件/串区 UNK_142c1e9/142c1e6；`FUN_1428d9390`（索引分发表）参与选择。
  - `register_required_components` @0x141237200（O）：bevy 组件注册；`clone_behavior` @0x140002370：ICF 折叠单行（O）。
