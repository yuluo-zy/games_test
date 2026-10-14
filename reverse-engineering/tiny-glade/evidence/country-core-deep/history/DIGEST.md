# history 域深挖 DIGEST（2026-10-14，agent tg_a1）

B 版（EXE sha256 cf048e27…）。证据来源：RE-MCP 批量反编译
`evidence/country-core-deep/history/decomp/decomp-<va>.txt`（61/64 成功）、
`evidence/render-architecture/public-symbols.json`（S_PUB32 权威名表，101,405 条）、
rne-read 原字节/胖指针直读、`sysres-procedures.json`（PDB 编译单元元数据，**本域
多处名/址错位**，见 §7）。

> 关键结论先行：**本域 sysres-procedures.json 的“名→址”配对存在系统性错位**。
> 权威名字一律以 public-symbols.json（S_PUB32）为准；本 DIGEST 的地址以
> public-symbols.json 为准，sysres 仅作线索。

## 1. 模块清单

| 文件（reconstruction 对应） | 业务过程数 | 说明 |
|---|---|---|
| `resources/history/mod.rs` + `history_chain` | 17 个公开方法 + 4 个本地小函数 | History 权威缓冲核心 |
| `resources/history/wall_history.rs` | ev_new_wall / ev_height_changed / EditType::new_height_changed | 墙历史事件 |
| `resources/history/wall_compound_edit.rs` | CompoundEditBuilder（new_edit_sequence/next/build）、panic_deprecated_operation、WallHistoryEdit Debug/serde 族 | 墙复合编辑 |
| `resources/history/decorator_edit.rs` | is_default_lantern_brightness（=is_default_stair_width 同址）、DecoratorHistoryState::From、convert_before_after_states_to_changes、serde visitor 族 | 装饰编辑 |
| `resources/history/terrain_edit.rs` | Debug::fmt、revert、reapply、（本地）reapply::closure | 地形编辑 |
| `resources/history/roof_edit.rs` / `color_edit.rs` / `path_edit.rs` / `auto_clutter_edit.rs` / `stairs_history_edit.rs` / `tree_edit.rs` | serde 手写 visitor 族（各 1-3 个） | 编辑类型合同 |
| `resources/history/replay_cmd_buffer.rs` | ReplayBuffer::len/pop_front/front_mut_bucket/make_animations_instant | 回放命令缓冲 |
| `systems/history/*` | history_process、replay_edit、HistoryCmd::interrupt、generate_replay_cmds_from_edit×4、generate_path_replay_cmds、replay_wall、WallReplayEditCmd::process、_deprecated_morph_curve、replay_terrain(+forward/backward)、replay_stairs、replay_auto_clutter、replay_raster×6、RasterReplayCmd::forward/backward、replay_mode、finish_replay、replay_pop_bucket、signifier_replay_active(+register_required_components)、history_start_recording、add_history_systems/resources/events（注册胶水） | 回放/录制系统族 |

批量反编译 64 个目标：61 ok；3 失败——`0x140a84c40`（debug_set_history_start，
3 字节，已原字节解）、`0x140af5b20`（HistoryCmd::interrupt，6 字节，
`b8 00 02 00 02 c3` = 返回 0x02000200 ✓）、`0x1416286d0`（Ghidra 无函数，
已原字节解为跳板，见 §3.10）。

## 2. 布局表

### 2.1 History（0x78B，已有权威，引用 reconstruction mod.rs 模块文档）

| 偏移 | 字段 | 备注 |
|---|---|---|
| +0x00/+0x18/+0x30 | edit_chain / snapshots / pending | cap-first Vec；元素 0x28/0x10/40B |
| +0x58 | 计数 | nuke 时 clamp |
| +0x60 | mutation_nonce | |
| +0x68/+0x6c/+0x70 | chain_index / history_start / tail u32 | |

新增证据：`add` 0x140a84ec0 第三参数为 **track-caller 文件路径 `&'static str`**
（panic 格式化用：`FUN_1428d9390(&args, param_3)` 中 param_3 = 文件名）；
ev_height_changed 调用 add 时 r8 传一个指向 .rdata 的静态（0x142ae7818，
内联串区域）。

### 2.2 ReplayBuffer / ReplayBucket / ReplayCmd（新证据）

- **静态实例**：`0x142ae5430`（.rdata 内初始化数据；history_process 直接引用）。
- **ReplayBuffer 头部**（replay_pop_bucket 0x140b38250 反编译）：
  `+0x00..+0x18` = 桶 Vec（cap-first，**桶元素 0x40=64B**），`+0x18` = 活跃计数
  （`puVar3[3] != 0` 才有桶可弹）。reconstruction 文档头的
  “环形 {head,buf,cap,len,modification_counter}” **与证据不符**（见 §5）。
- **ReplayBucket（0x40B，O）**：`{decorators Arc@+0x00, stairs Arc@+0x08,
  cmds Vec@+0x10..+0x28（cap-first，元素 0x30=48B）, @+0x28..+0x40 其余}`。
  证据：replay_pop_bucket 读前桶 `+0x10`（cmds len）判空后 `pop_front` 返回
  `(decorators Arc, stairs Arc)` 两指针；replay_wall 0x140951b80 用
  `桶+0x08/+0x10` 迭代 0x30 步长 cmds。
- **ReplayCmd（0x30B）**：判别 u8@+0x00、instant@+0x08、duration f32@+0x0c、
  负载@+0x10、elapsed@+0x18、undo@+0x1c（各 replay 系统一致读，与
  reconstruction 的 ReplayAnimationHeader 一致）。
