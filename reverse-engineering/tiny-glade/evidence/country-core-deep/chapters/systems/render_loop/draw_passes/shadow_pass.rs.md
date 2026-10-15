# systems/render_loop/draw_passes/shadow_pass.rs

> 文件作用：阴影图光栅：`shadow_pass`（W23，太阳阴影 atlas 层光栅）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass` | 0x1409cc270 | 20 | W23 阴影 pass：puffin scope + utils::get_res_mut + RasterPass::new + DepthAttachment::layer（shadow atlas 层）+ CommonDrawParams::new + 遍历 DrawListItem + to_draw_packet + RasterPipelineOpts::set_depth_compare_mode；shader `_mei_shadow.raster.*`（I：无同名文件，阴影图光栅） | O | render-loop\decomp\decomp-0x1409cc270.txt；wp5 draw-passes §3 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `shadow_pass`（0x1409cc270）—— 3 级联阴影 atlas（d_main §1.6 完整解码）
处理链（O）：
1. **3 个级联各建一个 pass**（循环）：深度 tag0、`layer = cascade`（shadow atlas 层）、清除 1.0；无颜色附件；
2. `DrawBuilder::new(variant=1)`；
3. `bind_common(params, item)` 后 **`state.byte0 &= 0xcf`**；
4. 读列表 0（SHADOW）；逐项 to_draw_packet + `RasterPipelineOpts::set_depth_compare_mode`；
5. `ItemMode::Shadow` 下 `bind_common` 之后用 `CommonDrawFlag::shadow_cascade(c)=c+2`（B_i_api §1.1/§2：bool uniform = `flag==0`，u32 整字节 2..=4）。
- 着色器 `_mei_shadow.raster.*`（I：无同名文件，阴影图光栅）。
- 数据流：`DirectionalLight` look-at 基（directional_light 章）→ 3 个级联视图矩阵 → frame_constants UBO → shadow atlas → main_pass/晚延迟的 shadow 采样。
- 帧序 W23（main_pass 前，W25）。

## 关键事实

- 阴影 atlas 分层（DepthAttachment::layer）。
- 帧序 W23（frame-wiring，主 pass 前）。
