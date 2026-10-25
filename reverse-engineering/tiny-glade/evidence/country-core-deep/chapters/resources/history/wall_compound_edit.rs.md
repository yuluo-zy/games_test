# resources/history/wall_compound_edit.rs

> 文件作用：墙复合历史编辑域：WallHistoryEdit 操作枚举（ChangeFreehandHeightType/PlasterPeelAmount/ChangePlasterPeelAmount/MorphPremadeShape/MorphFreehandWall…，niche 判别首字段）、WallHistoryCompoundEdit（0x108B 负载）、ConcurrentEdits、CompoundEditBuilder（new_edit_sequence/next/build 0x140a82d50）、panic_deprecated_operation 0x140a82e20、update_compatibility（内联）、gather_wall_ids；手写 serde visitor 族（deserialize 0x141401960 等 ~40 变体）。

函数总计 79：业务 11 / 内联无独立符号 68 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::history::wall_compound_edit::impl$5::fmt` | 0x140a87970 | 15 | WallHistoryCompoundEdit Display（写串 "WallHistoryCompoundEdit"+参数） | O | decomp-0x140a87970.txt |
| `country_core::resources::history::wall_compound_edit::CompoundEditBuilder::new_edit_sequence` | 0x140a82b20 | 43 | 开始新编辑序列 | O | 符号 0x140a82b20 |
| `country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next` | 0x140a82b40 | 58 | 推进到下一组 | O | 符号 0x140a82b40 |
| `country_core::resources::history::wall_compound_edit::CompoundEditBuilder::build` | 0x140a82d50 | 62 | 构建 Option<WallHistoryCompoundEdit> | O | 符号 0x140a82d50 |
| `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x1414022c0 | 80 | serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口） | O(入口) | history\decomp\decomp-0x1414022c0.txt |
| `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str<serde_json::error::Error>` | 0x141402680 | 80 | serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str<serde_json::error::Error>` | 0x141402790 | 80 | serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x141401960 | 80 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::wall_compound_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x140314a00 | 80 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::wall_compound_edit::panic_deprecated_operation` | 0x140a82e20 | 213 | 已弃用操作 panic | O | 符号 0x140a82e20 |
| `country_core::resources::history::wall_compound_edit::impl$4::fmt` | 0x140a82eb0 | 317 | WallHistoryEdit Debug::fmt（niche 跳表 0x142ad888c） | O | decomp-0x140a82eb0.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_newtype_struct`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::serialize`（行 15）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::impl$6::clone`（行 15）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::wall_compound_edit::impl$7::fmt`（行 21）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::wall_compound_edit::impl$8::clone`（行 21）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit::new`（行 28）— 0x108B 负载构造（I；内联）
- `country_core::resources::history::wall_compound_edit::impl$0::new::closure$0`（行 31）— WallHistoryCompoundEdit::new 内组处理闭包（I；内联）
- `country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next_empty`（行 48）— 当前组判空（I；内联）
- `country_core::resources::history::wall_compound_edit::CompoundEditBuilder::with_concurrent`（行 53）— 加入并发组（I；内联）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$1::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$12::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$13::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$14::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$15::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$16::deserialize`（行 80）— MultiEdit::deserialize（serde 巨函数；与 visit_seq/visit_map 合并区）（O；decomp-0x140b36460/0x140b365e0/0x140b36840.txt）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$17::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$18::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$19::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$20::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$21::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$22::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$23::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$24::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$25::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$26::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$27::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$28::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$29::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$30::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$31::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$32::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$33::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$34::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$35::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$36::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$37::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$38::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$39::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$40::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$41::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$42::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$43::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$44::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$44::visit_map::impl$0::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$44::visit_map::impl$1::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$45::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$46::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$47::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$48::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$49::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$50::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::serialize::impl$0::serialize`（行 80）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::WallHistoryEdit::update_compatibility`（行 221）— 兼容性更新（无独立符号=内联）（U；CLOSURE §1）
- `country_core::resources::history::wall_compound_edit::WallHistoryEdit::gather_wall_ids`（行 276）— 收集涉及的墙 id（U；内联）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_str`（行 301）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str`（行 301）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）

<!-- 未覆盖函数数：0 -->
## 实现方式（算法级，2026-10-15 深挖）

- **CompoundEditBuilder**：new_edit_sequence 0x140a82b20（开始新序列）→ next
  0x140a82b40（推进到下一组）→ build 0x140a82d50（组装 Option<WallHistoryCompoundEdit>
  0x108B）；`wall_deleted 0x140a876a0` = systems::wall::deferred_wall_history 的 trait
  impl（墙删除合并入口）。panic_deprecated_operation 0x140a82e20 = 已弃用操作 panic。
- **WallHistoryEdit Debug::fmt 0x140a82eb0**：首字段 niche 判别（`*p -
  0x7fffffffffffffff` 得变体）经跳表 0x142ad888c 分派打印各操作变体。
- **update_compatibility / gather_wall_ids**：无独立符号（内联；CLOSURE §1 排除
  0x140a82940..0x140a82b20 全为 drop/fmt glue）。
- serde deserialize 族（0x141401960/0x1414022c0/0x141402680/0x141402790）：操作 tag
  串 ChangeFreehandHeightType / PlasterPeelAmount / ChangePlasterPeelAmount /
  MorphPremadeShape / MorphFreehandWall…（~40 变体 visitor）。

