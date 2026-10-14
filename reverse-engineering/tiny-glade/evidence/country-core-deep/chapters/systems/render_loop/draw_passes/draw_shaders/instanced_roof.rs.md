# systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs

> 文件作用：屋顶实例绘制：`draw_instanced_roof_shader_nani`、`draw_instanced_roof_spire_shader`（尖顶）、`draw_instanced_roof_support_beam_shader`（支撑梁）。

函数总计 5：业务 3 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…instanced_roof::draw_instanced_roof_shader_nani` | 0x1409d9070 | 24 | 屋顶实例绘制（nani）：DrawBuilder draw_instanced（屋顶面） | I（绘制结构） | render-loop\decomp\decomp-0x1409d9070.txt |
| `…instanced_roof::draw_instanced_roof_spire_shader` | 0x1409d96a0 | 58 | 屋顶尖顶实例绘制：同型 | I | render-loop\decomp\decomp-0x1409d96a0.txt |
| `…instanced_roof::draw_instanced_roof_support_beam_shader` | 0x1409d98a0 | 73 | 屋顶支撑梁实例绘制：同型 | I | render-loop\decomp\decomp-0x1409d98a0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项

## 关键事实

- 消费 roof 域实例缓冲（WoodenPlank/RoofTile 相关）。
