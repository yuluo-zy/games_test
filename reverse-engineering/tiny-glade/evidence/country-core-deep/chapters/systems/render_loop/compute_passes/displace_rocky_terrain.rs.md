# systems/render_loop/compute_passes/displace_rocky_terrain.rs

> 文件作用：岩质地形位移：`displace_rocky_terrain`（W4，按 CPU 岩质地层掩码在 GPU 位移地形高度）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::displace_rocky_terrain::displace_rocky_terrain` | 0x140a3ee80 | 14 | W4 岩质地形位移：串证据 `Displace Rocky Terrain/rocky_terrain/displace_rocky_terrain.cs.hlsl`；bind_terrain_heightmap + read_texture + push_constants + dispatch | O | render-loop\decomp\decomp-0x140a3ee80.txt（串：Displace Rocky Terrain、/rocky_terrain/displace_rocky_terrain.cs.hlsl）；terrain-water\decomp\decomp-0x140a3ee80.txt；wp5 compute-passes §21 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `displace_rocky_terrain`（0x140a3ee80，W4）
处理链（O，B_c2 §2）：
1. **首 qword `/3` 截 u32 = 组数**；
2. 5 个纹理（预设 9/0xa/0xe/5/0x13：terrain_heightmap / visual_terrain_heightmap / rocky_terrain / +2）；
3. push constants 4B；`dispatch_1d`。
- 数据流：rocky 掩码/高度（`displace_rocky_terrain.cs.hlsl`）→ 位移地形网格 → `rocky_terrain_shader`（draw_shaders 章）读 RockyTerrainVertex/TriangleSSBO。
- 帧序 W4（W3 rasterize_terrain_stroke 后）。

## 关键事实

- 读 rocky_terrain 掩码/高度，位移地形网格（rocky_terrain 域资源）。
- 帧序 W4（frame-wiring，W3 rasterize_terrain_stroke 后）。
