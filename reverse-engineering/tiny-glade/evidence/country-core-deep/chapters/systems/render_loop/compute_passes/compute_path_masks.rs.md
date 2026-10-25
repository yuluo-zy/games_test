# systems/render_loop/compute_passes/compute_path_masks.rs

> 文件作用：路径掩码计算：`compute_path_masks`（W10，合并路径排除区/方向收缩/可视化掩码）、`blur_path_mask`/`blur_exclusion_mask`（高斯模糊路径掩码）。

函数总计 3：业务 2 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks` | 0x1409014e0 | 26 | W10 路径掩码：串证据 `Blur Path Stroke/gauss_blur_r16f.cs.hlsl` + `Directional contraction/paths/directional_contract.cs.hlsl` + `star/star.raster.hlsl`；`/paths/{combine_path_exclusions,directional_contract,path_mask_visual}.cs.hlsl`；on_texture_processed + write_texture + dispatch_2d | O | render-loop\decomp\decomp-0x1409014e0.txt（串：Blur Path Stroke、/gauss_blur_r16f.cs.hlsl、Directional contraction、/paths/directional_contract.cs.hlsl）；round2\decomp-0x1409014e0.txt；wp5 compute-passes §21 |
| `country_core::systems::render_loop::compute_passes::compute_path_masks::blur_path_mask` | 0x140903350 | 121 | 路径掩码模糊：串证据 `Blur Path Stroke/gauss_blur_r16f.cs.hlsl`（r16f 高斯）；`/gauss_blur_mask.cs.hlsl` 族（wp5 I） | O | render-loop\decomp\decomp-0x140903350.txt；wp5 compute-passes §21 |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `blur_exclusion_mask`（行 158）— 排除区掩码模糊（`/gauss_blur_mask.cs.hlsl` 族，wp5 I）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `compute_path_masks`（0x1409014e0，W10）
处理链（O，B_c2 §2 + round3）：
1. **5 资源全部先取**（A/B/C/D/E）；**运行条件 A∥B∥C∥E非空∥D**；
2. A/B/C/D **无条件清**；
3. `combine_path_exclusions`（`/paths/combine_path_exclusions.cs.hlsl`）→ `directional_contract`（`/paths/directional_contract.cs.hlsl`）→ 两次 blur → **4×`blur_path_mask`**（`Blur Path Stroke/gauss_blur_r16f.cs.hlsl`）→ `path_mask_visual`（`/paths/path_mask_visual.cs.hlsl`）；全部 512²；
4. `on_texture_processed` + write_texture + dispatch_2d。
- **round3 闭合**：资源 E = **`Events<RecalculateHeightMapCmd>`**（全 EXE 9 处命中；"非空" = events_a/events_b 两 Vec 长度）。`PathStrokeQueues` 占位名作废。
- 数据流：路径掩码 → `path_pebbles`（W17）+ 植被放置采样（garden/water spawn 避开路径）。

### `blur_path_mask` / `blur_exclusion_mask`
`Blur Path Stroke` r16f 高斯（`/gauss_blur_r16f.cs.hlsl` + `/gauss_blur_mask.cs.hlsl` 族，wp5 I）。

## 关键事实

- 路径掩码为后续路径光栅/pebbles 提供输入（W17 path_pebbles 消费）。
- **round3 闭合（2026-10-15）**：B_c2 U 项"资源 E"（TypeId `0xcda6af2f6afb130e/0x036e88a927bd70aa`，只读 +0x10/+0x30 两 u64）
  = **`Events<rasterize_terrain_stroke::RecalculateHeightMapCmd>`**（全 EXE 9 处命中全为 `Events<RecalculateHeightMapCmd>` 的
  init_state/register_event 实例，O）；"非空"判断 = events_a/events_b 两 Vec 长度（同 `Events<ShowDemoBorderOverlay>` 形制，
  B_d_cb_a §2）。`PathStrokeQueues` 占位名作废。运行条件 A∥B∥C∥E非空∥D 中的 E = 该事件队列（高度重算后重算路径掩码，I）。
  详见 `round3/RENDER-U-CLOSURE.md` §8。
