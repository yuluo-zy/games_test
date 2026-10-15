# systems/terrain/mod.rs

> 文件作用：**地形系统模块注册：add_terrain_systems/insert_terrain_resources/add_terrain_events 三个注册函数 + 子模块声明 + 跨模块 ECS 类型门面。**

函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$0::add_terrain_systems` | 0x1409455f0 | 37 | 构造/注册/生成 | O：系统注册（Bevy Plugin 接线；register_required_components 胶水 + add_systems 族） | terrain-water\decomp\decomp-0x1409455f0.txt |
| `impl$0::insert_terrain_resources` | 0x1409457c0 | 56 | 构造/注册/生成 | O：地形资源插入（TerrainHeightsData 双实例/网格等 init） | terrain-water\decomp\decomp-0x1409457c0.txt |
| `impl$0::add_terrain_events` | 0x140945a20 | 65 | 构造/注册/生成 | O：地形事件注册（Bevy 事件声明；注册胶水） | terrain-water\decomp\decomp-0x140945a20.txt |

## 实现方式（算法级）

- `add_terrain_systems`（0x9455f0）：Bevy Plugin 系统注册（register_required_components 胶水 + add_systems 族）。
- `insert_terrain_resources`（0x9457c0）：TerrainHeightsData 双实例（权威/视觉）+ 网格等资源插入。
- `add_terrain_events`（0x945a20）：地形事件声明/注册。
