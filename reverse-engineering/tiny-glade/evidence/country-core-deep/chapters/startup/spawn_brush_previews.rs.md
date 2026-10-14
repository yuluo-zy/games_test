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
