# systems/ivy/mod.rs

> 文件作用：**ivy 插件（资源/系统/事件注册）**

函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ivy::impl$0::add_ivy_systems` | 0x140944070 | 35 | 系统:注册 | O：add_ivy_systems——插件系统注册 | collision-eco\decomp\decomp-0x140944070.txt |
| `country_core::systems::ivy::impl$0::insert_ivy_resources` | 0x1409440c0 | 52 | 系统:注册 | O：insert_ivy_resources——IvyStorage/IvyLeafStorage/IvySpawnNoise/IvyDirectionProposer | collision-eco\decomp\decomp-0x1409440c0.txt |
| `country_core::systems::ivy::impl$0::add_ivy_events` | 0x140944280 | 59 | 系统:注册 | O：add_ivy_events | collision-eco\decomp\decomp-0x140944280.txt |

## 实现方式（算法级）

- add_ivy_systems（0x140944070）：**22 个系统**（`FUN_141664d80(..., 0x16)` + `FUN_141450fa0` 建 SystemSet 后 `FUN_140b67280` add_systems）——grower/spawner/pruner/leaf_spawner/动画/GPU 上传族。调度序：grower 相状态机 → pruner → leaf_spawner → send_to_gpu。
- insert_ivy_resources（0x1409440c0，21KB）：构造并插入 **IvyStorage**（FUN_141532130）、**IvyLeafStorage**（FUN_141533920）、**IvySpawnNoise**（FUN_14152eee0，FastNoise 参数：频率 0.3（0x3e99999a）、lacunarity 2.0（0x40000000），hashbrown 表键 `0x742800bfa8ac239d`）、**IvyDirectionProposer**。`_UNK_142a96720/28/30/38` = IvySpawnNoise 初始串字段（空）。
- add_ivy_events（0x140944280）：注册 3 个事件类型（bevy Events TypeId 对）：
  - `0x43c6e997585378f3 / 0xe7c5a5c8430a7743` → `IvySegmentDoneGrowingMsg`（leaf_spawner 消费）
  - `0xd17f425c3a59146b / 0x5becdae426f61a91`（IvySegmentDoneGrowing 事件族）
  - `0x9a304ccd0b599630 / 0xf4c5df6884242f85`
