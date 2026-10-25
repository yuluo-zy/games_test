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

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- 三个 nani roof 实例绘制（`draw_instanced_roof_shader_nani` 0x1409d9070 / `_spire` 0x1409d96a0 / `_support_beam` 0x1409d98a0）：DrawBuilder draw_instanced。
- 回调（d_cb_b §2 屋顶族）：`roof_nani_cb`（`RoofHighlight`）、`roof_spire_cb`、`roof_support_beam_cb`（`AssetSsboLibrary`→`LibView`）；`roof_tiles_gravity`（W19）生成屋顶砖；`RoofSpireSsbo`/`RoofSupportBeamSsbo` 实例缓冲消费。
- 数据流：`RoofTileSsbo`/`WoodenPlankSsbo`（SparseInstanceBuffer 分块，sparse 章）→ nani draw list → draw_instanced。
- shader body 不挖（GPU 后置）。

## 关键事实

- 消费 roof 域实例缓冲（WoodenPlank/RoofTile 相关）。
