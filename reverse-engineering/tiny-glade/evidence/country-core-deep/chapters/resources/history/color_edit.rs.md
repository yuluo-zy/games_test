# resources/history/color_edit.rs

> 文件作用：颜色编辑负载：ColorEdit（目标 Wall/Roof/Halftimber/Decorator/StairRailing/FlatRoof/StairLadder/Brightness + from/to + 目标 id；字段 chimney_h_offset/allow_connection）手写 serde visitor 族（serialize 0x14003c5d0 / deserialize 0x1417befe0，visit_str 0x1417bf8a0）。

函数总计 48：业务 3 / 内联无独立符号 45 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x1417bf8a0 | 15 | serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口） | O(入口) | history\decomp\decomp-0x1417bf8a0.txt |
| `country_core::resources::history::color_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x1417befe0 | 15 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::color_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x14003c5d0 | 15 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$1::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$12::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$13::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$14::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$15::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$16::deserialize`（行 15）— MultiEdit::deserialize（serde 巨函数；与 visit_seq/visit_map 合并区）（O；decomp-0x140b36460/0x140b365e0/0x140b36840.txt）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$17::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$18::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$19::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$20::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$21::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$22::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$23::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$24::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$25::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$26::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$27::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$28::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$29::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$30::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$31::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$32::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$33::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$34::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$35::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$36::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$37::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$38::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$39::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$40::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$41::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::color_edit::impl$1::fmt`（行 15）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）

<!-- 未覆盖函数数：0 -->
