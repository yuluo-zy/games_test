# RESTORE — unlock/replay-decorator（unlock-replay-decorator，2026-10-17）

> 角色：unlock-replay-decorator（解依赖线 #6）。
> 目标：在 country-core history/replay 侧补齐 `ReplayCommand` 的 Decorator 变体
> （0x30B 记录族）+ `ReplayBucket` 的 `Arc<[DecoratorId]>` cull-allow 表，解锁
> decorator `replay_decorator` / color `replay_color`。
> 硬标准：逐字段 fresh 反编译判等，不发明。
> 工具：ghidra `--worker shared`（只读）；本批 fresh 反编译 + 反汇编 + 原始字节。
>
> **落地模式：合同 + 登记（未改 src）**。目标文件
> `src/resources/history/replay_cmd_buffer.rs`、`src/resources/history/decorator_edit.rs`、
> `src/resources/history/mod.rs` 均为 history 组**未提交在途文件**
> （`git status` M；mtime 19:55–21:53），按任务约定「history 组在途文件→合同+登记」
> 不改写，供 history 组按本合同落地。本轮所有证据落
> `reports/unlock/replay-decorator/evidence/`。

## 三态回执

- **完成**：原生 `ReplayBucket` / 0x30B `ReplayCmd` / cull-allow 生产链 / `replay_decorator`
  消费体已全部 fresh 判等并落合同（实现 0，合同 4 组：Bucket 布局、0x30B 记录族、
  cull 生产链、消费体）。
- **登记**：在途 core 模型与原生结构差异 7 项 + 未解 1 项（U）；见 §6/§7。
- **阻塞**：无。`cargo check -p country-core` 通过（14 warning，0 error）。

---

## §1 原生 0x30B `ReplayCmd` 记录族（byte-level，O）

命令容器为 `Vec<ReplayCmd>`，**元素 stride 0x30B**（`replay_tree` 0x141bc34f0：
`end = commands_ptr + len*0x30`；`replay_decorator` 0x14080c8d0 同）。

| 偏移 | 类型 | 语义 | fresh 证据 |
|---|---|---|---|
| +0x00 | u8 | tag（判别） | 生成器写 `c6 45 28 <tag>` |
| +0x08 | u32 | flags：**bit0 = timed = `!instant`** | 生成器 `xor dil,1` → `[+8]` |
| +0x0c | f32 | duration | Color/Decorator/Stairs `0x3e4ccccd`(0.2)、AutoClutter `0x3dcccccd`(0.1) |
| +0x10 | Arc | 载荷（按 tag；ArcInner ptr） | 生成器 `mov rax,[rcx+8]; lock inc [rax]; mov [rec+0x10],rax` |
| +0x18 | f32 | elapsed（初值 0.0） | `c7 45 40 00000000` |
| +0x1c | bool | undo | 生成器 `mov [rec+0x1c],bl` |
| +0x1d..0x2f | — | padding（0） | 生成器 `mov byte [rec+0x1d],0` |

生成器字节证据（`evidence/generator-bytes.json`，地址 = 跳表体起点）：

| edit tag | 生成体 VA | 命令 tag | duration | 判读 |
|---|---|---|---|---|
| 9 Color | 0x14143fcc0 | **6** | 0x3e4ccccd (0.2) | `...c6452806` |
| 10 DecoratorV5 | 0x14143fed0 | **7** | 0x3e4ccccd (0.2) | `...c6452807` |
| 13 AutoClutter | 0x14143fc1e | 9 | 0x3dcccccd (0.1) | `...c6452809` |
| 14 Stairs | 0x14143fbc7 | 10 | 0x3e4ccccd (0.2) | `...c645280a` |
| 8 Roof | 0x14143ff27 | (5) | 0x3e4ccccd | 截断，与既有表一致 |

