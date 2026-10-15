# systems/render_loop/draw_passes/draw_shaders/grass_shader.rs

> 文件作用：草地绘制 shader：`draw_common`（公共绑定）、`draw_grass_shader`、`draw_grass_lod1_shader`（LOD1 草地）。

函数总计 3：业务 2 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…grass_shader::draw_grass_shader` | 0x140b4af70 | 37 | 草地绘制：DrawBuilder draw（草地实例；读 GrassBladeDataSsbo） | I（绘制结构） | render-loop\decomp\decomp-0x140b4af70.txt |
| `…grass_shader::draw_grass_lod1_shader` | 0x140b4b1d0 | 47 | 草地 LOD1 绘制：同型（远距离草地 LOD） | I | render-loop\decomp\decomp-0x140b4b1d0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `draw_common`（行 18）— 公共绑定（两 draw 共用）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `draw_grass_shader`（0x140b4af70）/`draw_grass_lod1_shader`（0x140b4b1d0）：DrawBuilder draw + 读 `GrassBladeDataSsbo`；`draw_common`（公共绑定：地形高度/光照/主题 uniform）。
- 分派链：draw_lists `grass`/`grass_lod1` 材质名 → 回调（d_gap §1.3：`draw_grass_shader`/`draw_grass_lod1_shader` 已恢复本体）→ 消费 `grass_compute`（W29）生成的 `DrawIndexedIndirectBuffer/DrawIndirectBuffer/AtomicCounterBuffer` 间接绘制参数。
- shader body 不挖（GPU 后置）。

## 关键事实

- 消费 grass.rs 生成的草地间接绘制缓冲。
