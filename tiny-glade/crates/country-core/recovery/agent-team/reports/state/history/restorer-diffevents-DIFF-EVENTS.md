# DIFF-EVENTS — history 装饰/楼梯 diff 事件链（restorer-diffevents，2026-10-17）

> 依据 2026-10-16-logic-verify-protocol.md（只以 fresh 反编译为准）。
> fresh 证据：ghidra shared 批次 `impl-history-diffevents/results/batch1-fresh.json`
> + `read_memory` 跳表 `batch13/15` + `disassemble_function`（0x140b3fab0、0x1417bdae0、
> 0x140a854a0）+ r2 原始字节核对（0x1417bdf2f/0x1417bdba0/0x1417be0f2/0x140b3fb80/
> 0x140b3fcac）。落盘 `impl-history-diffevents/decomp/`。
> 红线遵守：缺跨域字段登记不编造；只改 history 域（含 History 容器
> `recovered_building/history_transactions.rs`）；宏胶水不重建。

---

## 1. `send_state_diff_events` 0x1417bdae0（无 body → 实现）

### 1.1 签名/调用链（fresh）

- 调用点：`DecoratorStorage::restore_decorators<closure_env_0>` 0x14155d13f。
  `RCX = &HashMap<DecoratorId, DecoratorBeforeAfterWithStorage>`（栈上 0x30B map 拷贝），
  `RDX = 捕获环境`，函数内 `R8 = RDX; RCX = [R8]` 后作为
  `DeferredDecoratorOpEvents::send(0x1408336b0)` 的 self。
- 迭代：hashbrown RawTable 步进 **0xc8**（key `DecoratorId` u32 @+0x00，值 0xc0）。
  值类型 `DecoratorBeforeAfterWithStorage = {before: 0x60B, after: 0x60B}`；
  单侧 `DecoratorStateWithStorage = {storage_disc u64 @+0x00,
  storage_payload u64 @+0x08, DecoratorHistoryState @+0x10 (0x50B)}`。
  `restore_decorators` 0x14155c228 调 `impl$1::from` 产出 0x50B 状态并放在
  值 `+0x10`；另一侧判别写 3（`MOV [RBP+0x220],3`）。
- 事件：0x88B 记录，末尾 `DeferredDecoratorOpEvents::send` 入队。

### 1.2 分派（fresh disasm 逐分支）

| before.storage_disc | after.storage_disc | 行为 |
|---|---|---|
| 4 | 任意 | **立即 return**（终止迭代，0x1417bde0a JZ 0x1417be152） |
| 3 | 3 | `unreachable!()`（0x1417be1ce → "internal error: entered unreachable code"，文件 decorator_edit.rs） |
| 3 | ≠3 | 新增事件：`+0x00=3`，`+0x08/+0x10=after 存储槽`，`+0x18..0x3f=after 状态前 0x28B`，`+0x40=DecoratorId`，`+0x44=1`，`+0x45=after rank`，`+0x7c=1`，`+0x81=after ty` |
| ≠3,4 | 3 | 移除事件：`+0x00=4`，`+0x08/+0x10=before 存储槽`，`+0x18..0x3f=before 状态前 0x28B`，`+0x40..0x5f=风格簇`（按 `clamp(ty-0x38)=0..7` 跳表 0x142ce2360 分派，见 §1.3），`+0x60=id`，`+0x7c=1`，`+0x81=ty` |
| 其他 | ≠3 | `convert_before_after_states_to_changes`；事件 `+0x00=before 判别`，`+0x08/+0x10=before 存储槽`，`+0x10..0x67=changes(0x58)`，`+0x68=id`，`+0x7c=1`，`+0x81=before ty` |

### 1.3 移除事件 ty 跳表（case = clamp(ty-0x38)，ty 0x38..0x3f；>=0x40 → 6）

公共字段（before 状态）：`+0x40=u28`、`+0x44=decorator_id`、`+0x48=case 字段`、
`+0x4c=case 字节`、`+0x50=u40`、`+0x54=u44`、`+0x58=u4b`、`+0x59=case tag`、
`+0x5c=rank`、`+0x5d=u49`、`+0x5e=ty`。

| case | ty | +0x48 字段 | +0x4c 字节 | +0x59 |
|---|---|---|---|---|
| 0/1/2 | 0x38/0x39/0x3a | before.u34 | after.u20 低字节（EDI） | 6 |
| 3 | 0x3b | before.u30 位 | before.u4d | 4 |
| 4 | 0x3c | before.u38 位 | after.u20 低字节 | 7 |
| 5 | 0x3d | before.u4c | after.u20 低字节 | 5 |
| 6 | 0x3e 或 >=0x40 | before.u34 | after.u20 低字节 | 3 |
| 7 | 0x3f | before.u30 位 | before.u3c 低字节 | before.style |

