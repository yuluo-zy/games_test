# LOGIC — resources/history + systems/history 逐函数逻辑判等

> verifier: logic-history，2026-10-17。依据 2026-10-16-logic-verify-protocol.md。
> 每函数以 fresh 反编译为准（shared worker，batch 1-6，decomp 落盘
> `recovery/agent-team/verifier-history/decomp/`）。verdict：判等 / 未实现-需重做 / 胶水-非业务。
> 未实现（无 Rust body）单独登记 §4。

---

## 1. resources/history

### 1.1 mod.rs（History 类型 = recovered_building/history_transactions.rs 的 HistoryTransactions<EditType>）

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x140a84220 | `HistoryIdentity::generate` | **判等** | fastrand u64 全范围（`with<fastrand::Rng, RangeInclusive<u64>>`）；src `fastrand::u64(..)`。mutate 同源。 |
| 0x140b3b630 | `EditType deserialize::visit_str`（tag 表权威） | **判等** | 15 变体 tag 逐项命中：Terrain=0(7字符)/Path=1/Cobblestone1-3=2-4/Water=5/Garden=6/Wall=7(含 13字符 WallNew…)/Roof=8/Color=9/DecoratorV5=10/Decorator=11/Tree=12(4字符)/AutoClutter=13/Stairs=14(6字符)。src `native_tag()` 全表一致 ✓（RE-EDITTYPE 旧表 tag0=Tree 已被 fresh 证伪；src 现行表正确）。serde visitor 本身按协议为胶水，不审。 |
| 0x140a84ec0 | `History::add` | **未实现-需重做** | 分歧：**缺 tag-7 Wall 族校验分支**。原版在 pending 空 + cursor-len<-2 的 nuke 之后、push 之前，对 `*in_RDX==7` 的 Wall 复合编辑逐 group/逐操作 niche 解码（`+0x10` 判别），变体 0 断言 f32 高度对 `Slice start…`、变体 1/2 断言 `+0x18>=0`（`None is not allowed in CreateWa…/DeleteWa…`），违规 panic。src `HistoryTransactions::add`（generic）无任何墙校验（record_new_wall 只在创建时用 slice_source_curve 返 Err，删除/None 校验不在 add 链）。其余：pending<0 原版 panic "Tried writing…" vs src `Err(RecordingNotStarted)`（条件一致，错误通道建模差异）；24B 记录 `{tag, records, count}` push 内容等价（src push EditType 枚举）。 |
| 0x140a854a0 | `History::add_decorator_diff` | **未实现** | 无 Rust body（src 全库无 `fn add_decorator_diff`）。原版 4 参：楼梯 checksum 校验（`VERIFY_STAIR_CHECKSUMS` 跨域开关 + panic "Stairs graph history recording…"）、两 u32 边对循环 push tag 0xE/0x1 记录、Arc 替换装饰/楼梯两表。 |
| 0x140a86270 | `History::start_input_recording` | **判等-限定** | AppMode==Replay(0xd)→原版 panic vs src `Err(ReplayRecordingForbidden)`；否则换新空 pending。条件一致。 |
| 0x140a86510 | `History::try_start_input_recording` | **判等** | mode==0xd 或已在录→false，否则新建 pending→true。逐分支对应。 |
| 0x140a86730 | `History::finish_input_recording` | **判等-限定** | take pending；空（edits 空 && 3 aux 计数 0）→丢弃返回无提交；非空 push entries、nonce=fastrand、cursor+1、断言 cursor==entries.len-1（原版 assert_failed vs src `Err(CursorNotAtCommittedTail)`）。事件写出（observed_input_replay_command_tag）→ src 以 `HistoryCommit` 返回建模。 |
| 0x140a86970 | `History::is_recording` | **判等** | pending.is_some() 单字段。 |
| 0x140a86980 | `History::finish_potential_mutation` | **判等** | active 时清 resolved_snapshots（逐 Arc drop）+ nonce=fastrand 写 mutation_nonce（+0x60）。src 逐句对应。 |
| 0x140a84c50 | `History::nuke_future_states` | **判等-限定** | cursor!=len-1 时：cursor<0 → 全清（entries/resolved_snapshots/pending None/cursor=-1/count_58=0）+ 重新 alloc；否则截断 cursor+1 + `count_58=min` + `retain(chain_index<=cursor)`；最后 nonce=fastrand。原版截断处 assert_failed vs src Err(InvalidCursor)（不可能路径）。 |
| 0x140a844f0 | `History::insert_starting_edit` | **判等** | 断言 cursor>=-1（panic/assert），cursor+1、链首 insert、nonce、count_58=0。 |
| 0x140a84650 | `History::current_snapshot` | **判等** | 无分支二分 + 最近者选择（`dist[p]<=dist[p-1]→p else p-1`，p==0→0，p==len→最后）；选中 `chain_index==cursor` 才返回（否则 drop Arc）。src nearest_index_by/current_snapshot 逐分支一致。 |
| 0x140a84760 | `History::set_current_snapshot` | **判等** | 同一选择；已存在 cursor 快照→panic（assert），否则 partition 有序插入 + memmove。src 一致。 |
| 0x140a84a50 | `History::closest_resolved_snapshot` | **未实现-需重做** | 选择算法一致，但**过滤谓词**（filter_map 闭包，即 0x140a84260 `TryFrom` 规则：`[*+0x10] < 2` 才 resolved）**未实现**——src 改由调用方传 `is_resolved`（注释明示"原谓词规则未恢复"）。 |
| 0x140a84260 | `impl$9::try_from`（resolved 谓词） | **未实现** | 无 Rust body；规则 `[*in_RDX+0x10] < 2` 未还原。 |
| 0x140a84290 | `is_decorator_history_diff_empty` | **判等** | 机器码 `cmp [+0x18],0; sete al`。src `item_count==0`（参数面 = 调用方传入 +0x18 值，等价）。 |
| 0x140a842a0 | `is_stairs_history_diff_empty` | **判等** | 机器码 `cmp [+0x30],0` 且 `cmp [+0x48],0`（两容量）。src `a==0 && b==0`。 |
| 0x140a84ba0/0xc40 | `debug_display` / `debug_set_history_start` | **未实现** | 无 Rust body（调试工具，非业务主干，登记）。 |
| 0x140a84a40 | `iter_resolved_snapshots` | **胶水-非业务** | 16B 迭代器转发。 |

