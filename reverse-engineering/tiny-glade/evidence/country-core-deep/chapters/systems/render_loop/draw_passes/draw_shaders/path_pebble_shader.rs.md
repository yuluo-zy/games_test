# systems/render_loop/draw_passes/draw_shaders/path_pebble_shader.rs

> 文件作用：路径卵石绘制：`path_pebble_draw`（draw_lists 分派目标，draw_instanced 卵石）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…draw_shaders::path_pebble_shader::path_pebble_draw` | 0x1408e5a10 | 14 | 路径卵石绘制：DrawBuilder draw_instanced（读 PebblePlacementSsbo，消费 path_pebbles W17） | I（绘制结构） | render-loop\decomp\decomp-0x1408e5a10.txt |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `path_pebble_draw`（0x1408e5a10）：DrawBuilder draw_instanced（读 `PebblePlacementSsbo`，48B = [u32;12]，B_c3 §6）；回调本体已恢复（d_gap §1.3：`path_pebble_draw`）。
- 数据流：`path_pebbles_compute`（W17，5 纹理 + 3 i32 uniform `3/0x240/0x18`，dispatch_1d(0x240)）→ PebblePlacementSsbo → draw_instanced。
- shader body 不挖（GPU 后置）。

## 关键事实

- 与 path_pebbles.rs（W17）生成配对。
