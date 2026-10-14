# systems/render_loop/draw_passes/object_grass_push.rs

> 文件作用：物体草地推入：`object_grass_push_pass`（W9，把相机/光标附近草地实例推入 ObjectGrass 渲染缓冲）。

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass` | 0x1409daaa0 | 26 | W9 物体草地推入：串证据 `Grass object push pass` + `grass_object_push` + `_grass_object_push.raster`（wp5 O）；puffin scope + utils::get_res/get_res_mut + MainCamera::eye_position + CursorTerrainRaycast::position（closure$1 eye/cursor → Vec3::extend Vec4）+ World::query + Iterator::chain + RasterPass::new + DrawPacket::new + uniform + set_depth_test_enable/set_blend_mode | O | render-loop\decomp\decomp-0x1409daaa0.txt（串：Grass object push pass、grass_object_push）；wp5 draw-passes §8 |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项（closure$1：eye/cursor → Vec4）

## 关键事实

- ObjectGrass 渲染缓冲（与 grass.rs compute 共享 ObjectGrassPush）。
- 帧序 W9（frame-wiring）。
