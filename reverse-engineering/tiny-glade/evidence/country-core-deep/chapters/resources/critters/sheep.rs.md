# resources/critters/sheep.rs

> 文件作用：**Sheep 组件族（serde 存档+快照；业务体在 systems/critters）**

函数总计 28：业务 11 / 内联无独立符号 0 / 派生胶水 17

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::critters::sheep::impl$1::register_required_components` | 0x14089ec50 | 21 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x14089ec50.txt |
| `country_core::resources::critters::sheep::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x1413ffb60 | 29 | 派生胶水 | O：serde derive（复用依赖，不计业务） |  |
| `country_core::resources::critters::sheep::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >` | 0x1413ff8a0 | 29 | 派生胶水 | O：serde derive（复用依赖，不计业务） | collision-eco\decomp\decomp-0x1413ff8a0.txt |
| `country_core::resources::critters::sheep::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1413ff520 | 29 | 派生胶水 | O：serde derive（复用依赖，不计业务） | collision-eco\decomp\decomp-0x1413ff520.txt |
| `country_core::resources::critters::sheep::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >` | 0x1413ff400 | 29 | 派生胶水 | O：serde derive（复用依赖，不计业务） | collision-eco\decomp\decomp-0x1413ff400.txt |
| `country_core::resources::critters::sheep::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x140080440 | 29 | 派生胶水 | O：serde derive（复用依赖，不计业务） |  |
| `country_core::resources::critters::sheep::impl$7::register_required_components` | 0x14089f340 | 46 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） |  |
| `country_core::resources::critters::sheep::Sheep::being_petted` | 0x14089d4c0 | 102 | 查询/计算 | O：Sheep::being_petted=f32@+0x58>0.0 | collision-eco\decomp\decomp-0x14089d4c0.txt |
| `country_core::resources::critters::sheep::impl$8::register_required_components` | 0x14089fa30 | 106 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x14089fa30.txt |
| `country_core::resources::critters::sheep::impl$9::register_required_components` | 0x1408a0120 | 117 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x1408a0120.txt |
| `country_core::resources::critters::sheep::impl$10::register_required_components` | 0x1408a0810 | 124 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x1408a0810.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`deserialize`、`eq`、`expecting`、`fmt`、`visit_enum`、`visit_map`、`visit_seq`、`visit_str` 等 17 项
