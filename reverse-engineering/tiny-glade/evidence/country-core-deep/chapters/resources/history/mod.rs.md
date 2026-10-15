# resources/history/mod.rs

> 文件作用：权威历史缓冲核心（S05）：History 0x78B 布局（edit_chain/snapshots/pending/计数/nonce/cursor/history_start）、undo/redo/录制/快照方法族、MultiEdit/HistoryEdit/DecoratorDiffTable/StairsDiffTable、HistoryChainIndex/HistoryEditChain、HistorySnapshotId/MaybePendingSnapshotId/HistoryChainSnapshots、EditType 与 MultiEdit 的 serde 巨函数（deserialize 合并区 0x140b3ad60-0x140b3bc83）。

函数总计 107：业务 34 / 内联无独立符号 73 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::history::HistoryIdentity::generate` | 0x140a84220 | 98 | fastrand u64 全范围随机身份 | O | decomp-0x140a84220.txt |
| `country_core::resources::history::impl$2::deserialize<enum2$<serde_json::value::Value> >` | 0x140b363e0 | 134 | HistoryChainSnapshots::deserialize | O | history\decomp\decomp-0x140b363e0.txt |
| `country_core::resources::history::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x140316660 | 236 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x140316500 | 236 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | 见文件作用所列 serde 地址 |
| `country_core::resources::history::impl$9::try_from` | 0x140a84260 | 263 | ResolvedHistoryChainSnapshot TryFrom（真实过滤谓词，规则 U） | O | 符号 0x140a84260 |
| `country_core::resources::history::history_chain::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1408aa9f0 | 319 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | history\decomp\decomp-0x1408aa9f0.txt |
| `country_core::resources::history::history_chain::HistoryChainIndex::new` | 0x140833250 | 333 | 恒等 u32 封装 | O | 符号 0x140833250 |
| `country_core::resources::history::history_chain::impl$2::fmt` | 0x140b31c90 | 361 | Display fmt（写 u32 十进制） | O | decomp-0x140b31c90.txt |
| `country_core::resources::history::history_chain::HistoryEditChain::iter` | 0x140b31cf0 | 412 | 切片迭代 | O | 符号 0x140b31cf0（iter_mut 同址族） |
| `country_core::resources::history::is_decorator_history_diff_empty` | 0x140a84290 | 462 | diff+0x18==0 | O | decomp-0x140a84290.txt |
| `country_core::resources::history::is_stairs_history_diff_empty` | 0x140a842a0 | 466 | 两 Vec cap（+0x30/+0x48）==0 | O | decomp-0x140a842a0.txt |
| `country_core::resources::history::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x140b3b630 | 493 | serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口） | O(入口) | history\decomp\decomp-0x140b3b630.txt |
| `country_core::resources::history::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x140b3ad60 | 493 | serde 主入口 monomorph（serde_json::Value / Serializer 实例） | O(入口) | history\decomp\decomp-0x140b3ad60.txt |
| `country_core::resources::history::EditType::Water::constructor$0` | 0x1416bb7f0 | 500 | Water 构造 | O | 符号 0x1416bb7f0 |
| `country_core::resources::history::EditType::Garden::constructor$0` | 0x1416960d0 | 501 | Garden 构造 | O | 符号 0x1416960d0 |
| `country_core::resources::history::impl$14::fmt` | 0x140a842c0 | 516 | EditType Debug::fmt（跳表分派） | O | decomp-0x140a842c0.txt |
| `country_core::resources::history::History::insert_starting_edit` | 0x140a844f0 | 561 | 断言 cursor>-2；cursor+1；首部插入起始编辑；写 nonce；count=0 | O | decomp-0x140a844f0.txt |
| `country_core::resources::history::History::current_snapshot` | 0x140a84650 | 590 | 对 snapshots 二分 chain_index<=cursor；仅当 ==cursor 才 Arc clone 返回 | O | decomp-0x140a84650.txt |
| `country_core::resources::history::History::set_current_snapshot` | 0x140a84760 | 595 | 断言 current_snapshot().is_none() 后有序插入（不替换） | O | decomp-0x140a84760.txt |
| `country_core::resources::history::History::iter_resolved_snapshots` | 0x140a84a40 | 611 | snapshots 切片 filter_map 迭代 | O | decomp-0x140a84a40 区；符号 0x140a84a40 |
| `country_core::resources::history::History::closest_resolved_snapshot` | 0x140a84a50 | 627 | filter_map 构建 Resolved Vec 后二分 chain_index<=cursor 最近者 | O | decomp-0x140a84a50.txt |
| `country_core::resources::history::History::debug_display` | 0x140a84ba0 | 691 | 格式串 History: {} / {}. Start at {} | O | decomp-0x140a84ba0.txt |
| `country_core::resources::history::History::debug_set_history_start` | 0x140a84c40 | 701 | mov [rcx+0x6c],edx | O | history\decomp\decomp-0x140a84c40.txt |
| `country_core::resources::history::History::nuke_future_states` | 0x140a84c50 | 705 | cursor!=len-1 时整组重置或 truncate+retain；写 nonce | O | decomp-0x140a84c50.txt |
| `country_core::resources::history::History::add` | 0x140a84ec0 | 740 | pending 未录制 panic；空桶+cursor-len<=-2 先 nuke；tag7 墙族校验；push 24B 记录；第 3 参=track-caller 文件 | O | decomp-0x140a84ec0.txt |
| `country_core::resources::history::History::add_decorator_diff` | 0x140a854a0 | 815 | 4 参：未录制 panic；nuke 判定+重建桶；楼梯 checksum 校验；两 u32 边对循环 push tag 0xE/1；Arc 替换两表 | O | decomp-0x140a854a0.txt；DIGEST §3.2 |
| `country_core::resources::history::History::start_input_recording` | 0x140a86270 | 905 | AppMode::Replay(0xd) 断言；drop 旧桶；新桶（64B 表+96B 表+全局序列） | O | decomp-0x140a86270.txt |
| `country_core::resources::history::History::try_start_input_recording` | 0x140a86510 | 916 | 非 Replay 且未录制才启动，返回 bool | O | decomp-0x140a86510.txt |
| `country_core::resources::history::History::finish_input_recording` | 0x140a86730 | 930 | 取出 pending；空则 drop；push 链、cursor+1、断言、nonce、snapshots 追加 48B 记录 | O | decomp-0x140a86730.txt |
| `country_core::resources::history::History::is_recording` | 0x140a86970 | 980 | pending+0x30>=0（哨兵判别） | O | history\decomp\decomp-0x140a86970.txt |
| `country_core::resources::history::History::finish_potential_mutation` | 0x140a86980 | 1025 | was_active 为真则清 snapshots（Arc drop）+nonce；panic-in-progress 门控 0x14362c420 | O | decomp-0x140a86980.txt |
| `country_core::resources::history::impl$16::deserialize<enum2$<serde_json::value::Value> >` | 0x140b36460 | 1078 | MultiEdit::deserialize（serde 巨函数；与 visit_seq/visit_map 合并区） | O | decomp-0x140b36460/0x140b365e0/0x140b36840.txt |
| `country_core::resources::history::impl$16::deserialize::impl$0::visit_seq<ref_mut$<serde_json::value::de::SeqDeserializer> >` | 0x140b365e0 | 1092 | MultiEdit::deserialize（serde 巨函数；与 visit_seq/visit_map 合并区） | O | decomp-0x140b36460/0x140b365e0/0x140b36840.txt |
| `country_core::resources::history::impl$16::deserialize::impl$0::visit_map<ref_mut$<serde_json::value::de::MapDeserializer> >` | 0x140b36840 | 1106 | MultiEdit::deserialize（serde 巨函数；与 visit_seq/visit_map 合并区） | O | decomp-0x140b36460/0x140b365e0/0x140b36840.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::resources::history::_::impl$0::deserialize`（行 43）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::_::impl$0::deserialize::impl$0::visit_str`（行 43）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::_::impl$0::deserialize::impl$1::deserialize`（行 43）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::_::impl$0::deserialize::impl$2::expecting`（行 43）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::_::impl$0::deserialize::impl$2::visit_map`（行 43）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::_::impl$0::deserialize::impl$2::visit_seq`（行 43）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::impl$17::default`（行 43）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::_::impl$0::serialize`（行 92）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::impl$23::eq`（行 92）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::HistoryIdentity::mutate`（行 102）— 重随机身份（I；内联）
- `country_core::resources::history::impl$26::clone`（行 115）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::impl$28::eq`（行 115）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::impl$1::serialize`（行 120）— HistoryIdentity serde Serialize（u64 数字）（I；内联（闭包并入））
- `country_core::resources::history::impl$1::serialize::closure$0`（行 127）— HistoryIdentity serde 数字序列化闭包（I；内联）
- `country_core::resources::history::impl$2::deserialize`（行 134）— HistoryChainSnapshots::deserialize（O；符号 0x140b363e0）
- `country_core::resources::history::impl$2::deserialize::closure$0`（行 141）— HistoryChainSnapshots::deserialize（O；符号 0x140b363e0）
- `country_core::resources::history::impl$32::fmt`（行 152）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::MaybePendingSnapshotId::get`（行 162）— 取待定快照 id（U；内联）
- `country_core::resources::history::MaybePendingSnapshotId::assign`（行 183）— 分配快照 id（U；内联）
- `country_core::resources::history::MaybePendingSnapshotId::assign_invalid`（行 187）— 分配快照 id（U；内联）
- `country_core::resources::history::impl$33::clone`（行 193）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::impl$4::eq`（行 202）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::HistoryChainSnapshot::snapshot_id`（行 225）— 从快照负载解析 id（U；内联）
- `country_core::resources::history::impl$6::history_chain_index`（行 231）— 取快照的 chain_index 字段（I；内联（+0x08 i32））
- `country_core::resources::history::impl$7::history_chain_index`（行 244）— 取快照的 chain_index 字段（I；内联（+0x08 i32））
- `country_core::resources::history::impl$8::from`（行 250）— 快照负载 From 转换（U；内联）
- `country_core::resources::history::impl$44::eq`（行 271）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::HistorySnapshotId::from_snapshot_blob`（行 279）— blob → id 解析（U；内联）
- `country_core::resources::history::HistorySnapshotId::file_name`（行 283）— id → 文件名（U；内联）
- `country_core::resources::history::HistorySnapshotId::from_file_stem`（行 288）— 文件 stem → id（U；内联）
- `country_core::resources::history::HistoryMutationToken::i_promise_there_were_no_mutations`（行 301）— 消费 mutation token（置 inactive）（O；内联进 finish_input_recording）
- `country_core::resources::history::impl$12::drop`（行 307）— token Drop 守卫（active 未消费 panic；0x14362c420 panic-in-progress 门控）（O；DIGEST §6）
- `country_core::resources::history::history_chain::_::impl$0::deserialize`（行 319）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::history_chain::_::impl$0::serialize`（行 319）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::history_chain::impl$9::eq`（行 319）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::history_chain::HistoryChainIndex::prev`（行 337）— -1（I；内联）
- `country_core::resources::history::history_chain::HistoryChainIndex::next`（行 342）— +1（I；内联）
- `country_core::resources::history::history_chain::impl$3::index`（行 375）— Index usize（I；内联）
- `country_core::resources::history::history_chain::HistoryEditChain::new_from_inner`（行 381）— Vec 封装构造（I；内联）
- `country_core::resources::history::history_chain::HistoryEditChain::get`（行 386）— 下标访问（I；内联）
- `country_core::resources::history::history_chain::HistoryEditChain::is_empty`（行 400）— 判空（I；内联）
- `country_core::resources::history::history_chain::HistoryEditChain::truncate`（行 404）— 截断（I；内联）
- `country_core::resources::history::history_chain::HistoryEditChain::push`（行 408）— push 元素 0x28（I；内联；grow_one 0x28）
- `country_core::resources::history::history_chain::HistoryEditChain::iter_mut`（行 416）— 切片迭代（O；符号 0x140b31cf0（iter_mut 同址族））
- `country_core::resources::history::history_chain::HistoryEditChain::insert_at_start`（行 421）— 首部插入（memmove 右移）（I；内联进 insert_starting_edit）
- `country_core::resources::history::impl$48::default`（行 437）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::impl$49::clone`（行 437）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::MultiEdit::iter`（行 475）— edits 迭代（I；内联）
- `country_core::resources::history::MultiEdit::is_empty`（行 483）— 四条件空判定（O；内联进 finish_input_recording）
- `country_core::resources::history::_::impl$0::deserialize::impl$2::visit_enum`（行 493）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize`（行 493）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map`（行 493）— serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）（O(入口)；见文件作用所列 serde 地址）
- `country_core::resources::history::impl$50::clone`（行 493）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::EditType::Path::constructor$0`（行 496）— Path 变体构造（O；符号族；serde visitor）
- `country_core::resources::history::EditType::Cobblestone1::constructor$0`（行 497）— Cobblestone1 构造（O；符号族）
- `country_core::resources::history::EditType::Cobblestone2::constructor$0`（行 498）— Cobblestone2 构造（O；符号族）
- `country_core::resources::history::EditType::Cobblestone3::constructor$0`（行 499）— Cobblestone3 构造（O；符号族）
- `country_core::resources::history::History::new_from_params`（行 538）— （reconstruction 别名；真名 insert_starting_edit）（O；见下）
- `country_core::resources::history::History::edit_chain_mut`（行 578）— edit_chain 可变访问器（I；内联）
- `country_core::resources::history::impl$15::current_snapshot::closure$0`（行 592）— 二分比较闭包（chain_index 键）（I；内联进 0x140a84650）
- `country_core::resources::history::impl$15::set_current_snapshot::closure$0`（行 600）— 二分比较闭包（chain_index 键）（I；内联进 0x140a84650）
- `country_core::resources::history::impl$15::iter_resolved_snapshots::closure$0`（行 616）— filter_map 谓词（TryFrom 0x140a84260 ok 过滤）（I；round3 round2 修正：不内联进 0x140a84a40——该址 16B 仅为切片迭代器构造 mov rax,[rcx+0x20]; mov rdx,[rcx+0x28]; shl rdx,4; add rdx,rax; ret，谓词调用在消费点）
- `country_core::resources::history::History::closest_snapshot`（行 619）— 最近快照二分辅助（非公开方法）（I；内联；CLOSURE §2）
- `country_core::resources::history::History::remove_snapshot_record`（行 640）— 移除快照记录（无独立符号=内联；sysres 名址错位）（U；CLOSURE §1）
- `country_core::resources::history::impl$15::remove_snapshot_record::closure$0`（行 642）— 移除谓词闭包（随宿主内联，无独立符号）（U；CLOSURE §1）
- `country_core::resources::history::History::undo`（行 646）— 游标守卫三重检查后 cursor-1（O；内联进 history_process；DIGEST §3.3）
- `country_core::resources::history::History::redo`（行 672）— cursor>=-1 断言；target 守卫；cursor=target（O；内联进 history_process）
- `country_core::resources::history::impl$15::nuke_future_states::closure$0`（行 719）— retain 谓词（chain_index<=cursor）（I；内联进 0x140a84c50）
- `country_core::resources::history::History::new`（行 987）— History 构造（i64::MIN 哨兵 + cursor=-1 + tail=0 初始化）（O；CLOSURE §1（0x140a84e60 机器码））
- `country_core::resources::history::History::pending_undo_count`（行 1014）— 待撤销计数（U；内联）
- `country_core::resources::history::closest_history_snapshot`（行 1033）— （sysres 名址双错；真实逻辑=current_snapshot/closest_resolved + save::snapshot 域）（纠错；CLOSURE §2）
- `country_core::resources::history::closest_history_snapshot::closure$0`（行 1037）— （sysres 名址双错；真实逻辑=current_snapshot/closest_resolved + save::snapshot 域）（纠错；CLOSURE §2）
- `country_core::resources::history::impl$16::deserialize::impl$0::expecting`（行 1087）— MultiEdit::deserialize（serde 巨函数；与 visit_seq/visit_map 合并区）（O；decomp-0x140b36460/0x140b365e0/0x140b36840.txt）

<!-- 未覆盖函数数：0 -->
## 实现方式（算法级，2026-10-15 深挖；全文见 history/ALGORITHMS.md）

- **History::add 0x140a84ec0**（3 参：`(&mut self, edit: &HistoryEdit 24B, file: &'static str)`）：
  1) `pending(+0x30) < 0` → panic "Tried writing {edit:?}…"（@0x142ad8250，参数 = EditType Debug::fmt 0x140a842c0 + track-caller 文件）；2) 桶空且 `cursor - len < -2` →
  先 `nuke_future_states 0x140a84c50` 再重建 pending 桶（64B 装饰表 Arc{1,1}+模板指针
  0x142af1470..0x142af1488+全局序列 FUN_1400d9b20；96B 楼梯表 Arc{1,1}+两空 Vec）；
  3) **tag 7（墙）校验**：records = 组数组（16B {入口指针数组, 组数}），逐条目 niche
  解码（`载荷+0x10 - 0x7fffffffffffffff`）：变体 0 → 断言 +0x100 f32 > +0xfc f32
  （否则 panic "Slice start_u must be smaller than end_u. Otherwise an invalid curve
  will be created!" @0x142ad8170）；变体 1/2 → 断言 +0x18 >= 0（panic "None is not
  allowed in `CreateWall`/`DeleteWall` history command" @0x142ad81b8/0x142ad8200）；
  4) push 24B {tag, records, count} 进 pending.edits（元素 0x18）。
