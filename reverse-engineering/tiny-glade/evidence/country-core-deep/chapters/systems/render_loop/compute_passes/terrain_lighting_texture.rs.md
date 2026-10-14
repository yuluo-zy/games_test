# systems/render_loop/compute_passes/terrain_lighting_texture.rs

> 文件作用：地形光照纹理：`calc_terrain_lighting_texture`（W24，按地形高度/阴影计算地形光照贴图）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::terrain_lighting_texture::calc_terrain_lighting_texture` | 0x140a51440 | 19 | W24 地形光照纹理：串证据 `Calculate Terrain Lighting Texture/terrain_lighting_texture.cs.hlsl` + `/terrain_lighting_texture.cs.hlsl` + terrain；bind_terrain_heightmap + bind_shadowmap + uniform + write_texture + dispatch | O | render-loop\decomp\decomp-0x140a51440.txt（串：Calculate Terrain Lighting Texture、/terrain_lighting_texture.cs.hlsl）；terrain-water\decomp\decomp-0x140a51440.txt；wp5 compute-passes §21 |

## 关键事实

- 输出地形光照纹理（W38 ray_trace_gi / draw_passes bind_terrain_lighting 消费）。
