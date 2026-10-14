# resources/walls/material_stats.rs

> 文件作用：墙材质统计：7 特征长度 prev@+0x28..+0x40/cur@+0x90..+0xa8（plain/lintels/arches/栅栏/gates/stairs/sewers）、8 段/柱计数 u32@+0xac..+0xc8、seed u64@+0x88、pillar placements Vec@+0x70{cap,ptr,len}（自旋锁@+0x68 保护）；add_*_length 为 CAS 原子累加（加本特征扣 plain 预算）；which_material_changed_the_most = 7 对 |cur-prev| argmax（plain 带 -0.01 bias）。

函数总计 34：业务 18 / 内联无独立符号 14 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::material_stats::MaterialStats::init_this_update` | 0x140a88280 | 45 | init_this_update（帧初状态推进） | O | wall\decomp\decomp-0x140a88280.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_pillar_placements` | 0x140a88320 | 63 | set_pillar_placements（自旋锁 + futex 式等待 + 换入 Vec@+0x70{cap,ptr,len}） | O | wall\decomp\decomp-0x140a88320.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_lintel_pillars_count` | 0x140a88420 | 75 | set_lintel_pillars_count（u32@+0xb4） | O | wall\decomp\decomp-0x140a88420.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_arch_pillars_count` | 0x140a88430 | 81 | set_arch_pillars_count（u32@+0xbc） | O | wall\decomp\decomp-0x140a88430.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_path_segmentation_count` | 0x140a88440 | 87 | set_path_segmentation_count（u32@+0xac） | O | wall\decomp\decomp-0x140a88440.txt |
| `country_core::resources::walls::material_stats::MaterialStats::add_lintels_length` | 0x140a88450 | 93 | add_lintels_length（CAS +0x94+=x, +0x9c-=x） | O | wall\decomp\decomp-0x140a88450.txt |
| `country_core::resources::walls::material_stats::MaterialStats::add_arches_length` | 0x140a884a0 | 98 | add_arches_length（CAS +0x98+=x, +0x90-=x） | O | wall\decomp\decomp-0x140a884a0.txt |
| `country_core::resources::walls::material_stats::MaterialStats::add_gate_length` | 0x140a884f0 | 103 | add_gate_length（CAS +0xa0+=x, +0x90-=x） | O | wall\decomp\decomp-0x140a884f0.txt |
| `country_core::resources::walls::material_stats::MaterialStats::add_sewer_length` | 0x140a88540 | 108 | add_sewer_length（CAS +0xa8+=x, +0x90-=x） | O | wall\decomp\decomp-0x140a88540.txt |
| `country_core::resources::walls::material_stats::MaterialStats::add_stairs_length` | 0x140a88590 | 113 | add_stairs_length（CAS +0xa4+=x, +0x90-=x） | O | wall\decomp\decomp-0x140a88590.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_lintels_segments_count` | 0x140a885e0 | 118 | set_lintels_segments_count（u32@+0xb0） | O | wall\decomp\decomp-0x140a885e0.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_arches_segments_count` | 0x140a885f0 | 124 | set_arches_segments_count（u32@+0xb8） | O | wall\decomp\decomp-0x140a885f0.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_gates_segments_count` | 0x140a88600 | 128 | set_gates_segments_count（u32@+0xc0） | O | wall\decomp\decomp-0x140a88600.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_sewer_segments_count` | 0x140a88610 | 132 | set_sewer_segments_count（u32@+0xc4） | O | wall\decomp\decomp-0x140a88610.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_count` | 0x140a88620 | 136 | set_fences_segments_count（u32@+0xc8） | O | wall\decomp\decomp-0x140a88620.txt |
| `country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_seed` | 0x140a88630 | 140 | set_fences_segments_seed（u64@+0x88） | O | wall\decomp\decomp-0x140a88630.txt |
| `country_core::resources::walls::material_stats::MaterialStats::max_pillar_movement` | 0x140a88640 | 204 | max_pillar_movement | I | wall\decomp\decomp-0x140a88640.txt |
| `country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most` | 0x140a887d0 | 280 | which_material_changed_the_most（7 对 |cur-prev| argmax 索引 0..6，plain 带 -0.01 bias，前 5 对和>0 门控） | O | wall\decomp\decomp-0x140a887d0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::material_stats::MaterialStats::pillar_count`（行 67）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::path_segmentation_delta`（行 143）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::has_fence_seed_changed`（行 157）— 谓词/校验
- `country_core::resources::walls::material_stats::MaterialStats::fence_length_delta`（行 161）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::bricks_length_delta`（行 166）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::lintels_length_delta`（行 171）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::gates_length_delta`（行 176）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::sewer_length_delta`（行 180）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::lintels_pillar_delta`（行 194）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::arches_pillar_delta`（行 199）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::arches_length_delta`（行 220）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::stairs_length_delta`（行 225）— 待推断
- `country_core::resources::walls::material_stats::MaterialStats::has_material_switched`（行 230）— 谓词/校验
- `country_core::resources::walls::material_stats::MaterialStats::latest_update_material_proportions`（行 267）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项