- **history_process 内第二张跳表 `0x142ae545c`（round2 已逐项解码）**：ReplayCmd
  判别字节 0-9 的 **drop/清理分派表**（= drop_in_place<ReplayCmd> 的 match；
  case 0xa/0xb 指向表尾数据=无效；`10 < tag` 走 OOM 错误路径 0x140af6660，
  即有效判别 0..=10）。第三张跳表 `0x142ae5420`：第二事件流 16B 记录
  byte[+9] 的 28-case 命令分派（0x0/0x1=末命令标志字节门控、0x2=跳过、
  0x3=make_animations_instant、0x4-0xe=校验/合并、0xf-0x18=命令移除/清理簇
  0x140af7d88-0x140af7e9b、0x19-0x1b=表尾数据无效）。两表均为编译器生成
  静态只读数据，详见 round2/history-CLOSURE.md §4。
- `generate_replay_cmds_from_edit`×4 的跳表：`0x142c5f248`。

### 2.3 装饰/楼梯 diff 相关

- **DecoratorDiffTable（64B）**：模板指针 0x142af1460/0x142af1470（0x142af1470
  处字节为 `60 14 af 42 01 00 00 00` = 指向 0x142af1460 的指针）。
- **add_decorator_diff 真实签名（O，4 参）**：
  `(&mut History, &DecoratorDiff, &StairsDiffTable, &HashRemap)`。
  param_4 = hashbrown RawTable（+0x00 ctrl、+0x08 bucket_mask、+0x18 items）。
- **is_decorator_history_diff_empty 0x140a84290**：`cmp [rcx+0x18],0; sete al`
  → diff 的 +0x18 字段 == 0（**不是**“容器 len”，reconstruction mod.rs 注释正确）。
- **is_stairs_history_diff_empty 0x140a842a0**：`[rcx+0x30] != 0 → false;
  [rcx+0x48] == 0` → 两个 Vec 的 **cap 字段**（cap-first 布局下 cap@+0x30/+0x48，
  cap==0 ⟺ 空；空 Vec 的 dangling ptr=4 在 +0x38/+0x50）。与 reconstruction
  “读 len(+0x40/+0x58)”**等价但不相同**（len/cap 对空 Vec 同为 0，两读法都成立；
  原版实际读 cap）。
- **StairsDiffTable**：tag0@+0x00、tag1@+0x08（=1）；vec_a@+0x30（cap-first）、
  vec_b@+0x48；两 Vec 元素 = **u32 边对 {min,max}**（add_decorator_diff 中按
  无序边做 min/max 归一后哈希查表）。

### 2.4 编辑负载

- **HistoryEdit（24B）**：`{tag: u32@+0x00, records: u64@+0x08, count: u64@+0x10}`
  ✓（历史合同）。add_decorator_diff 写入两种新 tag：**tag 0xE（14）**（“移除边”）
  与 **tag 1**（“新增边”），records 指向 32B 记录
  `{1, 1, type: u32@+0x10, min: u32@+0x14, max: u32@+0x18}`（type=0/1 区分两表）。
- **WallHistoryEdit 记录（24B）**（new_height_changed 0x140a83fb0 O）：
  `{1, 1, 0x8000000000000002@+0x10（niche 判别 = 变体 2）, wall: u64@+0x18,
  h0: u32@+0x20, h1: u32@+0x24}`；随后分配 0x108（264B）blob 拷贝 + 
  FUN_141696f50 校验，最终 HistoryEdit = {tag 7, &24B 记录, 1}。
- **TerrainReplayCmd**（forward 0x140994f30 输出 O）：
  `{data 指针@+0x00, u32@+0x08=0, duration f32@+0x0c, u32@+0x10=2}`。
  duration = `clamp(Σ点距*0.01, 0.4, 3.0)`；无点默认 0.3；点集元素 0xc=12B；
  非有限 → panic "assertion failed: stroke_length.is_finite()"
  （terrain_edit.rs:0x2b=43）。注意 **0x01 是米换算（cm→m），0.4/3.0 是
  clamp 上下界**。
- **TerrainEdit 判别（Debug::fmt 0x140858c80 O）**：首字段 niche
  `x ^ 0x8000000000000000 < 2` → 变体 0/1/2；变体 0 = `+4` 处 int（stroke id），
  变体 1 = `+1` 处 0x11B payload，变体 2 = `+4` 处 0x10B payload。
- **DecoratorHistoryState::From（0x140b3f930 O）**：源状态 type 字节 @+0x41
  （==8 preview → panic）；变体 2/3 读 u32@+0x30，变体 4/5/6/7 读 byte@+0x30，
  其余（8..=255）读 +0x34；输出 0x50B+ 记录（字节字段 @+0x44..+0x4e）。
- **is_default_lantern_brightness 0x140b3f910（O）**：`cmpeqss x, [0x142925904
  = 1.0f]` → `x == 1.0`。**is_default_stair_width 同址同码**（ICF 合并）→
  默认灯笼亮度 = 默认楼梯宽 = 1.0。
- **WallHistoryEdit（Debug::fmt 0x140a82eb0 O）**：首字段 niche 判别
  （`*p - 0x7fffffffffffffff` 得变体）跳表 @0x142ad888c 分派。
