# history 域算法级深挖（ALGORITHMS.md，2026-10-15）

> 本文件把 history 域 27 章从"函数行为"升级到"算法级"：处理链图、逐函数算法、状态机、
> 常量表。证据 = `history/decomp/decomp-<va>.txt`（B 版机器码/反编译）+ rne-read 原字节/
> 串 + round2/history-CLOSURE.md 跳表解码。O = 机器码证实；I = 推断（注明依据）；U = 未决。
> 与 DIGEST.md §3 互补：DIGEST 记录"是什么"，本文件记录"怎么算"。

---

## 1. 处理链图（三大流）

### 1.1 录制流（输入 → History → ReplayBuffer）

```
输入系统（wall/terrain/decorator/stairs/raster/garden/water/path）
   │  ev_new_wall 0x140a83300 / ev_height_changed 0x140a83f80
   │  DecoratorStorage diff → add_decorator_diff 0x140a854a0
   │  TerrainEdit / RasterEdit / ... → History::add 0x140a84ec0
   ▼
History（0x78B 权威缓冲）
   ├─ pending（+0x30..+0x58：edits Vec<24B> + decorators Arc + stairs Arc）
   │    └─ start_input_recording 0x140a86270 / try_start 0x140a86510 建桶
   │    └─ add / add_decorator_diff 追加 HistoryEdit 记录
   ├─ edit_chain（+0x00..+0x18：Vec<MultiEdit 0x28B>）
   │    └─ finish_input_recording 0x140a86730 把 pending 上链（cursor+1）
   ├─ snapshots（+0x18..+0x28：Vec<16B {Arc<T>, chain_index}>）
   │    └─ set_current_snapshot 0x140a84760 有序插入
   └─ chain_index（+0x68）← undo/redo（history_process 消费 HistoryCmd）
   ▼
history_process 0x140af5b30（每帧）
   ├─ 读 HistoryCmd（4B）→ undo（反转+游标-1）/ redo（游标+1）
   ├─ 对 edit_chain[cursor] 的 MultiEdit：
   │    generate_replay_cmds_from_edit（按 tag 跳表 0x142c5f248）→ 命令 Vec<0x30B>
   │    Arc clone decorators/stairs 表（LOCK inc）
   │    undo 且命令数>1 → 整表反转（0x30B 元素两两交换）
   │    校验（跳表 0x142ae545c）→ push 0x40B 桶进 ReplayBuffer
   └─ 第二事件流（16B 记录，byte[+9] 跳表 0x142ae5420）→ 校验/合并/移除/动画即时化
   ▼
ReplayBuffer（静态 0x142ae5430）：桶队列 {cmds Vec, decorators Arc, stairs Arc}
```

### 1.2 回放流（ReplayBuffer → 世界状态）

```
ReplayBuffer 桶队列
   │  active count（+0x18）!= 0
   ▼
replay_wall 0x140951b80（tag 4）  ── WallReplayEditCmd::process 0x140abfb30（自递归分派）
replay_terrain 0x140995150（tag 3）── TerrainEdit::reapply 0x1408592d0 / revert 0x140858d30（ease_out_cubic）
replay_stairs 0x140af75b0（tag 10）── StairsRemovedSupports 边 insert/remove_entry + PlayStairsAudioCmd
replay_auto_clutter 0x140aa4a60（tag 9）── HashMap<WallId,()> 插墙集 / DeletedAutoClutter erase
replay_raster×6（tag 1/2/8 等，0x1409f1cf0..0x1409febf0）── RasterReplayCmd::forward/backward
   │  （通用骨架：迭代前桶 cmds → 按 tag 过滤 → 推进动画 → 完成后索引入列表）
   ▼
完成命令移除：memmove 压缩前桶 cmds Vec + 按判别字节 drop 分派
   ▼
replay_pop_bucket 0x140b38250（前桶 cmds 空时）
   ├─ FUN_1409f18f0 pop_front 取 (decorators Arc, stairs Arc)
   ├─ decorators 非空 → FUN_14155ba30（restore_decorators）→ Arc drop
   ├─ stairs 非空 → 楼梯应用链：1408ff210→140900ba0→14157cec0→140b50cc0→140b50d60→1408fefe0
   └─ 收尾：FUN_140b2cb30(…,1) + FUN_140b743e0(0x70,0x10) + FUN_1408cd5c0（collider/archivist flush）
   ▼
finish_replay 0x1408b36a0（缓冲与待处理命令双空时）
   ├─ pending 恰 1 条 redo → CmdUnfreezeSheep
   └─ RestoreAppTo 写回 AppMode + toolbar switch_mode + MetaCommander 哨兵弹栈 + 日志
```

### 1.3 存档流（History → save 文件）

