# systems/render_loop/draw_passes/object_grass_push.rs

> 文件作用：物体草地推入：`object_grass_push_pass`（W9，把相机/光标附近草地实例推入 ObjectGrass 渲染缓冲）。

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass` | 0x1409daaa0 | 26 | W9 物体草地推入：串证据 `Grass object push pass` + `grass_object_push` + `_grass_object_push.raster`（wp5 O）；puffin scope + utils::get_res/get_res_mut + MainCamera::eye_position + CursorTerrainRaycast::position（closure$1 eye/cursor → Vec3::extend Vec4）+ World::query + Iterator::chain + RasterPass::new + DrawPacket::new + uniform + set_depth_test_enable/set_blend_mode | O | render-loop\decomp\decomp-0x1409daaa0.txt（串：Grass object push pass、grass_object_push）；wp5 draw-passes §8 |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项（closure$1：eye/cursor → Vec4）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `object_grass_push_pass`（0x1409daaa0）—— 半径选择 + 推入（d_gap §6 完整解码）
- **半径选择**（O，GladeTheme 首 u32 t 查 14 项跳转表 0x142ac2dd8）：t=6 → `sub==0 ? 1.5 : 0`；7/8 → `P.f0 + 6.0`；9 → `sub!=3 ? 7.5 : 0`；10 → 7.5；11..16/范围外 → 0；17 → `P.f8*0.75 + 6.0`；18 → 1.5；19 → `CameraMatrices[+0x4d0]==3 ? 9.0 : 0`。
- **半径 > 0 才产生额外项**：`camera_mode==3` → `Vec4(eye_position, r)`，否则光标 `Vec4(hit ? pos : 0, r)`；查询项 = `Vec4(translation, ObjectGrassPush)`（**ObjectGrassPush 是单个 f32**，register_component 0x140a5fc10 Layout size 4/align 4，B_i_infra §5）。
- 处理链：puffin scope + `MainCamera::eye_position` + `CursorTerrainRaycast::position`（closure$1 eye/cursor → Vec3::extend Vec4）+ World query + `Iterator::chain` → RasterPass::new + DrawPacket::new + uniform + set_depth_test_enable/set_blend_mode。
- 着色器 `_grass_object_push.raster`（"Grass object push pass"）。
- 数据流：推入缓冲 → `grass_compute`（W29）读 `DrawIndexedIndirectBuffer/DrawIndirectBuffer/AtomicCounterBuffer` 消费（ObjectGrassPush 与 grass.rs compute 共享）。
- 帧序 W9。

## 关键事实

- ObjectGrass 渲染缓冲（与 grass.rs compute 共享 ObjectGrassPush）。
- 帧序 W9（frame-wiring）。
