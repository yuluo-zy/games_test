# systems/wall/wall_highlight.rs

> 文件作用：墙高亮系统：消费 HighlightWallCmd，WallEntities 查实体，写 WallHighlight{entity, wall, ty}；EditingHighlight rgba 按 ty 四常量选择。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_highlight::wall_highlight` | 0x140981390 | 17 | wall_highlight（WallEntities 查表 + WallHighlight 写入 + ty→rgba 四常量） | O | wall\decomp\decomp-0x140981390.txt |
