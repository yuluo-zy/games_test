# resources/stairs/structural_graph.rs

> 文件作用：**楼梯支撑碰撞体图：span↔collider 双向映射+支撑高+GraphMap（拱/托架装配的支撑结构）**

函数总计 8：业务 7 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::stairs::structural_graph::StairsStructuralGraph::clear` | 0x140869a30 | 44 | 系统:写入 | O：clear——清 8 张表 | collision-eco\decomp\decomp-0x140869a30.txt |
| `country_core::resources::stairs::structural_graph::StairsStructuralGraph::upsert_span` | 0x140869d20 | 64 | 系统:写入 | O：upsert_span——旧 span 有 collider 先解绑（断言 remove_node(prev)）→加节点/边/高/visited | collision-eco\decomp\decomp-0x140869d20.txt |
| `country_core::resources::stairs::structural_graph::StairsStructuralGraph::upsert_support_source_collider` | 0x140869f90 | 81 | 系统:写入 | O：upsert_support_source_collider——只加节点/高/visited（Ghidra varnode 失败，合同语义） | collision-eco\decomp\decomp-0x140869f90.txt |
| `country_core::resources::stairs::structural_graph::StairsStructuralGraph::add_edge_by_span` | 0x140869ff0 | 87 | 系统:写入 | O：add_edge_by_span——两 span collider 间加边（已存在重写 0） | collision-eco\decomp\decomp-0x140869ff0.txt |
| `country_core::resources::stairs::structural_graph::StairsStructuralGraph::add_edge_by_collider` | 0x14086a260 | 112 | 系统:写入 | O：add_edge_by_collider——直接加边+IndexMap 写 0 | collision-eco\decomp\decomp-0x14086a260.txt |
| `country_core::resources::stairs::structural_graph::StairsStructuralGraph::remove_node_by_collider` | 0x14086a320 | 149 | 系统:写入 | O：remove_node_by_collider——删邻边+remove_node+逆映射清理（断言 remove_node(collider)/span 移除） | collision-eco\decomp\decomp-0x14086a320.txt |
| `country_core::resources::stairs::structural_graph::StairsStructuralGraph::disconnect_node_by_collider` | 0x14086a4c0 | 166 | 系统:写入 | O：disconnect_node_by_collider——枚举边移除（保留节点） | collision-eco\decomp\decomp-0x14086a4c0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项

## 实现方式（算法级）

完整算法见 `collision-eco/ALGORITHMS.md` §3.4。要点：

### upsert_span（0x140869d20）8 表接线
1. **旧关联解绑**（span→collider map @+0xc0 命中且判别 != 0x11）：遍历旧 collider 邻边（FUN_14093b1c0 迭代器，判别 0x11 终止）逐个插入 connections 集 @+0x150（FUN_141265a10）；从 collider→span map @+0x90 移除旧 span（FUN_140b77fd0）；`graph.remove_node(prev)` 断言失败 panic `"assertion failed: self.graph.remove_node(prev)"`（0x142a96b48）；从 @+0xf0 表移除旧条目（FUN_140b787d0）。
2. **新 collider 挂接**：graph 节点确保（FUN_14093afc0）→ collider→span @+0x90 插入（FUN_14152c190）→ span→collider @+0xc0 插入（FUN_14152ba60）→ span→height @+0xf0 插入（FUN_1412668d0，param_4）→ connections @+0x150 插入（FUN_141265a10）。

表分布（本轮复核）：+0x90 collider→span、+0xb0 collider map hash、+0xc0 span→collider（= `span_to_collider_handle` 字段，断言串 0x142a96c50）、+0xe0 span hash、+0xf0 span→height、+0x110 hash、+0x150 connections。
