# systems/render_loop/compute_passes/blur_pyramid.rs

> 文件作用：模糊金字塔原语（复用型，非独立帧 pass）：`bidirectional_blur_pyramid`（双向 = 下采样 + 反向上采样）、`blur_pyramid`（下采样）、`rev_blur_pyramid`（反向）。供 GI/nani/后处理复用。

函数总计 3：业务 2 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::blur_pyramid::bidirectional_blur_pyramid` | 0x1409ed860 | 10 | 双向模糊金字塔：串证据 "full blur pyramid" + "rev_blur" + `/blur_pyramid/rev_blur.cs.hlsl`；组合 downsample+upsample（wp5：rev_blur_pyramid 供 GI/nani 复用） | O | render-loop\decomp\decomp-0x1409ed860.txt（串：full blur pyramid、rev_blur、/blur_pyramid/rev_blur.cs.hlsl）；wp5 compute-passes §19 |
| `country_core::systems::render_loop::compute_passes::blur_pyramid::blur_pyramid` | 0x1409ee1e0 | 21 | 模糊金字塔下采样：串证据 "blur" + `/blur_pyramid/blur.cs.hlsl`；dispatch_2d，overflowing_sub+max 迭代（wp5 O） | O | render-loop\decomp\decomp-0x1409ee1e0.txt（串：blur、/blur_pyramid/blur.cs.hlsl）；wp5 compute-passes §19 |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `rev_blur_pyramid`（行 98）— 反向（上采样）金字塔：dispatch_3d + read_texture_view（wp5 O）

## 关键事实

- 具体复用点（GI/nani/后处理）：U（wp5 同注）。