- **HistoryChainIndex**：new/inner 0x140833250 恒等；Display 0x140b31c90 =
  `{}` u32（formatter +0x30/+0x38 是 buf 指针/长度，**不是**容器切片）；
  serde Serialize 0x1408aa9f0 = 数字（base-100 对查表 itoa，无千分位）。

## 3. 方法/系统行为

### 3.1 History 核心（O；细节引用 reconstruction mod.rs 模块文档，此处只列新证/修正）

- **insert_starting_edit 0x140a844f0**：真名（public PDB）= `insert_starting_edit`
  —— reconstruction 依 sysres 改名为 `new_from_params` **是错的**，旧合同名正确。
- **debug_display 0x140a84ba0（O）**：格式串 "History: {} / {}. Start at {}"，
  3 参 = {cursor（HistoryChainIndex Display）, len-1, history_start}。
- **remove_snapshot_record**：**不在 0x140a84ba0**（那是 debug_display）。
  真实地址 U（候选：0x140a84a00..0x140a84a3b 的 59B 本地函数——但该函数形态是
  `[rdx+0x5f] 标志 + [rdx+0x50] Arc drop（lock dec → 0x1402e5d80 族）`，更像
  drop glue；或位于 0x140a84908..0x140a849fc 区间内）。sysres 的
  “remove_snapshot_record@0x140a84ba0”与 reconstruction 的“委托
  FUN_140d79f90 + prev 回调”均**无证据支持**（FUN_140d79f90 = format dispatch）。
- **closest_resolved_snapshot 0x140a84a50（O 机器码核对）**：入参
  (edi=cursor, rsi=self)；读 snapshots ptr/len（+0x20/+0x28）构建
  filter_map 迭代器（闭包 = `<ResolvedHistoryChainSnapshot as TryFrom<&
  HistoryChainSnapshot>>` 0x140a84260），二分 chain_index<=cursor 最近者。
  与 reconstruction 一致。**注意 TryFrom 0x140a84260 是真实过滤谓词**
  （reconstruction 的 TryFrom 恒 Ok 是占位，过滤规则 U）。
- **set_current_snapshot 0x140a84760**：断言不替换（既有证伪结论 ✓ 不变）。
- **finish_input_recording 0x140a86730**：追加到 `ResMut<HistoryChainSnapshots>`
  （+0x20..+0x38 Vec 48B 记录 + +0x40 计数器）✓ 与 reconstruction 一致。
  `<HistoryChainSnapshots as Deserialize>::deserialize` = 0x140b363e0（新）。

### 3.2 add_decorator_diff 0x140a854a0（O，全文已读）

签名（修正 reconstruction 的 2 参）：`(&mut self, diff: &DecoratorDiff,
stairs: &StairsDiffTable, remap: &RawTable)`。流程：

1. `decorator_empty = diff.+0x18 == 0`；`stairs_empty = stairs.vec_a.cap==0 &&
   vec_b.cap==0`。
2. pending 哨兵（+0x30 < 0）→ panic "Tried writing decorator_history_diff to
   History when the input recording is finished"（mod.rs）。
3. **非空路径**（decorator 非空，或 decorator 空但 stairs 非空）且
   pending.edits 为空 → 与 `add` 相同的 nuke 判定
   （cursor - edit_chain.len < -1 → nuke_future_states）并**重建 pending 桶**
   （64B 装饰表 + 96B 楼梯表 + 全局序列 FUN_1400d9b20 递增快照）。
   **reconstruction 的“diff 为空则直接 return”是错的**——空 decorator diff 仍
   会走 stairs 边记录与 Arc 替换路径。
4. 楼梯图校验（仅当全局开关 `byte[0x14362c011]==1` 且 stairs.tag0==1）：
   断言 cursor >= -1（"assertion failed: self.0 >= -1"，mod.rs:0x1e=30）；
   从 edit_chain[cursor] 向前扫描（0x28 步长），找第一个装饰表
   `type_counts[0]@+0x20 == 1` 的 MultiEdit；若 `stairs.tag1(==1) !=
   该表 +0x28 的 u64` → log（target "country_core::resources::history"，
   fmt "Stairs diff: "，参数 = stairs 表的 Display FUN_140a879b0；log 开关
   `*byte[0x14362c3a8]`）→ panic "Stairs graph history recording error:
   checksum mismatch; last known: {…}"（mod.rs）。
5. 边记录循环 A：遍历 stairs.vec_a 的 u32 边对（min/max 归一），hashbrown
   查 param_4（remap 表），命中 → push HistoryEdit{tag 0xE, 32B rec
   {1,1,type=0,min,max}, 1} 进 pending.edits（元素 0x18）。
6. 循环 B：遍历 stairs.vec_b 边对，FUN_140b76ce0 查表（返回 enum），==1 →
   push HistoryEdit{tag 1, 32B rec {1,1,type=1,min,max}, 1}。
7. 尾部：从 diff 构建新 64B 装饰表 **替换 pending.decorators Arc**
   （LOCK dec 旧 Arc，0 时 FUN_140b5cb60 drop）；从 stairs 拷贝构建新 96B
   楼梯表 **替换 pending.stairs Arc**（0 时 FUN_140b5c420 drop）。

### 3.3 history_process 0x140af5b30（O，42KB 全文分段读）

