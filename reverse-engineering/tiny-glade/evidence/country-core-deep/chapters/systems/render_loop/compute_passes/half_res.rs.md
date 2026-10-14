# systems/render_loop/compute_passes/half_res.rs

> 文件作用：半分辨率深度/法线提取：`extract_half_res_depth_normals`（W35，从主 G-buffer 深度+模板提取半分辨率深度/法线，供 GI/后处理）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals` | 0x1408f6180 | 14 | W35 半分辨率深度/法线：串证据 `/extract_half_res_depth_normals.cs.hlsl`；RenderGraphRecorder::create + read_texture（`gbuffer_input_tex`）+ read_texture_view（`stencil_tex`）+ write_texture + dispatch_2d | O | render-loop\decomp\decomp-0x1408f6180.txt（串：/extract_half_res_depth_normals.cs.hlsl）；wp5 compute-passes §8 |

## 关键事实

- 输出半分辨率深度/法线纹理供 GI（W38 ray_trace_gi）/SSGI 等消费。