```
save_game（session-save 域）→ save_game_to_file / add_history_edit_to_save_file
   └─ History 信封 {edit_chain, chain_index, snapshots, history_start, snapshot_id}
      的序列化在 save 系统内联（无独立 <History as Serialize> 符号）
   └─ MultiEdit::serialize 0x140b39b80（h63c9 单态）
   └─ <HistoryChainSnapshots as Deserialize> 0x140b363e0（读档侧）
```

---

## 2. 逐函数算法

### 2.1 History 方法族（mod.rs）

#### History::add 0x140a84ec0（O）

签名 `(&mut self, edit: &HistoryEdit 24B, file: &'static str)`（第 3 参 = track-caller 文件路径）。

1. `pending(+0x30) < 0`（录制已结束）→ panic `"Tried writing {edit:?} ..."`：
   fmt 参数 = EditType::Debug::fmt（FUN_140a842c0）+ file（串 "Tried writing " @0x142ad8250）。
2. 追加判定：`pending.edits.len(+0x40) != 0 || cursor - edit_chain.len >= -2` → 直接走追加；
   否则（pending 桶空且游标落后链尾 ≥2）→ **重建桶**：
   a. `nuke_future_states 0x140a84c50`（清未来分支）；
   b. 新 64B 装饰表 Arc {1,1} + 模板指针 {0x142af1470/0x142af1478/0x142af1480/0x142af1488}
      + 全局序列 FUN_1400d9b20（16B 计数）；
   c. 新 96B 楼梯表 Arc {1,1} + tag0/tag1=0 + 两空 Vec（cap=0, 悬垂 ptr=4, len=0）；
   d. drop 旧 pending（FUN_14088bae0）→ 写 pending {edits={0,8,0}, decorators@+0x48, stairs@+0x50}。
3. **tag 7（墙）校验**（edit.records ≠ 0 时）：records = 组数组（16B {入口指针数组, 组内条目数}）；
   对每组每条目（8B 指针 → 0x108B WallHistoryEdit 载荷）按 niche（`载荷+0x10 - 0x7fffffffffffffff`）分派：
   - 变体 0：`载荷+0xf8 < 2` 且 `+0x100 的 f32 <= +0xfc 的 f32`（结束高度 ≤ 起始高度）→
     panic "Slice start_u must be smaller than end_u. Otherwise an invalid curve will be created!"（@0x142ad8170）
   - 变体 1：`载荷+0x18 < 0` → panic "None is not allowed in `CreateWall` history command"（@0x142ad81b8）
   - 变体 2：`载荷+0x18 < 0` → panic "None is not allowed in `DeleteWall` history command"（@0x142ad8200）
   全部 panic 携带 file（track-caller）。
4. **追加**：push 24B {tag u32@+0x00, records u64@+0x08, count u64@+0x10} 进 pending.edits
   （cap-first，元素 0x18，grow FUN_140562db0），len++。

#### History::nuke_future_states 0x140a84c50（O）

1. `cursor(+0x68) == len-1` → 直接跳 4（无事可清）。
2. `cursor < 0` → **全重置**：drop edit_chain（FUN_14088acf0）后重建
   {cap=0, ptr=8(悬垂), len=0, +0x20=8, +0x28=0, pending=i64::MIN, snapshots 域拷贝,
   +0x68=-1（chain_index+history_start 区）, +0x70=0（tail）}。
3. `cursor >= 0`：`truncate(edit_chain, cursor+1)`——len = cursor+1，逐个 drop 尾元素
   （0x28 步长，FUN_14088bae0）；若 cursor+1 仍 != len-1 → panic（@0x142ad8100 区）；
   `count(+0x58) = min(count, len)`（clamp）；`FUN_1412f5320(+0x18, +0x68)` =
   snapshots.retain(chain_index <= cursor)。
4. **无条件**：`nonce(+0x60) = fastrand u64`（FUN_14132ade0，全局 RNG @0x142ad7918）。

#### History::insert_starting_edit 0x140a844f0（O）

1. 断言 `cursor >= -1`（`-2 < cursor` 否则 panic "assertion failed: self.0 >= -1" mod.rs:30）。
2. `cursor+1`。
3. edit_chain **首部插入**（insert_at_start）：必要时 grow（FUN_140562ab0）；
   `func_0x000142923c10(ptr+0x28, ptr, len*0x28)` = **memmove 右移一整个元素**；
   在索引 0 写 0x28B MultiEdit（字段拷贝自 param_2 的 0x28B）；len++。
4. `nonce(+0x60) = fastrand`；`count(+0x58) = 0`。

#### History::start_input_recording 0x140a86270 / try_start 0x140a86510（O）

- 公共谓词 `func_0x000140a825e0(mode, 0xd)` = `app_mode != AppMode::Replay(0xd)`（0xd 为
  内联常量，非串）。
