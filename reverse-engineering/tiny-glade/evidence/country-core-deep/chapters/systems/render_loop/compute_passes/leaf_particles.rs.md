# systems/render_loop/compute_passes/leaf_particles.rs

> 文件作用：树叶粒子计算：`leaf_particles_compute`（W18，树叶粒子模拟/更新）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::leaf_particles::leaf_particles_compute` | 0x1408f4d00 | 19 | W18 树叶粒子：串证据 `LeafParticles/leaf_particles.cs.hlsl` + `/leaf_particles.cs.hlsl`；utils::get_res + min + uniform + dispatch | O | render-loop\decomp\decomp-0x1408f4d00.txt（串：LeafParticles、/leaf_particles.cs.hlsl）；wp5 compute-passes §21 |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项（LeafParticleSsbo 等默认）

## 关键事实

- 结构体（U 布局）：`LeafParticleSsbo/ParticleEmitterSsbo/LoadedParticleEmitterCount`（wp5 记名）。
- 粒子由 leaf_particle_draw（draw_shaders）消费绘制。
