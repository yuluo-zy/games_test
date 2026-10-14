# systems/wall/merge_walls.rs

> 文件作用：墙合并系统：检测共线/相邻墙（地形采样），Curve::crashy_new_from_points 合并曲线，InnerWalls::new_wall；经 DecoratorArchivist::decorator_cull_allow_any 放行装饰归档重挂。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::merge_walls::merge_walls` | 0x140999920 | 0 | merge_walls（地形采样 + crashy_new_from_points + decorator_cull_allow_any） | O | wall\decomp\decomp-0x140999920.txt |
