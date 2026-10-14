# systems/wall/wall_splitter.rs

> 文件作用：墙分裂系统：光标拖过墙到 get_curve_segment_from_u/get_u_at_coord 求 split u 到 ChildWall::new 两子墙到 InnerWalls::new_wall；命中处 WallStyle::convert_to_stone_version；装饰经归档资源重挂（split_wall/split_wall_scissors signifier）。

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_splitter::wall_splitter` | 0x140a74c20 | 0 | wall_splitter（get_curve_segment_from_u/get_u_at_coord + ChildWall::new + convert_to_stone_version） | O | wall\decomp\decomp-0x140a74c20.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项
