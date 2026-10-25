# REVIEW — systems/roofs(R6 部分终验 → RoofBoundsDims 闭合 + 交接登记)

审查员: reviewer-interaction | 日期: 2026-10-16
基线: restorer RESTORE.md(reports/interaction/roofs/,roof_visual_shape.rs 新建);
`cargo check --lib` exit 0(独立复验,6 警告)。

## 核对

### ① RoofBoundsDims::{as_circle, as_rectangle} — PASS
- `RoofBoundsDims` 判别:Circle bit0==0、Rectangle bit0==1(Rust enum Circle=0/Rectangle=1
  对齐)✓。
- `as_circle`(0x140a88200):Circle → {center Vec2@+4, radius f32@+8};Rectangle → panic
  (误调用语义)✓。
- `as_rectangle`(0x140a88240):Rectangle → 16 字节矩形参数 [f32;4];Circle → panic ✓。
- hex 手解 0x140a88200/0x140a88240 与实现一致 ✓。

### ② 交接登记诚实 — PASS(逐项与 roofs 主链台账一致)
- `get_height_01_unclamped_from_ws`(0x1408e4790):(pos.y+noise·0.12?−floor@+0x4c)/
  (height_cap−floor),噪声=roof_noise::noise(已建);反向映射+smoothstep cap 待实现。
- `get_roof_shape_from_pos_ws_clamped`(0x1408e4750)、`get_roof_shape`(0x1408e4920=
  surface.rs 3004 案例,q=1.5p+(1-p)0.5、t^(1/q) 混合)、`get_roof_expanded_top_and_bottom`
  (0x140a880c0)——登记待续。
- `roof_ridge.rs`(屋脊创建/参数/更新,7 业务)未实现,登记待续 ✓。
- roof_tiles/dormer_tiles/roof_buffers:Bevy 胶水(HIGHLIGHTS #9 不计业务)登记 ✓。
全部与 roofs 主链台账一致,无冒充。

## 结论与 DISPATCH 建议

- R6 部分复验通过:**RoofBoundsDims 闭合**;剩余 roof 视觉方法(get_height_01/get_roof_shape/
  expanded_top_bottom)与 roof_ridge 7 业务登记交接,归新专项续派。
- 建议:R6 → 闭合(部分:RoofBoundsDims);交接项登记。编译 exit 0 已验。
