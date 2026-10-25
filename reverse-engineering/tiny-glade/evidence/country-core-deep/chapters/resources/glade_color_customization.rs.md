# resources/glade_color_customization.rs

> 文件作用：**谷地自定义配色（serde 存档；默认字节 O）。**

函数总计 16：业务 3 / 内联无独立符号 0 / 派生胶水 13

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::glade_color_customization::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error> | 0x14085f550 | 9 | serde 反序列化派生（GladeColorCustomization visit_str） | O（serde 派生） |  |
| country_core::resources::glade_color_customization::_::impl$0::serialize<serde_json::value::ser::Serializer> | 0x1401746e0 | 9 | serde 序列化派生（value::Serializer） | O（serde 派生） |  |
| country_core::resources::glade_color_customization::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > | 0x14085f980 | 59 | serde json 序列化派生 | O（serde 派生） |  |

## 实现方式（算法级）（O 派生级 + reanalyst 2026-10-15 字段全表）

**GladeColorCustomization**：谷地自定义配色 serde 存档类型——无独立业务函数体，全部
serde 派生（deserialize visit_str @0x14085f550、serialize value @0x1401746e0、serialize
Compact @0x14085f980）；默认字节 O。算法链：存档读写即 serde 往返（I），无游戏自有
计算过程。O/I 混合

**15 序列化字段全表（O，reanalyst：to_json_string 0x14085ee90 + visit_str 0x14085f550
fresh 全解；字段名串 RNE 0x142a956f0 连续打包）**：

| index | 字段名 | 偏移 | 类型 |
|---|---|---|---|
| 10 | windows | +0x00 | ColorId(u32) |
| 14 | stairs | +0x08 | ColorId(u32) |
| 0 | freeform_wall | +0x18 | ColorId(u32) |
| 1 | building | +0x1c | ColorId(u32) |
| 2 | roof | +0x20 | ColorId(u32) |
| 5 | halftimber | +0x24 | ColorId(u32) |
| 6 | lantern_light | +0x28 | ColorId(u32) |
| 7 | fence | +0x2c | ColorId(u32) |
| 8 | window_light | +0x30 | ColorId(u32) |
| 9 | flag | +0x34 | bool |
| 11 | gates | +0x38 | ColorId(u32) |
| 13 | lantern_brightness | +0x3c | f32/u32 |
| 4 | freehand_ty | +0x40 | 字节枚举 |
| 12 | flat_rim_style | +0x41 | 字节枚举 |
| 3 | building_wall_style | +0x42 | 字节枚举(StairRailingType?) |

详见 state/save/RE-DOMAIN-TYPES.md §3。

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`default`、`deserialize`、`eq`、`expecting`、`serialize`、`visit_map`、`visit_seq`、`visit_str` 等 13 项
