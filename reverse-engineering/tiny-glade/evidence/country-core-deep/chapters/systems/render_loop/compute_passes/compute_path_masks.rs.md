# systems/render_loop/compute_passes/compute_path_masks.rs

> 文件作用：路径掩码计算：`compute_path_masks`（W10，合并路径排除区/方向收缩/可视化掩码）、`blur_path_mask`/`blur_exclusion_mask`（高斯模糊路径掩码）。

函数总计 3：业务 2 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks` | 0x1409014e0 | 26 | W10 路径掩码：串证据 `Blur Path Stroke/gauss_blur_r16f.cs.hlsl` + `Directional contraction/paths/directional_contract.cs.hlsl` + `star/star.raster.hlsl`；`/paths/{combine_path_exclusions,directional_contract,path_mask_visual}.cs.hlsl`；on_texture_processed + write_texture + dispatch_2d | O | render-loop\decomp\decomp-0x1409014e0.txt（串：Blur Path Stroke、/gauss_blur_r16f.cs.hlsl、Directional contraction、/paths/directional_contract.cs.hlsl）；round2\decomp-0x1409014e0.txt；wp5 compute-passes §21 |
| `country_core::systems::render_loop::compute_passes::compute_path_masks::blur_path_mask` | 0x140903350 | 121 | 路径掩码模糊：串证据 `Blur Path Stroke/gauss_blur_r16f.cs.hlsl`（r16f 高斯）；`/gauss_blur_mask.cs.hlsl` 族（wp5 I） | O | render-loop\decomp\decomp-0x140903350.txt；wp5 compute-passes §21 |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `blur_exclusion_mask`（行 158）— 排除区掩码模糊（`/gauss_blur_mask.cs.hlsl` 族，wp5 I）

## 关键事实

- 路径掩码为后续路径光栅/pebbles 提供输入（W17 path_pebbles 消费）。
