# systems/ivy/mod.rs

> 文件作用：**ivy 插件（资源/系统/事件注册）**

函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ivy::impl$0::add_ivy_systems` | 0x140944070 | 35 | 系统:注册 | O：add_ivy_systems——插件系统注册 | collision-eco\decomp\decomp-0x140944070.txt |
| `country_core::systems::ivy::impl$0::insert_ivy_resources` | 0x1409440c0 | 52 | 系统:注册 | O：insert_ivy_resources——IvyStorage/IvyLeafStorage/IvySpawnNoise/IvyDirectionProposer | collision-eco\decomp\decomp-0x1409440c0.txt |
| `country_core::systems::ivy::impl$0::add_ivy_events` | 0x140944280 | 59 | 系统:注册 | O：add_ivy_events | collision-eco\decomp\decomp-0x140944280.txt |
