# systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs

> 文件作用：活跃地形笔触高亮：`highlight_active_terrain_stroke`（W8，当前地形笔触高亮掩码生成 + 高斯模糊）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::highlight_active_terrain_stroke` | 0x140a58450 | 20 | W8 地形笔触高亮：串证据 `Create a highlighted terrain stroke mask/terrain_editing/calc_highlight_hill_mask.cs.hlsl` + `Blur Terrain Highlight Mask/gauss_blur_r16f.cs.hlsl` + blur temp；EventCursor::read + last + uniform + write_texture + dispatch；`AddTerrainHighlightCmd` | O | render-loop\decomp\decomp-0x140a58450.txt（串：Create a highlighted terrain stroke mask、/terrain_editing/calc_highlight_hill_mask.cs.hlsl、Blur Terrain Highlight Mask、/gauss_blur_r16f.cs.hlsl）；terrain-water\decomp\decomp-0x140a58450.txt；wp5 compute-passes §21 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `highlight_active_terrain_stroke`（0x140a58450，W8）
处理链（O，B_c3 §2）：
1. **模式 5/10 才取事件**（AddTerrainHighlightCmd i32 @+8）：**新缓冲最后一项优先，否则旧缓冲**；
2. **抑制字节 @+0xf9 强制 -1**；
3. `idx != -1` 时两遍 `gauss_blur_r16f`（水平 → 垂直，sigma=1.0，"blur temp" 256²/0x4c）；
4. 事件类型（B_c3 §6）：`AddTerrainHighlightCmd(pub i32)`。
- 数据流：地形笔触高亮掩码（`/terrain_editing/calc_highlight_hill_mask.cs.hlsl`）→ 编辑器叠加显示。
- 帧序 W8（W7 foliage exclusion 后）。

## 关键事实

- 输出高亮掩码 + r16f 高斯模糊（blur temp）。