### 1.2 edit 结构（各 edit 构造/载荷——核心）

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x14003c5d0 | `ColorEdit::serialize` | **未实现-需重做** | fresh 反编译遇 "Control flow encountered bad instruction data" + 未收敛跳表，函数体按变体判别（`bVar9 = target-3`）分派多组 `serialize_entry`（载荷含 WallId/TerrainStrokeId/FlatRoofStyle×2/DecoratorId+f32×2/DecoratorId+bool×2/StairRailingType×2/StairSupportType×2/String…）——即原版按 **ColorTarget 变体写互异载荷**。src 为平面 `{target, from, to, target_id, chimney_h_offset, allow_connection}` 的 derive serde，与原版载荷无对应（target 判别载荷未对账，src 注释自认"字段序/类型待取证"）。判等不成立。 |
| 0x1404f46e0 | `DecoratorHistoryEdit::serialize` | **未实现-需重做** | 原版写单 entry map（`serialize_entry<str, __SerializeWith>`），载荷为 nested diff；src 平面 `{id, before, after}` 三字段 derive，结构不等。 |
| 0x140b3f930 | `DecoratorHistoryState::from`（= From<&DecoratorInfo>） | **未实现-需重做** | 原版完整 info→state 转换（style 字节分支读 6/8/9/0xb/0xc/0xd 字段 + is_preview(+0x47) 才 panic）；src 仅 `from_preview()` panic 占位，**转换逻辑缺失**。 |
| 0x140b3f910 | `is_default_stair_width` / `is_default_lantern_brightness` | **判等** | cmpeqss `x==1.0`（NaN false），ICF 同址。src 两个 `==1.0`。 |
| 0x1408c5680 | `is_default_chimney_smoke` | **未实现** | 无 Rust body；注意 fresh 解析为 `walls::inner_wall_state::StructureType::is_fence`（4B），inventory 该条目疑 PDB 错位/地址复用，登记待核。 |
| 0x140b3fab0 | `convert_before_after_states_to_changes` | **未实现** | 无 Rust body。原版按 style 字节分变体比较前后浮点输出变更集（含 NaN 语义），src 全无。 |
| 0x1417bdae0 | `send_state_diff_events` | **未实现** | 无 Rust body。replay_pop_bucket 消费链的装饰 diff 事件，src 全无。 |
| 0x1404896f0 | `RasterEdit::serialize` + `impl$1::from` | **未实现-需重做** | 原版 serialize 字段序 `points(Vec<Vec3>) / is_additive(bool) / brush_size(f32) / blend_mode(StrokeBlendMode enum)`；**src `points: Vec<Vec2>`、序 points/brush_size/is_additive/blend_mode(u8)**——点类型（3D vs 2D）、字段序、blend_mode 类型全不符。`From`(0x140a976b0, Arc<[Vec3]>→RasterEdit) 无 src body。 |
| 0x1416e8510 | `RasterEdit::deserialize` | **未实现-需重做** | 手写 visitor（deserialize_struct + Arc 建）；src 为 derive，字段 schema 不一致。 |
| 0x14003d4e0 | `TreeEditData::serialize` | **未实现-需重做** | 原版字段序 `position(Vec3)/scale(f32)/rot(f32)/<i32>/prefab(String)/is_preplaced(bool)`；src `{id:u64, position, scale, rotation_degrees, prefab_name, is_preplaced}`——**无 i32 位、id 型 u64≠i32、字段序不符**（id 应非首字段）。 |
| 0x1409e6210 | `missing_prefab` | **未实现** | 无 Rust body。原版返回 24B String（0x18 len + "MISSING_PREFAB…" 字节常量）。 |
| 0x140259af0 | `StairsHistoryEdit::serialize` | **未实现-需重做** | 原版按 `*in_RCX & 1` 分支写 `{"<名>": {StairGraphEdge…}}` 嵌套边集；src `{before/after/hash}` 平面 + hash，载荷结构不等（StairGraphEdge vs (u32,u32) 边对）。 |
| 0x140a18ee0 | `StairsHistoryEdit::deserialize` | **未实现-需重做** | 同上，手写 visitor vs derive。 |

