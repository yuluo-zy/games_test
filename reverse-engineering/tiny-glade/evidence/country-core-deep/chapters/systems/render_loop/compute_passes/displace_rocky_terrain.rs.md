# systems/render_loop/compute_passes/displace_rocky_terrain.rs

> 文件作用：岩质地形位移：`displace_rocky_terrain`（W4，按 CPU 岩质地层掩码在 GPU 位移地形高度）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::displace_rocky_terrain::displace_rocky_terrain` | 0x140a3ee80 | 14 | W4 岩质地形位移：串证据 `Displace Rocky Terrain/rocky_terrain/displace_rocky_terrain.cs.hlsl`；bind_terrain_heightmap + read_texture + push_constants + dispatch | O | render-loop\decomp\decomp-0x140a3ee80.txt（串：Displace Rocky Terrain、/rocky_terrain/displace_rocky_terrain.cs.hlsl）；terrain-water\decomp\decomp-0x140a3ee80.txt；wp5 compute-passes §21 |

## 关键事实

- 读 rocky_terrain 掩码/高度，位移地形网格（rocky_terrain 域资源）。
- 帧序 W4（frame-wiring，W3 rasterize_terrain_stroke 后）。
