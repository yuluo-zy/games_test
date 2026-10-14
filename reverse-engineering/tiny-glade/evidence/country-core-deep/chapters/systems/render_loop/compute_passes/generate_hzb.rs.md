# systems/render_loop/compute_passes/generate_hzb.rs

> 文件作用：HZB 生成：`generate_hzb`（W26，单趟 SPD 深度层级 Z-buffer，供遮挡剔除/nani cull 第 2 轮）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb` | 0x1409286f0 | 15 | W26 HZB 生成：串证据 `Generate HZB/generate_hzb.cs.hlsl` + HZB；SPD 单趟；ustr 名 + next_power_of_two + div_ceil + Handle::desc；read_texture（main 深度）+ read_write_texture（各 mip）+ push_constants（spdConstants）+ dispatch_groups；bind_individual_mips + write_rg_storage_buffer + get_or_create_temporal | O | render-loop\decomp\decomp-0x1409286f0.txt（串：Generate HZB、/generate_hzb.cs.hlsl、HZB）；wp5 compute-passes §17 |

## 关键事实

- HZB 纹理供 W27/W28（gpu_generate_draw_lists round 非 0）`read_texture`（hzb 变体 cull）。
- 帧序 W26（frame-wiring，主 depth 后、nani 第二轮前）。
