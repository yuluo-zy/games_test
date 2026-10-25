# systems/collision/visualize_colliders.rs

> 文件作用：**debug 碰撞体线框可视化**

函数总计 6：业务 2 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::collision::visualize_colliders::visualize_colliders` | 0x141326370 | 28 | 系统:调试 | O：visualize_colliders——debug 线框遍历 | collision-eco\decomp\decomp-0x141326370.txt |
| `country_core::systems::collision::visualize_colliders::visualize_shape` | 0x141326930 | 135 | 系统:调试 | O：visualize_shape——parry 形状→线框图元 | collision-eco\decomp\decomp-0x141326930.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项

## 实现方式（算法级）

- visualize_colliders（0x141326370）：遍历 RaycastWorld 17 槽 collider 集合 → 每 collider 形状 → 线框图元队列（debug 视图）；visualize_shape（0x141326930）按 parry 形状类型（凸/网格/球/立方体）生成边/面片图元。两函数体为重建 stub（图元队列类型 U）。