（0x1417bdf2f/0x1417bdba0/0x1417be0de/0x1417be12b/0x1417be146/0x1417be0f2 逐字节核对。）

### 1.4 落地

`resources/history/decorator_edit.rs`：
`DecoratorStateWithStorage`（0x60B）、`DecoratorBeforeAfterWithStorage`（0xc0B）、
`DecoratorStateChanges`（0x58B，显式偏移 `to_bytes`）、`DecoratorDiffEvent([u8;0x88])`
（`from_changes`/`from_added`/`from_removed_with_after`）、
`send_state_diff_events(&HashMap<...>, &DeferredDecoratorOpEvents)`。

**登记 U（不编造）**：原生事件 `+0x6c..+0x70`、`+0x4d..+0x50`、`+0x41..0x44` 等
字节在三条路径中**从未初始化**（fresh 直接读未写栈槽），Rust 侧置 0 并标注 padding；
`before.storage_disc==4` 的清空语义按“终止迭代”实现。

---

## 2. `add_decorator_diff` 0x140a854a0（无 body → 实现）

fresh 参数：`RCX=History`、`RDX=&DecoratorHistoryDiff`（0x30B map）、
`R8=&StairsHistoryDiff`（0x60B）、`R9=&mut HashMap<StairGraphEdge,()>`
（调用点 `history_finish_recording` 0x1409dd9bc；`RCX=RSI`、`R9=[RAX]`）。

逐段（0x140a854a0 disasm 500 条）：

1. pending 门：`[History+0x30] < 0`（未录制）→ panic
   "Tried writing decorator history…"（0x142ad83c0）。
2. `cursor(+0x68) - committed(+0x10) < -2` → `nuke_future_states`（0x140a84c50）
   + 重建空 MultiEdit（alloc 0x68B：记录 Vec + checksum HashMap + 两个 Arc 槽）。
3. `VERIFY_STAIR_CHECKSUMS`（全局字节 0x14362c011）==1 且 stairs diff 判别
   `+0x00 == 1`：倒序找最近“首记录 tag==1”条目，比较 `record+0x28` checksum 与
   `stairs_diff+0x08`；不等 → log "Stairs diff …" + panic
   "Stairs graph history recording…"（0x142ad8310）。
4. 两 u32 边对循环（排序 lo/hi，`StairGraphEdge::new`）：
   - 原生 Vec@+0x40/+0x48：只 probe `R9` 集合，**命中** → Arc 0x20B 记录
     `{tag 0xE, variant 0 = SupportsDelete, edge @+0x14}`；
   - 原生 Vec@+0x28/+0x30：`remove_entry`，**移除成功** → `{tag 0xE,
     variant 1 = SupportsRecreate, edge}`。
   每条记录 push 到 pending（0x18B `{tag,records,count}`；Rust 以逐
   `EditType::Stairs(Arc<StairsHistoryEdit>)` 建模）。
5. Arc 替换（原序）：`History+0x48 = Arc::new(RDX[0..0x30])`（drop 旧
   `Arc<DecoratorHistoryDiff>`）→ `History+0x50 = Arc::new(R8[0..0x60])`
   （drop 旧 `Arc<StairsHistoryDiff>`）。

落地：`HistoryTransactions<EditType>::add_decorator_diff`（decorator_edit.rs），
容器新增 `decorator_diff`/`stairs_diff` 两个 Arc 槽（`history_transactions.rs`）。

**登记 U（跨域）**：
- `VERIFY_STAIR_CHECKSUMS` 原生是 stairs 域可变全局 → 以参数注入；
- 原生 `StairsHistoryDiff` 0x60B 布局为 `{u32 tag@+0x00, u64 checksum@+0x08,
  未知@+0x10/+0x18/+0x20, Vec@+0x28, Vec@+0x40, @+0x58}`；当前 stairs 域类型
  `{before_checksum, after_checksum, removed, added}` 为近似，Vec 字段与
  +0x40/+0x28 的精确对应、tag 字段由 stairs 域后续对齐（本实现按
  added→SupportsDelete、removed→SupportsRecreate 语义映射）。
- 条目 checksum 槽以 `HistoryEntry.observed_stairs_checksum: Option<u64>`
  建模（原生为 plain u64，地址=记录 +0x28）；无 checksum 的旧条目跳过比较。

