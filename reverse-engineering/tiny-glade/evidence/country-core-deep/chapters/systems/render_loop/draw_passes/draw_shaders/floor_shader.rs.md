# systems/render_loop/draw_passes/draw_shaders/floor_shader.rs

> 文件作用：地板 shader：`draw_floor_shader`（木/石地板绘制）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…draw_shaders::floor_shader::draw_floor_shader` | 0x1409cacb0 | 14 | 地板绘制：DrawBuilder draw（地板材质；结合 bind_stone/wooden_floor_textures） | I（绘制结构；无独立串证据） | render-loop\decomp\decomp-0x1409cacb0.txt |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `draw_floor_shader`（0x1409cacb0）：DrawBuilder draw；与 `bind_stone_floor_textures`（0x14090c300）/`bind_wooden_floor_textures`（0x14090c980）搭配（石/木地板纹理 read_texture 绑定）。
- 分派链：draw_lists 的 `floor` 材质名 → `draw_floor_shader(b, false)`（closure_env$46 = `draw_floor_shader(b, false)`，d_gap §1.3 恢复）；floor 的 nani 对应 = `RoofFloor`（kind 9）/`StairsFloor`（kind 0x1a）实例数据（nani systems 章）。

## 关键事实

- 与 common_draw_params 的 bind_stone_floor_textures/bind_wooden_floor_textures 搭配。
