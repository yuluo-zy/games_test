# resources/glade_color_customization.rs

> 文件作用：**谷地自定义配色（serde 存档；默认字节 O）。**

函数总计 16：业务 3 / 内联无独立符号 0 / 派生胶水 13

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::glade_color_customization::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error> | 0x14085f550 | 9 | serde 反序列化派生（GladeColorCustomization visit_str） | O（serde 派生） |  |
| country_core::resources::glade_color_customization::_::impl$0::serialize<serde_json::value::ser::Serializer> | 0x1401746e0 | 9 | serde 序列化派生（value::Serializer） | O（serde 派生） |  |
| country_core::resources::glade_color_customization::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > | 0x14085f980 | 59 | serde json 序列化派生 | O（serde 派生） |  |

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`default`、`deserialize`、`eq`、`expecting`、`serialize`、`visit_map`、`visit_seq`、`visit_str` 等 13 项
