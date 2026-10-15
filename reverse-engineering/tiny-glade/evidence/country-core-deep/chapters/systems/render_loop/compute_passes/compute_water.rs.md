# systems/render_loop/compute_passes/compute_water.rs

> 文件作用：水面掩码/水上植被生成：`water_rasterize_from_cpu`（W2，CPU 水面掩码 → GPU）、`water_spawn_vegetation`（W13，水面植被程序化生成：lilypads/reeds/boats/waterwheels）。

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu` | 0x140a12680 | 22 | W2 水面 CPU→GPU：串证据 `Blur water mask/gauss_blur_mask.cs.hlsl` + `water blur temp`；`/water/*` → `_water_draw_water_mask`；on_texture_processed + write_texture + dispatch_2d | O | render-loop\decomp\decomp-0x140a12680.txt（串：Blur water mask、/gauss_blur_mask.cs.hlsl、water blur temp）；terrain-water\decomp\decomp-0x140a12680.txt；wp5 compute-passes §20 |
| `country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation` | 0x140a13970 | 73 | W13 水面植被：串证据 `Spawn lilypads/water/spawn_lilypads.cs.hlsl` + `Spawn reeds/water/spawn_reeds.cs.hlsl` + sun_tint/moon_tint/sky_low/sky_high color 常量 + `Spawn boats/water/spawn_boats.cs.hlsl`（boats/wheels URI 动态拼接 U）；ActiveSession::is_autumn + bind_terrain_heightmap + write_texture + write_draw_indirect_cmd_buffer + dispatch | O | render-loop\decomp\decomp-0x140a13970.txt（串：Spawn lilypads/reeds、/water/spawn_lilypads.cs.hlsl、/water/spawn_reeds.cs.hlsl、sun_tint_colormoon_tint_colorsky_low_colorsky_high_color）；wp5 compute-passes §20 |

## 关键事实

- 结构体（U 布局）：`LilypadSsbo/LilypadFlowerSsbo/ReedSsbo/BoatSsbo`（wp5 记名）。
- 帧序 W2→W13（frame-wiring）。

## 实现方式（算法级）

### water_rasterize_from_cpu（W2，0x140a12680）

```
ssbo = 库按 type-id 0x1ad182936286a0ee 查 WaterRasterStagingSsbo
if raster+0x29b（mask 重传脏）:
  ssbo.version(+0x5c) ← 库 version(+0x5b0)；清 +0x29b；memset(bitmap@+0x98, 0, 0x200)
  建纹理（格式 0x61 = rgba16fr16f）
  pass：puffin "water blur temp"/"Blur water mask" + /gauss_blur_mask.cs.hlsl
  → 水 draw mask 上传 → write_texture + dispatch_2d
```

### water_spawn_vegetation（W13，0xa13970）

两 compute pass："Spawn lilypads" /water/spawn_lilypads.cs.hlsl（LilypadSsbo+LilypadFlowerSsbo）、"Spawn reeds" /water/spawn_reeds.cs.hlsl（ReedSsbo）+ indirect draw 缓冲；ActiveSession::is_autumn + bind_terrain_heightmap + dispatch。帧序 W2→W13。
