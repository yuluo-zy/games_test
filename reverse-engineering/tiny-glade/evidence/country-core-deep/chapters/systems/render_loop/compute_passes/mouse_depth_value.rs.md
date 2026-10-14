# systems/render_loop/compute_passes/mouse_depth_value.rs

> 文件作用：鼠标深度回读：`mouse_depth_compute`（W33，把鼠标位置写入深度值 SSBO）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute` | 0x140a01620 | 15 | W33 鼠标深度：`/read_mouse_depth_value.cs.hlsl`；WindowSize::as_vec2 + clamp + read_texture + uniform + dispatch；写 `MouseDepthValueSsbo`（write_to_cpu_mapped_storage_buffer，wp1 §1.1） | O | render-loop\decomp\decomp-0x140a01620.txt；wp5 compute-passes §15 |

## 关键事实

- 结构体（U 布局）：`MouseDepthValueSsbo`（CPU 映射回读，供光标地形射线/UI 使用）。