ECS 参数面（9 参）：
- param_1 = `&mut Mut<History>`（tick 回写 +0x1c）；
- param_2 = `&mut Mut<Events<HistoryCmd>>`（4B 记录流，puVar37）；
- param_3 = `&mut Mut<ReplayBuffer>`（tick 回写；静态 0x142ae5430）；
- param_4 = `&mut Local<双游标读取器>`（数据 +0x08/+0x10 与 +0x28/+0x30 两条
  **16B 记录**事件流，游标 +0x18/+0x38——即“第二事件流”）；
- param_6/param_7/param_8 = 三个 `&mut Mut<…>`（tick 回写；param_7 内含
  cap-first Vec<u64>@+0x20..+0x40 + 计数器@+0x40；param_8 内含 0x20 处的
  Vec（grow FUN_140562870））；
- param_9 = `&…`（`*param_9+0x10 != 0 → return`，另一待处理队列）。

行为：
1. 消费 4B HistoryCmd 事件（cursor 滑动 `*puStack_c8 = 余量`）：
   byte[1]==4 或 byte[0]==2 → Interrupt（直接 return，不消费后续）；
   byte[1]==0 → Undo（三重守卫后 cursor-1）；byte[1]==1 → Redo
   （assert cursor>=-1 "assertion failed: self.0 >= -1"，目标守卫后
   cursor=target）；其余 → `unreachable!`（history_process.rs:0x28=40，
   "internal error: entered unreachable code"）。
2. 取 `edit_chain[cursor]`（0x28 步长）→ 对其 edits（0x18 步长 24B）逐个
   `generate_replay_cmds_from_edit`（FUN_14143fb50 单态）写入 0x30B 命令 Vec；
   Arc clone 该 MultiEdit 的 decorators(+0x18)/stairs(+0x20) 表
   （LOCK inc + 溢出 panic 路径）；
3. **undo 且命令数 >1 → 命令 Vec 整体反转**（0x30 元素两两交换）——O 新证；
4. 构造 0x40B 桶（decorators/stairs Arc + cmds Vec）push 进 ReplayBuffer
   （0x140af6381 起：Vec<24B 记录> 追加 + 0x138/0x140 索引 Vec 维护 +
   0x120 长度校验），随后按 ReplayCmd 判别字节经跳表 0x142ae545c 逐条校验
   （重复/冗余命令丢弃逻辑，见 §2.2）；
5. **第二事件流**（16B 记录，`Local` 游标）：record byte[+9] 经跳表
   0x142ae5420 分派（0x0/0x1 = 检查另一 Vec 末元素标志字节；0x2 = 跳过；
   0x3 = `ReplayBuffer::make_animations_instant` 0x1409f1ab0；0x13 = 从
   0x30 步长命令区移除一条并分派；0x18 = 直接 return 等）；
6. 两流都空时：ReplayBuffer+0x18 != 0 或 param_9 队列非空 → return（让
   回放系统排空）；录制中（pending 哨兵 >= 0）→ log（target
   "country_core::systems::history::history_process"）**"Can't process
   History while recording a new edit is active!"**（0x142ae5290）。

### 3.4 replay_edit 0x140af6170（O）

`(&MultiEdit, is_undo: bool, closure_env, &mut sink)`：对 edits 逐个
generate_replay_cmds_from_edit；Arc clone 两表；undo 时反转；构造 0x40B 桶
push 进 ReplayBuffer 尾部（含逐命令去重校验，与 history_process 同一套
跳表逻辑）。

### 3.5 generate_replay_cmds_from_edit ×4（O）

`0x14143d2c0/0x14143fb50/0x1414423e0/0x141444c70` 全部是 316B 跳板：
`(*(code *)(&UNK_142c5f248 + *(int *)(&UNK_142c5f248 + *param_1*4)))()` ——
按 HistoryEdit.tag 经跳表 0x142c5f248 尾调进共享核心（各单态仅闭包环境不同，
ICF 近似合并）。**generate_path_replay_cmds 0x1416286d0 同理**：读 [rcx] niche
判别、`dec; cmp 4; jae` 变体 0..3 过滤后尾跳到 0x1414433xx 共享核心
（Ghidra 因跳板未建函数，故 batch NotFound；机器码已解）。

### 3.6 replay_pop_bucket 0x140b38250（O，全文已读）

参数面：9 参（`&mut Mut<ReplayBuffer>` + 多个 Mut tick 回写 + 1 值参）。
行为：ReplayBuffer 活跃计数非 0 且**前桶 cmds Vec len(+0x10)==0** →
`pop_front`（0x1409f18f0）取出 (decorators Arc, stairs Arc)：
- decorators Arc 非空 → `FUN_14155ba30(storage, arc+2, …)`（装饰 diff 恢复，
  = reconstruction 记的 `DecoratorStorage::restore_decorators<closure_env$0>`），
  后 LOCK dec + drop（FUN_140b5cb60）；
- stairs Arc 非空 → 楼梯应用链：FUN_1408ff210 → FUN_140900ba0 → 
  FUN_14157cec0(+0xb8,+0x28,+0x148) → FUN_140b50cc0(+0xb8) →
  FUN_140b50d60(+0x100) → (+0x178)=0 → FUN_1408fefe0（=
  `StairsState::restore_backup/apply_diff/iter_edges_and_stubs` 链），后
  LOCK dec + drop（FUN_140b5c420）；
- 收尾：FUN_140b2cb30(…, 1) + FUN_140b743e0(…) + FUN_1408cd5c0
  （RaycastWorld flush / archivist extract）。

### 3.7 领域回放系统（O 骨架级）

