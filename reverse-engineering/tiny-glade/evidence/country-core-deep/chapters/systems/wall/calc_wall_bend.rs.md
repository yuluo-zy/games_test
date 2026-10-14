# systems/wall/calc_wall_bend.rs

> 文件作用：墙弯折角重算系统：消费 OnWallChanged，读发布曲线前 4 段法向 delta 相邻点积平均到 acos 到 clamp<=1.0（点>5 才算，否则 0），写 WallBendChangedEvent（16B 行）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::calc_wall_bend::calc_wall_bend` | 0x140a8a710 | 13 | calc_wall_bend（>5 点算 bend，acos clamp<=1.0；写 WallBendChangedEvent） | O | wall\decomp\decomp-0x140a8a710.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