### 1.3 auto_clutter_edit / replay_cmd_buffer / tree_edit 扩展

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| — | `AutoClutterReplayCmd::expanded_shared`（tag-13 展开，@0x14143fc1e 内联证据） | **判等** | tag 9 命令、duration 0x3dcccccd=0.1、elapsed 0、`timed=!instant`、共享 Arc。src 逐字段一致。 |
| 0x1409f1ab0 | `ReplayBuffer::make_animations_instant` | **判等-限定** | 遍历全部 bucket 按命令 tag 分派置零（raster case0-2/terrain case3/wall case4 内层/default）；src 按 ReplayCommand 变体置零。反编译遇坏指令，字段偏移无法逐字节核对（结构与方向一致）。 |
| 0x1409f18f0 | `ReplayBuffer::pop_front` | **判等-限定** | 原版 pop 前桶并写 0x30B 事件（tag 0x8000000000000000, +0x28=入口 tag）；src `pop_empty_replay_bucket` pop + 写 `ReplayBucketFinished`。事件载荷面有建模差异（原版还带 aux 计数，src 前置拒绝非零 aux）。 |
| 0x140949170 | `ReplayBuffer::len` | **判等** | 5B 返回 buckets.len()。 |
| 0x1409f1a80 | `ReplayBuffer::front_mut_bucket` | **判等** | 39B 判空返回 front。 |