- **replay_wall 0x140951b80**：迭代前桶 cmds，判别 4（Wall）→ 每
  `WallReplayEditCmd`（0x18B）调 `process`（**0x140abfb30，自递归**，参数
  (cmd, dt*速度, &mut Mut, …，8 参)）；done 标志 @+0x15；全部 done 时按
  `x-0x7fffffffffffffff` niche 经 vtable 0x142ab4910 收集已改墙
  （FUN_1402ca2f0）；全局序列 FUN_1400d9b20 驱动。
- **replay_terrain 0x140995150**：判别 3；`TerrainReplayCmd::forward/
  backward`（0x140994f30/0x140995040 同构 1533B）构造 duration
  （§2.4）；undo(+0x1c)==0 → reapply，!=0 → revert。
- **replay_stairs 0x140af75b0**：判别 10；undo==0 → 边集合 insert（
  hashbrown FUN_1405b1be0/0x140b76ce0），!=0 → remove_entry；查
  DecoratorStorage::lookup_decorator_info + CachedDecoratorTransforms；
  发 PlayStairsAudioCmd。
- **replay_auto_clutter 0x140aa4a60**：判别 9；undo==0 → 插墙集；!=0 →
  DeletedAutoClutter erase (DecoratorId, DecoratorId)。
- **replay_raster ×6**（0x1409f1cf0/0x1409f4650/0x1409f6ff0/0x1409f9930/
  0x1409febf0/0x1409fc290，57–67KB 重内联）：泛型 `replay_raster<T,R,F>`；
  panic "assertion failed: !cmd.edit.stroke.points.is_empty()"
  （replay_raster.rs）；"Tried to init CurveU from an invalid u: " 复用；
  `RasterReplayCmd::forward/backward` = 0x141700430/0x141700620。
- **replay_mode 0x140a7c550**：`*(i32*)app_mode != 0xd → return`（**AppMode::
  Replay = 0xd 再次确认**）；进入/离开回放 log "Replay mode started"/
  "Replay mode finished"（target "country_core::systems::history::
  replay_mode"）；写回放状态资源（+0x28 f32 = 1.0 等）。
- **finish_replay 0x1408b36a0**：pending 恰 1 条 redo → CmdUnfreezeSheep；
  pending 空且 ReplayBuffer+0x18 空 → RestoreAppTo 写回 AppMode、
  toolbar switch_mode（+0x80/+0x88 条件）、MetaCommander 顶部哨兵 log+pop；
  log 串见 §4。
- **signifier_replay_active 0x140b1b860**：ReplayModeElapsedTime（enum2$ 判别）
  活跃时累加；Query<(&mut Transform, &SignifierReplayIndex)> 弹性抖动
  （`0.5*exp2(-10t)*sin(7.853982t)`）；register_required_components
  0x140b1d680。
- **history_finish_recording 0x1409dd790（round2 纠错：sysres-v1 名
  history_start_recording 错，v2+S_PUB32 双证为 finish）**：多事件游标
  （param_7/8/9）+ AppMode 检查 → 录制启停（0x140b2cb30 等辅助）。
  history_start_recording 无独立符号（内联）。
- **add_history_systems 0x1409438a0 / insert_history_resources 0x140943e10 /
  add_history_events 0x140943f40**：Bevy 注册胶水（~10 系统注册、
  ReplayBuffer 等资源插入、HistoryCmd 等事件添加）——按依赖过滤不计业务。

### 3.8 TerrainEdit（O）

- **revert 0x140858d30**：判别 0（NewStroke）时若 `param_6 < 0.99`（静态
  0x142a948d8 = 0.99f，动画进度阈值）→ panic "Requested terrain stroke
  {id}"（terrain_edit.rs，0x142a94cf8）；否则 hashbrown 查 48B/元素 的 stroke
  表（FUN_1405b0ed0），命中即清该 stroke 的 active 标志（+0x28=0）。
- **reapply 0x1408592d0**：对称流程（同 panic 族）。
- **duration 真身 U**：sysres/reconstruction 称“duration@0x140858d30”——
  **错**（那是 revert）。duration 的 clamp 公式实现在
  `TerrainReplayCmd::forward` 内联（§2.4）；独立 duration 符号未在 S_PUB32
  中出现（本地符号，候选在 0x140859740 一带）。

### 3.9 装饰（O）

- **convert_before_after_states_to_changes 0x140b3fab0**：真实地址（
  reconstruction 写 0x140b31220 **错**）。签名 `(out: *mut Change,
  before: *const State, after: *const State)`。行为：按 style 字节（+0x32）
  分变体比较浮点字段（变体 2/3 比 4 个 f32；4 比 6 个；default 比 8 个 f32 +
  字节 + u64）；style 族字节（+0x5a，-0x38 后 clamp 到 6）不等时输出 '@'
  哨兵；输出 = after 状态拷贝 + `changed` 布尔 @+0x40 与 @+0x52。
- **DecoratorHistoryState::From** 见 §2.4。
- serde visitor（0x1417bff80/0x1417c0cf0/0x1417bf8a0）：字段
  lantern_brightness / chimney_smoke / stair_width / ladder_color / railing /
  stairs_supports / magic_color / chimney_h_offset；枚举 tag ClutterVersion /
  DecoratorRank{None,Low,High,WoodenFull}；错误串 "map with a single key"、
  "string or map"。

### 3.10 墙复合（O）