- start_input_recording：谓词失败 → panic `"assertion failed: **app_mode != AppMode::Replay"`
  （@0x142ad83f8，mod.rs:0x2f）；否则建新 pending 桶（同 add 的重建：64B 装饰 Arc + 96B
  楼梯 Arc + 全局序列；**不 drop 旧桶——调用方保证未录制**）。
- try_start：谓词通过 **且** pending < 0（未录制）→ 建桶并返回 1；否则返回 0（不 panic）。
  桶字段：pending {edits={0,8,0}, decorators@+0x48, stairs@+0x50}。

#### History::finish_input_recording 0x140a86730（O）

1. `pending(+0x30) = i64::MIN`（**先置 None**）；原值 < 0 → panic
   "Called finish_input_recording when history recording was already Finished"（@0x142ad84a8）。
2. **MultiEdit::is_empty 四条件**（edits.len==0 && decorators表+0x28==0 && stairs表+0x40==0
   && stairs表+0x58==0）→ drop pending（FUN_14088bae0）返回 0（空录制不上链）。
3. 非空：push pending 桶内容（0x28B）进 edit_chain（grow FUN_140562ab0）；len++。
4. `nonce(+0x60) = fastrand`。
5. `cursor(+0x68) = cursor+1`（先断言 cursor >= -1）。
6. **快照追加**（仅当新 cursor == len-1，即编辑在链顶；否则 panic @0x142ad8440 区）：
   `HistoryChainSnapshots`（param_2 = ResMut，+0x20..+0x38 Vec 48B 记录 + +0x40 计数器）push
   48B 记录 {+0x00 = 0x8000000000000000（待定快照 id 哨兵）, +0x08 = 1, +0x28 = 计数器}；
   +0x10..+0x27 未显式写（结构 U），返回 1。

#### History::finish_potential_mutation 0x140a86980（O）

`(was_active: bool)`：was_active 为真 → 清空 snapshots Vec（len=0），逐个 LOCK dec 记录内
Arc，归零 → drop（0x1402e5d80 族）；`nonce(+0x60) = fastrand`。token 未消费的 Drop 守卫
panic 串 "History was mutated, but wasn't told about it"（@0x142a98ad8）由守卫路径触发；
panic-in-progress 门控 `*0x14362c420 & 0x7fffffffffffffff != 0` → 跳过（std::thread::
panicking 语义）。本反编译中该分支为死码（bStack_29 恒 0）。

#### 快照二分族（同一 partition_point + 距离平局算法，O）

**current_snapshot 0x140a84650**：
1. snapshots（+0x20 ptr / +0x28 len，元素 16B {Arc, chain_index i32@+8}）len==0 → None。
2. 二分 `partition_point(|s| s.chain_index < cursor)`：找最后索引 uVar10 使
   snapshots[uVar10].chain_index < cursor（标准二分：`lVar1=(uVar10>>1)+lVar8;
   if (idx[lVar1] < cursor) lVar8 = lVar1; uVar10 -= uVar10>>1;`）。
3. **距离平局**：若 uVar10 != 0，比较 |snapshots[uVar10-1].idx - cursor| 与
   |snapshots[uVar10].idx - cursor|，取更近者；平局取后者（iVar6 <= iVar9 → 前进一格）。
   （注意：平局分支可越过 cursor 取到 chain_index > cursor 的快照。）
4. 命中 → **Arc clone**（LOCK inc + 溢出检查）；`idx == cursor` 才返回 Some，
   否则 Arc dec 返回 None。

**closest_resolved_snapshot 0x140a84a50**：先 `FUN_1413056c0` 对 snapshots 切片做
filter_map（TryFrom<&HistoryChainSnapshot> 谓词 0x140a84260，规则 U）收集到新 Vec，
再对过滤结果跑同一二分 + 距离平局；**不 clone**（返回借用，Option 形式 {flag, Arc, idx}）。

**set_current_snapshot 0x140a84760**：同一二分找最近者 → 若存在 idx == cursor 的快照 →
**panic（断言不替换，@0x142ad8050 mod.rs:0x33）**（先 Arc clone 再 dec 释放）；
否则在 partition_point 处**有序插入** {新 Arc, cursor}（memmove 右移 0x10 步长 + grow
FUN_140562630，Vec {cap@+0x18, ptr@+0x20, len@+0x28}）。

### 2.2 add_decorator_diff 0x140a854a0（O，全文逐行）

签名（修正为 4 参）：`(&mut History, diff: &DecoratorDiff 64B, stairs: &StairsDiffTable 96B,
remap: &hashbrown RawTable)`。param_4 表 = {ctrl@+0x00, bucket_mask@+0x08, growth_left@+0x10,
items@+0x18}，条目 16B {u64 hash, u32 min, u32 max}，键 = 归一化边对。

