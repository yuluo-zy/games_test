# systems/ui_edit_freehand_walls.rs

> 文件作用：自由手绘墙编辑：拖拽笔划进 EditFreehandWallCache，松手提交墙段给 wall-constructor（wp20 合同）。

函数总计 4：业务 1 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_edit_freehand_walls::ui_edit_freehand_walls` | 0x140ac98e0 | 0 | 系统:周期性处理（ui_edit_freehand_walls） | O：12.1KB（wp20 + 本域复核一致；细节分支未逐行） | ui-input\decomp\decomp-0x140ac98e0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 3 项
