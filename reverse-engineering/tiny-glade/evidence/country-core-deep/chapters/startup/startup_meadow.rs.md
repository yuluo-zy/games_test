# startup/startup_meadow.rs

> 文件作用：**草地/meadow 植物载入：plants.json + 低模花 + meadow_plant_shader。**

函数总计 8：业务 1 / 内联无独立符号 0 / 派生胶水 7

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_meadow::init_meadow` | 0x1408639f0 | 28 | `init_meadow`：deserialize `plants.json`（`deserialize plants` panic），载入 meadow 植物网格 + `meadow_plant_shader`。 | **O** | decomp-0x1408639f0.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 7 项：closure$0, deserialize, expecting, visit_map, visit_seq, visit_str 等（derive 直接复用，不计业务）。