- **nuke_future_states 0x140a84c50**：`cursor == len-1` → 仅重随机 nonce；`cursor < 0`
  → 全重置（drop edit_chain + 重建 {cap=0,ptr=8,len=0, pending=i64::MIN, cursor=-1,
  tail=0}）；否则 truncate 至 cursor+1（逐个 drop 尾元素）+ `count(+0x58)=min(count,len)`
  + `FUN_1412f5320` = snapshots.retain(chain_index<=cursor)；末尾**无条件**
  `nonce(+0x60) = fastrand u64`。
- **insert_starting_edit 0x140a844f0**：断言 cursor>=-1 → cursor+1 → edit_chain 首部
  插入（`func_0x000142923c10` memmove 右移 0x28）→ nonce=fastrand、count=0。
- **finish_input_recording 0x140a86730**：pending 先置 i64::MIN（原值 <0 → panic
  "Called finish_input_recording when history recording was already Finished"
  @0x142ad84a8）；MultiEdit::is_empty 四条件（edits.len==0 && 装饰表+0x28==0 &&
  楼梯表+0x40==0 && +0x58==0）→ drop 返回 0；非空 → push 0x28B 上链、cursor+1
  （断言 >=-1）、nonce=fastrand；仅当 cursor == len-1 时快照追加（48B 记录
  {i64::MIN 待定 id, 1, 计数器}，否则 panic @0x142ad8440）。
