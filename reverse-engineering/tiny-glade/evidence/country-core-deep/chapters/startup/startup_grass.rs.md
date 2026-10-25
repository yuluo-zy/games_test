# startup/startup_grass.rs

> 文件作用：**草地资源载入：grass blade 网格 + grass shader。**

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_grass::startup_grass` | 0x140a21620 | 16 | `startup_grass`：载入 `grass blade`/`grass_blade` 网格 + `grass_object_push`/`grass` shader（`/grass_object_push.raster.hlsl`、`/grass.raster.hlsl`）。 | **O** | decomp-0x140a21620.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 1 项：closure$0 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **startup_grass 0x140a21620（O）**：grass blade/grass_blade 网格（load_mesh_into_library）
  + 3x shader（grass_object_push、grass，'/grass_object_push.raster.hlsl'、'/grass.raster.hlsl'）
  + 2 个 spawn 记录（FUN_1412dc470）+ 特殊注册 FUN_1412fff70（grass 实例化后端接线）。
