# WP13 — `resources::history` 业务逻辑合同（S05 权威历史缓冲）

B 版本（EXE sha256 cf048e27…，PDB sha256 2d9686b8…）。本文件记录
`crates/country-core/src/resources/history/*`（12 个模块 + mod.rs）恢复的
**Observed / Inferred / Unknown** 证据与 typed gap。实现见
`src/resources/history/`。差分仅用于裁决关键数值/边界歧义，不作为实现手段。

## 模块清单（B skeleton，12 个 .rs + mod.rs）

`auto_clutter_edit` / `color_edit` / `decorator_edit` / `path_edit` /
`replay_cmd_buffer` / `roof_edit` / `stairs_history_edit` / `terrain_edit` /
`tree_edit` / `wall_compound_edit` / `wall_history` / `mod`。
子模块 `history_chain`（`HistoryChainIndex` / `HistoryEditChain`）在 `mod.rs`
编译单元内（候选 TG-3846e898cf43c29d09 已定位 `HistoryChainIndex`）。

## O — 直接观察（B 反编译 + 原机器码 + PDB 骨架）

### History 资源布局（Ghidra 反编译 `nuke_future_states` 0x140a84c50、
`finish_input_recording` 0x140a86730、`start_input_recording` 0x140a86270、
`add` 0x140a84ec0 的字段访问/重置写）：

| 偏移 | 大小 | 含义 | 证据 |
|---|---|---|---|
| +0x00 | Vec<MultiEdit> | `edit_chain`（元素 0x28=40B） | 重置分支写 ptr/cap/len；finish 用 0x28 步长 push；debug_display 读 len-1 |
| +0x18 | Vec<HistoryChainSnapshot> | `snapshots`（元素 0x10=16B） | current_snapshot 二分搜索 0x10 步长；finish_potential_mutation 清空 |
| +0x30 | 40B | 进行中的 `pending` MultiEdit（累计桶） | start_input_recording 写入 [0]=0,[1]=8,[2]=0,[3]=alloc,[4]=alloc；finish 取出 |
| +0x58 | usize | 计数，nuke 时 clamp 到 min(自身, edit_chain.len) | nuke_future_states `+0x58 = min(+0x58, len)` |
| +0x60 | u64 | `mutation_nonce`（fastrand::u64） | add/finish/nuke/insert_starting_edit 均写 `+0x60` |
| +0x68 | i32 | `chain_index`/cursor（undo/redo 目标） | history_process 读写；debug_set_history_start 写 +0x6c 故 cursor=+0x68 |
| +0x6c | i32 | `history_start`（undo 下界锚） | debug_set_history_start `*(u32*)(rcx+0x6c)=edx`；history_process `cursor<=start→返回` |
| +0x70 | u32 | U（多出的 4B；序列化 `snapshot` 相关） | 反编译显示重置写 0 |

### History 方法行为（全部 B 反编译，地址见 evidence/）

- `add` 0x140a84ec0：若 pending.edits 为空 **且** `cursor(+0x68) - len(+0x10) < -1`
  则先 `nuke_future_states`（0x140a84c50）。随后校验 edit 内层（panic 串
  “None is not allowed in `CreateWall`/`DeleteWall` history command”、
  “Tried writing to History when there was no input recording”）。向 pending 桶累计。
- `start_input_recording` 0x140a86270：`AppMode::ne` 断言（replay 模式下 panic，
  O：比较 AppMode）；丢弃旧 pending 桶（drop 旧 MultiEdit），分配新桶
  `[0]=0,[1]=8,[2]=0,[3]=type表,[4]=aux`。
- `try_start_input_recording` 0x140a86510：`AppMode::ne` 且当前未在录制才启动，返回 bool。
- `finish_input_recording` 0x140a86730：取出 pending（panic 若未录制）；若空则 drop 不提交；
  否则 push 到 edit_chain（0x28 元素）、`cursor+1`、断言 `cursor==len-1`、写 nonce，
  并向 `snapshots` 追加 `{0x8000...MIN, 1, chain_index=cursor}` 记录。
- `finish_potential_mutation` 0x140a86980：若 active（DL≠0）则清空 snapshots Vec
  （drop Arc），写 nonce。匹配事务观察模型。
- `nuke_future_states` 0x140a84c50：若 `cursor != len-1`：cursor<0 → 整组重置
  （`+0x30=i64::MIN`、两 Vec 清空、count=0、nonce）；否则 `truncate(cursor+1)`、
  clamp `+0x58`、`retain(snapshot.index<=cursor)`；最后写 nonce。
- `insert_starting_edit` 0x140a844f0：`cursor>-2` 断言；`cursor+1`；edit_chain
  首部 memmove 右移后插入（起始编辑）；写 nonce、`+0x58=0`。
- `current_snapshot` 0x140a84650：对 snapshots 二分搜索 `chain_index(+0x08) <= cursor`
  最近者，返回 Arc clone（refcount++）。
- `set_current_snapshot` 0x140a84760：二分插入 `{snapshot Arc, chain_index=cursor}`；
  若已有同 index 记录则替换。
