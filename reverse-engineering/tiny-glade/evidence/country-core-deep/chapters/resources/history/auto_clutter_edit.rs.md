# resources/history/auto_clutter_edit.rs

> 文件作用：自动杂物编辑负载：AutoClutterEdit（墙派生装饰记录；host wall_id + decorator_ids）的手写 serde visitor 族（serialize 0x14003d300 / deserialize 0x140a18500）。

函数总计 9：业务 2 / 内联无独立符号 7 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::history::auto_clutter_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x140a18500 | 8 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::auto_clutter_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x14003d300 | 8 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::resources::history::auto_clutter_edit::_::impl$0::deserialize::impl$0::visit_str`（行 8）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::auto_clutter_edit::_::impl$0::deserialize::impl$1::deserialize`（行 8）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::auto_clutter_edit::_::impl$0::deserialize::impl$2::visit_enum`（行 8）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::auto_clutter_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str`（行 8）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::auto_clutter_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize`（行 8）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::auto_clutter_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map`（行 8）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::auto_clutter_edit::impl$1::fmt`（行 8）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）

<!-- 未覆盖函数数：0 -->
## 实现方式（算法级，2026-10-15 深挖）

- **AutoClutterEdit**（墙派生装饰记录：host wall_id + decorator_ids）：手写 serde
  visitor 族（serialize 0x14003d300 / deserialize 0x140a18500）。
- 运行侧消费：replay_auto_clutter 0x140aa4a60（undo==0 插墙集 / !=0 从
  DeletedAutoClutter erase (DecoratorId, DecoratorId)）。

