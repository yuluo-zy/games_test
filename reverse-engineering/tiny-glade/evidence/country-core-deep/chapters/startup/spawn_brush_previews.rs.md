# startup/spawn_brush_previews.rs

> 文件作用：**画笔预览实体生成：载入 ~13 个 brush 预览网格并 spawn 预览实体（鼠标光标笔刷）。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::spawn_brush_previews::spawn_brush_previews` | 0x140924b40 | 23 | `spawn_brush_previews`：载入 brush_arrow, brush_circle{, _cross,_scribble}, brush_4_arrows, brush_2_arrows, brush_5_replay, brush_arrow_flat{, _single_up,_single_foward,_outline}, flat_arrow, corner_arrow_{outline,switch_ridge_dir}, move_shape{, _outline}, hand(temp_sphere), dashed_lines(snapping_line/circle) 等（13× load_mesh_into_library）+ spawn 预览实体（17×）。 | **O** | decomp-0x140924b40.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **spawn_brush_previews 0x140924b40（51KB，O）**：**24 次网格载入（13x 名式 FUN_140b1aca0 +
  11x 文件式 FUN_140b0eeb0）**（章节旧述 ~13 为欠计）+ **17 个 spawn 记录**（17x
  FUN_1428a9a60/FUN_141564d30/FUN_140d146a0 三件套 = 记录表 push）+ 13x unwrap panic。
  名族（0x142aaed1c 区等）：brush_arrow、brush_circle{,_cross,_scribble}、brush_4_arrows、
  brush_2_arrows、brush_5_replay、brush_arrow_flat{,_single_up,_single_foward,_outline}、
  flat_arrow{,_outline}、corner_arrow{,_outline,_switch_ridge_dir}、move_shape{,_outline}、
  vertex_color_shader（0x142aaece6 区）。
- 消费：spawn 记录 → 渲染/backend 预览实体（Ui2dImage::new 组件，ui-input 域线索）。