1. **空判定**：`decorator_empty = diff.+0x18 == 0`；`stairs_empty = stairs.vec_a.cap(+0x30)==0
   && vec_b.cap(+0x48)==0`（原版读 cap 而非 len；空 Vec 时等价）。
2. `pending < 0` → panic "Tried writing decorator_history_diff to History when the input
   recording is finished"（@0x142ad83c0）。
3. **重建判定**：decorator 非空或 stairs 非空，且 pending.edits.len(+0x40)==0 → 同 add 的
   nuke 判定（`cursor - len < -1` → FUN_140a84c50）+ 重建 pending 桶（64B 装饰 Arc + 96B
   楼梯 Arc + 全局序列 FUN_1400d9b20；drop 旧桶）。**空 decorator diff 不 return**。
4. **楼梯图 checksum 校验**（门控：全局字节 `*0x14362c011 == 1` 且 stairs.tag0(+0x00)==1）：
   a. 断言 `cursor >= -1`（mod.rs:30 panic）；
   b. **反向扫描** edit_chain[cursor..0]（0x28 步长，每次取 `element[i].stairs Arc(+0x20)`，
      即先从 cursor 开始逐级回退），找第一个楼梯表 tag0（Arc 表 +0x20）== 1 的 MultiEdit；
   c. 其 tag1（Arc 表 +0x28）≠ incoming stairs.tag1(+0x08) → log（target
      "country_core::resources::history"，fmt "Stairs diff: " @0x142ad8280，用 stairs 表
      Display FUN_140a879b0；级别门控 `*0x14362c3a8 != 0`）→ panic "Stairs graph history
      recording error: checksum mismatch; last known: {…}"（@0x142ad8310）。
   （DIGEST 早前记"装饰表 type_counts[0]@+0x20"——机器码为 **stairs 表** tag0，纠正。）
5. **循环 A**（stairs.vec_a 边对，ptr@+0x38 / len@+0x40）：逐对 min/max 归一 →
   `items(+0x18) != 0` 时 hashbrown SIMD 探测（FUN_1405b1be0 + 内联 group match，
   key = 8B {min,max}，条目 {hash, min, max}）→ 命中 → alloc 32B 记录
   {1, 1, type=0@+0x10, min@+0x14, max@+0x18} → push HistoryEdit{tag 0xE(=14),
   &32B 记录, count=0}（pending.edits 元素 0x18，grow FUN_140562db0）。
6. **循环 B**（stairs.vec_b 边对，ptr@+0x50 / len@+0x58）：归一 → `FUN_140b76ce0`
   查同表（返回 enum，==1 命中）→ push 32B 记录 {1, 1, type=1@+0x10, min,max} + 同
   tag 0xE HistoryEdit。
   （DIGEST 早前记 loop B 为 tag 1——机器码两循环均写 tag 0xE，type 字段区分，纠正。）
7. **尾部 Arc 替换**：新 64B 装饰表 = {Arc{1,1}@+0x00/+0x08, diff+0x00→+0x10,
   +0x08→+0x18, +0x10→+0x20, +0x18→+0x28, +0x20→+0x30, +0x28→+0x38}（整体后移 0x10）
   → LOCK dec 旧 pending.decorators（归零 FUN_140b5cb60）→ 换新；
   新 96B 楼梯表 = {Arc{1,1}, tag0→+0x10, tag1→+0x18, +0x10→+0x20, +0x18→+0x28,
   vec_a{cap,ptr,len}→+0x30/+0x38/+0x40, vec_b→+0x48/+0x50/+0x58} → LOCK dec 旧
   pending.stairs（归零 FUN_140b5c420）→ 换新。

> 语义 I：tag 0xE + type 0 = "移除边"（vec_a 且 remap 命中）、type 1 = "新增边"（vec_b）；
> remap 表 = 边对 → 新增/移除 的映射/去重表（构造方在楼梯域，卡点转 wall/stairs）。

### 2.3 history_process 0x140af5b30（O，42KB 全文）

ECS 参数面：param_1=Mut<History>、param_2=Mut<Events<HistoryCmd>>（4B 记录）、
param_3=Mut<ReplayBuffer>、param_4=Local<双游标读取器>（两条 16B 记录流：
{数据@+0x08/+0x28, len@+0x10/+0x30, 游标@+0x18/+0x38} + 共享计数@+0x40）、
param_6/7/8=Mut<…>（param_7/8 含 cap-first Vec<u64>@+0x20..+0x40 + 计数器@+0x40；
param_6 含 Vec<16B>@+0x20 + 计数器@+0x40）、param_9=&待处理队列（+0x10 != 0 → return）。

