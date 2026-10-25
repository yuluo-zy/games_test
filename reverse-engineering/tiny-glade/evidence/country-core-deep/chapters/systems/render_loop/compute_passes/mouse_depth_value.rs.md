# systems/render_loop/compute_passes/mouse_depth_value.rs

> 文件作用：鼠标深度回读：`mouse_depth_compute`（W33，把鼠标位置写入深度值 SSBO）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute` | 0x140a01620 | 15 | W33 鼠标深度：`/read_mouse_depth_value.cs.hlsl`；WindowSize::as_vec2 + clamp + read_texture + uniform + dispatch；写 `MouseDepthValueSsbo`（write_to_cpu_mapped_storage_buffer，wp1 §1.1） | O | render-loop\decomp\decomp-0x140a01620.txt；wp5 compute-passes §15 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `mouse_depth_compute`（0x140a01620，W33）
处理链（O，B_c3 §2）：
1. 三个 world 资源 → **光标 Vec2**、`filter_box_y_frac = clamp(NaN→0)`、WindowSize → Vec2；
2. `read_texture` + uniform + `dispatch_1d(1)`；
3. 写 `MouseDepthValueSsbo`（**write_to_cpu_mapped_storage_buffer**，`MappedStorageBuffer` 16B `float4(world.xyz, depth)`，memory 4 回读，B_i_infra §4）。
- 数据流：鼠标深度 → `dof_autofocus`（W32 的 point_uv 鼠标模式）、光标地形射线（terrain-water 域）、`mouse_read_terrain_stroke_index`（W6）。

## 关键事实

- 结构体（U 布局）：`MouseDepthValueSsbo`（CPU 映射回读，供光标地形射线/UI 使用）。
