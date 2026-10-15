# startup/startup_sheep.rs

> 文件作用：**羊资源载入：羊动画网格 + 附属物（heart/umbrella/floatie）+ shader + 羊叫音效。**

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_sheep::startup_sheep` | 0x140a093e0 | 27 | `startup_sheep`：载入 sheep_animation/{1,delighted}, sheep_heart, umbrella, floatie 网格 + `sheep_shader`(solid_vertex_color) + `event:/SFX/Sheep/Baah` + Pitch。47.8KB。 | **O** | decomp-0x140a093e0.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 1 项：closure$0 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **startup_sheep 0x140a093e0（47.8KB，O）**：
  1. 5x 文件式网格（FUN_140b0eeb0）：'assets/meshes/sheep_animation/1.json'（0x142ac6a23 区）、
     sheep_animation/delighted.json、sheep_heart、umbrella、floatie
  2. 4x 名式网格（FUN_140b1aca0）：sheep_mesh（0x142ac6a18）等
  3. 5x 属性提取（FUN_1405b1a70：'Vertex_Position'(15, 0x142ac6553) / 'Vertex_Normal'(13, 0x142ac6562)）
     — 网格后处理（动画顶点数据）
  4. 注册链 FUN_14130efe0/FUN_1412e23e0/FUN_1416f0ea0（动画网格注册）
  5. 音频：FMOD `event:/SFX/Sheep/Baah` + 参数 'Pitch'（0x142ac6c48）
  6. 6x unwrap panic（0x142ac6360 区，行 43 族）
