# systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs

> 文件作用：林中空地树木 shader：`tree_trunk`、`tree_canopy`（树干/树冠绘制，draw_lists 分派目标）。

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…clearing_shaders::tree_trunk` | 0x1409c9830 | 26 | 树干绘制：DrawBuilder draw 分派目标（draw_lists 0x1409ba540 分派到 FUN_1409ca350=tree_canopy 邻近族）；name_hash 匹配 nani 树干材质 | I（分派目标；shader body 属 GPU 后置） | render-loop\decomp\decomp-0x1409c9830.txt；wp5 draw-passes §1 |
| `…clearing_shaders::tree_canopy` | 0x1409ca350 | 80 | 树冠绘制：draw_lists 分派目标（build_draw_lists 按 canopy_flowers 等名分派到 0x1409ca350） | O（分派目标证实） | render-loop\decomp\decomp-0x1409ca350.txt；wp5 draw-passes §1 |

## 关键事实

- `tree_canopy`（0x1409ca350）是 build_draw_lists（0x1409ba540）按材质名分派的目标之一。
- shader 内部算法不挖（GPU 后置项目规则）。
