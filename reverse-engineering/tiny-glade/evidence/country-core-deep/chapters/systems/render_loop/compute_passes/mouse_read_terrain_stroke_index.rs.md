# systems/render_loop/compute_passes/mouse_read_terrain_stroke_index.rs

> 文件作用：鼠标地形笔触索引回读：`mouse_read_terrain_stroke_index`（W6，读鼠标下地形笔触索引 SSBO）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::mouse_read_terrain_stroke_index` | 0x1409768b0 | 39 | W6 读地形笔触索引：串证据 `Read terrain stroke/read_terrain_stroke.cs.hlsl` + `/read_terrain_stroke.cs.hlsl`；CursorTerrainRaycast::position + read_texture + push_constants + dispatch；写 `MouseTerrainRaycastSsbo` | O | render-loop\decomp\decomp-0x1409768b0.txt（串：Read terrain stroke、/read_terrain_stroke.cs.hlsl）；terrain-water\decomp\decomp-0x1409768b0.txt；wp5 compute-passes §21 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `mouse_read_terrain_stroke_index`（0x1409768b0，W6）
处理链（O，B_c3 §2）：
1. `CursorTerrainRaycast::position`（tag bit0 为 Some，Vec3@+4）→ **tag&1 选 x/z 位模式**；
2. **8 字节 push constants**；
3. `dispatch_1d(1)`；写 `MouseTerrainRaycastSsbo`（cpu-mapped，B_i_infra §4）。
- 数据流：鼠标下地形笔触索引 → 编辑器交互（绘制/擦除高亮）。

## 关键事实

- 结构体：`MouseTerrainRaycastSsbo`（U 布局）；CursorTerrainRaycast（terrain-water 域）。
