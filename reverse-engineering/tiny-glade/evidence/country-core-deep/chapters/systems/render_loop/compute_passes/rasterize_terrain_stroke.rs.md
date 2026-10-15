# systems/render_loop/compute_passes/rasterize_terrain_stroke.rs

> 文件作用：地形笔触光栅：`rasterize_terrain_stroke`（W3，把 CPU 地形笔触光栅到地形掩码/高度 + 高斯模糊 + 高度图重算）。

函数总计 5：业务 1 / 内联无独立符号 1 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke` | 0x1408c7120 | 65 | W3 地形笔触光栅：串证据 `Blur Terrain Stroke` + `Terrain Derivative/terrain_derivative.cs.hlsl` + blur temp + `assertion`（多段）；Events::is_empty 守卫 + DynamicConstantsChunk::with_data + DynamicConstants::push_slice + uniform + write_texture + dispatch_2d + bind_terrain_heightmap；GaussBlur（`/gauss_blur.cs.hlsl:cs_main`）+ RecalculateHeightMapCmd（U） | O | render-loop\decomp\decomp-0x1408c7120.txt（串：Blur Terrain Stroke、Terrain Derivative、/terrain_derivative.cs.hlsl、blur temp）；terrain-water\decomp\decomp-0x1408c7120.txt；wp5 compute-passes §21 |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `impl$0::as_shader_def_value`（行 43）— shader def 值转换（terrain stroke 类型 → def）

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`hash` 等 3 项

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `rasterize_terrain_stroke`（0x1408c7120 + 0x1415fb550 closure，W3）
处理链（O，B_c1 §2）：
1. 早退条件（事件/笔画表空）；`width > 0.01`；点数 **0x400 上限**；>0x3fff 一次性错误日志 `"Ran out of live terrain strokes"`；
2. `StrokeInfo`（16B = `(id<<10)|(count&0x3ff)` + `pack_2x16f_uint`）+ 断言文案；
3. **5 个 pass**：上传 → `GaussBlur`（`/gauss_blur.cs.hlsl:cs_main`，`GaussBlur{fmt}`，PERM_fmt/PERM_ty）→ `Terrain Derivative`（`/terrain_derivative.cs.hlsl`）→ blur → 高度图重算（RecalculateHeightMapCmd 事件）；
4. 栅格化 pass 的 `dispatch_with` 闭包（closure_env$1 0x1415fb550）：每 tracker `track` + `dispatch_3d(512,512,..)`。
- 数据流：地形笔触掩码/高度 → W4 displace_rocky_terrain + 地形网格跟随；`compute_path_masks`（W10）的 E 资源 = `Events<RecalculateHeightMapCmd>`（round3 闭合）。
- 帧序 W3（W2 water_rasterize 后）。

## 关键事实

- 输出地形高度/掩码（供 W4 displace_rocky_terrain、地形网格跟随等）。
- 帧序 W3（frame-wiring）。
