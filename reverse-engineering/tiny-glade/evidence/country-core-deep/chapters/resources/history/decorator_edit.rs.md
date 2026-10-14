# resources/history/decorator_edit.rs

> 文件作用：装饰历史编辑域：DecoratorHistoryEdit/Diff/State、DecoratorHistoryState::From（0x140b3f930，preview 类型 panic）、is_default_* 谓词（lantern_brightness==stair_width==1.0f，0x140b3f910 ICF）、convert_before_after_states_to_changes（0x140b3fab0，按 style 字节分变体比较浮点输出变更集）、send_state_diff_events（0x1417bdae0，replay_pop_bucket 消费）、cull_allow_list；手写 serde visitor 族。

函数总计 45：业务 10 / 内联无独立符号 35 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x1417c0890 | 17 | serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口） | O(入口) | history\decomp\decomp-0x1417c0890.txt |
| `country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x1417bff80 | 17 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | history\decomp\decomp-0x1417bff80.txt |
| `country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1404f46e0 | 17 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::decorator_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x1417c0cf0 | 70 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | history\decomp\decomp-0x1417c0cf0.txt |
| `country_core::resources::history::decorator_edit::is_default_chimney_smoke` | 0x1408c5680 | 145 | ICF 多符号同址 0x1408c5680（含 is_default_allow_connection 等） | O(ICF) | 符号 0x1408c5680 |
| `country_core::resources::history::decorator_edit::is_default_stair_width` | 0x140b3f910 | 168 | f32==1.0f | O | 符号 0x140b3f910 |
| `country_core::resources::history::decorator_edit::impl$1::from` | 0x140b3f930 | 173 | DecoratorHistoryState::From（type@+0x41==8 preview panic） | O | decomp-0x140b3f930.txt |
| `country_core::resources::history::decorator_edit::impl$25::fmt` | 0x140b30a10 | 288 | Display/Debug：写串 "DecoratorStorageAddr…"（装饰存储地址键） | O | decomp-0x140b30a10.txt |
| `country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>` | 0x1417bdae0 | 322 | 见文件作用 | O | decomp-0x140b38250.txt |
| `country_core::resources::history::decorator_edit::convert_before_after_states_to_changes` | 0x140b3fab0 | 380 | before/after → 变更集（按 style 字节分变体比较浮点，输出 changed 布尔） | O | decomp-0x140b3fab0.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$1::deserialize`（行 17）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum`（行 17）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::decorator_edit::impl$8::fmt`（行 17）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::decorator_edit::impl$9::clone`（行 17）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::decorator_edit::_::impl$0::deserialize`（行 25）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::expecting`（行 25）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::decorator_edit::impl$11::fmt`（行 25）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::decorator_edit::impl$13::fmt`（行 28）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map`（行 31）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::decorator_edit::impl$15::fmt`（行 31）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str`（行 34）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq`（行 34）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::decorator_edit::_::impl$0::serialize`（行 34）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::decorator_edit::impl$18::fmt`（行 34）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::decorator_edit::impl$19::clone`（行 70）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::decorator_edit::impl$20::fmt`（行 70）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::decorator_edit::impl$22::eq`（行 70）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::decorator_edit::is_default`（行 136）— 通用默认值判定（U；内联）
- `country_core::resources::history::decorator_edit::is_default_lantern_brightness`（行 152）— f32==1.0f（与 is_default_stair_width ICF 同址）（O；原字节 0x140b3f910）
- `country_core::resources::history::decorator_edit::impl$2::from`（行 229）— From 转换（负载构造/归一）（I；内联）
- `country_core::resources::history::decorator_edit::impl$3::from`（行 235）— From 转换（负载构造/归一）（I；内联）
- `country_core::resources::history::decorator_edit::impl$4::from`（行 282）— From 转换（负载构造/归一）（I；内联）
- `country_core::resources::history::decorator_edit::impl$23::default`（行 288）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::decorator_edit::impl$5::from`（行 294）— From 转换（负载构造/归一）（I；内联）
- `country_core::resources::history::decorator_edit::impl$5::from::closure$0`（行 299）— From 转换（负载构造/归一）（I；内联）
- `country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize`（行 449）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize`（行 449）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::decorator_edit::_::impl$0::serialize::impl$0::serialize`（行 449）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::decorator_edit::impl$27::default`（行 449）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::decorator_edit::impl$29::clone`（行 449）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::decorator_edit::DecoratorHistoryDiff::is_empty`（行 456）— diff 判空（I；内联）
- `country_core::resources::history::decorator_edit::DecoratorHistoryDiff::cull_allow_list`（行 461）— 装饰 cull 允许列表（to_arc_slice）（I；内联）
- `country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0`（行 465）— 装饰 cull 允许列表（to_arc_slice）（I；内联）
- `country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure$0`（行 471）— 装饰 cull 允许列表（to_arc_slice）（I；内联）
- `country_core::resources::history::decorator_edit::impl$32::clone`（行 477）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）

<!-- 未覆盖函数数：0 -->
