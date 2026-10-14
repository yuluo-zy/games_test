# resources/walls/inner_wall_state.rs

> 文件作用：内部墙状态权威：WallState 0x70B（enclosure 记录@+0x38、height f32@+0x54、terrain floor@+0x58、freehand_ty u8@+0x60、arch_type u8@+0x64、removed_supports u8@+0x67、双 Vec 曲线）+ WallStyle/StructureType/ArchType/FreehandHeightType/PlasterPeelAmount 枚举族与兼容 serde（13 字段名表）。

函数总计 45：业务 11 / 内联无独立符号 7 / 派生胶水 27

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x14085ff00 | 19 | 序列化 | U |  |
| `country_core::resources::walls::inner_wall_state::FreehandHeightType::toggle` | 0x1408c5660 | 46 | FreehandHeightType::toggle（*self ^= 1） | O | wall\decomp\decomp-0x1408c5660.txt |
| `country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x140860d50 | 49 | 序列化 | U |  |
| `country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x1404f40a0 | 49 | 序列化 | U |  |
| `country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered` | 0x1408c5670 | 69 | WallStyle::is_halftimbered（cmp byte,2; setae） | O | wall\decomp\decomp-0x1408c5670.txt |
| `country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1404f3da0 | 84 | 序列化 | U |  |
| `country_core::resources::walls::inner_wall_state::StructureType::is_fence` | 0x1408c5680 | 156 | StructureType::is_fence（判别读取，比较内联调用方） | O | wall\decomp\decomp-0x1408c5680.txt |
| `country_core::resources::walls::inner_wall_state::StructureType::is_wall` | 0x140833230 | 161 | ClutterVersion::is_initial（cmp byte,0; sete） | O | wall\decomp\decomp-0x140833230.txt |
| `country_core::resources::walls::inner_wall_state::is_default_arch_height` | 0x1408c5690 | 169 | is_default_arch_height（cmpeqss 2.8f@0x142a8cc60 + and 1 → arch_height==2.8f） | O | wall\decomp\decomp-0x1408c5690.txt |
| `country_core::resources::walls::inner_wall_state::WallState::get_enclosure_params` | 0x1408c56b0 | 211 | WallState::get_enclosure_params（tag@+0x38<2 ? &self+0x38 : null） | O | wall\decomp\decomp-0x1408c56b0.txt |
| `country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >` | 0x14085ebc0 | 257 | deserialize_wall_style 兼容反序列化（Stone/StoneWithSkirts/HalfTimbered + 13 字段名表） | O | wall\decomp\decomp-0x14085ebc0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version`（行 62）— 待推断
- `country_core::resources::walls::inner_wall_state::is_default`（行 139）— 谓词/校验
- `country_core::resources::walls::inner_wall_state::WallState::new`（行 174）— 构造
- `country_core::resources::walls::inner_wall_state::WallState::height`（行 195）— 待推断
- `country_core::resources::walls::inner_wall_state::WallState::is_premade_shape`（行 202）— 谓词/校验
- `country_core::resources::walls::inner_wall_state::WallState::is_enclosed`（行 219）— 谓词/校验
- `country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height`（行 239）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`deserialize`、`eq`、`expecting`、`fmt`、`serialize`、`visit_bool`、`visit_enum`、`visit_map`、`visit_newtype_struct`、`visit_seq`、`visit_str` 等 27 项
