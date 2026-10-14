# systems/wall/valid_enclosure/mod.rs

> 文件作用：有效围合 ShapeParameters：Curve/Circle/Rectangle 枚举（tag@+0 位 0 判别、载荷@+4）；center/as_curve2/as_curve3/as_circle/as_rectangle/contains_point/contains_shape/perimeter/get_normal_at_u/get_pos_at_u/get_closest_u_from_pos/expand；serde 名 IsEnclosedNoYesPremadeShapeIsEnclosedV2。

函数总计 45：业务 20 / 内联无独立符号 2 / 派生胶水 23

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x1416ebaf0 | 11 | 序列化 | U |  |
| `country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error> >` | 0x1416ebe70 | 11 | 序列化 | I | wall\decomp\decomp-0x1416ebe70.txt |
| `country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x140259300 | 11 | 序列化 | U |  |
| `country_core::systems::wall::valid_enclosure::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x1416eb640 | 11 | 序列化 | I | wall\decomp\decomp-0x1416eb640.txt |
| `country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>` | 0x1416ebf70 | 22 | 序列化 | U |  |
| `country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x1416ebea0 | 22 | 序列化 | U |  |
| `country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<serde_json::error::Error> >` | 0x1416ec140 | 22 | 序列化 | U |  |
| `country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> >` | 0x1416eca30 | 62 | 序列化 | U |  |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::center` | 0x1411ff2a0 | 82 | ShapeParameters::center（15B 极短） | O | wall\decomp\decomp-0x1411ff2a0.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2` | 0x1411ff2b0 | 104 | ShapeParameters::as_curve2 | O | wall\decomp\decomp-0x1411ff2b0.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3` | 0x1411ff2f0 | 118 | ShapeParameters::as_curve3 | O | wall\decomp\decomp-0x1411ff2f0.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point` | 0x140ad2950 | 125 | ShapeParameters::contains_point/get_closest_u_from_pos（叶转发） | O | wall\decomp\decomp-0x140ad2950.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape` | 0x1411ff330 | 132 | ShapeParameters::contains_shape（parry3d 路径） | O | wall\decomp\decomp-0x1411ff330.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u` | 0x1411ff600 | 191 | ShapeParameters::get_normal_at_u（位 0 判别分发） | O | wall\decomp\decomp-0x1411ff600.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u` | 0x1411ff620 | 198 | ShapeParameters::get_pos_at_u（位 0 判别分发） | O | wall\decomp\decomp-0x1411ff620.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter` | 0x1411ff640 | 205 | ShapeParameters::perimeter（位 0 判别分发） | O | wall\decomp\decomp-0x1411ff640.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos` | 0x1411ff660 | 212 | ShapeParameters::perimeter（位 0 判别分发 + 载荷几何） | O | wall\decomp\decomp-0x1411ff660.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::expand` | 0x1411ff680 | 220 | ShapeParameters::expand（向外扩张 amount） | O | wall\decomp\decomp-0x1411ff680.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::as_circle` | 0x1411ff6f0 | 233 | ShapeParameters::as_circle/as_rectangle | O | wall\decomp\decomp-0x1411ff6f0.txt |
| `country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle` | 0x1411ff740 | 241 | 访问器/转换 | I | wall\decomp\decomp-0x1411ff740.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::valid_enclosure::impl$0::from`（行 46）— 待推断
- `country_core::systems::wall::valid_enclosure::impl$2::from`（行 290）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`deserialize`、`eq`、`expecting`、`fmt`、`serialize`、`visit_bytes`、`visit_enum`、`visit_map`、`visit_seq`、`visit_str`、`visit_u64` 等 23 项
