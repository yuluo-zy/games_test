# systems/render_loop/compute_passes/roof_tiles_gravity.rs

> 文件作用：屋顶瓦片重力模拟：`roof_tiles_gravity_compute`（W19，屋顶瓦片受重力位移/掉落模拟）。

函数总计 5：业务 1 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute` | 0x140a29b10 | 13 | W19 屋顶瓦片重力：串证据 `Roof tiles/roof_tiles_gravity.cs.hlsl` + `/roof_tiles_gravity.cs.hlsl`；AssetSsboLibrary::get（RoofTileSsbo）+ World::query（RoofTiles 组件）+ flat_map + uniform；PushConstants（闭包 closure$0/closure$1，dispatch_1d）；`+frame_constants_storage_buffer@1.1`（O） | O | render-loop\decomp\decomp-0x140a29b10.txt（串：Roof tiles、/roof_tiles_gravity.cs.hlsl）；wp5 compute-passes §21 |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项（PushConstants 构造闭包 ×2 + 迭代器）

## 关键事实

- 读 `RoofTileSsbo`（roof_tile_ssbo 域资源，nani 剔除同用）；查询 `RoofTiles` 组件。
- 帧序 W19（frame-wiring）。
