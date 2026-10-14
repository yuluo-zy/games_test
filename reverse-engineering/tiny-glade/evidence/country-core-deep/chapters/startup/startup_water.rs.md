# startup/startup_water.rs

> 文件作用：**水面资源载入：water_plane 网格 + 水 shader 族 + 睡莲/芦苇/水石网格。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_water::startup_water` | 0x1408f1a20 | 0 | `startup_water`：载入 water_plane, lilypad, lilypad_flower, reed{, _autumn}, water_stones, fallen_leaves, water_grass_winter 网格；shader — water_plane, ice_plane, water_plane_refraction_copy, water_snow_plane, lilypad, lilypad_flower, reed, water_x_paths。 | **O** | decomp-0x1408f1a20.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。