- **CompoundEditBuilder**：new_edit_sequence 0x140a82b20 / next 0x140a82b40 /
  build 0x140a82d50 / panic_deprecated_operation 0x140a82e20
  （reconstruction 写 0x140a83120 **错**）/ wall_deleted 0x140a876a0
  （systems::wall::deferred_wall_history 的 trait impl）。
- **update_compatibility 真实地址 U**：0x140a82eb0 = WallHistoryEdit
  Debug::fmt（sysres 名错）。候选本地函数区间 0x140a82940..0x140a82b20
  （4 个本地函数）或 0x140a82d50..0x140a82e20；S_PUB32 无此符号。
- **ev_new_wall 0x140a83300**：断言 "new_wall.curve.length_ws() != 0.0"
  （wall_history.rs）；构建 0x108B WallHistoryEdit + FUN_141696f50 校验 →
  History::add({tag 7, …}, track_caller 文件)。
- **ev_height_changed 0x140a83f80（O 机器码全解）**：0x30B 小函数——
  rdi=栈 24B 缓冲 → `EditType::new_height_changed(buf, wall, h0, h1)`
  （call 0x140a83fb0）→ `History::add(self, &buf_edit, &静态文件串)`
  （call 0x140a84ec0，r8 = 0x142ae7818 区域静态）。
- **WallHistoryEdit serde deserialize**（0x141401960/0x1414022c0/0x141402680/
  0x141402790 等族）：操作 tag 串 ChangeFreehandHeightType / PlasterPeelAmount /
  ChangePlasterPeelAmount / MorphPremadeShape / MorphFreehandWall…。

### 3.11 serde 巨函数真实身份（§7 主纠错）

- **0x140b36460（77KB）≠ closest_history_snapshot**：
  S_PUB32 = `<MultiEdit as Deserialize>::deserialize`。Ghidra 未切开其与
  visit_seq 0x140b365e0 / visit_map 0x140b36840 的边界（另 91KB 文件）。
  内容：入口按首字节跳表 0x142aee31c 分派 EditType/HistoryEdit tag（0..=8+），
  niche 写入 0x8000000000000000+i；字段名证据：wall_index（0x142aedfb8）、
  roof_shape_params（0x142aeda00，SIMD 16B 逐字节比较）、bend_vector/pad0/
  custom0/custom1（0x142aee250 区）、StairsRemovedSupports/edges、
  信封五字段 chain_index/history_start/edit_chain/snapshots/snapshot_id
  （0x142aee01f 区 blob + 0x142aee0ac）；serde_json 错误
  "map with a single key"（0x142aedac0）。
  **（round2 纠错）** time_of_day/sun_xz_rotation/latitude_degrees
  （0x142aee29d 区）**不属于** history 快照负载——它们属邻接模块
  `resources::time_of_day` 的 serde（0x140b3bc00+ 区，TodLightingSnapshot/
  TimeOfDay），Ghidra 因函数区越过 0x140b3bc83 边界把两者并入了同一反编译；
  快照负载 T 具体类型仍 U（转 session-save 域，见 round2/history-CLOSURE.md §3）。
- **0x140b363e0** = `<HistoryChainSnapshots as Deserialize>::deserialize`（新）。
- **roof_edit visit_str 0x140b392d0**：操作 tag FlatRoofRimStyleChange /
  FlatRoofRimStyleChange(变体) / SwitchRoofRidgeDir。
- **HistorySnapshotId::serialize** = 0x140b39a20（新）。

## 4. panic/日志串清单（原文 + 地址）

| 串 | 地址 | 来源函数 |
|---|---|---|
| "Tried writing decorator_history_diff to History when the input recording is finished" | fat 0x142ad83c0 | add_decorator_diff |
| "Stairs graph history recording error: checksum mismatch; last known: {…}" | fat 0x142ad8310 | add_decorator_diff |
| log target "country_core::resources::history" + fmt "Stairs diff: " | 0x142ad82a8 / fat 0x142ad8280 | add_decorator_diff |
| "assertion failed: self.0 >= -1"（mod.rs:30） | 0x142aed680 | add_decorator_diff / history_process redo |
| "assertion failed: new_wall.curve.length_ws() != 0.0"（wall_history.rs） | 0x142ad7d48 | ev_new_wall |
| "Tried to init CurveU from an invalid u: " | fat 0x142aa53c0 | ev_new_wall / replay_raster 族 |
| "Requested terrain stroke {id}"（terrain_edit.rs） | fat 0x142a94cf8 | TerrainEdit::revert/reapply |
| "assertion failed: stroke_length.is_finite()"（terrain_edit.rs:43） | 0x142a94c34 | TerrainReplayCmd::forward |
| "Attempting to capture a DecoratorHistoryState of a preview "（decorator_edit.rs） | fat 0x142aeeea0 | DecoratorHistoryState::From |
| "Can't process History while recording a new edit is active!" | fat 0x142ae5290 | history_process |
| "internal error: entered unreachable code"（history_process.rs:40） | 0x142ae51f2 | history_process |
| "Replay mode started" / "Replay mode finished"（replay_mode.rs） | fat 0x142ad7120 / 0x142ad7190 | replay_mode |
| "History replay has naturally finished. Unblocking game commander."（finish_replay.rs） | fat 0x142a9f208 | finish_replay |
| "History replay was not active. Unblocking game commander." | fat 0x142a9f298 | finish_replay |
| "assertion failed: !cmd.edit.stroke.points.is_empty()"（replay_raster.rs） | 0x142ac5958 | replay_raster |
| "map with a single key" / "string or map"（serde_json） | fat 0x142aedac0 / 0x142ce11f8 | serde 族 |
| OOM "memory allocation of … failed" 族 | 0x142ad8350/0x142ad8338/0x142ae52e8/0x142c38b78(alloc slice.rs) | grow 族 |

