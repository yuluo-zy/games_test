# systems/wall/structure_outline.rs

> 文件作用：结构描边系统：消费 OutlineWallCmd/OutlineStairsCmd，Esc 菜单关闭才画；逐墙 ShapeParameters（as_rectangle/as_circle/as_curve2/center/expand/perimeter）+ 曲线采样到 UiSignifierStream 描边 signifier（outline_wall_top_/bottom_/roof_ridge_/roof_side_/circle_*/rect_corner_ 名前缀）。

函数总计 17：业务 1 / 内联无独立符号 0 / 派生胶水 16

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::structure_outline::structure_outline` | 0x140959fc0 | 77 | structure_outline（Esc 门控 + ShapeParameters + outline_* signifier 族） | O | wall\decomp\decomp-0x140959fc0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 16 项
