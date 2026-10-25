# systems/render_loop/compute_passes/terrain_lighting_texture.rs

> 文件作用：地形光照纹理：`calc_terrain_lighting_texture`（W24，按地形高度/阴影计算地形光照贴图）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::terrain_lighting_texture::calc_terrain_lighting_texture` | 0x140a51440 | 19 | W24 地形光照纹理：串证据 `Calculate Terrain Lighting Texture/terrain_lighting_texture.cs.hlsl` + `/terrain_lighting_texture.cs.hlsl` + terrain；bind_terrain_heightmap + bind_shadowmap + uniform + write_texture + dispatch | O | render-loop\decomp\decomp-0x140a51440.txt（串：Calculate Terrain Lighting Texture、/terrain_lighting_texture.cs.hlsl）；terrain-water\decomp\decomp-0x140a51440.txt；wp5 compute-passes §21 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `calc_terrain_lighting_texture`（0x140a51440，W24）
处理链（O，B_c2 §2）：
1. `get_rg_texture(8)`（terrain_lighting 预设 8）；
2. 写 preset 8（0x32）；`camera_position` Vec3（`CameraMatrices::eye_position` 0x140c96d40）；
3. `bool_render_shadows=true`；
4. `dispatch_2d(w,h)`；
5. **再 `generate_mips("terrain")`**（mip 链）。
- 数据流：terrain_lighting_texture → W38 ray_trace_gi / draw_passes `bind_terrain_lighting` + `deferred_raster`。
- 帧序 W24（shadow_pass W23 后、main_pass W25 前）。

## 关键事实

- 输出地形光照纹理（W38 ray_trace_gi / draw_passes bind_terrain_lighting 消费）。