## 5. 与现有合同/代码的差异（“代码写 X，证据为 Y”）

1. **0x140b36460 身份**：任务/合同称 `closest_history_snapshot`（快照解析/
   筛选系统）→ 证据：`<MultiEdit as Deserialize>::deserialize`（S_PUB32），
   实际是 serde 反序列化巨函数。真实 closest 族 = History::current_snapshot
   0x140a84650 / closest_resolved_snapshot 0x140a84a50 /
   save::snapshot::validate_and_load_closest_snapshot_data 0x1417a6e50。
2. **0x140b36840** = `MultiEditVisitor::visit_map`（非“impl$16::deserialize::
   visit_map”的 sysres 名，那只是展开差异，身份一致）。
3. **HistoryChainIndex::prev@0x140b31c90** → 证据：0x140b31c90 =
   `HistoryChainIndex as Display>::fmt`（+0x30/+0x38 是 formatter buf，
   非容器切片迭代）；prev/next 无独立符号（内联）。
4. **remove_snapshot_record@0x140a84ba0** → 证据：0x140a84ba0 =
   debug_display（O 机器码）；remove_snapshot_record 真实地址 U。
   reconstruction mod.rs 的“委托 FUN_140d79f90 + prev 回调”描述无证据。
5. **update_compatibility@0x140a82eb0** → 证据：0x140a82eb0 =
   WallHistoryEdit Debug::fmt。真实地址 U（候选 §3.10）。
6. **CompoundEditBuilder::build@0x1414022c0** → 证据：0x1414022c0 =
   WallHistoryEdit serde Deserialize 族；build = 0x140a82d50。
7. **TerrainEdit::duration@0x140858d30** → 证据：0x140858d30 = revert；
   duration 公式实现在 TerrainReplayCmd::forward 0x140994f30 内联。
   （reconstruction 的 duration 数值公式本身正确。）
8. **is_default_lantern_brightness@0x140b3f930** → 证据：真函数在
   **0x140b3f910**，签名 `(f32)->bool`，语义 `x == 1.0`；0x140b3f930 =
   DecoratorHistoryState::From。且 **is_default_stair_width 与
   is_default_lantern_brightness 同址同码**（都 ==1.0，ICF）。
   reconstruction 的 `(record: &DecoratorRecord)->bool` 签名错。
9. **convert_before_after_states_to_changes@0x140b31220**（reconstruction
   注释）→ 证据：真实地址 0x140b3fab0。
10. **ev_height_changed@0x140a83fb0**（任务）→ 证据：0x140a83fb0 =
    `EditType::new_height_changed`；ev_height_changed = 0x140a83f80。
11. **WallReplayEditCmd::process@0x140abfb30** ✓ 真名（sysres 的
    “replay_wall::closure$0”错）。
12. **replay_path::replay_path@0x1416286d0** → 证据：0x1416286d0 =
    `generate_path_replay_cmds`（跳板，尾调 0x1414433xx 共享核心）；
    replay_path 系统本体 = replay_raster 的 Path 单态。
13. **RasterReplayCmd::backward_instant@0x1409fc290** → 证据：0x1409fc290 =
    第 6 个 replay_raster 单态（h7643…）；forward = 0x141700430，
    backward = 0x141700620。
14. **add_decorator_diff 空 diff 直接 return**（reconstruction）→ 证据：空
    decorator diff 不 return（仍处理 stairs 边 + Arc 替换）；且真实签名 4 参
    （reconstruction 2 参）。
15. **ReplayBuffer 布局“环形 {head,buf,cap,len,modification_counter}”**
    （reconstruction 文档头）→ 证据：头部 = 桶 Vec（cap-first，0x40 元素）
    @+0x00..+0x18 + 活跃计数 @+0x18；reconstruction 结构体用 VecDeque 的
    选择与证据不冲突但“环形”头注释无证据（U）。
16. **is_stairs_history_diff_empty 读 len(+0x40/+0x58)**（reconstruction）→
    证据：原版读 **cap(+0x30/+0x48)**（对空 Vec 等价，但字段不同）。
17. **History::add 签名**（reconstruction 2 参）→ 证据：3 参，第 3 参 =
    track-caller 文件路径 &'static str（panic 定位用）。
18. **EditType::new_height_changed 无参**（reconstruction）→ 证据：
    `(out: &mut 24B, wall: u64, h0: u32, h1: u32)`，写 niche 判别
    `0x8000000000000002`（变体 2）+ 0x108B blob 拷贝 + FUN_141696f50。
19. **path_edit serde 地址**（reconstruction 写 0x16e8340/0x16e8510）→
    证据：0x1416e8510 区（丢了一位 1，量级差 0x140000000）。
20. **insert_starting_edit 改名 new_from_params**（reconstruction）→ 证据：
    S_PUB32 真名 = insert_starting_edit（旧合同名正确）。
21. **HistoryChainIndex serde“数字”** ✓（0x1408aa9f0 = base-100 对查表
    itoa，确认无千分位）。
22. **TryFrom 恒 Ok**（reconstruction 占位）→ 证据：0x140a84260 是真实
    过滤谓词（谓词规则 U，与“resolved”语义有关）。

## 6. 跨领域线索（供中央索引）