**主循环**（`do{…}while(true)`）：
1. 计算两流剩余：`剩余 = 计数 - 游标`（负钳 0），起点 = 数据 + 剩余*16，计数 -= 剩余A+剩余B。
2. 两流均空 → **HistoryCmd 处理**：
   a. ReplayBuffer 活跃计数（+0x18）!= 0 → return（让回放系统排空）；
   b. param_9 队列 +0x10 != 0 → return；
   c. Events<HistoryCmd> 无剩余 → return；有 → 取一条 4B 记录（len--）：
      - `byte[1]==4 || byte[0]==2` → **Interrupt：直接 return**（不消费后续记录）；
      - `byte[1]==0`（Undo）：守卫 `cursor>=0`、`cursor>history_start(+0x6c)`、
        `cursor<edit_chain.len` 后 **cursor--**，取 edit_chain[cursor] 生成命令（undo=1）；
      - `byte[1]==1`（Redo）：断言 `cursor>=-1`（mod.rs:30 panic），目标 = cursor+1，
        守卫 `>=0 && <len` 后 **cursor=目标**，取 edit_chain[目标] 生成命令（undo=0）；
      - 其余 → `unreachable!`（history_process.rs:40，@0x142ae51f2）。
   d. **生成命令**：对该 MultiEdit 的 edits（24B 元素）逐个
      `FUN_14143fb50(edit, is_undo, 标志, &mut Vec<0x30B>)`（= generate_replay_cmds_from_edit
      <history_process 单态>）；Arc clone 两表（LOCK inc + 溢出 panic 路径）；
      **undo 且命令数>1 → 整表反转**（0x30B=6 qword 元素两两交换，双指针向中间推进）；
      分配 0x40B 桶 {cmds Vec, decorators Arc, stairs Arc}。
   e. **push + 校验**（0x140af6381 区）：对新桶每条命令与 ReplayBuffer 既有命令做
      **去重/冗余校验**（跳表 0x142ae545c 按判别字节分派）：规则如
      `(tag & 7) != 0 → 无效`、`tag==0：instant 与 duration 全等 → 重复`、
      `tag==4：count==1 且记录标志 → 对既有命令 +0x49 字节置 0x10（更新标记）`、
      `tag==6/7/0/5/0xa 的白名单`、`tag>10 → 容量错误路径`；校验通过的命令写入
      ReplayBuffer 共享命令区 + 索引 Vec（+0x130/+0x138/+0x140，8B 步长）。
3. 两流非空 → **第二事件流**（16B 记录，偏好流 A）：先查录制态
   `History.pending(+0x30) >= 0` → log "Can't process History while recording a new edit is
   active!"（target country_core::systems::history::history_process，级别 >1 才打，
   @0x142ae5290）→ return；否则 `switch(byte[+9])` 经跳表 0x142ae5420 分派：
   - 0x0：Events<HistoryCmd> 有剩余且**末条 byte[1]==1**（redo）→ 消费（goto 0x140af5df8）；
   - 0x1：末条 byte[1]==0（undo）→ 消费；
   - 0x2：无条件消费；
   - 0x3：`ReplayBuffer::make_animations_instant`（FUN_1409f1ab0）→ 消费；
   - 0x4-0xe：校验/合并分支（0x140af64cf 校验 tag 0/4/8、0x140af6571 置标志、错误 → 0x140af6660）；
   - 0x13：**移除命令**——按记录给出的索引从 0x30 步长命令区取出一条（拷贝 24B 载荷 +
     memmove 左移压缩 + len--），再按被移除命令的判别字节经 0x142ae545c **drop 分派**
     （Arc LOCK dec 族）；0x17：Arc dec 分派；0x18：return；0x19-0x1b：表尾无效；
   - **消费动作**（0x140af5df8）：清空 Events<HistoryCmd>（len=0, idx=0）并把
     param_8 的计数器值 push 进其 Vec<u64>（+0x40++）；
   - 每条记录处理完后（tag != 3）：`param_6` 的 Vec<16B> push {param_6 计数器值,
     record.byte[+0xb]}，计数器++（**命令日志**，供消费侧检测"历史又动了"）。
4. 返回路径上的清理：索引 Vec / 命令区 / 桶 Vec 的 drop glue 全链（0x140af6ea5 区）。

### 2.4 replay_edit 0x140af6170（O）

与 history_process 的"生成命令 → 反转 → 桶 push"段同构的独立入口
（`(&MultiEdit, is_undo, 标志, &mut sink)`）：逐 edit 调 FUN_14143fb50 → Arc clone →
undo 反转 → 0x40B 桶 push 进 ReplayBuffer → 同一套逐命令去重校验跳表。

### 2.5 generate_replay_cmds_from_edit ×4（O）

