# systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs

> 文件作用：墙高 widget/曲线 UI：UiWallHeight{widget,signifier,new} + adjust_wall_height_curve 插值曲线（widget 层）。

函数总计 7：业务 2 / 内联无独立符号 2 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier` | 0x140956cd0 | 0 | UiWallHeight::signifier（构建 wall_height_edit_ signifier：读预览曲线点数组 +0x90/+0xe0、长度 +0x98/+0xe8） | O | ui-input\decomp\decomp-0x140956cd0.txt, wall\decomp\decomp-0x140956cd0.txt |
| `country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget` | 0x1409566d0 | 0 | UiWallHeight::new/widget 族 | O | ui-input\decomp\decomp-0x1409566d0.txt, wall\decomp\decomp-0x1409566d0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::new`（行 32）— 构造
- `country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve`（行 165）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项