---

## 3. `convert_before_after_states_to_changes` 0x140b3fab0（无 body → 实现）

fresh：`RCX=out(0x58B)`、`RDX=before 0x60B`、`R8=after 0x60B`。

1. 存储槽相等：`(before.disc as u32)==(after.disc as u32)` 且
   `(after.disc as u32 != 0 || before.payload == after.payload)`（Wall=0 才比较
   WallId；Terrain/Stair 不比较）。
2. 变体 selector：before `tag→(tag-2<3 ? tag-1 : 0)`，after
   `tag→(tag<2 ? 0 : tag-1)`；不等 → 变更。`tag = 状态 +0x22`（dst tag，
   **不是 style 字节**；旧任务描述为 style 有误，fresh 以 +0x32 半槽偏移 =
   state+0x22 为准）。
3. 跳表 0x142aef050（sel 0..3）：
   - sel 0（tag 0/1 等）：f32 比较 state+0x10/+0x14/+0x20/+0x24/+0x28/+0x2c
     （UCOMISS，NaN 视为不等）+ byte +0x30 + qword +0x18 + tag 相等 + byte +0x31；
   - sel 1/2（tag 2/3）：f32 +0x10/+0x14/+0x18；
   - sel 3（tag 4）：**整数** dword +0x10/+0x14 + f32 +0x18/+0x1c/+0x20。
4. 全等 → `out+0x00=3`，否则 `= after.disc`；`+0x08=after.payload`；
   `+0x10..0x37 = after 状态前 0x28B`；`+0x38=after rank`；
   `+0x40 = CMPNEQSS(after.u30,before.u30)`（NaN→1）、`+0x44=after.u30`；
   `+0x48 = CMPNEQSS(after.u3c,before.u3c)`、`+0x4c=after.u3c`；
   `+0x50=rank 变更`、`+0x51=after rank`、`+0x52=id 变更`、
   `+0x53=(after_id-before_id) 低字节`、
   `+0x54=(u4c 相等?2:after.u4c)`、`+0x55=类型标记`
   （clamp(ty-0x38) 不同 → after.ty；相同且 (clamp!=6 或 ty 相等) → 0x40；
   否则 after.ty）。

落地：`DecoratorStateChanges` + `convert_before_after_states_to_changes`。
**登记 U**：全等路径 `+0x33/+0x37` 原生未写（读未初始化栈），Rust 以 after 字节
填充（等价数据）。

---

## 4. `DecoratorHistoryState::from` 0x140b3f930（重做 → fresh 复核判等）

fresh 逐写复核（0x140b3f930 2746B）：

- 入口 `info+0x47 == 0` else panic "Attempting to capture a
  DecoratorHistoryState of a preview"；
- style=info+0x41（`u40` byte1）；`b12=clamp(style-2,0..5)`；
- 直拷 `+0x00..0x28`（u28 = `(decorator_id<<32)|u28`）、rank/u45/ty；
- 派生簇：`+0x30`（style 4/8+/<2）、`+0x34`（Window 6）、`+0x38`（Lantern 7，
  否则 1.0）、`+0x3c/+0x40/+0x44/+0x4b/+0x4e`（`(byte)(style-8)<0xfa`）、
  `+0x4c`（Flag 5）、`+0x4d`（Chimney 4）。

结论：现有 `impl From<&DecoratorInfo> for DecoratorHistoryState`（含
`u30_hi`/`(style-8)<0xfa`/NaN 无关整数派生）与 fresh 逐字段一致，**判等**；
`to_info` 保留既有 U 标注。另：0x140b3f930 在 `restore_decorators`
0x14155c228 被调用来生成 0x60B 槽内 0x50B 状态，进一步确认类型归属。

---

## 5. walls `restore_decorators` 接口语义（供 walls 接线）

`replay_pop_bucket` 0x140b38250 链（fresh）：

1. 前桶命令空 → `ReplayBuffer::pop_front`（0x1409f18f0）；
2. 若 aux decorator diff Arc 非空 → `DecoratorStorage::restore_decorators`
   （0x14155ba30，闭包 = `send_state_diff_events` 消费者）→ 逐条
   `remove_collider<DecoratorId>`（0x140b409b0，返回 `Some(handle)` 时调用方
   `Arc<RaycastMeshData>` 递减；miss=0x11 保留）→ 递减并 drop `Arc<diff>`；
3. 若 stairs diff Arc 非空 → `restore_backup`/`apply_diff`/
   `iter_edges_and_stubs`/两张 IndexMap clear/`+0x178=0`/`make_backup`；
