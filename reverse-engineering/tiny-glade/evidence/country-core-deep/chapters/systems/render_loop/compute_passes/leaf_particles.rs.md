# systems/render_loop/compute_passes/leaf_particles.rs

> 文件作用：树叶粒子计算：`leaf_particles_compute`（W18，树叶粒子模拟/更新）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::leaf_particles::leaf_particles_compute` | 0x1408f4d00 | 19 | W18 树叶粒子：串证据 `LeafParticles/leaf_particles.cs.hlsl` + `/leaf_particles.cs.hlsl`；utils::get_res + min + uniform + dispatch | O | render-loop\decomp\decomp-0x1408f4d00.txt（串：LeafParticles、/leaf_particles.cs.hlsl）；wp5 compute-passes §21 |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项（LeafParticleSsbo 等默认）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `leaf_particles_compute`（0x1408f4d00，W18）
处理链（O，B_c2 §2）：
1. **粒子总数 `min(n, 0x340)`，n=0 直接返回**；
2. `total_particle_count` / `particle_emitter_count` 两个 i32 uniform（`LoadedLeafParticleCount(usize)` TypeId 0x3dce… 与 `LoadedParticleEmitterCount(u64)` 0x4259…，c2 §3 拆名）；
3. **第二资源在第一个 uniform 之后才取**（缺失 unwrap panic）；
4. `dispatch_1d(n)`。
- 数据流：LeafParticleSsbo → `leaf_particle_draw`（draw_shaders 章）draw_instanced。
- 帧序 W18（roof_tiles_gravity W19 前）。

## 关键事实

- 结构体（U 布局）：`LeafParticleSsbo/ParticleEmitterSsbo/LoadedParticleEmitterCount`（wp5 记名）。
- 粒子由 leaf_particle_draw（draw_shaders）消费绘制。
