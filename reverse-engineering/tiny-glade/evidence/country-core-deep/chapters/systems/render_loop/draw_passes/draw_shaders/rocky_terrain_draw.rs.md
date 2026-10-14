# systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs

> 文件作用：岩质地形绘制：`rocky_terrain_shader`（岩质地形网格绘制）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…draw_shaders::rocky_terrain_draw::rocky_terrain_shader` | 0x1409cb4e0 | 16 | 岩质地形绘制：DrawBuilder draw（岩质地形网格；读 rocky_terrain 位移数据） | I（绘制结构） | render-loop\decomp\decomp-0x1409cb4e0.txt；terrain-water\decomp\decomp-0x1409cb4e0.txt |

## 关键事实

- 消费 displace_rocky_terrain（W4）位移结果。