- `systems::save::snapshot` 族（session-save 域）：capture_game_snapshot
  0x1417a1d00 / capture_game_snapshot_save_data 0x1417a0430 /
  init_game_state_from_snapshot 0x1417a9c50 / validate_and_load_closest_
  snapshot_data 0x1417a6e50 / try_chunk 0x1417a6d70 / SnapshotLoadResource
  SystemParam 0x1417ab290 —— **快照负载 T 真正的主场**（round2 纠错：原“含
  sun 字段”论断撤销，那些串属 time_of_day 模块）。另：`resources::time_of_day`
  模块 serde（TodLightingSnapshot::serialize 0x14007ff80、TimeOfDay::
  serialize 0x1400802c0、deserialize visitor 0x140b0cc40/0x140b3bc00 区）
  在 Ghidra 反编译中与 history serde 区重叠。
- `systems::save::save_game::add_history_edit_to_save_file` 0x140a439b0。
- wall 域：`systems::wall::deferred_wall_history` 族（history_record_wall_
  changed 0x1408dccf0 / history_record_merge_walls 0x1408dd1b0 /
  PendingWallChanged::try_new 0x1408dc9f0 / PendingWallHistory /
  PendingMergeWalls / CompoundEditBuilder::wall_deleted 0x140a876a0）。
- stairs 域：`StairsState::restore_snapshot` 0x1408ff590、
  `resources::stairs::stairs_state` 边对、`PlayermadeStairsAssemblyRequests::
  prev_requests` 0x140274480；`StairsRemovedSupports` 资源（replay_stairs/
  add_decorator_diff 共同消费）。
- 装饰域：`DecoratorStorage::restore_snapshot` 0x140a20d70、
  `system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory`
  （drop 0x14014b5e0）、`systems::wall::eraser::archivist::WallHistoryEdits`
  （drop 0x140894900）、`AnimateWallBulldozeBuffer`。
- 地形域：`systems::terrain::deferred_terrain_history::PendingTerrainHistory`
  （drop 0x14014b7e0）。
- 其他：AppMode::Replay = 0xd（再次确认）；bevy_ecs 0.16.0（事件 collections.rs
  路径）；serde_json 1.0.85（ser.rs 路径）；toolbar（finish_replay 引用
  toolbar.rs）；`country_core::resources::session::NextSession::ignore_snapshot`
  0x140b46a20。
- **进程全局变量族（配合 audio-misc 域线索）**：
  - `0x14362c420`（qword）：**panic-in-progress 门控**。finish_potential_mutation
    0x140a86980 在 token 未消费分支先检查
    `(*0x14362c420 & 0x7fffffffffffffff) != 0 → 跳过 panic`（跳到
    FUN_1428d86b0，疑似 std::thread::panicking 语义）。与 reconstruction 的
    `if self.active && !std::thread::panicking()` Drop 守卫语义完全对应——该全局
    即原版“是否正在 panic”的来源。语义线索：非零（符号位掩掉后）= 正在 panic。
  - `0x14362c3a8`（字节/整型）：**日志级别全局**（数字越大越详细）。历史域
    四处日志点的不同阈值：add_decorator_diff `!=0`、history_process `>1`、
    replay_mode `>3`（两处）、finish_replay `>=4`（两处）。语义线索：
    全局 verbosity 计数器（非线程局部）。
  - `0x14362c011`（字节）：add_decorator_diff 楼梯图 checksum 校验开关
    （==1 才执行校验分支；U：debug 构建标志或运行时开关）。
  - `0x14362c370`（qword）：add_history_systems 0x1409438a0 经
    FUN_140b218a0 读写的静态（Bevy 注册胶水用；U）。

## 7. 错位/纠错登记

- **sysres-procedures.json 本域 11 处名/址错位**（全部以 S_PUB32 裁决）：
  closest_history_snapshot→MultiEdit::deserialize；remove_snapshot_record→
  debug_display；update_compatibility→Debug::fmt；CompoundEditBuilder::build
  （0x1414022c0→serde Deserialize 族）；duration→revert；revert::closure$0→
  reapply；is_default_lantern_brightness（+0x20 偏移）；ev_height_changed
  （+0x30 偏移）；WallReplayEditCmd::process（闭包名）；replay_path::replay_path→
  generate_path_replay_cmds；RasterReplayCmd::backward_instant→replay_raster
  单态。**模式**：多数错位 = 编译单元元数据中相邻符号串位（名属前/后一函数），
  个别为 +0x20/+0x30 地址偏移。
- 0x140b31c90 的 +0x30/+0x38 是 Display formatter 内部字段，不是“容器切片”。
- 已证伪并沿用的旧结论：set_current_snapshot 断言不替换 ✓；
  finish_potential_mutation 0x140a86980 签名 (was_active: bool) ✓；
  is_recording = 0x140a86970 ✓（均与 S_PUB32 一致）。

## 8. U（未决，不臆造）

- remove_snapshot_record / update_compatibility / TerrainEdit::duration 真实地址。
- closest_history_snapshot 独立符号是否存在（或内联进 save::snapshot）。
- 快照负载 T 的完整类型（已知含 sun 环境字段）。
- 跳表 0x142ae5420/0x142ae545c 的逐 case 语义（Ghidra 未恢复跳表）。
- add_decorator_diff 的 remap 表（param_4）内容与用途。
- History 完整 serde 手写 impl 位置（S_PUB32 无 History serde 符号——内联）。