`0x14143d2c0 / 0x14143fb50 / 0x1414423e0 / 0x141444c70`（garden/history_process/path/water
单态）：按 HistoryEdit.tag 经跳表（各单态独立基址，0x142c5f248 属 garden 单态，water 为
0x142c5f2fc；round3 round2 修正：实函数约 10.4KB，非 316B 跳板——316B 为 Ghidra 跳表恢复
截断）尾调共享核心，把各领域编辑展开为 0x30B ReplayCmd。`generate_path_replay_cmds
0x1416286d0`：读 [rcx] niche、`dec; cmp 4; jae` 过滤变体 0..3 后尾调 0x1414423e0。

### 2.6 领域回放系统（通用骨架，O）

**通用骨架**（replay_wall 0x140951b80 / replay_terrain 0x140995150 / replay_stairs
0x140af75b0 / replay_auto_clutter 0x140aa4a60 / replay_raster×6 均同构）：
1. `ReplayBuffer.active(+0x18) == 0 → return`；`speed = ReplayBuffer+0x28`（f32 全局速率）；
   `dt = Time+0x28`（param 中 Res<Time> 的 delta_seconds）。
2. 取**前桶**（索引 0）：cmds {ptr@+0x08, len@+0x10}；按 0x30 步长迭代，只处理本域 tag
   （墙 4 / 地形 3 / 楼梯 10 / 自动杂物 9 / 光栅 1/2/8）。
3. 每命令处理（见下各域）；完成后把**命令索引**推入局部完成列表
   （cap-first Vec<u64>，grow FUN_140b81df0）。
4. **收尾**：对完成列表**逆序**逐条：边界断言（索引 < 前桶 cmds len，否则 panic_bounds
   check）→ 取出 0x30B 命令 → `func_0x000142923c10` memmove 左移压缩 → len-- →
   **按判别字节 drop 分派**（同一 switch：tag 0/1/2 → FUN_140b5cfc0、tag 3 →
   FUN_140b5c2a0、tag 4 → FUN_140891eb0（墙内联 drop）、tag 5/6/7 → 0x140b5cd40/
   0x140b5d010、tag 8 → FUN_140b5c040、tag 9 → 0x1402e5d80、default → 0x1406978e0）。

**replay_wall（tag 4）**：
- 每 WallReplayCmd：`count@+0x20 != 0` 时循环处理记录元素：对元素内 24B
  WallReplayEditCmd 子记录（fat 指针 {ptr@+0x08, len@+0x10} 指向子列表）逐个
  `process 0x140abfb30(cmd, dt*speed, …)`（自递归，instant/动画两路径）；`done = 子记录+0x15`
  AND 聚合；非全 done → 本帧停止；全 done → **收集已改墙**：对每条子记录 niche 解码
  （`+0x10 - 0x7fffffffffffffff`）经 vtable 0x142ab4910 取目标，FUN_1402ca2f0 收集入局部
  Vec；`count > 1` 时再收集**下一条**元素的墙集，FUN_1408b1fb0 合并比对（返回非 0 才消费：
  count@+0x20--）；命令索引入完成列表。
- 全局序列 FUN_1400d9b20 每命令取一次（墙变更事件的代）。

**replay_terrain（tag 3）**：见 ALGORITHMS 主文档 replay 骨架 + 下列：
- 载荷 = TerrainEdit（niche @+0x10：== 0x8000000000000000 时变体字段在 +0x18）。
- **instant 路径**（`duration(+0x10)==0.0 且非 NaN 且 undo(+0x1c)==0`）：拷贝点集
  （12B/点，断言点数 < 0x3fffff，@0x142aae990 64 字串）；更新地形 stroke 计数
  （+0x38 = max(索引+1, 原值)）；`FUN_141526d10` 直接插入 TerrainEditingData（+0x8 起点），
  完成后释放点拷贝。构造默认命令 {ptr, 0.4f, 2, stroke_id}。
- **动画路径**：`t = clamp(elapsed/duration, 0, 1)`，`eased = 1-(1-t)^3`（ease_out_cubic，
  常量 0.4 为 clamp 下限、1.0 常量 @0x142925904）；`undo==0 → reapply 0x1408592d0
  (payload+0x10, …, eased)`，`!=0 → revert 0x140858d30`；`eased >= 1.0` 时 +0x18 计数--，
  < 0 → 完成入列表；`elapsed += dt*speed`（+0x10）。
- TerrainEdit::revert/reapply（0x140858d30/0x1408592d0）：判别 0（NewStroke）时
  `progress < 0.99`（静态 0x142a948d8）→ panic "Requested terrain stroke {id}"
  （terrain_edit.rs，@0x142a94cf8）；否则 hashbrown 查 48B/元素 stroke 表
  （FUN_1405b0ed0），命中 → 清 stroke active 标志（+0x28=0）。