- `closest_resolved_snapshot` 0x140a84a50：filter_map 构建
  `Vec<ResolvedHistoryChainSnapshot>` 后二分，返回 `Option<{payload, chain_index}>`。
- `is_recording` 0x140a86970、`iter_resolved_snapshots` 0x140a84a40、
  `debug_set_history_start` 0x140a84c40、`debug_display` 0x140a84ba0
  （格式串 “History: {} / {}. Start at {}”，读 cursor/start）。
- `HistoryIdentity::generate` 0x140a84220：`fastrand::Rng.with(RangeInclusive<u64>)` → u64。
- `is_stairs_history_diff_empty` 0x140a842a0 / `is_decorator_history_diff_empty`
  0x140a84290：对 diff 容器判空。

### history_chain 子模块

- `HistoryChainIndex::new/inner` 0x140833250（恒等 u32 封装）；`next`/`prev`（内联进
  insert_starting_edit / history_process）；Display fmt 0x140b31c90；serde（数字）。
- `HistoryEditChain`（Vec<MultiEdit> 封装）：`new_from_inner` / `is_empty` /
  `iter` / `iter_mut` 0x140b31cf0 / `get` / `push` / `insert_at_start` /
  `truncate` / Index。push 元素 0x28=40B（O：grow_one 元素 0x28）。

### 撤销/重做（内联进 `history_process` 0x140af5b30，O）

HistoryCmd 4 字节字：byte[0]=命令族；byte[1]=子命令：
- `byte[1]==0` → **Undo**：`if cursor<0 → 返回; if cursor<=history_start(+0x6c) → 返回;
  if len<=cursor → 返回; cursor-1`；随后 `replay_edit()`。
- `byte[1]==1` → **Redo**：`if cursor<-1 → panic; target=cursor+1; if target<0 → 返回;
  if len<=target → 返回; cursor=target`；随后 `replay_edit()`。
- `byte[1]==4` 或 `byte[0]==2` → **Interrupt**（`HistoryCmd::interrupt()` 构造 0x02000200，
  候选 TG-c4cdc0aa5e62db9266；中断批量命令消费）。

### 持久化 schema（复用 reconstruction/history 观察模型 + `SaveFileSection`）

- `History` 序列化字段：`edit_chain`、`chain_index`、`snapshots`、`history_start`。
- 外层命令 tag（O，starting-builds JSON 2,120 edit_chain 条目）：`WallNewSystem`/
  `Wall`/`Roof`/`DecoratorV5`/`Terrain`/`Tree`/`Color`/`Stairs`/`Path`/`Water`/
  `Garden`/`Cobblestone1`/`Cobblestone2` → `EditType` 枚举（Path, Cobblestone1..3,
  Garden, Water 的 constructor 已由 PDB 证实；其余为观察 tag）。
- `MultiEdit` 元素 40B：`edits`（Vec）@+0x00..0x18 + 三个辅助计数
  （decorators/stairs/…），空判定 `edits.len==0 && aux 计数==0`（O）。
- `HistoryChainSnapshot` 16B：`{payload: Arc<Mutex<T>>@+0x00, chain_index: i32@+0x08}`（O）。
- `HistorySnapshotId`：`from_snapshot_blob`/`from_file_stem`/`file_name`；
  HashSet<HistorySnapshotId> 键（O：hashbrown HashSet 过程出现）。

## I — 推断

- `MultiEdit` 的辅助容器归属 = decorators `diffs` / stairs 边对（观察 schema 的
  `decorators`/`stairs` 字段），未从运行时字段逐一确认。
- pending 桶与提交后 MultiEdit 同构（40B），`+0x30` 的 `i64::MIN` 哨兵 =
  “未录制”状态（add/start 的 neg/jo 检查）。
- `HistoryMutationToken` 由各编辑 owner 产生，`i_promise_there_were_no_mutations`
  内联进 finish_input_recording（O：PDB host 0x1472D00 区域）；Drop 未消费 active
  token 时 panic（来自 reconstruction/history/transaction.rs 观察模型）。

## U — 未知（typed gap）

- MultiEdit 三个辅助容器的精确 Rust 类型与字段（edits 的 inner 结构按 EditType）。
- snapshots payload `T` 的具体类型（Ghidra 将 `Arc<Mutex<…>>` 内层误标
  CrashMiscInfo；实为历史快照负载）。
- `+0x70` 字段语义；`History` 完整 serde 手写 impl 字段序。
- `HistoryChainSnapshots`/`ResolvedHistoryChainSnapshot` 精确布局。
- `auto_clutter_edit`/`color_edit`/`decorator_edit`/`path_edit`/`roof_edit`/
  `stairs_history_edit`/`terrain_edit`/`tree_edit`/`wall_compound_edit`/
  `wall_history` 各命令子类型字段（观察 schema 的开放枚举，非封闭）。
- `ReplayBuffer`（replay_cmd_buffer.rs）内部桶布局（前段 pop/make_animations_instant
  地址已定位，语义 U）。