全命令 tag 面（生成器 + 既有 fresh）：0/1/2 Raster、3 Terrain、4 Wall、5 Roof、
6 Color、7 Decorator、8 Tree、9 AutoClutter、10 Stairs。

**修正（重要）**：wave3 依据 `replay_decorator` 释放跳表 PDB 类型名给出的
「0/1/2 RasterEdit、3 TerrainEdit、4 WallReplayCmd、5 RoofEdit、6/7
DecoratorHistoryEdit、8 TreeEdit、9 CrashMiscInfo Mutex、default PhysicalSize Mutex」
**只是在途反编译器的类型恢复结果，与生成器字节矛盾**（生成器明确写出
AutoClutter 命令 tag=9、Stairs=10，不可能是 CrashMiscInfo/PhysicalSize）。
该跳表 default 覆盖 tag>9，Ghidra 对共享 drop glue 的 PDB 名称恢复不可靠；
**语义映射以生成器 tag 为准**。Rust 枚举 `Drop` 自动释放，不需手写跳表。

**Rust 对照**：`ReplayCommand::Decorator(DecoratorReplayCmd)` 已由 history 组在途
落地（`replay_cmd_buffer.rs`），`native_tag()==7`、`DecoratorReplayCmd`
`{Arc<DecoratorHistoryEdit> @+0x10, timed=flags.bit0, duration, elapsed, undo}` 与上表
**判等**（除 §6-D3 的桶级 `timing` 重复模型）。

---

## §2 原生 `ReplayBucket` 布局（0x40B，O）

证据：
- 构造：`from_iter<ReplayBucket, Map<Iter<Vec<usize>>, schedule_replay_commands::closure_env_2>>`
  0x1412ffd50（反汇编 `evidence/disas_0x1412ffd50_schedule_buckets.txt`）；
- drop：`drop_in_place<ReplayBucket>` 0x140606e40；
- 移动：`ReplayBuffer::pop_front` 0x1409f18f0（逐 8 qword、stride 0x40）；
- 消费：`replay_decorator` 0x14080c8d0 / `replay_tree` 0x141bc34f0。

| 偏移 | 原生字段 | fresh 证据 |
|---|---|---|
| +0x00 | `commands.cap`（Rust Vec 本构建布局 cap/ptr/len） | 0x1412ffd50 `MOVUPS [base],XMM0`；drop `if (*rcx!=0) dealloc` |
| +0x08 | `commands.ptr` | 构造 `MOVUPS` 高 qword；消费 `[bucket+8]` 当指针 |
| +0x10 | `commands.len` | 构造 `[base+0x10]=R8`；remove 时 `len-1` |
| +0x18 | **`cull_allow`：`Option<Arc<[DecoratorId]>>` 的 ArcInner ptr**（null=None） | 构造 `[+0x18]=arc`；消费 `ptr=[+0x18]; [+0x18]=0`；drop `Arc::drop_slow<slice<DecoratorId>>` |
| +0x20 | `cull_allow` 元素个数（fat ptr） | 构造 `[+0x20]=len`；消费 `id=[ArcInner+0x10 + i*4]` |
| +0x28 | **`Option<Arc<DecoratorHistoryDiff>>`** | 构造清零；drop `drop_slow<DecoratorHistoryDiff>` |
| +0x30 | **`Option<Arc<StairsHistoryDiff>>`** | 构造清零；drop `drop_slow<StairsHistoryDiff>` |
| +0x38 | `undo: bool` | 构造 `MOV byte [base+0x38],AL`（AL=捕获 undo） |
| 0x39..0x3f | padding | — |

注：本构建的 `Vec` 内存序实测为 **cap@0 / ptr@8 / len@0x10**（空 Vec = (0,8,0)，
见 0x14130e520 收尾 `[RSI]=0,[RSI+8]=8,[RSI+0x10]=0`；构造器把 sret Vec 的
qword0/1/2 依次写入 +0/+8/+0x10）。合同按源码字段 `Vec` 声明即可，勿手写 repr(C)。

