# systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs

> 文件作用：砖绘制：`indirect_draw_bricks<0>` / `<1>`（两单态，间接绘制砖块实例）。

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…draw_shaders::draw_bricks::indirect_draw_bricks<0>` | 0x1408a53d0 | 19 | 间接绘制砖块（变体 0）：DrawBuilder 间接绘制（`DrawBuilder::draw_indexed_indirect`）+ 读砖块 SSBO；串仅 unwrap 错误 | I（间接绘制结构） | render-loop\decomp\decomp-0x1408a53d0.txt |
| `…draw_shaders::draw_bricks::indirect_draw_bricks<1>` | 0x1408a4ca0 | 19 | 间接绘制砖块（变体 1，如材质变体/季节变体）：同上 | I | render-loop\decomp\decomp-0x1408a4ca0.txt |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `indirect_draw_bricks<0>`（0x1408a53d0）/`<1>`（0x1408a4ca0）→ draw_lists 回调 `bricks_lod0_cb`/`bricks_lod1_cb`（d_cb_a §3 #2/#3，机器码 O）：
  - **代理 3 字节 → gate → 先 discard 再读资源**：LOD0 `mode>=2 || b1==1`、LOD1 `(bit, sel)` 判定都在 `AssetSsboLibrary` 取值**之前**（否则原版 discard 变 panic）；
  - 常规路径读 `WallHighlight.ty`（byte@+0x10）与四个擦除高亮资源（`read_erase_highlight` = bind_brick_erase_highlight 输入）；
  - `DrawBuilder::draw_indexed_indirect`（`SparseInstanceBuffer<BrickTransformSsbo>` 内嵌缓冲 @+0x68 间接参数，B_i_api §1.5 纠错）。
- 两单态对应 nani 砖材质变体；shader body 不挖（GPU 后置）。

## 关键事实

- 两单态对应 nani 砖材质变体（wp5 draw-passes §1 材质族）。
- shader body 不挖（GPU 后置）。
