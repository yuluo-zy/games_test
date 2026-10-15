# systems/ivy/ivy_branch_thickness_animation.rs

> 文件作用：**分支粗细动画**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ivy::ivy_branch_thickness_animation::ivy_branch_thickness_animation` | 0x140995a80 | 7 | 系统:动画 | O：ivy_branch_thickness_animation——分支粗细动画 | collision-eco\decomp\decomp-0x140995a80.txt |

## 实现方式（算法级）

- ivy_branch_thickness_animation（0x140995a80）：遍历 Ivy 各株各段 → 按段龄/生长状态平滑过渡粗细参数（GPU 段属性），随 grower 推进每帧更新。