`front_mut_bucket` 0x1409f1a80：取 `ReplayBuffer.buckets` VecDeque 前元素指针
（空则 None）；`ReplayBuffer.speed` @+0x28（`replay_decorator`
`MOVSS XMM6,[ReplayBuffer+0x28]`）。`GameTime.delta` @+0x28。

**结论**：原生桶 = `{ commands, cull_allow, decorator_diff, stairs_diff, undo }`，
共 0x40B；**aux 两个 Arc 是桶内联字段，不是桶外并行队列**。

---

## §3 cull-allow 表的生产链（`replay_edit` 0x140af6170，O）

1. `replay_edit` 遍历 History entry（stride 0x18）逐 edit 调
   `generate_replay_cmds_from_edit`（跳表 0x142c5f284，分派器 0x14143fb50；命令
   生成表见 §1）。
2. 对 entry 的 **装饰 diff `Arc<DecoratorHistoryDiff>`（来源结构 +0x18）** 与
   楼梯 diff（+0x20）各做引用计数 +1。
3. 调 `DecoratorHistoryDiff::cull_allow_list`（PDB `impl$7::cull_allow_list`）：
   - 具体化链（symbol 原文）：
     `alloc::sync::impl$74::to_arc_slice<DecoratorId,
       FlatMap<Values<HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff>>,
               Filter<Copied<Keys<HashMap<DecoratorId, DecoratorHistoryState>>>,
                      cull_allow_list::closure_0::closure_env_0>,
               cull_allow_list::closure_env_0>>`
     → `from_iter` 0x1412feb50 → `Arc<[DecoratorId]>`；
   - `to_arc_slice` 0x140a96d20：ArcInner `{strong=1@0, weak=1@8, data@0x10}`，
     返回 (ArcInner ptr, len)，与 §2/+0x18、+0x20 及消费端 `data=ptr+0x10` 一致；
   - 迭代语义（fresh 0x1412feb50 + 0x1415635a0）：外层 `Values` 逐个
     `DecoratorStorageHistoryDiff`（含两张 **0x30B 状态 map**，同值内 +0x00/+0x30），
     取其中一张 map 的 `Keys<DecoratorId, DecoratorHistoryState>`（entry stride 0x58 =
     4B key + 0x50B state + pad），**过滤掉在另一张 map 中存在的 id**
     （hash 探针命中 → 跳过收集，未命中 → 收集）。方向选择由闭包捕获 flag 决定，
     flag 来源未解（§7-U1）。
4. 该 `Arc<[DecoratorId]>` 传进 `schedule_replay_commands` 的
   `closure_env_2`，`from_iter<ReplayBucket>` 0x1412ffd50 对**每个**桶
   `lock inc [arc]` 后写 +0x18/+0x20 —— 即**同一调度调用产生的所有桶共享同一
   cull 表**（不是仅装饰桶）。History 无装饰 diff → 桶 cull_allow=None。
5. 桶的 +0x28/+0x30 由 0x1412ffd50 清零；aux Arc 的挂接在同一 `replay_edit`
   后段完成（Arc 计数移动进桶），与当前 Rust `ReplayBuffer.aux` 并行队列不同
   （§6-D2）。

---

## §4 消费者 `replay_decorator` 0x14080c8d0（997B，O 逐语句）

证据：`evidence/decomp_0x14080c8d0_replay_decorator.c`（与本轮 fresh 反编译
逐字节一致）、`evidence/disas_0x14080c8d0_replay_decorator.txt`。

参数面：in_RCX=`ResMut<GameTime>`（delta @+0x28）、in_RDX=`ResMut<ReplayBuffer>`
（speed @+0x28）、in_R8=`ResMut<DecoratorArchivist>`（cull 集合 @+0x30）。