**replay_stairs（tag 10）**：
- 命令 {tag, flags byte@+0x08（bit0=instant）, duration f32@+0x0c, payload@+0x10,
  elapsed f32@+0x18, undo@+0x1c}；payload+0x10 = 边记录 {edge1 u32, edge2 u32, 标志字节}。
- 动画门控：`elapsed < max(duration/speed, 下限常量 _UNK_142930d70)` 时仅 `elapsed += dt`
  并等下一帧；满足后（或 instant）→ 处理。
- **处理**：`undo==0 → hashbrown insert 边入 StairsRemovedSupports（FUN_1405b1be0 +
  FUN_140b76ce0，表基 +0x20）`；`!=0 → FUN_1402c8640 remove_entry(edge1, edge2)`；
  查 `DecoratorStorage::lookup_decorator_info`（FUN_140a1c930，uVar6=param_6；type 字节
  `+0x41 | 0xf8 > 0xf9 → 跳过`）+ `CachedDecoratorTransforms`（FUN_1405b0ed0 SIMD 探测，
  键 = edge1）→ **push PlayStairsAudioCmd**（Events<PlayStairsAudioCmd>，
  {edge id, decorator info u64 对}）。

**replay_auto_clutter（tag 9）**：`undo==0 → HashMap<WallId,()> insert`；
`!=0 → DeletedAutoClutter.erase((DecoratorId, DecoratorId))`；instant/动画节奏同骨架。

**replay_raster×6（0x1409f1cf0 Cobblestone2 / 0x1409f4650 Water / 0x1409f6ff0 Garden /
0x1409f9930 Cobblestone3 / 0x1409fc290 Cobblestone1 / 0x1409febf0 Path；泛型
replay_raster<T,R,F>）**：前桶过滤本域命令；undo 清缓冲 + 重绘存储 stroke，redo 画
stroke + 发绘制事件；panic "assertion failed: !cmd.edit.stroke.points.is_empty()"
（replay_raster.rs，@0x142ac5958）；RasterReplayCmd::forward 0x141700430 /
backward 0x141700620 = {ptr, +0x8=0, duration f32@+0xc, +0x10=2}。

### 2.7 replay_pop_bucket 0x140b38250（O，109 行全文）

参数面 9 参（Mut<ReplayBuffer> + 多 Mut tick 回写 + 值参）。
1. `active(+0x18) != 0` 且**前桶 cmds len(+0x10) == 0** → `FUN_1409f18f0` pop_front
   取 (decorators Arc, stairs Arc)。
2. decorators Arc 非空 → `FUN_14155ba30(storage, arc+2, …)` = 装饰 diff 恢复
   （reconstruction 的 DecoratorStorage::restore_decorators）→ LOCK dec（归零
   FUN_140b5cb60 drop）。
3. stairs Arc 非空 → 楼梯应用链（8 连）：FUN_1408ff210（restore_backup）→
   FUN_140900ba0（apply_diff）→ FUN_14157cec0(+0xb8, +0x28, +0x148) → FUN_140b50cc0
   (+0xb8) → FUN_140b50d60(+0x100) → `(+0x178)=0` → FUN_1408fefe0（
   StairsState::restore_backup/apply_diff/iter_edges_and_stubs 链）→ LOCK dec（归零
   FUN_140b5c420 drop）。
4. **收尾**（两分支共用）：`FUN_140b2cb30(…, 1)` + `FUN_140b743e0(…, 0x70, 0x10)` +
   `FUN_1408cd5c0(*param_7)`（RaycastWorld flush / archivist extract）。

### 2.8 replay_mode 0x140a7c550 / finish_replay 0x1408b36a0（O）

- replay_mode：`*app_mode != 0xd(Replay) → return`；进入/离开日志 "Replay mode started"/
  "Replay mode finished"（target country_core::systems::history::replay_mode，级别 >3，
  @0x142ad7120/0x142ad7190）；写回放状态资源（+0x28 f32 = 1.0 等）；长按快进
  （HoldToFastReplay，FUN_140c904e0 时间函数）时发 redo HistoryCmd；驱动
  ReplayModeElapsedTime（enum2$ 判别）计时。
- finish_replay：pending 恰 1 条 redo → CmdUnfreezeSheep；pending 空且 ReplayBuffer+0x18
  空 → RestoreAppTo 写回 AppMode、toolbar switch_mode（+0x80/+0x88 条件）、MetaCommander
  顶部哨兵 log+pop（FUN_1408b2270）；日志 "History replay has naturally finished.
  Unblocking game commander."（@0x142a9f208）/ "History replay was not active.
  Unblocking game commander."（@0x142a9f298，级别 >=4）。

