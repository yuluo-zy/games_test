# systems/wall/set_wall_style.rs

> 文件作用：墙样式命令系统：三事件读器（SetWallStyleCmd 0x18B/SetFreehandHeightTypeCmd 0x20B/SetPlasterPeelAmountCmd 0x18B）；style 到 update_style + CompoundEditBuilder 历史；freehand ty 到 update_freehand_ty+update_arch_type；plaster peel 到 update_plaster_peel；样式改变后写 0x20B RequestWallMaterialAudio {tag=0x10, style, pos}。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::set_wall_style::set_wall_style` | 0x1409a2290 | 44 | set_wall_style（三读器 + CompoundEditBuilder + RequestWallMaterialAudio 0x20B） | O | wall\decomp\decomp-0x1409a2290.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