1. `speed = ReplayBuffer.speed`；`front_mut_bucket()`；无桶 return。
2. **取出 cull 表**：`arc = *(bucket+0x18)`；`len = *(bucket+0x20)`；
   `*(bucket+0x18) = 0`（Option take）。arc 非空时：
   `for i in 0..len { archivist.cull_set.insert(DecoratorId(*(u32*)(arc+0x10+i*4))) }`
   （`HashMap::insert<DecoratorId,()>` 0x1402c9200，archivist+0x30）；
   然后 `lock dec [arc]`，归零 `drop_slow<slice<DecoratorId>>`。
3. **命令扫描**：`ptr=*(bucket+8)`、`len=*(bucket+0x10)`、stride 0x30；
   **`tag != 7` 直接跳过**。对 tag==7：
   - `flags.byte(+0x8) & 1 == 0` → 记移除下标（立即完成，不推进 elapsed）；
   - 否则：`required = duration(+0xc) / speed`；
     `if !(required > 0.05) { required = 0.05 }`（MAXSS；**NaN → 0.05**）；
     `if required < elapsed(+0x18)`（UCOMISS+JBE；**严格大于才完成，NaN → 不完成**）
     → 记移除下标；随后**无条件** `elapsed = elapsed + delta`（bit0 置位路径，
     即使本帧完成也写）。
4. 若移除下标数 ≥2，先**原地反转** usize 数组；再逐个（递减顺序）`Vec::remove`
   （memmove `(len-1-idx)*0x30`），随后按 tag 释放载荷。
5. **本函数不发任何应用/进度事件**——装饰编辑的应用面不在此系统（cull 表 +
   elapsed 推进是它的全部输出）。

---

## §5 落地合同（给 history 组，直接可套）

### 5.1 `src/resources/history/replay_cmd_buffer.rs`

```rust
use crate::resources::walls::decorator::DecoratorId;
use crate::resources::history::decorator_edit::DecoratorHistoryDiff;
use crate::resources::stairs::stairs_state::StairsHistoryDiff;

pub struct ReplayBucket {
    pub commands: Vec<ReplayCommand>,                       // +0x00 (Vec 0x18)
    /// 原生 +0x18/+0x20：Option<Arc<[DecoratorId]>>；ptr==null = None。
    pub cull_allow: Option<std::sync::Arc<[DecoratorId]>>,
    /// 原生 +0x28（原 ReplayBucketAux 两槽应内联到这里）。
    pub decorator_diff: Option<std::sync::Arc<DecoratorHistoryDiff>>,
    /// 原生 +0x30。
    pub stairs_diff: Option<std::sync::Arc<StairsHistoryDiff>>,
    /// 原生 +0x38。
    pub undo: bool,
    // ---- crate-local（非原生字段，保留需注明）----
    pub entry_index: usize,          // 调度簿记；原生桶无此字段
    pub timing: Vec<CommandTiming>,  // 本地模型；原生时序内嵌 0x30B（见 5.3）
}
```

- `ReplayBuffer`：原生仅 `buckets + speed`；`aux: VecDeque<ReplayBucketAux>` 是
  本地等价模型，建议在本次补字段时把两 Arc 移入桶（`push_with_aux`/
  `pop_front_bucket` 保留为兼容包装亦可），避免双源（§6-D2）。
- `DecoratorReplayCmd` 已判等，无需改字段；`timed` 为 `flags.bit0`。

### 5.2 生产者：`DecoratorHistoryDiff::cull_allow_list`

原生 `DecoratorHistoryDiff` 是 **0x30B 的 HashMap 新类型**
（`HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff>`，非 type alias）；
当前 `pub type DecoratorHistoryDiffMap = HashMap<...>` 无法承载 `impl$7`。合同：

