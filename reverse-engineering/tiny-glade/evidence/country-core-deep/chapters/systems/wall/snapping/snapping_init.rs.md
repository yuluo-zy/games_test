# systems/wall/snapping/snapping_init.rs

> 文件作用：吸附初始化：重置 mode/snapped；DebugGui 禁用或进行中墙空返回；首段长<=1.65 返回；后续点投影首段 <=1.5 到 mode=1+snapped。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::snapping_init::snapping_init` | 0x140ada600 | 11 | snapping_init（重置；首段<=1.65 返回；投影<=1.5 → mode=1） | O | wall\decomp\decomp-0x140ada600.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
