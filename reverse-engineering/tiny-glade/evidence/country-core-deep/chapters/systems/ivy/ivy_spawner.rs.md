# systems/ivy/ivy_spawner.rs

> 文件作用：**常春藤生成：随机根点/光标旁根点**

函数总计 5：业务 1 / 内联无独立符号 3 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ivy::ivy_spawner::ivy_spawner` | 0x14094cf90 | 31 | 系统:生成 | O：ivy_spawner——try_random_ivy_root/try_ivy_root_next_to_cursor/is_point_inside_wall_segment | collision-eco\decomp\decomp-0x14094cf90.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::ivy::ivy_spawner::try_random_ivy_root`（行 212）— I：随机根点尝试（RNG 采样）
- `country_core::systems::ivy::ivy_spawner::try_ivy_root_next_to_cursor`（行 245）— I：光标旁根点尝试（ivy_spawner 内联）
- `country_core::systems::ivy::ivy_spawner::is_point_inside_wall_segment`（行 294）— 谓词/校验

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）

- ivy_spawner（0x14094cf90，38KB）：随机根点尝试（try_random_ivy_root）/ 光标旁根点（try_ivy_root_next_to_cursor）/ 点是否在墙段内（is_point_inside_wall_segment）三内联函数组成；callee 结构证实调 `closest_segment_index`（0x14094c800）选墙 + `func_0x000140c9cdc0` ×6（墙段几何采样）+ RNG/panic 路径；生成后经 `IvyStorage::spawn_ivy`（0x1408696f0，next_ivy_id+1）入存储。
