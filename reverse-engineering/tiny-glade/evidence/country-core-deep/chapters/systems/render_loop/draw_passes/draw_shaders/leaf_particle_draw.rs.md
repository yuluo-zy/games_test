# systems/render_loop/draw_passes/draw_shaders/leaf_particle_draw.rs

> 文件作用：树叶粒子绘制：`leaf_particle_draw`（draw_lists 分派目标，draw_instanced 树叶粒子）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…draw_shaders::leaf_particle_draw::leaf_particle_draw` | 0x140a8a090 | 13 | 树叶粒子绘制：draw_lists（0x1409ba540）按 `leaf_particle` 名分派目标；DrawBuilder draw_instanced（读 LeafParticleSsbo） | O（分派目标） | render-loop\decomp\decomp-0x140a8a090.txt；wp5 draw-passes §1 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- draw_lists `leaf_particle` 名分派目标；DrawBuilder draw_instanced（读 `LeafParticleSsbo`）。回调 `leaf_particle_cb`（d_cb_b §2）：取 `GladeSettings`（落叶粒子数资源，B_d_cb_b §3.3 纠错）+ 组件。
- 数据流：`leaf_particles_compute`（W18，`total_particle_count`/`particle_emitter_count` uniform，dispatch_1d(n)，n=min(0x340)）→ LeafParticleSsbo → draw_instanced。
- shader body 不挖（GPU 后置）。

## 关键事实

- 消费 leaf_particles.rs（W18）生成的粒子缓冲。
