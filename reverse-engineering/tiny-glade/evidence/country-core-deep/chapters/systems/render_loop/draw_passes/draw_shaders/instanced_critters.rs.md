# systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs

> 文件作用：小动物实例绘制：蝴蝶/萤火虫/萤火虫辉光/鸭/鹪鹩五个 draw 函数（draw_lists 分派目标）。

函数总计 5：业务 5 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…instanced_critters::draw_instanced_butterfly_shader` | 0x1415547e0 | 19 | 蝴蝶实例绘制：draw_lists（0x1409ba540）分派目标（0x1415547e0）；DrawBuilder draw_instanced | O（分派目标） | render-loop\decomp\decomp-0x1415547e0.txt；collision-eco\decomp\decomp-0x1415547e0.txt；wp5 draw-passes §1 |
| `…draw_instanced_firefly_shader` | 0x1415549b0 | 34 | 萤火虫实例绘制：分派目标（firefly 材质名） | O | render-loop\decomp\decomp-0x1415549b0.txt；collision-eco\decomp\decomp-0x1415549b0.txt；wp5 §1 |
| `…draw_instanced_firefly_glow_shader` | 0x141554cf0 | 55 | 萤火虫辉光实例绘制：分派目标（firefly_glow） | O | render-loop\decomp\decomp-0x141554cf0.txt；collision-eco\decomp\decomp-0x141554cf0.txt；wp5 §1 |
| `…draw_duck_shader` | 0x141555080 | 80 | 鸭子实例绘制：分派目标（nani_duck，build_draw_lists 0x1409ba540 按 "duck" 名分派） | O | render-loop\decomp\decomp-0x141555080.txt；collision-eco\decomp\decomp-0x141555080.txt；wp5 §1 |
| `…draw_wren_shader` | 0x141555ec0 | 124 | 鹪鹩实例绘制：分派目标（nani_wren） | O | render-loop\decomp\decomp-0x141555ec0.txt；collision-eco\decomp\decomp-0x141555ec0.txt；wp5 §1 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- 五者均为 build_draw_lists（0x1409ba540）材质名分派目标（O 抽查：draw_instanced_butterfly_shader callee=bind_texture_to_draw_builder×2 + read_storage_buffer<draw 资源> + draw_instanced_w_gl_draw_flag；draw_wren_shader callee=nani_draw(0x140904ae0)，nani_wren 目标成立）。
- 回调（d_cb_a/d_cb_b）：`duck_cb`（GladeSettings、CritterControl、PrevTransform+Duck+DuckState，B_d_cb_b §3.3 纠错：鸭第一组件是 PrevTransform 非 Transform）；`wren_cb`（nani_draw(Wren=0x11)）；`firefly_cb`/`firefly_glow_cb`（GladeSettings/CritterControl 粒子数资源）。
- 数据流：critters 域 `MappedStorageBuffer`（Butterfly 0x300/Firefly 0x600/FireflyVisibility 0x180，B_i_infra §4）→ `visibility_queries`（W31 Firefly vis query）→ 实例缓冲 → draw_instanced。
- shader body 不挖（GPU 后置）。

## 关键事实

- 五者均为 build_draw_lists（0x1409ba540）的材质名分派目标（串证据：duck/wren/butterfly/firefly/ivy 等片段）。
- shader body 不挖（GPU 后置）。