### 1.4 wall_compound_edit（CompoundEditBuilder，REVIEW10 已核结构；本审逻辑）

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x140a82b20 | `new_edit_sequence` | **判等-限定** | fresh 27B：仅写 3 qword（`[0]=0,[1]=8,[2]=0`，Vec 复位/置空）——**未见 started 标志写**。src `started=true; current.clear()` 的 started 置位在原函数体中无对应（若原 started 由调用方管理则为建模差异）。 |
| 0x140a82b40 | `next` | **判等-限定** | fresh：拷贝 in_RDX（current ConcurrentEdits 0xf8B）→Box(0x108)→push groups，**无空检**（无条件 push）。src `!current.is_empty()` 守卫——若原调用方保证仅非空调用 next 则等价，否则空组 push 行为有差异。 |
| 0x140a82d50 | `build` | **判等-限定** | fresh：reset groups Vec 后 `from_iter<ConcurrentEdits, Map<IntoIter<Vec<Vec<Arc…>>>>>` 收组——**未见调用 next()、未见 started/groups 空检查**。src `build` 先 self.next() + `!started||groups空→None`。语义近似（收组→Option<WallCompoundHistoryEdit>），但空/未 started 判定与原反编译不对应。 |
| 0x140a82e20 | `panic_deprecated_operation` | **判等** | panic_fmt 文案待取证；src panic!("attempted to use a deprecated wall history operation")。 |

### 1.5 wall_history.rs（ev_new_wall / ev_height_changed / new_height_changed）

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x140a83300 | `ev_new_wall` | **判等-限定** | ratio=0.2/len clamp[0,1]（0 长 panic、越界 panic、NaN panic）、crashy_slice、seed Create group + StateChange group、`History::add`(tag 7)。src `record_new_wall` 同构（slice_source_curve Err 化；其余等价）。 |
| 0x140a83f80/0x140a83fb0 | `ev_height_changed` / `new_height_changed` | **判等-限定** | new_height_changed 构造 tag 7 + 判别 0x8000000000000002 复合编辑 → add。src 无同名；以 AdjustWallHeightCmd 域迁移（登记对应面）。 |

---

