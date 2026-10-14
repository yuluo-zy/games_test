# systems/render_loop/compute_passes/rasterize_buildings.rs

> 文件作用：建筑掩码光栅：`rasterize_buildings`（W11，把建筑/地面物体光栅到掩码纹理 + 高斯模糊 + 动画掩码）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings` | 0x140a32770 | 20 | W11 建筑光栅：串证据 `Animate buildings mask/rasterize_buildings_animate.cs.hlsl` + `/rasterize_buildings_animate.cs.hlsl` + `Blur buildings mask/gauss_blur_building_mask.cs.hlsl` + buildings blur temp；`/rasterize_buildings.cs.hlsl` + `/gauss_blur_building_mask.cs.hlsl`；HashMap::iter + AssetSsboLibrary::get + InstanceBuffer::gpu_write + write_texture + dispatch_2d；`GroundedBuildingsSsbo` | O | render-loop\decomp\decomp-0x140a32770.txt（串：Animate buildings mask、/rasterize_buildings_animate.cs.hlsl、Blur buildings mask、/gauss_blur_building_mask.cs.hlsl、buildings blur temp）；wp5 compute-passes §21 |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项（HashMap 迭代闭包）

## 关键事实

- 结构体（U 布局）：`GroundedBuildingsSsbo`；输出建筑掩码（供植被/布置避开建筑）。
- 帧序 W11（frame-wiring）。
