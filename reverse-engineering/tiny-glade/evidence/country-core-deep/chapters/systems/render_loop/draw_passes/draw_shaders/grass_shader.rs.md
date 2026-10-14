# systems/render_loop/draw_passes/draw_shaders/grass_shader.rs

> 文件作用：草地绘制 shader：`draw_common`（公共绑定）、`draw_grass_shader`、`draw_grass_lod1_shader`（LOD1 草地）。

函数总计 3：业务 2 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…grass_shader::draw_grass_shader` | 0x140b4af70 | 37 | 草地绘制：DrawBuilder draw（草地实例；读 GrassBladeDataSsbo） | I（绘制结构） | render-loop\decomp\decomp-0x140b4af70.txt |
| `…grass_shader::draw_grass_lod1_shader` | 0x140b4b1d0 | 47 | 草地 LOD1 绘制：同型（远距离草地 LOD） | I | render-loop\decomp\decomp-0x140b4b1d0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `draw_common`（行 18）— 公共绑定（两 draw 共用）

## 关键事实

- 消费 grass.rs 生成的草地间接绘制缓冲。