## 2. systems/history

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x140af5b30 | `history_process`（src `process_history_cursor_commands`） | **未实现-需重做** | 普通 undo/redo 分支判等：pop HistoryCmd → interrupt(`byte0==2||byte1==4`) → 写 consumed 事件 → undo(cursor>0 && >undo_anchor && <len 才 -1)/redo(`<-1` panic、`len<=next` 跳过、+1) → replay_edit。src 逐分支对应 ✓（consumed 事件 src 以 ReplayBucketFinished 类事件建模）。**分歧：第二事件流（16B 记录 byte[+9] 跳表 0x142ae5420，case 0..0x17 校验/动画即时化/移除）未实现**（src 注释明示）。 |
| 0x140af6170 | `replay_edit`（src `ReplayBucket::expand` + cursor_step_expanding） | **未实现-需重做** | 原版逐 edit `generate_replay_cmds_from_edit`（按 tag 跳表 0x142c5f284 分派）+ undo 逆序 + 调度为 ReplayBucket + **装饰/楼梯 diff Arc 处理**（to_arc_slice）。src expand 覆盖 Raster/AutoClutter/Roof/Wall/Stairs/Tree；**缺 Color(9)/Decorator(10/11) 命令展开**（返回 UnrestoredReplayOwner）、Terrain 显式拒绝、装饰/楼梯 diff 以 `observed_auxiliary_counts != [0;3]` 拒绝而非处理。 |
| 0x14143fb50/0x1414423e0 | `generate_replay_cmds_from_edit` | **未实现** | 仅跳表壳（60B 按 `*in_RCX` tag 分派）；各 tag 命令生成器 src 以 expand match 部分替代（缺 Color/Decorator/Terrain 全量）。 |
| 0x140b38250 | `replay_pop_bucket` | **未实现** | 无 Rust body（src 仅 `pop_empty_replay_bucket` 拒绝非零 aux）。原版：前桶命令空 → pop_front + restore_decorators + Stairs restore_backup/apply_diff/make_backup + flush_colliders。 |
| 0x1408b36a0 | `finish_replay` | **未实现-需重做** | 原版：AppMode==0xd 且双队列空 → **RestoreAppTo 写回 AppMode**（+ 哨兵弹栈 -0x7fffffffffffffff）→ Toolbar::switch_mode / CmdUnfreezeSheep（"History replay has naturally finished. Unblocking game commander."）→ pop MetaCommand。src 仅 `is_replay_naturally_finished`（replay 空 && pending 空）+ log；RestoreAppTo/MetaCommander/工具栏/UnfreezeSheep 全部宿主登记未实现。 |
| 0x140a7c550 | `replay_mode` | **未实现-需重做** | 原版系统：读 HistoryCmd 队列、AppMode 进出 0xd 检测（DAT_143634d88 标志）、clearing_canvas_w_layer_and_sound、**HoldToFastReplay 阈值 0.2**（`fVar24<=0.2` 保持，`0.2<` 触发）、time_continuous_lerp、写事件、AppMode 恢复。src 仅 `is_replay_mode(mode.native_tag()==13)` + `replay_mode_transition` 纯谓词 + **FAST_REPLAY_HOLD_MS=500（≠原 0.2s）**；系统本体未实现。 |
| 0x140aa4a60 | `replay_auto_clutter`（src `advance_auto_clutter`+replay_auto_clutter） | **判等-限定** | 前桶 tag9 命令：elapsed==0 首应用（undo==0→insert/undo!=0→erase）；instant（+8 bit0）→完成；否则 `duration/speed` clamp 下限 0.05（`fVar30<=0.05→0.05`，MAXSS NaN 语义）、`elapsed>阈值` 才完成、后 `elapsed+=dt`；逆序移除。src 逐句一致。限定：删除集合 src 用 `DeletedAutoClutter(HashSet<u64>)` vs 原版 `HashMap<WallId,()>`+`erase<DecoratorId,DecoratorId>`（消费者面建模）。 |
| 0x140af75b0 | `replay_stairs`（src `StairsReplayCmd::advance`+replay_stairs） | **未实现-需重做** | 分歧：**动画门控下限原版 0.05，src `REPLAY_DURATION_FLOOR=0.1`**（src 注释自标 U，fresh 明确 `fVar44<=0.05→0.05`）。其余：tag10 命令、非 instant 时 `elapsed<=max(dur/speed,0.05)` 仅推进、就绪后 undo==0→insert 边集/!=0→remove_entry、lookup_decorator_info + type 字节 `|0xf8>0xf9` 过滤、CachedDecoratorTransforms 查位置、Push PlayStairsAudioCmd、逆序移除——src 对应 ✓（装饰过滤字段面 src 读 dst_payload>>8 vs 原 +0x41，待 DecoratorInfo 布局对账）。 |
| 0x140951b80 | `replay_wall`（src replay_wall → advance_wall_groups_observed） | **未实现-需重做** | 原版：前桶 tag4 命令，逐命令 `WallReplayEditCmd::process`（0x140abfb30，done 字节+0x15、非完成按跳表 0x142adf44c/0x142adf408 分派 17 操作 + cubic_in_out），组完成 → insert 墙集 + **is_disjoint 相邻组墙集守卫** + 移除。src 以 `advance_wall_groups_observed`（wall_history_replay.rs）解释 `WallHistoryEdit`，**仅 EnclosureChange/StateChange(op0) 有实现**，其余操作返 `UnrestoredOperation`——原版 17 操作跳表分派未还原（recovered_building 共享面，本审登记 systems 侧结论）。 |
| 0x140abfb30 | `WallReplayEditCmd::process` | **未实现-需重做** | 见上：done 字节 + 跳表分派（cubic_in_out 已实现于 src advance），操作分支主体在 recovered_building 未逐操作对账。 |
| 0x141700430/0x141700620 | `RasterReplayCmd::forward` / `backward` | **判等** | pair-unrolled 距离和（无上限）、`(len/5)*0.025`、floor 0.4、NaN 保留；构造 cmd（stroke/domain/undo/duration/instant）。src `duration_based_on_stroke_length` 与 forward/backward 逐句一致。 |
| 0x1409f4650 等 6 实例 | `replay_raster<T>`（src `advance_raster_replay`/replay_rasters） | **判等-限定** | 未 started→断言点数非空 + 置 started + undo 时 clear_buffer(0) + **按 History 条目回溯重绘至 Arc 同一性为止**；`duration<0.01`→立即 paint+反馈+dirty+stage；动画→`elapsed+=speed*dt`、progress clamp、cubic_in_out、slice_into 0..t、dirty 交换、copy_to_staging、feedback、`progress>=1&&!undo` commit。src 逐段对应 ✓。限定：6 实例合并为单系统（原版各自 tag 过滤）；原版 SsboId 上传 vs src 泛型 upload 面。 |
| 0x140ae5830 | `get_param`（replay_raster 资源取参） | **未实现** | 无 Rust body；原版 4 索引稀疏表取 4 组参数否则 panic "Resource requested by…"。 |
| 0x1416286d0 | `generate_path_replay_cmds` | **判等-限定** | niche 0..3 过滤 → 尾调共享生成器；src 由 expand `raster()` 六域覆盖（Path/C1-3/Water/Garden）。 |
| 0x141695e10 | `record_garden_history` | **未实现** | 属 systems/garden（logic-ecob 范围）；登记：原版 pending 清空后 Arc<RasterStroke>→`History::add`。 |
| 0x140ac9770 | `history_record_terrain` | **未实现** | 属 systems/terrain（logic-terrain 范围）；登记：原版写 0x38B 记录（判别 0x8000000000000002）+`History::add`。 |
| 0x1409dd790 | `history_finish_recording` | **未实现** | 无 Rust body（src systems/history/recording.rs 为 roof 录制适配器，非原版 finish_recording）。 |
| 0x140b1b860/0x140b1d680 | `signifier_replay_active` / `register_required_components` | **未实现** | 无 Rust body。 |

