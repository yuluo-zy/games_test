# systems/render_loop/compute_passes/reprojection.rs

> 文件作用：重投影图：`calculate_reprojection_map`（W36，深度→重投影映射图，供时域滤波）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::reprojection::calculate_reprojection_map` | 0x140a0d190 | 11 | W36 重投影图：串证据 `Reprojection map/calculate_reprojection_map.cs.hlsl` + `Copy depth/copy_depth_to_r.cs.hlsl` + reprojection map；uniform + read_texture + write_texture + dispatch_2d | O | render-loop\decomp\decomp-0x140a0d190.txt（串：Reprojection map、/calculate_reprojection_map.cs.hlsl、Copy depth、/copy_depth_to_r.cs.hlsl）；wp5 compute-passes §9 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `calculate_reprojection_map`（0x140a0d190，W36）
处理链（O，B_c3 §2）：
1. 新建 "reprojection map"（**深度 extent、格式 0x5c**）；
2. "Reprojection map" pass（`/calculate_reprojection_map.cs.hlsl`）：`history_valid` = movzx 字节；
3. "Copy depth" pass（`/copy_depth_to_r.cs.hlsl`）：**深度拷进历史**。
- 数据流：重投影图 → SSGI（W37）/ReSTIR（W41）/DoF 时域滤波消费 + `water/ice_surface`（W45）的 `reprojection_tex`（D 槽）。
- 帧序 W36（W35 半分辨率后）。

## 关键事实

- 输出重投影图（SSGI/ReSTIR/DoF 时域滤波消费）。