```rust
pub struct DecoratorHistoryDiff(pub DecoratorHistoryDiffMap); // 0x30B HashMap
impl DecoratorHistoryDiff {
    /// fresh replay_edit 0x140af6170 → to_arc_slice 0x140a96d20。
    /// 语义：对每个 storage diff 的 before/after 两张 states map，
    /// 取其中的 Keys，剔除在另一张 map 中也存在的 id，collect 为 Arc<[DecoratorId]>。
    /// 方向 flag 来源 U1（closure_0 未还原）——落地时先取 undo 方向做断言测试，
    /// 不得静默二选一；见 §7-U1。
    pub fn cull_allow_list(&self, direction: bool) -> std::sync::Arc<[DecoratorId]>;
}
```

`ReplayBucket::expand_with_aux` / 调度侧：
`cull_allow = history.decorator_diff.as_ref().map(|d| d.cull_allow_list(dir));`
并把同一 `Arc` clone 给本次调度的**每个**桶（原生行为）。

### 5.3 消费体（history 组在途 `src/systems/history/replay_decorator.rs` 需重写）

现文件是结构壳（自带 `DecoratorHistoryReplayApply` 事件、按比例 progress、
retain、无 cull、无 speed/0.05）——与 §4 不符，属「事件自造 + 载荷近似」。
重写要点（逐语句）：

```rust
let speed = replay.speed;
let dt = time.delta; // GameTime +0x28
let Some(bucket) = replay.front_mut_bucket() else { return; };
if let Some(ids) = bucket.cull_allow.take() {       // 取 +0x18/+0x20
    for id in ids.iter() { archivist.culling.allowed_ids.insert(id.0); }
    // ids Arc 在本作用域结束 drop（原生显式 dec/drop_slow）
}
let mut remove = Vec::new();
for (i, cmd) in bucket.commands.iter_mut().enumerate() {
    let ReplayCommand::Decorator(c) = cmd else { continue }; // tag != 7 skip
    if !c.timed { remove.push(i); continue; }           // flags.bit0==0 → 立即完成
    let required = (c.duration / speed).max(0.05);      // 注：原生 NaN→0.05
    if required < c.elapsed { remove.push(i); }         // 严格 >
    c.elapsed += dt;                                    // 本路径无条件推进
}
for i in remove.into_iter().rev() { let _ = bucket.commands.remove(i); }
```

注意：`f32::max` 的 NaN 语义与 MAXSS 一致（NaN→0.05），可用；`required < elapsed`
天然排除 NaN。不得保留 apply 事件（原生无）。

### 5.4 color `replay_color`

`ReplayCommand::Color(ColorReplayCmd)` 与 §1 记录面已判等；`replay_color` 可直接
匹配 `ReplayCommand::Color`，用命令内嵌 `timed/duration/elapsed/undo` 与
`payload`，**不需要桶 cull 表**。其真实消费体（13 个颜色写者 + payload target
字节布局）仍属 color 域 U（见 `crates/systems/color/src/replay_color.rs` 登记）。

---

## §6 在途实现与原生差异登记（本轮不改 src）

| # | 差异 | 原生（fresh） | 在途 Rust | 处置 |
|---|---|---|---|---|
| D1 | 桶缺 cull 表 | +0x18/+0x20 `Option<Arc<[DecoratorId]>>` | 无字段 | 本合同 5.1 补 |
| D2 | aux diff 位置 | 桶内 +0x28/+0x30 | `ReplayBucketAux` + `ReplayBuffer.aux` 并行队列 | 建议内联；否则登记等价模型 |
| D3 | 时序模型 | 命令记录内嵌（+8/+0xc/+0x18） | 桶级 `timing: Vec<CommandTiming>`（0x18B 额外字段） | 原生桶无；tag6/7 必须用命令内嵌，`timing` 对应槽保持同步或标记 unused |
| D4 | `systems/history/replay_decorator.rs` | 无事件、cull 插入、speed/0.05、严格 >、反序 remove | 自造 `DecoratorHistoryReplayApply` + ratio progress + retain | 按 5.3 重写（in-flight 文件） |
| D5 | decorator crate 消费模型 | cull id 在**桶级** Arc；命令载荷 = `Arc<DecoratorHistoryEdit>` | `crates/systems/decorator/src/replay_decorator.rs` 把 `decorator_id` 放在**每条命令**且 `decorator_cmd_of` 恒 None | core 落地后改为匹配 `ReplayCommand::Decorator` + 桶 cull；per-command id 删除 |
| D6 | 释放跳表类型名 | 生成器 tag 0..10（AutoClutter=9/Stairs=10） | wave3 记 9=CrashMiscInfo/default=PhysicalSize | **修正**：PDB 共享 drop glue 名称不可信，按生成器；Rust Drop 自动 |
| D7 | `systems/history/replay_color.rs` | 直接派发 13 颜色写者 | 空壳 + 登记 | 不阻塞（Color 变体已解锁），消费者本体仍 U |

