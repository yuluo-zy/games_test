# RESTORE — roofs 视觉核心（R6，交接登记）

实现者: restorer-interaction
日期: 2026-10-16

## 本批实现

`resources/roofs/roof_visual_shape.rs` 新建：
- `RoofBoundsDims::{as_circle, as_rectangle}`（O 0x140a88200/0x140a88240，hex 手解：
  Circle bit0==0 / Rectangle bit0==1；误调用 panic）。

## 交接登记（上下文将尽，逐项待续）

- `Roof::get_height_01_unclamped_from_ws`（0x1408e4790）：`(pos.y + noise(x,z)·0.12? −
  floor@+0x4c) / (height_cap − floor)`；噪声 = `roof_noise::noise`（已建）。反向映射 +
  smoothstep cap 待实现。
- `Roof::get_roof_shape_from_pos_ws_clamped`（0x1408e4750）：h01 clamp[0,1] → get_roof_shape。
- `Roof::get_roof_shape`（0x1408e4920 = reconstruction/roof/surface.rs，3,004 案例）：
  q=1.5p+(1-p)0.5、t^(1/q) 混合屋檐/屋脊；矩→{frame,dims} 28B、圆→{frame,radius} 20B。
- `get_roof_expanded_top_and_bottom`（0x140a880c0）。
- `roof_ridge.rs`（屋脊创建/参数/更新，7 业务）：未实现，登记待续。
- roof_tiles/dormer_tiles/roof_buffers：Bevy 胶水（HIGHLIGHTS #9 不计业务），登记即可。

## 编译状态

`cargo check -p country-core --lib` 0 error。

## 留给 reviewer 的核对点

1. RoofBoundsDims Circle/Rectangle 判别（bit0）与 panic 语义。
2. 交接项（get_height_01/get_roof_shape/roof_ridge）登记与 roofs 主链台账一致。
