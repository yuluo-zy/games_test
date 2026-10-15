# systems/render_loop/compute_passes/rasterize_buildings.rs

> 文件作用：建筑掩码光栅：`rasterize_buildings`（W11，把建筑/地面物体光栅到掩码纹理 + 高斯模糊 + 动画掩码）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings` | 0x140a32770 | 20 | W11 建筑光栅：串证据 `Animate buildings mask/rasterize_buildings_animate.cs.hlsl` + `/rasterize_buildings_animate.cs.hlsl` + `Blur buildings mask/gauss_blur_building_mask.cs.hlsl` + buildings blur temp；`/rasterize_buildings.cs.hlsl` + `/gauss_blur_building_mask.cs.hlsl`；HashMap::iter + AssetSsboLibrary::get + InstanceBuffer::gpu_write + write_texture + dispatch_2d；`GroundedBuildingsSsbo` | O | render-loop\decomp\decomp-0x140a32770.txt（串：Animate buildings mask、/rasterize_buildings_animate.cs.hlsl、Blur buildings mask、/gauss_blur_building_mask.cs.hlsl、buildings blur temp）；wp5 compute-passes §21 |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项（HashMap 迭代闭包）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `rasterize_buildings`（0x140a32770 + 0x1412fc510，W11）
处理链（O，B_c3 §2）：
1. `filter_map` 闭包（**进度 >0.05 / kind>1 丢弃**，圆/矩形两种 **32B 元素**）；
2. ≥5000 个 → **`"Exceeded maximum number of buildings per level"`**；
3. **未注册缓冲 → 静默跳过上传**；
4. 4 个 pass（`/rasterize_buildings.cs.hlsl` + `/rasterize_buildings_animate.cs.hlsl` + `/gauss_blur_building_mask.cs.hlsl` ×2）；
5. `HashMap::iter` + `AssetSsboLibrary::get` + `InstanceBuffer::gpu_write` + write_texture + dispatch_2d。
- 元素 32B（B_c3 §4）：kind==1 → `[w3c,w38,v34.0,v34.1,w44,w40,1,0]`；kind==0 → `[w44,w40,w3c,w3c,0,0,0,0]`（字段名全 U）。
- 数据流：建筑掩码 → 植被/布置避开建筑 + `garden/water spawn` 排除；帧序 W11。

## 关键事实

- 结构体（U 布局）：`GroundedBuildingsSsbo`；输出建筑掩码（供植被/布置避开建筑）。
- 帧序 W11（frame-wiring）。
