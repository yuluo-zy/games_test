# resources/history/path_edit.rs

> 文件作用：路径/鹅卵石光栅编辑负载：RasterEdit（points/brush_size/is_additive/blend_mode；ReplayCmd 判别 0/1/2 的 Arc 负载）+ RasterEditSerialized 手写 serde（serialize 0x1404896f0/0x1416e8340、deserialize 0x1416e8510、From 0x140a976b0）。

函数总计 13：业务 3 / 内联无独立符号 10 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::history::path_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x1416e8510 | 8 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | history\decomp\decomp-0x1416e8510.txt |
| `country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1404896f0 | 8 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::path_edit::impl$1::from` | 0x140a976b0 | 48 | DecoratorHistoryState::From（type@+0x41==8 preview panic） | O | decomp-0x140b3f930.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::resources::history::path_edit::impl$2::fmt`（行 8）— Display fmt（写 u32 十进制）（O；decomp-0x140b31c90.txt）
- `country_core::resources::history::path_edit::impl$3::clone`（行 8）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::path_edit::_::impl$0::deserialize`（行 19）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::path_edit::_::impl$0::deserialize::impl$0::visit_str`（行 19）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::path_edit::_::impl$0::deserialize::impl$1::deserialize`（行 19）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::expecting`（行 19）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::visit_map`（行 19）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::visit_seq`（行 19）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::path_edit::_::impl$0::serialize`（行 19）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::path_edit::impl$0::from`（行 35）— From 转换（RasterEdit 构造）（I；内联）

<!-- 未覆盖函数数：0 -->
