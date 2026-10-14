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
