# systems/render_loop/draw_passes/draw_shaders/floor_shader.rs

> 文件作用：地板 shader：`draw_floor_shader`（木/石地板绘制）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…draw_shaders::floor_shader::draw_floor_shader` | 0x1409cacb0 | 14 | 地板绘制：DrawBuilder draw（地板材质；结合 bind_stone/wooden_floor_textures） | I（绘制结构；无独立串证据） | render-loop\decomp\decomp-0x1409cacb0.txt |

## 关键事实

- 与 common_draw_params 的 bind_stone_floor_textures/bind_wooden_floor_textures 搭配。