- **start_input_recording 0x140a86270**：谓词 `func_0x000140a825e0(mode, 0xd)` =
  `app_mode != Replay(0xd)`，失败 → panic "assertion failed: **app_mode !=
  AppMode::Replay"（@0x142ad83f8）；建新桶。**try_start 0x140a86510**：谓词通过且
  pending < 0 → 建桶返回 1，否则 0。
- **current_snapshot 0x140a84650**：snapshots（+0x20/+0x28，元素 16B {Arc, idx@+8}）
  partition_point(idx < cursor) 二分 + **距离平局**（|prev-idx| vs |next-idx|，平局取
  后者）；Arc clone 后 **idx == cursor 才返回 Some**。**closest_resolved_snapshot
  0x140a84a50**：先 filter_map（TryFrom 0x140a84260 谓词）收集，再同一二分+平局，
  不 clone（借用）。**set_current_snapshot 0x140a84760**：二分后若存在 idx==cursor
  快照 → panic（断言不替换，@0x142ad8050 mod.rs:0x33）；否则在 partition_point 处
  memmove 右移 0x10 有序插入 {Arc, cursor}。
- **finish_potential_mutation 0x140a86980**：was_active → 清空 snapshots（逐记录
  LOCK dec Arc，归零 drop 0x1402e5d80）+ nonce=fastrand；token 未消费 panic
  "History was mutated, but wasn't told about it"（@0x142a98ad8）由 Drop 守卫触发，
  panic-in-progress 门控 `*0x14362c420`。