### 2.9 signifier_replay_active 0x140b1b860（O）

ReplayModeElapsedTime 活跃时累加；Query<(&mut Transform, &SignifierReplayIndex)> 弹性抖动
`0.5 * exp2(-10t) * sin(7.853982t)`（ease_out_elastic_jiggle，系数机器码级 U）。

---

## 3. 状态机

### 3.1 History 游标与撤销/重做

```
edit_chain: [0 .. len-1]    cursor = 最后应用的编辑
undo: cursor>history_start 且 cursor<len → cursor--，edit_chain[cursor] 以 undo=1 回放
redo: cursor>=-1 且 cursor+1<len → cursor=cursor+1，edit_chain[cursor] 以 undo=0 回放
写入分支（add / add_decorator_diff 重建桶时）：cursor - len < -1 → nuke_future_states
  （截断 edit_chain 至 cursor+1、clamp count、retain snapshots、重随机 nonce）
pending（+0x30）：>= 0 = 录制中（i64::MIN = None/Finished）
```

### 3.2 ReplayBuffer 桶生命周期

```
push（history_process / replay_edit）→ 桶 {cmds Vec, decorators Arc, stairs Arc}
   → 各域系统消费前桶命令（按 tag）→ 命令完成 → 从前桶 cmds 移除（压缩+drop）
   → 前桶 cmds 空 → replay_pop_bucket pop_front → 应用装饰/楼梯 diff → Arc drop
   → active-- → 全部空 → finish_replay 收尾
```

### 3.3 第二事件流记录格式（I：字段名）

```
16B 记录：{+0x00..+0x07 载荷/索引, +0x08..+0x0b 值 u32, +0x09 分派字节, +0x0b 日志标签}
byte[+9] ∈ 0x0..0x18（0x19+ 无效）；0x0/0x1 = 末 HistoryCmd 标志门控；0x2 = 消费；
0x3 = make_animations_instant；0xf-0x18 = 命令移除/清理簇；0x13 = 按索引移除命令
```

---

## 4. 常量表

| 常量/静态 | 值 | 用途 |
|---|---|---|
| 0x142ae5430 | ReplayBuffer 静态实例 | history_process 直接引用 |
| 0x142ae5420 | 28 项跳表（第二事件流 byte[+9]） | round2 CLOSURE §4 逐项解码 |
| 0x142ae545c | 12 项跳表（ReplayCmd 判别 0-9 drop/清理） | case 0xa/0xb 表尾无效；tag 有效域 0..=10 |
| 0x142c5f248 | generate_replay_cmds 跳表（garden 单态） | water 单态基址 0x142c5f2fc |
| 0x142ab4910 | 墙句柄 niche 变体 vtable | replay_wall 收集已改墙 |
| 0x142af1470..0x142af1488 | 装饰表模板指针区 | 建桶时写入新 Arc |
| 0x142925904 | 1.0f | 进度/阈值常量（is_default_lantern_brightness 等） |
| 0x142a948d8 | 0.99f | revert/reapply 动画进度阈值 |
| 0x1429cd060 | 0.4f | terrain 动画 clamp 下限 |
| 0x142930d70 | 下限常量 | replay_stairs 动画门控 |
| 0x14362c011 | 字节 | **存档版本 > 0x1a(26) 时楼梯 checksum 校验开关**（load_game 写，见 session-save） |
| 0x14362c308 | 字节 | 存档版本 > 0x19(25) 的兼容开关（load_game 写） |
| 0x14362c3a8 | 整型 | 日志级别全局（add_decorator_diff !=0、history_process >1、replay_mode >3、finish_replay >=4） |
| 0x14362c420 | qword | panic-in-progress 门控（finish_potential_mutation） |
| 0x142ad7918 | 全局 RNG | fastrand（nonce / HistoryIdentity） |

## 5. panic/断言算法级清单

见 DIGEST §4 全表 + 本文件新增解析：
- add tag7 校验三串（0x142ad8170/0x142ad81b8/0x142ad8200，曲线无效/None 墙命令）；
- "Tried writing "（0x142ad8250）+ EditType fmt + track-caller 文件；
- "assertion failed: **app_mode != AppMode::Replay"（0x142ad83f8，start_input_recording）；
- "History was mutated, but wasn't told about it"（0x142a98ad8，token Drop 守卫）；
- "Called finish_input_recording when history recording was already Finished"
  （0x142ad84a8）。

## 6. 跨域移交

- 楼梯 checksum 校验的 tag1 语义与 remap 表构造方 → stairs/wall 域。
- 快照负载 T 类型 → session-save（save::snapshot 域）。
- 0x14362c011/0x14362c308 由 load_game 按存档版本写入（本次新证，session-save 域互证）。
