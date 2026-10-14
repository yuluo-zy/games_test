# resources/critters/sheep/control.rs

> 文件作用：**SheepControlSnapshot/SheepSnapshot（serde 快照）**

函数总计 15：业务 3 / 内联无独立符号 1 / 派生胶水 11

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::critters::sheep::control::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1416f9eb0 | 11 | 派生胶水 | O：serde derive（复用依赖，不计业务） |  |
| `country_core::resources::critters::sheep::control::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x1416f9d30 | 11 | 派生胶水 | O：serde derive（复用依赖，不计业务） |  |
| `country_core::resources::critters::sheep::control::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x1405c9c50 | 11 | 派生胶水 | O：serde derive（复用依赖，不计业务） |  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::critters::sheep::control::capture_sheep_snapshot`（行 29）— I：绵羊快照捕获（serde 保存/回放路径）

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`deserialize`、`eq`、`expecting`、`visit_map`、`visit_seq`、`visit_str` 等 11 项
