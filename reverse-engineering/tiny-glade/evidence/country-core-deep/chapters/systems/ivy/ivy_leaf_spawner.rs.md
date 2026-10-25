# systems/ivy/ivy_leaf_spawner.rs

> 文件作用：**叶生成：消费段完成消息沿末段放叶**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ivy::ivy_leaf_spawner::ivy_leaf_spawner` | 0x140996080 | 0 | 系统:生成 | O：ivy_leaf_spawner——消费 IvySegmentDoneGrowingMsg（"Received IvySegmentDoneGrowingMsg with ivy id "）沿末段放叶 | collision-eco\decomp\decomp-0x140996080.txt |

## 实现方式（算法级）

- ivy_leaf_spawner（0x140996080）：消费 `IvySegmentDoneGrowingMsg`（段生长完成事件；未知 ivy id → panic "Received IvySegmentDoneGrowingMsg with ivy id " 0x142abaeb0）→ 沿末段取样放叶：`IvyLeafStorage::add`（0x1408fe280，104B IvyLeaf push 到对应 ivy）+ 共享 ivy helper（FUN_140992af0 ×2）+ puffin scope。