---

## §7 未解/登记（ledger-ready）

- **U1（re-analysis）**：`DecoratorHistoryDiff::cull_allow_list` 的 `closure_0`
  方向 flag 来源（before→after 还是 after→before；疑似 undo 方向，未取到实参）。
  已解：被迭代/被探针的两张 inner states map 相对偏移 +0x00/+0x30、entry stride
  0x58、过滤语义 = 「剔除存在于另一张 map 的 id」。
- **登记 CORE-1**：`ReplayBucket` 补
  `cull_allow: Option<Arc<[DecoratorId]>>`（+0x18/+0x20）、
  `decorator_diff`/`stairs_diff` 内联（+0x28/+0x30）；0x30B 内嵌时序为权威，
  桶级 `timing` 为本地模型。→ 历史组在途文件落地（本报告 5.1/5.2）。
- **登记 CONS-1**：`systems/history/replay_decorator.rs` 按 §4/5.3 重写；
  `crates/systems/decorator/src/replay_decorator.rs` 按 D5 改接。
- **登记 U2**：AutoClutter/Stairs 生成器 tag 9/10 与释放跳表 PDB 名称冲突已按
  生成器裁决（D6）；若后续发现 `ReplayCmd` 未包含 9/10，需重新核对（低风险）。

---

## §8 编译

- `cargo check -p country-core`：**Finished dev profile，0 error**（14 warning，
  均为既有 non_snake_case/serde 面，非本轮引入）。本轮未改 src，故为基线确认。

## 证据索引（`reports/unlock/replay-decorator/evidence/`）

| 文件 | 内容 |
|---|---|
| `decomp_0x14080c8d0_replay_decorator.c` | 消费体 fresh 全文（997B） |
| `disas_0x14080c8d0_replay_decorator.txt` | 消费体反汇编（桶 +0x8/+0x10/+0x18/+0x20、64B 扫描、反序 remove） |
| `decomp_0x1412ffd50_schedule_buckets.c` / `disas_0x1412ffd50_schedule_buckets.txt` | 桶构造（8 qword、stride 0x40、cull 写入 +0x18/+0x20） |
| `decomp_0x140606e40_drop_replaybucket.c` | 桶字段全集（commands + slice Arc + DecoratorHistoryDiff Arc + StairsHistoryDiff Arc） |
| `decomp_0x1409f18f0_pop_front.c` | 桶移动 8 qword/0x40 |
| `decomp_0x1409f1a80_front_mut_bucket.c` | front bucket |
| `decomp_0x140af6170_replay_edit.c` | cull 生产链（to_arc_slice 调用点、命令调度谓词） |
| `decomp_0x1412feb50_cull_allow_from_iter.c` | `cull_allow_list` 具体化 + 过滤探针 |
| `decomp_0x140a96d20_to_arc_slice.c` | `Arc<[T]>` 构造（ArcInner +0x10） |
| `generator-bytes.json` | Color/Decorator/AutoClutter/Stairs/Roof 生成器原始字节（tag/duration/flags） |
