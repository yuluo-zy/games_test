# systems/paths/record_path_history.rs

> 文件作用：**路径/石板编辑历史记录（4 命令→撤销历史）**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::paths::record_path_history::record_path_history` | 0x140b1c270 | 0 | 系统:历史 | O：record_path_history——4 命令事件游标差→历史写入 | collision-eco\decomp\decomp-0x140b1c270.txt |

## 实现方式（算法级）

- record_path_history（0x140b1c270）：Events<T> 游标差（当前 − 上次 → 饱和/边界 clamp）→ 逐新事件把 4 类路径/石板命令实体（PathDraw/Paint/Eraser/石板 系）写入撤销历史（history 域交叉）。
