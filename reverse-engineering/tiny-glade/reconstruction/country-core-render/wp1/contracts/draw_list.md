# draw_list.rs 业务契约

标注：**O**=反编译直接支持；**I**=调用链/类型关系组合；**U**=未恢复。

## 1. `DrawListItem`（I，无独立编译过程）

`DrawListItem::to_draw_packet`（inline 22 宿主）：把 CPU draw-list 项转成 `DrawPacket`。

类型关系证据（structures.json + enriched）：`core::slice::sort::stable::driftsort_main<
  tuple<DrawListItem, fn(DrawBuilder, &EntityDrawParams, &mut World)> >`（module 590）——
draw-list 项携带实体绘制回调（绘制时构造 `DrawBuilder` 并执行回调产出绘制包）。

字段：排序键（I：按渲染顺序稳定排序，driftsort）+ 绘制回调。`EntityDrawParams` 归属 WP4 渲染列表（引用不定义）。

## 2. 与 CPU 帧的接缝（O，INDEX.md R2）

`tiny_glade::render_loop` 的 `DrawLists::new/build/sort`（`country_core::systems::render_loop::draw_passes`）构造 CPU draw-list 项；`draw_stream::DrawStream` 聚合这些项生成 `DrawPacket`，供 `RasterPass`/主图消费。