4. drop ReplayBucket → `DecoratorArchivist::extract_diff`（0x140b2cb30）
   → drop 0x30B map → `RaycastWorld::flush_colliders`（0x1408cd5c0）。
   命令非空路径同样 extract_diff。

walls 侧接线契约：
- `restore_decorators(&mut self, addr, diff: &DecoratorStorageHistoryDiff,
  events: &DeferredDecoratorOpEvents, colliders: &mut RaycastWorld)`：
  - 应用 before/after（既有 `apply_storage_diff` 语义）；
  - 需构建每 id 的 `DecoratorStateWithStorage`：`storage_disc/payload` =
  `DecoratorStorageAddr` 两字（WallId 仅 Wall 变体比较），`state` =
  `DecoratorHistoryState::from(&DecoratorInfo)`；
  - 一侧缺失写 `storage_disc = 3`；整批完成后调用
  `send_state_diff_events(&map, events)` **一次**（与原生闭包调用点一致）；
  - 原生闭包在每装饰器前后按 before/after 判别 2–4 次调用
  `remove_collider<DecoratorId>`（collision HARDPOINT2 已落地接口：
  `remove_collider(world, map, key) -> Option<Handle>`，命中后调用方递减
  自有 `Arc<RaycastMeshData>`；miss tag 0x11 保留 Arc）。
- `add_decorator_diff` 的 aux Arc 槽（History+0x48/+0x50）现已存在于
  `HistoryTransactions`；ReplayBucket 的 aux diff Arc 承载仍由 replay 域接线。

---

## 6. 编译与测试

- 本模块新增 13 个单测（`resources/history/decorator_edit.rs::tests`）：
  `state_and_slot_native_sizes`、`convert_marks_equal_state_as_3_and_type_marker_40`、
  `convert_storage_payload_compared_only_for_wall_disc`、
  `convert_nan_is_a_change_and_uses_after_value`、
  `convert_variant_tag4_compares_wall_words_as_integers`、
  `convert_reports_rank_id_and_type_changes`、
  `changed_event_layout_matches_fresh_offsets`、
  `added_event_uses_kind_3_and_after_state_prefix`、
  `removed_event_dispatch_cases_capture_style_cluster_fields`、
  `send_state_diff_events_dispatches_all_three_kinds`、
  `add_decorator_diff_pushes_tag_e_records_and_replaces_arcs`、
  `add_decorator_diff_panics_on_checksum_mismatch`、
  `add_decorator_diff_requires_recording`（另有既有 6 测试）。
- `cargo check -p country-core --lib`：**通过**（Finished，12 warnings 均属他域既有）。
- `cargo test -p country-core --lib resources::history`：**22 PASS / 0 failed**
  （含本模块 18：13 新增 + 5 既有；及 history 其余 4）。
- `cargo test -p country-core --lib systems::history`：**20 PASS / 0 failed**。
- 期间工作树曾被他域并发 WIP（ui_clutter/terrain/audio/save 测试）阻断，均已恢复；
  最终结果以上述两次运行为准。

---

## 7. 汇总

- **实现**：`send_state_diff_events`（三分派 + ty 跳表 + 0x88B 事件）、
  `convert_before_after_states_to_changes`（变体/NaN/存储槽语义）、
  `add_decorator_diff`（pending/nuke/checksum/两边对循环/Arc 替换）、
  `DecoratorHistoryState::from` fresh 复核判等。
- **类型**：`DecoratorStateWithStorage`(0x60)/`DecoratorBeforeAfterWithStorage`(0xc0)/
  `DecoratorStateChanges`(0x58)/`DecoratorDiffEvent`(0x88)/
  `DecoratorHistoryDiffMap`(0x30)；History 容器新增两 Arc 槽。
- **登记 U（解锁条件）**：
  1. `VERIFY_STAIR_CHECKSUMS` 全局（stairs 域）→ 参数注入；stairs 类型布局
     0x60B 对齐后切换到精确字段；
  2. 旧 `DecoratorHistoryDiff{before,after}`（0xa0）与原生 0x30B map 不一致 →
     `DecoratorHistoryEdit` wire 负载更新须随装饰 serde 线；
  3. 原生事件未初始化 padding 字节置 0；
  4. `restore_decorators` 闭包内 before/after 配对的 walls 域装配（本报告 §5 契约）。
- **阻塞**：无（最终 `cargo check -p country-core --lib` 通过；history 42 测试全绿）。