---

## 3. 汇总

- **判等（含限定）**：HistoryIdentity::generate/mutate、EditType native_tag 表、start/try_start_input_recording、finish_input_recording、is_recording、finish_potential_mutation、nuke_future_states、insert_starting_edit、current_snapshot、set_current_snapshot、is_decorator_history_diff_empty、is_stairs_history_diff_empty、is_default_stair_width/lantern_brightness、CompoundEditBuilder 三函数（限定：started/空检/next 调用面与 fresh 有出入）、panic_deprecated_operation、AutoClutterReplayCmd::expanded_shared、ReplayBuffer len/front_mut_bucket/pop_front、RasterReplayCmd forward/backward、replay_auto_clutter、replay_raster advance、generate_path_replay_cmds、record_new_wall/ev_new_wall。
- **未实现-需重做**：History::add（缺 tag-7 墙校验）、closest_resolved_snapshot 谓词、finish_replay、replay_mode、replay_stairs（0.05≠0.1）、replay_wall/process（17 操作未还原）、history_process 第二事件流、replay_edit/expand（Color/Decorator/Terrain 命令）、ColorEdit、DecoratorHistoryEdit、RasterEdit、TreeEditData、StairsHistoryEdit 载荷结构、DecoratorHistoryState::from。
- **未实现（无 body）**：add_decorator_diff、send_state_diff_events、convert_before_after_states_to_changes、missing_prefab、is_default_chimney_smoke、replay_pop_bucket、get_param、debug_display/debug_set_history_start、history_finish_recording、signifier_replay_active×2、replay_edit 生成器主体、tree/stairs 各 From。

**核心结论**：EditType 判别与 History 游标/快照/录制链为真实现（逐句判等）；**History::add 缺 Wall 复合编辑的 tag-7 入史校验**是用户硬标准下的实质缺口；**各 edit 载荷（Color/Raster/Tree/Decorator/Stairs）均为 src 平面近似，与原版 target/style/变体分派载荷结构不符**，属"结构对上、载荷假"一类；replay 族中 auto_clutter/raster 判等，wall/stairs/process 未判等。
