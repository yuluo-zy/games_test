# resources/history/tree_edit.rs

> 文件作用：树编辑负载：TreeEdit（Place/Move + tree{id,position,scale,rotation_degrees,prefab_name,is_preplaced}）、missing_prefab 0x1409e6210、TreeCanopyMaxCullDistance 手写 serde visitor 族（serialize 0x14003d4e0/0x1417c2770/0x1417c2600/0x1417c2a50、deserialize 0x1417c2f60）。

函数总计 25：业务 5 / 内联无独立符号 20 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x1417c2960 | 12 | serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口） | O(入口) | history\decomp\decomp-0x1417c2960.txt |
| `country_core::resources::history::tree_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x14003d4e0 | 12 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::tree_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x1417c2770 | 12 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::tree_edit::missing_prefab` | 0x1409e6210 | 28 | prefab 缺失回退：返回 24B {0x142ac4708,0x142ac4710,"FAB_NAME"}（串 MISSING_TREE_PREFAB_NAME/TREE_PREFAB_NAME 区；树 prefab 回退名候选） | O | decomp-0x1409e6210.txt |
| `country_core::resources::history::tree_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x1417c2f60 | 46 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::resources::history::tree_edit::_::impl$0::deserialize`（行 12）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$1::deserialize`（行 12）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$2::expecting`（行 12）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$2::visit_map`（行 12）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$2::visit_seq`（行 12）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::impl$2::clone`（行 12）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::tree_edit::impl$3::fmt`（行 12）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$0::visit_str`（行 46）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$2::visit_enum`（行 46）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str`（行 46）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize`（行 46）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map`（行 46）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str`（行 46）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize`（行 46）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map`（行 46）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str`（行 46）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize`（行 46）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map`（行 46）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::tree_edit::impl$6::fmt`（行 46）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance::get_dist`（行 100）— 取树冠最大剔除距离（I；内联）

<!-- 未覆盖函数数：0 -->
