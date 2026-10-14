# render/pass_builder/draw_list.rs

> 文件作用：DrawListItem / EntityDrawParams：CPU draw-list 项（0x80B 元组 = 0x50B params 首字段 + vao + shader + i32 排序键 + 模板参考 + 回调 fn；无 PDB 独立过程，全部内联进宿主 pass）。

函数总计 1：业务 0 / 内联无独立符号 1 / 派生胶水 0

## 内联/无独立符号函数

- `country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet`（行 32）— 内联 22 宿主：DrawListItem（0x70B，首字段 EntityDrawParams 0x50B）+ 回调 fn@+0x70（元组 0x80B，driftsort 稳定排序）；元素布局 O（0x1409bc8f0..0x1409bc97c 逐字段），to_draw_packet 具体形态 I—— O/I

## 证据注记（tg_rn1）
- 元素 = `(DrawListItem, fn(DrawBuilder, &EntityDrawParams, &mut World))` 元组 0x80B；回调第二实参即元素指针（`[rdx+8]` = Entity）。
- 字段表（O，0x1409bc8f0..0x1409bc97c）：+0 Option<DrawTopology>（tag u32，None=2）、+8 Entity、+0x10 shader 名 &str、+0x20 Transform 0x30B、+0x50 Handle<Vao>、+0x58 Handle<RasterShaderProgram>、+0x60 排序键 i32、+0x64/0x65 Option<u8> 模板、+0x70 回调。
- `DrawTopology` 变体名 U（tag 0/1 + f32 载荷；`Alt` = tag 1 为 I 命名）；`sort_draw_list` = `sort_by_key(i32)` 稳定（driftsort）。
- 纠错（B_d_gap §2）：旧 `DrawListItem{sort_key:u64, shader_id}` 与 `to_draw_packet` 的 `[0,0]` 头按机器码改为 `[1, vao]`；`EntityDrawParams` 是元素**首字段**。
