# resources/history/roof_edit.rs

> 文件作用：屋顶编辑负载：RoofEdit 操作枚举（FlatRoofRimStyleChange/SwitchRoofRidgeDir 等 tag）手写 serde visitor 族（serialize 0x140259e30、deserialize 0x140b38b80、visit_str 0x140b392d0/0x140b39400/0x140b394f0/0x140b395d0）。重建侧缺此模块。

函数总计 22：业务 6 / 内联无独立符号 16 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x140b392d0 | 15 | serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口） | O(入口) | history\decomp\decomp-0x140b392d0.txt |
| `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str<serde_json::error::Error>` | 0x140b39400 | 15 | serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str<serde_json::error::Error>` | 0x140b394f0 | 15 | serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str<serde_json::error::Error>` | 0x140b395d0 | 15 | serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口） | O(入口) | history\decomp\decomp-0x140b395d0.txt |
| `country_core::resources::history::roof_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x140b38b80 | 15 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::roof_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x140259e30 | 15 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$1::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$12::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$13::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$14::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::_::impl$0::serialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::roof_edit::impl$2::fmt`（行 15）— Display fmt（写 u32 十进制）（O；decomp-0x140b31c90.txt）

<!-- 未覆盖函数数：0 -->
## 实现方式（算法级，2026-10-15 深挖）

- **RoofEdit 操作枚举**：FlatRoofRimStyleChange（含变体）/ SwitchRoofRidgeDir 等 tag；
  手写 serde visitor 族（serialize 0x140259e30 / deserialize 0x140b38b80 / visit_str
  0x140b392d0/0x140b39400/0x140b394f0/0x140b395d0）——按操作 tag 串匹配分派，加载存档
  时还原屋顶编辑负载。重建侧缺此模块（按合同恢复，无自定义业务算法）。

