# startup/load_shaders.rs

> 文件作用：**shader 载入：UI shader 与主渲染 shader 入库（经 startup/mod.rs 的 load_shader_into_library）。**

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::load_shaders::load_ui_shaders` | 0x140aaaf80 | 13 | `load_ui_shaders`：载入 6 个 UI shader — ui_2d, ui_2d_ws, ui_2d_color_button, text, mei_shadow, custom_cursor（`.raster.hlsl`）。 | **O** | decomp-0x140aaaf80.txt |
| `country_core::startup::load_shaders::load_shaders` | 0x140aab0c0 | 70 | `load_shaders`：载入 ~13 个渲染 shader — meadow_flower, vertex_color_shader, dashed_line, timer_circle, final_post, egui, solid_vertex_color{, _albedo_map,_wooden_gate}, flag_{roof_cloth,wall_cloth,small}, stairs_foliage_exclusion。 | **O** | decomp-0x140aab0c0.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **load_ui_shaders 0x140aaaf80（O）**：6 个 UI shader 依次经 load_shader_into_library<&str>（FUN_140b3fe40）：
  ui_2d、ui_2d_ws、ui_2d_color_button、text、mei_shadow、custom_cursor（`.raster.hlsl`）。
- **load_shaders 0x140aab0c0（O）**：~13 个渲染 shader（串表 0x142add604 区证实 10 个路径+名对）：
  meadow_flower、vertex_color_shader、dashed_line、timer_circle、final_post、egui、
  solid_vertex_color{,_albedo_map,_wooden_gate}、flag_{roof_cloth,wall_cloth,small}、stairs_foliage_exclusion；
  每对 = '{/…}.raster.hlsl' 路径 + 名称键。
